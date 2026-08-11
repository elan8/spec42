//! Unified semantic cache foundation (`UNIFY_CACHE_PLAN.md` §5.1, §6.1, §7, §8, §9).
//!
//! This module provides the storage substrate for the unified cache: typed BLAKE3 identities,
//! the on-disk object store with its envelope format and lock-free atomic publication, capacity
//! management, and the public [`CacheStore`] trait. It does not yet define the concrete artifact
//! payload types (parse outcomes, library indexes/closures, semantic graphs) or wire any call
//! site to use it — those are later slices of the plan's cutover.

pub mod api;
pub mod config;
pub mod envelope;
pub mod store;

pub use api::{
    ArtifactIdentity, ArtifactKind, CacheArtifact, CacheHitMetadata, CacheKindStats, CacheLookup,
    CacheMaintenanceReport, CacheMissReason, CacheStatus, CacheStore, CacheStoreOutcome,
    CacheWriteFailure,
};
pub use config::{CacheConfig, CacheLimits, CacheMode};
pub use store::FileCacheStore;

// Typed BLAKE3 source/artifact identities (plan §5.1) are a pure semantic-layer value type and
// live in `sysml_model` so `SysmlDocument` and other semantic-layer types can use them without
// `sysml_model` depending on `workspace` (dependencies only flow workspace -> sysml_model).
// Re-exported here so existing `workspace::cache::{Blake3Digest, ...}` call sites are unaffected.
pub use sysml_model::{
    ArtifactKey, Blake3Digest, CanonicalEncoder, ContentDigest, DigestParseError, RootDigest,
};
