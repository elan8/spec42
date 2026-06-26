use workspace::WorkspaceError;

#[test]
fn host_error_codes_are_stable() {
    assert_eq!(
        WorkspaceError::invalid_document_uri("bad").code(),
        "invalid_document_uri"
    );
    assert_eq!(
        WorkspaceError::parser_failure(Some("file:///x"), "parse").code(),
        "parser_failure"
    );
    assert_eq!(
        WorkspaceError::unresolved_library_environment("missing").code(),
        "unresolved_library_environment"
    );
    assert_eq!(
        WorkspaceError::unsupported_view("general-view", "nope").code(),
        "unsupported_view"
    );
    assert_eq!(WorkspaceError::cancelled().code(), "cancelled");
    assert_eq!(
        WorkspaceError::resource_limit_exceeded("max_documents", "too many").code(),
        "resource_limit_exceeded"
    );
    assert_eq!(
        WorkspaceError::internal_invariant_failure("bug").code(),
        "internal_invariant_failure"
    );
}

#[test]
fn host_errors_display_includes_code_prefix() {
    let err = WorkspaceError::unsupported_view("general-view", "missing graph");
    let rendered = err.to_string();
    assert!(rendered.contains("unsupported_view"));
    assert!(rendered.contains("general-view"));
}
