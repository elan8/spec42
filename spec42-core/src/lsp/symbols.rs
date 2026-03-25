use crate::lsp::types::ServerState;
use tower_lsp::lsp_types::{
    CodeLens, Command, InlayHint, InlayHintKind, InlayHintLabel, Position, Url,
};

pub(crate) fn build_inlay_hints(state: &ServerState, uri_norm: &Url) -> Vec<InlayHint> {
    let mut hints = Vec::new();
    for sym in state.symbol_table.iter().filter(|s| s.uri == *uri_norm) {
        if let Some(sig) = &sym.signature {
            if let Some((_, rhs)) = sig.split_once(':') {
                hints.push(InlayHint {
                    position: Position::new(sym.range.end.line, sym.range.end.character),
                    label: InlayHintLabel::String(format!(" :{}", rhs.trim_end_matches(';').trim())),
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
    for sym in state.symbol_table.iter().filter(|s| s.uri == *uri_norm) {
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
