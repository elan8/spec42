use std::sync::Arc;

use workspace::{
    EngineBuilder, HostContext, HostWorkspaceSnapshot, InMemoryDocumentProvider,
    Spec42Engine, SysmlDocument, SysmlDocumentSourceKind, WorkspaceLoadRequest,
};
use tempfile::TempDir;
use url::Url;

pub fn test_engine(cache: &TempDir) -> Spec42Engine {
    EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .build()
        .expect("engine")
}

pub fn memory_document(path: &std::path::Path, content: &str) -> SysmlDocument {
    let uri = Url::from_file_path(path).expect("file uri");
    SysmlDocument {
        uri,
        content: content.to_string(),
        path_hint: path
            .file_name()
            .map(|name| name.to_string_lossy().replace('\\', "/")),
        source_kind: SysmlDocumentSourceKind::Workspace,
        sha256: None,
        byte_size: None,
    }
}

pub fn load_snapshot(
    engine: &Spec42Engine,
    cache: &TempDir,
    filename: &str,
    content: &str,
) -> Arc<HostWorkspaceSnapshot> {
    let model_path = cache.path().join(filename);
    std::fs::write(&model_path, content).expect("write model file");
    let document = memory_document(&model_path, content);
    let provider = InMemoryDocumentProvider::new(vec![document]);
    engine
        .load_workspace(
            provider,
            WorkspaceLoadRequest::single_target(model_path),
            HostContext::default(),
        )
        .expect("snapshot")
}
