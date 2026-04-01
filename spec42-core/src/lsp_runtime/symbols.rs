use crate::workspace::ServerState;
use tower_lsp::lsp_types::{CodeLens, Command, Url};

pub(crate) fn build_code_lens(state: &ServerState, uri_norm: &Url) -> Vec<CodeLens> {
    let mut out = Vec::new();
    let mut seen_ranges = std::collections::HashSet::<(u32, u32, u32, u32)>::new();
    let mut sorted_symbols: Vec<_> = state
        .symbol_table
        .iter()
        .filter(|s| s.uri == *uri_norm)
        .collect();
    sorted_symbols.sort_by_key(|s| (s.range.start.line, s.range.start.character, s.name.clone()));
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
        let refs = crate::lsp_runtime::references_resolver::resolved_references_for_symbol(
            state, sym, false,
        )
        .len();
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
    out
}
