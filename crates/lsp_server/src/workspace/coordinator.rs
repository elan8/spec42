use std::sync::Arc;
use tokio::sync::watch;

use super::state::SemanticLifecycle;

/// Token returned by [`SemanticCoordinator::schedule_relink`].
///
/// The async relink task carries this value and passes it back to
/// [`SemanticCoordinator::commit_relink`] to prove it is still the
/// current (non-superseded) relink.  Both `generation` and
/// `snapshot_version` must match the coordinator at commit time.
pub(crate) struct RelinkToken {
    generation: u64,
    snapshot_version: u64,
}

/// State machine for the semantic graph lifecycle.
///
/// All lifecycle transitions go through named methods on this type,
/// which enforce valid transitions (via `debug_assert`) and broadcast
/// each change over a [`watch`] channel so query handlers can wait
/// outside the `RwLock` with zero overhead.
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
    lifecycle: SemanticLifecycle,
    /// Bumped on every transition and on bare `bump_version` calls.
    /// Used as a monotonic "did anything change?" discriminator for
    /// in-flight async tasks.
    version: u64,
    /// Incremented each time a new relink is scheduled.  The newest
    /// token's generation is the only one that will pass
    /// `is_token_current`, so older debounce tasks self-cancel.
    relink_generation: u64,
    lifecycle_tx: Arc<watch::Sender<SemanticLifecycle>>,
}

impl Default for SemanticCoordinator {
    fn default() -> Self {
        let (tx, _rx) = watch::channel(SemanticLifecycle::Cold);
        Self {
            lifecycle: SemanticLifecycle::Cold,
            version: 0,
            relink_generation: 0,
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
        self.lifecycle
    }

    pub(crate) fn version(&self) -> u64 {
        self.version
    }

    /// Resets to `Cold` (server re-initialization).
    pub(crate) fn reset(&mut self) {
        self.lifecycle = SemanticLifecycle::Cold;
        let _ = self.lifecycle_tx.send(SemanticLifecycle::Cold);
        // Version is intentionally not bumped — all in-flight tasks
        // should have already been invalidated by the re-initialize.
    }

    /// `Cold → Indexing` — workspace startup scan begins.
    pub(crate) fn begin_startup(&mut self) {
        debug_assert_eq!(self.lifecycle, SemanticLifecycle::Cold);
        self.transition(SemanticLifecycle::Indexing);
    }

    /// `Cold/Indexing → Ready` — startup complete (with or without files).
    pub(crate) fn complete_startup(&mut self) -> u64 {
        debug_assert!(matches!(
            self.lifecycle,
            SemanticLifecycle::Cold | SemanticLifecycle::Indexing
        ));
        self.transition(SemanticLifecycle::Ready)
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
        debug_assert!(matches!(
            self.lifecycle,
            SemanticLifecycle::Ready | SemanticLifecycle::Reindexing
        ));
        self.relink_generation = self.relink_generation.wrapping_add(1);
        let version = self.transition(SemanticLifecycle::Reindexing);
        RelinkToken {
            generation: self.relink_generation,
            snapshot_version: version,
        }
    }

    /// Returns `true` when this token still represents the current
    /// pending relink (not superseded by a newer edit).
    pub(crate) fn is_token_current(&self, token: &RelinkToken) -> bool {
        self.relink_generation == token.generation
            && self.version == token.snapshot_version
    }

    /// `Reindexing → Ready` — async relink committed.
    ///
    /// Returns `true` if committed successfully, `false` if the token
    /// has been superseded by a newer relink.
    pub(crate) fn commit_relink(&mut self, token: &RelinkToken) -> bool {
        if !self.is_token_current(token) {
            return false;
        }
        debug_assert_eq!(self.lifecycle, SemanticLifecycle::Reindexing);
        self.transition(SemanticLifecycle::Ready);
        true
    }

    /// `* → Reindexing` — library paths changed; full reindex begins.
    pub(crate) fn begin_library_reindex(&mut self) {
        self.transition(SemanticLifecycle::Reindexing);
    }

    /// `Reindexing → Ready` — library reindex (or configuration reindex) done.
    pub(crate) fn complete_reindex(&mut self) -> u64 {
        debug_assert_eq!(self.lifecycle, SemanticLifecycle::Reindexing);
        self.transition(SemanticLifecycle::Ready)
    }

    /// Bumps the version without a lifecycle change.
    ///
    /// Use for events that invalidate in-flight tasks (e.g. a file is
    /// deleted, or the index is updated during startup ingestion) but
    /// do not affect the lifecycle state.
    pub(crate) fn bump_version(&mut self) -> u64 {
        self.version = self.version.wrapping_add(1);
        self.version
    }

    fn transition(&mut self, new: SemanticLifecycle) -> u64 {
        self.lifecycle = new;
        self.version = self.version.wrapping_add(1);
        let _ = self.lifecycle_tx.send(new);
        self.version
    }
}

impl RelinkToken {
    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    #[cfg(test)]
    pub(crate) fn snapshot_version(&self) -> u64 {
        self.snapshot_version
    }
}
