use language_service::{InMemoryWorkspace, WorkspaceSnapshot};
use semantic_core::{
    InMemoryDocumentProvider, SysmlDocument, SysmlDocumentSourceKind,
};

#[test]
fn inmemory_workspace_builds_graph_and_symbols() {
    let doc = SysmlDocument::from_memory_path(
        "workspace",
        "Demo.sysml",
        "package Demo { part def Thing {} }".to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("doc");
    let provider = InMemoryDocumentProvider::new(vec![doc]);
    let workspace = InMemoryWorkspace::from_provider(&provider).expect("workspace");
    assert!(!workspace.index_uris().is_empty());
    assert!(!workspace.symbol_table().is_empty());
    assert!(
        workspace
            .symbol_table()
            .iter()
            .any(|entry| entry.name == "Thing"),
        "expected Thing symbol"
    );
}
