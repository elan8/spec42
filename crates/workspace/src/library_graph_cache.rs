//! Disk cache for the fully-built semantic graph of library files.
//!
//! Building the library graph on every startup requires ~10 s of disk I/O
//! (walking library directories + reading files) plus ~2 s of graph construction.
//! Library files never change between sessions, so we persist the result.
//!
//! # On-disk format
//!
//! One file per library configuration: `<cache_dir>/<hex-sha256>.bin`
//!
//! ```text
//! [magic:       4 bytes  "LGCX"]
//! [version:    16 bytes  spec42 semver string, zero-padded]
//! [payload:    remainder — bincode-encoded LibraryGraphCachePayload]
//! ```
//!
//! # Invalidation (two-level)
//!
//! **Level 1 — filename key**: SHA-256 of sorted library path strings +
//! `CARGO_PKG_VERSION`. Invalidates on path config changes or binary upgrades.
//!
//! **Level 2 — file metadata fingerprint** (inside payload): sorted list of
//! `(path, size_bytes, mtime_secs)` for every `.sysml`/`.kerml` file under each
//! library root. Checked with `fs::metadata()` only (no file reads) on every
//! cache load. Invalidates when the user manually replaces or upgrades library
//! files in-place.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

use crate::semantic::SemanticGraph;

const MAGIC: &[u8; 4] = b"LGCX";
const VERSION_FIELD_LEN: usize = 16;

fn version_field() -> [u8; VERSION_FIELD_LEN] {
    let v = env!("CARGO_PKG_VERSION").as_bytes();
    let mut field = [0u8; VERSION_FIELD_LEN];
    let len = v.len().min(VERSION_FIELD_LEN);
    field[..len].copy_from_slice(&v[..len]);
    field
}

/// Per-file metadata snapshot used for Level-2 invalidation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FileMetaEntry {
    /// Path relative to its library root, forward-slash separated.
    rel_path: String,
    size_bytes: u64,
    mtime_secs: u64,
}

/// The serialized payload stored in the cache file.
#[derive(Serialize, Deserialize)]
struct LibraryGraphCachePayload {
    /// Sorted list of library root paths (for human-readable validation).
    library_paths: Vec<String>,
    /// Level-2 fingerprint: sorted metadata of all library source files.
    file_fingerprint: Vec<FileMetaEntry>,
    /// The fully-built semantic graph for all library files.
    graph: SemanticGraph,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Resolve the platform-specific cache directory for library graph caches.
pub fn default_cache_dir() -> Option<PathBuf> {
    Some(
        dirs::cache_dir()?
            .join("spec42")
            .join("library-graph-cache"),
    )
}

/// Try to load a cached [`SemanticGraph`] for the given library paths.
///
/// Returns `None` on any miss, version mismatch, stale file metadata, or
/// decode error. All failures are silent.
pub fn load(library_paths: &[Url]) -> Option<SemanticGraph> {
    let cache_dir = default_cache_dir()?;
    let key = cache_key(library_paths);
    let path = entry_path(&cache_dir, &key);

    tracing::debug!(
        path = %path.display(),
        "library graph cache: attempting load"
    );

    let mut file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            tracing::debug!(path = %path.display(), "library graph cache: file not found");
            return None;
        }
    };

    // Check magic + version header.
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic).ok()?;
    if &magic != MAGIC {
        tracing::debug!("library graph cache: magic mismatch");
        return None;
    }
    let mut version = [0u8; VERSION_FIELD_LEN];
    file.read_exact(&mut version).ok()?;
    if version != version_field() {
        tracing::debug!(
            stored = %String::from_utf8_lossy(&version),
            current = %env!("CARGO_PKG_VERSION"),
            "library graph cache: version mismatch"
        );
        return None;
    }

    // Decode payload.
    let mut payload_bytes = Vec::new();
    file.read_to_end(&mut payload_bytes).ok()?;
    let payload: LibraryGraphCachePayload = match serde_json::from_slice(&payload_bytes) {
        Ok(v) => v,
        Err(e) => {
            tracing::debug!(err = %e, "library graph cache: json decode failed");
            return None;
        }
    };

    // Level-2 check: verify file metadata fingerprint.
    if !fingerprint_valid(&payload.file_fingerprint, library_paths) {
        tracing::debug!(
            stored_entries = payload.file_fingerprint.len(),
            "library graph cache: file metadata fingerprint mismatch"
        );
        return None;
    }

    tracing::debug!(
        library_paths = payload.library_paths.len(),
        "library graph cache: HIT"
    );
    Some(payload.graph)
}

/// Persist a built [`SemanticGraph`] for the given library paths.
///
/// Creates the cache directory if needed. Silently ignores all errors —
/// caching is always best-effort.
pub fn store(library_paths: &[Url], graph: &SemanticGraph) {
    tracing::debug!("library graph cache: attempting store");
    if let Err(e) = store_inner(library_paths, graph) {
        tracing::warn!("library graph cache store failed: {e}");
    } else {
        tracing::debug!("library graph cache: store succeeded");
    }
}

/// Delete cache entries whose version header does not match the current binary.
/// Call once at startup on a background thread. Non-fatal.
pub fn evict_stale_entries() {
    let Some(cache_dir) = default_cache_dir() else {
        return;
    };
    let Ok(entries) = std::fs::read_dir(&cache_dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("bin") {
            continue;
        }
        if is_stale(&path) {
            let _ = std::fs::remove_file(&path);
        }
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Level-1 cache key: SHA-256 of sorted library paths + binary version.
fn cache_key(library_paths: &[Url]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(env!("CARGO_PKG_VERSION").as_bytes());
    let mut sorted: Vec<&str> = library_paths.iter().map(|u| u.as_str()).collect();
    sorted.sort_unstable();
    for path in sorted {
        hasher.update(path.as_bytes());
        hasher.update(b"\0");
    }
    hasher.finalize().into()
}

fn entry_path(cache_dir: &Path, key: &[u8; 32]) -> PathBuf {
    let hex: String = key.iter().map(|b| format!("{b:02x}")).collect();
    cache_dir.join(format!("{hex}.bin"))
}

/// Build a metadata fingerprint by walking all library root directories.
/// Only `.sysml` and `.kerml` files are included.
fn build_fingerprint(library_paths: &[Url]) -> Vec<FileMetaEntry> {
    let mut entries = Vec::new();
    for root_url in library_paths {
        let Ok(root_path) = root_url.to_file_path() else {
            continue;
        };
        let walker = walkdir::WalkDir::new(&root_path)
            .follow_links(false)
            .sort_by_file_name();
        for entry in walker.into_iter().flatten() {
            let ext = entry.path().extension().and_then(|e| e.to_str());
            if !matches!(ext, Some("sysml") | Some("kerml")) {
                continue;
            }
            let Ok(meta) = entry.metadata() else {
                continue;
            };
            let mtime_secs = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let rel_path = entry
                .path()
                .strip_prefix(&root_path)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .replace('\\', "/");
            entries.push(FileMetaEntry {
                rel_path,
                size_bytes: meta.len(),
                mtime_secs,
            });
        }
    }
    entries.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    entries
}

/// Check that the stored fingerprint still matches current disk state.
fn fingerprint_valid(stored: &[FileMetaEntry], library_paths: &[Url]) -> bool {
    let current = build_fingerprint(library_paths);
    stored == current.as_slice()
}

fn store_inner(
    library_paths: &[Url],
    graph: &SemanticGraph,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = default_cache_dir().ok_or("no cache dir")?;
    std::fs::create_dir_all(&cache_dir)?;

    let file_fingerprint = build_fingerprint(library_paths);
    let payload = LibraryGraphCachePayload {
        library_paths: library_paths.iter().map(|u| u.to_string()).collect(),
        file_fingerprint,
        graph: graph.clone(),
    };

    let payload_bytes = serde_json::to_vec(&payload)?;

    let key = cache_key(library_paths);
    let path = entry_path(&cache_dir, &key);
    let mut file = std::fs::File::create(&path)?;
    file.write_all(MAGIC)?;
    file.write_all(&version_field())?;
    file.write_all(&payload_bytes)?;
    Ok(())
}

fn is_stale(path: &Path) -> bool {
    let Ok(mut file) = std::fs::File::open(path) else {
        return true;
    };
    let mut magic = [0u8; 4];
    if file.read_exact(&mut magic).is_err() || &magic != MAGIC {
        return true;
    }
    let mut version = [0u8; VERSION_FIELD_LEN];
    if file.read_exact(&mut version).is_err() {
        return true;
    }
    version != version_field()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn cache_key_is_deterministic_and_order_independent() {
        let a = Url::parse("file:///lib/a").unwrap();
        let b = Url::parse("file:///lib/b").unwrap();
        let key1 = cache_key(&[a.clone(), b.clone()]);
        let key2 = cache_key(&[b.clone(), a.clone()]);
        assert_eq!(key1, key2, "cache key must be order-independent");

        let c = Url::parse("file:///lib/c").unwrap();
        let key3 = cache_key(&[a, b, c]);
        assert_ne!(key1, key3, "different paths must produce different keys");
    }

    #[test]
    fn fingerprint_detects_missing_directory() {
        let stored = vec![FileMetaEntry {
            rel_path: "Foo.sysml".into(),
            size_bytes: 42,
            mtime_secs: 1000,
        }];
        // Non-existent library path → current fingerprint is empty → mismatch.
        let bogus = Url::parse("file:///nonexistent/lib").unwrap();
        assert!(!fingerprint_valid(&stored, &[bogus]));
    }

    #[test]
    fn empty_library_paths_produces_empty_fingerprint() {
        let fp = build_fingerprint(&[]);
        assert!(fp.is_empty());
    }

    #[test]
    fn round_trip_empty_graph() {
        let dir = tempdir().unwrap();

        // Override cache dir by testing store_inner + load_inner directly.
        let graph = SemanticGraph::default();
        let payload = LibraryGraphCachePayload {
            library_paths: vec![],
            file_fingerprint: vec![],
            graph: graph.clone(),
        };
        let payload_bytes = serde_json::to_vec(&payload).unwrap();

        let key = [0u8; 32];
        let path = entry_path(dir.path(), &key);
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(MAGIC).unwrap();
        file.write_all(&version_field()).unwrap();
        file.write_all(&payload_bytes).unwrap();
        drop(file);

        // Decode it back.
        let mut file = std::fs::File::open(&path).unwrap();
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic).unwrap();
        assert_eq!(&magic, MAGIC);
        let mut version = [0u8; VERSION_FIELD_LEN];
        file.read_exact(&mut version).unwrap();
        assert_eq!(version, version_field());
        let mut payload_bytes2 = Vec::new();
        file.read_to_end(&mut payload_bytes2).unwrap();
        let decoded: LibraryGraphCachePayload =
            serde_json::from_slice(&payload_bytes2).unwrap();

        assert_eq!(decoded.library_paths, Vec::<String>::new());
        assert_eq!(decoded.graph.graph.node_count(), 0);
    }
}
