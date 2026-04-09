use std::fs;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub const DEFAULT_STDLIB_VERSION: &str = "2026-02";
pub const DEFAULT_STDLIB_REPO: &str = "Systems-Modeling/SysML-v2-Release";
pub const DEFAULT_STDLIB_CONTENT_PATH: &str = "sysml.library";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardLibraryConfig {
    pub version: String,
    pub repo: String,
    pub content_path: String,
}

impl Default for StandardLibraryConfig {
    fn default() -> Self {
        Self {
            version: DEFAULT_STDLIB_VERSION.to_string(),
            repo: DEFAULT_STDLIB_REPO.to_string(),
            content_path: DEFAULT_STDLIB_CONTENT_PATH.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardLibraryMetadata {
    pub installed_version: String,
    pub install_path: String,
    pub installed_at: String,
    pub repo: String,
    pub content_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StandardLibraryStatus {
    pub pinned_version: String,
    pub installed_version: Option<String>,
    pub install_path: Option<String>,
    pub is_installed: bool,
    pub source: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StandardLibraryPaths {
    pub managed_root: PathBuf,
    pub metadata_path: PathBuf,
}

pub fn project_dirs() -> Result<ProjectDirs, String> {
    ProjectDirs::from("io", "Elan8", "spec42")
        .ok_or_else(|| "Could not determine a user config directory for spec42.".to_string())
}

pub fn standard_library_paths() -> Result<StandardLibraryPaths, String> {
    let project_dirs = project_dirs()?;
    let data_dir = project_dirs.data_local_dir().to_path_buf();
    let managed_root = data_dir.join("standard-library");
    let metadata_path = managed_root.join("metadata.toml");
    Ok(StandardLibraryPaths {
        managed_root,
        metadata_path,
    })
}

pub fn managed_install_path(
    paths: &StandardLibraryPaths,
    config: &StandardLibraryConfig,
) -> PathBuf {
    paths
        .managed_root
        .join("versions")
        .join(&config.version)
        .join(normalize_content_path(&config.content_path))
}

pub fn load_managed_metadata(
    paths: &StandardLibraryPaths,
) -> Result<Option<StandardLibraryMetadata>, String> {
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
    paths: &StandardLibraryPaths,
    metadata: &StandardLibraryMetadata,
) -> Result<(), String> {
    fs::create_dir_all(&paths.managed_root)
        .map_err(|err| format!("Failed to create {}: {err}", paths.managed_root.display()))?;
    let raw = toml::to_string(metadata)
        .map_err(|err| format!("Failed to serialize standard library metadata: {err}"))?;
    fs::write(&paths.metadata_path, raw)
        .map_err(|err| format!("Failed to write {}: {err}", paths.metadata_path.display()))
}

pub fn remove_managed_metadata(paths: &StandardLibraryPaths) -> Result<(), String> {
    if paths.metadata_path.exists() {
        fs::remove_file(&paths.metadata_path)
            .map_err(|err| format!("Failed to remove {}: {err}", paths.metadata_path.display()))?;
    }
    Ok(())
}

pub fn managed_status(
    paths: &StandardLibraryPaths,
    config: &StandardLibraryConfig,
) -> Result<StandardLibraryStatus, String> {
    let metadata = load_managed_metadata(paths)?;
    let is_installed = metadata
        .as_ref()
        .is_some_and(|metadata| Path::new(&metadata.install_path).is_dir());
    Ok(StandardLibraryStatus {
        pinned_version: config.version.clone(),
        installed_version: metadata
            .as_ref()
            .map(|metadata| metadata.installed_version.clone()),
        install_path: metadata
            .as_ref()
            .map(|metadata| metadata.install_path.clone()),
        is_installed,
        source: metadata.map(|_| "managed".to_string()),
    })
}

pub fn install_standard_library(
    paths: &StandardLibraryPaths,
    config: &StandardLibraryConfig,
) -> Result<StandardLibraryMetadata, String> {
    let normalized_content_path = normalize_content_path(&config.content_path);
    if normalized_content_path.is_empty() {
        return Err("Standard library content path must not be empty.".to_string());
    }

    fs::create_dir_all(&paths.managed_root)
        .map_err(|err| format!("Failed to create {}: {err}", paths.managed_root.display()))?;

    let url = format!(
        "https://codeload.github.com/{}/zip/refs/tags/{}",
        config.repo, config.version
    );
    let bytes = download_archive(&url)?;
    let install_path = managed_install_path(paths, config);
    let version_root = install_path
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Managed install root is malformed.".to_string())?;
    if version_root.exists() {
        fs::remove_dir_all(version_root)
            .map_err(|err| format!("Failed to replace {}: {err}", version_root.display()))?;
    }
    fs::create_dir_all(&install_path)
        .map_err(|err| format!("Failed to create {}: {err}", install_path.display()))?;
    extract_archive_subset(&bytes, &normalized_content_path, &install_path)?;

    let installed_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());
    let metadata = StandardLibraryMetadata {
        installed_version: config.version.clone(),
        install_path: install_path.display().to_string(),
        installed_at,
        repo: config.repo.clone(),
        content_path: normalized_content_path,
    };
    save_managed_metadata(paths, &metadata)?;
    Ok(metadata)
}

pub fn remove_standard_library(paths: &StandardLibraryPaths) -> Result<bool, String> {
    let metadata = load_managed_metadata(paths)?;
    let Some(metadata) = metadata else {
        return Ok(false);
    };
    let install_path = PathBuf::from(&metadata.install_path);
    let managed_versions_root = paths.managed_root.join("versions");
    let version_root = install_path
        .parent()
        .and_then(Path::parent)
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

pub fn legacy_vscode_stdlib_path(config: &StandardLibraryConfig) -> Option<PathBuf> {
    let base = legacy_vscode_base_dir()?;
    let exact = base
        .join(&config.version)
        .join(normalize_content_path(&config.content_path));
    if exact.is_dir() {
        return Some(exact);
    }
    let mut discovered = fs::read_dir(&base)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| {
            entry
                .path()
                .join(normalize_content_path(&config.content_path))
        })
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    discovered.sort();
    discovered.pop()
}

fn legacy_vscode_base_dir() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(app_data) = std::env::var_os("APPDATA") {
        candidates.push(PathBuf::from(app_data));
    }
    if let Some(user_profile) = std::env::var_os("USERPROFILE") {
        candidates.push(PathBuf::from(user_profile).join("AppData").join("Roaming"));
    }
    candidates
        .into_iter()
        .map(|root| {
            root.join("Code")
                .join("User")
                .join("globalStorage")
                .join("elan8.spec42")
                .join("standard-library")
        })
        .find(|path| path.is_dir())
}

fn normalize_content_path(path: &str) -> String {
    path.trim_matches('/').trim_matches('\\').to_string()
}

fn download_archive(url: &str) -> Result<Vec<u8>, String> {
    let response = ureq::get(url)
        .set("User-Agent", "spec42-cli")
        .call()
        .map_err(|err| format!("Failed to download standard library archive from {url}: {err}"))?;
    let mut reader = response.into_reader();
    let mut out = Vec::new();
    reader
        .read_to_end(&mut out)
        .map_err(|err| format!("Failed to read standard library archive: {err}"))?;
    Ok(out)
}

fn extract_archive_subset(
    archive_bytes: &[u8],
    content_path: &str,
    destination_root: &Path,
) -> Result<(), String> {
    let cursor = Cursor::new(archive_bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|err| format!("Failed to open standard library archive: {err}"))?;
    if archive.is_empty() {
        return Err("Downloaded standard library archive is empty.".to_string());
    }

    let root_prefix = {
        let first = archive
            .by_index(0)
            .map_err(|err| format!("Failed to inspect archive: {err}"))?;
        first
            .name()
            .split('/')
            .next()
            .ok_or_else(|| "Downloaded standard library archive is malformed.".to_string())?
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
            "Path '{content_path}' was not found in the downloaded standard library archive."
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn legacy_vscode_path_is_computed_from_appdata() {
        let temp = tempfile::tempdir().expect("temp dir");
        let base = temp.path().join("Roaming");
        fs::create_dir_all(
            base.join("Code")
                .join("User")
                .join("globalStorage")
                .join("elan8.spec42")
                .join("standard-library")
                .join(DEFAULT_STDLIB_VERSION)
                .join(DEFAULT_STDLIB_CONTENT_PATH),
        )
        .expect("create vscode path");
        std::env::set_var("APPDATA", &base);
        let resolved = legacy_vscode_stdlib_path(&StandardLibraryConfig::default());
        assert!(resolved.is_some());
    }

    #[test]
    fn extract_archive_subset_only_extracts_requested_content_path() {
        let temp = tempfile::tempdir().expect("temp dir");
        let archive_path = temp.path().join("archive.zip");
        {
            let file = fs::File::create(&archive_path).expect("create zip");
            let mut zip = zip::ZipWriter::new(file);
            let options = zip::write::SimpleFileOptions::default();
            zip.start_file("release-2026-02/sysml.library/A.sysml", options)
                .expect("start file");
            zip.write_all(b"package A {}").expect("write file");
            zip.start_file("release-2026-02/other/B.sysml", options)
                .expect("start other");
            zip.write_all(b"package B {}").expect("write other");
            zip.finish().expect("finish zip");
        }

        let bytes = fs::read(&archive_path).expect("read archive");
        let destination = temp.path().join("extract");
        extract_archive_subset(&bytes, "sysml.library", &destination).expect("extract subset");

        assert!(destination.join("A.sysml").is_file());
        assert!(!destination.join("B.sysml").exists());
    }
}
