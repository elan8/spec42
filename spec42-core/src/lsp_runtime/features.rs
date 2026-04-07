use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tracing::debug;

use crate::common::util;
use crate::language::{
    collect_document_symbols, collect_folding_ranges, completion_prefix, find_reference_ranges,
    format_document, is_reserved_keyword, keyword_doc, keyword_hover_markdown,
    line_prefix_at_position, suggest_create_matching_part_def_quick_fix,
    suggest_install_stdlib_quick_fix, suggest_manage_custom_libraries_quick_fix,
    suggest_wrap_in_package, sysml_keywords, word_at_position,
};
use crate::semantic_model;
use crate::semantic_tokens::{ast_semantic_ranges, semantic_tokens_full, semantic_tokens_range};
use crate::workspace::ServerState;

use super::lookup_helpers::{collect_symbol_matches_for_lookup, debug_qualified_lookup_context};
use super::{hierarchy, navigation, references_resolver, symbols};

pub(crate) fn hover(state: &ServerState, uri: Url, pos: Position) -> Result<Option<Hover>> {
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
        return Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: md,
            }),
            range: Some(range),
        }));
    }

    if let Some(node) = state.semantic_graph.find_node_at_position(&uri_norm, pos) {
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
        let markdown = if let Some(target) = target_match {
            semantic_model::hover_markdown_for_node(
                &state.semantic_graph,
                target,
                target.id.uri != uri_norm,
            )
        } else {
            semantic_model::hover_markdown_for_node(
                &state.semantic_graph,
                node,
                node.id.uri != uri_norm,
            )
        };
        return Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown,
            }),
            range: Some(range),
        }));
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
            for entry in all_matches.iter() {
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
        return Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value,
            }),
            range: Some(range),
        }));
    }

    Ok(None)
}

pub(crate) fn completion(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<CompletionResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let line_prefix = line_prefix_at_position(text, pos.line, pos.character);
    let prefix = completion_prefix(&line_prefix);

    let mut items = Vec::new();
    for kw in sysml_keywords() {
        if prefix.is_empty() || kw.starts_with(prefix) {
            items.push(CompletionItem {
                label: (*kw).to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: keyword_doc(kw).map(String::from),
                ..Default::default()
            });
        }
    }

    let mut seen = std::collections::HashSet::<String>::new();
    for entry in &state.symbol_table {
        if (prefix.is_empty() || entry.name.starts_with(prefix)) && seen.insert(entry.name.clone())
        {
            items.push(CompletionItem {
                label: entry.name.clone(),
                kind: Some(CompletionItemKind::REFERENCE),
                detail: entry.description.clone().or_else(|| entry.detail.clone()),
                ..Default::default()
            });
        }
    }

    Ok(Some(CompletionResponse::Array(items)))
}

pub(crate) fn signature_help(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<SignatureHelp>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let line = text.lines().nth(pos.line as usize).unwrap_or("");
    let cursor_prefix = line
        .chars()
        .take(pos.character as usize)
        .collect::<String>();
    let active_param = cursor_prefix.matches(',').count() as u32;
    let label = if line.contains("part def") {
        "part def <Name> : <Type>"
    } else if line.contains("port def") || line.contains("port ") {
        "port <name> : <PortType>"
    } else if line.contains("attribute") {
        "attribute <name> : <AttributeType>"
    } else {
        "name : Type"
    };
    Ok(Some(SignatureHelp {
        signatures: vec![SignatureInformation {
            label: label.to_string(),
            documentation: Some(Documentation::String(
                "Basic SysML declaration shape".to_string(),
            )),
            parameters: None,
            active_parameter: Some(active_param),
        }],
        active_signature: Some(0),
        active_parameter: Some(active_param),
    }))
}

pub(crate) fn goto_definition(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<GotoDefinitionResponse>> {
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
                return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri: target.id.uri.clone(),
                    range: target.range,
                })));
            }
        }
    }

    let (same_file_matches, other_file_matches) =
        collect_symbol_matches_for_lookup(state, &uri_norm, &lookup_name, qualifier.as_deref());
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
        return Ok(Some(GotoDefinitionResponse::Scalar(location.clone())));
    }
    if !locations.is_empty() {
        return Ok(Some(GotoDefinitionResponse::Array(locations)));
    }
    if let Some(qualifier) = qualifier.as_deref() {
        debug_qualified_lookup_context(state, &lookup_name, qualifier, &uri_norm);
    }
    Ok(None)
}

pub(crate) fn references(
    state: &ServerState,
    uri: Url,
    pos: Position,
    include_declaration: bool,
) -> Result<Option<Vec<Location>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    Ok(references_resolver::resolved_references_at_position(
        state,
        &uri_norm,
        pos,
        include_declaration,
    ))
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

pub(crate) fn prepare_rename(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<PrepareRenameResponse>> {
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
    if is_reserved_keyword(&word) {
        return Ok(None);
    }
    Ok(Some(PrepareRenameResponse::Range(Range::new(
        Position::new(line, char_start),
        Position::new(line, char_end),
    ))))
}

pub(crate) fn rename(
    state: &ServerState,
    uri: Url,
    pos: Position,
    new_name: String,
) -> Result<Option<WorkspaceEdit>> {
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

    let mut locations = Vec::new();
    for (uri, entry) in &state.index {
        for range in find_reference_ranges(&entry.content, &word) {
            locations.push(Location {
                uri: uri.clone(),
                range,
            });
        }
    }
    if locations.is_empty() {
        return Ok(Some(WorkspaceEdit::default()));
    }

    let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> =
        std::collections::HashMap::new();
    for location in locations {
        changes
            .entry(location.uri.clone())
            .or_default()
            .push(TextEdit {
                range: location.range,
                new_text: new_name.clone(),
            });
    }
    Ok(Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    }))
}

pub(crate) fn document_symbol(
    state: &ServerState,
    uri: Url,
) -> Result<Option<DocumentSymbolResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let entry = match state.index.get(&uri_norm) {
        Some(entry) => entry,
        None => return Ok(None),
    };
    let doc = match &entry.parsed {
        Some(doc) => doc,
        None => return Ok(None),
    };
    Ok(Some(DocumentSymbolResponse::Nested(
        collect_document_symbols(doc),
    )))
}

pub(crate) fn folding_range(state: &ServerState, uri: Url) -> Result<Option<Vec<FoldingRange>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let entry = match state.index.get(&uri_norm) {
        Some(entry) => entry,
        None => return Ok(None),
    };
    let doc = match &entry.parsed {
        Some(doc) => doc,
        None => return Ok(None),
    };
    Ok(Some(collect_folding_ranges(doc)))
}

#[allow(deprecated)]
pub(crate) fn workspace_symbol(
    state: &ServerState,
    query: String,
) -> Result<Option<Vec<SymbolInformation>>> {
    let query = query.to_lowercase();
    let out = state
        .symbol_table
        .iter()
        .filter(|entry| query.is_empty() || entry.name.to_lowercase().contains(&query))
        .map(|entry| SymbolInformation {
            name: entry.name.clone(),
            kind: entry.kind,
            tags: None,
            deprecated: None,
            location: Location {
                uri: entry.uri.clone(),
                range: entry.range,
            },
            container_name: entry.container_name.clone(),
        })
        .collect();
    Ok(Some(out))
}

pub(crate) fn code_action(
    state: &ServerState,
    uri: Url,
    diagnostics: &[Diagnostic],
) -> Result<Option<CodeActionResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let mut actions = Vec::new();
    if let Some(action) = suggest_wrap_in_package(&text, &uri) {
        actions.push(CodeActionOrCommand::CodeAction(action));
    }
    for diagnostic in diagnostics {
        let is_untyped_part_usage = matches!(
            diagnostic.code.as_ref(),
            Some(NumberOrString::String(code)) if code == "untyped_part_usage"
        );
        if is_untyped_part_usage {
            if let Some(action) =
                suggest_create_matching_part_def_quick_fix(&text, &uri, diagnostic)
            {
                actions.push(CodeActionOrCommand::CodeAction(action));
            }
        }
        let is_missing_library_context = matches!(
            diagnostic.code.as_ref(),
            Some(NumberOrString::String(code)) if code == "missing_library_context"
        );
        if is_missing_library_context {
            actions.push(CodeActionOrCommand::CodeAction(
                suggest_install_stdlib_quick_fix(diagnostic),
            ));
            actions.push(CodeActionOrCommand::CodeAction(
                suggest_manage_custom_libraries_quick_fix(diagnostic),
            ));
        }
    }
    Ok(Some(actions))
}

pub(crate) fn code_lens(state: &ServerState, uri: Url) -> Result<Option<Vec<CodeLens>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    Ok(Some(symbols::build_code_lens(state, &uri_norm)))
}

pub(crate) fn formatting(
    state: &ServerState,
    uri: Url,
    options: FormattingOptions,
) -> Result<Option<Vec<TextEdit>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    Ok(Some(format_document(&text, &options)))
}

pub(crate) fn semantic_tokens_full_request(
    state: &ServerState,
    uri: Url,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => (
            entry.content.clone(),
            entry.parsed.as_ref().map(ast_semantic_ranges),
        ),
        None => return Ok(None),
    };
    let (tokens, logs) = semantic_tokens_full(&text, ast_ranges.as_deref());
    Ok(Some((tokens, logs)))
}

pub(crate) fn semantic_tokens_range_request(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => (
            entry.content.clone(),
            entry.parsed.as_ref().map(ast_semantic_ranges),
        ),
        None => return Ok(None),
    };
    let (tokens, logs) = semantic_tokens_range(
        &text,
        range.start.line,
        range.start.character,
        range.end.line,
        range.end.character,
        ast_ranges.as_deref(),
    );
    Ok(Some((tokens, logs)))
}

pub(crate) fn linked_editing_range(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<LinkedEditingRanges>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (line, _, _, word) = match word_at_position(text, pos.line, pos.character) {
        Some(parts) => parts,
        None => return Ok(None),
    };
    if is_reserved_keyword(&word) {
        return Ok(None);
    }
    let line_text = text.lines().nth(line as usize).unwrap_or("");
    let declaration_like = line_text.contains(" def ")
        || line_text.trim_start().starts_with("part ")
        || line_text.trim_start().starts_with("port ")
        || line_text.trim_start().starts_with("attribute ")
        || line_text.trim_start().starts_with("action ");
    if !declaration_like {
        return Ok(None);
    }
    let ranges: Vec<_> = find_reference_ranges(text, &word)
        .into_iter()
        .filter(|range| range.start.line == line)
        .collect();
    if ranges.is_empty() {
        return Ok(None);
    }
    Ok(Some(LinkedEditingRanges {
        ranges,
        word_pattern: None,
    }))
}

pub(crate) fn moniker(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<Moniker>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
        Some(node) => node,
        None => return Ok(None),
    };
    Ok(Some(vec![hierarchy::moniker_for_node(node)]))
}

pub(crate) fn prepare_type_hierarchy(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
        Some(node) => node,
        None => return Ok(None),
    };
    Ok(Some(vec![hierarchy::type_hierarchy_item_for_node(node)]))
}

pub(crate) fn supertypes(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let node = match state
        .semantic_graph
        .find_node_at_position(&uri, range.start)
    {
        Some(node) => node,
        None => return Ok(None),
    };
    let items = state
        .semantic_graph
        .outgoing_typing_or_specializes_targets(node)
        .into_iter()
        .map(hierarchy::type_hierarchy_item_for_node)
        .collect();
    Ok(Some(items))
}

pub(crate) fn subtypes(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let node = match state
        .semantic_graph
        .find_node_at_position(&uri, range.start)
    {
        Some(node) => node,
        None => return Ok(None),
    };
    let items = state
        .semantic_graph
        .incoming_typing_or_specializes_sources(node)
        .into_iter()
        .map(hierarchy::type_hierarchy_item_for_node)
        .collect();
    Ok(Some(items))
}

pub(crate) fn prepare_call_hierarchy(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<CallHierarchyItem>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
        Some(node) => node,
        None => return Ok(None),
    };
    Ok(Some(vec![hierarchy::call_hierarchy_item_for_node(node)]))
}

pub(crate) fn incoming_calls(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
    let node = match state
        .semantic_graph
        .find_node_at_position(&uri, range.start)
    {
        Some(node) => node,
        None => return Ok(None),
    };
    let from_ranges = vec![range];
    let calls = state
        .semantic_graph
        .incoming_perform_sources(node)
        .into_iter()
        .map(|src| CallHierarchyIncomingCall {
            from: hierarchy::call_hierarchy_item_for_node(src),
            from_ranges: from_ranges.clone(),
        })
        .collect();
    Ok(Some(calls))
}

pub(crate) fn outgoing_calls(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
    let node = match state
        .semantic_graph
        .find_node_at_position(&uri, range.start)
    {
        Some(node) => node,
        None => return Ok(None),
    };
    let from_ranges = vec![range];
    let calls = state
        .semantic_graph
        .outgoing_perform_targets(node)
        .into_iter()
        .map(|target| CallHierarchyOutgoingCall {
            to: hierarchy::call_hierarchy_item_for_node(target),
            from_ranges: from_ranges.clone(),
        })
        .collect();
    Ok(Some(calls))
}
