use crate::dto::WorkspaceSymbolMatch;
use crate::workspace::WorkspaceSnapshot;

/// Search workspace symbols by query string (case-insensitive substring match).
pub fn search_workspace_symbols(
    workspace: &impl WorkspaceSnapshot,
    query: &str,
) -> Vec<WorkspaceSymbolMatch> {
    let query = query.to_lowercase();
    workspace
        .symbol_table()
        .iter()
        .filter(|entry| query.is_empty() || entry.name.to_lowercase().contains(&query))
        .map(|entry| WorkspaceSymbolMatch {
            name: entry.name.clone(),
            path: workspace.path_for_uri(&entry.uri),
            uri: entry.uri.to_string(),
            range: entry.range,
            container: entry.container_name.clone(),
            detail: entry.detail.clone(),
        })
        .collect()
}
