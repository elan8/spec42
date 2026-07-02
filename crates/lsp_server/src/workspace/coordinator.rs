use std::sync::Arc;
use tokio::sync::watch;

use super::state::SemanticLifecycle;

fn to_semantic_lifecycle(lifecycle: workspace::SessionLifecycle) -> SemanticLifecycle {
    match lifecycle {
        workspace::SessionLifecycle::Cold => SemanticLifecycle::Cold,
        workspace::SessionLifecycle::Indexing => SemanticLifecycle::Indexing,
        workspace::SessionLifecycle::Ready => SemanticLifecycle::Ready,
        workspace::SessionLifecycle::Reindexing => SemanticLifecycle::Reindexing,
    }
}

/// Token returned by [`SemanticCoordinator::schedule_relink`].
///
/// The async relink task carries this value and passes it back to
/// [`SemanticCoordinator::commit_relink`] to prove it is still the
/// current (non-superseded) relink. Wraps `workspace::RelinkToken`, which
/// carries the actual generation/version bookkeeping — see
/// `crates/workspace/src/session.rs`.
pub(crate) struct RelinkToken {
    inner: workspace::RelinkToken,
}

impl RelinkToken {
    pub(crate) fn generation(&self) -> u64 {
        self.inner.generation()
    }
}

/// State machine for the semantic graph lifecycle.
///
/// All lifecycle transitions delegate their generation/version bookkeeping to
/// `workspace::WorkspaceSession` (see
/// `docs/engineering/TIER2-LSP-WORKSPACE-CONSOLIDATION.md`, Tier 2 Phase 2). This type
/// now only owns the LSP-specific pieces `workspace` deliberately doesn't provide (it
/// stays protocol/runtime-neutral, see `crates/workspace/tests/dependency_guardrails.rs`):
/// the `tokio::sync::watch` broadcast channel, and translating between
/// `workspace::SessionLifecycle` and this crate's own `SemanticLifecycle` (kept as a
/// separate type since it carries LSP-specific inherent methods like
/// `supports_semantic_queries`).
///
/// ## Valid transitions
///
/// ```text
/// Cold → Indexing             begin_startup()
/// Cold/Indexing → Ready       complete_startup()
/// Ready → Reindexing          schedule_relink()
/// Reindexing → Reindexing     schedule_relink()   (newer edit supersedes)
/// Reindexing → Ready          commit_relink()
/// * → Reindexing              begin_library_reindex()
/// Reindexing → Ready          complete_reindex()
/// * → Cold                    reset()
/// ```
///
/// ## Waiting without the lock
///
/// Call [`subscribe`] once at startup to obtain a
/// `watch::Receiver<SemanticLifecycle>`.  Store it outside the
/// `RwLock` and call `receiver.wait_for(|&l| l != Reindexing)` in
/// any query handler — it wakes the instant `commit_relink` fires,
/// with no polling and no lock acquisition.
///
/// [`subscribe`]: SemanticCoordinator::subscribe
pub(crate) struct SemanticCoordinator {
    session: workspace::WorkspaceSession,
    lifecycle_tx: Arc<watch::Sender<SemanticLifecycle>>,
}

impl Default for SemanticCoordinator {
    fn default() -> Self {
        let (tx, _rx) = watch::channel(SemanticLifecycle::Cold);
        Self {
            session: workspace::WorkspaceSession::new(),
            lifecycle_tx: Arc::new(tx),
        }
    }
}

impl SemanticCoordinator {
    /// Returns a receiver that is notified on every lifecycle transition.
    ///
    /// Clone the receiver and call `wait_for` on it in query handlers
    /// that need to stall while a relink is in flight.
    pub(crate) fn subscribe(&self) -> watch::Receiver<SemanticLifecycle> {
        self.lifecycle_tx.subscribe()
    }

    pub(crate) fn lifecycle(&self) -> SemanticLifecycle {
        to_semantic_lifecycle(self.session.lifecycle())
    }

    pub(crate) fn version(&self) -> u64 {
        self.session.version()
    }

    /// Resets to `Cold` (server re-initialization).
    pub(crate) fn reset(&mut self) {
        self.session.reset();
        self.publish();
        // Version is intentionally not bumped — all in-flight tasks
        // should have already been invalidated by the re-initialize.
    }

    /// `Cold → Indexing` — workspace startup scan begins.
    pub(crate) fn begin_startup(&mut self) {
        self.session.begin_startup();
        self.publish();
    }

    /// `Cold/Indexing → Ready` — startup complete (with or without files).
    pub(crate) fn complete_startup(&mut self) -> u64 {
        let version = self.session.complete_startup();
        self.publish();
        version
    }

    /// `Ready/Reindexing → Reindexing` — a file changed; schedule async relink.
    ///
    /// Returns a [`RelinkToken`] the caller passes to
    /// [`commit_relink`] once the background rebuild finishes.
    /// Bumps the version and increments the relink generation so any
    /// previously-issued tokens are automatically invalidated.
    ///
    /// [`commit_relink`]: SemanticCoordinator::commit_relink
    pub(crate) fn schedule_relink(&mut self) -> RelinkToken {
        let inner = self.session.schedule_relink();
        self.publish();
        RelinkToken { inner }
    }

    /// Returns `true` when this token still represents the current
    /// pending relink (not superseded by a newer edit).
    pub(crate) fn is_token_current(&self, token: &RelinkToken) -> bool {
        self.session.is_token_current(&token.inner)
    }

    /// `Reindexing → Ready` — async relink committed.
    ///
    /// Returns `true` if committed successfully, `false` if the token
    /// has been superseded by a newer relink.
    pub(crate) fn commit_relink(&mut self, token: &RelinkToken) -> bool {
        let committed = self.session.commit_relink(&token.inner);
        if committed {
            self.publish();
        }
        committed
    }

    /// `* → Reindexing` — library paths changed; full reindex begins.
    pub(crate) fn begin_library_reindex(&mut self) {
        self.session.begin_library_reindex();
        self.publish();
    }

    /// `Reindexing → Ready` — library reindex (or configuration reindex) done.
    pub(crate) fn complete_reindex(&mut self) -> u64 {
        let version = self.session.complete_reindex();
        self.publish();
        version
    }

    /// Bumps the version without a lifecycle change.
    ///
    /// Use for events that invalidate in-flight tasks (e.g. a file is
    /// deleted, or the index is updated during startup ingestion) but
    /// do not affect the lifecycle state.
    pub(crate) fn bump_version(&mut self) -> u64 {
        self.session.bump_version()
    }

    fn publish(&self) {
        let _ = self.lifecycle_tx.send(self.lifecycle());
    }
}
