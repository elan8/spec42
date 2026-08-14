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

#[derive(Debug, Clone)]
struct MaterializationPlan {
    source_files: Vec<(String, Vec<u8>)>,
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
    archive.materialize_to(destination_root)
}

/// Materialize every `.kpar` file in `directory` into subdirectories named after the file stem.
pub fn materialize_kpar_directory(
    directory: &Path,
    destination_root: &Path,
) -> Result<Vec<PathBuf>> {
    let mut plans = Vec::new();
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
        let archive = open_kpar_path(&path)?;
        plans.push((dest, archive.materialization_plan()?));
    }
    if plans.is_empty() {
        return Err(KparError::InvalidArchive(format!(
            "no .kpar files found in {}",
            directory.display()
        )));
    }
    plans.sort_by(|left, right| left.0.cmp(&right.0));
    if let Some(duplicate) = plans
        .windows(2)
        .find_map(|pair| (pair[0].0 == pair[1].0).then(|| pair[0].0.display().to_string()))
    {
        return Err(KparError::InvalidArchive(format!(
            "multiple archives have destination '{duplicate}'"
        )));
    }
    for (destination, _) in &plans {
        ensure_absent_publish_target(destination)?;
    }

    let mut roots = Vec::with_capacity(plans.len());
    for (dest, plan) in plans {
        publish_materialization(&dest, &plan)?;
        roots.push(dest);
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
            if !entry.algorithm.eq_ignore_ascii_case("SHA256") {
                return Err(KparError::InvalidArchive(format!(
                    "unsupported checksum algorithm '{}' for '{logical_path}'; expected SHA256",
                    entry.algorithm
                )));
            }
            let archive_path = self
                .meta
                .index
                .get(logical_path)
                .cloned()
                .unwrap_or_else(|| logical_path.clone());
            let archive_path = normalize_zip_path(&archive_path)?;
            let Some(bytes) = entries.get(&archive_path) else {
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

    /// Validate and stage this archive before publishing it to a fresh directory.
    ///
    /// Existing targets are refused. The portable standard library has no
    /// atomic no-clobber directory rename, so a competing creator between the
    /// final absence check and rename can still cause the platform rename to
    /// fail or replace that target; callers that need concurrency-safe
    /// publication must provide owner-scoped destination coordination.
    pub fn materialize_to(&self, destination_root: &Path) -> Result<MaterializedProject> {
        let plan = self.materialization_plan()?;
        publish_materialization(destination_root, &plan)?;
        let mut source_files = plan
            .source_files
            .iter()
            .map(|(logical_path, _)| destination_root.join(logical_path))
            .collect::<Vec<_>>();
        source_files.sort();

        Ok(MaterializedProject {
            project: self.project.clone(),
            meta: self.meta.clone(),
            root: destination_root.to_path_buf(),
            source_files,
        })
    }

    fn materialization_plan(&self) -> Result<MaterializationPlan> {
        self.verify_checksums()?;
        let entries = read_zip_entries(&self.bytes)?;

        let mut paths: Vec<(String, String)> = if self.meta.index.is_empty() {
            entries
                .keys()
                .filter(|p| is_source_path(p))
                .map(|path| (path.clone(), path.clone()))
                .collect()
        } else {
            self.meta
                .index
                .iter()
                .filter_map(|(logical_path, archive_path)| {
                    if is_source_path(logical_path) {
                        Some((logical_path.clone(), archive_path.clone()))
                    } else if is_source_path(archive_path) {
                        Some((archive_path.clone(), archive_path.clone()))
                    } else {
                        None
                    }
                })
                .collect()
        };
        paths.sort();

        // Validate every destination before creating the output directory. KPAR
        // indexes are archive data, so they must never be allowed to escape the
        // caller's materialization root, including on a host with Windows path
        // semantics.
        //
        // Tracked by destination -> source archive path, not just a set of destinations: OMG
        // library indexes legitimately alias a short name and a full name to the same file (for
        // example "USCU" and "USCustomaryUnits" both mapping to "USCustomaryUnits.sysml"), and
        // both collapse to the same (destination, archive_path) pair below since neither key is
        // itself a source-file name. That is two entries agreeing on one file, not a conflict --
        // only two entries that disagree on which archive content should land at the same
        // destination are an actual ambiguity.
        let mut planned = Vec::with_capacity(paths.len());
        let mut materialized_paths: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for (logical_path, archive_path) in paths {
            let logical_path = normalize_zip_path(&logical_path)?;
            let archive_path = normalize_zip_path(&archive_path)?;
            match materialized_paths.get(&logical_path) {
                Some(existing) if *existing == archive_path => continue,
                Some(_) => {
                    return Err(KparError::InvalidArchive(format!(
                        "multiple source entries materialize to '{logical_path}'"
                    )));
                }
                None => {
                    materialized_paths.insert(logical_path.clone(), archive_path.clone());
                }
            }
            let Some(bytes) = entries.get(&archive_path) else {
                return Err(KparError::InvalidArchive(format!(
                    "missing archive entry '{archive_path}' for '{logical_path}'"
                )));
            };
            planned.push((logical_path, bytes.clone()));
        }
        Ok(MaterializationPlan {
            source_files: planned,
        })
    }
}

fn publish_materialization(destination_root: &Path, plan: &MaterializationPlan) -> Result<()> {
    ensure_absent_publish_target(destination_root)?;
    let parent = destination_root.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|source| KparError::Io {
        path: parent.display().to_string(),
        source,
    })?;
    let staging = tempfile::Builder::new()
        .prefix(".kpar-staging-")
        .tempdir_in(parent)
        .map_err(|source| KparError::Io {
            path: parent.display().to_string(),
            source,
        })?;
    for (logical_path, bytes) in &plan.source_files {
        let staged_path = staging.path().join(logical_path);
        if let Some(parent) = staged_path.parent() {
            fs::create_dir_all(parent).map_err(|source| KparError::Io {
                path: parent.display().to_string(),
                source,
            })?;
        }
        fs::write(&staged_path, bytes).map_err(|source| KparError::Io {
            path: staged_path.display().to_string(),
            source,
        })?;
    }
    ensure_absent_publish_target(destination_root)?;
    fs::rename(staging.path(), destination_root).map_err(|source| KparError::Io {
        path: destination_root.display().to_string(),
        source,
    })
}

/// Refuse to publish over any existing filesystem object.
///
/// Public KPAR packing and materialization are no-force operations. A caller
/// must select a fresh target rather than treating archive publication as an
/// implicit destructive update. Directory publication performs an additional
/// pre-rename check, but cannot make that check atomic on every host.
pub(crate) fn ensure_absent_publish_target(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(_) => Err(KparError::InvalidArchive(format!(
            "refusing to replace existing publication target '{}'",
            path.display()
        ))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(KparError::Io {
            path: path.display().to_string(),
            source,
        }),
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
    project.validate_identity()?;
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
        let name = normalize_zip_path(entry.name())?;
        let mut buf = Vec::new();
        entry
            .read_to_end(&mut buf)
            .map_err(|e| KparError::Zip(format!("read {name}: {e}")))?;
        if entries.insert(name.clone(), buf).is_some() {
            return Err(KparError::InvalidArchive(format!(
                "duplicate archive entry '{name}'"
            )));
        }
    }
    Ok(entries)
}

/// Canonicalize a portable, relative KPAR entry path.
///
/// ZIP member names and KPAR index values are untrusted archive data. Treating
/// them as ordinary filesystem paths would make `destination_root.join(...)`
/// vulnerable to `..`, absolute, or Windows-drive path escapes.
pub(crate) fn normalize_zip_path(path: &str) -> Result<String> {
    let path = path.replace('\\', "/");
    if path.starts_with('/') || is_windows_drive_path(&path) {
        return Err(KparError::InvalidArchive(format!(
            "archive path must be relative: '{path}'"
        )));
    }

    let mut components = Vec::new();
    for component in path.split('/') {
        match component {
            "" => {
                return Err(KparError::InvalidArchive(format!(
                    "archive path contains an empty component: '{path}'"
                )));
            }
            "." => continue,
            ".." => {
                return Err(KparError::InvalidArchive(format!(
                    "archive path contains a parent component: '{path}'"
                )));
            }
            component if component.contains('\0') || component.contains(':') => {
                return Err(KparError::InvalidArchive(format!(
                    "archive path is not portable: '{path}'"
                )));
            }
            component => components.push(component),
        }
    }

    if components.is_empty() {
        return Err(KparError::InvalidArchive(
            "archive path is empty".to_string(),
        ));
    }
    Ok(components.join("/"))
}

fn is_windows_drive_path(path: &str) -> bool {
    matches!(path.as_bytes(), [drive, b':', ..] if drive.is_ascii_alphabetic())
}

fn is_source_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    SOURCE_EXTENSIONS.iter().any(|ext| lower.ends_with(ext))
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pack::{build_kpar, ArchiveTimestamp, PackOptions};
    use tempfile::tempdir;

    fn stored_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        fn push_u16(bytes: &mut Vec<u8>, value: u16) {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        fn push_u32(bytes: &mut Vec<u8>, value: u32) {
            bytes.extend_from_slice(&value.to_le_bytes());
        }

        let mut bytes = Vec::new();
        let mut central = Vec::new();
        for (name, contents) in entries {
            let offset = u32::try_from(bytes.len()).expect("small test archive");
            let name = name.as_bytes();
            let size = u32::try_from(contents.len()).expect("small test entry");
            let checksum = crc32fast::hash(contents);
            push_u32(&mut bytes, 0x0403_4b50);
            push_u16(&mut bytes, 20);
            push_u16(&mut bytes, 0);
            push_u16(&mut bytes, 0);
            push_u16(&mut bytes, 0);
            push_u16(&mut bytes, 0);
            push_u32(&mut bytes, checksum);
            push_u32(&mut bytes, size);
            push_u32(&mut bytes, size);
            push_u16(
                &mut bytes,
                u16::try_from(name.len()).expect("short test name"),
            );
            push_u16(&mut bytes, 0);
            bytes.extend_from_slice(name);
            bytes.extend_from_slice(contents);

            push_u32(&mut central, 0x0201_4b50);
            push_u16(&mut central, 20);
            push_u16(&mut central, 20);
            push_u16(&mut central, 0);
            push_u16(&mut central, 0);
            push_u16(&mut central, 0);
            push_u16(&mut central, 0);
            push_u32(&mut central, checksum);
            push_u32(&mut central, size);
            push_u32(&mut central, size);
            push_u16(
                &mut central,
                u16::try_from(name.len()).expect("short test name"),
            );
            push_u16(&mut central, 0);
            push_u16(&mut central, 0);
            push_u16(&mut central, 0);
            push_u16(&mut central, 0);
            push_u32(&mut central, 0);
            push_u32(&mut central, offset);
            central.extend_from_slice(name);
        }
        let central_offset = u32::try_from(bytes.len()).expect("small test archive");
        let central_size = u32::try_from(central.len()).expect("small test archive");
        bytes.extend_from_slice(&central);
        push_u32(&mut bytes, 0x0605_4b50);
        push_u16(&mut bytes, 0);
        push_u16(&mut bytes, 0);
        let count = u16::try_from(entries.len()).expect("small test archive");
        push_u16(&mut bytes, count);
        push_u16(&mut bytes, count);
        push_u32(&mut bytes, central_size);
        push_u32(&mut bytes, central_offset);
        push_u16(&mut bytes, 0);
        bytes
    }

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
                named_source_roots: vec![],
                excludes: vec![],
                timestamp: ArchiveTimestamp::default(),
                compression: crate::pack::ArchiveCompression::default(),
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

    #[test]
    fn open_rejects_path_like_project_identity() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("invalid-project.kpar");
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file(PROJECT_FILE, options)
                .expect("project start");
            zip.write_all(br#"{"name":"../outside","version":"1.0.0"}"#)
                .expect("project write");
            zip.start_file(META_FILE, options).expect("meta start");
            zip.write_all(br#"{"index":{},"created":"2025-03-13T00:00:00Z"}"#)
                .expect("meta write");
            zip.finish().expect("finish");
        }

        let error = open_kpar_path(&path).expect_err("path-like identity must be rejected");
        assert!(matches!(error, KparError::InvalidArchive(_)));
    }

    #[test]
    fn materialize_omg_style_index_entries() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("omg-style.kpar");
        let scalar_values = b"standard library package ScalarValues { attribute def Real; }";
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file(PROJECT_FILE, options)
                .expect("project start");
            zip.write_all(br#"{"name":"Kernel Data Type Library","version":"1.0.0"}"#)
                .expect("project write");
            zip.start_file(META_FILE, options).expect("meta start");
            let meta = format!(
                r#"{{
  "index": {{"ScalarValues": "ScalarValues.kerml"}},
  "created": "2025-03-13T00:00:00Z",
  "checksum": {{
    "ScalarValues.kerml": {{"value": "{}", "algorithm": "SHA256"}}
  }}
}}"#,
                sha256_hex(scalar_values)
            );
            zip.write_all(meta.as_bytes()).expect("meta write");
            zip.start_file("ScalarValues.kerml", options)
                .expect("source start");
            zip.write_all(scalar_values).expect("source write");
            zip.finish().expect("finish");
        }

        let bytes = fs::read(&path).expect("read");
        let dest = source.path().join("out");
        let materialized = materialize(&bytes, &dest).expect("materialize");

        assert!(dest.join("ScalarValues.kerml").is_file());
        assert_eq!(
            materialized.source_files,
            vec![dest.join("ScalarValues.kerml")]
        );
    }

    /// The real OMG standard library indexes a short alias ("USCU") alongside the full name
    /// ("USCustomaryUnits") for the same file. Neither key is itself a source-file name, so both
    /// fall back to materializing under the archive entry's own name -- two index entries
    /// agreeing on one destination, which must succeed, not be flagged as a conflict.
    #[test]
    fn materialize_allows_an_index_alias_for_the_same_archive_entry() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("aliased.kpar");
        let contents = b"standard library package USCustomaryUnits { }";
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file(PROJECT_FILE, options)
                .expect("project start");
            zip.write_all(br#"{"name":"Quantities and Units","version":"2.0.0"}"#)
                .expect("project write");
            zip.start_file(META_FILE, options).expect("meta start");
            let meta = format!(
                r#"{{
  "index": {{
    "USCU": "USCustomaryUnits.sysml",
    "USCustomaryUnits": "USCustomaryUnits.sysml"
  }},
  "created": "2025-03-13T00:00:00Z",
  "checksum": {{
    "USCustomaryUnits.sysml": {{"value": "{}", "algorithm": "SHA256"}}
  }}
}}"#,
                sha256_hex(contents)
            );
            zip.write_all(meta.as_bytes()).expect("meta write");
            zip.start_file("USCustomaryUnits.sysml", options)
                .expect("source start");
            zip.write_all(contents).expect("source write");
            zip.finish().expect("finish");
        }

        let bytes = fs::read(&path).expect("read");
        let dest = source.path().join("out");
        let materialized = materialize(&bytes, &dest).expect("aliased index must materialize");

        assert!(dest.join("USCustomaryUnits.sysml").is_file());
        assert_eq!(
            materialized.source_files,
            vec![dest.join("USCustomaryUnits.sysml")]
        );
    }

    /// Unlike the alias case above, two index *keys* that normalize to the same destination
    /// while pointing at different archive content are a real ambiguity, not aliasing, and must
    /// still be rejected: `./Model.sysml` and `Model.sysml` are different JSON keys (so parsing
    /// them is not itself a conflict) but the same destination once `.` components are dropped.
    #[test]
    fn materialize_rejects_an_index_conflict_between_different_archive_entries() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("conflicting.kpar");
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file(PROJECT_FILE, options)
                .expect("project start");
            zip.write_all(br#"{"name":"Conflicting","version":"1.0.0"}"#)
                .expect("project write");
            zip.start_file(META_FILE, options).expect("meta start");
            zip.write_all(
                br#"{"index":{"./Model.sysml":"a.sysml","Model.sysml":"b.sysml"},"created":"2025-03-13T00:00:00Z"}"#,
            )
            .expect("meta write");
            zip.start_file("a.sysml", options).expect("a start");
            zip.write_all(b"package A {}").expect("a write");
            zip.start_file("b.sysml", options).expect("b start");
            zip.write_all(b"package B {}").expect("b write");
            zip.finish().expect("finish");
        }

        let bytes = fs::read(&path).expect("read");
        let dest = source.path().join("out");
        let error = materialize(&bytes, &dest).expect_err("differing archive entries must clash");
        assert!(matches!(error, KparError::InvalidArchive(_)));
    }

    #[test]
    fn materialize_rejects_index_path_traversal_before_writing() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("traversal.kpar");
        let contents = b"package Escaped {}";
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file(PROJECT_FILE, options)
                .expect("project start");
            zip.write_all(br#"{"name":"Traversal","version":"1.0.0"}"#)
                .expect("project write");
            zip.start_file(META_FILE, options).expect("meta start");
            let meta = format!(
                r#"{{
  "index": {{"../escaped.sysml": "Source.sysml"}},
  "created": "2025-03-13T00:00:00Z",
  "checksum": {{
    "../escaped.sysml": {{"value": "{}", "algorithm": "SHA256"}}
  }}
}}"#,
                sha256_hex(contents)
            );
            zip.write_all(meta.as_bytes()).expect("meta write");
            zip.start_file("Source.sysml", options)
                .expect("source start");
            zip.write_all(contents).expect("source write");
            zip.finish().expect("finish");
        }

        let destination = source.path().join("out");
        let error = materialize(&fs::read(&path).expect("read"), &destination)
            .expect_err("traversal path must be rejected");
        assert!(matches!(error, KparError::InvalidArchive(_)));
        assert!(
            !destination.exists(),
            "invalid archive must not create output"
        );
        assert!(
            !source.path().join("escaped.sysml").exists(),
            "invalid archive must not write outside its root"
        );
    }

    #[test]
    fn open_rejects_traversal_zip_member() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("traversal-member.kpar");
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file(PROJECT_FILE, options)
                .expect("project start");
            zip.write_all(br#"{"name":"Traversal","version":"1.0.0"}"#)
                .expect("project write");
            zip.start_file(META_FILE, options).expect("meta start");
            zip.write_all(br#"{"index":{},"created":"2025-03-13T00:00:00Z"}"#)
                .expect("meta write");
            zip.start_file("../escaped.sysml", options)
                .expect("source start");
            zip.write_all(b"package Escaped {}").expect("source write");
            zip.finish().expect("finish");
        }

        let error = open_kpar_path(&path).expect_err("traversal member must be rejected");
        assert!(matches!(error, KparError::InvalidArchive(_)));
    }

    #[test]
    fn duplicate_archive_entries_are_rejected_without_replacing_destination() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("duplicate.kpar");
        fs::write(
            &path,
            stored_zip(&[
                (PROJECT_FILE, br#"{"name":"Duplicate","version":"1.0.0"}"#),
                (
                    META_FILE,
                    br#"{"index":{},"created":"2025-03-13T00:00:00Z"}"#,
                ),
                ("models/./Model.sysml", b"package First {}"),
                ("models/Model.sysml", b"package Second {}"),
            ]),
        )
        .expect("write duplicate archive");
        let destination = source.path().join("existing");
        fs::create_dir_all(&destination).expect("existing destination");
        fs::write(destination.join("sentinel.txt"), "keep").expect("sentinel");

        let error = materialize(&fs::read(&path).expect("read"), &destination)
            .expect_err("duplicate archive entry must be rejected");

        assert!(matches!(error, KparError::InvalidArchive(_)));
        assert_eq!(
            fs::read_to_string(destination.join("sentinel.txt")).expect("sentinel remains"),
            "keep"
        );
    }

    #[test]
    fn checksum_failure_preserves_existing_destination() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("corrupt.kpar");
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file(PROJECT_FILE, options)
                .expect("project start");
            zip.write_all(br#"{"name":"Corrupt","version":"1.0.0"}"#)
                .expect("project write");
            zip.start_file(META_FILE, options).expect("meta start");
            zip.write_all(
                br#"{"index":{"Model.sysml":"Model.sysml"},"created":"2025-03-13T00:00:00Z","checksum":{"Model.sysml":{"value":"not-a-sha256","algorithm":"SHA256"}}}"#,
            )
            .expect("meta write");
            zip.start_file("Model.sysml", options)
                .expect("source start");
            zip.write_all(b"package Corrupt {}").expect("source write");
            zip.finish().expect("finish");
        }
        let destination = source.path().join("existing");
        fs::create_dir_all(&destination).expect("existing destination");
        fs::write(destination.join("sentinel.txt"), "keep").expect("sentinel");

        let error = materialize(&fs::read(&path).expect("read"), &destination)
            .expect_err("checksum mismatch must be rejected");

        assert!(matches!(error, KparError::ChecksumMismatch { .. }));
        assert_eq!(
            fs::read_to_string(destination.join("sentinel.txt")).expect("sentinel remains"),
            "keep"
        );
    }

    #[test]
    fn unsupported_checksum_algorithm_preserves_existing_destination() {
        let source = tempdir().expect("tempdir");
        let path = source.path().join("unsupported-checksum.kpar");
        let contents = b"package Example {}";
        {
            use std::io::Write;
            use zip::write::{SimpleFileOptions, ZipWriter};
            let file = fs::File::create(&path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file(PROJECT_FILE, options)
                .expect("project start");
            zip.write_all(br#"{"name":"Example","version":"1.0.0"}"#)
                .expect("project write");
            zip.start_file(META_FILE, options).expect("meta start");
            let meta = format!(
                r#"{{"index":{{"Model.sysml":"Model.sysml"}},"created":"2025-03-13T00:00:00Z","checksum":{{"Model.sysml":{{"value":"{}","algorithm":"SHA1"}}}}}}"#,
                sha256_hex(contents)
            );
            zip.write_all(meta.as_bytes()).expect("meta write");
            zip.start_file("Model.sysml", options)
                .expect("source start");
            zip.write_all(contents).expect("source write");
            zip.finish().expect("finish");
        }
        let destination = source.path().join("existing");
        fs::create_dir_all(&destination).expect("existing destination");
        fs::write(destination.join("sentinel.txt"), "keep").expect("sentinel");

        let error = materialize(&fs::read(&path).expect("read"), &destination)
            .expect_err("unsupported checksum algorithm must be rejected");

        assert!(matches!(error, KparError::InvalidArchive(_)));
        assert_eq!(
            fs::read_to_string(destination.join("sentinel.txt")).expect("sentinel remains"),
            "keep"
        );
    }

    #[test]
    fn materialize_refuses_to_replace_an_existing_destination() {
        let source = tempdir().expect("tempdir");
        let model = source.path().join("models/Example.sysml");
        fs::create_dir_all(model.parent().expect("parent")).expect("create models");
        fs::write(&model, "package Example {}").expect("write model");
        let archive_path = source.path().join("example.kpar");
        build_kpar(
            &PackOptions {
                project: Project {
                    name: "example".to_string(),
                    version: "1.0.0".to_string(),
                    description: None,
                    license: None,
                    publisher: None,
                    maintainer: vec![],
                    website: None,
                    topic: vec![],
                    usage: vec![],
                },
                source_roots: vec![source.path().join("models")],
                named_source_roots: vec![],
                excludes: vec![],
                timestamp: ArchiveTimestamp::default(),
                compression: crate::pack::ArchiveCompression::default(),
            },
            &archive_path,
        )
        .expect("pack archive");
        let destination = source.path().join("existing");
        fs::create_dir_all(&destination).expect("existing destination");
        fs::write(destination.join("sentinel.txt"), "keep").expect("sentinel");

        let error = materialize(
            &fs::read(&archive_path).expect("read archive"),
            &destination,
        )
        .expect_err("materialization must not replace an existing directory");

        assert!(matches!(error, KparError::InvalidArchive(_)));
        assert_eq!(
            fs::read_to_string(destination.join("sentinel.txt")).expect("sentinel remains"),
            "keep"
        );
    }
}
