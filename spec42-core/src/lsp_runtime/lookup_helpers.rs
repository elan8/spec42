use tower_lsp::lsp_types::Url;
use tracing::warn;

use crate::language::SymbolEntry;
use crate::util;
use crate::workspace::ServerState;

pub(super) fn collect_symbol_matches_for_lookup<'a>(
    state: &'a ServerState,
    uri_norm: &Url,
    lookup_name: &str,
    qualifier: Option<&str>,
) -> (Vec<&'a SymbolEntry>, Vec<&'a SymbolEntry>) {
    let mut same_file = Vec::new();
    let mut other_files = Vec::new();
    for entry in &state.symbol_table {
        if !symbol_matches_definition_lookup(
            &entry.name,
            entry.container_name.as_deref(),
            entry.uri.path(),
            lookup_name,
            qualifier,
        ) {
            continue;
        }
        if entry.uri == *uri_norm {
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

pub(super) fn debug_qualified_lookup_context(
    state: &ServerState,
    lookup_name: &str,
    qualifier: &str,
    request_uri: &Url,
) {
    if lookup_name.is_empty() || qualifier.is_empty() {
        return;
    }
    let qualifier_lc = qualifier.to_ascii_lowercase();
    let needle = format!("<{}>", lookup_name);
    let mut inspected = Vec::new();
    let mut qualifier_symbol_hits = 0usize;
    for (candidate_uri, entry) in &state.index {
        let candidate_uri = util::normalize_file_uri(candidate_uri);
        if candidate_uri == *request_uri {
            continue;
        }
        if !util::uri_under_any_library(&candidate_uri, &state.library_paths) {
            continue;
        }
        let path_lc = candidate_uri.path().to_ascii_lowercase();
        let path_matches = path_lc.ends_with(&format!("/{}.sysml", qualifier_lc))
            || path_lc.ends_with(&format!("/{}.kerml", qualifier_lc));
        if !path_matches {
            continue;
        }
        let symbols_for_uri: Vec<&SymbolEntry> = state
            .symbol_table
            .iter()
            .filter(|symbol| util::normalize_file_uri(&symbol.uri) == candidate_uri)
            .collect();
        qualifier_symbol_hits += symbols_for_uri
            .iter()
            .filter(|symbol| symbol.name.eq_ignore_ascii_case(lookup_name))
            .count();
        inspected.push(format!(
            "{} symbols={} matching_name={} has_angle_short={}",
            candidate_uri.path(),
            symbols_for_uri.len(),
            symbols_for_uri
                .iter()
                .filter(|symbol| symbol.name.eq_ignore_ascii_case(lookup_name))
                .count(),
            entry.content.contains(&needle)
        ));
        if inspected.len() >= 5 {
            break;
        }
    }
    warn!(
        lookup_name = %lookup_name,
        qualifier = %qualifier,
        qualifier_symbol_hits = qualifier_symbol_hits,
        inspected = ?inspected,
        "goto_definition qualified lookup diagnostics"
    );
}
