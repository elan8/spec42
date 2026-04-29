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
