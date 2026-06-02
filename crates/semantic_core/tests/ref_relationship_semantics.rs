use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
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
fn in_scope_ref_usage_gets_typing_edge() {
    let doc = workspace_doc(
        "model.sysml",
        r#"package PartsTree {
  part def Tree;
  part def OrbitModel {
    ref sharedBranch : Tree;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let ref_node = graph
        .nodes_named("sharedBranch")
        .into_iter()
        .find(|node| node.element_kind == "ref")
        .expect("ref usage node");
    let typing_targets = graph.outgoing_typing_or_specializes_targets(ref_node);

    assert!(
        typing_targets.iter().any(|target| {
            target.name == "Tree"
                && target.element_kind == "part def"
                && graph
                    .edges_for_workspace_as_strings(&[])
                    .iter()
                    .any(|(src, tgt, kind, _)| {
                        src == &ref_node.id.qualified_name
                            && tgt == &target.id.qualified_name
                            && *kind == RelationshipKind::Typing
                    })
        }),
        "expected typing edge from ref usage to Tree, got targets: {:?}",
        typing_targets
            .iter()
            .map(|target| (&target.id.qualified_name, &target.element_kind))
            .collect::<Vec<_>>()
    );
}

#[test]
fn cross_file_ref_usage_resolves_after_workspace_merge() {
    let defs = workspace_doc(
        "defs.sysml",
        r#"package Domain {
  part def Tree;
}"#,
    );
    let usage = workspace_doc(
        "usage.sysml",
        r#"package Usage {
  import Domain::*;
  part def ImportedOrbitUsage {
    ref importedTree : Tree;
  }
}"#,
    );
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[defs.clone(), usage.clone()]).expect("semantic graph");

    let ref_node = graph
        .nodes_named("importedTree")
        .into_iter()
        .find(|node| node.element_kind == "ref")
        .expect("cross-file ref usage node");
    let typing_targets = graph.outgoing_typing_or_specializes_targets(ref_node);

    assert!(
        typing_targets.iter().any(|target| {
            target.name == "Tree"
                && target.element_kind == "part def"
                && target.id.uri == defs.uri
        }),
        "expected cross-file typing target in defs document, got: {:?}",
        typing_targets
            .iter()
            .map(|target| (&target.id.qualified_name, &target.element_kind, &target.id.uri))
            .collect::<Vec<_>>()
    );
}

#[test]
fn unresolved_ref_type_emits_specific_diagnostic() {
    let doc = workspace_doc(
        "broken_ref.sysml",
        r#"package Broken {
  part def BrokenOrbitUsage {
    ref unresolvedOrbitEndpoint : MissingOrbitBody;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());

    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "unresolved_ref_type_reference"),
        "expected unresolved ref-type diagnostic, got: {:?}",
        diagnostics
            .iter()
            .map(|diagnostic| (&diagnostic.code, &diagnostic.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn astronomy_orbit_pattern_uses_ref_relationships() {
    let doc = workspace_doc(
        "astronomy.sysml",
        r#"package Astronomy {
  part def CelestialBody;
  part def Orbit {
    ref centralBody : CelestialBody;
    ref orbitingBody : CelestialBody;
  }
  part sun : CelestialBody;
  part earth : CelestialBody;
  part earthOrbit : Orbit;
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let central_body_ref = graph
        .nodes_named("centralBody")
        .into_iter()
        .find(|node| node.element_kind == "ref")
        .expect("centralBody ref node");
    let orbiting_body_ref = graph
        .nodes_named("orbitingBody")
        .into_iter()
        .find(|node| node.element_kind == "ref")
        .expect("orbitingBody ref node");
    let ref_nodes = vec![central_body_ref, orbiting_body_ref];

    for ref_node in ref_nodes {
        let typing_targets = graph.outgoing_typing_or_specializes_targets(ref_node);
        assert!(
            typing_targets
                .iter()
                .any(|target| target.name == "CelestialBody" && target.element_kind == "part def"),
            "expected ref '{}' to resolve CelestialBody typing, got: {:?}",
            ref_node.id.qualified_name,
            typing_targets
                .iter()
                .map(|target| (&target.id.qualified_name, &target.element_kind))
                .collect::<Vec<_>>()
        );
    }
}
