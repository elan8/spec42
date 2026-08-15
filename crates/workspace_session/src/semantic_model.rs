//! Atomic publication owner for opaque immutable
//! [`sysml_query::resolved_slice::PublishedModel`] values.
//!
//! A build runs against an immutable source snapshot outside this owner.  The owner only admits
//! a completed model at its publication barrier, after checking both the owner-scoped relink token
//! and the model identity captured when the build started.  Failed, cancelled, stale, and
//! out-of-order builds therefore leave the last coherent model available to readers.

use std::sync::Arc;

use sysml_query::resolved_slice::{
    BuildRequest, PublicationCompleteness, PublicationIdentity, PublishedModel,
};
use workspace::{SessionLifecycle, WorkspaceSession};

use crate::{MutatePanicked, Mutation, SessionActor, SnapshotHandle, TracksRelink};

#[derive(Clone)]
struct PublicationOwnerState<P> {
    session: WorkspaceSession,
    publication: P,
}

impl<P> TracksRelink for PublicationOwnerState<P> {
    fn is_token_current(&self, token: &workspace::RelinkToken) -> bool {
        self.session.is_token_current(token)
    }

    fn rekey_for_actor(&mut self) {
        self.session.rekey_for_owner();
    }
}

trait PublicationValue<I> {
    fn identity(&self) -> &I;
    fn is_complete(&self) -> bool;
}

impl PublicationValue<PublicationIdentity> for Arc<PublishedModel> {
    fn identity(&self) -> &PublicationIdentity {
        self.publication().identity()
    }

    /// Only a `Complete` publication is admitted. Parse recovery, unsupported syntax, and a
    /// non-converged solve are each a distinct reason the build did not settle, and none of them
    /// may quietly replace the last coherent model a reader is using.
    fn is_complete(&self) -> bool {
        self.publication().completeness() == PublicationCompleteness::Complete
    }
}

#[derive(Debug, Clone)]
struct PublicationBuildToken<I> {
    relink: workspace::RelinkToken,
    identity: I,
}

#[derive(Clone)]
struct PublicationSessionCore<P, I> {
    actor: SessionActor<PublicationOwnerState<P>>,
    snapshot: SnapshotHandle<PublicationOwnerState<P>>,
    identity: std::marker::PhantomData<fn() -> I>,
}

impl<P, I> PublicationSessionCore<P, I>
where
    P: PublicationValue<I> + Clone + Send + Sync + 'static,
    I: Clone + Eq + Send + 'static,
{
    fn new(initial: P) -> Self {
        assert!(
            initial.is_complete(),
            "the initial semantic publication must be complete"
        );
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let (actor, snapshot) = SessionActor::spawn(PublicationOwnerState {
            session,
            publication: initial,
        });
        Self {
            actor,
            snapshot,
            identity: std::marker::PhantomData,
        }
    }

    fn current(&self) -> P {
        self.snapshot.current().publication.clone()
    }

    async fn begin_build(&self, identity: I) -> Result<PublicationBuildToken<I>, MutatePanicked> {
        self.actor
            .mutate(move |state| PublicationBuildToken {
                relink: state.session.schedule_relink(),
                identity,
            })
            .await
    }

    async fn finish_build(
        &self,
        token: PublicationBuildToken<I>,
        result: Result<P, SemanticBuildFailureKind>,
    ) -> Result<SemanticPublicationOutcome, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |state| {
                if !state.session.is_token_current(&token.relink) {
                    return Mutation::Unchanged(SemanticPublicationOutcome::Stale);
                }

                let outcome = match result {
                    Err(SemanticBuildFailureKind::Failed) => {
                        assert!(state.session.commit_relink(&token.relink));
                        SemanticPublicationOutcome::DiscardedFailed
                    }
                    Err(SemanticBuildFailureKind::Cancelled) => {
                        assert!(state.session.commit_relink(&token.relink));
                        SemanticPublicationOutcome::DiscardedCancelled
                    }
                    Ok(publication) if !publication.is_complete() => {
                        assert!(state.session.commit_relink(&token.relink));
                        SemanticPublicationOutcome::DiscardedIncomplete
                    }
                    Ok(publication) if publication.identity() != &token.identity => {
                        assert!(state.session.commit_relink(&token.relink));
                        SemanticPublicationOutcome::DiscardedIdentityMismatch
                    }
                    Ok(publication) => {
                        state.publication = publication;
                        assert!(state.session.commit_relink(&token.relink));
                        SemanticPublicationOutcome::Published
                    }
                };
                Mutation::Changed(outcome)
            })
            .await
            .map(|outcome| outcome.value)
    }

    fn lifecycle(&self) -> SessionLifecycle {
        self.snapshot.current().session.lifecycle()
    }
}

/// Token carried by a background whole-model build.
///
/// The identity is part of the token rather than inferred from the eventual result, so a caller
/// cannot accidentally publish a model built from inputs other than the snapshot it scheduled.
#[derive(Debug, Clone)]
pub struct SemanticBuildToken {
    inner: PublicationBuildToken<PublicationIdentity>,
}

impl SemanticBuildToken {
    pub fn identity(&self) -> &PublicationIdentity {
        &self.inner.identity
    }

    pub fn relink(&self) -> &workspace::RelinkToken {
        &self.inner.relink
    }
}

/// Why a build did not replace the current semantic model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticPublicationOutcome {
    Published,
    DiscardedFailed,
    DiscardedCancelled,
    DiscardedIncomplete,
    DiscardedIdentityMismatch,
    Stale,
}

/// A reader's immutable semantic publication.
#[derive(Debug, Clone)]
pub struct PublishedModelSnapshot {
    model: Arc<PublishedModel>,
}

impl PublishedModelSnapshot {
    pub fn model(&self) -> &PublishedModel {
        &self.model
    }

    pub fn into_model(self) -> Arc<PublishedModel> {
        self.model
    }
}

/// Owns the current immutable semantic publication and serializes its publication barrier.
///
/// Readers clone an [`Arc`] to the model without waiting for a build.  A build result is visible
/// only after the actor has checked the complete identity and owner-scoped token.  The owner does
/// not expose mutable graph state or a partial model.
#[derive(Clone)]
pub struct SemanticPublicationSession {
    core: PublicationSessionCore<Arc<PublishedModel>, PublicationIdentity>,
}

impl SemanticPublicationSession {
    /// Starts a ready session with an already settled model.
    pub fn new(initial: Arc<PublishedModel>) -> Self {
        Self {
            core: PublicationSessionCore::new(initial),
        }
    }

    /// Returns an immutable model `Arc` immediately.  A reader may retain it while a replacement
    /// build is running or after a newer model has been published.
    pub fn current(&self) -> Arc<PublishedModel> {
        self.core.current()
    }

    /// Returns a reader-owned immutable publication wrapper.
    pub fn snapshot(&self) -> PublishedModelSnapshot {
        PublishedModelSnapshot {
            model: self.current(),
        }
    }

    /// Schedules a whole-model build against an immutable request, superseding any older build.
    ///
    /// The token identity is derived from `request` inside this method.  Callers cannot provide a
    /// separate identity that accidentally disagrees with the source, construction, evaluation,
    /// or configuration inputs used by the build.
    pub async fn begin_build(
        &self,
        request: &BuildRequest,
    ) -> Result<SemanticBuildToken, MutatePanicked> {
        self.core
            .begin_build(request.identity().clone())
            .await
            .map(|inner| SemanticBuildToken { inner })
    }

    /// Completes a build at the publication barrier.
    ///
    /// Errors and cancellation are explicit outcomes and retain the previous model.  A stale
    /// result is ignored without changing the newer build's lifecycle.  A current result that is
    /// incomplete or has a different identity is rejected and leaves the previous model in place.
    pub async fn finish_build(
        &self,
        token: SemanticBuildToken,
        result: Result<Arc<PublishedModel>, SemanticBuildFailureKind>,
    ) -> Result<SemanticPublicationOutcome, MutatePanicked> {
        self.core.finish_build(token.inner, result).await
    }

    /// Returns the owner lifecycle associated with the current publication.
    pub fn lifecycle(&self) -> SessionLifecycle {
        self.core.lifecycle()
    }
}

/// Explicit outcome for a build that did not produce a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticBuildFailureKind {
    Failed,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[derive(Debug)]
    struct FakePublication {
        identity: u64,
        complete: bool,
    }

    impl PublicationValue<u64> for Arc<FakePublication> {
        fn identity(&self) -> &u64 {
            &self.identity
        }

        fn is_complete(&self) -> bool {
            self.complete
        }
    }

    fn publication(identity: u64) -> Arc<FakePublication> {
        Arc::new(FakePublication {
            identity,
            complete: true,
        })
    }

    fn incomplete_publication(identity: u64) -> Arc<FakePublication> {
        Arc::new(FakePublication {
            identity,
            complete: false,
        })
    }

    #[tokio::test]
    async fn current_build_atomically_replaces_publication_and_retains_old_readers() {
        let first = publication(1);
        let second = publication(2);
        let session = PublicationSessionCore::new(first.clone());
        let retained = session.current();

        let token = session.begin_build(2).await.unwrap();
        assert_eq!(session.lifecycle(), SessionLifecycle::Reindexing);
        assert_eq!(token.identity, 2);
        assert_eq!(
            session
                .finish_build(token, Ok(second.clone()))
                .await
                .unwrap(),
            SemanticPublicationOutcome::Published
        );

        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
        assert!(Arc::ptr_eq(&retained, &first));
        assert!(Arc::ptr_eq(&session.current(), &second));
    }

    #[tokio::test]
    async fn failed_cancelled_incomplete_and_mismatched_builds_keep_prior_publication() {
        let first = publication(1);
        let session = PublicationSessionCore::new(first.clone());

        let failed = session.begin_build(1).await.unwrap();
        assert_eq!(
            session
                .finish_build(failed, Err(SemanticBuildFailureKind::Failed))
                .await
                .unwrap(),
            SemanticPublicationOutcome::DiscardedFailed
        );

        let cancelled = session.begin_build(1).await.unwrap();
        assert_eq!(
            session
                .finish_build(cancelled, Err(SemanticBuildFailureKind::Cancelled))
                .await
                .unwrap(),
            SemanticPublicationOutcome::DiscardedCancelled
        );

        let incomplete = session.begin_build(2).await.unwrap();
        assert_eq!(
            session
                .finish_build(incomplete, Ok(incomplete_publication(2)))
                .await
                .unwrap(),
            SemanticPublicationOutcome::DiscardedIncomplete
        );

        let mismatch = session.begin_build(2).await.unwrap();
        assert_eq!(
            session
                .finish_build(mismatch, Ok(publication(3)))
                .await
                .unwrap(),
            SemanticPublicationOutcome::DiscardedIdentityMismatch
        );

        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
        assert!(Arc::ptr_eq(&session.current(), &first));
    }

    #[tokio::test]
    async fn stale_out_of_order_result_cannot_replace_newer_publication() {
        let first = publication(1);
        let second = publication(2);
        let third = publication(3);
        let session = PublicationSessionCore::new(first);

        let stale = session.begin_build(2).await.unwrap();
        let current = session.begin_build(3).await.unwrap();
        assert_eq!(
            session.finish_build(stale, Ok(second)).await.unwrap(),
            SemanticPublicationOutcome::Stale
        );
        assert_eq!(session.lifecycle(), SessionLifecycle::Reindexing);
        assert_eq!(
            session
                .finish_build(current, Ok(third.clone()))
                .await
                .unwrap(),
            SemanticPublicationOutcome::Published
        );
        assert!(Arc::ptr_eq(&session.current(), &third));
    }

    #[tokio::test]
    async fn token_from_another_owner_is_stale() {
        let first = PublicationSessionCore::new(publication(1));
        let second = publication(2);
        let second_owner = PublicationSessionCore::new(second.clone());
        let foreign = first.begin_build(2).await.unwrap();

        assert_eq!(
            second_owner
                .finish_build(foreign, Ok(second.clone()))
                .await
                .unwrap(),
            SemanticPublicationOutcome::Stale
        );
        assert!(Arc::ptr_eq(&second_owner.current(), &second));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_readers_retain_coherent_publications() {
        let session = Arc::new(PublicationSessionCore::new(publication(0)));
        let reader_session = session.clone();
        let reader = tokio::spawn(async move {
            for _ in 0..2_000 {
                assert!(reader_session.current().complete);
            }
        });

        for identity in 1..=32 {
            let token = session.begin_build(identity).await.unwrap();
            assert_eq!(
                session
                    .finish_build(token, Ok(publication(identity)))
                    .await
                    .unwrap(),
                SemanticPublicationOutcome::Published
            );
        }

        tokio::time::timeout(Duration::from_secs(2), reader)
            .await
            .expect("readers must not block on publication")
            .unwrap();
    }
}
