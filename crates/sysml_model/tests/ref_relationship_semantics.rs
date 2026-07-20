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
    let (graph, _parsed) = build_semantic_graph_from_documents(&[defs.clone(), usage.clone()])
        .expect("semantic graph");

    let ref_node = graph
        .nodes_named("importedTree")
        .into_iter()
        .find(|node| node.element_kind == "ref")
        .expect("cross-file ref usage node");
    let typing_targets = graph.outgoing_typing_or_specializes_targets(ref_node);

    assert!(
        typing_targets.iter().any(|target| {
            target.name == "Tree" && target.element_kind == "part def" && target.id.uri == defs.uri
        }),
        "expected cross-file typing target in defs document, got: {:?}",
        typing_targets
            .iter()
            .map(|target| (
                &target.id.qualified_name,
                &target.element_kind,
                &target.id.uri
            ))
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
    ref part centralBody : CelestialBody;
    ref part orbitingBody : CelestialBody;
  }
  part system {
    part sun : CelestialBody;
    part earth : CelestialBody;
    part earthOrbit : Orbit {
      ref part centralBody = sun;
      ref part orbitingBody : CelestialBody = earth;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let central_body_ref = graph
        .nodes_named("centralBody")
        .into_iter()
        .find(|node| node.element_kind == "ref" && node.attributes.contains_key("value"))
        .expect("centralBody assigned ref node");
    let orbiting_body_ref = graph
        .nodes_named("orbitingBody")
        .into_iter()
        .find(|node| node.element_kind == "ref" && node.attributes.contains_key("value"))
        .expect("orbitingBody assigned ref node");

    let typing_targets = graph.outgoing_typing_or_specializes_targets(orbiting_body_ref);
    assert!(
        typing_targets
            .iter()
            .any(|target| target.name == "CelestialBody" && target.element_kind == "part def"),
        "expected typed ref assignment to resolve CelestialBody typing, got: {:?}",
        typing_targets
            .iter()
            .map(|target| (&target.id.qualified_name, &target.element_kind))
            .collect::<Vec<_>>()
    );

    let edges = graph.edges_for_workspace_as_strings(&[]);
    assert!(
        edges.iter().any(|(src, tgt, kind, _)| {
            src == &central_body_ref.id.qualified_name
                && tgt.ends_with("system::sun")
                && *kind == RelationshipKind::Reference
        }),
        "expected centralBody reference edge to sun, got: {edges:#?}"
    );
    assert!(
        edges.iter().any(|(src, tgt, kind, _)| {
            src == &orbiting_body_ref.id.qualified_name
                && tgt.ends_with("system::earth")
                && *kind == RelationshipKind::Reference
        }),
        "expected orbitingBody reference edge to earth, got: {edges:#?}"
    );
}

#[test]
fn part_def_ref_assignment_creates_reference_edge() {
    let doc = workspace_doc(
        "part_def_ref_assignment.sysml",
        r#"package Astronomy {
  part def CelestialBody;
  part def OrbitTemplate {
    part sun : CelestialBody;
    ref part centralBody : CelestialBody = sun;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let central_body_ref = graph
        .nodes_named("centralBody")
        .into_iter()
        .find(|node| node.element_kind == "ref" && node.attributes.contains_key("value"))
        .expect("assigned ref in part def");
    let edges = graph.edges_for_workspace_as_strings(&[]);

    assert!(
        edges.iter().any(|(src, tgt, kind, _)| {
            src == &central_body_ref.id.qualified_name
                && tgt.ends_with("OrbitTemplate::sun")
                && *kind == RelationshipKind::Reference
        }),
        "expected reference edge from part-def ref assignment, got: {edges:#?}"
    );
}

#[test]
fn part_usage_ref_redefinition_shorthand_creates_reference_edge() {
    let doc = workspace_doc(
        "part_usage_ref_redefinition_shorthand.sysml",
        r#"package Astronomy {
  part def CelestialBody;
  part def Orbit {
    ref part centralBody : CelestialBody;
  }
  part system {
    part sun : CelestialBody;
    part earthOrbit : Orbit {
      ref part :>> centralBody = sun;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let central_body_ref = graph
        .nodes_named("centralBody")
        .into_iter()
        .find(|node| node.element_kind == "ref" && node.attributes.contains_key("value"))
        .expect("assigned ref with :>> shorthand");
    let edges = graph.edges_for_workspace_as_strings(&[]);

    assert!(
        edges.iter().any(|(src, tgt, kind, _)| {
            src == &central_body_ref.id.qualified_name
                && tgt.ends_with("system::sun")
                && *kind == RelationshipKind::Reference
        }),
        "expected reference edge from shorthand redefinition ref assignment, got: {edges:#?}"
    );
}

#[test]
fn multi_target_ref_typing_emits_an_edge_per_target() {
    // S42-004: `sysml-v2-parser` 0.45.0 stopped collapsing a comma-separated `:` typing clause
    // to a single joined display string on `ref` declarations (`RefDecl.typing` now carries
    // every target). This asserts spec42 actually wires an edge for each one, not just the
    // first, mirroring `AttributeDef`/`AttributeUsage`'s existing `typing_targets` consumption.
    //
    // Uses an action-def-body `ref` (`action_ref_decl`), the one `RefDecl` call site the
    // parser fix made multi-target-aware for a bare `:` clause. The other ad hoc call sites
    // (`part_ref_usage`, `connection.rs`/`interface.rs::ref_decl`, `state.rs::state_ref`) still
    // only parse a single `qualified_name` after `:` -- deliberately out of scope for that
    // release, no confirmed real multi-target usage there.
    let doc = workspace_doc(
        "multi_target_ref.sysml",
        r#"package P {
  part def A;
  part def B;
  action def C {
    ref multi : A, B;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let ref_node = graph
        .nodes_named("multi")
        .into_iter()
        .find(|node| node.element_kind == "ref")
        .expect("multi-target ref node");
    let typing_targets = graph.outgoing_typing_or_specializes_targets(ref_node);

    for expected in ["A", "B"] {
        assert!(
            typing_targets
                .iter()
                .any(|target| target.name == expected && target.element_kind == "part def"),
            "expected typing edge to {expected}, got targets: {:?}",
            typing_targets
                .iter()
                .map(|target| (&target.id.qualified_name, &target.element_kind))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn action_ref_redefines_then_multi_target_typing_wires_both() {
    // Regression test for the Systems Library `Actions.sysml` bug found while auditing
    // S42-004 against real usage: `ref NAME :>> redefinesTarget: Type1, Type2 { ... }` inside
    // an action def body previously discarded the redefines target *and* the entire typing
    // clause as unparsed text once `:>>` was seen (sysml-v2-parser's old `action_ref_decl`).
    // Shape mirrors `Actions.sysml`'s `SendAction.sentMessage`/`AcceptMessageAction.
    // acceptedMessage` (`ref sentMessage :>> sentTransfer: MessageTransfer, MessageAction`).
    let doc = workspace_doc(
        "action_ref_redefines_typing.sysml",
        r#"package P {
  part def MessageTransfer;
  part def MessageAction;
  action def BaseAction {
    ref transfer : MessageTransfer;
  }
  action def DerivedAction :> BaseAction {
    ref message :>> transfer: MessageTransfer, MessageAction;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let ref_node = graph
        .nodes_named("message")
        .into_iter()
        .find(|node| node.element_kind == "ref")
        .expect("redefining ref node");
    let typing_targets = graph.outgoing_typing_or_specializes_targets(ref_node);

    for expected in ["MessageTransfer", "MessageAction"] {
        assert!(
            typing_targets
                .iter()
                .any(|target| target.name == expected && target.element_kind == "part def"),
            "expected typing edge to {expected}, got targets: {:?}",
            typing_targets
                .iter()
                .map(|target| (&target.id.qualified_name, &target.element_kind))
                .collect::<Vec<_>>()
        );
    }

    let edges = graph.edges_for_workspace_as_strings(&[]);
    assert!(
        edges.iter().any(|(src, tgt, kind, _)| {
            src == &ref_node.id.qualified_name
                && tgt.ends_with("transfer")
                && *kind == RelationshipKind::Redefinition
        }),
        "expected Redefinition edge from 'message' to 'transfer', got: {edges:#?}"
    );
}

#[test]
fn part_def_and_part_usage_ref_assignments_both_emit_reference_edges() {
    let doc = workspace_doc(
        "ref_parity.sysml",
        r#"package P {
  part def Body;
  part def Template {
    part anchor : Body;
    ref part linkInDef : Body = anchor;
  }
  part instance {
    part anchor : Body;
    part usageBox {
      ref part linkInUsage : Body = anchor;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let edges = graph.edges_for_workspace_as_strings(&[]);

    for ref_name in ["linkInDef", "linkInUsage"] {
        let ref_node = graph
            .nodes_named(ref_name)
            .into_iter()
            .find(|node| node.element_kind == "ref" && node.attributes.contains_key("value"))
            .unwrap_or_else(|| panic!("expected assigned ref '{ref_name}'"));
        assert!(
            edges.iter().any(|(src, tgt, kind, _)| {
                src == &ref_node.id.qualified_name
                    && tgt.contains("anchor")
                    && *kind == RelationshipKind::Reference
            }),
            "expected Reference edge for '{ref_name}', got: {edges:#?}"
        );
    }
}
