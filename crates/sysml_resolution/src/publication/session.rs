//! The publication lifecycle for a long-lived host, and the barrier that admits a finished build.
//!
//! A session owns the current publication and the input revision from which the next publication
//! is built. Every semantic-input mutation advances that revision. A build token captures it, so
//! work completed for superseded inputs can never cross the publication barrier.

use std::sync::Arc;

use crate::PublicationIdentity;

/// Lifecycle state tracked by a [`Session`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionLifecycle {
    #[default]
    Cold,
    Indexing,
    Ready,
    Reindexing,
    Closed,
}

/// Owner-scoped identity of the complete state a session currently publishes.
///
/// Background work captures this value together with the immutable inputs it reads, and may
/// publish only while [`Session::is_publication_current`] still accepts it. The owner component
/// keeps a token from one session from being mistaken for one from another.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationToken {
    owner: u64,
    version: u64,
}

impl PublicationToken {
    /// Monotonic version within the owning session; not an identity without the owner.
    pub fn version(&self) -> u64 {
        self.version
    }
}

/// Token carried by a background build: the owner it was started in, its place in the build
/// order, and the identity it must produce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildToken {
    owner: u64,
    input_revision: u64,
    generation: u64,
    identity: PublicationIdentity,
}

impl BuildToken {
    pub fn identity(&self) -> &PublicationIdentity {
        &self.identity
    }
}

/// Why a finished build did or did not replace the current publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationOutcome {
    Published,
    /// A newer build was started after this one, or the session was re-keyed or closed.
    Superseded,
    /// The build produced a publication whose identity differs from the one its token names.
    IdentityMismatch,
    /// Construction failed; the previous publication remains.
    Failed,
}

/// Anything a session can publish: it has a dependency-complete identity.
pub trait Published {
    fn identity(&self) -> &PublicationIdentity;
}

impl Published for crate::PublishedResolution {
    fn identity(&self) -> &PublicationIdentity {
        self.identity()
    }
}

/// The lifecycle and publication barrier of one host session.
///
/// ```text
/// Cold → Indexing             begin_startup()
/// Cold/Indexing → Ready       complete_startup()
/// Ready/Reindexing → Reindexing invalidate_inputs()
/// * → Reindexing              begin_library_reindex()
/// Reindexing → Ready          complete_reindex()
/// Cold/Indexing/Ready/Reindexing → Cold  reset()
/// * → Closed                  close() (terminal)
/// ```
#[derive(Debug)]
pub struct Session<P> {
    owner: u64,
    lifecycle: SessionLifecycle,
    /// Bumped on every transition and on bare `bump_version` calls: the "did anything change?"
    /// discriminator in-flight work checks before publishing.
    version: u64,
    /// Incremented for every mutation of a semantic construction prerequisite.
    input_revision: u64,
    /// Incremented per started build; only the newest build may be admitted.
    build_generation: u64,
    current: Arc<P>,
}

impl<P> Clone for Session<P> {
    fn clone(&self) -> Self {
        Self {
            owner: self.owner,
            lifecycle: self.lifecycle,
            version: self.version,
            input_revision: self.input_revision,
            build_generation: self.build_generation,
            current: Arc::clone(&self.current),
        }
    }
}

impl<P: Published> Session<P> {
    /// A cold session publishing `initial`.
    pub fn new(initial: Arc<P>) -> Self {
        Self {
            owner: next_session_owner(),
            lifecycle: SessionLifecycle::Cold,
            version: 0,
            input_revision: 0,
            build_generation: 0,
            current: initial,
        }
    }

    /// A session already at `Ready`, for hosts that assembled a publication before serving.
    pub fn ready(initial: Arc<P>) -> Self {
        let mut session = Self::new(initial);
        session.begin_startup();
        session.complete_startup();
        session
    }

    /// The current publication. Readers may keep the `Arc` past any later admission.
    pub fn current(&self) -> &Arc<P> {
        &self.current
    }

    pub fn lifecycle(&self) -> SessionLifecycle {
        self.lifecycle
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    /// The identity of the complete state this session currently publishes.
    pub fn publication(&self) -> PublicationToken {
        PublicationToken {
            owner: self.owner,
            version: self.version,
        }
    }

    /// Whether `token` still names the complete state this session currently owns.
    pub fn is_publication_current(&self, token: &PublicationToken) -> bool {
        self.owner == token.owner
            && self.version == token.version
            && self.lifecycle != SessionLifecycle::Closed
    }

    /// Gives a newly spawned owner a distinct identity, invalidating tokens inherited from a
    /// cloned seed state.
    pub fn rekey_for_owner(&mut self) {
        self.owner = next_session_owner();
    }

    /// Resets to `Cold`, invalidating every outstanding token.
    pub fn reset(&mut self) {
        self.input_revision = increment(self.input_revision, "input revision");
        self.transition(SessionLifecycle::Cold);
    }

    /// Permanently closes this session; a closed session accepts no later background result.
    pub fn close(&mut self) {
        self.transition(SessionLifecycle::Closed);
    }

    pub fn begin_startup(&mut self) {
        debug_assert_eq!(self.lifecycle, SessionLifecycle::Cold);
        self.transition(SessionLifecycle::Indexing);
    }

    pub fn complete_startup(&mut self) -> u64 {
        debug_assert!(matches!(
            self.lifecycle,
            SessionLifecycle::Cold | SessionLifecycle::Indexing
        ));
        self.transition(SessionLifecycle::Ready)
    }

    /// Marks the semantic construction inputs changed and supersedes all work for older inputs.
    pub fn invalidate_inputs(&mut self) -> u64 {
        self.input_revision = increment(self.input_revision, "input revision");
        match self.lifecycle {
            SessionLifecycle::Ready | SessionLifecycle::Reindexing => {
                self.transition(SessionLifecycle::Reindexing);
            }
            SessionLifecycle::Cold | SessionLifecycle::Indexing => {
                self.version = increment(self.version, "publication version");
            }
            SessionLifecycle::Closed => {}
        }
        self.input_revision
    }

    pub fn begin_library_reindex(&mut self) {
        self.input_revision = increment(self.input_revision, "input revision");
        self.transition(SessionLifecycle::Reindexing);
    }

    pub fn complete_reindex(&mut self) -> u64 {
        debug_assert_eq!(self.lifecycle, SessionLifecycle::Reindexing);
        self.transition(SessionLifecycle::Ready)
    }

    /// Bumps the version without a lifecycle change, invalidating in-flight work.
    pub fn bump_version(&mut self) -> u64 {
        self.input_revision = increment(self.input_revision, "input revision");
        self.version = increment(self.version, "publication version");
        self.version
    }

    /// Starts a build for `identity`, superseding any build started earlier.
    pub fn begin_build(&mut self, identity: PublicationIdentity) -> BuildToken {
        self.build_generation = increment(self.build_generation, "build generation");
        BuildToken {
            owner: self.owner,
            input_revision: self.input_revision,
            generation: self.build_generation,
            identity,
        }
    }

    /// The publication barrier: admit a finished build only if it is still the newest and
    /// produced exactly the identity its token names. Failures keep the current publication.
    pub fn admit<E>(
        &mut self,
        token: &BuildToken,
        result: Result<Arc<P>, E>,
    ) -> PublicationOutcome {
        if token.owner != self.owner
            || token.input_revision != self.input_revision
            || token.generation != self.build_generation
            || self.lifecycle == SessionLifecycle::Closed
        {
            return PublicationOutcome::Superseded;
        }
        match result {
            Ok(published) if published.identity() == &token.identity => {
                self.current = published;
                self.version = increment(self.version, "publication version");
                if self.lifecycle == SessionLifecycle::Reindexing {
                    self.lifecycle = SessionLifecycle::Ready;
                }
                PublicationOutcome::Published
            }
            Ok(_) => {
                self.finish_current_build();
                PublicationOutcome::IdentityMismatch
            }
            Err(_) => {
                self.finish_current_build();
                PublicationOutcome::Failed
            }
        }
    }

    fn finish_current_build(&mut self) {
        if self.lifecycle == SessionLifecycle::Reindexing {
            self.transition(SessionLifecycle::Ready);
        }
    }

    /// Finishes a failed preparation for the current semantic inputs. There is no build token yet,
    /// but the lifecycle must not claim work remains in flight.
    pub fn finish_preparation_failure(&mut self) {
        if self.lifecycle == SessionLifecycle::Reindexing {
            self.transition(SessionLifecycle::Ready);
        }
    }

    fn transition(&mut self, new: SessionLifecycle) -> u64 {
        assert!(
            self.lifecycle != SessionLifecycle::Closed || new == SessionLifecycle::Closed,
            "a closed publication session cannot be reopened"
        );
        self.lifecycle = new;
        self.version = increment(self.version, "publication version");
        self.version
    }
}

fn increment(value: u64, label: &str) -> u64 {
    value
        .checked_add(1)
        .unwrap_or_else(|| panic!("publication session {label} exhausted"))
}

fn next_session_owner() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_SESSION_OWNER: AtomicU64 = AtomicU64::new(1);
    NEXT_SESSION_OWNER
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |owner| {
            owner.checked_add(1)
        })
        .expect("publication session owner identities exhausted")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Fake(PublicationIdentity);

    impl Published for Fake {
        fn identity(&self) -> &PublicationIdentity {
            &self.0
        }
    }

    fn identity(tag: &str) -> PublicationIdentity {
        let source = crate::SourceInput::new(
            format!("memory://t/{tag}.sysml"),
            format!("package {tag};"),
            sysml_source::SourceKind::Workspace,
        );
        crate::BuildRequest::new(
            vec![source],
            crate::ConstructionSchedule::Sequential,
            "test",
        )
        .unwrap()
        .identity()
        .clone()
    }

    fn ready() -> Session<Fake> {
        Session::ready(Arc::new(Fake(identity("initial"))))
    }

    #[test]
    fn startup_transitions_cold_indexing_ready() {
        let mut session = Session::new(Arc::new(Fake(identity("initial"))));
        assert_eq!(session.lifecycle(), SessionLifecycle::Cold);
        assert_eq!(session.version(), 0);
        session.begin_startup();
        assert_eq!(session.lifecycle(), SessionLifecycle::Indexing);
        let version = session.complete_startup();
        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
        assert_eq!(version, session.version());
    }

    #[test]
    fn an_input_change_supersedes_a_build_for_the_previous_revision() {
        let mut session = ready();
        let stale = session.begin_build(identity("stale"));
        session.invalidate_inputs();
        assert_eq!(
            session.admit::<()>(&stale, Ok(Arc::new(Fake(identity("stale"))))),
            PublicationOutcome::Superseded
        );
    }

    #[test]
    fn bump_reset_and_close_invalidate_outstanding_work() {
        let mut session = ready();
        let token = session.begin_build(identity("pending"));
        session.bump_version();
        session.invalidate_inputs();
        assert_eq!(
            session.admit::<()>(&token, Ok(Arc::new(Fake(identity("pending"))))),
            PublicationOutcome::Superseded
        );
        let publication = session.publication();
        session.reset();
        assert_eq!(session.lifecycle(), SessionLifecycle::Cold);
        assert!(!session.is_publication_current(&publication));
        session.close();
        assert!(!session.is_publication_current(&session.publication()));
    }

    #[test]
    fn a_token_from_another_session_is_never_current() {
        let mut first = ready();
        let mut second = ready();
        let first_token = first.begin_build(identity("x"));
        let second_token = second.begin_build(identity("x"));
        assert_eq!(
            second.admit::<()>(&first_token, Ok(Arc::new(Fake(identity("x"))))),
            PublicationOutcome::Superseded
        );
        assert_eq!(
            second.admit::<()>(&second_token, Ok(Arc::new(Fake(identity("x"))))),
            PublicationOutcome::Published
        );
    }

    #[test]
    fn builds_are_admitted_by_order_and_identity() {
        let mut session = ready();
        let older = session.begin_build(identity("older"));
        let newer = session.begin_build(identity("newer"));
        assert_eq!(
            session.admit::<()>(&older, Ok(Arc::new(Fake(identity("older"))))),
            PublicationOutcome::Superseded
        );
        assert_eq!(session.current().0, identity("initial"));
        assert_eq!(
            session.admit::<()>(&newer, Ok(Arc::new(Fake(identity("other"))))),
            PublicationOutcome::IdentityMismatch
        );
        assert_eq!(
            session.admit::<()>(&newer, Err(())),
            PublicationOutcome::Failed
        );
        assert_eq!(session.current().0, identity("initial"));
        assert_eq!(
            session.admit::<()>(&newer, Ok(Arc::new(Fake(identity("newer"))))),
            PublicationOutcome::Published
        );
        assert_eq!(session.current().0, identity("newer"));
    }

    #[test]
    fn successful_admission_invalidates_the_previous_publication_token() {
        let mut session = ready();
        let previous = session.publication();
        let token = session.begin_build(identity("new"));
        assert_eq!(
            session.admit::<()>(&token, Ok(Arc::new(Fake(identity("new"))))),
            PublicationOutcome::Published
        );
        assert!(!session.is_publication_current(&previous));
        assert!(session.is_publication_current(&session.publication()));
    }

    #[test]
    fn reset_and_library_reindex_supersede_preexisting_builds() {
        let mut session = ready();
        let before_reset = session.begin_build(identity("before-reset"));
        session.reset();
        assert_eq!(
            session.admit::<()>(&before_reset, Ok(Arc::new(Fake(identity("before-reset"))))),
            PublicationOutcome::Superseded
        );

        session.begin_startup();
        session.complete_startup();
        let before_reindex = session.begin_build(identity("before-reindex"));
        session.begin_library_reindex();
        assert_eq!(
            session.admit::<()>(
                &before_reindex,
                Ok(Arc::new(Fake(identity("before-reindex"))))
            ),
            PublicationOutcome::Superseded
        );
    }

    #[test]
    fn failed_preparation_finishes_reindex_without_changing_the_last_good_publication() {
        let mut session = ready();
        let last_good = Arc::clone(session.current());
        session.invalidate_inputs();
        assert_eq!(session.lifecycle(), SessionLifecycle::Reindexing);

        session.finish_preparation_failure();

        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
        assert!(Arc::ptr_eq(session.current(), &last_good));
    }

    #[test]
    fn a_closed_or_rekeyed_session_admits_nothing() {
        let mut session = ready();
        let token = session.begin_build(identity("x"));
        session.rekey_for_owner();
        assert_eq!(
            session.admit::<()>(&token, Ok(Arc::new(Fake(identity("x"))))),
            PublicationOutcome::Superseded
        );
        let token = session.begin_build(identity("x"));
        session.close();
        assert_eq!(
            session.admit::<()>(&token, Ok(Arc::new(Fake(identity("x"))))),
            PublicationOutcome::Superseded
        );
    }

    #[test]
    #[should_panic(expected = "cannot be reopened")]
    fn closed_session_cannot_be_reset() {
        let mut session = ready();
        session.close();
        session.reset();
    }
}
