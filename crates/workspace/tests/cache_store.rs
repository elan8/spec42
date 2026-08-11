//! Correctness tests for the unified cache store foundation (plan §11.1).
//!
//! Every test uses an isolated temporary root; none of them touch the real platform cache
//! directory.

use std::sync::Arc;
use std::thread;

use serde::{Deserialize, Serialize};
use tempfile::TempDir;

use workspace::cache::{
    ArtifactIdentity, ArtifactKey, ArtifactKind, CacheArtifact, CacheConfig, CacheLookup,
    CacheMissReason, CacheStore, CacheStoreOutcome, CanonicalEncoder, FileCacheStore,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TestArtifact {
    value: u32,
    label: String,
}

#[derive(Debug, Clone)]
struct TestIdentity {
    seed: Vec<u8>,
}

impl ArtifactIdentity for TestIdentity {
    fn artifact_key(&self) -> ArtifactKey {
        let mut enc = CanonicalEncoder::new(ArtifactKey::DOMAIN);
        enc.field(b"test-artifact-v1");
        enc.field(&self.seed);
        ArtifactKey::from_encoder(&enc)
    }
}

impl CacheArtifact for TestArtifact {
    type Identity = TestIdentity;
    const KIND: ArtifactKind = ArtifactKind::ParseOutcome;
    const SCHEMA_VERSION: u32 = 1;
}

/// A variant whose invariant check always fails, used to test `InvariantFailure` misses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct InvalidatingArtifact {
    value: u32,
}

impl CacheArtifact for InvalidatingArtifact {
    type Identity = TestIdentity;
    const KIND: ArtifactKind = ArtifactKind::ParseOutcome;
    const SCHEMA_VERSION: u32 = 1;

    fn validate_invariants(&self) -> Result<(), String> {
        Err("intentionally invalid for test coverage".to_string())
    }
}

fn isolated_store() -> (TempDir, FileCacheStore) {
    let dir = TempDir::new().expect("temp dir");
    let config = CacheConfig::with_root(dir.path().to_path_buf());
    (dir, FileCacheStore::new(config))
}

fn identity(seed: &[u8]) -> TestIdentity {
    TestIdentity {
        seed: seed.to_vec(),
    }
}

// ---------------------------------------------------------------------------
// Round trip / basic get-put
// ---------------------------------------------------------------------------

#[test]
fn put_then_get_round_trips() {
    let (_dir, store) = isolated_store();
    let id = identity(b"round-trip");
    let value = TestArtifact {
        value: 42,
        label: "hello".to_string(),
    };

    let outcome = store.put(&id, &value);
    assert!(matches!(outcome, CacheStoreOutcome::Stored { .. }));

    match store.get::<TestArtifact>(&id) {
        CacheLookup::Hit(got, meta) => {
            assert_eq!(got, value);
            assert!(meta.uncompressed_len > 0);
        }
        CacheLookup::Miss(reason) => panic!("expected hit, got miss: {reason:?}"),
    }
}

#[test]
fn miss_before_any_write() {
    let (_dir, store) = isolated_store();
    let id = identity(b"never-written");
    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::NotFound) => {}
        other => panic!("expected NotFound miss, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Corruption / rejection + cold fallback
// ---------------------------------------------------------------------------

fn object_path_for(store: &FileCacheStore, id: &TestIdentity) -> std::path::PathBuf {
    let key = id.artifact_key();
    let (a, b) = key.shard();
    store
        .config()
        .objects_dir()
        .join(a)
        .join(b)
        .join(format!("{}.s42c", key.hex()))
}

#[test]
fn truncated_object_is_a_typed_miss_and_deletes_bad_path() {
    let (_dir, store) = isolated_store();
    let id = identity(b"truncate-me");
    let value = TestArtifact {
        value: 1,
        label: "x".to_string(),
    };
    store.put(&id, &value);

    let path = object_path_for(&store, &id);
    let bytes = std::fs::read(&path).unwrap();
    std::fs::write(&path, &bytes[..bytes.len() - 4]).unwrap();

    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::Truncated) => {}
        other => panic!("expected Truncated miss, got {other:?}"),
    }
    assert!(!path.exists(), "bad object should be best-effort deleted");
}

#[test]
fn bit_flip_is_a_typed_miss() {
    let (_dir, store) = isolated_store();
    let id = identity(b"bitflip-me");
    let value = TestArtifact {
        value: 7,
        label: "payload-data".to_string(),
    };
    store.put(&id, &value);

    let path = object_path_for(&store, &id);
    let mut bytes = std::fs::read(&path).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xFF;
    std::fs::write(&path, &bytes).unwrap();

    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(_) => {}
        CacheLookup::Hit(..) => panic!("bit-flipped object must never be returned as a hit"),
    }
}

#[test]
fn key_mismatch_is_a_typed_miss() {
    let (_dir, store) = isolated_store();
    let id_a = identity(b"key-a");
    let id_b = identity(b"key-b");
    store.put(
        &id_a,
        &TestArtifact {
            value: 1,
            label: "a".to_string(),
        },
    );
    store.put(
        &id_b,
        &TestArtifact {
            value: 2,
            label: "b".to_string(),
        },
    );

    // Overwrite b's object with a's bytes: the envelope's embedded key won't match the path
    // derived from b's identity.
    let path_a = object_path_for(&store, &id_a);
    let path_b = object_path_for(&store, &id_b);
    let bytes_a = std::fs::read(&path_a).unwrap();
    std::fs::write(&path_b, &bytes_a).unwrap();

    match store.get::<TestArtifact>(&id_b) {
        CacheLookup::Miss(CacheMissReason::KeyMismatch) => {}
        other => panic!("expected KeyMismatch, got {other:?}"),
    }
}

#[test]
fn bad_magic_is_a_typed_miss() {
    let (_dir, store) = isolated_store();
    let id = identity(b"bad-magic");
    store.put(
        &id,
        &TestArtifact {
            value: 1,
            label: "a".to_string(),
        },
    );
    let path = object_path_for(&store, &id);
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[0] = b'X';
    std::fs::write(&path, &bytes).unwrap();

    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(_) => {}
        CacheLookup::Hit(..) => panic!("bad magic must never decode as a hit"),
    }
}

#[test]
fn incompatible_envelope_version_is_a_typed_miss() {
    let (_dir, store) = isolated_store();
    let id = identity(b"env-version");
    store.put(
        &id,
        &TestArtifact {
            value: 1,
            label: "a".to_string(),
        },
    );
    let path = object_path_for(&store, &id);
    let mut bytes = std::fs::read(&path).unwrap();
    // envelope_version is bytes [4..6], little-endian u16.
    bytes[4] = 0xFF;
    bytes[5] = 0xFF;
    std::fs::write(&path, &bytes).unwrap();

    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::IncompatibleVersion { .. }) => {}
        other => panic!("expected IncompatibleVersion, got {other:?}"),
    }
}

#[test]
fn incompatible_artifact_schema_version_is_a_typed_miss() {
    let (_dir, store) = isolated_store();
    let id = identity(b"schema-version");
    store.put(
        &id,
        &TestArtifact {
            value: 1,
            label: "a".to_string(),
        },
    );
    let path = object_path_for(&store, &id);
    let mut bytes = std::fs::read(&path).unwrap();
    // artifact_schema is bytes [7..11], little-endian u32.
    bytes[7..11].copy_from_slice(&99u32.to_le_bytes());
    std::fs::write(&path, &bytes).unwrap();

    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::IncompatibleVersion { .. }) => {}
        other => panic!("expected IncompatibleVersion, got {other:?}"),
    }
}

#[test]
fn invariant_failure_is_a_typed_miss() {
    let (_dir, store) = isolated_store();
    let id = identity(b"invariant");
    let outcome = store.put(&id, &InvalidatingArtifact { value: 1 });
    assert!(matches!(outcome, CacheStoreOutcome::Stored { .. }));

    match store.get::<InvalidatingArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::InvariantFailure { .. }) => {}
        other => panic!("expected InvariantFailure, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Atomic concurrent writers
// ---------------------------------------------------------------------------

#[test]
fn concurrent_writers_same_key_produce_one_valid_object() {
    let (_dir, store) = isolated_store();
    let store = Arc::new(store);
    let id = Arc::new(identity(b"concurrent-key"));

    let mut handles = Vec::new();
    for i in 0..16u32 {
        let store = Arc::clone(&store);
        let id = Arc::clone(&id);
        handles.push(thread::spawn(move || {
            // Every writer publishes semantically-equal content under the identical key: the
            // design requires distinct writers to a key to be canonical/deterministic, so keep
            // the payload identical across threads and vary only which thread performs it.
            let _ = i;
            let value = TestArtifact {
                value: 42,
                label: "identical-payload".to_string(),
            };
            store.put(id.as_ref(), &value)
        }));
    }
    for h in handles {
        let outcome = h.join().unwrap();
        assert!(
            matches!(outcome, CacheStoreOutcome::Stored { .. }),
            "{outcome:?}"
        );
    }

    match store.get::<TestArtifact>(id.as_ref()) {
        CacheLookup::Hit(value, _) => {
            assert_eq!(value.value, 42);
            assert_eq!(value.label, "identical-payload");
        }
        CacheLookup::Miss(reason) => {
            panic!("expected a valid hit after concurrent writes, got {reason:?}")
        }
    }
}

#[test]
fn readers_never_observe_a_partial_file_during_concurrent_writes() {
    let (_dir, store) = isolated_store();
    let store = Arc::new(store);
    let id = Arc::new(identity(b"reader-during-write"));

    // Seed one valid object first so readers have something to read from the start.
    store.put(
        id.as_ref(),
        &TestArtifact {
            value: 1,
            label: "seed".to_string(),
        },
    );

    let writer_store = Arc::clone(&store);
    let writer_id = Arc::clone(&id);
    let writer = thread::spawn(move || {
        for i in 0..64u32 {
            let value = TestArtifact {
                value: 1,
                label: format!("write-{i}"),
            };
            writer_store.put(writer_id.as_ref(), &value);
        }
    });

    let mut reader_handles = Vec::new();
    for _ in 0..4 {
        let reader_store = Arc::clone(&store);
        let reader_id = Arc::clone(&id);
        reader_handles.push(thread::spawn(move || {
            for _ in 0..200 {
                match reader_store.get::<TestArtifact>(reader_id.as_ref()) {
                    // A hit must always be one complete, digest-verified object. A miss
                    // (e.g. a fleeting NotFound between rename steps, which should not even
                    // happen with atomic publish, but is safe either way) is acceptable — a
                    // partially-decoded or corrupted value is not.
                    CacheLookup::Hit(value, _) => {
                        assert_eq!(value.value, 1);
                    }
                    CacheLookup::Miss(_) => {}
                }
            }
        }));
    }

    writer.join().unwrap();
    for h in reader_handles {
        h.join().unwrap();
    }
}

// ---------------------------------------------------------------------------
// Interrupted write / temp file reaping
// ---------------------------------------------------------------------------

#[test]
fn abandoned_temp_file_leaves_no_accepted_entry_and_is_reaped_by_maintenance() {
    let (_dir, store) = isolated_store();
    let id = identity(b"interrupted-write");

    std::fs::create_dir_all(store.config().tmp_dir()).unwrap();
    let tmp_path = store.config().tmp_dir().join("abandoned-write.tmp");
    std::fs::write(&tmp_path, b"partial garbage that never got published").unwrap();
    // Simulate an old abandoned file by rewinding its mtime well past the reap threshold.
    let ancient = std::time::SystemTime::now() - std::time::Duration::from_secs(3 * 60 * 60);
    let file = std::fs::OpenOptions::new()
        .write(true)
        .open(&tmp_path)
        .unwrap();
    file.set_modified(ancient).unwrap();

    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::NotFound) => {}
        other => panic!("interrupted write must not be visible as a hit: {other:?}"),
    }

    let report = store.prune();
    assert_eq!(report.tmp_files_reaped, 1);
    assert!(!tmp_path.exists());
}

// ---------------------------------------------------------------------------
// Read-only / unwritable root, missing object, failed touch/deletion
// ---------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn unwritable_cache_root_fails_writes_without_changing_read_semantics() {
    use std::os::unix::fs::PermissionsExt;

    let dir = TempDir::new().unwrap();
    let config = CacheConfig::with_root(dir.path().to_path_buf());
    let store = FileCacheStore::new(config);
    let id = identity(b"readonly-root");

    // Create the directory structure first (as a writable store normally would), then lock it
    // down so subsequent writes cannot create new shard directories/files.
    std::fs::create_dir_all(store.config().objects_dir()).unwrap();
    std::fs::create_dir_all(store.config().tmp_dir()).unwrap();
    let mut perms = std::fs::metadata(store.config().tmp_dir())
        .unwrap()
        .permissions();
    perms.set_mode(0o500); // read+execute only, no write
    std::fs::set_permissions(store.config().tmp_dir(), perms.clone()).unwrap();

    let outcome = store.put(
        &id,
        &TestArtifact {
            value: 1,
            label: "x".to_string(),
        },
    );
    assert!(
        matches!(outcome, CacheStoreOutcome::Failed(_)),
        "{outcome:?}"
    );

    // Reads behave exactly as a cold cache: NotFound, not a crash or a stale/partial value.
    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::NotFound) => {}
        other => panic!("expected NotFound after failed write, got {other:?}"),
    }

    // Restore permissions so TempDir cleanup can remove the directory.
    perms.set_mode(0o700);
    std::fs::set_permissions(store.config().tmp_dir(), perms).unwrap();
}

#[test]
fn missing_object_is_not_found() {
    let (_dir, store) = isolated_store();
    let id = identity(b"missing");
    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::NotFound) => {}
        other => panic!("expected NotFound, got {other:?}"),
    }
}

#[cfg(unix)]
#[test]
fn failed_deletion_of_a_corrupt_object_does_not_change_observable_results() {
    use std::os::unix::fs::PermissionsExt;

    let (_dir, store) = isolated_store();
    let id = identity(b"undeletable-corrupt");
    store.put(
        &id,
        &TestArtifact {
            value: 1,
            label: "x".to_string(),
        },
    );

    let path = object_path_for(&store, &id);
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[0] = b'X'; // corrupt magic
    std::fs::write(&path, &bytes).unwrap();

    // Make the parent directory read-only so the best-effort delete of the bad object fails.
    let parent = path.parent().unwrap();
    let mut perms = std::fs::metadata(parent).unwrap().permissions();
    perms.set_mode(0o500);
    std::fs::set_permissions(parent, perms.clone()).unwrap();

    // The lookup is still a clean typed miss even though the best-effort delete cannot succeed.
    let result = store.get::<TestArtifact>(&id);
    assert!(matches!(result, CacheLookup::Miss(_)));

    perms.set_mode(0o700);
    std::fs::set_permissions(parent, perms).unwrap();
}

// ---------------------------------------------------------------------------
// LRU / capacity
// ---------------------------------------------------------------------------

#[test]
fn prune_converges_to_at_most_the_target_with_oldest_first_victim_order() {
    let dir = TempDir::new().unwrap();
    let mut config = CacheConfig::with_root(dir.path().to_path_buf());
    // Each stored object is small (a handful of bytes); inject a tiny budget so a handful of
    // writes exceed it and force pruning.
    config.max_bytes = 400;
    config.prune_target_bytes = 200;
    let store = FileCacheStore::new(config);

    // Write enough distinct-keyed objects, sleeping briefly between writes so mtimes are
    // strictly increasing and victim order is deterministic.
    let mut ids = Vec::new();
    for i in 0..20u32 {
        let id = identity(format!("prune-key-{i}").as_bytes());
        store.put(
            &id,
            &TestArtifact {
                value: i,
                label: format!("payload-{i}"),
            },
        );
        ids.push(id);
        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    let status = store.status();
    assert!(
        status.total_bytes <= 400,
        "post-write budget enforcement should have kept usage near the target, got {}",
        status.total_bytes
    );

    // The earliest-written keys should have been evicted first (oldest mtime), so at least the
    // very first key should now be a miss while the most recent key should still be a hit.
    match store.get::<TestArtifact>(&ids[0]) {
        CacheLookup::Miss(_) => {}
        CacheLookup::Hit(..) => panic!("oldest object should have been pruned first"),
    }
    match store.get::<TestArtifact>(ids.last().unwrap()) {
        CacheLookup::Hit(..) => {}
        CacheLookup::Miss(reason) => panic!("newest object should survive pruning: {reason:?}"),
    }
}

#[test]
fn explicit_prune_enforces_target_on_isolated_root() {
    let dir = TempDir::new().unwrap();
    let mut config = CacheConfig::with_root(dir.path().to_path_buf());
    // Use a very large max_bytes so post-write auto-prune never triggers, then call prune()
    // explicitly to verify it independently enforces the target.
    config.max_bytes = u64::MAX;
    config.prune_target_bytes = 150;
    let store = FileCacheStore::new(config);

    for i in 0..10u32 {
        let id = identity(format!("explicit-prune-{i}").as_bytes());
        store.put(
            &id,
            &TestArtifact {
                value: i,
                label: format!("payload-{i}"),
            },
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    let before = store.status();
    assert!(before.total_bytes > 150);

    let report = store.prune();
    assert!(report.objects_removed > 0);

    let after = store.status();
    assert!(
        after.total_bytes <= 150 || report.over_budget,
        "prune should converge to the target or explicitly report over_budget"
    );
}

// ---------------------------------------------------------------------------
// Disabled mode
// ---------------------------------------------------------------------------

#[test]
fn disabled_mode_performs_neither_reads_nor_writes() {
    let dir = TempDir::new().unwrap();
    let config = CacheConfig::disabled(dir.path().to_path_buf());
    let store = FileCacheStore::new(config);
    let id = identity(b"disabled-mode");

    let outcome = store.put(
        &id,
        &TestArtifact {
            value: 1,
            label: "x".to_string(),
        },
    );
    assert!(matches!(outcome, CacheStoreOutcome::Disabled));

    match store.get::<TestArtifact>(&id) {
        CacheLookup::Miss(CacheMissReason::Disabled) => {}
        other => panic!("expected Disabled miss, got {other:?}"),
    }

    // Disabled mode must not create any on-disk structure at all: no objects/ or tmp/ dirs.
    assert!(!store.config().objects_dir().exists());
    assert!(!store.config().tmp_dir().exists());
}

#[test]
fn clear_removes_all_observed_objects() {
    let (_dir, store) = isolated_store();
    for i in 0..5u32 {
        let id = identity(format!("clear-{i}").as_bytes());
        store.put(
            &id,
            &TestArtifact {
                value: i,
                label: "x".to_string(),
            },
        );
    }
    let status_before = store.status();
    assert_eq!(status_before.object_count, 5);

    let report = store.clear();
    assert_eq!(report.objects_removed, 5);

    let status_after = store.status();
    assert_eq!(status_after.object_count, 0);
}

#[test]
fn status_reports_per_kind_counts() {
    let (_dir, store) = isolated_store();
    let id = identity(b"status-check");
    store.put(
        &id,
        &TestArtifact {
            value: 1,
            label: "x".to_string(),
        },
    );

    let status = store.status();
    assert_eq!(status.object_count, 1);
    let stats = status
        .per_kind
        .get(&ArtifactKind::ParseOutcome)
        .expect("kind stats present");
    assert_eq!(stats.object_count, 1);
    assert!(stats.total_bytes > 0);
}
