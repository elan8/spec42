//! Tokio-actor concurrency wrapper for embedder-owned session state.
//!
//! Gives readers a lock-free, always-immediately-available (possibly stale) snapshot via
//! [`SnapshotHandle`], and gives writers a single-actor mailbox via [`SessionActor`] so
//! in-progress rebuilds never block reads and superseded rebuilds are dropped silently. The
//! state type `M` and its supersession token are the embedder's; this crate knows nothing about
//! what `M` holds.
//!
//! This crate deliberately depends on `tokio` and on no SysML crate and no protocol crate — see
//! `tests/dependency_guardrails.rs`. It is a shared, protocol-neutral-but-async layer usable by
//! an LSP server and an HTTP server alike.

mod actor;
mod snapshot;

pub use actor::{MutatePanicked, Mutation, MutationOutcome, SessionActor, TracksRelink};
pub use snapshot::SnapshotHandle;
