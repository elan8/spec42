use std::fs;
use std::io::{self, Cursor, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::stdlib::install_path_is_ready;

pub const DEFAULT_DOMAIN_LIBRARIES_VERSION: &str = env!("SPEC42_DOMAIN_LIBRARIES_VERSION");
pub const DEFAULT_DOMAIN_LIBRARIES_REPO: &str = env!("SPEC42_DOMAIN_LIBRARIES_REPO");
pub const DEFAULT_DOMAIN_LIBRARIES_CONTENT_PATH: &str =
    env!("SPEC42_DOMAIN_LIBRARIES_CONTENT_PATH");
pub const EMBEDDED_DOMAIN_LIBRARIES_REPO: &str = "embedded";

#[cfg(feature = "embed-domain-libraries")]
pub const EMBEDDED_DOMAIN_LIBRARIES_ARCHIVE: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/domain-libraries.embedded.zip"));

#[cfg(not(feature = "embed-domain-libraries"))]
pub const EMBEDDED_DOMAIN_LIBRARIES_ARCHIVE: &[u8] = &[];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainLibrariesConfig {
    pub version: String,
    pub repo: String,
    pub content_path: String,
}

impl Default for DomainLibrariesConfig {
    fn default() -> Self {
        Self {
            version: DEFAULT_DOMAIN_LIBRARIES_VERSION.to_string(),
            repo: DEFAULT_DOMAIN_LIBRARIES_REPO.to_string(),
            content_path: DEFAULT_DOMAIN_LIBRARIES_CONTENT_PATH.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainLibrariesMetadata {
    pub installed_version: String,
    pub install_path: String,
    pub installed_at: String,
    pub repo: String,
    pub content_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DomainLibrariesStatus {
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

#[derive(Debug, Clone)]
pub struct DomainLibrariesPaths {
    pub managed_root: PathBuf,
    pub metadata_path: PathBuf,
}

pub fn domain_libraries_paths_from_data_dir(data_dir: PathBuf) -> DomainLibrariesPaths {
    let managed_root = data_dir.join("domain-libraries");
    let metadata_path = managed_root.join("metadata.toml");
    DomainLibrariesPaths {
        managed_root,
        metadata_path,
    }
}

pub fn managed_install_path(
    paths: &DomainLibrariesPaths,
    config: &DomainLibrariesConfig,
) -> PathBuf {
    paths
        .managed_root
        .join("versions")
        .join(&config.version)
        .join(normalize_content_path(&config.content_path))
}

pub fn load_managed_metadata(
    paths: &DomainLibrariesPaths,
) -> Result<Option<DomainLibrariesMetadata>, String> {
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
    paths: &DomainLibrariesPaths,
    metadata: &DomainLibrariesMetadata,
) -> Result<(), String> {
    ensure_directory_path(&paths.managed_root, "Managed domain-libraries root")?;
    let raw = toml::to_string(metadata)
        .map_err(|err| format!("Failed to serialize domain libraries metadata: {err}"))?;
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

pub fn remove_managed_metadata(paths: &DomainLibrariesPaths) -> Result<(), String> {
    if paths.metadata_path.exists() {
        fs::remove_file(&paths.metadata_path)
            .map_err(|err| format!("Failed to remove {}: {err}", paths.metadata_path.display()))?;
    }
    Ok(())
}

pub fn managed_status(
    paths: &DomainLibrariesPaths,
    config: &DomainLibrariesConfig,
) -> Result<DomainLibrariesStatus, String> {
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
                "Managed domain libraries path is not readable: {}",
                metadata.install_path
            ))
        } else if !version_matches {
            Some(format!(
                "Managed domain libraries version {} is stale; pinned version is {}.",
                metadata.installed_version, config.version
            ))
        } else if !path_matches {
            Some(format!(
                "Managed domain libraries path {} does not match pinned path {}.",
                metadata.install_path,
                expected_path.display()
            ))
        } else {
            None
        }
    });
    Ok(DomainLibrariesStatus {
        pinned_version: config.version.clone(),
        installed_version: metadata
            .as_ref()
            .map(|metadata| metadata.installed_version.clone()),
        install_path: metadata
            .as_ref()
            .map(|metadata| metadata.install_path.clone()),
        is_installed,
        source: metadata.as_ref().map(|m| {
            if m.repo == EMBEDDED_DOMAIN_LIBRARIES_REPO {
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

pub fn install_embedded_domain_libraries(
    paths: &DomainLibrariesPaths,
    config: &DomainLibrariesConfig,
) -> Result<DomainLibrariesMetadata, String> {
    #[allow(clippy::const_is_empty)]
    if EMBEDDED_DOMAIN_LIBRARIES_ARCHIVE.is_empty() {
        return Err("This spec42 binary was built without embedded domain libraries.".to_string());
    }
    let mut cfg = config.clone();
    cfg.repo = EMBEDDED_DOMAIN_LIBRARIES_REPO.to_string();
    install_domain_libraries_from_bytes(paths, &cfg, EMBEDDED_DOMAIN_LIBRARIES_ARCHIVE)
}

pub fn install_domain_libraries_from_bytes(
    paths: &DomainLibrariesPaths,
    config: &DomainLibrariesConfig,
    archive_bytes: &[u8],
) -> Result<DomainLibrariesMetadata, String> {
    let normalized_content_path = normalize_content_path(&config.content_path);
    if normalized_content_path.is_empty() {
        return Err("Domain libraries content path must not be empty.".to_string());
    }

    ensure_directory_path(&paths.managed_root, "Managed domain-libraries root")?;

    let install_path = managed_install_path(paths, config);
    if install_path_is_ready(&install_path) {
        return metadata_for_ready_install(paths, config, &normalized_content_path, &install_path);
    }

    let _install_lock = acquire_install_lock(paths)?;
    if install_path_is_ready(&install_path) {
        return metadata_for_ready_install(paths, config, &normalized_content_path, &install_path);
    }

    let version_root = install_path
        .parent()
        .ok_or_else(|| "Managed install root is malformed.".to_string())?;
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
    let staging_install_path = staging_version_root.join(&normalized_content_path);
    ensure_directory_path(
        &staging_install_path,
        "Managed domain-libraries staging path",
    )?;
    extract_archive_subset(
        archive_bytes,
        &normalized_content_path,
        &staging_install_path,
    )?;
    if version_root.exists() {
        fs::remove_dir_all(version_root).map_err(|err| {
            format!(
                "Failed to replace corrupt managed domain libraries directory {}: {err}",
                version_root.display()
            )
        })?;
    }
    if let Some(parent) = version_root.parent() {
        ensure_directory_path(parent, "Managed domain-libraries versions root")?;
    }
    fs::rename(&staging_version_root, version_root).map_err(|err| {
        format!(
            "Failed replacing managed domain libraries version directory {} with {}: {err}",
            staging_version_root.display(),
            version_root.display()
        )
    })?;
    if staging_root.exists() {
        let _ = fs::remove_dir_all(&staging_root);
    }
    if !install_path_is_ready(&install_path) {
        return Err(format!(
            "Managed domain libraries install at {} is not readable after extraction.",
            install_path.display()
        ));
    }

    metadata_for_ready_install(paths, config, &normalized_content_path, &install_path)
}

pub fn remove_domain_libraries(paths: &DomainLibrariesPaths) -> Result<bool, String> {
    let metadata = load_managed_metadata(paths)?;
    let Some(metadata) = metadata else {
        return Ok(false);
    };
    let install_path = PathBuf::from(&metadata.install_path);
    let managed_versions_root = paths.managed_root.join("versions");
    let version_root = install_path
        .parent()
        .ok_or_else(|| "Managed install root is malformed.".to_string())?;
    if !version_root.starts_with(&managed_versions_root) {
        return Err(format!(
            "Refusing to remove {} because it is outside {}.",
            version_root.display(),
            managed_versions_root.display()
        ));
    }
    if version_root.exists() {
        fs::remove_dir_all(version_root)
            .map_err(|err| format!("Failed to remove {}: {err}", version_root.display()))?;
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

fn acquire_install_lock(paths: &DomainLibrariesPaths) -> Result<InstallLockGuard, String> {
    ensure_directory_path(&paths.managed_root, "Managed domain-libraries root")?;
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
                        "Timed out waiting for domain-libraries install lock at {}",
                        lock_path.display()
                    ));
                }
                thread::sleep(Duration::from_millis(INSTALL_LOCK_POLL_MS));
            }
            Err(err) => {
                return Err(format!(
                    "Failed to acquire domain-libraries install lock at {}: {err}",
                    lock_path.display()
                ));
            }
        }
    }
}

fn metadata_for_ready_install(
    paths: &DomainLibrariesPaths,
    config: &DomainLibrariesConfig,
    normalized_content_path: &str,
    install_path: &Path,
) -> Result<DomainLibrariesMetadata, String> {
    let installed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());
    let metadata = DomainLibrariesMetadata {
        installed_version: config.version.clone(),
        install_path: install_path.display().to_string(),
        installed_at,
        repo: config.repo.clone(),
        content_path: normalized_content_path.to_string(),
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

fn normalize_content_path(path: &str) -> String {
    path.trim_matches('/').trim_matches('\\').to_string()
}

fn canonicalize_lossy(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn extract_archive_subset(
    archive_bytes: &[u8],
    content_path: &str,
    destination_root: &Path,
) -> Result<(), String> {
    let cursor = Cursor::new(archive_bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|err| format!("Failed to open domain libraries archive: {err}"))?;
    if archive.is_empty() {
        return Err("Domain libraries archive is empty.".to_string());
    }

    let root_prefix = {
        let first = archive
            .by_index(0)
            .map_err(|err| format!("Failed to inspect archive: {err}"))?;
        first
            .name()
            .split('/')
            .next()
            .ok_or_else(|| "Domain libraries archive is malformed.".to_string())?
            .to_string()
    };
    let wanted_prefix = format!("{root_prefix}/{content_path}/");
    let mut extracted_any = false;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|err| format!("Failed to read archive entry: {err}"))?;
        let name = entry.name().to_string();
        if !name.starts_with(&wanted_prefix) || name.ends_with('/') {
            continue;
        }
        let relative = name.trim_start_matches(&wanted_prefix);
        let destination = destination_root.join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("Failed to create {}: {err}", parent.display()))?;
        }
        let mut file = fs::File::create(&destination)
            .map_err(|err| format!("Failed to create {}: {err}", destination.display()))?;
        std::io::copy(&mut entry, &mut file)
            .map_err(|err| format!("Failed to extract {}: {err}", destination.display()))?;
        extracted_any = true;
    }
    if !extracted_any {
        return Err(format!(
            "Path '{content_path}' was not found in the domain libraries archive."
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn managed_install_path_uses_content_subdirectory() {
        let paths = domain_libraries_paths_from_data_dir(PathBuf::from("/tmp/spec42-data"));
        let config = DomainLibrariesConfig {
            version: "dc378a9".to_string(),
            repo: "elan8/sysml-domain-libraries".to_string(),
            content_path: "tree".to_string(),
        };
        let install = managed_install_path(&paths, &config);
        assert!(install.ends_with("versions/dc378a9/tree"));
    }
}
