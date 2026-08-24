//! `textDocument/codeLens` support.

use std::time::Instant;

use crate::session::ServerState;
use tower_lsp::lsp_types::{CodeLens, Url};
use tracing::info;

/// This server publishes no code lenses.
///
/// It once carried a staged inherited-attribute lens that formatted a declared and an effective
/// value with the ancestor they came from. It was never emitted -- this function has always
/// returned an empty list -- and it derived every fact it showed by walking specialization edges
/// of the mutable graph, reading evaluation facts by graph node, and inferring a type from the
/// bracketed text of a rendered value. Rebuilding that over the immutable publication would have
/// been migrating a feature no user has; it is deleted instead, and a future lens starts from
/// `PublishedModel::element_details`.
pub(crate) fn build_code_lens(
    state: &ServerState,
    uri_norm: &Url,
    perf_logging_enabled: bool,
) -> Vec<CodeLens> {
    let started_at = Instant::now();
    let indexed_symbols = state
        .semantic_symbols
        .iter()
        .filter(|s| s.uri == *uri_norm)
        .count();
    let elapsed_ms = started_at.elapsed().as_millis();
    if perf_logging_enabled && elapsed_ms >= 10 {
        info!(
            target: "lsp_server::lsp_runtime::symbols",
            event = "symbols:buildCodeLens",
            uri = %uri_norm,
            indexed_symbols,
            emitted_lenses = 0,
            elapsed_ms,
            "build_code_lens completed"
        );
    }
    Vec::new()
}
