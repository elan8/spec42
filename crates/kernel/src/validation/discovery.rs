use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use tower_lsp::lsp_types::Url;
use walkdir::WalkDir;

use crate::common::util;

use super::ValidationRequest;

pub(super) fn resolve_workspace_root(
    request: &ValidationRequest,
) -> Result<Option<PathBuf>, String> {
    if let Some(root) = &request.workspace_root {
        return normalize_existing_path(root).map(Some);
    }
    let first = request
        .targets
        .first()
        .ok_or_else(|| "No target path was provided.".to_string())?;
    if first.is_dir() {
        return normalize_existing_path(first).map(Some);
    }
    normalize_existing_path(first)?
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            format!(
                "Could not infer a workspace root from target file {}.",
                first.display()
            )
        })
        .map(Some)
}

pub(super) fn discover_target_files(targets: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
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
            if !entry.file_type().is_file() {
                continue;
            }
            let entry_path = entry.path().to_path_buf();
            if is_sysml_like(&entry_path) {
                files.insert(entry_path);
            }
        }
    }
    Ok(files.into_iter().collect())
}

fn normalize_existing_path(path: &Path) -> Result<PathBuf, String> {
    let path = path
        .canonicalize()
        .map_err(|err| format!("Failed to resolve {}: {err}", path.display()))?;
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    Ok(path)
}

fn is_sysml_like(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("sysml") | Some("kerml")
    )
}

pub(super) fn path_to_file_url(path: &Path) -> Result<Url, String> {
    Url::from_file_path(path)
        .map(|uri| util::normalize_file_uri(&uri))
        .map_err(|_| format!("Could not convert {} to file:// URL.", path.display()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{discover_target_files, resolve_workspace_root};
    use crate::validation::ValidationRequest;

    #[test]
    fn discover_target_files_keeps_only_sysml_like_files() {
        let temp = tempfile::tempdir().expect("temp dir");
        let target_dir = temp.path().join("models");
        std::fs::create_dir_all(&target_dir).expect("create models dir");
        let sysml = target_dir.join("a.sysml");
        let kerml = target_dir.join("b.kerml");
        let txt = target_dir.join("ignore.txt");
        std::fs::write(&sysml, "package A {}").expect("write sysml");
        std::fs::write(&kerml, "package B {}").expect("write kerml");
        std::fs::write(&txt, "text").expect("write txt");

        let files = discover_target_files(&[target_dir]).expect("discover");
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|file| file.ends_with("a.sysml")));
        assert!(files.iter().any(|file| file.ends_with("b.kerml")));
    }

    #[test]
    fn resolve_workspace_root_uses_parent_for_file_target() {
        let temp = tempfile::tempdir().expect("temp dir");
        let root = temp.path().join("workspace");
        std::fs::create_dir_all(&root).expect("create workspace dir");
        let target = root.join("model.sysml");
        std::fs::write(&target, "package P {}").expect("write model");
        let request = ValidationRequest {
            targets: vec![target],
            workspace_root: None,
            library_paths: Vec::<PathBuf>::new(),
            parallel_enabled: false,
        };
        let resolved = resolve_workspace_root(&request)
            .expect("resolve")
            .expect("workspace root");
        assert_eq!(resolved, root.canonicalize().expect("canonical root"));
    }
}
