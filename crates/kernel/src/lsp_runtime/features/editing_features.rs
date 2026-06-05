use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::common::util;
use crate::language::{
    collect_document_symbols, collect_folding_ranges, format_document,
    suggest_create_definition_for_unresolved_type_quick_fix,
    suggest_create_matching_part_def_quick_fix, suggest_explicit_redefinition_quick_fix,
    suggest_manage_custom_libraries_quick_fix, suggest_open_library_view_quick_fix,
    suggest_search_library_for_symbol_quick_fix, suggest_show_standard_library_info_quick_fix,
    suggest_wrap_in_package,
};
use crate::workspace::ServerState;

use super::super::references_resolver;

fn collect_brace_folding_ranges(text: &str) -> Vec<FoldingRange> {
    let mut out = Vec::new();
    let mut stack: Vec<u32> = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let line_no = line_idx as u32;
        for ch in line.chars() {
            if ch == '{' {
                stack.push(line_no);
            } else if ch == '}' {
                if let Some(start_line) = stack.pop() {
                    if line_no > start_line {
                        out.push(FoldingRange {
                            start_line,
                            start_character: None,
                            end_line: line_no,
                            end_character: None,
                            kind: Some(FoldingRangeKind::Region),
                            collapsed_text: None,
                        });
                    }
                }
            }
        }
    }

    out
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

pub(crate) fn prepare_rename(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<PrepareRenameResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let target = match references_resolver::resolve_symbol_target_at_position(state, &uri_norm, pos)
    {
        Some(target) if target.is_renameable => target,
        _ => return Ok(None),
    };
    Ok(Some(PrepareRenameResponse::Range(Range::new(
        target.identifier_range.start,
        target.identifier_range.end,
    ))))
}

pub(crate) fn rename(
    state: &ServerState,
    uri: Url,
    pos: Position,
    new_name: String,
) -> Result<Option<WorkspaceEdit>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let _target = match references_resolver::resolve_symbol_target_at_position(state, &uri_norm, pos)
    {
        Some(target) if target.is_renameable => target,
        _ => return Ok(None),
    };

    let locations =
        match references_resolver::resolved_references_at_position(state, &uri_norm, pos, true) {
            Some(locations) if !locations.is_empty() => locations,
            _ => return Ok(Some(WorkspaceEdit::default())),
        };

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
    if let Some(doc) = &entry.parsed {
        let parsed_ranges = collect_folding_ranges(doc);
        if !parsed_ranges.is_empty() {
            return Ok(Some(parsed_ranges));
        }
    }
    Ok(Some(collect_brace_folding_ranges(&entry.content)))
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
        let is_implicit_redefinition_without_operator = matches!(
            diagnostic.code.as_ref(),
            Some(NumberOrString::String(code)) if code == "implicit_redefinition_without_operator"
        );
        if is_implicit_redefinition_without_operator {
            if let Some(action) = suggest_explicit_redefinition_quick_fix(&text, &uri, diagnostic) {
                actions.push(CodeActionOrCommand::CodeAction(action));
            }
        }
        let is_unresolved_type_reference = matches!(
            diagnostic.code.as_ref(),
            Some(NumberOrString::String(code))
                if code == "unresolved_type_reference" || code == "unresolved_ref_type_reference"
        );
        if is_unresolved_type_reference {
            if let Some(action) =
                suggest_create_definition_for_unresolved_type_quick_fix(&text, &uri, diagnostic)
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
                suggest_manage_custom_libraries_quick_fix(diagnostic),
            ));
            actions.push(CodeActionOrCommand::CodeAction(
                suggest_show_standard_library_info_quick_fix(diagnostic),
            ));
            actions.push(CodeActionOrCommand::CodeAction(
                suggest_open_library_view_quick_fix(diagnostic),
            ));
        }
        if is_missing_library_context || is_unresolved_type_reference {
            if let Some(action) = suggest_search_library_for_symbol_quick_fix(diagnostic) {
                actions.push(CodeActionOrCommand::CodeAction(action));
            }
        }
    }
    Ok(Some(actions))
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
