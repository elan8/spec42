use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind};

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

fn child_with_kind<'a>(
    graph: &'a sysml_model::SemanticGraph,
    parent: &sysml_model::SemanticNode,
    kind: &str,
    name: &str,
) -> Option<&'a sysml_model::SemanticNode> {
    graph
        .children_of(parent)
        .into_iter()
        .find(|child| child.element_kind == kind && child.name == name)
}

#[test]
fn occurrence_def_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "occurrence.sysml",
        r#"package P {
  occurrence def Event {
    attribute id : String;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let occurrence_def = graph
        .nodes_named("Event")
        .into_iter()
        .find(|node| node.element_kind == "occurrence def")
        .expect("occurrence def");
    assert!(
        child_with_kind(&graph, occurrence_def, "attribute", "id").is_some(),
        "expected attribute child under occurrence def"
    );
}

#[test]
fn occurrence_def_body_materializes_nested_part_usage() {
    let doc = workspace_doc(
        "occurrence_part.sysml",
        r#"package P {
  part def Wheel;
  occurrence def Event {
    part wheel : Wheel;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let occurrence_def = graph
        .nodes_named("Event")
        .into_iter()
        .find(|node| node.element_kind == "occurrence def")
        .expect("occurrence def");
    assert!(
        child_with_kind(&graph, occurrence_def, "part", "wheel").is_some(),
        "expected nested part usage under occurrence def"
    );
}

#[test]
fn flow_def_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "flow.sysml",
        r#"package P {
  flow def Power {
    attribute rate : Real;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let flow_def = graph
        .nodes_named("Power")
        .into_iter()
        .find(|node| node.element_kind == "flow def")
        .expect("flow def");
    assert!(
        graph.children_of(flow_def).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "rate"
        }),
        "expected attribute child under flow def"
    );
}

#[test]
fn flow_def_body_materializes_nested_part_usage() {
    let doc = workspace_doc(
        "flow_part.sysml",
        r#"package P {
  part def Wheel;
  flow def PowerFlow {
    part wheel : Wheel;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let flow_def = graph
        .nodes_named("PowerFlow")
        .into_iter()
        .find(|node| node.element_kind == "flow def")
        .expect("flow def");
    assert!(
        child_with_kind(&graph, flow_def, "part", "wheel").is_some(),
        "expected nested part usage under flow def"
    );
}

#[test]
fn allocation_def_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "allocation.sysml",
        r#"package P {
  allocation def Map {
    attribute id : String;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let allocation_def = graph
        .nodes_named("Map")
        .into_iter()
        .find(|node| node.element_kind == "allocation def")
        .expect("allocation def");
    assert!(
        graph.children_of(allocation_def).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "id"
        }),
        "expected attribute child under allocation def"
    );
}

#[test]
fn flow_usage_brace_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "flow_usage.sysml",
        r#"package P {
  item def Payload;
  flow cargo : Payload {
    attribute weight : Real;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let flow_usage = graph
        .nodes_named("cargo")
        .into_iter()
        .find(|node| node.element_kind == "flow")
        .expect("flow usage");
    assert!(
        graph.children_of(flow_usage).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "weight"
        }),
        "expected attribute child under flow usage brace body"
    );
}

#[test]
fn occurrence_usage_brace_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "occurrence_usage.sysml",
        r#"package P {
  occurrence def Event;
  occurrence sample : Event {
    attribute id : String;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let occurrence_usage = graph
        .nodes_named("sample")
        .into_iter()
        .find(|node| node.element_kind == "occurrence")
        .expect("occurrence usage");
    assert!(
        graph.children_of(occurrence_usage).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "id"
        }),
        "expected attribute child under occurrence usage brace body"
    );
}

#[test]
fn rendering_def_body_materializes_filter_and_view_rendering() {
    let doc = workspace_doc(
        "rendering.sysml",
        r#"package P {
  rendering def DiagramStyle {
    filter true;
    render diagram : DiagramRenderer;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let rendering_def = graph
        .nodes_named("DiagramStyle")
        .into_iter()
        .find(|node| node.element_kind == "rendering def")
        .expect("rendering def");
    let children = graph.children_of(rendering_def);
    assert!(
        children.iter().any(|child| child.element_kind == "filter"),
        "expected filter child under rendering def"
    );
    assert!(
        children
            .iter()
            .any(|child| child.element_kind == "view rendering" && child.name == "diagram"),
        "expected view rendering child under rendering def"
    );
}

#[test]
fn use_case_def_body_materializes_first_succession_and_then_use_case() {
    let doc = workspace_doc(
        "use_case_body.sysml",
        r#"package P {
  use case def Mission {
    subject;
    first start;
    then use case step : StepType;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let use_case_def = graph
        .nodes_named("Mission")
        .into_iter()
        .find(|node| node.element_kind == "use case def")
        .expect("use case def");
    assert_eq!(
        use_case_def
            .attributes
            .get("hasSubject")
            .and_then(|v| v.as_bool()),
        Some(true)
    );
    assert_eq!(
        use_case_def
            .attributes
            .get("firstSuccessionTarget")
            .and_then(|v| v.as_str()),
        Some("start")
    );
    assert!(
        child_with_kind(&graph, use_case_def, "succession", "start").is_some(),
        "expected materialized start succession node"
    );
    assert!(
        graph
            .children_of(use_case_def)
            .iter()
            .any(|child| child.element_kind == "use case" && child.name == "step"),
        "expected then use case child"
    );
    assert!(
        graph.pending_relationships.is_empty(),
        "expected no pending relationships after first/then use case wiring"
    );
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !diagnostics
            .iter()
            .any(|d| d.code == "unresolved_pending_relationship"),
        "unexpected unresolved_pending_relationship: {:?}",
        diagnostics
            .iter()
            .filter(|d| d.code == "unresolved_pending_relationship")
            .collect::<Vec<_>>()
    );
}

#[test]
fn use_case_def_vacuum_succession_chain_resolves_flow_edges() {
    let doc = workspace_doc(
        "vacuum_use_case.sysml",
        r#"package P {
  use case def Vacuming {
    first start;
    then action doCheckSuctionChamber;
    then action doCheckBatteryCharge;
    then done;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let use_case_def = graph
        .nodes_named("Vacuming")
        .into_iter()
        .find(|node| node.element_kind == "use case def")
        .expect("use case def");

    assert!(
        child_with_kind(&graph, use_case_def, "succession", "start").is_some(),
        "expected start succession node"
    );
    assert!(child_with_kind(&graph, use_case_def, "action", "doCheckSuctionChamber").is_some());
    assert!(child_with_kind(&graph, use_case_def, "action", "doCheckBatteryCharge").is_some());
    assert!(child_with_kind(&graph, use_case_def, "verdict", "done").is_some());

    let has_edge = |source_suffix: &str, target_suffix: &str, kind: RelationshipKind| {
        graph
            .edges_for_uri_as_strings(&uri)
            .iter()
            .any(|(src, tgt, k, _)| {
                *k == kind && src.ends_with(source_suffix) && tgt.ends_with(target_suffix)
            })
    };
    assert!(
        has_edge("::Vacuming", "::start", RelationshipKind::Flow),
        "expected Flow from use case def to start"
    );
    assert!(
        has_edge("::start", "::doCheckSuctionChamber", RelationshipKind::Flow),
        "expected Flow from start to first action"
    );
    assert!(
        has_edge(
            "::doCheckSuctionChamber",
            "::doCheckBatteryCharge",
            RelationshipKind::Flow
        ),
        "expected Flow between then actions"
    );
    assert!(
        has_edge(
            "::doCheckBatteryCharge",
            "::_verdict",
            RelationshipKind::Flow
        ),
        "expected Flow from last action to done verdict"
    );
    assert!(graph.pending_relationships.is_empty());
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(!diagnostics
        .iter()
        .any(|d| d.code == "unresolved_pending_relationship"));
}

#[test]
fn analysis_def_body_materializes_ref_redefinition() {
    let doc = workspace_doc(
        "analysis_ref.sysml",
        r#"package P {
  analysis def PowerCheck {
    subject;
    ref :>> withinBudget {
      return true;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let analysis_def = graph
        .nodes_named("PowerCheck")
        .into_iter()
        .find(|node| node.element_kind == "analysis def")
        .expect("analysis def");
    assert!(
        graph.children_of(analysis_def).iter().any(|child| {
            child.element_kind == "ref redefinition" && child.name == "withinBudget"
        }),
        "expected ref redefinition child under analysis def"
    );
}

#[test]
fn occurrence_def_assert_constraint_materializes_child_node() {
    let doc = workspace_doc(
        "assert.sysml",
        r#"package P {
  occurrence def Checked {
    assert constraint {
      true;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let occurrence_def = graph
        .nodes_named("Checked")
        .into_iter()
        .find(|node| node.element_kind == "occurrence def")
        .expect("occurrence def");
    assert!(
        graph
            .children_of(occurrence_def)
            .iter()
            .any(|child| child.element_kind == "assert constraint"),
        "expected assert constraint child under occurrence def"
    );
}
