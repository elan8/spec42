use std::collections::HashSet;

use sysml_model::{
    resolve_member_via_type, resolve_type_reference_targets, ElementKind, NodeId, ResolveResult,
    TextPosition, TextRange,
};
use url::Url;

use crate::dto::{DefinitionResult, ReferencesResult, SourceLocation};
use crate::keywords::is_reserved_keyword;
use crate::lookup::collect_symbol_matches_for_lookup;
use crate::symbol::{
    find_reference_ranges, location_node_id, symbol_entry_node_id, SymbolEntry,
};
use crate::text::word_at_position;
use crate::workspace::WorkspaceSnapshot;

pub const TYPE_LOOKUP_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "use case def",
    "concern def",
    "kermlDecl",
];

/// [`ElementKind`] equivalent of [`TYPE_LOOKUP_KINDS`], for call sites that take the
/// type-safe allowed-kinds slice (e.g. `resolve_type_reference_targets`).
pub const TYPE_LOOKUP_ELEMENT_KINDS: &[ElementKind] = &[
    ElementKind::PartDef,
    ElementKind::PortDef,
    ElementKind::Interface,
    ElementKind::ItemDef,
    ElementKind::AttributeDef,
    ElementKind::ActionDef,
    ElementKind::ActorDef,
    ElementKind::OccurrenceDef,
    ElementKind::FlowDef,
    ElementKind::AllocationDef,
    ElementKind::StateDef,
    ElementKind::RequirementDef,
    ElementKind::UseCaseDef,
    ElementKind::ConcernDef,
    ElementKind::KermlDecl,
];

#[derive(Debug, Clone)]
pub struct ResolvedSymbolTarget {
    pub target_id: NodeId,
    pub name: String,
    pub definition_location: SourceLocation,
    pub identifier_range: TextRange,
}

type LocationKey = (String, u32, u32, u32, u32);

fn position_in_range(pos: TextPosition, range: TextRange) -> bool {
    (pos.line > range.start.line
        || (pos.line == range.start.line && pos.character >= range.start.character))
        && (pos.line < range.end.line
            || (pos.line == range.end.line && pos.character <= range.end.character))
}

pub fn find_references_at_position(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
    include_declaration: bool,
) -> ReferencesResult {
    let uri = match workspace.resolve_uri_for_path(path) {
        Some(uri) => workspace.normalize_uri(&uri),
        None => return ReferencesResult {
            locations: Vec::new(),
        },
    };
    let pos = position;
    let Some(mut target) = resolve_symbol_target_at_position(workspace, &uri, pos) else {
        return ReferencesResult {
            locations: Vec::new(),
        };
    };
    if let Some(def_loc) = goto_definition_at_position(workspace, path, position)
        .locations
        .into_iter()
        .next()
    {
        if let Some(def_uri) = workspace.resolve_uri_for_path(&def_loc.path) {
            if let Some(def_id) = location_node_id(
                workspace.semantic_graph(),
                &workspace.normalize_uri(&def_uri),
                &target.name,
                def_loc.range,
            ) {
                target.target_id = def_id;
                target.definition_location = def_loc;
            }
        }
    }
    let mut locations = collect_references_for_lookup(
        workspace,
        &uri,
        &target.name,
        vec![target.definition_location.clone()],
        Some(std::iter::once(target.target_id.clone()).collect()),
        include_declaration,
    );
    if !include_declaration {
        let def = target.definition_location.clone();
        locations.retain(|loc| !reference_is_declaration_site(&def, loc));
    }
    ReferencesResult { locations }
}

fn reference_is_declaration_site(definition: &SourceLocation, reference: &SourceLocation) -> bool {
    definition.path == reference.path
        && definition.range.start.line == reference.range.start.line
        && reference.range.start.character >= definition.range.start.character
        && reference.range.end.character <= definition.range.end.character
}

fn is_definition_symbol(entry: &SymbolEntry) -> bool {
    entry.detail.as_deref().is_some_and(|kind| {
        TYPE_LOOKUP_KINDS.contains(&kind) || kind.ends_with(" def")
    })
}

fn definition_locations_from_entries(
    workspace: &impl WorkspaceSnapshot,
    entries: &[&SymbolEntry],
) -> Vec<SourceLocation> {
    entries
        .iter()
        .filter(|entry| is_definition_symbol(entry))
        .map(|entry| SourceLocation {
            path: workspace.path_for_uri(&entry.uri),
            range: entry.range,
        })
        .collect()
}

pub fn goto_definition_at_position(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> DefinitionResult {
    let uri = match workspace.resolve_uri_for_path(path) {
        Some(uri) => workspace.normalize_uri(&uri),
        None => return DefinitionResult {
            locations: Vec::new(),
        },
    };
    let text = match workspace.document_text(&uri) {
        Some(text) => text,
        None => {
            return DefinitionResult {
                locations: Vec::new(),
            }
        }
    };
    let (_, _, _, word) = match word_at_position(text, position.line, position.character) {
        Some(parts) => parts,
        None => {
            return DefinitionResult {
                locations: Vec::new(),
            }
        }
    };
    let lookup_name = word
        .rsplit("::")
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| word.clone());
    let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());

    if is_reserved_keyword(&word) || is_reserved_keyword(&lookup_name) {
        return DefinitionResult {
            locations: Vec::new(),
        };
    }

    let graph = workspace.semantic_graph();
    if let Some(node) = graph.find_node_at_position(&uri, position) {
        for target in graph.outgoing_typing_or_specializes_targets(node) {
            if target.name == lookup_name
                || target
                    .id
                    .qualified_name
                    .ends_with(&format!("::{}", lookup_name))
            {
                return DefinitionResult {
                    locations: vec![SourceLocation {
                        path: workspace.path_for_uri(&target.id.uri),
                        range: target.range,
                    }],
                };
            }
        }
        if word != node.name {
            if let Some(target) = resolve_type_reference_targets(
                graph,
                node,
                &word,
                TYPE_LOOKUP_ELEMENT_KINDS,
            )
            .into_iter()
            .find_map(|target_id| graph.get_node(&target_id))
            .filter(|target| {
                target.name == lookup_name
                    || target
                        .id
                        .qualified_name
                        .ends_with(&format!("::{}", lookup_name))
            })
            {
                return DefinitionResult {
                    locations: vec![SourceLocation {
                        path: workspace.path_for_uri(&target.id.uri),
                        range: target.range,
                    }],
                };
            }
        }
    }

    let (same_file_matches, other_file_matches) =
        collect_symbol_matches_for_lookup(workspace, &uri, &lookup_name, qualifier.as_deref());
    let same_file_definitions =
        definition_locations_from_entries(workspace, &same_file_matches);
    let other_file_definitions =
        definition_locations_from_entries(workspace, &other_file_matches);
    let same_file: Vec<SourceLocation> = same_file_matches
        .into_iter()
        .map(|entry| SourceLocation {
            path: workspace.path_for_uri(&entry.uri),
            range: entry.range,
        })
        .collect();
    let other_files: Vec<SourceLocation> = other_file_matches
        .into_iter()
        .map(|entry| SourceLocation {
            path: workspace.path_for_uri(&entry.uri),
            range: entry.range,
        })
        .collect();
    let locations = if same_file_definitions.len() == 1 {
        same_file_definitions
    } else if other_file_definitions.len() == 1 {
        other_file_definitions
    } else if same_file.is_empty() {
        other_files
    } else {
        same_file
    };
    DefinitionResult { locations }
}

pub fn resolve_symbol_target_at_position(
    workspace: &impl WorkspaceSnapshot,
    uri_norm: &Url,
    pos: TextPosition,
) -> Option<ResolvedSymbolTarget> {
    let text = workspace.document_text(uri_norm)?;
    let (line, char_start, char_end, word) = word_at_position(text, pos.line, pos.character)?;
    if is_non_code_position(text, line, char_start) {
        return None;
    }
    let lookup_name = word
        .rsplit("::")
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| word.clone());
    let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());
    if is_reserved_keyword(&word) || is_reserved_keyword(&lookup_name) {
        return None;
    }

    let graph = workspace.semantic_graph();

    if let Some(node) = graph.find_node_at_position(uri_norm, pos) {
        for target in graph.outgoing_typing_or_specializes_targets(node) {
            if target.name == lookup_name
                || target
                    .id
                    .qualified_name
                    .ends_with(&format!("::{}", lookup_name))
            {
                return Some(ResolvedSymbolTarget {
                    target_id: target.id.clone(),
                    name: lookup_name.clone(),
                    definition_location: SourceLocation {
                        path: workspace.path_for_uri(&target.id.uri),
                        range: target.range,
                    },
                    identifier_range: TextRange {
                        start: TextPosition {
                            line,
                            character: char_start,
                        },
                        end: TextPosition {
                            line,
                            character: char_end,
                        },
                    },
                });
            }
        }
        if word != node.name {
            if let Some(target) = resolve_type_reference_targets(
                graph,
                node,
                &word,
                TYPE_LOOKUP_ELEMENT_KINDS,
            )
            .into_iter()
            .find_map(|target_id| graph.get_node(&target_id))
            .filter(|target| {
                target.name == lookup_name
                    || target
                        .id
                        .qualified_name
                        .ends_with(&format!("::{}", lookup_name))
            })
            {
                return Some(ResolvedSymbolTarget {
                    target_id: target.id.clone(),
                    name: lookup_name.clone(),
                    definition_location: SourceLocation {
                        path: workspace.path_for_uri(&target.id.uri),
                        range: target.range,
                    },
                    identifier_range: TextRange {
                        start: TextPosition {
                            line,
                            character: char_start,
                        },
                        end: TextPosition {
                            line,
                            character: char_end,
                        },
                    },
                });
            }
        }
    }

    let selected_defs =
        select_defs_for_position(workspace, uri_norm, &lookup_name, qualifier.as_deref(), pos);
    let explicit_target_ids: HashSet<NodeId> = graph
        .nodes_for_uri(uri_norm)
        .into_iter()
        .filter(|node| node.name == lookup_name && position_in_range(pos, node.range))
        .map(|node| node.id.clone())
        .collect();

    let from_symbol_ids: HashSet<NodeId> = selected_defs
        .iter()
        .filter_map(|entry| symbol_entry_node_id(graph, entry))
        .collect();

    let mut target_ids: HashSet<NodeId> = if explicit_target_ids.is_empty() {
        from_symbol_ids
    } else if from_symbol_ids.len() == 1 {
        // Prefer unambiguous symbol-table definition over same-site usage/reference nodes.
        from_symbol_ids
    } else {
        explicit_target_ids
    };
    let same_uri_target_ids: HashSet<NodeId> = target_ids
        .iter()
        .filter(|id| workspace.normalize_uri(&id.uri) == *uri_norm)
        .cloned()
        .collect();
    if !same_uri_target_ids.is_empty() {
        target_ids = same_uri_target_ids;
    }
    if target_ids.len() != 1 {
        return None;
    }
    let target_id = target_ids.into_iter().next()?;
    let target_node = graph.get_node(&target_id)?;
    let definition_location = selected_defs
        .iter()
        .find(|entry| symbol_entry_node_id(graph, entry).as_ref() == Some(&target_id))
        .map(|entry| SourceLocation {
            path: workspace.path_for_uri(&entry.uri),
            range: entry.range,
        })
        .unwrap_or_else(|| SourceLocation {
            path: workspace.path_for_uri(&target_node.id.uri),
            range: target_node.range,
        });

    Some(ResolvedSymbolTarget {
        target_id,
        name: lookup_name,
        definition_location,
        identifier_range: TextRange {
            start: TextPosition {
                line,
                character: char_start,
            },
            end: TextPosition {
                line,
                character: char_end,
            },
        },
    })
}

fn collect_references_for_lookup(
    workspace: &impl WorkspaceSnapshot,
    query_uri: &Url,
    lookup_name: &str,
    selected_defs: Vec<SourceLocation>,
    explicit_target_ids: Option<HashSet<NodeId>>,
    include_declaration: bool,
) -> Vec<SourceLocation> {
    let graph = workspace.semantic_graph();
    let mut target_ids: HashSet<NodeId> = explicit_target_ids.unwrap_or_else(|| {
        selected_defs
            .iter()
            .filter_map(|location| {
                workspace
                    .resolve_uri_for_path(&location.path)
                    .and_then(|uri| {
                        location_node_id(graph, &workspace.normalize_uri(&uri), lookup_name, location.range)
                    })
            })
            .collect()
    });
    let same_uri_target_ids: HashSet<NodeId> = target_ids
        .iter()
        .filter(|id| workspace.normalize_uri(&id.uri) == *query_uri)
        .cloned()
        .collect();
    if !same_uri_target_ids.is_empty() {
        target_ids = same_uri_target_ids;
    }
    if target_ids.is_empty() {
        return Vec::new();
    }
    let def_locations: HashSet<LocationKey> = selected_defs
        .into_iter()
        .map(|loc| location_key_for_location(&loc))
        .collect();

    let mut locations: Vec<SourceLocation> = Vec::new();
    for uri in workspace.index_uris() {
        let uri = workspace.normalize_uri(&uri);
        let Some(text) = workspace.document_text(&uri) else {
            continue;
        };
        for range in find_reference_ranges(text, lookup_name) {
            if is_non_code_position(text, range.start.line, range.start.character) {
                continue;
            }
            let location = SourceLocation {
                path: workspace.path_for_uri(&uri),
                range,
            };
            let semantic_candidate_ids: HashSet<NodeId> = graph
                .nodes_for_uri(&uri)
                .into_iter()
                .filter(|node| {
                    node.name == lookup_name && position_in_range(location.range.start, node.range)
                })
                .map(|node| node.id.clone())
                .collect();
            let (candidate_same, candidate_other) =
                collect_symbol_matches_for_lookup(workspace, &uri, lookup_name, None);
            let candidate_ids: HashSet<NodeId> = if !semantic_candidate_ids.is_empty() {
                semantic_candidate_ids
            } else {
                let candidate_defs = if candidate_same.len() <= 1 {
                    candidate_same
                } else {
                    select_defs_for_position(
                        workspace,
                        &uri,
                        lookup_name,
                        None,
                        location.range.start,
                    )
                };
                if !candidate_defs.is_empty() {
                    candidate_defs
                        .iter()
                        .filter_map(|entry| symbol_entry_node_id(graph, entry))
                        .collect()
                } else {
                    let other_ids: HashSet<NodeId> = candidate_other
                        .iter()
                        .filter_map(|entry| symbol_entry_node_id(graph, entry))
                        .collect();
                    if other_ids.len() == 1 {
                        other_ids
                    } else {
                        HashSet::new()
                    }
                }
            };
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
    workspace: &'a impl WorkspaceSnapshot,
    uri_norm: &Url,
    lookup_name: &str,
    qualifier: Option<&str>,
    pos: TextPosition,
) -> Vec<&'a SymbolEntry> {
    let (same_file_defs, other_file_defs) =
        collect_symbol_matches_for_lookup(workspace, uri_norm, lookup_name, qualifier);
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
            resolve_owner_member_defs(workspace, uri_norm, lookup_name, pos, &same_file_defs)
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
            workspace,
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
        let local_definitions = positional_same_file_defs
            .iter()
            .any(|entry| is_definition_symbol(entry));
        if !local_definitions && other_file_defs.len() == 1 {
            other_file_defs
        } else {
            positional_same_file_defs
        }
    }
}

fn dotted_owner_at_position(
    workspace: &impl WorkspaceSnapshot,
    uri: &Url,
    lookup_name: &str,
    pos: TextPosition,
) -> Option<String> {
    let content = workspace.document_text(uri)?;
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
    workspace: &'a impl WorkspaceSnapshot,
    uri: &Url,
    lookup_name: &str,
    pos: TextPosition,
    candidates: &[&'a SymbolEntry],
) -> Option<Vec<&'a SymbolEntry>> {
    let owner_ident = dotted_owner_at_position(workspace, uri, lookup_name, pos)?;
    let graph = workspace.semantic_graph();
    let owner_node = graph
        .find_deepest_node_at_position(uri, pos)
        .or_else(|| {
            graph
                .nodes_for_uri(uri)
                .into_iter()
                .find(|n| n.name == owner_ident)
        })?;
    let resolved = resolve_member_via_type(graph, owner_node, lookup_name);
    let resolved_id = match resolved {
        ResolveResult::Resolved(id) => id,
        ResolveResult::Ambiguous | ResolveResult::Unresolved => return None,
    };
    let filtered: Vec<&SymbolEntry> = candidates
        .iter()
        .copied()
        .filter(|entry| symbol_entry_node_id(graph, entry).as_ref() == Some(&resolved_id))
        .collect();
    if filtered.is_empty() {
        None
    } else {
        Some(filtered)
    }
}

fn location_key_for_location(loc: &SourceLocation) -> LocationKey {
    (
        loc.path.clone(),
        loc.range.start.line,
        loc.range.start.character,
        loc.range.end.line,
        loc.range.end.character,
    )
}

fn is_non_code_position(source: &str, line_no: u32, character: u32) -> bool {
    let line = match source.lines().nth(line_no as usize) {
        Some(line) => line,
        None => return true,
    };
    let mut in_string = false;
    let mut escaped = false;
    for (idx, ch) in line.chars().enumerate() {
        if idx as u32 >= character {
            break;
        }
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_string {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if !in_string && ch == '/' {
            let next_is_slash = line.chars().nth(idx + 1) == Some('/');
            if next_is_slash {
                return true;
            }
        }
    }
    in_string
}
