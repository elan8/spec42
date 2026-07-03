//! Disk cache for parsed [`RootNamespace`] values.
//!
//! Libraries are read-only between spec42 upgrades. Parsing them on every
//! server start is wasteful. This module caches the parse result keyed by the
//! SHA-256 of the source text so that subsequent starts skip parsing entirely.
//!
//! # On-disk format
//!
//! One file per source file: `<cache_dir>/<hex-sha256>.bin`
//!
//! ```text
//! [magic:   4 bytes  "KPC\0"]
//! [version: 16 bytes spec42 semver string, zero-padded]
//! [payload: remainder — bincode-encoded RootNamespace]
//! ```
//!
//! The 20-byte header allows fast staleness detection without decoding the tree.
//! The version field is taken from this crate's Cargo version (shared across the
//! whole workspace via `version.workspace = true`), so every release automatically
//! invalidates the cache without any manual bump.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use sysml_v2_parser::RootNamespace;

const MAGIC: &[u8; 4] = b"KPC\0";
const VERSION_FIELD_LEN: usize = 16;

fn version_field() -> [u8; VERSION_FIELD_LEN] {
    // First 12 bytes: spec42 semver string; last 4 bytes: PARSE_AST_VERSION (le).
    // Incorporating the parser schema version invalidates caches when the AST
    // schema changes between parser releases, even within a single spec42 version.
    let spec42 = env!("CARGO_PKG_VERSION").as_bytes();
    let mut field = [0u8; VERSION_FIELD_LEN];
    let spec42_len = spec42.len().min(12);
    field[..spec42_len].copy_from_slice(&spec42[..spec42_len]);
    field[12..16].copy_from_slice(&sysml_v2_parser::PARSE_AST_VERSION.to_le_bytes());
    field
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Resolve the platform-specific cache directory for spec42 parse results.
/// Returns `None` if the OS provides no suitable cache directory.
pub fn default_cache_dir() -> Option<PathBuf> {
    Some(dirs::cache_dir()?.join("spec42").join("parse-cache"))
}

/// Compute the SHA-256 of `bytes` and return it as a 32-byte array.
pub fn content_hash(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

/// Try to load a cached [`RootNamespace`] for a file whose content hashes to `hash`.
///
/// Returns `None` on any miss, version mismatch, or I/O / decoding error.
/// All failures are silent — the caller falls back to a fresh parse.
pub fn load(cache_dir: &Path, hash: &[u8; 32]) -> Option<RootNamespace> {
    let path = entry_path(cache_dir, hash);
    let mut file = std::fs::File::open(&path).ok()?;

    let mut magic = [0u8; 4];
    file.read_exact(&mut magic).ok()?;
    if &magic != MAGIC {
        return None;
    }
    let mut version = [0u8; VERSION_FIELD_LEN];
    file.read_exact(&mut version).ok()?;
    if version != version_field() {
        return None;
    }

    let mut payload = Vec::new();
    file.read_to_end(&mut payload).ok()?;

    let config = bincode::config::standard();
    bincode::serde::decode_from_slice::<RootNamespace, _>(&payload, config)
        .ok()
        .map(|(root, _)| root)
}

/// Write a freshly parsed [`RootNamespace`] to the cache.
///
/// Creates the cache directory if needed. Silently ignores all I/O errors —
/// caching is always a best-effort optimisation.
pub fn store(cache_dir: &Path, hash: &[u8; 32], root: &RootNamespace) {
    if let Err(e) = store_inner(cache_dir, hash, root) {
        tracing::debug!("parse cache store failed (non-fatal): {e}");
    }
}

/// Delete cache entries whose `ast_version` header does not match the current
/// binary's [`AST_VERSION`]. Call once at startup on a background thread.
/// Non-fatal — errors are silently ignored.
pub fn evict_stale_entries(cache_dir: &Path) {
    let Ok(entries) = std::fs::read_dir(cache_dir) else {
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

fn entry_path(cache_dir: &Path, hash: &[u8; 32]) -> PathBuf {
    let hex = hex_encode(hash);
    cache_dir.join(format!("{hex}.bin"))
}

fn hex_encode(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn store_inner(
    cache_dir: &Path,
    hash: &[u8; 32],
    root: &RootNamespace,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(cache_dir)?;

    let config = bincode::config::standard();
    let payload = bincode::serde::encode_to_vec(root, config)?;

    let path = entry_path(cache_dir, hash);
    let mut file = std::fs::File::create(&path)?;
    file.write_all(MAGIC)?;
    file.write_all(&version_field())?;
    file.write_all(&payload)?;
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
    use sysml_v2_parser::RootNamespace;
    use tempfile::tempdir;

    fn parse(src: &str) -> RootNamespace {
        sysml_v2_parser::parse(src).expect("parse")
    }

    #[test]
    fn round_trip_store_and_load() {
        let dir = tempdir().unwrap();
        let root = parse("package Demo { part def Engine; }");
        let hash = content_hash(b"package Demo { part def Engine; }");

        store(dir.path(), &hash, &root);
        let loaded = load(dir.path(), &hash).expect("should hit cache");
        assert_eq!(root, loaded);
    }

    #[test]
    fn load_returns_none_for_unknown_hash() {
        let dir = tempdir().unwrap();
        let hash = content_hash(b"not stored");
        assert!(load(dir.path(), &hash).is_none());
    }

    #[test]
    fn load_returns_none_for_wrong_version() {
        let dir = tempdir().unwrap();
        let root = parse("package X {}");
        let hash = content_hash(b"package X {}");

        // Write with a deliberately wrong version
        let path = entry_path(dir.path(), &hash);
        let config = bincode::config::standard();
        let payload = bincode::serde::encode_to_vec(&root, config).unwrap();
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(MAGIC).unwrap();
        f.write_all(&[0u8; VERSION_FIELD_LEN]).unwrap(); // all-zero != any real version
        f.write_all(&payload).unwrap();

        assert!(load(dir.path(), &hash).is_none());
    }

    #[test]
    fn evict_removes_stale_entries_and_keeps_current() {
        let dir = tempdir().unwrap();
        let root = parse("package Keep {}");
        let keep_hash = content_hash(b"keep");
        store(dir.path(), &keep_hash, &root);

        // Write a stale entry manually
        let stale_hash = content_hash(b"stale");
        let stale_path = entry_path(dir.path(), &stale_hash);
        let mut f = std::fs::File::create(&stale_path).unwrap();
        f.write_all(MAGIC).unwrap();
        f.write_all(&[0u8; VERSION_FIELD_LEN]).unwrap(); // all-zero != any real version
        f.write_all(&[0u8; 8]).unwrap();

        evict_stale_entries(dir.path());

        assert!(!stale_path.exists(), "stale entry should be deleted");
        assert!(
            load(dir.path(), &keep_hash).is_some(),
            "current entry should survive eviction"
        );
    }

    #[test]
    fn content_hash_is_deterministic() {
        let a = content_hash(b"hello world");
        let b = content_hash(b"hello world");
        assert_eq!(a, b);
        let c = content_hash(b"hello world!");
        assert_ne!(a, c);
    }
}
