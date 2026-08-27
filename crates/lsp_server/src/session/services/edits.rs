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

/// Commits an already-computed parse result and replaces the immutable publication.
pub(crate) fn apply_parsed_document_update(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    version: i32,
    document: sysml_query::source::SourceDocument,
    parsed: sysml_query::syntax::ParsedSource,
    _parse_time_ms: u32,
    _evaluate: bool,
) -> Vec<(MessageType, String)> {
    let mut warnings = Vec::new();
    let Some(entry) = state.index_mut().get_mut(uri_norm) else {
        return warnings;
    };
    let diagnostic_count = parsed.diagnostics().len();
    entry.document = document;
    entry.parsed = parsed;
    if diagnostic_count > 0 {
        warnings.push((
            MessageType::LOG,
            format!(
                "sysml parse for editor produced {diagnostic_count} diagnostic(s) after didChange for {uri_norm} (version {version})."
            ),
        ));
    }
    warnings
}

pub(crate) fn remove_document(state: &mut impl DocumentStore, uri_norm: &Url) {
    state.index_mut().remove(uri_norm);
    state
        .recovery_search_symbols_mut()
        .retain(|entry| entry.uri != *uri_norm);
}
