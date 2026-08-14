//! Relink coordination for long-lived workspace consumers (e.g. a live LSP server) that
//! need to keep serving a snapshot to readers while a relink is in progress, and to
//! discard a relink superseded by a newer edit.
//!
//! This generalizes the token/generation state-machine pattern already proven in
//! production by `lsp_server`'s `SemanticCoordinator`
//! (`crates/lsp_server/src/workspace/coordinator.rs`). See
//! Migration history lives in git.
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
    Closed,
}

/// Owner-scoped identity for a coherent workspace publication.
///
/// Background work captures this value together with the immutable inputs it reads.  It may
/// publish only when [`WorkspaceSession::is_publication_current`] still accepts it.  The owner
/// component prevents a revision from one workspace session being mistaken for a revision from
/// another session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationToken {
    owner: u64,
    version: u64,
}

impl PublicationToken {
    /// Monotonic version within this token's owning session. This is useful for cache keys and
    /// observability, but is not an identity without the owner retained by this token.
    pub fn version(&self) -> u64 {
        self.version
    }
}

/// Token returned by [`WorkspaceSession::schedule_relink`].
///
/// The async relink task carries this value and passes it back to
/// [`WorkspaceSession::commit_relink`] to prove it is still the current
/// (non-superseded) relink. Both the relink generation and the lifecycle version must
/// match the session at commit time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelinkToken {
    publication: PublicationToken,
    generation: u64,
}

impl RelinkToken {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// The owner-scoped publication identity captured when this relink was scheduled.
    pub fn publication(&self) -> PublicationToken {
        self.publication
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
/// Cold/Indexing/Ready/Reindexing → Cold  reset()
/// * → Closed                  close() (terminal)
/// ```
#[derive(Debug, Clone)]
pub struct WorkspaceSession {
    owner: u64,
    lifecycle: SessionLifecycle,
    /// Bumped on every transition and on bare `bump_version` calls. Used as a
    /// monotonic "did anything change?" discriminator for in-flight async tasks.
    version: u64,
    /// Incremented each time a new relink is scheduled. Only the newest token's
    /// generation passes `is_token_current`, so superseded relinks self-cancel.
    relink_generation: u64,
}

impl Default for WorkspaceSession {
    fn default() -> Self {
        Self {
            owner: next_session_owner(),
            lifecycle: SessionLifecycle::Cold,
            version: 0,
            relink_generation: 0,
        }
    }
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

    /// Captures the identity of the complete state currently published by this session.
    ///
    /// Call this immediately after taking an immutable input snapshot for background work, and
    /// use [`Self::is_publication_current`] at the actor's publication barrier.
    pub fn publication(&self) -> PublicationToken {
        PublicationToken {
            owner: self.owner,
            version: self.version,
        }
    }

    /// Returns whether `token` still names the complete state currently owned by this session.
    pub fn is_publication_current(&self, token: &PublicationToken) -> bool {
        self.owner == token.owner
            && self.version == token.version
            && self.lifecycle != SessionLifecycle::Closed
    }

    /// Gives a newly spawned actor a distinct owner identity, invalidating tokens inherited
    /// from a cloned seed state.
    pub fn rekey_for_owner(&mut self) {
        self.owner = next_session_owner();
    }

    /// Resets to `Cold` (e.g. workspace re-initialization). This invalidates every
    /// outstanding publication and relink token.
    pub fn reset(&mut self) {
        self.transition(SessionLifecycle::Cold);
    }

    /// Permanently closes this session. All outstanding publication and relink tokens become
    /// stale, and a closed session cannot accept a later background result.
    pub fn close(&mut self) {
        self.transition(SessionLifecycle::Closed);
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
        self.relink_generation = increment(self.relink_generation, "relink generation");
        self.transition(SessionLifecycle::Reindexing);
        RelinkToken {
            publication: self.publication(),
            generation: self.relink_generation,
        }
    }

    /// Returns `true` when `token` still represents the current pending relink (i.e.
    /// has not been superseded by a newer edit).
    pub fn is_token_current(&self, token: &RelinkToken) -> bool {
        self.relink_generation == token.generation
            && self.is_publication_current(&token.publication)
            && self.lifecycle == SessionLifecycle::Reindexing
    }

    /// `Reindexing → Ready` — async relink committed.
    ///
    /// Returns `true` if committed, `false` if `token` has been superseded by a newer
    /// relink (the caller should discard its computed snapshot in that case).
    pub fn commit_relink(&mut self, token: &RelinkToken) -> bool {
        if !self.is_token_current(token) {
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
        self.version = increment(self.version, "publication version");
        self.version
    }

    fn transition(&mut self, new: SessionLifecycle) -> u64 {
        assert!(
            self.lifecycle != SessionLifecycle::Closed || new == SessionLifecycle::Closed,
            "a closed workspace session cannot be reopened"
        );
        self.lifecycle = new;
        self.version = increment(self.version, "publication version");
        self.version
    }
}

fn increment(value: u64, label: &str) -> u64 {
    value
        .checked_add(1)
        .unwrap_or_else(|| panic!("workspace session {label} exhausted"))
}

fn next_session_owner() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_SESSION_OWNER: AtomicU64 = AtomicU64::new(1);
    NEXT_SESSION_OWNER
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |owner| {
            owner.checked_add(1)
        })
        .expect("workspace session owner identities exhausted")
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
        assert!(!session.is_token_current(&token));
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
    fn relink_token_carries_generation_and_owner_scoped_publication() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let token = session.schedule_relink();
        assert_eq!(token.generation(), 1);
        assert_eq!(token.publication(), session.publication());
    }

    #[test]
    fn publication_from_another_session_cannot_commit_here() {
        let mut first = WorkspaceSession::new();
        first.complete_startup();
        let first_token = first.schedule_relink();

        let mut second = WorkspaceSession::new();
        second.complete_startup();
        let second_token = second.schedule_relink();

        assert_eq!(first_token.generation(), second_token.generation());
        assert_ne!(first_token.publication(), second_token.publication());
        assert!(!second.is_token_current(&first_token));
        assert!(!second.commit_relink(&first_token));
        assert!(second.is_token_current(&second_token));
    }

    #[test]
    fn close_invalidates_all_outstanding_publication_work() {
        let mut session = WorkspaceSession::new();
        session.complete_startup();
        let publication = session.publication();
        let relink = session.schedule_relink();

        session.close();

        assert_eq!(session.lifecycle(), SessionLifecycle::Closed);
        assert!(!session.is_publication_current(&publication));
        assert!(!session.is_token_current(&relink));
        assert!(!session.commit_relink(&relink));
    }

    #[test]
    #[should_panic(expected = "cannot be reopened")]
    fn closed_session_cannot_be_reset_or_restarted() {
        let mut session = WorkspaceSession::new();
        session.close();
        session.reset();
    }
}
