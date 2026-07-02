//! Relink coordination for long-lived workspace consumers (e.g. a live LSP server) that
//! need to keep serving a snapshot to readers while a relink is in progress, and to
//! discard a relink superseded by a newer edit.
//!
//! This generalizes the token/generation state-machine pattern already proven in
//! production by `lsp_server`'s `SemanticCoordinator`
//! (`crates/lsp_server/src/workspace/coordinator.rs`). See
//! `docs/engineering/TIER2-LSP-WORKSPACE-CONSOLIDATION.md` for the migration plan.
//!
//! `workspace` is deliberately protocol/runtime-neutral (see
//! `tests/dependency_guardrails.rs` — no `tokio`, `clap`, `axum`, etc.), so this type has
//! no async or subscription API of its own: it's a plain synchronous state machine.
//! Consumers that need to *wait* on a transition without polling (e.g. `lsp_server`, which
//! already owns `tokio`) should layer their own notification channel around
//! [`WorkspaceSession::lifecycle`] the same way `SemanticCoordinator` already wraps a
//! `tokio::sync::watch` today.
//!
//! **Phase 1 status:** this type is added but not yet used by any caller. `workspace`
//! crate's existing synchronous API (`Spec42Engine::load_workspace`/`update_snapshot`) is
//! unaffected; nothing here changes behavior for CLI/MCP/batch callers.

/// Lifecycle state tracked by a [`WorkspaceSession`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionLifecycle {
    #[default]
    Cold,
    Indexing,
    Ready,
    Reindexing,
}

/// Token returned by [`WorkspaceSession::schedule_relink`].
///
/// The async relink task carries this value and passes it back to
/// [`WorkspaceSession::commit_relink`] to prove it is still the current
/// (non-superseded) relink. Both the relink generation and the lifecycle version must
/// match the session at commit time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelinkToken {
    generation: u64,
    version: u64,
}

impl RelinkToken {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    #[cfg(test)]
    fn version(&self) -> u64 {
        self.version
    }
}

/// State machine coordinating async, cancellable relinks of a workspace snapshot.
///
/// `WorkspaceSession` does not itself hold the snapshot — it only tracks *when* it is
/// safe to commit one. Callers own the actual snapshot storage (e.g.
/// `Arc<HostWorkspaceSnapshot>`) and its synchronization; this type only answers "is
/// this relink still current?" and "what lifecycle state are we in?". Like
/// `SemanticCoordinator`, it is not internally synchronized — wrap it in whatever lock
/// (or actor/single-writer discipline) already guards the caller's snapshot state.
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
#[derive(Debug, Default)]
pub struct WorkspaceSession {
    lifecycle: SessionLifecycle,
    /// Bumped on every transition and on bare `bump_version` calls. Used as a
    /// monotonic "did anything change?" discriminator for in-flight async tasks.
    version: u64,
    /// Incremented each time a new relink is scheduled. Only the newest token's
    /// generation passes `is_token_current`, so superseded relinks self-cancel.
    relink_generation: u64,
}

impl WorkspaceSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lifecycle(&self) -> SessionLifecycle {
        self.lifecycle
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    /// Resets to `Cold` (e.g. workspace re-initialization). Any tokens issued before
    /// reset are permanently stale — the relink generation is intentionally *not*
    /// bumped, since a `Cold` lifecycle already invalidates every in-flight consumer.
    pub fn reset(&mut self) {
        self.lifecycle = SessionLifecycle::Cold;
    }

    /// `Cold → Indexing` — workspace startup scan begins.
    pub fn begin_startup(&mut self) {
        debug_assert_eq!(self.lifecycle, SessionLifecycle::Cold);
        self.transition(SessionLifecycle::Indexing);
    }

    /// `Cold/Indexing → Ready` — startup complete (with or without files).
    pub fn complete_startup(&mut self) -> u64 {
        debug_assert!(matches!(
            self.lifecycle,
            SessionLifecycle::Cold | SessionLifecycle::Indexing
        ));
        self.transition(SessionLifecycle::Ready)
    }

    /// `Ready/Reindexing → Reindexing` — a document changed; schedule an async relink.
    ///
    /// Returns a [`RelinkToken`] the caller passes to [`Self::commit_relink`] once the
    /// background rebuild finishes. Bumps the relink generation so any previously
    /// issued token is automatically invalidated.
    pub fn schedule_relink(&mut self) -> RelinkToken {
        debug_assert!(matches!(
            self.lifecycle,
            SessionLifecycle::Ready | SessionLifecycle::Reindexing
        ));
        self.relink_generation = self.relink_generation.wrapping_add(1);
        let version = self.transition(SessionLifecycle::Reindexing);
        RelinkToken {
            generation: self.relink_generation,
            version,
        }
    }

    /// Returns `true` when `token` still represents the current pending relink (i.e.
    /// has not been superseded by a newer edit).
    pub fn is_token_current(&self, token: &RelinkToken) -> bool {
        self.relink_generation == token.generation && self.version == token.version
    }

    /// `Reindexing → Ready` — async relink committed.
    ///
    /// Returns `true` if committed, `false` if `token` has been superseded by a newer
    /// relink (the caller should discard its computed snapshot in that case).
    pub fn commit_relink(&mut self, token: &RelinkToken) -> bool {
        // `reset()` intentionally doesn't bump the relink generation/version (a `Cold`
        // lifecycle already invalidates every in-flight consumer by itself), so a
        // token issued before a `reset()` can still look "current" by generation and
        // version alone. Checking `lifecycle == Reindexing` here catches that case
        // (and a double-commit of the same token) without relying on callers never
        // committing after a reset.
        if !self.is_token_current(token) || self.lifecycle != SessionLifecycle::Reindexing {
            return false;
        }
        self.transition(SessionLifecycle::Ready);
        true
    }

    /// `* → Reindexing` — library paths changed; full reindex begins.
    pub fn begin_library_reindex(&mut self) {
        self.transition(SessionLifecycle::Reindexing);
    }

    /// `Reindexing → Ready` — library reindex (or configuration reindex) done.
    pub fn complete_reindex(&mut self) -> u64 {
        debug_assert_eq!(self.lifecycle, SessionLifecycle::Reindexing);
        self.transition(SessionLifecycle::Ready)
    }

    /// Bumps the version without a lifecycle change. Use for events that invalidate
    /// in-flight tasks (e.g. a document is removed) but don't affect the lifecycle
    /// state.
    pub fn bump_version(&mut self) -> u64 {
        self.version = self.version.wrapping_add(1);
        self.version
    }

    fn transition(&mut self, new: SessionLifecycle) -> u64 {
        self.lifecycle = new;
        self.version = self.version.wrapping_add(1);
        self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_cold() {
        let session = WorkspaceSession::new();
        assert_eq!(session.lifecycle(), SessionLifecycle::Cold);
        assert_eq!(session.version(), 0);
    }

    #[test]
    fn startup_transitions_cold_indexing_ready() {
        let mut session = WorkspaceSession::new();
        session.begin_startup();
        assert_eq!(session.lifecycle(), SessionLifecycle::Indexing);
        let version = session.complete_startup();
        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
        assert_eq!(version, session.version());
    }

    #[test]
    fn complete_startup_allowed_directly_from_cold() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
    }

    #[test]
    fn schedule_relink_moves_to_reindexing_and_returns_current_token() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let token = session.schedule_relink();
        assert_eq!(session.lifecycle(), SessionLifecycle::Reindexing);
        assert!(session.is_token_current(&token));
    }

    #[test]
    fn newer_relink_invalidates_older_token() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let stale = session.schedule_relink();
        let fresh = session.schedule_relink();
        assert!(!session.is_token_current(&stale));
        assert!(session.is_token_current(&fresh));
        assert_ne!(stale.generation(), fresh.generation());
    }

    #[test]
    fn commit_relink_with_current_token_succeeds_and_returns_to_ready() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let token = session.schedule_relink();
        assert!(session.commit_relink(&token));
        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
    }

    #[test]
    fn commit_relink_with_stale_token_fails_and_leaves_state_untouched() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let stale = session.schedule_relink();
        let _fresh = session.schedule_relink();
        assert!(!session.commit_relink(&stale));
        // Still reindexing, waiting on the fresh token.
        assert_eq!(session.lifecycle(), SessionLifecycle::Reindexing);
    }

    #[test]
    fn reset_returns_to_cold_and_invalidates_pending_relinks() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let token = session.schedule_relink();
        session.reset();
        assert_eq!(session.lifecycle(), SessionLifecycle::Cold);
        assert!(!session.commit_relink(&token));
    }

    #[test]
    fn library_reindex_round_trip() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        session.begin_library_reindex();
        assert_eq!(session.lifecycle(), SessionLifecycle::Reindexing);
        session.complete_reindex();
        assert_eq!(session.lifecycle(), SessionLifecycle::Ready);
    }

    #[test]
    fn bump_version_does_not_change_lifecycle() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let before = session.lifecycle();
        let v1 = session.bump_version();
        let v2 = session.bump_version();
        assert_eq!(session.lifecycle(), before);
        assert!(v2 > v1);
    }

    #[test]
    fn bump_version_invalidates_outstanding_tokens() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let token = session.schedule_relink();
        session.bump_version();
        assert!(!session.is_token_current(&token));
    }

    #[test]
    fn relink_token_carries_generation_and_version() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let token = session.schedule_relink();
        assert_eq!(token.generation(), 1);
        assert_eq!(token.version(), session.version());
    }
}
