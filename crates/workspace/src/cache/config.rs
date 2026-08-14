//! Cache store configuration (plan §7.1, §8).

use std::path::PathBuf;

/// Default maximum total size of the cache before a prune is triggered (5 GiB).
pub const DEFAULT_MAX_BYTES: u64 = 5 * 1024 * 1024 * 1024;
/// Default prune target: usage is pruned back down to this size (4 GiB).
pub const DEFAULT_PRUNE_TARGET_BYTES: u64 = 4 * 1024 * 1024 * 1024;
/// Default cap on a single object's declared uncompressed payload length. Guards against
/// decompression bombs and pathological inputs independent of the disk budget.
pub const DEFAULT_MAX_UNCOMPRESSED_OBJECT_BYTES: u64 = 512 * 1024 * 1024;
/// Default cap on a single object's file length on disk.
pub const DEFAULT_MAX_COMPRESSED_OBJECT_BYTES: u64 = 128 * 1024 * 1024;
/// Environment variable that overrides the default cache root.
pub const CACHE_DIR_ENV_VAR: &str = "SPEC42_CACHE_DIR";
/// Minimum interval between best-effort access-time touches of the same object, per process.
pub const TOUCH_MIN_INTERVAL_SECS: u64 = 60;
/// Minimum age of an abandoned temp file before maintenance reaps it.
pub const TMP_REAP_MIN_AGE_SECS: u64 = 60 * 60;

/// Whether the cache store performs reads/writes at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CacheMode {
    #[default]
    Enabled,
    Disabled,
}

/// Resource limits enforced before decoding any object (plan §7.2, §7.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheLimits {
    pub max_uncompressed_object_bytes: u64,
    pub max_compressed_object_bytes: u64,
}

impl Default for CacheLimits {
    fn default() -> Self {
        Self {
            max_uncompressed_object_bytes: DEFAULT_MAX_UNCOMPRESSED_OBJECT_BYTES,
            max_compressed_object_bytes: DEFAULT_MAX_COMPRESSED_OBJECT_BYTES,
        }
    }
}

/// Configuration for a [`crate::cache::store::FileCacheStore`].
///
/// The default root comes from the platform cache directory, joined with `spec42/cache`, and
/// can be overridden explicitly (for embedders/tests) or via `SPEC42_CACHE_DIR`. Capacity
/// limits default to the plan's fixed 5 GiB/4 GiB policy; tests may inject a smaller budget
/// together with an isolated temporary root.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub mode: CacheMode,
    pub root: PathBuf,
    pub max_bytes: u64,
    pub prune_target_bytes: u64,
    pub limits: CacheLimits,
}

impl CacheConfig {
    /// Resolves the default configuration: enabled, platform cache root (or `SPEC42_CACHE_DIR`
    /// when set), and the plan's fixed capacity policy.
    ///
    /// Returns `None` if the platform provides no suitable cache directory and
    /// `SPEC42_CACHE_DIR` is unset; callers should fall back to [`CacheMode::Disabled`] in that
    /// case rather than guessing a location.
    pub fn discover_default() -> Option<Self> {
        let root = default_root()?;
        Some(Self::with_root(root))
    }

    /// Builds a configuration rooted at an explicit path (embedders, tests). Capacity limits
    /// use the plan's defaults; override the fields directly for isolated-budget tests.
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            mode: CacheMode::Enabled,
            root,
            max_bytes: DEFAULT_MAX_BYTES,
            prune_target_bytes: DEFAULT_PRUNE_TARGET_BYTES,
            limits: CacheLimits::default(),
        }
    }

    pub fn disabled(root: PathBuf) -> Self {
        Self {
            mode: CacheMode::Disabled,
            ..Self::with_root(root)
        }
    }

    pub fn objects_dir(&self) -> PathBuf {
        self.root.join("objects")
    }

    pub fn tmp_dir(&self) -> PathBuf {
        self.root.join("tmp")
    }
}

/// Resolves the default cache root: `SPEC42_CACHE_DIR` takes precedence, otherwise the
/// platform cache directory joined with `spec42/cache`.
fn default_root() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var(CACHE_DIR_ENV_VAR) {
        if !dir.is_empty() {
            return Some(PathBuf::from(dir));
        }
    }
    Some(dirs::cache_dir()?.join("spec42").join("cache"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_var_overrides_default_root() {
        // SAFETY: test-local env mutation; no other test in this process reads this var
        // concurrently with this specific check (cargo test runs each test in its own thread
        // but env vars are process-global, so this test only asserts the *logic*, not a global
        // guarantee about interleaving with other tests).
        let key = CACHE_DIR_ENV_VAR;
        let prior = std::env::var(key).ok();
        unsafe {
            std::env::set_var(key, "/tmp/spec42-cache-config-test");
        }
        let root = default_root().unwrap();
        assert_eq!(root, PathBuf::from("/tmp/spec42-cache-config-test"));
        unsafe {
            match prior {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }
    }
}
