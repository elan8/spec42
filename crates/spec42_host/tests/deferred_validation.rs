#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{memory_document, test_engine};
use spec42_host::{
    HostContext, InMemoryDocumentProvider, ValidationTiming, WorkspaceLoadRequest,
};
use tempfile::tempdir;

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
    let provider = InMemoryDocumentProvider::new(vec![document.clone()]);

    let eager = engine
        .load_workspace(
            InMemoryDocumentProvider::new(vec![document.clone()]),
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
    assert_eq!(deferred.validation().summary.document_count, 0);

    let collected = deferred.ensure_validation().expect("ensure validation");
    assert_eq!(
        collected.summary.document_count,
        eager.validation().summary.document_count
    );
    assert_eq!(
        collected.summary.error_count,
        eager.validation().summary.error_count
    );
    assert_eq!(
        collected.summary.warning_count,
        eager.validation().summary.warning_count
    );
}
