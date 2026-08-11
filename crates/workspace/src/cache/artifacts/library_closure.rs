//! `LibraryClosure` cache artifact (`UNIFY_CACHE_PLAN.md` §6.4).
//!
//! Key inputs: the complete workspace source root (conservatively committing all workspace
//! text, via [`source_identity::RootDigest`]); the ordered [`ArtifactKey`]s of the library
//! indexes that fed the closure; and one canonical [`LibraryClosurePolicy`] covering bootstrap
//! flags, implied package seeds, full-scan mode, source roles, standard-library roots, root
//! precedence, and the closure algorithm version.
//!
//! Per §6.4's explicit trade-off, a closure entry may over-invalidate on unrelated workspace
//! content (the whole workspace root is committed, not just the parts that actually influenced
//! selection), but it must never omit an input because a heuristic signature happened to be
//! unchanged. That is why the key commits the *entire* workspace `RootDigest` rather than a
//! derived "closure seed signature" the way `crates/workspace/src/library_graph_cache.rs`
//! currently does.
//!
//! `LibraryClosurePolicy` is the single canonical policy this module defines. Routing every
//! production surface through it is plan step 5 (`SemanticBuildService`), out of scope here. See
//! the module docs below for the concrete existing call sites that currently diverge from a
//! single policy — that divergence is exactly what `UNIFY_CACHE_PLAN.md` §1 and §6.4 describe.
//!
//! ## Step 5 work list: existing call sites that must be routed through this policy
//!
//! - `crates/workspace/src/provider/filesystem.rs::HostFilesystemProvider::new` (CLI/host):
//!   injects a fixed 20-name `IMPLIED_SEMANTIC_PACKAGES` list as `seed_packages`, with default
//!   bootstrap flags and no full-scan concept at all.
//! - `crates/lsp_server/src/lsp_runtime/documents/startup.rs` (LSP startup): uses
//!   `LibraryClosureOptions::default()` (no implied seeds) for its closure signature, and
//!   separately gates on the `SPEC42_LIBRARY_FULL_SCAN` environment variable via
//!   `crates/lsp_server/src/workspace/library_closure.rs::library_full_scan_enabled`.
//! - `crates/lsp_server/src/workspace/library_closure.rs::load_library_closure_scan_entries`:
//!   also uses `LibraryClosureOptions::default()`, independently of the startup signature above.
//! - `crates/sysml_model/src/semantic/source/providers/filesystem.rs`
//!   (`FileSystemDocumentProvider`): threads whatever `library_seed_packages` its caller
//!   happened to set, which is empty unless the CLI/host path populated it.
//!
//! None of these are modified by this change (routing them is plan step 5, which depends on
//! `SemanticBuildService`); they are recorded here as the concrete divergence this policy type
//! is meant to eventually replace.

use serde::{Deserialize, Serialize};

use source_identity::{ArtifactKey, CanonicalEncoder, ContentDigest, RootDigest, SourceRole};

use crate::cache::{ArtifactIdentity, ArtifactKind, CacheArtifact};

/// Version of the closure-selection algorithm. Bump whenever the *procedure* that turns a
/// workspace root, library indexes, and a policy into a selected file set changes.
pub const LIBRARY_CLOSURE_ALGORITHM_VERSION: u32 = 1;

/// The single canonical closure policy every Spec42 host must eventually share (plan §6.4).
///
/// `root_roles[slot]` is the [`SourceRole`] classification of library-root `slot` (e.g.
/// `StandardLibrary` vs `Library`), in the same precedence order as the library-index keys this
/// policy accompanies; this folds "standard-library roots" and "root precedence" into one
/// ordered list rather than two separately-ordered ones that could disagree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryClosurePolicy {
    pub bootstrap_sysml_namespace: bool,
    pub bootstrap_typing_references: bool,
    /// Implied package seeds, in authored order. Order is committed into the key: two policies
    /// naming the same seeds in a different order are different policies, matching this module's
    /// general refusal to treat reordering as a no-op (plan §5.2's ordering-policy precedent).
    pub seed_packages: Vec<String>,
    pub full_scan: bool,
    pub root_roles: Vec<SourceRole>,
    pub algorithm_version: u32,
}

impl LibraryClosurePolicy {
    /// A policy with both bootstrap flags on, no implied seeds, full-scan off, and no
    /// classified roots — the shape `LibraryClosureOptions::default()` uses today. Provided so
    /// call sites migrating onto this type have an explicit, named starting point instead of
    /// hand-rolling the same defaults.
    pub fn conservative_default() -> Self {
        Self {
            bootstrap_sysml_namespace: true,
            bootstrap_typing_references: true,
            seed_packages: Vec::new(),
            full_scan: false,
            root_roles: Vec::new(),
            algorithm_version: LIBRARY_CLOSURE_ALGORITHM_VERSION,
        }
    }

    fn encode(&self, enc: &mut CanonicalEncoder) {
        enc.field(&[self.bootstrap_sysml_namespace as u8]);
        enc.field(&[self.bootstrap_typing_references as u8]);
        enc.field_u64(self.seed_packages.len() as u64);
        for seed in &self.seed_packages {
            enc.field(seed.as_bytes());
        }
        enc.field(&[self.full_scan as u8]);
        enc.field_u64(self.root_roles.len() as u64);
        for role in &self.root_roles {
            enc.field(&[role_tag(*role)]);
        }
        enc.field_u64(self.algorithm_version as u64);
    }
}

fn role_tag(role: SourceRole) -> u8 {
    match role {
        SourceRole::Workspace => 0,
        SourceRole::StandardLibrary => 1,
        SourceRole::Library => 2,
        SourceRole::External => 3,
    }
}

/// Everything the [`ArtifactKey`] for a [`LibraryClosure`] must commit (plan §6.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryClosureIdentity {
    /// `RootDigest` of the *complete* workspace source manifest, committing all workspace text
    /// conservatively rather than a derived seed signature.
    pub workspace_root_digest: RootDigest,
    /// Ordered `ArtifactKey`s of the library indexes that fed this closure, in configured
    /// precedence order.
    pub library_index_keys: Vec<ArtifactKey>,
    pub policy: LibraryClosurePolicy,
}

impl ArtifactIdentity for LibraryClosureIdentity {
    fn artifact_key(&self) -> ArtifactKey {
        let mut enc = CanonicalEncoder::new(ArtifactKey::DOMAIN);
        enc.field(b"library-closure.v1");
        enc.field(self.workspace_root_digest.as_bytes());
        enc.field_u64(self.library_index_keys.len() as u64);
        for key in &self.library_index_keys {
            enc.field(key.as_bytes());
        }
        self.policy.encode(&mut enc);
        ArtifactKey::from_encoder(&enc)
    }
}

/// One selected library file (plan §6.4): the same portable, root-slot-relative identity shape
/// as [`super::library_index::LibraryFileIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedLibraryFile {
    pub root_slot: u32,
    pub relative_path: String,
    pub content_digest: ContentDigest,
}

impl SelectedLibraryFile {
    const DOMAIN: &'static str = "spec42.cache.selected_library_file.v1";

    fn leaf_digest(&self) -> [u8; 32] {
        let mut enc = CanonicalEncoder::new(Self::DOMAIN);
        enc.field_u64(self.root_slot as u64);
        enc.field(self.relative_path.as_bytes());
        enc.field(self.content_digest.as_bytes());
        *enc.finish().as_bytes()
    }
}

/// A workspace-declared package that influenced closure selection, e.g. by shadowing a
/// same-named library package (plan §6.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePackageInfluence {
    pub qualified_name: String,
    /// True when this workspace package shadows a library package of the same name.
    pub shadows_library_package: bool,
}

/// The `LibraryClosure` payload (plan §6.4): the deterministically ordered selected library
/// files, the workspace packages that influenced selection, and the selected-files root digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryClosure {
    /// Ordered by `(root_slot, relative_path)`; the constructor and invariant check both enforce
    /// this so warm hits observe the same deterministic order cold builds would produce.
    pub selected_files: Vec<SelectedLibraryFile>,
    pub influencing_packages: Vec<WorkspacePackageInfluence>,
    pub selected_files_root_digest: RootDigest,
}

impl LibraryClosure {
    const SELECTED_FILES_ROOT_DOMAIN: &'static str = "spec42.cache.library_closure.selected.v1";

    /// Computes the root digest a set of selected files must have, given they are already
    /// canonically ordered. Used both to build a fresh [`LibraryClosure`] and to verify one on
    /// decode (`validate_invariants`).
    pub fn compute_selected_files_root_digest(
        selected_files: &[SelectedLibraryFile],
    ) -> RootDigest {
        let mut enc = CanonicalEncoder::new(RootDigest::DOMAIN);
        enc.field(Self::SELECTED_FILES_ROOT_DOMAIN.as_bytes());
        enc.field_u64(selected_files.len() as u64);
        for file in selected_files {
            enc.field(&file.leaf_digest());
        }
        RootDigest::from_encoder(&enc)
    }

    /// Builds a [`LibraryClosure`] from an unordered file list, canonically ordering it and
    /// deriving the root digest, so callers never have to hand-sort or hand-hash.
    pub fn new(
        mut selected_files: Vec<SelectedLibraryFile>,
        influencing_packages: Vec<WorkspacePackageInfluence>,
    ) -> Self {
        selected_files
            .sort_by(|a, b| (a.root_slot, &a.relative_path).cmp(&(b.root_slot, &b.relative_path)));
        let selected_files_root_digest = Self::compute_selected_files_root_digest(&selected_files);
        Self {
            selected_files,
            influencing_packages,
            selected_files_root_digest,
        }
    }
}

impl CacheArtifact for LibraryClosure {
    type Identity = LibraryClosureIdentity;

    const KIND: ArtifactKind = ArtifactKind::LibraryClosure;
    const SCHEMA_VERSION: u32 = 1;

    fn validate_invariants(&self) -> Result<(), String> {
        let expected = Self::compute_selected_files_root_digest(&self.selected_files);
        if expected != self.selected_files_root_digest {
            return Err(
                "LibraryClosure.selected_files_root_digest does not match the recomputed digest \
                 of selected_files"
                    .to_string(),
            );
        }
        let mut prev: Option<(u32, &str)> = None;
        for file in &self.selected_files {
            let key = (file.root_slot, file.relative_path.as_str());
            if let Some(p) = prev {
                if p >= key {
                    return Err(
                        "LibraryClosure.selected_files must be strictly ordered by (root_slot, \
                         relative_path) with no duplicates"
                            .to_string(),
                    );
                }
            }
            prev = Some(key);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(slot: u32, path: &str, content: &[u8]) -> SelectedLibraryFile {
        SelectedLibraryFile {
            root_slot: slot,
            relative_path: path.to_string(),
            content_digest: ContentDigest::of_bytes(content),
        }
    }

    fn identity(
        workspace_root_digest: RootDigest,
        library_index_keys: Vec<ArtifactKey>,
        policy: LibraryClosurePolicy,
    ) -> LibraryClosureIdentity {
        LibraryClosureIdentity {
            workspace_root_digest,
            library_index_keys,
            policy,
        }
    }

    fn some_root_digest(seed: &[u8]) -> RootDigest {
        RootDigest::of_bytes(seed)
    }

    #[test]
    fn identity_key_changes_when_workspace_root_digest_changes() {
        let policy = LibraryClosurePolicy::conservative_default();
        let a = identity(some_root_digest(b"ws-a"), vec![], policy.clone());
        let b = identity(some_root_digest(b"ws-b"), vec![], policy);
        assert_ne!(a.artifact_key(), b.artifact_key());
    }

    #[test]
    fn identity_key_changes_when_library_index_key_order_changes() {
        let policy = LibraryClosurePolicy::conservative_default();
        let k1 = ArtifactKey::of_bytes(b"index-1");
        let k2 = ArtifactKey::of_bytes(b"index-2");
        let a = identity(some_root_digest(b"ws"), vec![k1, k2], policy.clone());
        let b = identity(some_root_digest(b"ws"), vec![k2, k1], policy);
        assert_ne!(a.artifact_key(), b.artifact_key());
    }

    #[test]
    fn identity_key_changes_for_each_policy_field() {
        let base = LibraryClosurePolicy::conservative_default();
        let root = some_root_digest(b"ws");
        let base_key = identity(root, vec![], base.clone()).artifact_key();

        let mut bootstrap_ns = base.clone();
        bootstrap_ns.bootstrap_sysml_namespace = false;
        assert_ne!(
            base_key,
            identity(root, vec![], bootstrap_ns).artifact_key()
        );

        let mut bootstrap_typing = base.clone();
        bootstrap_typing.bootstrap_typing_references = false;
        assert_ne!(
            base_key,
            identity(root, vec![], bootstrap_typing).artifact_key()
        );

        let mut seeds = base.clone();
        seeds.seed_packages = vec!["Base".to_string()];
        assert_ne!(base_key, identity(root, vec![], seeds).artifact_key());

        let mut seed_order = base.clone();
        seed_order.seed_packages = vec!["Base".to_string(), "Parts".to_string()];
        let mut seed_order_swapped = base.clone();
        seed_order_swapped.seed_packages = vec!["Parts".to_string(), "Base".to_string()];
        assert_ne!(
            identity(root, vec![], seed_order).artifact_key(),
            identity(root, vec![], seed_order_swapped).artifact_key()
        );

        let mut full_scan = base.clone();
        full_scan.full_scan = true;
        assert_ne!(base_key, identity(root, vec![], full_scan).artifact_key());

        let mut roles = base.clone();
        roles.root_roles = vec![SourceRole::StandardLibrary];
        assert_ne!(base_key, identity(root, vec![], roles).artifact_key());

        let mut algo = base;
        algo.algorithm_version += 1;
        assert_ne!(base_key, identity(root, vec![], algo).artifact_key());
    }

    #[test]
    fn determinism_same_inputs_different_order_produce_same_selection_and_digest() {
        let files_a = vec![
            file(0, "Base.kerml", b"base"),
            file(1, "Parts.kerml", b"parts"),
        ];
        let files_b = vec![
            file(1, "Parts.kerml", b"parts"),
            file(0, "Base.kerml", b"base"),
        ];
        let closure_a = LibraryClosure::new(files_a, vec![]);
        let closure_b = LibraryClosure::new(files_b, vec![]);
        assert_eq!(closure_a.selected_files, closure_b.selected_files);
        assert_eq!(
            closure_a.selected_files_root_digest,
            closure_b.selected_files_root_digest
        );
        closure_a.validate_invariants().unwrap();
        closure_b.validate_invariants().unwrap();
    }

    #[test]
    fn invariant_rejects_tampered_root_digest() {
        let mut closure = LibraryClosure::new(vec![file(0, "Base.kerml", b"base")], vec![]);
        closure.selected_files_root_digest = RootDigest::of_bytes(b"not-the-real-digest");
        assert!(closure.validate_invariants().is_err());
    }

    #[test]
    fn invariant_rejects_out_of_order_selected_files() {
        let mut closure = LibraryClosure::new(
            vec![file(0, "A.kerml", b"a"), file(0, "B.kerml", b"b")],
            vec![],
        );
        closure.selected_files.reverse();
        // Recompute a digest that matches the (now out-of-order) list so only the ordering
        // invariant is exercised, not the digest-mismatch one.
        closure.selected_files_root_digest =
            LibraryClosure::compute_selected_files_root_digest(&closure.selected_files);
        assert!(closure.validate_invariants().is_err());
    }
}
