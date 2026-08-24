use language_service::{InMemoryWorkspace, WorkspaceSnapshot};
use sysml_query::{source::SourceKind, Services};

#[test]
fn inmemory_workspace_indexes_an_injected_publication_and_symbols() {
    let services = Services::new();
    let doc = services
        .source
        .admit_memory(
            "workspace",
            "Demo.sysml",
            "package Demo { part def Thing {} }",
            SourceKind::Workspace,
        )
        .expect("doc");
    let publication = services
        .publication
        .publish(std::slice::from_ref(&doc), [])
        .expect("publication");
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
