use super::*;

/// Applies LSP changes to an owned prospective text value without publishing it.
pub(crate) fn apply_content_changes(
    current: &str,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> (String, bool, Vec<(MessageType, String)>) {
    let mut warnings = Vec::new();
    let mut content = current.to_string();
    for change in content_changes {
        if let Some(range) = change.range {
            if let Some(new_text) = util::apply_incremental_change(&content, &range, &change.text) {
                content = new_text;
            } else {
                warnings.push((
                    MessageType::WARNING,
                    format!(
                        "didChange: ignored invalid incremental edit for {} at {}:{}..{}:{} (version {}).",
                        uri_norm, range.start.line, range.start.character,
                        range.end.line, range.end.character, version,
                    ),
                ));
            }
        } else {
            content = change.text;
        }
    }
    let changed = content != current;
    (content, changed, warnings)
}

pub(crate) fn remove_document(state: &mut impl DocumentStore, uri_norm: &Url) {
    state.index_mut().remove(uri_norm);
    state
        .recovery_search_symbols_mut()
        .retain(|entry| entry.uri != *uri_norm);
}
