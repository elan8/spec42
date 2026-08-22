//! Content-addressed cache foundation for source-derived, non-semantic artifacts.
//!
//! This module provides the storage substrate for the unified cache: typed BLAKE3 identities,
//! the on-disk object store with its envelope format and lock-free atomic publication, capacity
//! management, and the public [`CacheStore`] trait. It does not yet define the concrete artifact
//! payload types (parse outcomes and library indexes/closures) or wire any call site to use it.
//! Immutable semantic publications are deliberately not cache payloads.

pub mod api;
pub mod artifacts;
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

// The typed identities this module keys artifacts by (`ArtifactKey`, `ContentDigest`,
// `RootDigest`, `CanonicalEncoder`) live in the `source_identity` crate. They are deliberately
// not re-exported here: source identity is shared by the semantic layer and the cache layer
// alike, so consumers name its owning crate rather than reaching for it through whichever
// consumer they happen to already import.
