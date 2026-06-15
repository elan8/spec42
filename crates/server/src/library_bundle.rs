//! Shared library bundle configuration and KPAR materialization helpers.

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use zip::read::ZipArchive;

pub const FORMAT_KPAR: &str = "kpar";
pub const STDLIB_KPAR_EMBED_PREFIX: &str = "bundled-sysml-kpar/";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LibraryBundleConfig {
    pub version: String,
    pub repo: String,
    #[serde(rename = "contentPath")]
    pub content_path: String,
    #[serde(default = "default_format_kpar")]
    pub format: String,
    #[serde(default)]
    pub artifact: Option<String>,
}

impl LibraryBundleConfig {
    pub fn is_kpar(&self) -> bool {
        self.format.eq_ignore_ascii_case(FORMAT_KPAR)
    }
}

fn default_format_kpar() -> String {
    FORMAT_KPAR.to_string()
}

pub fn normalize_content_path(path: &str) -> String {
    path.trim_matches('/').trim_matches('\\').to_string()
}

pub fn kpar_error(err: kpar::KparError) -> String {
    err.to_string()
}

pub fn materialize_kpar_bytes(
    archive_bytes: &[u8],
    destination_root: &Path,
) -> Result<kpar::MaterializedProject, String> {
    kpar::materialize(archive_bytes, destination_root).map_err(kpar_error)
}

pub fn is_kpar_bytes(bytes: &[u8]) -> bool {
    kpar::is_kpar_archive(bytes)
}

pub fn is_embedded_stdlib_kpar_bundle(bytes: &[u8]) -> bool {
    read_zip_entry_names(bytes)
        .map(|names| {
            names
                .iter()
                .any(|name| name.starts_with(STDLIB_KPAR_EMBED_PREFIX) && name.ends_with(".kpar"))
        })
        .unwrap_or(false)
}

/// Extract embedded `bundled-sysml-kpar/*.kpar` entries and materialize each under `destination_root`.
pub fn materialize_embedded_stdlib_kpar_bundle(
    archive_bytes: &[u8],
    destination_root: &Path,
) -> Result<Vec<PathBuf>, String> {
    fs::create_dir_all(destination_root).map_err(|e| e.to_string())?;
    let kpar_dir = destination_root.join("_kpar_archives");
    if kpar_dir.exists() {
        fs::remove_dir_all(&kpar_dir).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(&kpar_dir).map_err(|e| e.to_string())?;

    let cursor = Cursor::new(archive_bytes);
    let mut archive =
        ZipArchive::new(cursor).map_err(|e| format!("open embedded stdlib archive: {e}"))?;
    let mut extracted = 0usize;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|e| format!("read embedded stdlib entry {index}: {e}"))?;
        let name = entry.name().replace('\\', "/");
        if !name.starts_with(STDLIB_KPAR_EMBED_PREFIX) || name.ends_with('/') {
            continue;
        }
        let file_name = name
            .trim_start_matches(STDLIB_KPAR_EMBED_PREFIX)
            .trim_start_matches('/');
        if !file_name.ends_with(".kpar") {
            continue;
        }
        let dest = kpar_dir.join(file_name);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut file = fs::File::create(&dest).map_err(|e| e.to_string())?;
        std::io::copy(&mut entry, &mut file).map_err(|e| e.to_string())?;
        extracted += 1;
    }
    if extracted == 0 {
        return Err("embedded stdlib archive contains no KPAR files".to_string());
    }

    kpar::materialize_kpar_directory(&kpar_dir, destination_root).map_err(kpar_error)
}

fn read_zip_entry_names(bytes: &[u8]) -> Result<Vec<String>, String> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("open zip archive: {e}"))?;
    let mut names = Vec::new();
    for index in 0..archive.len() {
        let entry = archive
            .by_index(index)
            .map_err(|e| format!("read zip entry {index}: {e}"))?;
        if !entry.is_dir() {
            names.push(entry.name().replace('\\', "/"));
        }
    }
    Ok(names)
}

pub fn discover_library_roots(install_path: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if !install_path.is_dir() {
        return roots;
    }
    if let Ok(entries) = fs::read_dir(install_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && directory_contains_models(&path) {
                roots.push(path);
            }
        }
    }
    if roots.is_empty() && directory_contains_models(install_path) {
        roots.push(install_path.to_path_buf());
    }
    roots.sort();
    roots
}

fn directory_contains_models(path: &Path) -> bool {
    fn walk(path: &Path) -> bool {
        let Ok(entries) = fs::read_dir(path) else {
            return false;
        };
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file()
                && entry_path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext == "sysml" || ext == "kerml")
            {
                return true;
            }
            if entry_path.is_dir() && walk(&entry_path) {
                return true;
            }
        }
        false
    }
    walk(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "embed-stdlib")]
    #[test]
    fn materialize_embedded_stdlib_kpar_bundle_writes_scalar_values() {
        fn list_files(root: &Path) -> Vec<String> {
            fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
                let Ok(entries) = std::fs::read_dir(dir) else {
                    return;
                };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        out.push(path.strip_prefix(root).unwrap().display().to_string());
                    } else if path.is_dir() {
                        walk(&path, root, out);
                    }
                }
            }
            let mut out = Vec::new();
            walk(root, root, &mut out);
            out.sort();
            out
        }

        let archive = crate::stdlib::EMBEDDED_STDLIB_ARCHIVE;
        if archive.is_empty() {
            return;
        }
        let temp = tempfile::tempdir().expect("tempdir");
        let dest = temp.path().join("kpar");
        let roots =
            materialize_embedded_stdlib_kpar_bundle(archive, &dest).expect("materialize bundle");
        assert!(
            !roots.is_empty(),
            "expected materialized KPAR roots under {}",
            dest.display()
        );
        let scalar_values = dest
            .join("Kernel_Data_Type_Library-1.0.0")
            .join("ScalarValues.kerml");
        if !scalar_values.is_file() {
            panic!(
                "expected ScalarValues.kerml on disk, roots {:?}, files {:?}",
                roots,
                list_files(&dest)
            );
        }
        assert!(
            discover_library_roots(&dest).iter().any(|root| {
                root.file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("Kernel_Data_Type_Library"))
            }),
            "discovered roots {:?}",
            discover_library_roots(&dest)
        );
    }
}
