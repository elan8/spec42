use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::common::text_span::{to_core_position, to_lsp_range};
use crate::common::util;
use crate::workspace::snapshot::ServerStateSnapshot;
use crate::workspace::ServerState;

use language_service::{complete as ls_complete, CompletionItemDto, CompletionItemKindDto, WorkspaceSnapshot};

const COMPLETION_RESOLVE_DATA_KEY: &str = "spec42Completion";

#[derive(Debug, Clone, serde::Serialize)]
struct CompletionResolveData {
    detail: Option<String>,
    documentation: Option<String>,
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

pub(crate) fn completion_resolve(
    _state: &ServerState,
    mut item: CompletionItem,
) -> Result<CompletionItem> {
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
) -> Result<Option<CompletionResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
    let path = snapshot.path_for_uri(&uri_norm);
    let position = to_core_position(pos);
    let Some(result) = ls_complete(&snapshot, &path, position) else {
        return Ok(None);
    };

    Ok(Some(CompletionResponse::List(CompletionList {
        is_incomplete: result.is_incomplete,
        items: result.items.into_iter().map(map_completion_item).collect(),
    })))
}

#[cfg(test)]
mod tests {
    use super::map_completion_item;
    use language_service::dto::CompletionTextEditDto;
    use language_service::{CompletionItemDto, CompletionItemKindDto};
    use sysml_model::{TextPosition, TextRange};
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
        });
        assert_eq!(item.kind, Some(CompletionItemKind::SNIPPET));
        assert_eq!(item.insert_text_format, Some(tower_lsp::lsp_types::InsertTextFormat::SNIPPET));
        assert_eq!(item.preselect, Some(true));
    }
}
