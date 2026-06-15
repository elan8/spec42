use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use zip::read::ZipArchive;

use crate::error::{KparError, Result};
use crate::schema::{Meta, Project, META_FILE, PROJECT_FILE, SOURCE_EXTENSIONS};

/// Parsed KPAR archive (project + meta + raw bytes).
#[derive(Debug, Clone)]
pub struct KparArchive {
    pub project: Project,
    pub meta: Meta,
    bytes: Vec<u8>,
}

/// Result of materializing a KPAR to disk.
#[derive(Debug, Clone)]
pub struct MaterializedProject {
    pub project: Project,
    pub meta: Meta,
    pub root: PathBuf,
    pub source_files: Vec<PathBuf>,
}

/// Returns true when `bytes` is a zip containing `.project.json` at the archive root.
pub fn is_kpar_archive(bytes: &[u8]) -> bool {
    open_kpar_bytes(bytes).is_ok()
}

pub fn open_kpar_bytes(bytes: &[u8]) -> Result<KparArchive> {
    let (project, meta) = parse_manifests(bytes)?;
    Ok(KparArchive {
        project,
        meta,
        bytes: bytes.to_vec(),
    })
}

pub fn open_kpar_path(path: &Path) -> Result<KparArchive> {
    let bytes = fs::read(path).map_err(|source| KparError::Io {
        path: path.display().to_string(),
        source,
    })?;
    open_kpar_bytes(&bytes)
}

pub fn verify_checksums(bytes: &[u8]) -> Result<()> {
    let archive = open_kpar_bytes(bytes)?;
    archive.verify_checksums()
}

pub fn materialize(bytes: &[u8], destination_root: &Path) -> Result<MaterializedProject> {
    let archive = open_kpar_bytes(bytes)?;
    archive.verify_checksums()?;
    archive.materialize_to(destination_root)
}

/// Materialize every `.kpar` file in `directory` into subdirectories named after the file stem.
pub fn materialize_kpar_directory(directory: &Path, destination_root: &Path) -> Result<Vec<PathBuf>> {
    let mut roots = Vec::new();
    let entries = fs::read_dir(directory).map_err(|source| KparError::Io {
        path: directory.display().to_string(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| KparError::Io {
            path: directory.display().to_string(),
            source,
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
            continue;
        };
        if ext != "kpar" {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("library");
        let dest = destination_root.join(stem);
        if dest.exists() {
            fs::remove_dir_all(&dest).map_err(|source| KparError::Io {
                path: dest.display().to_string(),
                source,
            })?;
        }
        materialize(
            &fs::read(&path).map_err(|source| KparError::Io {
                path: path.display().to_string(),
                source,
            })?,
            &dest,
        )?;
        roots.push(dest);
    }
    if roots.is_empty() {
        return Err(KparError::InvalidArchive(format!(
            "no .kpar files found in {}",
            directory.display()
        )));
    }
    roots.sort();
    Ok(roots)
}

impl KparArchive {
    pub fn project(&self) -> &Project {
        &self.project
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn verify_checksums(&self) -> Result<()> {
        if self.meta.checksum.is_empty() {
            return Ok(());
        }
        let entries = read_zip_entries(&self.bytes)?;
        for (logical_path, entry) in &self.meta.checksum {
            let archive_path = self
                .meta
                .index
                .get(logical_path)
                .cloned()
                .unwrap_or_else(|| logical_path.clone());
            let Some(bytes) = entries.get(&normalize_zip_path(&archive_path)) else {
                return Err(KparError::InvalidArchive(format!(
                    "indexed path '{logical_path}' not found in archive"
                )));
            };
            let actual = sha256_hex(bytes);
            if actual != entry.value {
                return Err(KparError::ChecksumMismatch {
                    path: logical_path.clone(),
                    expected: entry.value.clone(),
                    actual,
                });
            }
        }
        Ok(())
    }

    pub fn materialize_to(&self, destination_root: &Path) -> Result<MaterializedProject> {
        fs::create_dir_all(destination_root).map_err(|source| KparError::Io {
            path: destination_root.display().to_string(),
            source,
        })?;

        let entries = read_zip_entries(&self.bytes)?;
        let mut source_files = Vec::new();

        let paths: Vec<String> = if self.meta.index.is_empty() {
            entries
                .keys()
                .filter(|p| is_source_path(p))
                .cloned()
                .collect()
        } else {
            self.meta.index.keys().cloned().collect()
        };

        for logical_path in paths {
            let archive_path = self
                .meta
                .index
                .get(&logical_path)
                .cloned()
                .unwrap_or_else(|| logical_path.clone());
            let normalized = normalize_zip_path(&archive_path);
            let Some(bytes) = entries.get(&normalized) else {
                return Err(KparError::InvalidArchive(format!(
                    "missing archive entry '{normalized}' for '{logical_path}'"
                )));
            };
            if !is_source_path(&logical_path) {
                continue;
            }
            let dest = destination_root.join(&logical_path);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent).map_err(|source| KparError::Io {
                    path: parent.display().to_string(),
                    source,
                })?;
            }
            fs::write(&dest, bytes).map_err(|source| KparError::Io {
                path: dest.display().to_string(),
                source,
            })?;
            source_files.push(dest);
        }

        source_files.sort();

        Ok(MaterializedProject {
            project: self.project.clone(),
            meta: self.meta.clone(),
            root: destination_root.to_path_buf(),
            source_files,
        })
    }
}

fn parse_manifests(bytes: &[u8]) -> Result<(Project, Meta)> {
    let entries = read_zip_entries(bytes)?;
    let project_bytes = entries
        .get(PROJECT_FILE)
        .ok_or(KparError::MissingFile(PROJECT_FILE))?;
    let meta_bytes = entries
        .get(META_FILE)
        .ok_or(KparError::MissingFile(META_FILE))?;
    let project: Project = serde_json::from_slice(project_bytes)?;
    let meta: Meta = serde_json::from_slice(meta_bytes)?;
    Ok((project, meta))
}

fn read_zip_entries(bytes: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
    let cursor = Cursor::new(bytes);
    let mut archive =
        ZipArchive::new(cursor).map_err(|e| KparError::Zip(format!("open archive: {e}")))?;
    let mut entries = HashMap::new();
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|e| KparError::Zip(format!("read entry {index}: {e}")))?;
        if entry.is_dir() {
            continue;
        }
        let name = normalize_zip_path(entry.name());
        let mut buf = Vec::new();
        entry
            .read_to_end(&mut buf)
            .map_err(|e| KparError::Zip(format!("read {name}: {e}")))?;
        entries.insert(name, buf);
    }
    Ok(entries)
}

fn normalize_zip_path(path: &str) -> String {
    path.trim_start_matches("./")
        .trim_start_matches('/')
        .replace('\\', "/")
}

fn is_source_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    SOURCE_EXTENSIONS
        .iter()
        .any(|ext| lower.ends_with(ext))
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pack::{build_kpar, PackOptions};
    use tempfile::tempdir;

    #[test]
    fn roundtrip_pack_and_materialize() {
        let source = tempdir().expect("tempdir");
        let model = source.path().join("domain/example.sysml");
        fs::create_dir_all(model.parent().unwrap()).expect("mkdir");
        fs::write(&model, "package Example {}").expect("write");

        let kpar_path = source.path().join("test.kpar");
        build_kpar(
            &PackOptions {
                project: Project {
                    name: "test-lib".to_string(),
                    version: "0.1.0".to_string(),
                    description: None,
                    license: None,
                    publisher: Some("elan8".to_string()),
                    maintainer: vec![],
                    website: None,
                    topic: vec![],
                    usage: vec![],
                },
                source_roots: vec![source.path().join("domain")],
                excludes: vec![],
            },
            &kpar_path,
        )
        .expect("pack");

        let bytes = fs::read(&kpar_path).expect("read kpar");
        assert!(is_kpar_archive(&bytes));
        verify_checksums(&bytes).expect("checksums");

        let dest = source.path().join("out");
        let materialized = materialize(&bytes, &dest).expect("materialize");
        assert_eq!(materialized.project.name, "test-lib");
        assert!(dest.join("domain/example.sysml").is_file());
    }

    #[test]
    fn missing_project_json_is_not_kpar() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("bad.zip");
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file("foo.sysml", options).expect("start");
            zip.write_all(b"package Foo {}").expect("write");
            zip.finish().expect("finish");
        }
        let bytes = fs::read(&path).expect("read");
        assert!(!is_kpar_archive(&bytes));
    }
}
