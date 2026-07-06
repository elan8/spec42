use language_service::WorkspaceSnapshot;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::common::text_span::{to_core_position, to_lsp_range};
use crate::common::util;
use crate::language::word_at_position;
use crate::workspace::{snapshot::ServerStateSnapshot, ServerState};

use crate::lsp_runtime::navigation;
use crate::lsp_runtime::references_resolver;

pub(crate) fn hover(
    state: &ServerState,
    uri: Url,
    pos: Position,
    perf_logging_enabled: bool,
) -> Result<Option<Hover>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
    let path = snapshot.path_for_uri(&uri_norm);
    let result = language_service::hover(&snapshot, &path, to_core_position(pos));
    Ok(result.map(map_hover_to_lsp))
}

pub(crate) fn goto_definition(
    state: &ServerState,
    uri: Url,
    pos: Position,
    perf_logging_enabled: bool,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
    let path = snapshot.path_for_uri(&uri_norm);
    let result = language_service::goto_definition(&snapshot, &path, to_core_position(pos));
    Ok(map_definition_to_lsp(&snapshot, result))
}

pub(crate) fn references(
    state: &ServerState,
    uri: Url,
    pos: Position,
    include_declaration: bool,
    perf_logging_enabled: bool,
) -> Result<Option<Vec<Location>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let snapshot = ServerStateSnapshot::new(state, perf_logging_enabled);
    let path = snapshot.path_for_uri(&uri_norm);
    let result = language_service::find_references(
        &snapshot,
        &path,
        to_core_position(pos),
        include_declaration,
    );
    Ok(Some(map_references_to_lsp(&snapshot, result)))
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
    perf_logging_enabled: bool,
) -> Result<Option<Vec<DocumentHighlight>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let locations = references_resolver::resolved_references_at_position(
        state,
        &uri_norm,
        pos,
        true,
        perf_logging_enabled,
    );
    let locations = match locations {
        Some(locations) if !locations.is_empty() => locations,
        _ => return Ok(None),
    };
    let highlights = locations
        .into_iter()
        .filter(|location| util::normalize_file_uri(&location.uri) == uri_norm)
        .map(|location| DocumentHighlight {
            range: location.range,
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

fn map_hover_to_lsp(result: language_service::HoverResult) -> Hover {
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: result.contents,
        }),
        range: result.range.map(to_lsp_range),
    }
}

fn map_definition_to_lsp(
    snapshot: &ServerStateSnapshot<'_>,
    result: language_service::DefinitionResult,
) -> Option<GotoDefinitionResponse> {
    let locations: Vec<Location> = result
        .locations
        .into_iter()
        .filter_map(|loc| map_source_location(snapshot, loc))
        .collect();
    match locations.as_slice() {
        [] => None,
        [location] => Some(GotoDefinitionResponse::Scalar(location.clone())),
        _ => Some(GotoDefinitionResponse::Array(locations)),
    }
}

fn map_references_to_lsp(
    snapshot: &ServerStateSnapshot<'_>,
    result: language_service::ReferencesResult,
) -> Vec<Location> {
    result
        .locations
        .into_iter()
        .filter_map(|loc| map_source_location(snapshot, loc))
        .collect()
}

fn map_source_location(
    snapshot: &ServerStateSnapshot<'_>,
    location: language_service::SourceLocation,
) -> Option<Location> {
    let uri = snapshot.resolve_uri_for_path(&location.path)?;
    Some(Location {
        uri,
        range: to_lsp_range(location.range),
    })
}
