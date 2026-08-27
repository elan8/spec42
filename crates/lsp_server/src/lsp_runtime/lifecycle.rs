use tower_lsp::lsp_types::{InitializeParams, Url};

pub(crate) fn workspace_roots_from_initialize(params: &InitializeParams) -> Vec<Url> {
    params
        .workspace_folders
        .as_ref()
        .filter(|f| !f.is_empty())
        .map(|folders| folders.iter().map(|f| f.uri.clone()).collect())
        .or_else(|| params.root_uri.as_ref().map(|u| vec![u.clone()]))
        .unwrap_or_default()
}

pub(crate) fn scan_roots(workspace_roots: &[Url], library_paths: &[Url]) -> Vec<Url> {
    workspace_roots
        .iter()
        .cloned()
        .chain(library_paths.iter().cloned())
        .collect()
}

/// Resolves an editor document to its nearest manifest project, bounded by the most specific
/// containing workspace folder. The folder is the manifestless fallback, never semantic proof
/// that every source below it belongs to one project.
pub(crate) fn project_boundary_for_uri(
    document: &Url,
    workspace_roots: &[Url],
) -> Option<sysml_query::source::ProjectBoundary> {
    let document = document.to_file_path().ok()?;
    let ceiling = workspace_roots
        .iter()
        .filter_map(|root| root.to_file_path().ok())
        .filter(|root| document.starts_with(root))
        .max_by_key(|root| root.components().count())?;
    sysml_query::source::discover_project_boundary(&document, &ceiling)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn open_file_uses_nearest_project_inside_editor_root() {
        let temp = tempfile::tempdir().unwrap();
        let repo = temp.path();
        let project = repo.join("models/a");
        fs::create_dir_all(project.join("src")).unwrap();
        fs::write(project.join(".project.json"), "{}").unwrap();
        let source = project.join("src/A.sysml");
        fs::write(&source, "package A;").unwrap();
        let document = Url::from_file_path(source).unwrap();
        let editor_root = Url::from_directory_path(repo).unwrap();

        let boundary = project_boundary_for_uri(&document, &[editor_root]).unwrap();
        assert_eq!(boundary.project_root(), fs::canonicalize(project).unwrap());
    }

    #[test]
    fn most_specific_workspace_folder_is_the_fallback_ceiling() {
        let temp = tempfile::tempdir().unwrap();
        let nested = temp.path().join("nested");
        fs::create_dir_all(&nested).unwrap();
        fs::write(temp.path().join(".project.json"), "{}").unwrap();
        let source = nested.join("Scratch.sysml");
        fs::write(&source, "package Scratch;").unwrap();

        let boundary = project_boundary_for_uri(
            &Url::from_file_path(source).unwrap(),
            &[
                Url::from_directory_path(temp.path()).unwrap(),
                Url::from_directory_path(&nested).unwrap(),
            ],
        )
        .unwrap();
        assert!(boundary.is_manifestless());
        assert_eq!(boundary.project_root(), fs::canonicalize(nested).unwrap());
    }
}
