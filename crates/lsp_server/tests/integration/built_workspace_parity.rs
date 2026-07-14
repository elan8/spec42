use std::path::PathBuf;
use std::sync::Arc;

use lsp_server::{
    default_server_config, semantic_report_from_built_workspace, validate_paths_with_semantics,
    BuiltWorkspaceInput, ValidationRequest,
};
use sysml_model::{
    build_semantic_graph_with_provider, FileSystemDocumentProvider, SysmlDocumentProvider,
};

#[test]
fn built_workspace_report_matches_full_validation_pipeline() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model_path = temp.path().join("RiskTrace.sysml");
    std::fs::write(
        &model_path,
        r#"
package RiskTrace {
    part def RiskControlMeasure;
    part control : RiskControlMeasure;
    requirement verifiedControl;
    satisfy verifiedControl by control;
}
"#,
    )
    .expect("write model");

    let request = ValidationRequest {
        targets: vec![model_path.clone()],
        workspace_root: None,
        library_paths: Vec::new(),
        parallel_enabled: false,
        strict_diagnostics: false,
    };
    let config = Arc::new(default_server_config());

    let full = validate_paths_with_semantics(&config, request.clone()).expect("full report");

    let provider = FileSystemDocumentProvider::new(
        model_path.clone(),
        model_path.parent().map(PathBuf::from),
        Vec::new(),
    );
    let (semantic_graph, parsed_documents) =
        build_semantic_graph_with_provider(&provider).expect("graph");
    let all_documents = provider.load_documents().expect("load documents");

    let built = BuiltWorkspaceInput {
        semantic_graph,
        all_documents,
        parsed_documents,
        library_urls: Vec::new(),
        workspace_root: model_path.parent().map(PathBuf::from),
    };
    let adapted =
        semantic_report_from_built_workspace(&config, &built, request).expect("built report");

    assert_eq!(
        full.validation.summary.document_count,
        adapted.validation.summary.document_count
    );
    assert_eq!(
        full.validation.summary.error_count,
        adapted.validation.summary.error_count
    );
    assert_eq!(
        full.validation.summary.warning_count,
        adapted.validation.summary.warning_count
    );
    assert_eq!(
        full.validation.summary.information_count,
        adapted.validation.summary.information_count
    );
    assert_eq!(
        full.validation.documents.len(),
        adapted.validation.documents.len()
    );
    for (left, right) in full
        .validation
        .documents
        .iter()
        .zip(adapted.validation.documents.iter())
    {
        assert_eq!(left.uri, right.uri);
        assert_eq!(left.diagnostics.len(), right.diagnostics.len());
        for (l_diag, r_diag) in left.diagnostics.iter().zip(right.diagnostics.iter()) {
            assert_eq!(l_diag.message, r_diag.message);
            assert_eq!(l_diag.code, r_diag.code);
            assert_eq!(l_diag.severity, r_diag.severity);
        }
    }
    assert_eq!(full.semantic_model.nodes, adapted.semantic_model.nodes);
    assert_eq!(
        full.semantic_model.relationships,
        adapted.semantic_model.relationships
    );
}

/// Regression test: a document with a hard syntax error is dropped from
/// `build_semantic_graph_with_provider`'s `parsed_documents`, so
/// `BuiltWorkspaceInput::all_documents` must still carry its raw text or the document's
/// index entry vanishes entirely and `spec42 check` reports zero diagnostics instead of a
/// parse error.
#[test]
fn built_workspace_reports_hard_parse_error() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model_path = temp.path().join("Bad.sysml");
    std::fs::write(&model_path, "package P { } }\n").expect("write model");

    let request = ValidationRequest {
        targets: vec![model_path.clone()],
        workspace_root: None,
        library_paths: Vec::new(),
        parallel_enabled: false,
        strict_diagnostics: false,
    };
    let config = Arc::new(default_server_config());

    let provider = FileSystemDocumentProvider::new(
        model_path.clone(),
        model_path.parent().map(PathBuf::from),
        Vec::new(),
    );
    let (semantic_graph, parsed_documents) =
        build_semantic_graph_with_provider(&provider).expect("graph");
    assert!(
        parsed_documents.is_empty(),
        "expected the hard parse error to be dropped from parsed_documents"
    );
    let all_documents = provider.load_documents().expect("load documents");
    assert_eq!(all_documents.len(), 1);

    let built = BuiltWorkspaceInput {
        semantic_graph,
        all_documents,
        parsed_documents,
        library_urls: Vec::new(),
        workspace_root: model_path.parent().map(PathBuf::from),
    };
    let adapted =
        semantic_report_from_built_workspace(&config, &built, request).expect("built report");

    assert_eq!(adapted.validation.documents.len(), 1);
    assert!(
        !adapted.validation.documents[0].diagnostics.is_empty(),
        "expected a parse-error diagnostic for the malformed document, got none"
    );
}
