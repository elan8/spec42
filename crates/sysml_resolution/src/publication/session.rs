//! The publication lifecycle for a long-lived host, and the barrier that admits a finished build.
//!
//! A session owns the current publication and two independent token spaces:
//!
//! * a **relink token** gates editor-staged symbol commits — every edit supersedes the previous
//!   one, and a relink result is committed only while its token is current;
//! * a **build token** carries the identity a background build was started for — a finished
//!   build is admitted only if no newer build has been started since and the publication it
//!   produced has exactly the identity the token names.
//!
//! Admission is independent of relink generation, so an edit that arrives while a build runs
//! does not discard a build whose inputs it did not change; the host's own revision check decides
//! whether the result is still worth mirroring. The session is a plain synchronous state machine:
//! whatever single-writer discipline guards the host's state guards this too.

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

/// Token returned by [`Session::schedule_relink`]. Only the newest relink's token is current.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelinkToken {
    publication: PublicationToken,
    generation: u64,
}

impl RelinkToken {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn publication(&self) -> PublicationToken {
        self.publication
    }
}

/// Token carried by a background build: the owner it was started in, its place in the build
/// order, and the identity it must produce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildToken {
    owner: u64,
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
/// Ready → Reindexing          schedule_relink()
/// Reindexing → Reindexing     schedule_relink()   (newer edit supersedes)
/// Reindexing → Ready          commit_relink()
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
    /// Incremented per scheduled relink; only the newest token's generation is current.
    relink_generation: u64,
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
            relink_generation: self.relink_generation,
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
            relink_generation: 0,
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

    /// A document changed: schedule an async relink, superseding any previous one.
    pub fn schedule_relink(&mut self) -> RelinkToken {
        debug_assert!(matches!(
            self.lifecycle,
            SessionLifecycle::Ready | SessionLifecycle::Reindexing
        ));
        self.relink_generation = increment(self.relink_generation, "relink generation");
        self.transition(SessionLifecycle::Reindexing);
        RelinkToken {
            publication: self.publication(),
            generation: self.relink_generation,
        }
    }

    /// Whether `token` is the current pending relink.
    pub fn is_token_current(&self, token: &RelinkToken) -> bool {
        self.relink_generation == token.generation
            && self.is_publication_current(&token.publication)
            && self.lifecycle == SessionLifecycle::Reindexing
    }

    /// Commits a relink; `false` when the token was superseded and nothing changed.
    pub fn commit_relink(&mut self, token: &RelinkToken) -> bool {
        if !self.is_token_current(token) {
            return false;
        }
        self.transition(SessionLifecycle::Ready);
        true
    }

    pub fn begin_library_reindex(&mut self) {
        self.transition(SessionLifecycle::Reindexing);
    }

    pub fn complete_reindex(&mut self) -> u64 {
        debug_assert_eq!(self.lifecycle, SessionLifecycle::Reindexing);
        self.transition(SessionLifecycle::Ready)
    }

    /// Bumps the version without a lifecycle change, invalidating in-flight work.
    pub fn bump_version(&mut self) -> u64 {
        self.version = increment(self.version, "publication version");
        self.version
    }

    /// Starts a build for `identity`, superseding any build started earlier.
    pub fn begin_build(&mut self, identity: PublicationIdentity) -> BuildToken {
        self.build_generation = increment(self.build_generation, "build generation");
        BuildToken {
            owner: self.owner,
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
            || token.generation != self.build_generation
            || self.lifecycle == SessionLifecycle::Closed
        {
            return PublicationOutcome::Superseded;
        }
        match result {
            Ok(published) if published.identity() == &token.identity => {
                self.current = published;
                PublicationOutcome::Published
            }
            Ok(_) => PublicationOutcome::IdentityMismatch,
            Err(_) => PublicationOutcome::Failed,
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
    fn newer_relink_invalidates_older_token_and_commit_returns_to_ready() {
        let mut session = ready();
        let stale = session.schedule_relink();
        let fresh = session.schedule_relink();
        assert!(!session.is_token_current(&stale));
        assert!(session.is_token_current(&fresh));
        assert!(!session.commit_relink(&stale));
        assert_eq!(session.lifecycle(), SessionLifecycle::Reindexing);
        assert!(session.commit_relink(&fresh));
        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
    }

    #[test]
    fn bump_reset_and_close_invalidate_outstanding_work() {
        let mut session = ready();
        let token = session.schedule_relink();
        session.bump_version();
        assert!(!session.is_token_current(&token));
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
        let first_token = first.schedule_relink();
        let second_token = second.schedule_relink();
        assert_eq!(first_token.generation(), second_token.generation());
        assert!(!second.is_token_current(&first_token));
        assert!(second.is_token_current(&second_token));
    }

    #[test]
    fn builds_are_admitted_by_order_and_identity_independently_of_relinks() {
        let mut session = ready();
        let older = session.begin_build(identity("older"));
        let newer = session.begin_build(identity("newer"));
        // An edit arriving while the builds run does not supersede them.
        let _relink = session.schedule_relink();

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
