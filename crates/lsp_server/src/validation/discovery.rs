use std::path::{Path, PathBuf};

use tower_lsp::lsp_types::Url;

use crate::common::util;
use workspace::snapshot::discovery;

use super::ValidationRequest;

pub(super) fn resolve_workspace_root(
    request: &ValidationRequest,
) -> Result<Option<PathBuf>, String> {
    let workspace_root = request.workspace_root.as_deref();
    workspace::snapshot::discovery::resolve_workspace_root(&request.targets, workspace_root)
        .map(Some)
        .map_err(|e| e.to_string())
}

pub(super) fn discover_target_files(targets: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    // Allow empty result (unlike workspace's strict version) — callers handle the empty case.
    match discovery::discover_target_files(targets) {
        Ok(files) => Ok(files),
        Err(e) if e.to_string().contains("No .sysml") => Ok(vec![]),
        Err(e) => Err(e.to_string()),
    }
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
            strict_diagnostics: false,
        };
        let resolved = resolve_workspace_root(&request)
            .expect("resolve")
            .expect("workspace root");
        assert_eq!(resolved, root.canonicalize().expect("canonical root"));
    }
}
