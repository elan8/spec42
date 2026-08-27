//! Provider-error mapping.

use super::WorkspaceError;
use sysml_query::source::SourceError;

pub(crate) fn map_provider_error(error: SourceError) -> WorkspaceError {
    match error {
        SourceError::Read { .. } | SourceError::NotUtf8 { .. } => {
            WorkspaceError::parser_failure(None::<String>, error.to_string())
        }
        SourceError::InvalidUri { .. } | SourceError::EmptyIdentity => {
            WorkspaceError::invalid_document_uri(error.to_string())
        }
        SourceError::PathNotFound { .. } | SourceError::NoSourcesFound => {
            WorkspaceError::unresolved_library_environment(error.to_string())
        }
        SourceError::Provider(message) => {
            if looks_like_parse_failure(&message) {
                WorkspaceError::parser_failure(None::<String>, message)
            } else {
                WorkspaceError::unresolved_library_environment(message)
            }
        }
    }
}

fn looks_like_parse_failure(message: &str) -> bool {
    let lowered = message.to_ascii_lowercase();
    lowered.contains("parse")
        || lowered.contains("syntax")
        || lowered.contains("parser")
        || lowered.contains("failed to read")
}
