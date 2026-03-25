use crate::lsp::types::ServerState;
use tower_lsp::lsp_types::Url;

pub(crate) fn indexed_text(state: &ServerState, uri_norm: &Url) -> Option<String> {
    state.index.get(uri_norm).map(|e| e.content.clone())
}
