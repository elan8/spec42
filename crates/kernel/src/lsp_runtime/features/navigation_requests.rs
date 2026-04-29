use std::time::Instant;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tracing::{debug, info};

use crate::common::util;
use crate::language::{
    find_reference_ranges, is_reserved_keyword, keyword_hover_markdown, word_at_position,
};
use crate::semantic::{self, ResolveResult};
use crate::workspace::ServerState;

use super::super::lookup_helpers::{
    collect_symbol_matches_for_lookup, debug_qualified_lookup_context,
};
use super::super::{navigation, references_resolver};
use super::shared::TYPE_LOOKUP_KINDS;

fn resolve_hover_type_reference_target<'a>(
    state: &'a ServerState,
    node: &crate::semantic::SemanticNode,
    word: &str,
    lookup_name: &str,
) -> Option<&'a crate::semantic::SemanticNode> {
    let mut candidates = Vec::<String>::new();
    let mut push_candidate = |candidate: String| {
        if !candidate.is_empty() && !candidates.iter().any(|existing| existing == &candidate) {
            candidates.push(candidate);
        }
    };

    push_candidate(word.to_string());
    if lookup_name != word {
        push_candidate(lookup_name.to_string());
    }

    if word.contains("::") {
        for ancestor in state.semantic_graph.ancestors_of(node) {
            push_candidate(format!("{}::{}", ancestor.id.qualified_name, word));
        }
    }

    for candidate in candidates {
        if let Some(target_id) = semantic::resolve_type_reference_targets(
            &state.semantic_graph,
            node,
            &candidate,
            TYPE_LOOKUP_KINDS,
        )
        .into_iter()
        .next()
        {
            if let Some(target) = state.semantic_graph.get_node(&target_id) {
                return Some(target);
            }
        }
    }

    None
}

fn resolve_hover_reference_target<'a>(
    state: &'a ServerState,
    uri: &Url,
    pos: Position,
    word: &str,
) -> Option<&'a crate::semantic::SemanticNode> {
    let context_node = state
        .semantic_graph
        .find_deepest_node_at_position(uri, pos)
        .or_else(|| {
            state
                .semantic_graph
                .nodes_for_uri(uri)
                .into_iter()
                .find(|n| n.name == word)
        });

    let context_node = context_node?;

    let mut prefixes = Vec::<Option<String>>::new();
    prefixes.push(Some(context_node.id.qualified_name.clone()));
    if let Some(parent_id) = &context_node.parent_id {
        prefixes.push(Some(parent_id.qualified_name.clone()));
    }
    for ancestor in state.semantic_graph.ancestors_of(context_node) {
        prefixes.push(Some(ancestor.id.qualified_name.clone()));
    }
    prefixes.push(None);

    for prefix in prefixes {
        let resolved = semantic::resolve_expression_endpoint_strict(
            &state.semantic_graph,
            uri,
            prefix.as_deref(),
            word,
        );
        if let ResolveResult::Resolved(target_id) = resolved {
            if let Some(target) = state.semantic_graph.get_node(&target_id) {
                return Some(target);
            }
        }
    }

    None
}

pub(crate) fn hover(state: &ServerState, uri: Url, pos: Position) -> Result<Option<Hover>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (line, char_start, char_end, word) = match word_at_position(&text, pos.line, pos.character)
    {
        Some(parts) => parts,
        None => return Ok(None),
    };
    let lookup_name = word
        .rsplit("::")
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| word.clone());
    let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());
    let range = Range::new(
        Position::new(line, char_start),
        Position::new(line, char_end),
    );

    if let Some(md) = keyword_hover_markdown(&lookup_name.to_lowercase()) {
        let response = Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: md,
            }),
            range: Some(range),
        });
        log_hover_result(
            &uri_norm,
            pos,
            &lookup_name,
            started_at,
            "hover resolved via keyword docs",
        );
        return Ok(response);
    }

    if let Some(node) = state
        .semantic_graph
        .find_deepest_node_at_position(&uri_norm, pos)
    {
        let target_match = state
            .semantic_graph
            .outgoing_typing_or_specializes_targets(node)
            .into_iter()
            .find(|target| {
                target.name == lookup_name
                    || target
                        .id
                        .qualified_name
                        .ends_with(&format!("::{}", lookup_name))
            });
        let markdown = if let Some(target) = target_match.as_ref() {
            semantic::hover_markdown_for_node(
                &state.semantic_graph,
                target,
                target.id.uri != uri_norm,
            )
        } else {
            semantic::hover_markdown_for_node(
                &state.semantic_graph,
                node,
                node.id.uri != uri_norm,
            )
        };
        let markdown = if target_match.is_none() && word != node.name {
            resolve_hover_type_reference_target(state, node, &word, &lookup_name)
                .map(|target| {
                    semantic::hover_markdown_for_node(
                        &state.semantic_graph,
                        target,
                        target.id.uri != uri_norm,
                    )
                })
                .unwrap_or(markdown)
        } else {
            markdown
        };
        let response = Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown,
            }),
            range: Some(range),
        });
        log_hover_result(
            &uri_norm,
            pos,
            &lookup_name,
            started_at,
            "hover resolved via semantic graph",
        );
        return Ok(response);
    }

    if let Some(target) = resolve_hover_reference_target(state, &uri_norm, pos, &word) {
        let response = Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: semantic::hover_markdown_for_node(
                    &state.semantic_graph,
                    target,
                    target.id.uri != uri_norm,
                ),
            }),
            range: Some(range),
        });
        log_hover_result(
            &uri_norm,
            pos,
            &lookup_name,
            started_at,
            "hover resolved via context-aware reference lookup",
        );
        return Ok(response);
    }

    let (same_file, other_files) =
        collect_symbol_matches_for_lookup(state, &uri_norm, &lookup_name, qualifier.as_deref());
    let all_matches = if same_file.is_empty() {
        &other_files
    } else {
        &same_file
    };
    if let Some(entry) = all_matches.first() {
        let value = if all_matches.len() > 1 {
            let mut md = format!(
                "**{}** - {} definitions (use Go to Definition to choose):\n\n",
                lookup_name,
                all_matches.len()
            );
            for entry in all_matches {
                let kind = entry.detail.as_deref().unwrap_or("element");
                let container = entry.container_name.as_deref().unwrap_or("(top level)");
                md.push_str(&format!("- `{}` in `{}`\n", kind, container));
            }
            md.push('\n');
            md.push_str(&util::symbol_hover_markdown(entry, entry.uri != uri_norm));
            md
        } else {
            util::symbol_hover_markdown(entry, entry.uri != uri_norm)
        };
        let response = Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value,
            }),
            range: Some(range),
        });
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "kernel::lsp_runtime::features",
                event = "feature:hover",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                same_file_matches = same_file.len(),
                other_file_matches = other_files.len(),
                elapsed_ms,
                "hover resolved via symbol lookup"
            );
        }
        return Ok(response);
    }

    log_hover_result(
        &uri_norm,
        pos,
        &lookup_name,
        started_at,
        "hover completed with no result",
    );
    Ok(None)
}

fn log_hover_result(
    uri: &Url,
    pos: Position,
    lookup_name: &str,
    started_at: Instant,
    message: &str,
) {
    let elapsed_ms = started_at.elapsed().as_millis();
    if elapsed_ms >= 10 {
        info!(
            target: "kernel::lsp_runtime::features",
            event = "feature:hover",
            uri = %uri,
            line = pos.line,
            character = pos.character,
            lookup_name = %lookup_name,
            elapsed_ms,
            "{message}"
        );
    }
}

pub(crate) fn goto_definition(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<GotoDefinitionResponse>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
        Some(parts) => parts,
        None => return Ok(None),
    };
    let lookup_name = word
        .rsplit("::")
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| word.clone());
    let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());
    debug!(
        uri = %uri_norm,
        line = pos.line,
        character = pos.character,
        word = %word,
        lookup_name = %lookup_name,
        qualifier = ?qualifier,
        "goto_definition tokenized input"
    );

    if is_reserved_keyword(&word) || is_reserved_keyword(&lookup_name) {
        return Ok(None);
    }

    if let Some(node) = state.semantic_graph.find_node_at_position(&uri_norm, pos) {
        for target in state
            .semantic_graph
            .outgoing_typing_or_specializes_targets(node)
        {
            if target.name == lookup_name
                || target
                    .id
                    .qualified_name
                    .ends_with(&format!("::{}", lookup_name))
            {
                return goto_definition_response(
                    &uri_norm,
                    pos,
                    &lookup_name,
                    started_at,
                    Some(GotoDefinitionResponse::Scalar(Location {
                        uri: target.id.uri.clone(),
                        range: target.range,
                    })),
                    "goto definition resolved via semantic graph",
                );
            }
        }
        if word != node.name {
            if let Some(target) = semantic::resolve_type_reference_targets(
                &state.semantic_graph,
                node,
                &word,
                TYPE_LOOKUP_KINDS,
            )
            .into_iter()
            .find_map(|target_id| state.semantic_graph.get_node(&target_id))
            {
                return goto_definition_response(
                    &uri_norm,
                    pos,
                    &lookup_name,
                    started_at,
                    Some(GotoDefinitionResponse::Scalar(Location {
                        uri: target.id.uri.clone(),
                        range: target.range,
                    })),
                    "goto definition resolved via import-aware semantic graph",
                );
            }
        }
    }

    let (same_file_matches, other_file_matches) =
        collect_symbol_matches_for_lookup(state, &uri_norm, &lookup_name, qualifier.as_deref());
    let same_file_match_count = same_file_matches.len();
    let other_file_match_count = other_file_matches.len();
    let same_file: Vec<Location> = same_file_matches
        .into_iter()
        .map(|entry| Location {
            uri: entry.uri.clone(),
            range: entry.range,
        })
        .collect();
    let other_files: Vec<Location> = other_file_matches
        .into_iter()
        .map(|entry| Location {
            uri: entry.uri.clone(),
            range: entry.range,
        })
        .collect();
    let locations = if same_file.is_empty() {
        other_files
    } else {
        same_file
    };
    if let [location] = locations.as_slice() {
        let response = Some(GotoDefinitionResponse::Scalar(location.clone()));
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "kernel::lsp_runtime::features",
                event = "feature:gotoDefinition",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                same_file_matches = same_file_match_count,
                other_file_matches = other_file_match_count,
                locations = 1,
                elapsed_ms,
                "goto definition resolved to single location"
            );
        }
        return Ok(response);
    }
    if !locations.is_empty() {
        let location_count = locations.len();
        let response = Some(GotoDefinitionResponse::Array(locations));
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "kernel::lsp_runtime::features",
                event = "feature:gotoDefinition",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                same_file_matches = same_file_match_count,
                other_file_matches = other_file_match_count,
                locations = location_count,
                elapsed_ms,
                "goto definition resolved to multiple locations"
            );
        }
        return Ok(response);
    }
    if let Some(qualifier) = qualifier.as_deref() {
        debug_qualified_lookup_context(state, &lookup_name, qualifier, &uri_norm);
    }
    let elapsed_ms = started_at.elapsed().as_millis();
    if elapsed_ms >= 10 {
        info!(
            target: "kernel::lsp_runtime::features",
            event = "feature:gotoDefinition",
            uri = %uri_norm,
            line = pos.line,
            character = pos.character,
            lookup_name = %lookup_name,
            same_file_matches = same_file_match_count,
            other_file_matches = other_file_match_count,
            elapsed_ms,
            "goto definition completed with no result"
        );
    }
    Ok(None)
}

fn goto_definition_response(
    uri: &Url,
    pos: Position,
    lookup_name: &str,
    started_at: Instant,
    response: Option<GotoDefinitionResponse>,
    message: &str,
) -> Result<Option<GotoDefinitionResponse>> {
    let elapsed_ms = started_at.elapsed().as_millis();
    if elapsed_ms >= 10 {
        info!(
            target: "kernel::lsp_runtime::features",
            event = "feature:gotoDefinition",
            uri = %uri,
            line = pos.line,
            character = pos.character,
            lookup_name = %lookup_name,
            elapsed_ms,
            "{message}"
        );
    }
    Ok(response)
}

pub(crate) fn references(
    state: &ServerState,
    uri: Url,
    pos: Position,
    include_declaration: bool,
) -> Result<Option<Vec<Location>>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let locations = references_resolver::resolved_references_at_position(
        state,
        &uri_norm,
        pos,
        include_declaration,
    );
    let elapsed_ms = started_at.elapsed().as_millis();
    if elapsed_ms >= 10 {
        info!(
            target: "kernel::lsp_runtime::features",
            event = "feature:references",
            uri = %uri_norm,
            line = pos.line,
            character = pos.character,
            include_declaration,
            locations = locations.as_ref().map(|items| items.len()).unwrap_or(0),
            elapsed_ms,
            "references request completed"
        );
    }
    Ok(locations)
}

pub(crate) fn document_link(state: &ServerState, uri: Url) -> Result<Option<Vec<DocumentLink>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let links = navigation::collect_document_links(text, |import_name| {
        state
            .symbol_table
            .iter()
            .find(|entry| entry.name == import_name)
            .map(|entry| entry.uri.clone())
    });
    Ok(Some(links))
}

pub(crate) fn document_highlight(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<DocumentHighlight>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
        Some(parts) => parts,
        None => return Ok(None),
    };
    if is_reserved_keyword(&word) {
        return Ok(None);
    }
    let highlights = find_reference_ranges(&text, &word)
        .into_iter()
        .map(|range| DocumentHighlight {
            range,
            kind: Some(DocumentHighlightKind::TEXT),
        })
        .collect();
    Ok(Some(highlights))
}

pub(crate) fn selection_range(
    state: &ServerState,
    uri: Url,
    positions: Vec<Position>,
) -> Result<Option<Vec<SelectionRange>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    Ok(Some(navigation::selection_ranges_for_positions(
        text,
        &positions,
        word_at_position,
    )))
}
