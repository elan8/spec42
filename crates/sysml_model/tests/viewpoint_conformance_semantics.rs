use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
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
fn view_usage_satisfy_viewpoint_def_creates_satisfy_edge() {
    let doc = workspace_doc(
        "viewpoint_ok.sysml",
        r#"package Demo {
  part def System;
  viewpoint def ArchitectureViewpoint;
  view def StructuralView;
  view structure : StructuralView {
    expose Demo::System;
  }
  satisfy structure by ArchitectureViewpoint;
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let edges = graph.edges_for_workspace_as_strings(&[]);
    assert!(
        edges.iter().any(|(source, target, kind, _)| {
            *kind == RelationshipKind::Satisfy
                && source.ends_with("structure")
                && target.ends_with("ArchitectureViewpoint")
        }),
        "expected view->viewpoint satisfy edge, got: {edges:#?}"
    );
}

#[test]
fn cross_file_viewpoint_conformance_resolves_after_workspace_merge() {
    let defs = workspace_doc(
        "defs.sysml",
        r#"package Defs {
  part def System;
  viewpoint def ArchitectureViewpoint;
  view def StructuralView;
}"#,
    );
    let usage = workspace_doc(
        "usage.sysml",
        r#"package Usage {
  import Defs::*;
  view structure : StructuralView {
    expose Defs::System;
  }
  satisfy structure by ArchitectureViewpoint;
}"#,
    );
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[defs, usage.clone()]).expect("semantic graph");
    let usage_uri = usage.uri;
    let edges = graph.edges_for_uri_as_strings(&usage_uri);
    assert!(
        edges.iter().any(|(source, target, kind, _)| {
            *kind == RelationshipKind::Satisfy
                && source.ends_with("structure")
                && target.ends_with("ArchitectureViewpoint")
        }),
        "expected cross-file satisfy edge in usage document, got: {edges:#?}"
    );
}

#[test]
fn unresolved_viewpoint_target_emits_specific_diagnostic() {
    let doc = workspace_doc(
        "viewpoint_missing.sysml",
        r#"package Demo {
  view def StructuralView;
  view structure : StructuralView;
  satisfy structure by MissingViewpoint;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "unresolved_viewpoint_conformance_target"),
        "expected unresolved_viewpoint_conformance_target, got: {:?}",
        diagnostics
            .iter()
            .map(|diagnostic| (&diagnostic.code, &diagnostic.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn non_viewpoint_satisfy_target_emits_conformance_kind_diagnostic() {
    let doc = workspace_doc(
        "viewpoint_invalid_target.sysml",
        r#"package Demo {
  requirement def SystemRequirement;
  view def StructuralView;
  view structure : StructuralView;
  satisfy structure by SystemRequirement;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "viewpoint_conformance_invalid_target_kind"),
        "expected viewpoint_conformance_invalid_target_kind, got: {:?}",
        diagnostics
            .iter()
            .map(|diagnostic| (&diagnostic.code, &diagnostic.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn astronomy_fixture_view_conforms_to_viewpoint() {
    let content = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../lsp_server/tests/fixtures/astronomy_viewpoint_conformance.sysml"),
    )
    .expect("read astronomy viewpoint fixture");
    let doc = workspace_doc("astronomy_viewpoint_conformance.sysml", &content);
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let edges = graph.edges_for_workspace_as_strings(&[]);
    assert!(
        edges.iter().any(|(source, target, kind, _)| {
            *kind == RelationshipKind::Satisfy
                && source.ends_with("solarSystemStructure")
                && target.ends_with("OrbitalStructureViewpoint")
        }),
        "expected astronomy view->viewpoint satisfy edge, got: {edges:#?}"
    );
}

#[test]
fn imported_view_and_viewpoint_types_resolve_from_nested_namespace() {
    let defs = workspace_doc(
        "defs_nested.sysml",
        r#"package Defs {
  package Views {
    viewpoint def ArchitectureViewpoint;
    view def StructuralView;
  }
}"#,
    );
    let usage = workspace_doc(
        "usage_nested.sysml",
        r#"package Usage {
  import Defs::Views::*;
  view structure : StructuralView {
    expose Usage;
  }
  satisfy structure by ArchitectureViewpoint;
}"#,
    );
    let usage_uri = usage.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[defs, usage]).expect("semantic graph");
    let diagnostics =
        collect_diagnostics_from_graph(&graph, &usage_uri, DiagnosticsOptions::default());

    assert!(
        diagnostics.iter().all(|diagnostic| {
            diagnostic.code != "unresolved_type_reference"
                && diagnostic.code != "unresolved_viewpoint_conformance_target"
        }),
        "expected imported view/viewpoint references to resolve from nested namespace, got: {:?}",
        diagnostics
            .iter()
            .map(|diagnostic| (&diagnostic.code, &diagnostic.message))
            .collect::<Vec<_>>()
    );
}
