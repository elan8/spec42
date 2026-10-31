use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::common::text_span::{to_core_position, to_lsp_range};
use crate::common::util;
use crate::session::snapshot::ServerStateSnapshot;
use crate::session::ServerState;

use language_service::{CompletionItemDto, CompletionItemKindDto, WorkspaceSnapshot};

const COMPLETION_RESOLVE_DATA_KEY: &str = "spec42Completion";

#[derive(Debug, Clone, serde::Serialize)]
struct CompletionResolveData {
    detail: Option<String>,
    documentation: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ImportResolveData {
    uri: Url,
    version: i32,
    open_interval: String,
    model_digest: String,
    selection: language_service::dto::ImportCompletionDto,
}

fn import_data(item: &CompletionItem) -> Result<Option<ImportResolveData>> {
    item.data
        .as_ref()
        .and_then(|data| data.get(COMPLETION_RESOLVE_DATA_KEY))
        .and_then(|data| data.get("import"))
        .map(|data| {
            serde_json::from_value(data.clone()).map_err(|_| {
                tower_lsp::jsonrpc::Error::invalid_params("Invalid completion import identity")
            })
        })
        .transpose()
}

pub(crate) fn completion_import_uri(item: &CompletionItem) -> Result<Option<Url>> {
    Ok(import_data(item)?.map(|data| data.uri))
}

pub(crate) fn same_editor_revision(before: &ServerState, after: &ServerState, uri: &Url) -> bool {
    let uri = util::normalize_file_uri(uri);
    let revision = |state: &ServerState| {
        state
            .index
            .get(&uri)
            .and_then(|entry| entry.editor_revision.as_ref())
            .map(|revision| (revision.token(), revision.version()))
    };
    revision(before) == revision(after)
}

pub(crate) fn same_edit_context(before: &ServerState, after: &ServerState, uri: &Url) -> bool {
    same_editor_revision(before, after, uri)
        && before.session.current().publication().model_digest()
            == after.session.current().publication().model_digest()
}

pub(crate) fn completion_resolve_import(
    state: &ServerState,
    item: CompletionItem,
) -> Result<CompletionItem> {
    let Some(data) = import_data(&item)? else {
        return completion_resolve(item);
    };
    let uri = util::normalize_file_uri(&data.uri);
    let entry = state
        .index
        .get(&uri)
        .ok_or_else(tower_lsp::jsonrpc::Error::content_modified)?;
    let revision = entry
        .editor_revision
        .as_ref()
        .ok_or_else(tower_lsp::jsonrpc::Error::content_modified)?;
    if revision.token().to_string() != data.open_interval
        || revision.version() != Some(data.version)
        || state
            .session
            .current()
            .publication()
            .model_digest()
            .to_string()
            != data.model_digest
    {
        return Err(tower_lsp::jsonrpc::Error::content_modified());
    }
    let snapshot = ServerStateSnapshot::new(state, false);
    let path = snapshot.path_for_uri(&uri);
    let (primary, edits) =
        language_service::resolve_completion_import(&snapshot, &path, &data.selection)
            .ok_or_else(tower_lsp::jsonrpc::Error::content_modified)?;
    let expected = CompletionTextEdit::Edit(TextEdit {
        range: to_lsp_range(primary.range),
        new_text: primary.new_text,
    });
    if item.text_edit.as_ref() != Some(&expected) {
        return Err(tower_lsp::jsonrpc::Error::content_modified());
    }
    let mut item = completion_resolve(item)?;
    item.additional_text_edits = Some(
        edits
            .into_iter()
            .map(|edit| TextEdit {
                range: to_lsp_range(edit.range),
                new_text: edit.replacement,
            })
            .collect(),
    );
    Ok(item)
}

fn markdown_documentation(markdown: String) -> Documentation {
    Documentation::MarkupContent(MarkupContent {
        kind: MarkupKind::Markdown,
        value: markdown,
    })
}

fn completion_kind_to_lsp(kind: CompletionItemKindDto) -> CompletionItemKind {
    match kind {
        CompletionItemKindDto::Keyword => CompletionItemKind::KEYWORD,
        CompletionItemKindDto::Snippet => CompletionItemKind::SNIPPET,
        CompletionItemKindDto::Module => CompletionItemKind::MODULE,
        CompletionItemKindDto::Class => CompletionItemKind::CLASS,
        CompletionItemKindDto::Interface => CompletionItemKind::INTERFACE,
        CompletionItemKindDto::Function => CompletionItemKind::FUNCTION,
        CompletionItemKindDto::Property => CompletionItemKind::PROPERTY,
        CompletionItemKindDto::Variable => CompletionItemKind::VARIABLE,
        CompletionItemKindDto::Event => CompletionItemKind::EVENT,
        CompletionItemKindDto::Reference => CompletionItemKind::REFERENCE,
    }
}

fn map_completion_item(dto: CompletionItemDto) -> CompletionItem {
    let resolve_detail = dto.resolve_detail.clone();
    let resolve_documentation = dto.resolve_documentation.clone();
    let documentation = dto.documentation.clone().map(|value| {
        if dto.documentation_is_markdown {
            markdown_documentation(value)
        } else {
            Documentation::String(value)
        }
    });
    let label_details = dto.label_details.map(|details| CompletionItemLabelDetails {
        detail: details.detail,
        description: details.description,
    });
    let text_edit = dto.text_edit.map(|edit| {
        CompletionTextEdit::Edit(TextEdit {
            range: to_lsp_range(edit.range),
            new_text: edit.new_text,
        })
    });
    let data = if resolve_detail.is_some() || resolve_documentation.is_some() {
        Some(serde_json::json!({
            COMPLETION_RESOLVE_DATA_KEY: CompletionResolveData {
                detail: resolve_detail,
                documentation: resolve_documentation,
            }
        }))
    } else {
        None
    };

    CompletionItem {
        label: dto.label,
        kind: dto.kind.map(completion_kind_to_lsp),
        detail: dto.detail,
        documentation,
        label_details,
        filter_text: dto.filter_text,
        text_edit,
        insert_text_format: dto
            .insert_text_format_snippet
            .then_some(InsertTextFormat::SNIPPET),
        sort_text: dto.sort_text,
        preselect: Some(dto.preselect),
        deprecated: Some(dto.deprecated),
        data,
        ..CompletionItem::default()
    }
}

pub(crate) fn completion_resolve(mut item: CompletionItem) -> Result<CompletionItem> {
    let Some(data) = item.data.as_ref() else {
        return Ok(item);
    };
    let Some(payload) = data.get(COMPLETION_RESOLVE_DATA_KEY) else {
        return Ok(item);
    };
    item.detail = item.detail.or_else(|| {
        payload
            .get("detail")
            .and_then(|value| value.as_str())
            .map(str::to_string)
    });
    if item.documentation.is_none() {
        item.documentation = payload
            .get("documentation")
            .and_then(|value| value.as_str())
            .map(|value| markdown_documentation(value.to_string()));
    }
    Ok(item)
}

pub(crate) fn completion(
    state: &ServerState,
    uri: Url,
    pos: Position,
    perf_logging_enabled: bool,
    import_resolve_supported: bool,
) -> Result<Option<CompletionResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
    let path = snapshot.path_for_uri(&uri_norm);
    let position = to_core_position(pos);
    let Some(result) = language_service::completion::complete_with_imports(
        &snapshot,
        &path,
        position,
        import_resolve_supported,
    ) else {
        return Ok(None);
    };

    let revision = state
        .index
        .get(&uri_norm)
        .and_then(|entry| entry.editor_revision.as_ref());
    let guard = revision.and_then(|revision| Some((revision.token(), revision.version()?)));
    let items = result
        .items
        .into_iter()
        .filter_map(|dto| {
            let selection = dto.import.clone();
            if selection.is_some()
                && (!import_resolve_supported
                    || guard.is_none()
                    || util::uri_under_any_library(&uri_norm, &state.library_paths))
            {
                return None;
            }
            let mut item = map_completion_item(dto);
            if let (Some(selection), Some((open_interval, version))) = (selection, guard) {
                // VS Code may commit the primary edit before an asynchronous resolve finishes.
                // Deliver both validated edits now so even immediate acceptance is atomic.
                let (_, edits) =
                    language_service::resolve_completion_import(&snapshot, &path, &selection)?;
                item.additional_text_edits = Some(
                    edits
                        .into_iter()
                        .map(|edit| TextEdit {
                            range: to_lsp_range(edit.range),
                            new_text: edit.replacement,
                        })
                        .collect(),
                );
                let data = ImportResolveData {
                    uri: uri_norm.clone(),
                    version,
                    open_interval: open_interval.to_string(),
                    model_digest: state
                        .session
                        .current()
                        .publication()
                        .model_digest()
                        .to_string(),
                    selection,
                };
                item.data.as_mut()?.get_mut(COMPLETION_RESOLVE_DATA_KEY)?["import"] =
                    serde_json::to_value(data).ok()?;
            }
            Some(item)
        })
        .collect();
    Ok(Some(CompletionResponse::List(CompletionList {
        is_incomplete: result.is_incomplete,
        items,
    })))
}

#[cfg(test)]
mod tests {
    use super::map_completion_item;
    use language_service::dto::CompletionTextEditDto;
    use language_service::{CompletionItemDto, CompletionItemKindDto};
    use sysml_query::resolved_slice::{TextPosition, TextRange};
    use tower_lsp::lsp_types::CompletionItemKind;

    #[test]
    fn maps_snippet_completion_to_lsp_shape() {
        let item = map_completion_item(CompletionItemDto {
            label: "part def".to_string(),
            kind: Some(CompletionItemKindDto::Snippet),
            detail: Some("part definition".to_string()),
            documentation: None,
            documentation_is_markdown: false,
            label_details: None,
            filter_text: Some("part def".to_string()),
            text_edit: Some(CompletionTextEditDto {
                range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 2)),
                new_text: "part def ${1:Name}".to_string(),
            }),
            insert_text_format_snippet: true,
            sort_text: None,
            preselect: true,
            deprecated: false,
            resolve_detail: None,
            resolve_documentation: None,
            import: None,
        });
        assert_eq!(item.kind, Some(CompletionItemKind::SNIPPET));
        assert_eq!(
            item.insert_text_format,
            Some(tower_lsp::lsp_types::InsertTextFormat::SNIPPET)
        );
        assert_eq!(item.preselect, Some(true));
    }
}
