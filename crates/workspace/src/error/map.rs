//! Map internal `String` errors from dependencies into structured host errors.

use super::WorkspaceError;

pub(crate) fn map_provider_error(message: String) -> WorkspaceError {
    if looks_like_parse_failure(&message) {
        return WorkspaceError::parser_failure(None::<String>, message);
    }
    WorkspaceError::unresolved_library_environment(message)
}

fn looks_like_parse_failure(message: &str) -> bool {
    let lowered = message.to_ascii_lowercase();
    lowered.contains("parse")
        || lowered.contains("syntax")
        || lowered.contains("parser")
        || lowered.contains("failed to read")
}
