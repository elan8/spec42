use url::Url;

use crate::symbol::SymbolEntry;
use crate::workspace::WorkspaceSnapshot;

pub fn collect_symbol_matches_for_lookup<'a>(
    workspace: &'a impl WorkspaceSnapshot,
    uri_norm: &Url,
    lookup_name: &str,
    qualifier: Option<&str>,
) -> (Vec<&'a SymbolEntry>, Vec<&'a SymbolEntry>) {
    let mut same_file = Vec::new();
    let mut other_files = Vec::new();
    for entry in workspace.symbol_table() {
        if !symbol_matches_definition_lookup(
            &entry.name,
            entry.container_name.as_deref(),
            entry.uri.path(),
            lookup_name,
            qualifier,
        ) {
            continue;
        }
        if workspace.normalize_uri(&entry.uri) == *uri_norm {
            same_file.push(entry);
        } else {
            other_files.push(entry);
        }
    }
    (same_file, other_files)
}

fn symbol_matches_definition_lookup(
    candidate_name: &str,
    container_name: Option<&str>,
    candidate_path: &str,
    lookup_name: &str,
    qualifier: Option<&str>,
) -> bool {
    if candidate_name != lookup_name {
        return false;
    }
    match qualifier {
        None => true,
        Some(q) => {
            let q_lc = q.to_ascii_lowercase();
            if container_name
                .map(|container| {
                    let container_lc = container.to_ascii_lowercase();
                    container_lc == q_lc || container_lc.ends_with(&format!("::{}", q_lc))
                })
                .unwrap_or(false)
            {
                return true;
            }
            let path_lc = candidate_path.to_ascii_lowercase();
            path_lc.ends_with(&format!("/{}.sysml", q_lc))
                || path_lc.ends_with(&format!("/{}.kerml", q_lc))
        }
    }
}
