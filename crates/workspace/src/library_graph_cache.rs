//! Disk cache for the workspace-scoped semantic graph of library files.
//!
//! Building the library graph on every startup requires ~10 s of disk I/O
//! (walking library directories + reading files) plus ~2 s of graph construction.
//! Library files rarely change between sessions, so we persist the result. Because
//! import-scoped loading produces a workspace-specific subset, the cache identity also
//! includes the workspace facts that seed library closure resolution.
//!
//! # On-disk format
//!
//! One file per library configuration: `<cache_dir>/<hex-sha256>.bin`
//!
//! ```text
//! [magic:       4 bytes  "LGCX"]
//! [version:    20 bytes  spec42 semver (12 bytes, zero-padded) + PARSE_AST_VERSION (4 bytes, le)
//!                       + library-graph semantics version (4 bytes, le)]
//! [payload:    remainder — bincode-encoded LibraryGraphCachePayload]
//! ```
//!
//! # Invalidation (two-level)
//!
//! **Level 1 — filename key**: SHA-256 of sorted library path strings +
//! `CARGO_PKG_VERSION` + `sysml_v2_parser::PARSE_AST_VERSION` + the library-graph semantics
//! version. Invalidates on path config changes, spec42 binary upgrades, parser AST changes, or
//! semantic changes to library closure/merging within the same development version.
//!
//! **Level 2 — content fingerprint** (inside payload): sorted SHA-256 digests
//! of every `.sysml`/`.kerml` file under each library root. Checked on every
//! cache load. This rejects in-place replacements even when file paths, sizes,
//! and modification times are preserved.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

use crate::semantic::SemanticGraph;

const MAGIC: &[u8; 4] = b"LGCX";
const VERSION_FIELD_LEN: usize = 20;
const LIBRARY_GRAPH_SEMANTICS_VERSION: u32 = 3;

fn version_field() -> [u8; VERSION_FIELD_LEN] {
    // First 12 bytes: spec42 semver string; then PARSE_AST_VERSION and the graph-semantics
    // version (both le u32). The latter invalidates cached graphs after closure/merge behavior
    // changes even when the in-development Spec42 semver and parser schema stay unchanged.
    let v = env!("CARGO_PKG_VERSION").as_bytes();
    let mut field = [0u8; VERSION_FIELD_LEN];
    let len = v.len().min(12);
    field[..len].copy_from_slice(&v[..len]);
    field[12..16].copy_from_slice(&sysml_v2_parser::PARSE_AST_VERSION.to_le_bytes());
    field[16..20].copy_from_slice(&LIBRARY_GRAPH_SEMANTICS_VERSION.to_le_bytes());
    field
}

/// Per-file content snapshot used for Level-2 invalidation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct FileFingerprintEntry {
    /// Canonical URL of the library root containing this file.
    library_root: String,
    /// Path relative to its library root, forward-slash separated.
    rel_path: String,
    /// SHA-256 digest of the source bytes.
    content_hash: [u8; 32],
}

/// The serialized payload stored in the cache file.
#[derive(Serialize, Deserialize)]
struct LibraryGraphCachePayload {
    /// Sorted list of library root paths (for human-readable validation).
    library_paths: Vec<String>,
    /// Workspace facts that determined the import-scoped library subset.
    closure_seed_signature: Vec<String>,
    /// Level-2 fingerprint: sorted content digests of all library source files.
    file_fingerprint: Vec<FileFingerprintEntry>,
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
/// Returns `None` on any miss, version mismatch, stale file content, or
/// decode error. All failures are silent.
pub fn load(library_paths: &[Url], closure_seed_signature: &[String]) -> Option<SemanticGraph> {
    let cache_dir = default_cache_dir()?;
    let key = cache_key(library_paths, closure_seed_signature);
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

    if payload.closure_seed_signature != normalized_signature(closure_seed_signature) {
        tracing::debug!("library graph cache: workspace closure signature mismatch");
        return None;
    }

    // Level-2 check: verify file content fingerprint.
    if !fingerprint_valid(&payload.file_fingerprint, library_paths) {
        tracing::debug!(
            stored_entries = payload.file_fingerprint.len(),
            "library graph cache: file content fingerprint mismatch"
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
pub fn store(library_paths: &[Url], closure_seed_signature: &[String], graph: &SemanticGraph) {
    tracing::debug!("library graph cache: attempting store");
    if let Err(e) = store_inner(library_paths, closure_seed_signature, graph) {
        tracing::warn!("library graph cache store failed: {e}");
    } else {
        tracing::debug!("library graph cache: store succeeded");
    }
}

/// Delete every cached entry, regardless of key. Used when a library's
/// resolution changes (enabled/disabled, overridden to a local path) so a
/// stale entry for the previous configuration doesn't linger on disk.
///
/// Safe to call even when nothing is cached; returns the number of files removed.
pub fn clear_all() -> Result<usize, String> {
    let Some(cache_dir) = default_cache_dir() else {
        return Ok(0);
    };
    clear_all_in(&cache_dir)
}

fn clear_all_in(cache_dir: &Path) -> Result<usize, String> {
    let entries = match std::fs::read_dir(cache_dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(format!("Failed to read {}: {e}", cache_dir.display())),
    };
    let mut cleared = 0usize;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("bin") {
            continue;
        }
        std::fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove {}: {e}", path.display()))?;
        cleared += 1;
    }
    Ok(cleared)
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

/// Level-1 cache key: SHA-256 of sorted library paths, workspace closure seeds, binary
/// version, parser AST schema version, and graph-semantics version (so either kind of semantic
/// change gets a fresh filename too, not just a header mismatch -- see `version_field`).
fn cache_key(library_paths: &[Url], closure_seed_signature: &[String]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(env!("CARGO_PKG_VERSION").as_bytes());
    hasher.update(sysml_v2_parser::PARSE_AST_VERSION.to_le_bytes());
    hasher.update(LIBRARY_GRAPH_SEMANTICS_VERSION.to_le_bytes());
    let mut sorted: Vec<&str> = library_paths.iter().map(|u| u.as_str()).collect();
    sorted.sort_unstable();
    for path in sorted {
        hasher.update(path.as_bytes());
        hasher.update(b"\0");
    }
    // This marker guarantees that entries produced by the old path-only scheme are
    // never reused, even when the workspace signature is empty.
    hasher.update(b"workspace-closure-seeds-v1\0");
    for seed in normalized_signature(closure_seed_signature) {
        hasher.update(seed.as_bytes());
        hasher.update(b"\0");
    }
    hasher.finalize().into()
}

fn normalized_signature(signature: &[String]) -> Vec<String> {
    let mut normalized = signature.to_vec();
    normalized.sort_unstable();
    normalized.dedup();
    normalized
}

fn entry_path(cache_dir: &Path, key: &[u8; 32]) -> PathBuf {
    let hex: String = key.iter().map(|b| format!("{b:02x}")).collect();
    cache_dir.join(format!("{hex}.bin"))
}

/// Build a content fingerprint by walking all library root directories.
/// Only `.sysml` and `.kerml` files are included.
///
/// `None` means a source file could not be read, so the caller must not treat
/// the resulting graph as a dependency-complete cached result.
fn build_fingerprint(library_paths: &[Url]) -> Option<Vec<FileFingerprintEntry>> {
    let mut entries = Vec::new();
    for root_url in library_paths {
        let Ok(root_path) = root_url.to_file_path() else {
            continue;
        };
        let walker = walkdir::WalkDir::new(&root_path)
            .follow_links(false)
            .sort_by_file_name();
        for entry in walker {
            let entry = entry.ok()?;
            let ext = entry.path().extension().and_then(|e| e.to_str());
            if !matches!(ext, Some("sysml") | Some("kerml")) {
                continue;
            }
            let rel_path = entry
                .path()
                .strip_prefix(&root_path)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .replace('\\', "/");
            let content_hash = crate::parse_cache::content_hash(&std::fs::read(entry.path()).ok()?);
            entries.push(FileFingerprintEntry {
                library_root: root_url.as_str().to_owned(),
                rel_path,
                content_hash,
            });
        }
    }
    entries.sort_by(|a, b| {
        a.library_root
            .cmp(&b.library_root)
            .then_with(|| a.rel_path.cmp(&b.rel_path))
    });
    Some(entries)
}

/// Check that the stored fingerprint still matches current disk state.
fn fingerprint_valid(stored: &[FileFingerprintEntry], library_paths: &[Url]) -> bool {
    build_fingerprint(library_paths).is_some_and(|current| stored == current)
}

fn store_inner(
    library_paths: &[Url],
    closure_seed_signature: &[String],
    graph: &SemanticGraph,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = default_cache_dir().ok_or("no cache dir")?;
    std::fs::create_dir_all(&cache_dir)?;

    let file_fingerprint = build_fingerprint(library_paths).ok_or("library source unavailable")?;
    let payload = LibraryGraphCachePayload {
        library_paths: library_paths.iter().map(|u| u.to_string()).collect(),
        closure_seed_signature: normalized_signature(closure_seed_signature),
        file_fingerprint,
        graph: graph.clone(),
    };

    let payload_bytes = serde_json::to_vec(&payload)?;

    let key = cache_key(library_paths, closure_seed_signature);
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
        let signature = vec!["import:ScalarValues::*".to_string()];
        let key1 = cache_key(&[a.clone(), b.clone()], &signature);
        let key2 = cache_key(&[b.clone(), a.clone()], &signature);
        assert_eq!(key1, key2, "cache key must be order-independent");

        let c = Url::parse("file:///lib/c").unwrap();
        let key3 = cache_key(&[a, b, c], &signature);
        assert_ne!(key1, key3, "different paths must produce different keys");
    }

    #[test]
    fn cache_key_changes_with_workspace_library_closure() {
        let library = Url::parse("file:///lib").unwrap();
        let without_metadata = vec!["import:ScalarValues::*".to_string()];
        let with_metadata = vec![
            "import:ScalarValues::*".to_string(),
            "import:ModelingMetadata::*".to_string(),
        ];

        assert_ne!(
            cache_key(std::slice::from_ref(&library), &without_metadata),
            cache_key(std::slice::from_ref(&library), &with_metadata),
            "a graph without ModelingMetadata must not satisfy a workspace that imports it"
        );
    }

    #[test]
    fn fingerprint_detects_missing_directory() {
        let stored = vec![FileFingerprintEntry {
            library_root: "file:///nonexistent/lib".into(),
            rel_path: "Foo.sysml".into(),
            content_hash: crate::parse_cache::content_hash(b"contents"),
        }];
        // Non-existent library path → current fingerprint is empty → mismatch.
        let bogus = Url::parse("file:///nonexistent/lib").unwrap();
        assert!(!fingerprint_valid(&stored, &[bogus]));
    }

    #[test]
    fn empty_library_paths_produces_empty_fingerprint() {
        let fp = build_fingerprint(&[]).unwrap();
        assert!(fp.is_empty());
    }

    #[test]
    fn fingerprint_accepts_unchanged_library_contents() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("Library.sysml"), b"package Library {}").unwrap();
        let library = Url::from_directory_path(dir.path()).unwrap();
        let stored = build_fingerprint(std::slice::from_ref(&library)).unwrap();

        assert!(
            fingerprint_valid(&stored, &[library]),
            "unchanged library bytes must keep the cache valid"
        );
    }

    #[test]
    fn fingerprint_rejects_same_size_and_mtime_content_replacement() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("Library.sysml");
        let original = b"package First {}";
        let replacement = b"package Other {}";
        assert_eq!(original.len(), replacement.len());

        std::fs::write(&source, original).unwrap();
        let original_metadata = std::fs::metadata(&source).unwrap();
        let original_mtime = original_metadata.modified().unwrap();
        let library = Url::from_directory_path(dir.path()).unwrap();
        let stored = build_fingerprint(std::slice::from_ref(&library)).unwrap();

        std::fs::write(&source, replacement).unwrap();
        std::fs::File::open(&source)
            .unwrap()
            .set_times(std::fs::FileTimes::new().set_modified(original_mtime))
            .unwrap();
        let replacement_metadata = std::fs::metadata(&source).unwrap();
        assert_eq!(replacement_metadata.len(), original_metadata.len());
        assert_eq!(replacement_metadata.modified().unwrap(), original_mtime);

        assert!(
            !fingerprint_valid(&stored, &[library]),
            "a byte replacement must invalidate the cache even with equal size and mtime"
        );
    }

    #[test]
    fn clear_all_removes_bin_files_and_leaves_others() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("a.bin"), b"x").unwrap();
        std::fs::write(dir.path().join("b.bin"), b"y").unwrap();
        std::fs::write(dir.path().join("keep.txt"), b"z").unwrap();

        let cleared = clear_all_in(dir.path()).unwrap();

        assert_eq!(cleared, 2);
        assert!(dir.path().join("keep.txt").exists());
        assert!(!dir.path().join("a.bin").exists());
        assert!(!dir.path().join("b.bin").exists());
    }

    #[test]
    fn clear_all_on_missing_dir_returns_zero() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("does-not-exist");
        assert_eq!(clear_all_in(&missing).unwrap(), 0);
    }

    #[test]
    fn round_trip_empty_graph() {
        let dir = tempdir().unwrap();

        // Override cache dir by testing store_inner + load_inner directly.
        let graph = SemanticGraph::default();
        let payload = LibraryGraphCachePayload {
            library_paths: vec![],
            closure_seed_signature: vec![],
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
        let decoded: LibraryGraphCachePayload = serde_json::from_slice(&payload_bytes2).unwrap();

        assert_eq!(decoded.library_paths, Vec::<String>::new());
        assert_eq!(decoded.graph.graph.node_count(), 0);
    }
}
