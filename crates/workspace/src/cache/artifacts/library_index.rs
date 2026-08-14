//! `LibraryIndex` cache artifact (`UNIFY_CACHE_PLAN.md` §6.3).
//!
//! Key inputs: ordered library-root content manifests expressed as root slot, relative path, and
//! content digest — deliberately without absolute install paths, so the index stays reachable
//! when identical library content is relocated to a different absolute root; plus the
//! package-index schema/algorithm version and the parser-independent lexical indexing policy.
//!
//! Payload: the package/import/type-reference facts closure resolution needs, the relative file
//! identities and their digests, and an explicit malformed/unsupported status. This module does
//! not implement a lexer or wire any producer — nothing in production builds a `LibraryIndex`
//! yet (that is plan step 5's `SemanticBuildService` routing). It defines the artifact's shape
//! and key so that work can build on a stable contract, per `AGENTS.md`'s "one canonical
//! derivation owner" and "state every prerequisite before caching" rules.
//!
//! `crates/workspace/src/catalog.rs::hash_package_roots` already assigns positional library-root
//! slots (`u32` index over `package_roots`) when building `SourceManifestEntry`s; this artifact's
//! `root_slot` fields are meant to line up with that same positional convention once a producer
//! is wired.

use serde::{Deserialize, Serialize};

use source_identity::{ArtifactKey, CanonicalEncoder, ContentDigest};

use crate::cache::{ArtifactIdentity, ArtifactKind, CacheArtifact};

/// Schema version of the [`LibraryIndex`] payload shape.
pub const LIBRARY_INDEX_SCHEMA_VERSION: u32 = 1;

/// Version of the parser-independent lexical indexing algorithm used to derive package/import/
/// type-reference facts from library file content. Bump whenever the indexing policy itself
/// changes (e.g. which constructs are recognized), independent of the payload schema.
pub const LIBRARY_INDEX_ALGORITHM_VERSION: u32 = 1;

/// One library file's relative, portable identity (plan §6.3): no absolute install path, only a
/// root slot (matching the ordered manifest's position) and a path relative to that root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryFileIdentity {
    pub root_slot: u32,
    pub relative_path: String,
    pub content_digest: ContentDigest,
}

impl LibraryFileIdentity {
    const DOMAIN: &'static str = "spec42.cache.library_file_identity.v1";

    fn leaf_digest(&self) -> [u8; 32] {
        let mut enc = CanonicalEncoder::new(Self::DOMAIN);
        enc.field_u64(self.root_slot as u64);
        enc.field(self.relative_path.as_bytes());
        enc.field(self.content_digest.as_bytes());
        *enc.finish().as_bytes()
    }
}

/// Everything the [`ArtifactKey`] for a [`LibraryIndex`] must commit (plan §6.3).
///
/// `root_manifest[slot]` holds the files admitted from library-root `slot`, in configured
/// precedence order; the outer vector is never resorted. Each inner vector must already be
/// sorted by `relative_path` (the constructor enforces this) so that key derivation and the
/// selected-file ordering are both deterministic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryIndexIdentity {
    root_manifest: Vec<Vec<LibraryFileIdentity>>,
}

impl LibraryIndexIdentity {
    /// Builds an identity from ordered per-root file lists, sorting each root's files by
    /// relative path for determinism. Root (outer) order is preserved exactly as supplied.
    pub fn new(mut root_manifest: Vec<Vec<LibraryFileIdentity>>) -> Self {
        for root in &mut root_manifest {
            root.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        }
        Self { root_manifest }
    }

    pub fn root_manifest(&self) -> &[Vec<LibraryFileIdentity>] {
        &self.root_manifest
    }
}

impl ArtifactIdentity for LibraryIndexIdentity {
    fn artifact_key(&self) -> ArtifactKey {
        let mut enc = CanonicalEncoder::new(ArtifactKey::DOMAIN);
        enc.field(b"library-index.v1");
        enc.field_u64(self.root_manifest.len() as u64);
        for (slot, files) in self.root_manifest.iter().enumerate() {
            enc.field_u64(slot as u64);
            enc.field_u64(files.len() as u64);
            for file in files {
                enc.field(&file.leaf_digest());
            }
        }
        enc.field_u64(LIBRARY_INDEX_SCHEMA_VERSION as u64);
        enc.field_u64(LIBRARY_INDEX_ALGORITHM_VERSION as u64);
        ArtifactKey::from_encoder(&enc)
    }
}

/// Whether indexing a library root succeeded (plan §6.3): an explicit status, never a silently
/// empty/defaulted fact set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LibraryIndexStatus {
    Ok,
    /// Library content could not be indexed at all (e.g. unreadable/invalid encoding).
    Malformed {
        detail: String,
    },
    /// Library content used a construct the lexical indexing policy does not yet recognize.
    /// Distinct from `Malformed`: the content is valid, just outside this policy version's
    /// coverage.
    Unsupported {
        detail: String,
    },
}

/// A package/namespace declaration fact, keyed to the file that declared it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryPackageFact {
    pub qualified_name: String,
    pub root_slot: u32,
    pub relative_path: String,
}

/// An import statement fact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryImportFact {
    pub root_slot: u32,
    pub relative_path: String,
    pub imported_qualified_name: String,
    pub is_wildcard: bool,
}

/// A type-reference fact: some file in the library referenced a type by name. Closure
/// resolution uses this to decide whether a library file must be admitted without needing a
/// full semantic parse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryTypeReferenceFact {
    pub root_slot: u32,
    pub relative_path: String,
    pub referenced_name: String,
}

/// The `LibraryIndex` payload (plan §6.3). Deliberately holds only the package/import/
/// type-reference facts closure resolution needs — it must not grow into a second semantic
/// parser; facts used only after parsing remain owned by the semantic pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryIndex {
    pub status: LibraryIndexStatus,
    pub files: Vec<LibraryFileIdentity>,
    pub packages: Vec<LibraryPackageFact>,
    pub imports: Vec<LibraryImportFact>,
    pub type_references: Vec<LibraryTypeReferenceFact>,
}

impl CacheArtifact for LibraryIndex {
    type Identity = LibraryIndexIdentity;

    const KIND: ArtifactKind = ArtifactKind::LibraryIndex;
    const SCHEMA_VERSION: u32 = LIBRARY_INDEX_SCHEMA_VERSION;

    fn validate_invariants(&self) -> Result<(), String> {
        if !matches!(self.status, LibraryIndexStatus::Ok) && !self.packages.is_empty() {
            return Err(
                "LibraryIndex with a non-Ok status must not carry package facts".to_string(),
            );
        }
        if !matches!(self.status, LibraryIndexStatus::Ok) && !self.imports.is_empty() {
            return Err(
                "LibraryIndex with a non-Ok status must not carry import facts".to_string(),
            );
        }
        if !matches!(self.status, LibraryIndexStatus::Ok) && !self.type_references.is_empty() {
            return Err(
                "LibraryIndex with a non-Ok status must not carry type-reference facts".to_string(),
            );
        }
        let mut seen = std::collections::HashSet::new();
        for file in &self.files {
            if !seen.insert((file.root_slot, file.relative_path.clone())) {
                return Err(format!(
                    "duplicate library file identity at root slot {} path {:?}",
                    file.root_slot, file.relative_path
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(slot: u32, path: &str, content: &[u8]) -> LibraryFileIdentity {
        LibraryFileIdentity {
            root_slot: slot,
            relative_path: path.to_string(),
            content_digest: ContentDigest::of_bytes(content),
        }
    }

    #[test]
    fn identity_key_changes_when_a_file_digest_changes() {
        let a = LibraryIndexIdentity::new(vec![vec![file(0, "Base.kerml", b"v1")]]);
        let b = LibraryIndexIdentity::new(vec![vec![file(0, "Base.kerml", b"v2")]]);
        assert_ne!(a.artifact_key(), b.artifact_key());
    }

    #[test]
    fn identity_key_changes_when_root_order_changes() {
        let root_a = vec![file(0, "A.kerml", b"a")];
        let root_b = vec![file(0, "B.kerml", b"b")];
        let a_then_b = LibraryIndexIdentity::new(vec![root_a.clone(), root_b.clone()]);
        let b_then_a = LibraryIndexIdentity::new(vec![root_b, root_a]);
        assert_ne!(a_then_b.artifact_key(), b_then_a.artifact_key());
    }

    #[test]
    fn identity_key_is_portable_across_relative_path_prefix_unrelated_to_content() {
        // Same root slot, same relative path, same content computed independently: proves the
        // key is a pure function of the manifest, not of any absolute install path (none is
        // even representable here).
        let a = LibraryIndexIdentity::new(vec![vec![file(0, "Base.kerml", b"same")]]);
        let b = LibraryIndexIdentity::new(vec![vec![file(0, "Base.kerml", b"same")]]);
        assert_eq!(a.artifact_key(), b.artifact_key());
    }

    #[test]
    fn invariant_rejects_facts_on_non_ok_status() {
        let bad = LibraryIndex {
            status: LibraryIndexStatus::Malformed {
                detail: "bad encoding".to_string(),
            },
            files: Vec::new(),
            packages: vec![LibraryPackageFact {
                qualified_name: "Base".to_string(),
                root_slot: 0,
                relative_path: "Base.kerml".to_string(),
            }],
            imports: Vec::new(),
            type_references: Vec::new(),
        };
        assert!(bad.validate_invariants().is_err());
    }

    #[test]
    fn invariant_rejects_duplicate_file_identities() {
        let bad = LibraryIndex {
            status: LibraryIndexStatus::Ok,
            files: vec![file(0, "Base.kerml", b"x"), file(0, "Base.kerml", b"y")],
            packages: Vec::new(),
            imports: Vec::new(),
            type_references: Vec::new(),
        };
        assert!(bad.validate_invariants().is_err());
    }
}
