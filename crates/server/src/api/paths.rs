use std::path::{Component, Path, PathBuf};

use super::error::ApiError;

/// Resolve a workspace-relative path and reject traversal outside the workspace root.
pub fn resolve_workspace_path(workspace_root: &Path, relative: &str) -> Result<PathBuf, ApiError> {
    let trimmed = relative.trim();
    let relative_path = if trimmed.is_empty() || trimmed == "." {
        PathBuf::from(".")
    } else {
        let path = Path::new(trimmed);
        if path.is_absolute() {
            return Err(ApiError::invalid_path(format!(
                "Path must be relative to workspace root, not absolute: {trimmed}"
            )));
        }
        for component in path.components() {
            if matches!(component, Component::ParentDir) {
                return Err(ApiError::invalid_path(format!(
                    "Path must not contain '..': {trimmed}"
                )));
            }
        }
        path.to_path_buf()
    };

    let joined = workspace_root.join(&relative_path);
    let canonical_workspace = workspace_root.canonicalize().map_err(|err| {
        ApiError::invalid_path(format!(
            "Workspace root is not accessible: {} ({err})",
            workspace_root.display()
        ))
    })?;

    let canonical_joined = if joined.exists() {
        joined.canonicalize().map_err(|err| {
            ApiError::invalid_path(format!("Path is not accessible: {} ({err})", joined.display()))
        })?
    } else {
        // Allow validating paths that do not exist yet (same as CLI) while still
        // constraining the resolved parent directory inside the workspace.
        let parent = joined
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(workspace_root);
        let canonical_parent = parent.canonicalize().map_err(|err| {
            ApiError::invalid_path(format!(
                "Parent path is not accessible: {} ({err})",
                parent.display()
            ))
        })?;
        if !canonical_parent.starts_with(&canonical_workspace) {
            return Err(ApiError::invalid_path(format!(
                "Path escapes workspace root: {trimmed}"
            )));
        }
        if let Some(file_name) = joined.file_name() {
            canonical_parent.join(file_name)
        } else {
            canonical_parent
        }
    };

    if !canonical_joined.starts_with(&canonical_workspace) {
        return Err(ApiError::invalid_path(format!(
            "Path escapes workspace root: {trimmed}"
        )));
    }

    Ok(canonical_joined)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn resolve_rejects_parent_dir() {
        let workspace = TempDir::new().expect("tempdir");
        let err = resolve_workspace_path(workspace.path(), "../outside")
            .expect_err("parent dir");
        assert_eq!(err.code(), "invalid_path");
    }

    #[test]
    fn resolve_rejects_absolute_path() {
        let workspace = TempDir::new().expect("tempdir");
        #[cfg(unix)]
        let absolute = "/etc/passwd";
        #[cfg(windows)]
        let absolute = "C:\\Windows\\System32";
        let err = resolve_workspace_path(workspace.path(), absolute).expect_err("absolute");
        assert_eq!(err.code(), "invalid_path");
    }

    #[test]
    fn resolve_allows_nested_file() {
        let workspace = TempDir::new().expect("tempdir");
        let nested = workspace.path().join("models");
        fs::create_dir_all(&nested).expect("mkdir");
        let file = nested.join("demo.sysml");
        fs::write(&file, "package Demo;").expect("write");

        let resolved =
            resolve_workspace_path(workspace.path(), "models/demo.sysml").expect("resolve");
        assert_eq!(resolved, file.canonicalize().expect("canonical"));
    }
}
