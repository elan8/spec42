//! Deferred structural-validation regressions from the compatibility corpus.
//!
//! These regressions assert the graph facts required by their structural checks.

use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, resolve_inherited_member_via_type, ElementKind,
    ResolveResult, SysmlDocument, SysmlDocumentSourceKind,
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
fn connection_like_definition_ends_share_one_positional_graph_fact() {
    let doc = workspace_doc(
        "connection_like_ends.sysml",
        r#"package P {
  occurrence def Occurrence;
  connection def Connector {
    end feature source : Occurrence;
    end feature target : Occurrence;
  }
  flow def Transfer {
    end feature source : Occurrence;
    end feature target : Occurrence;
  }
  allocation def Allocation {
    end feature source : Occurrence;
    end feature target : Occurrence;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    for (name, kind) in [
        ("Connector", ElementKind::ConnectionDef),
        ("Transfer", ElementKind::FlowDef),
        ("Allocation", ElementKind::AllocationDef),
    ] {
        let owner = graph
            .nodes_named(name)
            .into_iter()
            .find(|node| node.element_kind == kind)
            .unwrap_or_else(|| panic!("missing {name} {kind}"));
        let names: Vec<_> = graph
            .positional_end_features(owner)
            .into_iter()
            .map(|end| end.name.as_str())
            .collect();
        assert_eq!(names, ["source", "target"], "unexpected ends for {name}");
    }
    assert!(
        !collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default())
            .iter()
            .any(|diagnostic| diagnostic.code == "incomplete_connection_like_end_pair"),
        "complete connection-like definitions must not receive the incomplete-pair diagnostic"
    );
}

#[test]
fn flow_payload_occurrence_retains_its_resolved_type() {
    let source = r#"
package P {
  attribute def Scalar;
  part def Source;
  part def Target;
  flow transfer of Scalar from Source to Target;
}

"#;
    let doc = workspace_doc("flow_payload.sysml", source);
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let flow = graph
        .nodes_named("transfer")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Flow)
        .expect("named flow usage");
    let payload = graph
        .children_of(flow)
        .into_iter()
        .find(|node| node.element_kind == ElementKind::FlowPayload)
        .expect("payload feature");

    assert_eq!(
        payload.declared_facts.relationships.typing_display(),
        Some("Scalar"),
        "the authored payload type must remain a semantic fact"
    );
    assert!(
        graph
            .outgoing_typing_or_specializes_targets(payload)
            .iter()
            .any(|target| target.name == "Scalar"
                && target.element_kind == ElementKind::AttributeDef),
        "the payload feature must use the canonical typing relationship"
    );
}

#[test]
fn redefinition_resolves_through_its_featuring_type() {
    let source = r#"
package P {
  part def Vehicle { attribute mass : Real; }
  part def Car :> Vehicle { attribute mass :>> mass; }
}
"#;
    let doc = workspace_doc("redefinition_feature_type.sysml", source);
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let car = graph
        .nodes_named("Car")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::PartDef)
        .expect("Car definition");
    let mass = graph
        .children_of(car)
        .into_iter()
        .find(|node| node.name == "mass")
        .expect("redefining mass feature");

    assert_eq!(
        mass.declared_facts
            .relationships
            .redefinition
            .first()
            .map(|target| target.reference.as_str()),
        Some("mass"),
        "the authored redefinition must remain distinct from the resolved target"
    );
    let ResolveResult::Resolved(inherited_mass) =
        resolve_inherited_member_via_type(&graph, car, "mass")
    else {
        panic!("Car's inherited mass feature must resolve through Vehicle");
    };
    assert_eq!(inherited_mass.qualified_name, "P::Vehicle::mass");
}
