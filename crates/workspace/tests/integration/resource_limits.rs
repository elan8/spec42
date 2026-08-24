use sysml_query::source::{SourceAuthority, SourceError, SourceLoadReport, SourceService};
use tempfile::tempdir;
use url::Url;
use workspace::{
    EngineBuilder, HostContext, HostResourceLimits, InMemoryProvider, SourceKind, SourceProvider,
    WorkspaceLoadRequest,
};

struct TwoDocumentProvider;

impl SourceProvider for TwoDocumentProvider {
    fn load(&self, authority: &SourceAuthority) -> Result<SourceLoadReport, SourceError> {
        let first = authority
            .admit_memory(
                "workspace",
                "A.sysml",
                "package A { part def One; }",
                SourceKind::Workspace,
            )
            .expect("first");
        let second = authority
            .admit_memory(
                "workspace",
                "B.sysml",
                "package B { part def Two; }",
                SourceKind::Workspace,
            )
            .expect("second");
        Ok(SourceLoadReport {
            documents: vec![first, second],
            ..SourceLoadReport::default()
        })
    }
}

#[test]
fn max_documents_limit_rejects_oversized_workspace() {
    let cache = tempdir().expect("tempdir");
    let target = cache.path().join("A.sysml");
    std::fs::write(&target, "package A { part def One; }").expect("write");

    let engine = EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .build()
        .expect("engine");

    let context = HostContext::default().with_limits(HostResourceLimits {
        max_documents: Some(1),
        ..HostResourceLimits::default()
    });

    let err = engine
        .load_workspace(
            TwoDocumentProvider,
            WorkspaceLoadRequest::single_target(target),
            context,
        )
        .expect_err("expected resource limit");

    assert_eq!(err.code(), "resource_limit_exceeded");
    assert!(err.to_string().contains("max_documents"));
}

#[test]
fn max_total_bytes_limit_rejects_large_content() {
    let cache = tempdir().expect("tempdir");
    let target = cache.path().join("Large.sysml");
    std::fs::write(&target, "package L { part def Big; }").expect("write");

    let large = "x".repeat(2048);
    let document = SourceService::new()
        .admit_url(
            Url::from_file_path(&target).expect("uri"),
            &format!("package L {{ part def Big {{ attribute value : String = \"{large}\"; }} }}"),
            SourceKind::Workspace,
        )
        .with_path_hint("Large.sysml");

    let engine = EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .build()
        .expect("engine");

    let context = HostContext::default().with_limits(HostResourceLimits {
        max_total_bytes: Some(512),
        ..HostResourceLimits::default()
    });

    let err = engine
        .load_workspace(
            InMemoryProvider::new(vec![document]),
            WorkspaceLoadRequest::single_target(target),
            context,
        )
        .expect_err("expected byte limit");

    assert_eq!(err.code(), "resource_limit_exceeded");
    assert!(err.to_string().contains("max_total_bytes"));
}
