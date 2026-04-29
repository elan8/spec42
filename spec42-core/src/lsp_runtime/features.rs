mod completion;
mod editing_features;
mod navigation_requests;
mod shared;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tracing::info;

use crate::common::util;
use crate::language::{is_reserved_keyword, word_at_position};
use crate::semantic_tokens::{ast_semantic_ranges, semantic_tokens_full, semantic_tokens_range};
use crate::workspace::ServerState;

use super::{hierarchy, symbols};

pub(crate) use completion::{completion, completion_resolve};
pub(crate) use editing_features::{
    code_action, document_symbol, folding_range, formatting, prepare_rename, rename,
    signature_help, workspace_symbol,
};
pub(crate) use navigation_requests::{
    document_highlight, document_link, goto_definition, hover, references, selection_range,
};

static CODE_LENS_REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);
static SEMANTIC_TOKENS_FULL_REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);
static SEMANTIC_TOKENS_RANGE_REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

pub(crate) fn code_lens(state: &ServerState, uri: Url) -> Result<Option<Vec<CodeLens>>> {
    if !state.code_lens_enabled {
        return Ok(None);
    }
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let lenses = symbols::build_code_lens(state, &uri_norm);
    let elapsed_ms = started_at.elapsed().as_millis();
    let request_count = CODE_LENS_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if state.perf_logging_enabled {
        info!(
            target: "kernel::lsp_runtime::features",
            event = "feature:codeLens",
            uri = %uri_norm,
            lenses = lenses.len(),
            elapsed_ms,
            request_count,
            "code lens request completed"
        );
    }
    Ok(Some(lenses))
}

pub(crate) fn inlay_hint(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<InlayHint>>> {
    let _ = (state, uri, range);
    Ok(Some(Vec::new()))
}

pub(crate) fn semantic_tokens_full_request(
    state: &ServerState,
    uri: Url,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => (
            entry.content.clone(),
            entry.parsed.as_ref().map(ast_semantic_ranges),
        ),
        None => return Ok(None),
    };
    let (tokens, logs) = semantic_tokens_full(&text, ast_ranges.as_deref());
    let elapsed_ms = started_at.elapsed().as_millis();
    let request_count = SEMANTIC_TOKENS_FULL_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    info!(
        target: "kernel::lsp_runtime::features",
        event = "feature:semanticTokensFull",
        uri = %uri_norm,
        token_count = tokens.data.len(),
        log_count = logs.len(),
        elapsed_ms,
        request_count,
        "semantic tokens full request completed"
    );
    Ok(Some((tokens, logs)))
}

pub(crate) fn semantic_tokens_range_request(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let started_at = Instant::now();
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
    let elapsed_ms = started_at.elapsed().as_millis();
    let request_count = SEMANTIC_TOKENS_RANGE_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    info!(
        target: "kernel::lsp_runtime::features",
        event = "feature:semanticTokensRange",
        uri = %uri_norm,
        start_line = range.start.line,
        end_line = range.end.line,
        token_count = tokens.data.len(),
        log_count = logs.len(),
        elapsed_ms,
        request_count,
        "semantic tokens range request completed"
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
    let ranges: Vec<_> = crate::language::find_reference_ranges(text, &word)
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
