use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind,
};

fn workspace_doc(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "workspace",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

#[test]
fn recursive_namespace_import_resolves_nested_member_types() {
    let defs = workspace_doc(
        "defs.sysml",
        r#"package Domain {
  package Bodies {
    part def CelestialBody;
  }
}"#,
    );
    let usage = workspace_doc(
        "usage.sysml",
        r#"package Consumer {
  import Domain::**;
  part def Orbit {
    ref primary : CelestialBody;
  }
}"#,
    );

    let usage_uri = usage.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[defs, usage]).expect("graph");
    let diagnostics =
        collect_diagnostics_from_graph(&graph, &usage_uri, DiagnosticsOptions::default());

    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "unresolved_ref_type_reference"),
        "expected recursive namespace import to resolve CelestialBody, got: {:?}",
        diagnostics
            .iter()
            .map(|diagnostic| (&diagnostic.code, &diagnostic.message))
            .collect::<Vec<_>>()
    );
}

