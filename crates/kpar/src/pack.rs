use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::Utc;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use zip::write::{SimpleFileOptions, ZipWriter};

use crate::error::{KparError, Result};
use crate::schema::{ChecksumEntry, Meta, Project, META_FILE, PROJECT_FILE};

/// Options for [`build_kpar`].
#[derive(Debug, Clone)]
pub struct PackOptions {
    pub project: Project,
    /// Root directories whose files are included (e.g. domain/, technical/, generic/).
    pub source_roots: Vec<PathBuf>,
    /// Path prefixes to exclude (e.g. "examples/", "scripts/").
    pub excludes: Vec<String>,
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
            excludes: default_domain_excludes(),
        }
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
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    for root in &options.source_roots {
        if !root.is_dir() {
            continue;
        }
        let root_name = root.file_name().and_then(|n| n.to_str()).unwrap_or("src");
        collect_sources(root, root, root_name, &options.excludes, &mut files)?;
    }
    if files.is_empty() {
        return Err(KparError::InvalidArchive(
            "no source files found to pack".to_string(),
        ));
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let index = build_index(&files);
    let mut checksum = HashMap::new();
    for (path, bytes) in &files {
        checksum.insert(
            path.clone(),
            ChecksumEntry {
                value: sha256_hex(bytes),
                algorithm: "SHA256".to_string(),
            },
        );
    }

    let meta = Meta {
        index,
        created: Utc::now().to_rfc3339(),
        metamodel: Some("https://www.omg.org/spec/KerML/20250201".to_string()),
        includes_derived: Some(false),
        includes_implied: Some(false),
        checksum,
    };

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|source| KparError::Io {
            path: parent.display().to_string(),
            source,
        })?;
    }

    let file = File::create(dest).map_err(|source| KparError::Io {
        path: dest.display().to_string(),
        source,
    })?;
    let mut writer = ZipWriter::new(file);
    let options_zip = SimpleFileOptions::default();

    let project_json = serde_json::to_vec_pretty(&options.project)?;
    writer
        .start_file(PROJECT_FILE, options_zip)
        .map_err(|e| KparError::Zip(e.to_string()))?;
    writer
        .write_all(&project_json)
        .map_err(|e| KparError::Zip(e.to_string()))?;

    let meta_json = serde_json::to_vec_pretty(&meta)?;
    writer
        .start_file(META_FILE, options_zip)
        .map_err(|e| KparError::Zip(e.to_string()))?;
    writer
        .write_all(&meta_json)
        .map_err(|e| KparError::Zip(e.to_string()))?;

    for (path, bytes) in &files {
        writer
            .start_file(path, options_zip)
            .map_err(|e| KparError::Zip(e.to_string()))?;
        writer
            .write_all(bytes)
            .map_err(|e| KparError::Zip(e.to_string()))?;
    }

    writer.finish().map_err(|e| KparError::Zip(e.to_string()))?;
    Ok(())
}

fn collect_sources(
    repo_root: &Path,
    current: &Path,
    prefix: &str,
    excludes: &[String],
    out: &mut Vec<(String, Vec<u8>)>,
) -> Result<()> {
    for entry in WalkDir::new(current)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
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
        out.push((archive_path, bytes));
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

fn build_index(files: &[(String, Vec<u8>)]) -> HashMap<String, String> {
    let mut candidates = Vec::new();
    let mut counts = HashMap::<String, usize>::new();
    for (path, bytes) in files {
        let logical_name = std::str::from_utf8(bytes)
            .ok()
            .and_then(extract_package_name)
            .unwrap_or_else(|| path.clone());
        *counts.entry(logical_name.clone()).or_default() += 1;
        candidates.push((logical_name, path.clone()));
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

fn extract_package_name(content: &str) -> Option<String> {
    for line in content.lines().take(80) {
        let trimmed = line.trim();
        let rest = trimmed
            .strip_prefix("standard library package ")
            .or_else(|| trimmed.strip_prefix("library package "))
            .or_else(|| trimmed.strip_prefix("package "));
        if let Some(rest) = rest {
            let name = rest
                .split(|c: char| !c.is_ascii_alphanumeric() && c != ':' && c != '_')
                .next()
                .unwrap_or("")
                .trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    None
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
}
