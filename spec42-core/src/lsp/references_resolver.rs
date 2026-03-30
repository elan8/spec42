use crate::language::{find_reference_ranges, is_reserved_keyword, word_at_position, SymbolEntry};
use crate::lsp::types::ServerState;
use crate::semantic_model::NodeId;
use crate::semantic_model::ResolveResult;
use tower_lsp::lsp_types::{Location, Position, Url};

type LocationKey = (String, u32, u32, u32, u32);

pub(crate) fn symbol_name_position(state: &ServerState, symbol: &SymbolEntry) -> Option<Position> {
    let content = state.index.get(&symbol.uri)?.content.as_str();
    let line_text = content.lines().nth(symbol.range.start.line as usize)?;
    let start_char = symbol.range.start.character as usize;
    let line_chars: Vec<char> = line_text.chars().collect();
    if start_char >= line_chars.len() {
        return None;
    }
    let tail: String = line_chars[start_char..].iter().collect();
    let rel = tail.find(&symbol.name)?;
    let rel_char = tail[..rel].chars().count() as u32;
    Some(Position::new(
        symbol.range.start.line,
        symbol.range.start.character + rel_char,
    ))
}

pub(crate) fn resolved_references_for_symbol(
    state: &ServerState,
    symbol: &SymbolEntry,
    include_declaration: bool,
) -> Vec<Location> {
    let selected_defs = vec![symbol];
    collect_references_for_lookup(state, &symbol.name, selected_defs, include_declaration)
}

pub(crate) fn resolved_references_at_position(
    state: &ServerState,
    uri_norm: &Url,
    pos: Position,
    include_declaration: bool,
) -> Option<Vec<Location>> {
    let text = state.index.get(uri_norm).map(|e| e.content.as_str())?;
    let (_, _, _, word) = word_at_position(text, pos.line, pos.character)?;
    let lookup_name = word
        .rsplit("::")
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| word.clone());
    let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());
    if is_reserved_keyword(&word) || is_reserved_keyword(&lookup_name) {
        return Some(Vec::new());
    }

    let selected_defs =
        select_defs_for_position(state, uri_norm, &lookup_name, qualifier.as_deref(), pos);

    Some(collect_references_for_lookup(
        state,
        &lookup_name,
        selected_defs,
        include_declaration,
    ))
}

fn collect_references_for_lookup(
    state: &ServerState,
    lookup_name: &str,
    selected_defs: Vec<&SymbolEntry>,
    include_declaration: bool,
) -> Vec<Location> {
    let target_ids: std::collections::HashSet<NodeId> = selected_defs
        .iter()
        .filter_map(|entry| symbol_entry_node_id(state, entry))
        .collect();
    // Strict mode: if target cannot resolve to FQN, we return no references.
    if target_ids.is_empty() {
        return Vec::new();
    }
    let def_locations: std::collections::HashSet<LocationKey> = selected_defs
        .into_iter()
        .map(location_key_for_symbol)
        .collect();

    let mut locations: Vec<Location> = Vec::new();
    for (uri, entry) in state.index.iter() {
        for range in find_reference_ranges(&entry.content, lookup_name) {
            let location = Location {
                uri: uri.clone(),
                range,
            };
            let (candidate_same, candidate_other) =
                collect_symbol_matches_for_lookup(state, uri, lookup_name, None);
            let candidate_defs = if candidate_same.len() <= 1 {
                candidate_same
            } else {
                select_defs_for_position(state, uri, lookup_name, None, location.range.start)
            };
            let candidate_ids: std::collections::HashSet<NodeId> = if !candidate_defs.is_empty() {
                candidate_defs
                    .iter()
                    .filter_map(|entry| symbol_entry_node_id(state, entry))
                    .collect()
            } else {
                let other_ids: std::collections::HashSet<NodeId> = candidate_other
                    .iter()
                    .filter_map(|entry| symbol_entry_node_id(state, entry))
                    .collect();
                if other_ids.len() == 1 {
                    other_ids
                } else {
                    std::collections::HashSet::new()
                }
            };
            let candidate_ids: std::collections::HashSet<NodeId> =
                candidate_ids.iter().cloned().collect();
            let resolved_matches_target =
                !candidate_ids.is_empty() && candidate_ids.iter().any(|id| target_ids.contains(id));
            if resolved_matches_target {
                locations.push(location);
            }
        }
    }

    if include_declaration {
        locations
    } else {
        locations
            .into_iter()
            .filter(|loc| !def_locations.contains(&location_key_for_location(loc)))
            .collect()
    }
}

fn select_defs_for_position<'a>(
    state: &'a ServerState,
    uri_norm: &Url,
    lookup_name: &str,
    qualifier: Option<&str>,
    pos: Position,
) -> Vec<&'a SymbolEntry> {
    let (same_file_defs, other_file_defs) =
        collect_symbol_matches_for_lookup(state, uri_norm, lookup_name, qualifier);
    let mut positional_same_file_defs: Vec<&SymbolEntry> = same_file_defs
        .iter()
        .copied()
        .filter(|entry| {
            let r = entry.range;
            (pos.line > r.start.line
                || (pos.line == r.start.line && pos.character >= r.start.character))
                && (pos.line < r.end.line
                    || (pos.line == r.end.line && pos.character <= r.end.character))
        })
        .collect();
    if positional_same_file_defs.is_empty() {
        if let Some(owner_member_defs) =
            resolve_owner_member_defs(state, uri_norm, lookup_name, pos, &same_file_defs)
        {
            return owner_member_defs;
        }
        let same_line: Vec<&SymbolEntry> = same_file_defs
            .iter()
            .copied()
            .filter(|entry| entry.range.start.line == pos.line)
            .collect();
        if let Some(best) = same_line.into_iter().min_by_key(|entry| {
            let start_dist = pos.character.abs_diff(entry.range.start.character);
            let end_dist = pos.character.abs_diff(entry.range.end.character);
            start_dist.min(end_dist)
        }) {
            positional_same_file_defs.push(best);
        }
    }
    if positional_same_file_defs.len() > 1 {
        if let Some(owner_member_defs) = resolve_owner_member_defs(
            state,
            uri_norm,
            lookup_name,
            pos,
            &positional_same_file_defs,
        ) {
            positional_same_file_defs = owner_member_defs;
        }
    }
    if positional_same_file_defs.is_empty() {
        if same_file_defs.is_empty() {
            other_file_defs
        } else {
            same_file_defs
        }
    } else {
        positional_same_file_defs
    }
}

fn dotted_owner_at_position(
    state: &ServerState,
    uri: &Url,
    lookup_name: &str,
    pos: Position,
) -> Option<String> {
    let content = state.index.get(uri).map(|e| e.content.as_str())?;
    let line = content.lines().nth(pos.line as usize)?;
    let line_chars: Vec<char> = line.chars().collect();
    let pos_char = pos.character as usize;
    if pos_char > line_chars.len() {
        return None;
    }
    let prefix: String = line_chars[..pos_char].iter().collect();
    let mut owner = String::new();
    let mut seen_dot = false;
    for ch in prefix.chars().rev() {
        if !seen_dot {
            if ch.is_whitespace() {
                continue;
            }
            if ch == '.' {
                seen_dot = true;
                continue;
            }
            // Not a dotted access for this token.
            return None;
        }
        if ch.is_alphanumeric() || ch == '_' || ch == '-' {
            owner.push(ch);
            continue;
        }
        break;
    }
    if !seen_dot || owner.is_empty() {
        return None;
    }
    let owner_ident: String = owner.chars().rev().collect();
    if owner_ident == lookup_name {
        return None;
    }
    Some(owner_ident)
}

fn resolve_owner_member_defs<'a>(
    state: &ServerState,
    uri: &Url,
    lookup_name: &str,
    pos: Position,
    candidates: &[&'a SymbolEntry],
) -> Option<Vec<&'a SymbolEntry>> {
    let owner_ident = dotted_owner_at_position(state, uri, lookup_name, pos)?;
    let owner_node = state
        .semantic_graph
        .find_deepest_node_at_position(uri, pos)
        .or_else(|| {
            state
                .semantic_graph
                .nodes_for_uri(uri)
                .into_iter()
                .find(|n| n.name == owner_ident)
        })?;
    let resolved = crate::semantic_model::resolve_member_via_type(
        &state.semantic_graph,
        owner_node,
        lookup_name,
    );
    let resolved_id = match resolved {
        ResolveResult::Resolved(id) => id,
        ResolveResult::Ambiguous => return None,
        ResolveResult::Unresolved => return None,
    };
    let filtered: Vec<&SymbolEntry> = candidates
        .iter()
        .copied()
        .filter(|entry| symbol_entry_node_id(state, entry).as_ref() == Some(&resolved_id))
        .collect();
    if filtered.is_empty() {
        None
    } else {
        Some(filtered)
    }
}

fn location_key_for_symbol(entry: &SymbolEntry) -> LocationKey {
    (
        entry.uri.to_string(),
        entry.range.start.line,
        entry.range.start.character,
        entry.range.end.line,
        entry.range.end.character,
    )
}

fn location_key_for_location(loc: &Location) -> LocationKey {
    (
        loc.uri.to_string(),
        loc.range.start.line,
        loc.range.start.character,
        loc.range.end.line,
        loc.range.end.character,
    )
}

fn collect_symbol_matches_for_lookup<'a>(
    state: &'a ServerState,
    uri_norm: &Url,
    lookup_name: &str,
    qualifier: Option<&str>,
) -> (Vec<&'a SymbolEntry>, Vec<&'a SymbolEntry>) {
    let mut same_file = Vec::new();
    let mut other_files = Vec::new();
    for entry in state.symbol_table.iter() {
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
                .map(|c| {
                    let c_lc = c.to_ascii_lowercase();
                    c_lc == q_lc || c_lc.ends_with(&format!("::{}", q_lc))
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

fn symbol_entry_node_id(state: &ServerState, entry: &SymbolEntry) -> Option<NodeId> {
    state
        .semantic_graph
        .nodes_for_uri(&entry.uri)
        .into_iter()
        .find(|node| node.name == entry.name && node.range == entry.range)
        .map(|node| node.id.clone())
}
