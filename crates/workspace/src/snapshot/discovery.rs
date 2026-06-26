//! Target discovery and URI helpers for workspace snapshots.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use url::Url;
use walkdir::WalkDir;

use crate::error::{HostResult, Spec42HostError};

pub(crate) fn resolve_workspace_root(
    targets: &[PathBuf],
    workspace_root: Option<&Path>,
) -> HostResult<PathBuf> {
    if let Some(root) = workspace_root {
        return normalize_existing_path(root);
    }
    let first = targets.first().ok_or_else(|| {
        Spec42HostError::unresolved_library_environment("No target path was provided.")
    })?;
    if first.is_dir() {
        return normalize_existing_path(first);
    }
    normalize_existing_path(first)?
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            Spec42HostError::unresolved_library_environment(format!(
                "Could not infer a workspace root from target file {}.",
                first.display()
            ))
        })
}

pub(crate) fn discover_target_files(targets: &[PathBuf]) -> HostResult<Vec<PathBuf>> {
    let mut files = BTreeSet::new();
    for target in targets {
        let path = normalize_existing_path(target)?;
        if path.is_file() {
            if is_sysml_like(&path) {
                files.insert(path);
            }
            continue;
        }
        for entry in WalkDir::new(&path)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            let entry_path = entry.path();
            if entry.file_type().is_file() && is_sysml_like(entry_path) {
                files.insert(entry_path.to_path_buf());
            }
        }
    }
    if files.is_empty() {
        return Err(Spec42HostError::unresolved_library_environment(
            "No .sysml or .kerml files were found under the requested path.",
        ));
    }
    Ok(files.into_iter().collect())
}

pub(crate) fn path_to_file_url(path: &Path) -> HostResult<Url> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|err| {
                Spec42HostError::unresolved_library_environment(format!(
                    "Failed to resolve current directory: {err}"
                ))
            })?
            .join(path)
    };
    let canonical = std::fs::canonicalize(&absolute).unwrap_or(absolute);
    if canonical.is_dir() {
        Url::from_directory_path(&canonical)
    } else {
        Url::from_file_path(&canonical)
    }
    .map_err(|_| {
        Spec42HostError::invalid_document_uri(format!(
            "Failed to convert path to file URI: {}",
            canonical.display()
        ))
    })
}

fn normalize_existing_path(path: &Path) -> HostResult<PathBuf> {
    if !path.exists() {
        return Err(Spec42HostError::unresolved_library_environment(format!(
            "Path does not exist: {}",
            path.display()
        )));
    }
    Ok(std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()))
}

fn is_sysml_like(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext, "sysml" | "kerml"))
        .unwrap_or(false)
}
