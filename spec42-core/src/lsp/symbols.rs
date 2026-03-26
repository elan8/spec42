use crate::lsp::types::ServerState;
use tower_lsp::lsp_types::{
    CodeLens, Command, InlayHint, InlayHintKind, InlayHintLabel, Position, Url,
};

pub(crate) fn build_inlay_hints(state: &ServerState, uri_norm: &Url) -> Vec<InlayHint> {
    let mut hints = Vec::new();
    let mut seen = std::collections::HashSet::<(u32, u32, String)>::new();
    for sym in state.symbol_table.iter().filter(|s| s.uri == *uri_norm) {
        if let Some(sig) = &sym.signature {
            if let Some((_, rhs)) = sig.split_once(':') {
                let label = format!(" :{}", rhs.trim_end_matches(';').trim());
                let key = (sym.range.end.line, sym.range.end.character, label.clone());
                if !seen.insert(key) {
                    continue;
                }
                hints.push(InlayHint {
                    position: Position::new(sym.range.end.line, sym.range.end.character),
                    label: InlayHintLabel::String(label),
                    kind: Some(InlayHintKind::TYPE),
                    text_edits: None,
                    tooltip: None,
                    padding_left: Some(true),
                    padding_right: Some(false),
                    data: None,
                });
            }
        }
    }
    hints
}

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
        let refs = state
            .index
            .values()
            .map(|e| crate::language::find_reference_ranges(&e.content, &sym.name).len())
            .sum::<usize>();
        out.push(CodeLens {
            range: sym.range,
            command: Some(Command {
                title: format!("{} reference(s)", refs.saturating_sub(1)),
                command: "spec42.showReferencesCount".to_string(),
                arguments: None,
            }),
            data: None,
        });
    }
    out
}
