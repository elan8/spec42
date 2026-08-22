#![recursion_limit = "256"]

//! Tokio-actor concurrency wrapper for embedder-owned session state.
//!
//! Gives readers a lock-free, always-immediately-available (possibly stale) snapshot via
//! [`SnapshotHandle`], and gives writers a single-actor mailbox via [`SessionActor`] so
//! in-progress rebuilds never block reads and superseded rebuilds are dropped silently — the
//! same generation/token discipline already proven by `workspace::WorkspaceSession` (composed
//! by embedder state, not reimplemented here) and by `lsp_server`'s `SemanticCoordinator`
//! (`crates/lsp_server/src/workspace/coordinator.rs`), generalized from "one hard-coded
//! lifecycle enum" to "any embedder-owned state struct `M`".
//!
//! [`SemanticPublicationAuthority`] is the production boundary which composes canonical build/cache
//! ownership with the atomic publication barrier. Protocol hosts share it rather than owning
//! private semantic construction or publication policy.
//!
//! This crate deliberately depends on `tokio` and on `workspace`, but not on any
//! protocol/binary-layer crate (`tower-lsp`, `axum`, `rmcp`, `clap`, `lsp_server`) — see
//! `tests/dependency_guardrails.rs`. It stays a shared, protocol-neutral-but-async layer usable
//! by both an LSP server and an HTTP server.

mod actor;
mod semantic_model;
mod snapshot;

pub use actor::{MutatePanicked, Mutation, MutationOutcome, SessionActor, TracksRelink};
pub use semantic_model::{
    PublishedModelSnapshot, SemanticAuthorityBeginError, SemanticAuthorityBuild,
    SemanticAuthorityCompletion, SemanticAuthorityResult, SemanticBuildFailureKind,
    SemanticBuildToken, SemanticPublicationAuthority, SemanticPublicationOutcome,
    SemanticPublicationSession,
};
pub use snapshot::SnapshotHandle;

// Re-exported so callers building `report_job_result` call sites don't need a direct
// `workspace` dependency just for the token type.
pub use workspace::{PublicationToken, RelinkToken};

/// The raw model and the former workspace construction wrapper are deliberately inaccessible.
///
/// ```compile_fail
/// use workspace::{build_semantic_model_from_documents, SemanticModel};
/// ```
pub struct RawSemanticPublicationIsNotPublic;
