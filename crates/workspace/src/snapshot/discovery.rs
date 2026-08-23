//! Target discovery and URI helpers for workspace snapshots.
//!
//! File walking and URI normalisation belong to the source authority; this module only adds the
//! batch host's target semantics (a workspace root inferred from the first target) and maps the
//! authority's errors to the host's.

use std::path::{Path, PathBuf};

use sysml_query::source::{SourceError, SourceService, Url};

use crate::error::{WorkspaceError, WorkspaceResult};

pub use sysml_query::source::is_sysml_like;

pub fn resolve_workspace_root(
    targets: &[PathBuf],
    workspace_root: Option<&Path>,
) -> WorkspaceResult<PathBuf> {
    if let Some(root) = workspace_root {
        return normalize_existing_path(root);
    }
    let first = targets.first().ok_or_else(|| {
        WorkspaceError::unresolved_library_environment("No target path was provided.")
    })?;
    if first.is_dir() {
        return normalize_existing_path(first);
    }
    normalize_existing_path(first)?
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            WorkspaceError::unresolved_library_environment(format!(
                "Could not infer a workspace root from target file {}.",
                first.display()
            ))
        })
}

pub fn discover_target_files(targets: &[PathBuf]) -> WorkspaceResult<Vec<PathBuf>> {
    SourceService::new()
        .discover(targets)
        .map_err(map_source_error)
}

/// Convert a filesystem path to a canonicalized, drive-letter-normalized `file://` URL.
///
/// Public so embedders constructing publications directly can compute `library_urls` with the
/// same normalization that workspace snapshot construction applies.
pub fn path_to_file_url(path: &Path) -> WorkspaceResult<Url> {
    sysml_query::source::path_to_file_url(path).map_err(map_source_error)
}

fn map_source_error(error: SourceError) -> WorkspaceError {
    match error {
        SourceError::InvalidUri { .. } => WorkspaceError::invalid_document_uri(error.to_string()),
        other => WorkspaceError::unresolved_library_environment(other.to_string()),
    }
}

fn normalize_existing_path(path: &Path) -> WorkspaceResult<PathBuf> {
    if !path.exists() {
        return Err(WorkspaceError::unresolved_library_environment(format!(
            "Path does not exist: {}",
            path.display()
        )));
    }
    Ok(std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()))
}
