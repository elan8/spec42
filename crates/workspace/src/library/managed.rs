//! Config-driven managed KPAR libraries (domain, method, and future Elan8 bundles).
//!
//! Pins live in `config/libraries/*.json`. The build script embeds each archive and
//! generates `kpar_libraries_registry.rs` included below.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::library::bundle::{is_kpar_bytes, materialize_kpar_bytes, normalize_content_path};
use crate::library::stdlib::install_path_is_ready;

include!(concat!(env!("OUT_DIR"), "/kpar_libraries_registry.rs"));

pub const EMBEDDED_KPAR_LIBRARY_REPO: &str = "embedded";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KparLibraryConfig {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub repo: String,
    pub content_path: String,
    pub format: String,
    #[serde(default)]
    pub artifact: Option<String>,
}

impl KparLibraryConfig {
    pub fn is_kpar(&self) -> bool {
        self.format
            .eq_ignore_ascii_case(crate::library::bundle::FORMAT_KPAR)
    }

    pub fn from_embedded(entry: &EmbeddedKparLibrary) -> Self {
        Self {
            id: entry.id.to_string(),
            display_name: entry.display_name.to_string(),
            version: entry.version.to_string(),
            repo: entry.repo.to_string(),
            content_path: entry.content_path.to_string(),
            format: entry.format.to_string(),
            artifact: entry.artifact.map(str::to_string),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KparLibraryMetadata {
    pub installed_version: String,
    pub install_path: String,
    pub installed_at: String,
    pub repo: String,
    pub content_path: String,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub project_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KparLibraryStatus {
    pub id: String,
    pub display_name: String,
    pub pinned_version: String,
    pub installed_version: Option<String>,
    pub install_path: Option<String>,
    pub is_installed: bool,
    pub source: Option<String>,
    pub is_canonical_managed: bool,
    pub version_matches: bool,
    pub path_matches: bool,
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KparLibraryPaths {
    pub id: String,
    pub managed_root: PathBuf,
    pub metadata_path: PathBuf,
}

pub fn registry_configs() -> Vec<KparLibraryConfig> {
    EMBEDDED_KPAR_LIBRARIES
        .iter()
        .map(KparLibraryConfig::from_embedded)
        .collect()
}

pub fn embedded_entry(id: &str) -> Option<&'static EmbeddedKparLibrary> {
    EMBEDDED_KPAR_LIBRARIES.iter().find(|entry| entry.id == id)
}

pub fn embedded_archive(id: &str) -> Option<&'static [u8]> {
    embedded_entry(id).map(|entry| entry.archive)
}

pub fn kpar_library_paths_from_data_dir(data_dir: &Path, id: &str) -> KparLibraryPaths {
    let managed_root = data_dir.join("kpar-libraries").join(id);
    let metadata_path = managed_root.join("metadata.toml");
    KparLibraryPaths {
        id: id.to_string(),
        managed_root,
        metadata_path,
    }
}

pub fn managed_install_path(paths: &KparLibraryPaths, config: &KparLibraryConfig) -> PathBuf {
    let content = normalize_content_path(&config.content_path);
    let version_root = paths.managed_root.join("versions").join(&config.version);
    if content.is_empty() {
        version_root
    } else {
        version_root.join(content)
    }
}

pub fn load_managed_metadata(
    paths: &KparLibraryPaths,
) -> Result<Option<KparLibraryMetadata>, String> {
    if !paths.metadata_path.is_file() {
        return Ok(None);
    }
    let raw = fs::read_to_string(&paths.metadata_path)
        .map_err(|err| format!("Failed to read {}: {err}", paths.metadata_path.display()))?;
    toml::from_str(&raw)
        .map(Some)
        .map_err(|err| format!("Failed to parse {}: {err}", paths.metadata_path.display()))
}

pub fn save_managed_metadata(
    paths: &KparLibraryPaths,
    metadata: &KparLibraryMetadata,
) -> Result<(), String> {
    ensure_directory_path(&paths.managed_root, "Managed kpar-libraries root")?;
    let raw = toml::to_string(metadata)
        .map_err(|err| format!("Failed to serialize kpar library metadata: {err}"))?;
    let temp_path = paths.metadata_path.with_extension("toml.tmp");
    fs::write(&temp_path, raw)
        .map_err(|err| format!("Failed to write {}: {err}", temp_path.display()))?;
    fs::rename(&temp_path, &paths.metadata_path).map_err(|err| {
        format!(
            "Failed to move {} into place at {}: {err}",
            temp_path.display(),
            paths.metadata_path.display()
        )
    })
}

pub fn remove_managed_metadata(paths: &KparLibraryPaths) -> Result<(), String> {
    if paths.metadata_path.exists() {
        fs::remove_file(&paths.metadata_path)
            .map_err(|err| format!("Failed to remove {}: {err}", paths.metadata_path.display()))?;
    }
    Ok(())
}

pub fn managed_status(
    paths: &KparLibraryPaths,
    config: &KparLibraryConfig,
) -> Result<KparLibraryStatus, String> {
    let metadata = load_managed_metadata(paths)?;
    let expected_path = managed_install_path(paths, config);
    let version_matches = metadata
        .as_ref()
        .is_some_and(|metadata| metadata.installed_version == config.version);
    let path_matches = metadata.as_ref().is_some_and(|metadata| {
        canonicalize_lossy(Path::new(&metadata.install_path)) == canonicalize_lossy(&expected_path)
    });
    let path_ready = metadata
        .as_ref()
        .is_some_and(|metadata| install_path_is_ready(Path::new(&metadata.install_path)));
    let is_installed = version_matches && path_matches && path_ready;
    let status_message = metadata.as_ref().and_then(|metadata| {
        if !path_ready {
            Some(format!(
                "Managed {} path is not readable: {}",
                config.display_name, metadata.install_path
            ))
        } else if !version_matches {
            Some(format!(
                "Managed {} version {} is stale; pinned version is {}.",
                config.display_name, metadata.installed_version, config.version
            ))
        } else if !path_matches {
            Some(format!(
                "Managed {} path {} does not match pinned path {}.",
                config.display_name,
                metadata.install_path,
                expected_path.display()
            ))
        } else {
            None
        }
    });
    Ok(KparLibraryStatus {
        id: config.id.clone(),
        display_name: config.display_name.clone(),
        pinned_version: config.version.clone(),
        installed_version: metadata
            .as_ref()
            .map(|metadata| metadata.installed_version.clone()),
        install_path: metadata
            .as_ref()
            .map(|metadata| metadata.install_path.clone()),
        is_installed,
        source: metadata.as_ref().map(|m| {
            if m.repo == EMBEDDED_KPAR_LIBRARY_REPO {
                "bundled".to_string()
            } else {
                "managed".to_string()
            }
        }),
        is_canonical_managed: is_installed,
        version_matches,
        path_matches,
        status_message,
    })
}

pub fn install_embedded_kpar_library(
    paths: &KparLibraryPaths,
    config: &KparLibraryConfig,
) -> Result<KparLibraryMetadata, String> {
    let archive = embedded_archive(&config.id)
        .filter(|bytes| !bytes.is_empty())
        .ok_or_else(|| {
            format!(
                "This spec42 binary was built without embedded {} (id={}).",
                config.display_name, config.id
            )
        })?;
    let mut cfg = config.clone();
    cfg.repo = EMBEDDED_KPAR_LIBRARY_REPO.to_string();
    install_kpar_library_from_bytes(paths, &cfg, archive)
}

pub fn install_kpar_library_from_bytes(
    paths: &KparLibraryPaths,
    config: &KparLibraryConfig,
    archive_bytes: &[u8],
) -> Result<KparLibraryMetadata, String> {
    let normalized_content_path = normalize_content_path(&config.content_path);

    ensure_directory_path(&paths.managed_root, "Managed kpar-libraries root")?;

    let install_path = managed_install_path(paths, config);
    if install_path_is_ready(&install_path) {
        return metadata_for_ready_install(paths, config, &normalized_content_path, &install_path);
    }

    let _install_lock = acquire_install_lock(paths)?;
    if install_path_is_ready(&install_path) {
        return metadata_for_ready_install(paths, config, &normalized_content_path, &install_path);
    }

    let version_root = if normalized_content_path.is_empty() {
        install_path.clone()
    } else {
        install_path
            .parent()
            .ok_or_else(|| "Managed install root is malformed.".to_string())?
            .to_path_buf()
    };
    let managed_versions_root = paths.managed_root.join("versions");
    if !version_root.starts_with(&managed_versions_root) {
        return Err(format!(
            "Refusing to replace {} because it is outside {}.",
            version_root.display(),
            managed_versions_root.display()
        ));
    }
    if version_root.exists() && !version_root.is_dir() {
        return Err(format!(
            "Managed version path {} exists as a file; expected a directory.",
            version_root.display()
        ));
    }
    let staging_root =
        paths
            .managed_root
            .join(format!("staging-{}-{}", config.version, std::process::id()));
    if staging_root.exists() {
        fs::remove_dir_all(&staging_root)
            .map_err(|err| format!("Failed to clear {}: {err}", staging_root.display()))?;
    }
    let staging_version_root = staging_root.join(&config.version);
    let staging_install_path = if normalized_content_path.is_empty() {
        staging_version_root.clone()
    } else {
        staging_version_root.join(&normalized_content_path)
    };
    ensure_directory_path(&staging_install_path, "Managed kpar-libraries staging path")?;

    let project_name = if is_kpar_bytes(archive_bytes) {
        let materialized = materialize_kpar_bytes(archive_bytes, &staging_install_path)?;
        Some(materialized.project.name)
    } else {
        return Err(format!(
            "Expected a KPAR archive for {} installation.",
            config.display_name
        ));
    };
    if version_root.exists() {
        let remove_target = version_root.display().to_string();
        fs::remove_dir_all(&version_root).map_err(|err| {
            format!(
                "Failed to replace corrupt managed {} directory {}: {err}",
                config.display_name, remove_target
            )
        })?;
    }
    if let Some(parent) = version_root.parent() {
        ensure_directory_path(parent, "Managed kpar-libraries versions root")?;
    }
    let rename_target = version_root.display().to_string();
    fs::rename(&staging_version_root, &version_root).map_err(|err| {
        format!(
            "Failed replacing managed {} version directory {} with {}: {err}",
            config.display_name,
            staging_version_root.display(),
            rename_target
        )
    })?;
    if staging_root.exists() {
        let _ = fs::remove_dir_all(&staging_root);
    }
    if !install_path_is_ready(&install_path) {
        return Err(format!(
            "Managed {} install at {} is not readable after extraction.",
            config.display_name,
            install_path.display()
        ));
    }

    let installed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());
    let metadata = KparLibraryMetadata {
        installed_version: config.version.clone(),
        install_path: install_path.display().to_string(),
        installed_at,
        repo: config.repo.clone(),
        content_path: normalized_content_path,
        format: config.format.clone(),
        project_name,
    };
    save_managed_metadata(paths, &metadata)?;
    Ok(metadata)
}

pub fn remove_kpar_library(paths: &KparLibraryPaths) -> Result<bool, String> {
    let metadata = load_managed_metadata(paths)?;
    let Some(metadata) = metadata else {
        return Ok(false);
    };
    let install_path = PathBuf::from(&metadata.install_path);
    let managed_versions_root = paths.managed_root.join("versions");
    let version_root = if metadata.content_path.is_empty() {
        install_path.clone()
    } else {
        install_path
            .parent()
            .ok_or_else(|| "Managed install root is malformed.".to_string())?
            .to_path_buf()
    };
    if !version_root.starts_with(&managed_versions_root) {
        return Err(format!(
            "Refusing to remove {} because it is outside {}.",
            version_root.display(),
            managed_versions_root.display()
        ));
    }
    if version_root.exists() {
        let remove_target = version_root.display().to_string();
        fs::remove_dir_all(&version_root)
            .map_err(|err| format!("Failed to remove {}: {err}", remove_target))?;
    }
    remove_managed_metadata(paths)?;
    Ok(true)
}

const INSTALL_LOCK_FILE: &str = ".install.lock";
const INSTALL_LOCK_POLL_MS: u64 = 50;
const INSTALL_LOCK_TIMEOUT_MS: u64 = 120_000;

struct InstallLockGuard {
    lock_path: PathBuf,
    _file: fs::File,
}

impl Drop for InstallLockGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.lock_path);
    }
}

fn acquire_install_lock(paths: &KparLibraryPaths) -> Result<InstallLockGuard, String> {
    ensure_directory_path(&paths.managed_root, "Managed kpar-libraries root")?;
    let lock_path = paths.managed_root.join(INSTALL_LOCK_FILE);
    let deadline = Instant::now() + Duration::from_millis(INSTALL_LOCK_TIMEOUT_MS);
    loop {
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
        {
            Ok(mut file) => {
                let _ = writeln!(file, "{}", std::process::id());
                return Ok(InstallLockGuard {
                    lock_path,
                    _file: file,
                });
            }
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                if Instant::now() >= deadline {
                    return Err(format!(
                        "Timed out waiting for kpar-libraries install lock at {}",
                        lock_path.display()
                    ));
                }
                thread::sleep(Duration::from_millis(INSTALL_LOCK_POLL_MS));
            }
            Err(err) => {
                return Err(format!(
                    "Failed to acquire kpar-libraries install lock at {}: {err}",
                    lock_path.display()
                ));
            }
        }
    }
}

fn metadata_for_ready_install(
    paths: &KparLibraryPaths,
    config: &KparLibraryConfig,
    normalized_content_path: &str,
    install_path: &Path,
) -> Result<KparLibraryMetadata, String> {
    let installed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());
    let metadata = KparLibraryMetadata {
        installed_version: config.version.clone(),
        install_path: install_path.display().to_string(),
        installed_at,
        repo: config.repo.clone(),
        content_path: normalized_content_path.to_string(),
        format: config.format.clone(),
        project_name: None,
    };
    save_managed_metadata(paths, &metadata)?;
    Ok(metadata)
}

fn ensure_directory_path(path: &Path, role: &str) -> Result<(), String> {
    if path.exists() && !path.is_dir() {
        return Err(format!(
            "{role} path {} exists as a file; expected a directory.",
            path.display()
        ));
    }
    fs::create_dir_all(path).map_err(|err| format!("Failed to create {}: {err}", path.display()))
}

fn canonicalize_lossy(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn managed_install_path_uses_content_subdirectory() {
        let paths = kpar_library_paths_from_data_dir(Path::new("/tmp/spec42-data"), "domain");
        let config = KparLibraryConfig {
            id: "domain".to_string(),
            display_name: "Domain libraries".to_string(),
            version: "0.3.0".to_string(),
            repo: "elan8/sysml-domain-libraries".to_string(),
            content_path: String::new(),
            format: "kpar".to_string(),
            artifact: Some("elan8-domain-libraries-0.3.0.kpar".to_string()),
        };
        let install = managed_install_path(&paths, &config);
        assert!(
            install.ends_with("kpar-libraries/domain/versions/0.3.0")
                || install.ends_with(r"kpar-libraries\domain\versions\0.3.0")
        );
    }
}
