use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::common::text_span::{to_core_position, to_lsp_range};
use crate::common::util;
use crate::language::{
    collect_document_symbols, collect_folding_ranges, format_document,
    suggest_create_definition_for_unresolved_type_quick_fix,
    suggest_create_matching_part_def_quick_fix, suggest_explicit_redefinition_quick_fix,
    suggest_manage_custom_libraries_quick_fix, suggest_open_library_view_quick_fix,
    suggest_search_library_for_symbol_quick_fix, suggest_show_standard_library_info_quick_fix,
    suggest_wrap_in_package,
};
use language_service::WorkspaceSnapshot;
use crate::workspace::snapshot::ServerStateSnapshot;
use crate::workspace::ServerState;

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
    if util::uri_under_any_library(&uri_norm, &state.library_paths) {
        return Ok(None);
    }
    let snapshot = ServerStateSnapshot::new(state);
    let path = snapshot.path_for_uri(&uri_norm);
    let Some(target) = language_service::rename_target(&snapshot, &path, to_core_position(pos)) else {
        return Ok(None);
    };
    if let Some(def_uri) = snapshot.resolve_uri_for_path(&target.definition.path) {
        if util::uri_under_any_library(&def_uri, &state.library_paths) {
            return Ok(None);
        }
    }
    let Some(range) = language_service::prepare_rename(&snapshot, &path, to_core_position(pos))
    else {
        return Ok(None);
    };
    Ok(Some(PrepareRenameResponse::Range(to_lsp_range(range))))
}

pub(crate) fn rename(
    state: &ServerState,
    uri: Url,
    pos: Position,
    new_name: String,
) -> Result<Option<WorkspaceEdit>> {
    let uri_norm = util::normalize_file_uri(&uri);
    if util::uri_under_any_library(&uri_norm, &state.library_paths) {
        return Ok(None);
    }
    let snapshot = ServerStateSnapshot::new(state);
    let path = snapshot.path_for_uri(&uri_norm);
    if language_service::prepare_rename(&snapshot, &path, to_core_position(pos)).is_none() {
        return Ok(None);
    }
    let definition_uri = language_service::rename_target(&snapshot, &path, to_core_position(pos))
        .map(|target| {
            snapshot
                .resolve_uri_for_path(&target.definition.path)
                .unwrap_or_else(|| uri_norm.clone())
        });
    if let Some(def_uri) = definition_uri {
        if util::uri_under_any_library(&def_uri, &state.library_paths) {
            return Ok(None);
        }
    }

    let edits =
        language_service::apply_rename(&snapshot, &path, to_core_position(pos), &new_name);
    if edits.is_empty() {
        return Ok(Some(WorkspaceEdit::default()));
    }

    let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> =
        std::collections::HashMap::new();
    for edit in edits {
        let Some(edit_uri) = snapshot.resolve_uri_for_path(&edit.path) else {
            continue;
        };
        if util::uri_under_any_library(&edit_uri, &state.library_paths) {
            continue;
        }
        changes.entry(edit_uri).or_default().push(TextEdit {
            range: to_lsp_range(edit.range),
            new_text: edit.replacement,
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
    let snapshot = ServerStateSnapshot::new(state);
    let out = language_service::search_workspace_symbols(&snapshot, &query)
        .into_iter()
        .filter_map(|entry| {
            let uri = Url::parse(&entry.uri).ok()?;
            Some(SymbolInformation {
                name: entry.name,
                kind: workspace_symbol_kind(entry.detail.as_deref().unwrap_or("symbol")),
                tags: None,
                deprecated: None,
                location: Location {
                    uri,
                    range: to_lsp_range(entry.range),
                },
                container_name: entry.container,
            })
        })
        .collect();
    Ok(Some(out))
}

fn workspace_symbol_kind(kind: &str) -> SymbolKind {
    match kind {
        "package" | "namespace" | "library package" => SymbolKind::MODULE,
        "part def" | "classifier decl" => SymbolKind::CLASS,
        "port def" | "interface" | "port" => SymbolKind::INTERFACE,
        "attribute def" | "attribute" | "feature decl" | "ref" => SymbolKind::PROPERTY,
        "action def" => SymbolKind::FUNCTION,
        "part" => SymbolKind::OBJECT,
        "action" => SymbolKind::EVENT,
        "view def" | "viewpoint def" | "rendering def" | "view" | "viewpoint" | "rendering" => {
            SymbolKind::NAMESPACE
        }
        _ => SymbolKind::VARIABLE,
    }
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
