//! Public cache interfaces (plan §9).

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::config::CacheMode;
use source_identity::ArtifactKey;

/// Cacheable source-derived artifacts. Semantic publications are never graph cache entries.
///
/// Each kind is domain-separated into its artifact keys and is also recorded in the verified
/// object envelope, independent of which directory it lives under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ArtifactKind {
    ParseOutcome = 0,
    LibraryIndex = 1,
    LibraryClosure = 2,
}

impl ArtifactKind {
    pub const ALL: [ArtifactKind; 3] = [
        ArtifactKind::ParseOutcome,
        ArtifactKind::LibraryIndex,
        ArtifactKind::LibraryClosure,
    ];

    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(ArtifactKind::ParseOutcome),
            1 => Some(ArtifactKind::LibraryIndex),
            2 => Some(ArtifactKind::LibraryClosure),
            _ => None,
        }
    }
}

/// An artifact's identity: everything the [`ArtifactKey`] must commit. Implementors compute
/// the key using a [`source_identity::CanonicalEncoder`] scoped to `ArtifactKey::DOMAIN` so that
/// artifact-key identities never collide with content or root digests of the same bytes.
pub trait ArtifactIdentity {
    fn artifact_key(&self) -> ArtifactKey;
}

/// Ties a storable artifact type to its identity, kind, and schema version (plan §9).
///
/// `SCHEMA_VERSION` is a repository-owned constant that must be incremented whenever the
/// payload shape or its semantic meaning changes; a mismatched schema version is a coded miss
/// (`CacheMissReason::IncompatibleVersion`), never a best-effort decode.
pub trait CacheArtifact: Serialize + DeserializeOwned {
    type Identity: ArtifactIdentity;

    const KIND: ArtifactKind;
    const SCHEMA_VERSION: u32;

    /// Artifact-specific invariants checked after successful decode and before the value is
    /// returned as a hit. Returning `Err` makes the lookup a miss
    /// (`CacheMissReason::InvariantFailure`) even though the envelope and postcard decode both
    /// succeeded.
    fn validate_invariants(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Why a lookup was a miss (plan §7.5). Every failure path is explicit; none of them return
/// partial data, a default value, or a silently downgraded result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheMissReason {
    /// The store's [`CacheMode`] is `Disabled`; no filesystem access was attempted.
    Disabled,
    /// No object exists at the derived path.
    NotFound,
    /// The envelope or artifact schema version does not match what this process expects.
    IncompatibleVersion { detail: String },
    /// The envelope's expected key does not match the key derived from the current identity.
    KeyMismatch,
    /// The payload's BLAKE3 digest did not match the envelope's recorded digest.
    ChecksumFailure,
    /// The object file is shorter than the envelope declares.
    Truncated,
    /// A declared or actual length exceeded the configured [`super::config::CacheLimits`].
    ResourceLimit { detail: String },
    /// The envelope was well-formed but the payload failed to decode (postcard/zstd error).
    DecodeFailure { detail: String },
    /// Decode succeeded but [`CacheArtifact::validate_invariants`] rejected the value.
    InvariantFailure { detail: String },
    /// A filesystem operation failed for a reason other than "not found".
    IoFailure { detail: String },
}

/// Outcome of a cache lookup. There is no representation of "hit with incomplete data"; a
/// value is only ever returned complete and validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheLookup<T> {
    Hit(T, CacheHitMetadata),
    Miss(CacheMissReason),
}

impl<T> CacheLookup<T> {
    pub fn is_hit(&self) -> bool {
        matches!(self, CacheLookup::Hit(..))
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            CacheLookup::Hit(v, _) => Some(v),
            CacheLookup::Miss(_) => None,
        }
    }
}

/// Filesystem-derived metadata about a validated hit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheHitMetadata {
    pub compressed_len: u64,
    pub uncompressed_len: u64,
    /// Whether this lookup performed a best-effort access-time touch of the object.
    pub touched: bool,
}

/// Outcome of a [`CacheStore::put`] call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheStoreOutcome {
    /// The store is disabled; no write was attempted.
    Disabled,
    /// The object was published (or an identical object already existed under the same key).
    Stored {
        compressed_len: u64,
        uncompressed_len: u64,
        /// Set when the post-write budget check triggered a prune.
        pruned: Option<CacheMaintenanceReport>,
    },
    /// The write did not complete; the cache remains exactly as it was before the call.
    Failed(CacheWriteFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheWriteFailure {
    Encode { detail: String },
    Io { detail: String },
}

/// Per-kind size/count summary within [`CacheStatus`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CacheKindStats {
    pub object_count: u64,
    pub total_bytes: u64,
}

/// Snapshot report produced by [`CacheStore::status`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheStatus {
    pub mode: CacheMode,
    pub root: PathBuf,
    pub max_bytes: u64,
    pub prune_target_bytes: u64,
    pub total_bytes: u64,
    pub object_count: u64,
    pub per_kind: BTreeMap<ArtifactKind, CacheKindStats>,
    pub over_budget: bool,
    /// Objects observed during the scan whose envelope could not be read (e.g. truncated,
    /// unreadable). They still count toward `total_bytes`/`object_count` but not `per_kind`.
    pub unreadable_objects: u64,
}

/// Report produced by [`CacheStore::prune`] and [`CacheStore::clear`] (plan §7.4, §8).
///
/// These are filesystem-snapshot, best-effort reports, not transactions: concurrent writers or
/// readers may change the observed state before or after the call returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CacheMaintenanceReport {
    pub objects_removed: u64,
    pub bytes_reclaimed: u64,
    pub objects_remaining: u64,
    pub bytes_remaining: u64,
    pub tmp_files_reaped: u64,
    /// True if deletion failures or races left the store observably over budget after this
    /// maintenance pass. Never fails the caller; this is a report field, not an error.
    pub over_budget: bool,
}

/// The unified cache store contract (plan §9).
pub trait CacheStore {
    fn get<T: CacheArtifact>(&self, identity: &T::Identity) -> CacheLookup<T>;
    fn put<T: CacheArtifact>(&self, identity: &T::Identity, value: &T) -> CacheStoreOutcome;
    fn status(&self) -> CacheStatus;
    fn prune(&self) -> CacheMaintenanceReport;
    fn clear(&self) -> CacheMaintenanceReport;
}
