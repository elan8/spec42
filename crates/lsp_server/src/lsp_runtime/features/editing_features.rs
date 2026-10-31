use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::common::text_span::{to_core_position, to_lsp_range};
use crate::common::util;
use crate::language::{
    collect_document_symbols, collect_folding_ranges, configure_library_paths_action,
    format_document, quick_fix_action,
};
use crate::session::snapshot::ServerStateSnapshot;
use crate::session::ServerState;
use language_service::WorkspaceSnapshot;
use sysml_query::resolved_slice::DiagnosticCode;

/// Declaration templates are documentation, not callable signatures. Keep unsupported
/// signature help quiet until the semantic service owns callable/argument context.
pub(crate) fn signature_help(
    _state: &ServerState,
    _uri: Url,
    _pos: Position,
) -> Result<Option<SignatureHelp>> {
    Ok(None)
}

pub(crate) fn prepare_rename(
    state: &ServerState,
    uri: Url,
    pos: Position,
    perf_logging_enabled: bool,
) -> Result<Option<PrepareRenameResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    if util::uri_under_any_library(&uri_norm, &state.library_paths) {
        return Ok(None);
    }
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
    let path = snapshot.path_for_uri(&uri_norm);
    let Some(target) = language_service::rename_target(&snapshot, &path, to_core_position(pos))
    else {
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
    perf_logging_enabled: bool,
    versioned_document_changes: bool,
) -> Result<Option<WorkspaceEdit>> {
    let uri_norm = util::normalize_file_uri(&uri);
    if util::uri_under_any_library(&uri_norm, &state.library_paths) {
        return Ok(None);
    }
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
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

    let Some(edits) =
        language_service::apply_rename(&snapshot, &path, to_core_position(pos), &new_name)
    else {
        return Ok(None);
    };
    if edits.is_empty() {
        return Ok(Some(WorkspaceEdit::default()));
    }

    let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> =
        std::collections::HashMap::new();
    for edit in edits {
        let edit_uri = snapshot.resolve_uri_for_path(&edit.path).ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params("A required rename target is unavailable")
        })?;
        if !state.index.contains_key(&edit_uri) {
            return Err(tower_lsp::jsonrpc::Error::invalid_params(
                "A required rename target is unavailable; no edits were returned",
            ));
        }
        if util::uri_under_any_library(&edit_uri, &state.library_paths) {
            return Err(tower_lsp::jsonrpc::Error::invalid_params(
                "Rename requires changes to a configured read-only library; no edits were returned",
            ));
        }
        changes.entry(edit_uri).or_default().push(TextEdit {
            range: to_lsp_range(edit.range),
            new_text: edit.replacement,
        });
    }
    if versioned_document_changes {
        let mut documents = changes.into_iter().collect::<Vec<_>>();
        documents.sort_by(|(left, _), (right, _)| left.as_str().cmp(right.as_str()));
        let edits = documents
            .into_iter()
            .map(|(uri, edits)| {
                let version = state
                    .index
                    .get(&uri)
                    .and_then(|entry| entry.editor_version());
                TextDocumentEdit {
                    text_document: OptionalVersionedTextDocumentIdentifier { uri, version },
                    edits: edits.into_iter().map(OneOf::Left).collect(),
                }
            })
            .collect();
        Ok(Some(WorkspaceEdit {
            document_changes: Some(DocumentChanges::Edits(edits)),
            ..WorkspaceEdit::default()
        }))
    } else {
        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            ..WorkspaceEdit::default()
        }))
    }
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
    Ok(Some(DocumentSymbolResponse::Nested(
        collect_document_symbols(entry.syntax()?),
    )))
}

pub(crate) fn folding_range(state: &ServerState, uri: Url) -> Result<Option<Vec<FoldingRange>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let entry = match state.index.get(&uri_norm) {
        Some(entry) => entry,
        None => return Ok(None),
    };
    // One derivation: the syntax service's folding regions, which already survive parser
    // recovery. An empty answer means the document has no foldable region, not that a second
    // heuristic should guess one.
    Ok(Some(collect_folding_ranges(entry.syntax()?)))
}

#[allow(deprecated)]
pub(crate) fn workspace_symbol(
    state: &ServerState,
    query: String,
    perf_logging_enabled: bool,
) -> Result<Option<Vec<SymbolInformation>>> {
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
    let out = language_service::search_workspace_symbols(&snapshot, &query)
        .into_iter()
        .filter_map(|entry| {
            let uri = Url::parse(&entry.uri).ok()?;
            Some(SymbolInformation {
                name: entry.name,
                kind: crate::language::element_kind_label_to_lsp(entry.detail.as_deref()),
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

pub(crate) fn code_action(
    state: &ServerState,
    uri: Url,
    diagnostics: &[Diagnostic],
    perf_logging_enabled: bool,
    versioned_edits: bool,
    only: Option<&[CodeActionKind]>,
) -> Result<Option<CodeActionResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    if util::uri_under_any_library(&uri_norm, &state.library_paths) {
        return Ok(None);
    }
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
    let path = snapshot.path_for_uri(&uri_norm);
    let version = state
        .index
        .get(&uri_norm)
        .and_then(|entry| entry.editor_version());
    let mut actions = Vec::new();
    const IMPORT_KIND: &str = "source.fixAll.spec42.imports";
    let wanted = |kind: &str| {
        only.is_none_or(|kinds| {
            kinds.iter().any(|requested| {
                requested.as_str() == kind || kind.starts_with(&format!("{}.", requested.as_str()))
            })
        })
    };
    if versioned_edits && version.is_some() && wanted(IMPORT_KIND) {
        if let Some(fix) = language_service::fix_all_missing_imports(&snapshot, &path) {
            let empty = Diagnostic::default();
            let mut action = quick_fix_action(fix, &uri, version, &empty);
            action.kind = Some(CodeActionKind::new(IMPORT_KIND));
            action.diagnostics = None;
            actions.push(CodeActionOrCommand::CodeAction(action));
        }
    }
    if !wanted(CodeActionKind::QUICKFIX.as_str()) {
        return Ok(Some(actions));
    }
    for diagnostic in diagnostics {
        let Some(NumberOrString::String(code)) = diagnostic.code.as_ref() else {
            continue;
        };
        let position = to_core_position(diagnostic.range.start);
        let fixes = if [
            DiagnosticCode::UnresolvedTypeReference,
            DiagnosticCode::UnresolvedSpecializesReference,
            DiagnosticCode::AmbiguousReference,
        ]
        .iter()
        .any(|reference| reference.as_str() == code)
        {
            language_service::reference_quick_fixes(&snapshot, &path, position)
        } else if code == DiagnosticCode::ImplicitRedefinitionWithoutOperator.as_str() {
            language_service::explicit_redefinition_quick_fix(&snapshot, &path, position)
                .into_iter()
                .collect()
        } else {
            if code == DiagnosticCode::MissingLibraryContext.as_str() {
                actions.push(CodeActionOrCommand::CodeAction(
                    configure_library_paths_action(diagnostic),
                ));
            }
            continue;
        };
        actions.extend(fixes.into_iter().map(|fix| {
            CodeActionOrCommand::CodeAction(quick_fix_action(fix, &uri, version, diagnostic))
        }));
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
        .map(|entry| entry.content().to_owned())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    Ok(Some(format_document(&text, &options)))
}
