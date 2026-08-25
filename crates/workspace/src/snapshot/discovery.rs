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
    let first = targets.first().ok_or_else(|| {
        WorkspaceError::unresolved_library_environment("No target path was provided.")
    })?;
    let first = normalize_existing_path(first)?;
    let ceiling = match workspace_root {
        Some(root) => normalize_existing_path(root)?,
        None if first.is_dir() => first.clone(),
        None => first.parent().map(Path::to_path_buf).ok_or_else(|| {
            WorkspaceError::unresolved_library_environment(format!(
                "Could not infer a workspace root from target file {}.",
                first.display()
            ))
        })?,
    };
    let boundary =
        sysml_query::source::discover_project_boundary(&first, &ceiling).ok_or_else(|| {
            WorkspaceError::unresolved_library_environment(format!(
                "Target {} is outside workspace root {}.",
                first.display(),
                ceiling.display()
            ))
        })?;
    let root = boundary.project_root().to_path_buf();
    for target in targets.iter().skip(1) {
        let target = normalize_existing_path(target)?;
        let target_boundary = sysml_query::source::discover_project_boundary(&target, &ceiling)
            .ok_or_else(|| {
                WorkspaceError::unresolved_library_environment(format!(
                    "Target {} is outside workspace root {}.",
                    target.display(),
                    ceiling.display()
                ))
            })?;
        if target_boundary.project_root() != root {
            return Err(WorkspaceError::unresolved_library_environment(format!(
                "Targets belong to different SysML projects: {} and {}.",
                root.display(),
                target_boundary.project_root().display()
            )));
        }
    }
    Ok(root)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_workspace_is_a_ceiling_for_nearest_project_discovery() {
        let temp = tempfile::tempdir().unwrap();
        let repo = temp.path();
        let project = repo.join("models/vehicle");
        std::fs::create_dir_all(project.join("src")).unwrap();
        std::fs::write(project.join(".project.json"), "{}").unwrap();
        let source = project.join("src/Vehicle.sysml");
        std::fs::write(&source, "package Vehicle;").unwrap();

        assert_eq!(
            resolve_workspace_root(&[source], Some(repo)).unwrap(),
            std::fs::canonicalize(project).unwrap()
        );
    }

    #[test]
    fn targets_from_distinct_manifest_projects_are_rejected() {
        let temp = tempfile::tempdir().unwrap();
        let mut targets = Vec::new();
        for name in ["a", "b"] {
            let project = temp.path().join(name);
            std::fs::create_dir(&project).unwrap();
            std::fs::write(project.join(".project.json"), "{}").unwrap();
            let source = project.join("Model.sysml");
            std::fs::write(&source, "package Model;").unwrap();
            targets.push(source);
        }
        let error = resolve_workspace_root(&targets, Some(temp.path())).unwrap_err();
        assert!(error.to_string().contains("different SysML projects"));
    }
}
