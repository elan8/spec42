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
        let root_name = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("src");
        collect_sources(root, root, root_name, &options.excludes, &mut files)?;
    }
    if files.is_empty() {
        return Err(KparError::InvalidArchive(
            "no source files found to pack".to_string(),
        ));
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut index = HashMap::new();
    let mut checksum = HashMap::new();
    for (path, bytes) in &files {
        index.insert(path.clone(), path.clone());
        checksum.insert(
            path.clone(),
            ChecksumEntry {
                value: sha256_hex(bytes),
                algorithm: "sha-256".to_string(),
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

    writer
        .finish()
        .map_err(|e| KparError::Zip(e.to_string()))?;
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

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}
