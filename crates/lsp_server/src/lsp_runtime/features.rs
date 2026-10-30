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
    semantic_tokens_debug: bool,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => {
            let text = entry.content().to_owned();
            let ast_ranges = Some(ast_semantic_ranges(entry.syntax()?, &text));
            (text, ast_ranges)
        }
        None => return Ok(None),
    };
    let (tokens, logs) = semantic_tokens_full(&text, ast_ranges.as_deref(), semantic_tokens_debug);
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
    semantic_tokens_debug: bool,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => {
            let text = entry.content().to_owned();
            let ast_ranges = Some(ast_semantic_ranges(entry.syntax()?, &text));
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
        semantic_tokens_debug,
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

/// SysML declarations do not form spelling-based paired editing groups. Explicit rename
/// uses canonical identities; typing into one name must not rewrite unrelated declarations.
pub(crate) fn linked_editing_range(
    _state: &ServerState,
    _uri: Url,
    _pos: Position,
) -> Result<Option<LinkedEditingRanges>> {
    Ok(None)
}

/// The resolved reference target, or containing declaration when there is no reference.
fn element_at(
    state: &ServerState,
    uri: &Url,
    position: Position,
) -> Option<sysml_query::resolved_slice::ElementInspection> {
    use sysml_query::resolved_slice::{QueryAnswer, QueryOutcome, TextPosition};

    let model = state.published_model();
    let at = match model.inspection().inspect_at(
        uri.as_str(),
        TextPosition {
            line: position.line,
            character: position.character,
        },
    ) {
        QueryOutcome {
            completeness: _publication_completeness,
            answer: QueryAnswer::Resolved(at),
        } => at,
        _ => return None,
    };
    match at.referenced {
        sysml_query::resolved_slice::ReferenceAt::Resolved(target) => Some(*target),
        sysml_query::resolved_slice::ReferenceAt::None => at.containing,
        _ => None,
    }
}

/// One published specialization step, in either direction.
///
/// `AnySpecialization` is the scope the OMG Pilot's `Type::supertypes` uses, so a feature typing
/// and a subsetting are steps in this hierarchy exactly as a subclassification is.
fn hierarchy_step(
    state: &ServerState,
    symbol: sysml_query::resolved_slice::SymbolId,
    ascending: bool,
) -> Option<Vec<TypeHierarchyItem>> {
    use sysml_query::resolved_slice::{QueryAnswer, QueryOutcome, SpecializationScope};

    let model = state.published_model();
    let outcome = if ascending {
        model
            .types()
            .direct_supertypes(symbol, SpecializationScope::AnySpecialization)
    } else {
        model
            .types()
            .direct_subtypes(symbol, SpecializationScope::AnySpecialization)
    };
    let symbols = match outcome {
        QueryOutcome {
            completeness: _publication_completeness,
            answer: QueryAnswer::Resolved(symbols),
        } => symbols,
        _ => return None,
    };
    Some(
        symbols
            .iter()
            .filter_map(|symbol| match model.inspection().inspect(*symbol) {
                QueryOutcome {
                    completeness: _publication_completeness,
                    answer: QueryAnswer::Resolved(inspection),
                } => hierarchy::type_hierarchy_item(model, &inspection),
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
    Ok(hierarchy::type_hierarchy_item(state.published_model(), &element).map(|item| vec![item]))
}

pub(crate) fn supertypes(
    state: &ServerState,
    item: TypeHierarchyItem,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let symbol = hierarchy::item_symbol(state.published_model(), &item)?;
    Ok(hierarchy_step(state, symbol, true))
}

pub(crate) fn subtypes(
    state: &ServerState,
    item: TypeHierarchyItem,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let symbol = hierarchy::item_symbol(state.published_model(), &item)?;
    Ok(hierarchy_step(state, symbol, false))
}
