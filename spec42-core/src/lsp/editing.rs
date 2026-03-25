use crate::lsp::types::ServerState;
use tower_lsp::lsp_types::Url;

pub(crate) fn indexed_text_for_uri(state: &ServerState, uri_norm: &Url) -> String {
    state
        .index
        .get(uri_norm)
        .map(|e| e.content.as_str())
        .unwrap_or("")
        .to_string()
}
