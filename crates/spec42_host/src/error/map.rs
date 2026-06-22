//! Map internal `String` errors from dependencies into structured host errors.

use super::Spec42HostError;

pub(crate) fn map_provider_error(message: String) -> Spec42HostError {
    if looks_like_parse_failure(&message) {
        return Spec42HostError::parser_failure(None::<String>, message);
    }
    Spec42HostError::unresolved_library_environment(message)
}

pub(crate) fn map_graph_error(message: String) -> Spec42HostError {
    if looks_like_parse_failure(&message) {
        return Spec42HostError::parser_failure(None::<String>, message);
    }
    Spec42HostError::internal_invariant_failure(message)
}

pub(crate) fn map_view_error(view: &str, message: String) -> Spec42HostError {
    let lowered = message.to_ascii_lowercase();
    if lowered.contains("unsupported")
        || lowered.contains("view")
        || lowered.contains("renderer")
        || lowered.contains("empty state")
    {
        return Spec42HostError::unsupported_view(view, message);
    }
    Spec42HostError::unsupported_view(view, message)
}

pub(crate) fn map_language_service_error(message: String) -> Spec42HostError {
    if looks_like_parse_failure(&message) {
        return Spec42HostError::parser_failure(None::<String>, message);
    }
    Spec42HostError::internal_invariant_failure(message)
}

pub(crate) fn map_render_snapshot_error(message: String) -> Spec42HostError {
    Spec42HostError::internal_invariant_failure(message)
}

fn looks_like_parse_failure(message: &str) -> bool {
    let lowered = message.to_ascii_lowercase();
    lowered.contains("parse")
        || lowered.contains("syntax")
        || lowered.contains("parser")
        || lowered.contains("failed to read")
}
