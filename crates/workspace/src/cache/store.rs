//! The physical, lock-free cache store (plan §7, §8).

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use super::api::{
    ArtifactIdentity, ArtifactKind, CacheArtifact, CacheHitMetadata, CacheKindStats, CacheLookup,
    CacheMaintenanceReport, CacheMissReason, CacheStatus, CacheStore, CacheStoreOutcome,
    CacheWriteFailure,
};
use super::config::{
    CacheConfig, CacheLimits, CacheMode, TMP_REAP_MIN_AGE_SECS, TOUCH_MIN_INTERVAL_SECS,
};
use super::digest::ArtifactKey;
use super::envelope;

/// File extension for cache objects.
const OBJECT_EXT: &str = "s42c";

/// A [`CacheStore`] implementation backed directly by the filesystem: content-addressed
/// objects, no lock files, no database, no index (plan §7.3).
pub struct FileCacheStore {
    config: CacheConfig,
    /// In-memory, per-process throttle for the best-effort access-time touch on validated
    /// hits. Never persisted; losing this state on restart only means the next hit touches
    /// again, which is harmless.
    last_touch: Mutex<HashMap<ArtifactKey, Instant>>,
    /// Monotonically increasing counter used to make temp file names unique within a process,
    /// in addition to the OS process id and a random suffix.
    tmp_counter: AtomicU64,
}

impl FileCacheStore {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            last_touch: Mutex::new(HashMap::new()),
            tmp_counter: AtomicU64::new(0),
        }
    }

    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    fn is_enabled(&self) -> bool {
        matches!(self.config.mode, CacheMode::Enabled)
    }

    fn object_path(&self, key: ArtifactKey) -> PathBuf {
        let (a, b) = key.shard();
        self.config
            .objects_dir()
            .join(a)
            .join(b)
            .join(format!("{}.{OBJECT_EXT}", key.hex()))
    }

    fn ensure_dirs(&self) -> std::io::Result<()> {
        fs::create_dir_all(self.config.objects_dir())?;
        fs::create_dir_all(self.config.tmp_dir())?;
        set_user_only_permissions(&self.config.root);
        Ok(())
    }

    fn unique_tmp_path(&self) -> PathBuf {
        let counter = self.tmp_counter.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        self.config
            .tmp_dir()
            .join(format!("{pid:x}-{nanos:x}-{counter:x}.tmp"))
    }

    /// Best-effort delete of a corrupt/mismatched object. May race with another writer that
    /// just replaced it with a valid object; that is acceptable (plan §7.5) — it can only
    /// produce another miss, never observed corruption.
    fn best_effort_delete(&self, path: &Path) {
        let _ = fs::remove_file(path);
    }

    fn should_touch(&self, key: ArtifactKey) -> bool {
        let mut guard = match self.last_touch.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        let now = Instant::now();
        match guard.get(&key) {
            Some(last) if now.duration_since(*last).as_secs() < TOUCH_MIN_INTERVAL_SECS => false,
            _ => {
                guard.insert(key, now);
                true
            }
        }
    }

    /// Best-effort touch of the object's mtime. A failure never changes lookup semantics.
    fn touch(&self, path: &Path) {
        let now = filetime_now();
        let _ = set_file_mtime(path, now);
    }

    /// Encodes and atomically publishes `envelope_bytes` at `final_path`. Handles the
    /// lost-race case (another writer published first) safely: since encoding is canonical,
    /// any valid winner for the same key is acceptable.
    fn atomic_publish(&self, final_path: &Path, envelope_bytes: &[u8]) -> Result<(), String> {
        self.ensure_dirs()
            .map_err(|e| format!("failed to create cache directories: {e}"))?;
        if let Some(parent) = final_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("failed to create shard dir: {e}"))?;
        }

        let tmp_path = self.unique_tmp_path();
        {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&tmp_path)
                .map_err(|e| format!("failed to create temp file: {e}"))?;
            file.write_all(envelope_bytes)
                .map_err(|e| format!("failed to write temp file: {e}"))?;
            file.flush()
                .map_err(|e| format!("failed to flush temp file: {e}"))?;
            file.sync_all()
                .map_err(|e| format!("failed to fsync temp file: {e}"))?;
            set_user_only_permissions(&tmp_path);
        }

        match fs::rename(&tmp_path, final_path) {
            Ok(()) => Ok(()),
            Err(rename_err) => {
                // On platforms/filesystems where rename cannot silently replace an existing
                // destination (notably Windows), a rename failure while the destination already
                // exists means another writer won the race. Since every valid object for this
                // key is canonically byte-identical, discard our temp file and treat this as
                // success rather than a write failure.
                if final_path.exists() {
                    self.best_effort_delete(&tmp_path);
                    Ok(())
                } else {
                    self.best_effort_delete(&tmp_path);
                    Err(format!("failed to publish object: {rename_err}"))
                }
            }
        }
    }

    /// After a successful write, scans the object tree and prunes if usage exceeds the
    /// configured budget (plan §8).
    fn maybe_prune_after_write(&self) -> Option<CacheMaintenanceReport> {
        let total = scan_total_bytes(&self.config.objects_dir());
        if total > self.config.max_bytes {
            Some(self.run_prune())
        } else {
            None
        }
    }

    fn run_prune(&self) -> CacheMaintenanceReport {
        let tmp_files_reaped = reap_abandoned_tmp_files(&self.config.tmp_dir());

        let mut objects = list_objects(&self.config.objects_dir());
        // Victim order: oldest mtime first, then artifact key (here: file name) for
        // deterministic ties (plan §8).
        objects.sort_by(|a, b| a.mtime.cmp(&b.mtime).then_with(|| a.path.cmp(&b.path)));

        let mut total: u64 = objects.iter().map(|o| o.len).sum();
        let target = self.config.prune_target_bytes;
        let mut objects_removed = 0u64;
        let mut bytes_reclaimed = 0u64;
        for obj in &objects {
            if total <= target {
                break;
            }
            match fs::remove_file(&obj.path) {
                Ok(()) => {
                    total = total.saturating_sub(obj.len);
                    objects_removed += 1;
                    bytes_reclaimed += obj.len;
                }
                Err(_) => {
                    // Deletion failure is nonfatal; continue with the next victim and report
                    // that the store remains over budget (plan §8).
                }
            }
        }

        CacheMaintenanceReport {
            objects_removed,
            bytes_reclaimed,
            objects_remaining: objects.len() as u64 - objects_removed,
            bytes_remaining: total,
            tmp_files_reaped,
            over_budget: total > target,
        }
    }
}

impl CacheStore for FileCacheStore {
    fn get<T: CacheArtifact>(&self, identity: &T::Identity) -> CacheLookup<T> {
        if !self.is_enabled() {
            return CacheLookup::Miss(CacheMissReason::Disabled);
        }
        let key = identity.artifact_key();
        let path = self.object_path(key);

        let bytes = match read_object_bounded(&path, &self.config.limits) {
            Ok(b) => b,
            Err(reason) => {
                if matches!(reason, CacheMissReason::ResourceLimit { .. }) {
                    self.best_effort_delete(&path);
                }
                return CacheLookup::Miss(reason);
            }
        };

        let decoded =
            match envelope::decode(&bytes, T::KIND, T::SCHEMA_VERSION, key, &self.config.limits) {
                Ok(d) => d,
                Err(reason) => {
                    self.best_effort_delete(&path);
                    return CacheLookup::Miss(reason);
                }
            };

        let value: T = match postcard::from_bytes(&decoded.payload) {
            Ok(v) => v,
            Err(e) => {
                self.best_effort_delete(&path);
                return CacheLookup::Miss(CacheMissReason::DecodeFailure {
                    detail: e.to_string(),
                });
            }
        };

        if let Err(detail) = value.validate_invariants() {
            self.best_effort_delete(&path);
            return CacheLookup::Miss(CacheMissReason::InvariantFailure { detail });
        }

        let touched = if self.should_touch(key) {
            self.touch(&path);
            true
        } else {
            false
        };

        CacheLookup::Hit(
            value,
            CacheHitMetadata {
                compressed_len: decoded.compressed_len,
                uncompressed_len: decoded.uncompressed_len,
                touched,
            },
        )
    }

    fn put<T: CacheArtifact>(&self, identity: &T::Identity, value: &T) -> CacheStoreOutcome {
        if !self.is_enabled() {
            return CacheStoreOutcome::Disabled;
        }
        let key = identity.artifact_key();

        let plain_payload = match postcard::to_allocvec(value) {
            Ok(p) => p,
            Err(e) => {
                return CacheStoreOutcome::Failed(CacheWriteFailure::Encode {
                    detail: e.to_string(),
                });
            }
        };

        let envelope_bytes = match envelope::encode(
            T::KIND,
            T::SCHEMA_VERSION,
            key,
            &plain_payload,
            &self.config.limits,
        ) {
            Ok(b) => b,
            Err(detail) => return CacheStoreOutcome::Failed(CacheWriteFailure::Encode { detail }),
        };

        let compressed_len = envelope_bytes.len() as u64;
        let uncompressed_len = plain_payload.len() as u64;

        let final_path = self.object_path(key);
        if let Err(detail) = self.atomic_publish(&final_path, &envelope_bytes) {
            return CacheStoreOutcome::Failed(CacheWriteFailure::Io { detail });
        }

        let pruned = self.maybe_prune_after_write();
        CacheStoreOutcome::Stored {
            compressed_len,
            uncompressed_len,
            pruned,
        }
    }

    fn status(&self) -> CacheStatus {
        let mut per_kind: std::collections::BTreeMap<ArtifactKind, CacheKindStats> =
            std::collections::BTreeMap::new();
        let mut total_bytes = 0u64;
        let mut object_count = 0u64;
        let mut unreadable_objects = 0u64;

        if self.is_enabled() {
            for obj in list_objects(&self.config.objects_dir()) {
                object_count += 1;
                total_bytes += obj.len;
                match read_kind_prefix(&obj.path) {
                    Some(kind) => {
                        let entry = per_kind.entry(kind).or_default();
                        entry.object_count += 1;
                        entry.total_bytes += obj.len;
                    }
                    None => unreadable_objects += 1,
                }
            }
        }

        CacheStatus {
            mode: self.config.mode,
            root: self.config.root.clone(),
            max_bytes: self.config.max_bytes,
            prune_target_bytes: self.config.prune_target_bytes,
            total_bytes,
            object_count,
            per_kind,
            over_budget: total_bytes > self.config.max_bytes,
            unreadable_objects,
        }
    }

    fn prune(&self) -> CacheMaintenanceReport {
        if !self.is_enabled() {
            return CacheMaintenanceReport::default();
        }
        self.run_prune()
    }

    fn clear(&self) -> CacheMaintenanceReport {
        if !self.is_enabled() {
            return CacheMaintenanceReport::default();
        }
        let tmp_files_reaped = reap_abandoned_tmp_files(&self.config.tmp_dir());
        let mut objects_removed = 0u64;
        let mut bytes_reclaimed = 0u64;
        let mut over_budget = false;
        for obj in list_objects(&self.config.objects_dir()) {
            match fs::remove_file(&obj.path) {
                Ok(()) => {
                    objects_removed += 1;
                    bytes_reclaimed += obj.len;
                }
                Err(_) => {
                    over_budget = true;
                }
            }
        }
        CacheMaintenanceReport {
            objects_removed,
            bytes_reclaimed,
            objects_remaining: 0,
            bytes_remaining: 0,
            tmp_files_reaped,
            over_budget,
        }
    }
}

struct ObjectFile {
    path: PathBuf,
    len: u64,
    mtime: SystemTime,
}

/// Walks the two-level sharded object tree. Tolerates entries disappearing mid-scan (plan
/// §7.4) by skipping any that fail to stat rather than failing the whole scan.
fn list_objects(objects_dir: &Path) -> Vec<ObjectFile> {
    let mut out = Vec::new();
    let Ok(level1) = fs::read_dir(objects_dir) else {
        return out;
    };
    for l1 in level1.flatten() {
        let l1_path = l1.path();
        if !l1_path.is_dir() {
            continue;
        }
        let Ok(level2) = fs::read_dir(&l1_path) else {
            continue;
        };
        for l2 in level2.flatten() {
            let l2_path = l2.path();
            if !l2_path.is_dir() {
                continue;
            }
            let Ok(files) = fs::read_dir(&l2_path) else {
                continue;
            };
            for f in files.flatten() {
                let path = f.path();
                if path.extension().and_then(|e| e.to_str()) != Some(OBJECT_EXT) {
                    continue;
                }
                let Ok(meta) = f.metadata() else { continue };
                if !meta.is_file() {
                    continue;
                }
                let mtime = meta.modified().unwrap_or(UNIX_EPOCH);
                out.push(ObjectFile {
                    path,
                    len: meta.len(),
                    mtime,
                });
            }
        }
    }
    out
}

fn scan_total_bytes(objects_dir: &Path) -> u64 {
    list_objects(objects_dir).iter().map(|o| o.len).sum()
}

/// Opens an object and reads it into memory, rejecting a file whose size already exceeds what
/// [`envelope::decode`] could accept.
///
/// The size is taken from the open handle rather than from a separate `fs::metadata` call so a
/// concurrent republication cannot swap the file between the check and the read. Bounding the
/// read here is what keeps a corrupt or hostile oversized object from being allocated in full
/// before the envelope's own length checks run (plan §7.2, §7.4, prerequisite B10).
fn read_object_bounded(path: &Path, limits: &CacheLimits) -> Result<Vec<u8>, CacheMissReason> {
    use std::io::Read;

    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(CacheMissReason::NotFound);
        }
        Err(e) => {
            return Err(CacheMissReason::IoFailure {
                detail: e.to_string(),
            });
        }
    };

    let max = envelope::max_object_file_bytes(limits);
    let size = match file.metadata() {
        Ok(meta) => meta.len(),
        Err(e) => {
            return Err(CacheMissReason::IoFailure {
                detail: e.to_string(),
            })
        }
    };
    if size > max {
        return Err(CacheMissReason::ResourceLimit {
            detail: format!("object file is {size} bytes, exceeding the {max} byte limit"),
        });
    }

    // Read at most `max + 1` bytes so a file that grows between the stat and the read is still
    // bounded, and is then rejected for exceeding the limit rather than truncated into a
    // plausible-looking envelope.
    let mut bytes = Vec::with_capacity(size as usize);
    if let Err(e) = file.take(max + 1).read_to_end(&mut bytes) {
        return Err(CacheMissReason::IoFailure {
            detail: e.to_string(),
        });
    }
    if bytes.len() as u64 > max {
        return Err(CacheMissReason::ResourceLimit {
            detail: format!("object file exceeds the {max} byte limit"),
        });
    }
    Ok(bytes)
}

/// Reads just enough of an object to recover its `ArtifactKind` for status reporting.
///
/// Status and prune scan every object in the store, so this reads only the fixed header prefix
/// rather than the whole file; pulling a multi-megabyte payload into memory to recover one byte
/// would make maintenance cost scale with cache bytes instead of object count (plan §7.3).
fn read_kind_prefix(path: &Path) -> Option<ArtifactKind> {
    use std::io::Read;

    let mut file = fs::File::open(path).ok()?;
    let mut prefix = [0u8; envelope::KIND_OFFSET + 1];
    file.read_exact(&mut prefix).ok()?;
    if &prefix[0..4] != envelope::MAGIC {
        return None;
    }
    ArtifactKind::from_u8(prefix[envelope::KIND_OFFSET])
}

/// Removes temp files older than [`TMP_REAP_MIN_AGE_SECS`]. A race that removes a still-active
/// temp file only fails that optional write; it never affects a published object (plan §7.4).
fn reap_abandoned_tmp_files(tmp_dir: &Path) -> u64 {
    let mut reaped = 0u64;
    let Ok(entries) = fs::read_dir(tmp_dir) else {
        return 0;
    };
    let now = SystemTime::now();
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_file() {
            continue;
        }
        let Ok(modified) = meta.modified() else {
            continue;
        };
        let age = now.duration_since(modified).unwrap_or_default();
        if age.as_secs() >= TMP_REAP_MIN_AGE_SECS && fs::remove_file(&path).is_ok() {
            reaped += 1;
        }
    }
    reaped
}

fn filetime_now() -> SystemTime {
    SystemTime::now()
}

/// Best-effort mtime update used for the LRU access-time touch. Implemented via
/// [`fs::File::set_modified`] where supported; failures are swallowed by the caller.
fn set_file_mtime(path: &Path, time: SystemTime) -> std::io::Result<()> {
    let file = fs::OpenOptions::new().write(true).open(path)?;
    file.set_modified(time)
}

#[cfg(unix)]
fn set_user_only_permissions(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = fs::metadata(path) {
        let mut perms = meta.permissions();
        if meta.is_dir() {
            perms.set_mode(0o700);
        } else {
            perms.set_mode(0o600);
        }
        let _ = fs::set_permissions(path, perms);
    }
}

#[cfg(not(unix))]
fn set_user_only_permissions(_path: &Path) {
    // The plan requires user-only permissions "where the platform supports it". Non-Unix
    // platforms rely on their own ACL model (e.g. per-user profile directories on Windows);
    // no portable equivalent is applied here.
}
