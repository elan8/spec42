use std::collections::BTreeMap;
use std::fs;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, FixedOffset, SecondsFormat, Timelike, Utc};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use zip::write::{SimpleFileOptions, ZipWriter};
use zip::{CompressionMethod, DateTime as ZipDateTime};

use crate::error::{KparError, Result};
use crate::read::{ensure_absent_publish_target, normalize_zip_path};
use crate::schema::{ChecksumEntry, Meta, Project, META_FILE, PROJECT_FILE};

/// Timestamp policy for an archive produced by [`build_kpar`].
///
/// The default is deliberately fixed so identical model inputs produce identical
/// bytes. A release process that needs a real creation time must provide one
/// explicitly, making that non-reproducible input visible at the call site.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ArchiveTimestamp {
    #[default]
    ReproducibleEpoch,
    FixedRfc3339(String),
}

/// Compression policy for archive members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArchiveCompression {
    #[default]
    Deflated,
    Stored,
}

impl ArchiveCompression {
    fn zip_method(self) -> CompressionMethod {
        match self {
            Self::Deflated => CompressionMethod::Deflated,
            Self::Stored => CompressionMethod::Stored,
        }
    }
}

impl ArchiveTimestamp {
    fn metadata_value(&self) -> Result<String> {
        match self {
            Self::ReproducibleEpoch => Ok("1980-01-01T00:00:00Z".to_string()),
            Self::FixedRfc3339(value) => {
                Ok(parse_timestamp(value)?.to_rfc3339_opts(SecondsFormat::Secs, true))
            }
        }
    }

    fn zip_value(&self) -> Result<ZipDateTime> {
        let timestamp = match self {
            Self::ReproducibleEpoch => {
                return ZipDateTime::from_date_and_time(1980, 1, 1, 0, 0, 0).map_err(|error| {
                    KparError::InvalidArchive(format!(
                        "invalid reproducible archive timestamp: {error}"
                    ))
                });
            }
            Self::FixedRfc3339(value) => parse_timestamp(value)?.with_timezone(&Utc),
        };
        ZipDateTime::from_date_and_time(
            timestamp.year().try_into().map_err(|_| {
                KparError::InvalidArchive(format!(
                    "archive timestamp is outside ZIP's supported year range: {timestamp}"
                ))
            })?,
            timestamp.month().try_into().map_err(|_| {
                KparError::InvalidArchive(format!("invalid archive timestamp month: {timestamp}"))
            })?,
            timestamp.day().try_into().map_err(|_| {
                KparError::InvalidArchive(format!("invalid archive timestamp day: {timestamp}"))
            })?,
            timestamp.hour().try_into().map_err(|_| {
                KparError::InvalidArchive(format!("invalid archive timestamp hour: {timestamp}"))
            })?,
            timestamp.minute().try_into().map_err(|_| {
                KparError::InvalidArchive(format!("invalid archive timestamp minute: {timestamp}"))
            })?,
            timestamp.second().try_into().map_err(|_| {
                KparError::InvalidArchive(format!("invalid archive timestamp second: {timestamp}"))
            })?,
        )
        .map_err(|error| {
            KparError::InvalidArchive(format!("invalid archive timestamp '{timestamp}': {error}"))
        })
    }
}

fn parse_timestamp(value: &str) -> Result<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(value).map_err(|error| {
        KparError::InvalidArchive(format!("archive timestamp must be RFC 3339: {error}"))
    })
}

/// Options for [`build_kpar`].
#[derive(Debug, Clone)]
pub struct PackOptions {
    pub project: Project,
    /// Root directories whose files are included (e.g. domain/, technical/, generic/).
    /// Archive paths use each directory's file name as the prefix.
    pub source_roots: Vec<PathBuf>,
    /// Additional roots packed under an explicit archive prefix (e.g. `method` → `method/...`).
    pub named_source_roots: Vec<(String, PathBuf)>,
    /// Path prefixes to exclude (e.g. "examples/", "scripts/").
    pub excludes: Vec<String>,
    /// Explicit creation-time policy. Defaults to a reproducible fixed epoch.
    pub timestamp: ArchiveTimestamp,
    /// Compression policy. Defaults to deflate; `Stored` is useful for transparent inspection.
    pub compression: ArchiveCompression,
}

impl PackOptions {
    pub fn domain_libraries_defaults(project: Project, repo_root: &Path) -> Self {
        Self {
            project,
            source_roots: ["domain", "technical", "generic"]
                .iter()
                .map(|name| repo_root.join(name))
                .filter(|p| p.is_dir())
                .collect(),
            named_source_roots: Vec::new(),
            excludes: default_domain_excludes(),
            timestamp: ArchiveTimestamp::default(),
            compression: ArchiveCompression::default(),
        }
    }

    /// Pack `path` under `archive_prefix/` in the KPAR (e.g. method libraries as `method/`).
    pub fn with_named_source_root(
        mut self,
        archive_prefix: impl Into<String>,
        path: PathBuf,
    ) -> Self {
        if path.is_dir() {
            self.named_source_roots.push((archive_prefix.into(), path));
        }
        self
    }
}

pub fn default_domain_excludes() -> Vec<String> {
    vec![
        ".git/".to_string(),
        "examples/".to_string(),
        "scripts/".to_string(),
        "docs/".to_string(),
    ]
}

/// Pack source trees into a KPAR file at `dest`.
pub fn build_kpar(options: &PackOptions, dest: &Path) -> Result<()> {
    options.project.validate_identity()?;
    let mut files = Vec::new();
    for root in &options.source_roots {
        if !root.is_dir() {
            continue;
        }
        let root_name = root.file_name().and_then(|n| n.to_str()).unwrap_or("src");
        collect_sources(root, root, root_name, &options.excludes, &mut files)?;
    }
    for (prefix, root) in &options.named_source_roots {
        if !root.is_dir() {
            continue;
        }
        collect_sources(root, root, prefix, &options.excludes, &mut files)?;
    }
    if files.is_empty() {
        return Err(KparError::InvalidArchive(
            "no source files found to pack".to_string(),
        ));
    }
    files.sort_by(|a: &SourceFile, b: &SourceFile| a.archive_path.cmp(&b.archive_path));
    if let Some(duplicate) = files.windows(2).find_map(|pair| {
        (pair[0].archive_path == pair[1].archive_path).then(|| pair[0].archive_path.clone())
    }) {
        return Err(KparError::InvalidArchive(format!(
            "multiple source roots produce archive path '{duplicate}'"
        )));
    }

    let index = build_index(&files);
    let mut checksum = BTreeMap::new();
    for file in &files {
        checksum.insert(
            file.archive_path.clone(),
            ChecksumEntry {
                value: sha256_hex(&file.bytes),
                algorithm: "SHA256".to_string(),
            },
        );
    }

    let meta = Meta {
        index,
        created: options.timestamp.metadata_value()?,
        metamodel: Some("https://www.omg.org/spec/KerML/20250201".to_string()),
        includes_derived: Some(false),
        includes_implied: Some(false),
        checksum,
    };

    let archive = encode_archive(options, &meta, &files)?;
    publish_archive(dest, &archive)
}

fn encode_archive(options: &PackOptions, meta: &Meta, files: &[SourceFile]) -> Result<Vec<u8>> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options_zip = SimpleFileOptions::default()
        .last_modified_time(options.timestamp.zip_value()?)
        .compression_method(options.compression.zip_method())
        .unix_permissions(0o644);

    let project_json = serde_json::to_vec_pretty(&options.project)?;
    writer
        .start_file(PROJECT_FILE, options_zip)
        .map_err(|e| KparError::Zip(e.to_string()))?;
    writer
        .write_all(&project_json)
        .map_err(|e| KparError::Zip(e.to_string()))?;

    let meta_json = serde_json::to_vec_pretty(meta)?;
    writer
        .start_file(META_FILE, options_zip)
        .map_err(|e| KparError::Zip(e.to_string()))?;
    writer
        .write_all(&meta_json)
        .map_err(|e| KparError::Zip(e.to_string()))?;

    for file in files {
        writer
            .start_file(&file.archive_path, options_zip)
            .map_err(|e| KparError::Zip(e.to_string()))?;
        writer
            .write_all(&file.bytes)
            .map_err(|e| KparError::Zip(e.to_string()))?;
    }

    writer
        .finish()
        .map_err(|e| KparError::Zip(e.to_string()))
        .map(|cursor| cursor.into_inner())
}

fn publish_archive(dest: &Path, archive: &[u8]) -> Result<()> {
    ensure_absent_publish_target(dest)?;
    let parent = dest.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|source| KparError::Io {
        path: parent.display().to_string(),
        source,
    })?;
    let mut temporary =
        tempfile::NamedTempFile::new_in(parent).map_err(|source| KparError::Io {
            path: parent.display().to_string(),
            source,
        })?;
    temporary
        .write_all(archive)
        .map_err(|source| KparError::Io {
            path: temporary.path().display().to_string(),
            source,
        })?;
    temporary
        .persist_noclobber(dest)
        .map_err(|error| KparError::Io {
            path: dest.display().to_string(),
            source: error.error,
        })?;
    Ok(())
}

#[derive(Debug)]
struct SourceFile {
    archive_path: String,
    bytes: Vec<u8>,
    logical_name: Option<String>,
}

fn collect_sources(
    repo_root: &Path,
    current: &Path,
    prefix: &str,
    excludes: &[String],
    out: &mut Vec<SourceFile>,
) -> Result<()> {
    for entry in WalkDir::new(current).follow_links(false) {
        let entry = entry.map_err(|error| {
            KparError::InvalidArchive(format!(
                "failed to traverse source root '{}': {error}",
                current.display()
            ))
        })?;
        let path = entry.path();
        if entry.file_type().is_symlink() {
            return Err(KparError::InvalidArchive(format!(
                "source root contains symbolic link '{}'; bundle only regular files under the selected root",
                path.display()
            )));
        }
        if path.is_dir() {
            continue;
        }
        let relative = path
            .strip_prefix(repo_root)
            .map_err(|_| KparError::InvalidArchive("strip_prefix failed".to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        let archive_path = if prefix.is_empty() {
            relative.clone()
        } else if relative.is_empty() {
            prefix.to_string()
        } else {
            format!("{prefix}/{relative}")
        };
        let archive_path = normalize_zip_path(&archive_path)?;
        if should_exclude(&archive_path, excludes) {
            continue;
        }
        if !is_model_file(path) {
            continue;
        }
        let bytes = fs::read(path).map_err(|source| KparError::Io {
            path: path.display().to_string(),
            source,
        })?;
        let logical_name = package_name_from_strict_ast(&archive_path, &bytes)?;
        out.push(SourceFile {
            archive_path,
            bytes,
            logical_name,
        });
    }
    Ok(())
}

fn should_exclude(path: &str, excludes: &[String]) -> bool {
    let normalized = path.replace('\\', "/");
    excludes.iter().any(|ex| {
        let ex = ex.trim_matches('/');
        normalized.starts_with(&format!("{ex}/"))
            || normalized.contains(&format!("/{ex}/"))
            || normalized == ex
    })
}

fn is_model_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("sysml") | Some("kerml")
    )
}

fn build_index(files: &[SourceFile]) -> BTreeMap<String, String> {
    let mut candidates = Vec::new();
    let mut counts = BTreeMap::<String, usize>::new();
    for file in files {
        let logical_name = file
            .logical_name
            .clone()
            .unwrap_or_else(|| file.archive_path.clone());
        *counts.entry(logical_name.clone()).or_default() += 1;
        candidates.push((logical_name, file.archive_path.clone()));
    }

    candidates
        .into_iter()
        .map(|(logical_name, path)| {
            if counts.get(&logical_name).copied().unwrap_or(0) == 1 {
                (logical_name, path)
            } else {
                (path.clone(), path)
            }
        })
        .collect()
}

fn package_name_from_strict_ast(path: &str, bytes: &[u8]) -> Result<Option<String>> {
    let source = std::str::from_utf8(bytes).map_err(|error| {
        KparError::InvalidArchive(format!("source '{path}' is not valid UTF-8: {error}"))
    })?;
    // The parser authority answers this. `kpar` needs a package identity, not a syntax tree, and
    // parsing here would be a second parse of the same text against an AST this crate would then
    // have to keep in step with the pinned revision.
    let names = sysml_resolution::syntax::package_declaration_names(source).map_err(|error| {
        KparError::InvalidArchive(format!("source '{path}' failed strict parsing: {error}"))
    })?;
    if names.len() == 1 {
        Ok(names.into_iter().next())
    } else {
        Ok(None)
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read::{materialize, open_kpar_path, verify_checksums};
    use tempfile::tempdir;

    fn test_project() -> Project {
        Project {
            name: "elan8-domain-libraries".to_string(),
            version: "0.1.0".to_string(),
            description: Some("Elan8 SysML v2 domain libraries".to_string()),
            license: Some("MIT".to_string()),
            publisher: Some("elan8".to_string()),
            maintainer: vec![],
            website: None,
            topic: vec![],
            usage: vec![],
        }
    }

    #[test]
    fn domain_libraries_defaults_pack_expected_roots_and_materialize() {
        let repo = tempdir().expect("temp repo");
        let monetary_units = repo
            .path()
            .join("generic")
            .join("units")
            .join("MonetaryUnits.sysml");
        fs::create_dir_all(monetary_units.parent().unwrap()).expect("generic units dir");
        fs::write(
            &monetary_units,
            "package MonetaryUnits { attribute <EUR> 'euro'; }",
        )
        .expect("write monetary units");

        let robotics_core = repo
            .path()
            .join("domain")
            .join("robotics")
            .join("RoboticsCore.sysml");
        fs::create_dir_all(robotics_core.parent().unwrap()).expect("robotics dir");
        fs::write(&robotics_core, "package RoboticsCore {}").expect("write robotics");

        let software_core = repo
            .path()
            .join("technical")
            .join("software")
            .join("SoftwareCore.sysml");
        fs::create_dir_all(software_core.parent().unwrap()).expect("software dir");
        fs::write(&software_core, "package SoftwareCore {}").expect("write software");

        let duplicate_core = repo
            .path()
            .join("technical")
            .join("software")
            .join("duplicate")
            .join("SoftwareCore.sysml");
        fs::create_dir_all(duplicate_core.parent().unwrap()).expect("duplicate dir");
        fs::write(&duplicate_core, "package SoftwareCore {}").expect("write duplicate");

        let example = repo.path().join("examples").join("Ignored.sysml");
        fs::create_dir_all(example.parent().unwrap()).expect("examples dir");
        fs::write(&example, "package Ignored {}").expect("write ignored example");

        let options = PackOptions::domain_libraries_defaults(test_project(), repo.path());
        let kpar_path = repo.path().join("elan8-domain-libraries-0.1.0.kpar");
        build_kpar(&options, &kpar_path).expect("pack domain libraries");
        verify_checksums(&fs::read(&kpar_path).expect("read kpar")).expect("checksums");

        let archive = open_kpar_path(&kpar_path).expect("open kpar");
        assert_eq!(archive.project.name, "elan8-domain-libraries");
        assert_eq!(
            archive.meta.index.get("MonetaryUnits"),
            Some(&"generic/units/MonetaryUnits.sysml".to_string())
        );
        assert_eq!(
            archive
                .meta
                .checksum
                .get("generic/units/MonetaryUnits.sysml")
                .map(|entry| entry.algorithm.as_str()),
            Some("SHA256")
        );
        assert!(archive.meta.index.contains_key("RoboticsCore"));
        assert!(archive
            .meta
            .index
            .contains_key("technical/software/SoftwareCore.sysml"));
        assert!(archive
            .meta
            .index
            .contains_key("technical/software/duplicate/SoftwareCore.sysml"));
        assert!(!archive.meta.index.contains_key("SoftwareCore"));
        assert!(!archive.meta.index.contains_key("examples/Ignored.sysml"));

        let out = repo.path().join("out");
        let materialized =
            materialize(&fs::read(&kpar_path).expect("read kpar"), &out).expect("materialize");
        assert_eq!(materialized.source_files.len(), 4);
        assert!(out.join("generic/units/MonetaryUnits.sysml").is_file());
        assert!(out.join("domain/robotics/RoboticsCore.sysml").is_file());
        assert!(out.join("technical/software/SoftwareCore.sysml").is_file());
        assert!(out
            .join("technical/software/duplicate/SoftwareCore.sysml")
            .is_file());
    }

    #[test]
    fn named_source_root_packs_under_explicit_prefix() {
        let repo = tempdir().expect("temp repo");
        let domain = repo.path().join("domain").join("Core.sysml");
        fs::create_dir_all(domain.parent().unwrap()).expect("domain dir");
        fs::write(&domain, "package DomainCore {}").expect("write domain");

        let method = tempdir().expect("method library");
        let method_file = method.path().join("Elan8Method.sysml");
        fs::write(&method_file, "library package Elan8Method {}").expect("write method");

        let options = PackOptions::domain_libraries_defaults(test_project(), repo.path())
            .with_named_source_root("method", method.path().to_path_buf());
        let kpar_path = repo.path().join("bundle.kpar");
        build_kpar(&options, &kpar_path).expect("pack");

        let archive = open_kpar_path(&kpar_path).expect("open");
        assert_eq!(
            archive.meta.index.get("Elan8Method"),
            Some(&"method/Elan8Method.sysml".to_string())
        );

        let out = repo.path().join("out");
        materialize(&fs::read(&kpar_path).expect("read"), &out).expect("materialize");
        assert!(out.join("method/Elan8Method.sysml").is_file());
        assert!(out.join("domain/Core.sysml").is_file());
    }

    #[test]
    fn reproducible_timestamp_produces_identical_archive_bytes() {
        let repo = tempdir().expect("temp repo");
        let model = repo.path().join("domain/Example.sysml");
        fs::create_dir_all(model.parent().expect("parent")).expect("create source root");
        fs::write(&model, "package Example {}").expect("write model");
        let options = PackOptions::domain_libraries_defaults(test_project(), repo.path());
        let first = repo.path().join("first.kpar");
        let second = repo.path().join("second.kpar");

        build_kpar(&options, &first).expect("first pack");
        build_kpar(&options, &second).expect("second pack");

        assert_eq!(
            fs::read(first).expect("read first archive"),
            fs::read(second).expect("read second archive"),
            "a fixed default timestamp, ordered metadata, and fixed ZIP metadata make packing reproducible"
        );
    }

    #[test]
    fn packing_rejects_invalid_source_without_publishing_archive() {
        let repo = tempdir().expect("temp repo");
        let model = repo.path().join("domain/Broken.sysml");
        fs::create_dir_all(model.parent().expect("parent")).expect("create source root");
        fs::write(&model, "package Broken {").expect("write invalid model");
        let destination = repo.path().join("broken.kpar");

        let error = build_kpar(
            &PackOptions::domain_libraries_defaults(test_project(), repo.path()),
            &destination,
        )
        .expect_err("strict parse failure must reject packing");

        assert!(matches!(error, KparError::InvalidArchive(_)));
        assert!(
            !destination.exists(),
            "no archive may be published after source validation fails"
        );
    }

    #[test]
    fn packing_refuses_to_replace_an_existing_archive() {
        let repo = tempdir().expect("temp repo");
        let model = repo.path().join("domain/Example.sysml");
        fs::create_dir_all(model.parent().expect("parent")).expect("create source root");
        fs::write(&model, "package Example {}").expect("write model");
        let destination = repo.path().join("existing.kpar");
        fs::write(&destination, "preserve me").expect("write sentinel archive");

        let error = build_kpar(
            &PackOptions::domain_libraries_defaults(test_project(), repo.path()),
            &destination,
        )
        .expect_err("packing must not replace an existing archive");

        assert!(matches!(error, KparError::InvalidArchive(_)));
        assert_eq!(
            fs::read_to_string(destination).expect("sentinel remains"),
            "preserve me"
        );
    }

    #[test]
    fn packing_rejects_invalid_project_identity_without_publishing_archive() {
        let repo = tempdir().expect("temp repo");
        let model = repo.path().join("domain/Example.sysml");
        fs::create_dir_all(model.parent().expect("parent")).expect("create source root");
        fs::write(&model, "package Example {}").expect("write model");
        let destination = repo.path().join("invalid.kpar");
        let mut options = PackOptions::domain_libraries_defaults(test_project(), repo.path());
        options.project.version = "../invalid".to_string();

        let error = build_kpar(&options, &destination)
            .expect_err("invalid project identity must reject archive publication");

        assert!(matches!(error, KparError::InvalidArchive(_)));
        assert!(!destination.exists());
    }

    #[test]
    fn package_index_uses_the_strict_ast_not_comment_text() {
        let repo = tempdir().expect("temp repo");
        let model = repo.path().join("domain/Actual.sysml");
        fs::create_dir_all(model.parent().expect("parent")).expect("create source root");
        fs::write(&model, "// package Incorrect\npackage Actual {}").expect("write model");
        let destination = repo.path().join("actual.kpar");

        build_kpar(
            &PackOptions::domain_libraries_defaults(test_project(), repo.path()),
            &destination,
        )
        .expect("pack strict AST source");

        let archive = open_kpar_path(&destination).expect("open archive");
        assert_eq!(
            archive.meta.index.get("Actual"),
            Some(&"domain/Actual.sysml".to_string())
        );
        assert!(!archive.meta.index.contains_key("Incorrect"));
    }

    #[cfg(unix)]
    #[test]
    fn packing_rejects_symlinked_sources() {
        use std::os::unix::fs::symlink;

        let repo = tempdir().expect("temp repo");
        let domain = repo.path().join("domain");
        fs::create_dir_all(&domain).expect("create source root");
        let outside = repo.path().join("outside.sysml");
        fs::write(&outside, "package Outside {}").expect("write outside source");
        symlink(&outside, domain.join("linked.sysml")).expect("create source symlink");

        let error = build_kpar(
            &PackOptions::domain_libraries_defaults(test_project(), repo.path()),
            &repo.path().join("linked.kpar"),
        )
        .expect_err("symlinked source must be rejected");

        assert!(matches!(error, KparError::InvalidArchive(_)));
    }
}
