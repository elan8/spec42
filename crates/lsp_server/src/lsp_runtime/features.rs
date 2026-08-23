mod completion;
mod editing_features;
mod navigation_requests;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tracing::info;

use crate::common::util;
use crate::semantic_tokens::{ast_semantic_ranges, semantic_tokens_full, semantic_tokens_range};
use crate::session::ServerState;

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

pub(crate) fn code_lens(
    state: &ServerState,
    uri: Url,
    code_lens_enabled: bool,
    perf_logging_enabled: bool,
) -> Result<Option<Vec<CodeLens>>> {
    if !code_lens_enabled {
        return Ok(None);
    }
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let lenses = symbols::build_code_lens(state, &uri_norm, perf_logging_enabled);
    let elapsed_ms = started_at.elapsed().as_millis();
    let request_count = CODE_LENS_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if perf_logging_enabled {
        info!(
            target: "lsp_server::lsp_runtime::features",
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
    perf_logging_enabled: bool,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => {
            let text = entry.content().to_owned();
            let ast_ranges = Some(ast_semantic_ranges(&entry.parsed, &text));
            (text, ast_ranges)
        }
        None => return Ok(None),
    };
    let (tokens, logs) = semantic_tokens_full(&text, ast_ranges.as_deref());
    let elapsed_ms = started_at.elapsed().as_millis();
    let request_count = SEMANTIC_TOKENS_FULL_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if perf_logging_enabled {
        info!(
            target: "lsp_server::lsp_runtime::features",
            event = "feature:semanticTokensFull",
            uri = %uri_norm,
            token_count = tokens.data.len(),
            log_count = logs.len(),
            elapsed_ms,
            request_count,
            "semantic tokens full request completed"
        );
    }
    Ok(Some((tokens, logs)))
}

pub(crate) fn semantic_tokens_range_request(
    state: &ServerState,
    uri: Url,
    range: Range,
    perf_logging_enabled: bool,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => {
            let text = entry.content().to_owned();
            let ast_ranges = Some(ast_semantic_ranges(&entry.parsed, &text));
            (text, ast_ranges)
        }
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
    if perf_logging_enabled {
        info!(
            target: "lsp_server::lsp_runtime::features",
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
    }
    Ok(Some((tokens, logs)))
}

pub(crate) fn linked_editing_range(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<LinkedEditingRanges>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let entry = match state.index.get(&uri_norm) {
        Some(entry) => entry,
        None => return Ok(None),
    };
    let Some(token) = entry.parsed.token_at(pos.line, pos.character) else {
        return Ok(None);
    };
    if token.is_keyword {
        return Ok(None);
    }
    let line = token.range.start_line;
    // Linked editing only applies on a declaration's own header line. The syntax service says
    // which line that is; this used to guess from the line's leading keyword, which both missed
    // declaration forms it had not enumerated and fired inside comments and strings.
    let on_declaration_header = entry
        .parsed
        .declaration_at(line)
        .is_some_and(|declaration| declaration.head_range.start_line == line);
    if !on_declaration_header {
        return Ok(None);
    }
    // Occurrences the syntax service found in code. The substring scan this replaced also
    // matched inside comments and string literals, so editing a name could rewrite prose.
    let ranges: Vec<_> = entry
        .parsed
        .occurrences_of(&token.text)
        .into_iter()
        .filter(|range| range.start_line == line)
        .map(|range| {
            Range::new(
                Position::new(range.start_line, range.start_character),
                Position::new(range.end_line, range.end_character),
            )
        })
        .collect();
    if ranges.is_empty() {
        return Ok(None);
    }
    Ok(Some(LinkedEditingRanges {
        ranges,
        word_pattern: None,
    }))
}

/// The published element whose declaration contains `position`, if the publication settled one.
fn element_at(
    state: &ServerState,
    uri: &Url,
    position: Position,
) -> Option<sysml_query::resolved_slice::ElementInspection> {
    use sysml_query::resolved_slice::{QueryOutcome, TextPosition};

    let model = state.published_model();
    let at = match model.inspection().inspect_at(
        uri.as_str(),
        TextPosition {
            line: position.line,
            character: position.character,
        },
    ) {
        QueryOutcome::Resolved(at)
        | QueryOutcome::Recovered(at)
        | QueryOutcome::UnsupportedWith(at) => at,
        _ => return None,
    };
    at.containing
}

/// One published specialization step, in either direction.
///
/// `AnySpecialization` is the scope the OMG Pilot's `Type::supertypes` uses, so a feature typing
/// and a subsetting are steps in this hierarchy exactly as a subclassification is.
fn hierarchy_step(
    state: &ServerState,
    uri: &Url,
    position: Position,
    ascending: bool,
) -> Option<Vec<TypeHierarchyItem>> {
    use sysml_query::resolved_slice::{QueryOutcome, SpecializationScope};

    let model = state.published_model();
    let element = element_at(state, uri, position)?;
    let outcome = if ascending {
        model
            .types()
            .direct_supertypes(element.identity, SpecializationScope::AnySpecialization)
    } else {
        model
            .types()
            .direct_subtypes(element.identity, SpecializationScope::AnySpecialization)
    };
    let symbols = match outcome {
        QueryOutcome::Resolved(symbols)
        | QueryOutcome::Recovered(symbols)
        | QueryOutcome::UnsupportedWith(symbols) => symbols,
        _ => return Some(Vec::new()),
    };
    Some(
        symbols
            .iter()
            .filter_map(|symbol| match model.inspection().inspect(*symbol) {
                QueryOutcome::Resolved(inspection)
                | QueryOutcome::Recovered(inspection)
                | QueryOutcome::UnsupportedWith(inspection) => {
                    hierarchy::type_hierarchy_item(&inspection)
                }
                _ => None,
            })
            .collect(),
    )
}

pub(crate) fn prepare_type_hierarchy(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let Some(element) = element_at(state, &uri_norm, pos) else {
        return Ok(None);
    };
    Ok(hierarchy::type_hierarchy_item(&element).map(|item| vec![item]))
}

pub(crate) fn supertypes(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    Ok(hierarchy_step(state, &uri, range.start, true))
}

pub(crate) fn subtypes(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    Ok(hierarchy_step(state, &uri, range.start, false))
}
