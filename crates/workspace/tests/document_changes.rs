use url::Url;
use workspace::{apply_document_changes, DocumentChanges, SysmlDocumentSourceKind};

fn memory_doc(path: &str, content: &str) -> workspace::SysmlDocument {
    let uri = Url::parse(&format!("file://{path}")).expect("uri");
    workspace::SysmlDocument {
        uri,
        content: content.to_string(),
        path_hint: Some(
            std::path::Path::new(path)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        ),
        source_kind: SysmlDocumentSourceKind::Workspace,
        sha256: None,
        byte_size: None,
    }
}

#[test]
fn apply_document_changes_replaces_changed_uri() {
    let previous = vec![
        memory_doc("/tmp/A.sysml", "package A { part def One; }"),
        memory_doc("/tmp/B.sysml", "package B { part def Two; }"),
    ];
    let changes = DocumentChanges::new().with_changed(vec![memory_doc(
        "/tmp/A.sysml",
        "package A { part def Three; }",
    )]);

    let merged = apply_document_changes(&previous, &changes).expect("merge");
    assert_eq!(merged.len(), 2);
    assert!(merged.iter().any(|doc| doc.content.contains("Three")));
    assert!(!merged.iter().any(|doc| doc.content.contains("One")));
    assert!(merged.iter().all(|doc| doc.sha256.is_some()));
}

#[test]
fn apply_document_changes_adds_and_removes_documents() {
    let previous = vec![memory_doc("/tmp/A.sysml", "package A {}")];
    let removed = Url::parse("file:///tmp/A.sysml").unwrap();
    let changes = DocumentChanges::new()
        .with_removed(vec![removed])
        .with_added(vec![memory_doc("/tmp/C.sysml", "package C {}")]);

    let merged = apply_document_changes(&previous, &changes).expect("merge");
    assert_eq!(merged.len(), 1);
    assert!(merged[0].uri.path().ends_with("C.sysml"));
}

#[test]
fn duplicate_uri_across_buckets_is_rejected() {
    let doc = memory_doc("/tmp/A.sysml", "package A {}");
    let changes = DocumentChanges::new()
        .with_changed(vec![doc.clone()])
        .with_added(vec![doc]);

    let err = apply_document_changes(&[], &changes).expect_err("duplicate");
    assert_eq!(err.code(), "invalid_document_uri");
}
