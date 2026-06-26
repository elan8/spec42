use sysml_model::{SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind};
use workspace::{
    EngineBuilder, HostContext, HostResourceLimits, InMemoryDocumentProvider, WorkspaceLoadRequest,
};
use tempfile::tempdir;
use url::Url;

struct TwoDocumentProvider;

impl SysmlDocumentProvider for TwoDocumentProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        let first = SysmlDocument::from_memory_path(
            "workspace",
            "A.sysml",
            "package A { part def One; }".to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("first");
        let second = SysmlDocument::from_memory_path(
            "workspace",
            "B.sysml",
            "package B { part def Two; }".to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("second");
        Ok(vec![first, second])
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
    let document = SysmlDocument {
        uri: Url::from_file_path(&target).expect("uri"),
        content: format!("package L {{ part def Big {{ attribute value : String = \"{large}\"; }} }}"),
        path_hint: Some("Large.sysml".to_string()),
        source_kind: SysmlDocumentSourceKind::Workspace,
        sha256: None,
        byte_size: None,
    };

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
            InMemoryDocumentProvider::new(vec![document]),
            WorkspaceLoadRequest::single_target(target),
            context,
        )
        .expect_err("expected byte limit");

    assert_eq!(err.code(), "resource_limit_exceeded");
    assert!(err.to_string().contains("max_total_bytes"));
}
