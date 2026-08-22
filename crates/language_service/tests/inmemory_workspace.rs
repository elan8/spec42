use language_service::{InMemoryWorkspace, WorkspaceSnapshot};
use std::sync::Arc;

use sysml_query::resolved_slice::{
    build, BuildRequest, ConstructionStrategy, SourceDocument, SourceKind,
};
use sysml_source::{SysmlDocument, SysmlDocumentSourceKind};

#[test]
fn inmemory_workspace_indexes_an_injected_publication_and_symbols() {
    let doc = SysmlDocument::from_memory_path(
        "workspace",
        "Demo.sysml",
        "package Demo { part def Thing {} }".to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("doc");
    let source =
        SourceDocument::from_uri(doc.uri.as_str(), doc.content.clone(), SourceKind::Workspace)
            .expect("source");
    let request =
        BuildRequest::resolved(vec![source], ConstructionStrategy::Sequential).expect("request");
    let publication = Arc::new(build(request).expect("publication"));
    let workspace = InMemoryWorkspace::from_documents_and_publication(vec![doc], publication)
        .expect("workspace");
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
