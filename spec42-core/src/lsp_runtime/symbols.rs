use crate::workspace::ServerState;
use std::time::Instant;
use tower_lsp::lsp_types::{CodeLens, Command, Url};
use tracing::info;

pub(crate) fn build_code_lens(state: &ServerState, uri_norm: &Url) -> Vec<CodeLens> {
    let started_at = Instant::now();
    let mut out = Vec::new();
    let mut seen_ranges = std::collections::HashSet::<(u32, u32, u32, u32)>::new();
    let mut sorted_symbols: Vec<_> = state
        .symbol_table
        .iter()
        .filter(|s| s.uri == *uri_norm)
        .collect();
    sorted_symbols.sort_by_key(|s| (s.range.start.line, s.range.start.character, s.name.clone()));
    let indexed_symbols = sorted_symbols.len();
    for sym in sorted_symbols {
        if sym.name.trim().is_empty() {
            continue;
        }
        if sym
            .description
            .as_deref()
            .map(|d| d.starts_with("short name"))
            .unwrap_or(false)
            || sym.detail.as_deref() == Some("short name")
        {
            continue;
        }
        let key = (
            sym.range.start.line,
            sym.range.start.character,
            sym.range.end.line,
            sym.range.end.character,
        );
        if !seen_ranges.insert(key) {
            continue;
        }
        let symbol_started_at = Instant::now();
        let refs = crate::lsp_runtime::references_resolver::resolved_references_for_symbol(
            state, sym, false,
        )
        .len();
        let symbol_elapsed_ms = symbol_started_at.elapsed().as_millis();
        if state.perf_logging_enabled && symbol_elapsed_ms >= 10 {
            info!(
                target: "spec42_core::lsp_runtime::symbols",
                event = "symbols:codeLensReferenceCount",
                uri = %uri_norm,
                symbol = %sym.name,
                line = sym.range.start.line,
                character = sym.range.start.character,
                refs,
                elapsed_ms = symbol_elapsed_ms,
                "code lens reference count resolved"
            );
        }
        let reference_position =
            crate::lsp_runtime::references_resolver::symbol_name_position(state, sym)
                .unwrap_or(sym.range.start);
        out.push(CodeLens {
            range: sym.range,
            command: Some(Command {
                title: format!("{refs} reference(s)"),
                command: "spec42.showReferencesCount".to_string(),
                arguments: Some(vec![
                    serde_json::to_value(uri_norm).unwrap_or(serde_json::Value::Null),
                    serde_json::to_value(reference_position).unwrap_or(serde_json::Value::Null),
                ]),
            }),
            data: None,
        });
    }
    let elapsed_ms = started_at.elapsed().as_millis();
    if state.perf_logging_enabled && elapsed_ms >= 10 {
        info!(
            target: "spec42_core::lsp_runtime::symbols",
            event = "symbols:buildCodeLens",
            uri = %uri_norm,
            indexed_symbols,
            emitted_lenses = out.len(),
            elapsed_ms,
            "build_code_lens completed"
        );
    }
    out
}
