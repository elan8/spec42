use super::*;

pub(crate) fn apply_document_content_edit(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> (bool, Vec<(MessageType, String)>) {
    let mut runtime_warnings = Vec::new();
    let content_changed = if let Some(entry) = state.index_mut().get_mut(uri_norm) {
        let mut content_changed = false;
        for change in content_changes {
            if let Some(range) = change.range {
                if let Some(new_text) =
                    util::apply_incremental_change(&entry.content, &range, &change.text)
                {
                    if new_text != entry.content {
                        entry.content = new_text;
                        content_changed = true;
                    }
                } else {
                    runtime_warnings.push((
                        MessageType::WARNING,
                        format!(
                            "didChange: ignored invalid incremental edit for {} at {}:{}..{}:{} (version {}).",
                            uri_norm,
                            range.start.line,
                            range.start.character,
                            range.end.line,
                            range.end.character,
                            version,
                        ),
                    ));
                }
            } else if entry.content != change.text {
                entry.content = change.text;
                content_changed = true;
            }
        }
        content_changed
    } else {
        runtime_warnings.push((
            MessageType::WARNING,
            format!(
                "didChange: document {} was not in the server index (version {}). Change was ignored until a full open/watch refresh occurs.",
                uri_norm, version
            ),
        ));
        false
    };
    (content_changed, runtime_warnings)
}

/// Applies an already-computed parse result (produced off the write lock, e.g.
/// via `spawn_blocking`) to the document and incrementally patches the
/// semantic graph/symbol table for that URI. This is the potentially-slow
/// half of a document update — callers should compute `parsed_result` without
/// holding the server's write lock so a slow parse of malformed/incomplete
/// syntax can't stall every other request.
pub(crate) fn apply_parsed_document_update(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    version: i32,
    parsed_result: sysml_v2_parser::ParseResult,
    parse_time_ms: u32,
    evaluate: bool,
) -> Vec<(MessageType, String)> {
    let mut runtime_warnings = Vec::new();
    let Some(entry) = state.index_mut().get_mut(uri_norm) else {
        return runtime_warnings;
    };
    entry.parsed = Some(parsed_result.root);
    entry.parse_metadata = ParseMetadata {
        parse_time_ms,
        parse_cached: false,
    };
    if !parsed_result.errors.is_empty() {
        runtime_warnings.push((
            MessageType::LOG,
            format!(
                "sysml parse_for_editor produced {} diagnostic(s) after didChange for {} (version {}).",
                parsed_result.errors.len(),
                uri_norm,
                version
            ),
        ));
    }

    let parsed = state
        .index()
        .get(uri_norm)
        .and_then(|entry| entry.parsed.as_ref())
        .cloned();
    update_semantic_graph_for_uri(state, uri_norm, parsed.as_ref(), evaluate);
    refresh_symbols_for_uri(state, uri_norm);

    runtime_warnings
}

#[cfg(test)]
fn apply_document_changes_impl(
    state: &mut ServerState,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
    evaluate: bool,
) -> Vec<(MessageType, String)> {
    let (content_changed, mut runtime_warnings) =
        apply_document_content_edit(state, uri_norm, version, content_changes);

    if content_changed {
        let content = state
            .index
            .get(uri_norm)
            .map(|entry| entry.content.clone())
            .unwrap_or_default();
        let parse_start = Instant::now();
        let parsed_result = util::parse_for_editor(&content);
        let parse_time_ms = elapsed_ms(parse_start);
        runtime_warnings.extend(apply_parsed_document_update(
            state,
            uri_norm,
            version,
            parsed_result,
            parse_time_ms,
            evaluate,
        ));
    }

    runtime_warnings
}

#[cfg(test)]
pub(crate) fn apply_document_changes(
    state: &mut ServerState,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> Vec<(MessageType, String)> {
    apply_document_changes_impl(state, uri_norm, version, content_changes, true)
}

#[cfg(test)]
pub(crate) fn apply_document_changes_fast(
    state: &mut ServerState,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> Vec<(MessageType, String)> {
    apply_document_changes_impl(state, uri_norm, version, content_changes, false)
}

pub(crate) fn remove_document(state: &mut impl DocumentStore, uri_norm: &Url) {
    state.index_mut().remove(uri_norm);
    state.symbol_table_mut().retain(|entry| entry.uri != *uri_norm);
    state.semantic_graph_mut().remove_nodes_for_uri(uri_norm);
}

