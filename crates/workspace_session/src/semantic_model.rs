//! Atomic publication owner for immutable [`workspace::SemanticModel`] values.
//!
//! A build runs against an immutable source snapshot outside this owner.  The owner only admits
//! a completed model at its publication barrier, after checking both the owner-scoped relink token
//! and the model identity captured when the build started.  Failed, cancelled, stale, and
//! out-of-order builds therefore leave the last coherent model available to readers.

use std::sync::Arc;

use workspace::{
    SemanticBuildRequest, SemanticCompleteness, SemanticModel, SemanticModelIdentity,
    SessionLifecycle, WorkspaceSession,
};

use crate::{MutatePanicked, Mutation, SessionActor, SnapshotHandle, TracksRelink};

#[derive(Clone)]
struct SemanticOwnerState {
    session: WorkspaceSession,
    model: Arc<SemanticModel>,
}

impl TracksRelink for SemanticOwnerState {
    fn is_token_current(&self, token: &workspace::RelinkToken) -> bool {
        self.session.is_token_current(token)
    }

    fn rekey_for_actor(&mut self) {
        self.session.rekey_for_owner();
    }
}

/// Token carried by a background whole-model build.
///
/// The identity is part of the token rather than inferred from the eventual result, so a caller
/// cannot accidentally publish a model built from inputs other than the snapshot it scheduled.
#[derive(Debug, Clone)]
pub struct SemanticBuildToken {
    relink: workspace::RelinkToken,
    identity: SemanticModelIdentity,
}

impl SemanticBuildToken {
    pub fn identity(&self) -> &SemanticModelIdentity {
        &self.identity
    }

    pub fn relink(&self) -> &workspace::RelinkToken {
        &self.relink
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
pub struct SemanticModelSnapshot {
    model: Arc<SemanticModel>,
}

impl SemanticModelSnapshot {
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }

    pub fn into_model(self) -> Arc<SemanticModel> {
        self.model
    }
}

/// Owns the current immutable semantic publication and serializes its publication barrier.
///
/// Readers clone an [`Arc`] to the model without waiting for a build.  A build result is visible
/// only after the actor has checked the complete identity and owner-scoped token.  The owner does
/// not expose mutable graph state or a partial model.
#[derive(Clone)]
pub struct SemanticModelSession {
    actor: SessionActor<SemanticOwnerState>,
    snapshot: SnapshotHandle<SemanticOwnerState>,
}

impl SemanticModelSession {
    /// Starts a ready session with an already settled model.
    pub fn new(initial: Arc<SemanticModel>) -> Self {
        assert_eq!(
            initial.completeness(),
            SemanticCompleteness::Complete,
            "the initial semantic publication must be complete"
        );
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let (actor, snapshot) = SessionActor::spawn(SemanticOwnerState {
            session,
            model: initial,
        });
        Self { actor, snapshot }
    }

    /// Returns an immutable model `Arc` immediately.  A reader may retain it while a replacement
    /// build is running or after a newer model has been published.
    pub fn current(&self) -> Arc<SemanticModel> {
        self.snapshot.current().model.clone()
    }

    /// Returns a reader-owned immutable publication wrapper.
    pub fn snapshot(&self) -> SemanticModelSnapshot {
        SemanticModelSnapshot {
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
        request: &SemanticBuildRequest,
    ) -> Result<SemanticBuildToken, MutatePanicked> {
        let identity = request.identity();
        self.actor
            .mutate(move |state| SemanticBuildToken {
                relink: state.session.schedule_relink(),
                identity,
            })
            .await
    }

    /// Completes a build at the publication barrier.
    ///
    /// Errors and cancellation are explicit outcomes and retain the previous model.  A stale
    /// result is ignored without changing the newer build's lifecycle.  A current result that is
    /// incomplete or has a different identity is rejected and leaves the previous model in place.
    pub async fn finish_build(
        &self,
        token: SemanticBuildToken,
        result: Result<Arc<SemanticModel>, SemanticBuildFailureKind>,
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
                    Ok(model) if model.completeness() != SemanticCompleteness::Complete => {
                        assert!(state.session.commit_relink(&token.relink));
                        SemanticPublicationOutcome::DiscardedIncomplete
                    }
                    Ok(model) if model.identity() != &token.identity => {
                        assert!(state.session.commit_relink(&token.relink));
                        SemanticPublicationOutcome::DiscardedIdentityMismatch
                    }
                    Ok(model) => {
                        state.model = model;
                        assert!(state.session.commit_relink(&token.relink));
                        SemanticPublicationOutcome::Published
                    }
                };
                Mutation::Changed(outcome)
            })
            .await
            .map(|outcome| outcome.value)
    }

    /// Returns the owner lifecycle associated with the current publication.
    pub fn lifecycle(&self) -> SessionLifecycle {
        self.snapshot.current().session.lifecycle()
    }
}

/// Explicit outcome for a build that did not produce a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticBuildFailureKind {
    Failed,
    Cancelled,
}
