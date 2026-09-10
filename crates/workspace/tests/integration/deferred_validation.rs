use crate::comparison_fixtures::{memory_document, test_engine};
use tempfile::tempdir;
use workspace::{
    HostContext, InMemoryProvider, SourceKind, ValidationTiming, WorkspaceLoadRequest,
};

const MODEL: &str = r#"
package Demo {
    part def Thing;
    part item : Thing;
}
"#;

#[test]
fn deferred_validation_matches_eager_after_ensure() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let model_path = cache.path().join("Demo.sysml");
    std::fs::write(&model_path, MODEL).expect("write model");
    let document = memory_document(&model_path, MODEL);
    let provider = InMemoryProvider::new(vec![document.clone()]);

    let eager = engine
        .load_workspace(
            InMemoryProvider::new(vec![document.clone()]),
            WorkspaceLoadRequest::single_target(model_path.clone())
                .with_validation_timing(ValidationTiming::Eager),
            HostContext::default(),
        )
        .expect("eager snapshot");

    let deferred = engine
        .load_workspace(
            provider,
            WorkspaceLoadRequest::single_target(model_path)
                .with_validation_timing(ValidationTiming::Deferred),
            HostContext::default(),
        )
        .expect("deferred snapshot");

    assert!(
        !deferred.validation_ready(),
        "deferred load should not collect validation eagerly"
    );
    assert!(matches!(
        deferred.validation(),
        workspace::ValidationState::Deferred
    ));

    let collected = deferred.ensure_validation().expect("ensure validation");
    assert_eq!(
        collected.summary.document_count,
        eager
            .ensure_validation()
            .expect("eager validation")
            .summary
            .document_count
    );
    assert_eq!(
        collected.summary.error_count,
        eager
            .ensure_validation()
            .expect("eager validation")
            .summary
            .error_count
    );
    assert_eq!(
        collected.summary.warning_count,
        eager
            .ensure_validation()
            .expect("eager validation")
            .summary
            .warning_count
    );
}

#[test]
fn explicitly_targeted_library_document_reports_diagnostics() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let model_path = cache.path().join("Library.sysml");
    let source = "package Library { part usage : MissingDefinition; }";
    std::fs::write(&model_path, source).expect("write library model");
    let uri = workspace::path_to_file_url(&model_path).expect("library URL");
    let document = engine.source().admit_url(uri, source, SourceKind::Library);

    let snapshot = engine
        .load_workspace(
            InMemoryProvider::new(vec![document]),
            WorkspaceLoadRequest::single_target(model_path)
                .with_validation_timing(ValidationTiming::Deferred),
            HostContext::default(),
        )
        .expect("deferred library snapshot");

    assert_eq!(snapshot.documents()[0].kind(), SourceKind::Library);
    let report = snapshot.ensure_validation().expect("library validation");
    assert_eq!(report.documents.len(), 1);
    assert!(
        report.documents[0]
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "unresolved_type_reference"),
        "an explicitly targeted library document must be part of the publication's diagnostic set: {:#?}",
        report.documents[0].diagnostics
    );
}
