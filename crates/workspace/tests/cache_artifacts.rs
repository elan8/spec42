//! Round-trip and cross-cutting correctness tests for the source-derived cache artifacts:
//! `ParseOutcome`, `LibraryIndex`, and
//! `LibraryClosure`. Every test uses an isolated temporary cache root.

use tempfile::TempDir;

use source_identity::{ArtifactKey, ContentDigest, RootDigest, SourceRole};
use workspace::cache::artifacts::{
    library_closure::{
        LibraryClosure, LibraryClosureIdentity, LibraryClosurePolicy, SelectedLibraryFile,
    },
    library_index::{LibraryFileIdentity, LibraryIndex, LibraryIndexIdentity, LibraryIndexStatus},
    parse_outcome::{ParseMode, ParseOutcome, ParseOutcomeIdentity},
};
use workspace::cache::{
    ArtifactIdentity, CacheConfig, CacheLookup, CacheStore, CacheStoreOutcome, FileCacheStore,
};

fn isolated_store() -> (TempDir, FileCacheStore) {
    let dir = TempDir::new().expect("temp dir");
    let config = CacheConfig::with_root(dir.path().to_path_buf());
    (dir, FileCacheStore::new(config))
}

// ---------------------------------------------------------------------------
// ParseOutcome
// ---------------------------------------------------------------------------

#[test]
fn parse_outcome_strict_success_round_trips_through_the_real_store() {
    let (_dir, store) = isolated_store();
    let src = "package RoundTrip;";
    let result = sysml_v2_parser::next::parse(src);
    assert!(result.is_ok());
    let outcome = ParseOutcome::from_strict(&result);

    let id = ParseOutcomeIdentity {
        content_digest: ContentDigest::of_bytes(src.as_bytes()),
        mode: ParseMode::StrictSemantic,
    };
    assert!(matches!(
        store.put(&id, &outcome),
        CacheStoreOutcome::Stored { .. }
    ));

    match store.get::<ParseOutcome>(&id) {
        CacheLookup::Hit(got, _) => assert_eq!(got, outcome),
        CacheLookup::Miss(reason) => panic!("expected hit, got miss: {reason:?}"),
    }
}

#[test]
fn parse_outcome_editor_recovery_retains_complete_diagnostics_on_a_warm_hit() {
    // This is the pre-existing bug fix (plan §6.2): a warm hit for an editor-recovery outcome
    // must return the same diagnostic codes, ranges, and ordering as the cold parse, not an
    // empty diagnostic list.
    let (_dir, store) = isolated_store();
    let src = "package P { this is not valid sysml @@@ ";
    let cold = sysml_v2_parser::next::parse_for_editor(src);
    assert!(!cold.errors.is_empty(), "fixture must produce diagnostics");
    let cold_outcome = ParseOutcome::from_editor_recovery(&cold);
    assert!(!cold_outcome.diagnostics.is_empty());

    let id = ParseOutcomeIdentity {
        content_digest: ContentDigest::of_bytes(src.as_bytes()),
        mode: ParseMode::EditorRecovery,
    };
    store.put(&id, &cold_outcome);

    let warm_outcome = match store.get::<ParseOutcome>(&id) {
        CacheLookup::Hit(got, _) => got,
        CacheLookup::Miss(reason) => panic!("expected hit, got miss: {reason:?}"),
    };

    assert_eq!(warm_outcome.status, cold_outcome.status);
    assert_eq!(
        warm_outcome.diagnostics.len(),
        cold_outcome.diagnostics.len(),
        "warm hit must retain every diagnostic the cold parse reported"
    );
    // Same codes, ranges, and ordering: compare element-wise, not as a set.
    assert_eq!(warm_outcome.diagnostics, cold_outcome.diagnostics);
    assert!(warm_outcome.ast.is_some());
}

#[test]
fn parse_outcome_expected_negative_round_trips_under_the_same_complete_key() {
    let (_dir, store) = isolated_store();
    let src = "package P { @@@ not valid ";
    let result = sysml_v2_parser::next::parse(src);
    assert!(result.is_err());
    let outcome = ParseOutcome::from_strict(&result);
    assert_eq!(
        outcome.status,
        workspace::cache::artifacts::parse_outcome::ParseStatus::ExpectedSyntaxFailure
    );

    let id = ParseOutcomeIdentity {
        content_digest: ContentDigest::of_bytes(src.as_bytes()),
        mode: ParseMode::StrictSemantic,
    };
    assert!(matches!(
        store.put(&id, &outcome),
        CacheStoreOutcome::Stored { .. }
    ));
    match store.get::<ParseOutcome>(&id) {
        CacheLookup::Hit(got, _) => assert_eq!(got, outcome),
        CacheLookup::Miss(reason) => panic!("expected hit, got miss: {reason:?}"),
    }
}

#[test]
fn parse_outcome_identical_content_at_a_different_path_hits() {
    // Parse artifacts omit the URI entirely (plan §6.2); this test proves the identity truly
    // carries no path, by using two different logical paths that happen to share content and
    // confirming they collide on the same cache key/entry.
    let (_dir, store) = isolated_store();
    let src = "package Shared;";
    let outcome = ParseOutcome::from_strict(&sysml_v2_parser::next::parse(src));

    let id_at_path_a = ParseOutcomeIdentity {
        content_digest: ContentDigest::of_bytes(src.as_bytes()),
        mode: ParseMode::StrictSemantic,
    };
    // A second identity built independently (simulating a relocated checkout / different file
    // path producing the same bytes) must derive the identical key.
    let id_at_path_b = ParseOutcomeIdentity {
        content_digest: ContentDigest::of_bytes(src.as_bytes()),
        mode: ParseMode::StrictSemantic,
    };
    assert_eq!(id_at_path_a.artifact_key(), id_at_path_b.artifact_key());

    store.put(&id_at_path_a, &outcome);
    match store.get::<ParseOutcome>(&id_at_path_b) {
        CacheLookup::Hit(got, _) => assert_eq!(got, outcome),
        CacheLookup::Miss(reason) => panic!("expected hit, got miss: {reason:?}"),
    }
}

#[test]
fn parse_outcome_key_completeness_per_input() {
    let src_a = "package A;";
    let src_b = "package B;";
    let id = |src: &str, mode: ParseMode| ParseOutcomeIdentity {
        content_digest: ContentDigest::of_bytes(src.as_bytes()),
        mode,
    };

    // Content digest.
    assert_ne!(
        id(src_a, ParseMode::StrictSemantic).artifact_key(),
        id(src_b, ParseMode::StrictSemantic).artifact_key()
    );
    // Parse mode.
    assert_ne!(
        id(src_a, ParseMode::StrictSemantic).artifact_key(),
        id(src_a, ParseMode::EditorRecovery).artifact_key()
    );
}

#[test]
fn parse_outcome_corruption_falls_back_cold_as_a_typed_miss() {
    let (_dir, store) = isolated_store();
    let src = "package Corrupt;";
    let outcome = ParseOutcome::from_strict(&sysml_v2_parser::next::parse(src));
    let id = ParseOutcomeIdentity {
        content_digest: ContentDigest::of_bytes(src.as_bytes()),
        mode: ParseMode::StrictSemantic,
    };
    store.put(&id, &outcome);

    let key = id.artifact_key();
    let (a, b) = key.shard();
    let path = store
        .config()
        .objects_dir()
        .join(a)
        .join(b)
        .join(format!("{}.s42c", key.hex()));
    let mut bytes = std::fs::read(&path).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xFF;
    std::fs::write(&path, bytes).unwrap();

    match store.get::<ParseOutcome>(&id) {
        CacheLookup::Miss(_) => {}
        CacheLookup::Hit(..) => panic!("corrupted object must not be returned as a hit"),
    }
}

// ---------------------------------------------------------------------------
// LibraryIndex
// ---------------------------------------------------------------------------

fn lib_file(slot: u32, path: &str, content: &[u8]) -> LibraryFileIdentity {
    LibraryFileIdentity {
        root_slot: slot,
        relative_path: path.to_string(),
        content_digest: ContentDigest::of_bytes(content),
    }
}

#[test]
fn library_index_round_trips_through_the_real_store() {
    let (_dir, store) = isolated_store();
    let file = lib_file(0, "Base.kerml", b"package Base;");
    let identity = LibraryIndexIdentity::new(vec![vec![file.clone()]]);
    let index = LibraryIndex {
        status: LibraryIndexStatus::Ok,
        files: vec![file],
        packages: vec![],
        imports: vec![],
        type_references: vec![],
    };

    assert!(matches!(
        store.put(&identity, &index),
        CacheStoreOutcome::Stored { .. }
    ));
    match store.get::<LibraryIndex>(&identity) {
        CacheLookup::Hit(got, _) => assert_eq!(got, index),
        CacheLookup::Miss(reason) => panic!("expected hit, got miss: {reason:?}"),
    }
}

#[test]
fn library_index_identical_content_at_a_different_root_path_hits() {
    // Portability requirement (plan §6.3): identical library content moved to a different
    // absolute root must still hit, because the identity never carries an absolute path.
    let file_a = lib_file(0, "Base.kerml", b"same content");
    let file_b = lib_file(0, "Base.kerml", b"same content");
    let identity_a = LibraryIndexIdentity::new(vec![vec![file_a]]);
    let identity_b = LibraryIndexIdentity::new(vec![vec![file_b]]);
    assert_eq!(identity_a.artifact_key(), identity_b.artifact_key());
}

#[test]
fn library_index_key_completeness_per_input() {
    let base_key =
        LibraryIndexIdentity::new(vec![vec![lib_file(0, "A.kerml", b"a")]]).artifact_key();

    // Content digest.
    assert_ne!(
        base_key,
        LibraryIndexIdentity::new(vec![vec![lib_file(0, "A.kerml", b"b")]]).artifact_key()
    );
    // Relative path.
    assert_ne!(
        base_key,
        LibraryIndexIdentity::new(vec![vec![lib_file(0, "B.kerml", b"a")]]).artifact_key()
    );
    // Root slot.
    assert_ne!(
        base_key,
        LibraryIndexIdentity::new(vec![vec![lib_file(1, "A.kerml", b"a")]]).artifact_key()
    );
    // Root order.
    let root_a = vec![lib_file(0, "A.kerml", b"a")];
    let root_b = vec![lib_file(0, "B.kerml", b"b")];
    assert_ne!(
        LibraryIndexIdentity::new(vec![root_a.clone(), root_b.clone()]).artifact_key(),
        LibraryIndexIdentity::new(vec![root_b, root_a]).artifact_key()
    );
}

// ---------------------------------------------------------------------------
// LibraryClosure
// ---------------------------------------------------------------------------

fn selected(slot: u32, path: &str, content: &[u8]) -> SelectedLibraryFile {
    SelectedLibraryFile {
        root_slot: slot,
        relative_path: path.to_string(),
        content_digest: ContentDigest::of_bytes(content),
    }
}

fn closure_identity(
    workspace_seed: &[u8],
    index_keys: Vec<ArtifactKey>,
    policy: LibraryClosurePolicy,
) -> LibraryClosureIdentity {
    LibraryClosureIdentity {
        workspace_root_digest: RootDigest::of_bytes(workspace_seed),
        library_index_keys: index_keys,
        policy,
    }
}

#[test]
fn library_closure_round_trips_through_the_real_store() {
    let (_dir, store) = isolated_store();
    let closure = LibraryClosure::new(
        vec![
            selected(0, "Base.kerml", b"base"),
            selected(1, "Parts.kerml", b"parts"),
        ],
        vec![],
    );
    let identity = closure_identity(
        b"workspace-seed",
        vec![ArtifactKey::of_bytes(b"index-0")],
        LibraryClosurePolicy::conservative_default(),
    );

    assert!(matches!(
        store.put(&identity, &closure),
        CacheStoreOutcome::Stored { .. }
    ));
    match store.get::<LibraryClosure>(&identity) {
        CacheLookup::Hit(got, _) => assert_eq!(got, closure),
        CacheLookup::Miss(reason) => panic!("expected hit, got miss: {reason:?}"),
    }
}

#[test]
fn library_closure_key_completeness_per_input() {
    let base_policy = LibraryClosurePolicy::conservative_default();
    let base_key = closure_identity(
        b"ws",
        vec![ArtifactKey::of_bytes(b"idx")],
        base_policy.clone(),
    )
    .artifact_key();

    // Workspace root digest (conservative: any workspace byte invalidates).
    assert_ne!(
        base_key,
        closure_identity(
            b"ws-changed",
            vec![ArtifactKey::of_bytes(b"idx")],
            base_policy.clone()
        )
        .artifact_key()
    );
    // Library index key.
    assert_ne!(
        base_key,
        closure_identity(
            b"ws",
            vec![ArtifactKey::of_bytes(b"idx-2")],
            base_policy.clone()
        )
        .artifact_key()
    );
    // Library-index key order.
    assert_ne!(
        closure_identity(
            b"ws",
            vec![ArtifactKey::of_bytes(b"k1"), ArtifactKey::of_bytes(b"k2")],
            base_policy.clone()
        )
        .artifact_key(),
        closure_identity(
            b"ws",
            vec![ArtifactKey::of_bytes(b"k2"), ArtifactKey::of_bytes(b"k1")],
            base_policy.clone()
        )
        .artifact_key()
    );
    // Bootstrap flags.
    let mut p = base_policy.clone();
    p.bootstrap_sysml_namespace = false;
    assert_ne!(
        base_key,
        closure_identity(b"ws", vec![ArtifactKey::of_bytes(b"idx")], p).artifact_key()
    );
    // Seed packages.
    let mut p = base_policy.clone();
    p.seed_packages = vec!["Base".to_string()];
    assert_ne!(
        base_key,
        closure_identity(b"ws", vec![ArtifactKey::of_bytes(b"idx")], p).artifact_key()
    );
    // Full-scan mode.
    let mut p = base_policy.clone();
    p.full_scan = true;
    assert_ne!(
        base_key,
        closure_identity(b"ws", vec![ArtifactKey::of_bytes(b"idx")], p).artifact_key()
    );
    // Source roles.
    let mut p = base_policy.clone();
    p.root_roles = vec![SourceRole::StandardLibrary];
    assert_ne!(
        base_key,
        closure_identity(b"ws", vec![ArtifactKey::of_bytes(b"idx")], p).artifact_key()
    );
    // Closure algorithm version.
    let mut p = base_policy;
    p.algorithm_version += 1;
    assert_ne!(
        base_key,
        closure_identity(b"ws", vec![ArtifactKey::of_bytes(b"idx")], p).artifact_key()
    );
}

#[test]
fn library_closure_determinism_order_independent() {
    let a = LibraryClosure::new(
        vec![selected(0, "A.kerml", b"a"), selected(1, "B.kerml", b"b")],
        vec![],
    );
    let b = LibraryClosure::new(
        vec![selected(1, "B.kerml", b"b"), selected(0, "A.kerml", b"a")],
        vec![],
    );
    assert_eq!(a.selected_files, b.selected_files);
    assert_eq!(a.selected_files_root_digest, b.selected_files_root_digest);
}

#[test]
fn library_closure_corruption_falls_back_cold() {
    let (_dir, store) = isolated_store();
    let closure = LibraryClosure::new(vec![selected(0, "A.kerml", b"a")], vec![]);
    let identity = closure_identity(
        b"ws",
        vec![ArtifactKey::of_bytes(b"idx")],
        LibraryClosurePolicy::conservative_default(),
    );
    store.put(&identity, &closure);

    let key = identity.artifact_key();
    let (a, b) = key.shard();
    let path = store
        .config()
        .objects_dir()
        .join(a)
        .join(b)
        .join(format!("{}.s42c", key.hex()));
    let bytes = std::fs::read(&path).unwrap();
    std::fs::write(&path, &bytes[..bytes.len() - 3]).unwrap(); // truncate
    match store.get::<LibraryClosure>(&identity) {
        CacheLookup::Miss(_) => {}
        CacheLookup::Hit(..) => panic!("truncated object must not be returned as a hit"),
    }
}
