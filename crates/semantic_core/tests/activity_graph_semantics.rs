use semantic_core::{
    build_semantic_graph_from_documents, enrich_activity_diagrams_from_graph,
    extract_activity_diagrams, SysmlDocument, SysmlDocumentSourceKind,
};
use sysml_v2_parser::parse;
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
fn enrich_then_action_chain_actions_have_graph_ranges() {
    let content = r#"package P {
  action def A;
  action def B;
  action def Pipeline {
    then action step1 : A;
    then action step2 : B;
  }
}"#;
    let doc = workspace_doc("then_chain.sysml", content);
    let uri = doc.uri.clone();
    let (graph, parsed_docs) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let mut diagrams = extract_activity_diagrams(&parsed_docs[0].parsed);
    for diagram in &mut diagrams {
        diagram.uri = Some(uri.as_str().to_string());
    }
    enrich_activity_diagrams_from_graph(&mut diagrams, &graph, &[uri.clone()]);

    let diagram = diagrams
        .iter()
        .find(|d| d.name == "Pipeline")
        .expect("pipeline diagram");

    for step in ["step1", "step2"] {
        let action = diagram
            .actions
            .iter()
            .find(|a| a.name == step)
            .unwrap_or_else(|| panic!("expected action '{step}'"));
        assert!(
            action.range.is_some(),
            "graph-enriched action '{step}' should carry a source range"
        );
        assert_eq!(
            action.uri.as_deref(),
            Some(uri.as_str()),
            "graph-enriched action '{step}' should carry document uri"
        );
    }

    assert!(
        diagram.flows.iter().any(|f| {
            f.guard.as_deref() == Some("flow") && f.from == "step1" && f.to == "step2"
        }),
        "expected Flow edge step1 -> step2; flows={:?}",
        diagram.flows
    );
    assert!(
        diagram
            .flows
            .iter()
            .any(|f| f.guard.as_deref() == Some("perform") && f.to == "step1"),
        "expected Perform edge to step1"
    );
}

#[test]
fn ast_only_first_then_synthesis_still_works_without_enrichment() {
    let input = r#"
            package P {
                action def ExecuteMission {
                    action validateRoute { out ok : Boolean; };
                    action startMission { out started : Boolean; };
                    first validateRoute then startMission;
                }
            }
        "#;
    let root = parse(input).expect("parse");
    let diagrams = extract_activity_diagrams(&root);
    let diagram = diagrams
        .iter()
        .find(|d| d.name == "ExecuteMission")
        .expect("diagram");
    assert!(diagram.actions.iter().any(|a| a.name == "validateRoute"));
    assert!(diagram.actions.iter().any(|a| a.name == "startMission"));
}

#[test]
fn enrich_does_not_promote_interface_parameters_to_action_steps() {
    let content = r#"package P {
  action def ExecutePatrol {
    in route : String;
    out status : String;
    action finishMission;
    bind status = finishMission;
  }
}"#;
    let doc = workspace_doc("iface.sysml", content);
    let uri = doc.uri.clone();
    let (graph, parsed_docs) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let mut diagrams = extract_activity_diagrams(&parsed_docs[0].parsed);
    for diagram in &mut diagrams {
        diagram.uri = Some(uri.as_str().to_string());
    }
    enrich_activity_diagrams_from_graph(&mut diagrams, &graph, &[uri]);

    let diagram = diagrams
        .iter()
        .find(|d| d.name == "ExecutePatrol")
        .expect("diagram");
    assert!(
        diagram
            .actions
            .iter()
            .all(|a| a.name != "route" && a.name != "status"),
        "interface parameters must not become action steps; actions={:?}",
        diagram.actions.iter().map(|a| &a.name).collect::<Vec<_>>()
    );
}

#[test]
fn ast_extract_includes_decision_merge_assign_and_conditional_succession() {
    let input = r#"package P {
  action def Route;
  action def Deliver;
  action def Pipeline {
    then action validate : Route;
    action checkRoute : Decision;
    then assign status := "ok";
    for item in items {
      then action deliver : Deliver;
    }
    merge validate;
    succession validate to checkRoute of status == "ok";
  }
}"#;
    let root = parse(input).expect("parse");
    let diagrams = extract_activity_diagrams(&root);
    let diagram = diagrams
        .iter()
        .find(|d| d.name == "Pipeline")
        .expect("diagram");
    assert!(
        diagram.decisions.iter().any(|d| d.name == "checkRoute"),
        "expected decision node; decisions={:?}",
        diagram.decisions
    );
    assert!(
        diagram.states.iter().any(|s| s.state_type == "merge"),
        "expected merge state"
    );
    assert!(
        diagram.states.iter().any(|s| s.state_type == "assign"),
        "expected assign state"
    );
    assert!(
        diagram.states.iter().any(|s| s.state_type == "for-loop"),
        "expected for-loop state"
    );
}

#[test]
fn enrich_control_nodes_from_graph_after_ast_extraction() {
    let content = r#"package P {
  action def Step;
  action def Pipeline {
    then action step1 : Step;
    action route : Decision;
    merge step1;
  }
}"#;
    let doc = workspace_doc("control.sysml", content);
    let uri = doc.uri.clone();
    let (graph, parsed_docs) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let mut diagrams = extract_activity_diagrams(&parsed_docs[0].parsed);
    for diagram in &mut diagrams {
        diagram.uri = Some(uri.as_str().to_string());
    }
    enrich_activity_diagrams_from_graph(&mut diagrams, &graph, &[uri]);

    let diagram = diagrams
        .iter()
        .find(|d| d.name == "Pipeline")
        .expect("pipeline");
    assert!(
        diagram.states.iter().any(|s| s.state_type == "decision"),
        "graph enrichment should surface decision control node; states={:?}",
        diagram.states
    );
    assert!(
        diagram.states.iter().any(|s| s.state_type == "merge"),
        "graph enrichment should surface merge control node"
    );
}
