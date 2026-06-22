use std::path::PathBuf;
use std::sync::Arc;

use kernel::{
    default_server_config, semantic_report_from_built_workspace, validate_paths_with_semantics,
    BuiltWorkspaceInput, ValidationRequest,
};
use semantic_core::{build_semantic_graph_with_provider, FileSystemDocumentProvider};

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

    let built = BuiltWorkspaceInput {
        semantic_graph,
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
