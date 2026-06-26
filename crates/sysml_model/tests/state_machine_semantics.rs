use sysml_model::{
    build_semantic_graph_from_documents, build_workspace_state_machines,
    finalize_state_machines_for_response, SysmlDocument, SysmlDocumentSourceKind,
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
fn state_machine_demo_fixture_projects_states_and_transitions() {
    let content = include_str!("fixtures/parser_wave/state-machine-demo.sysml");
    let doc = workspace_doc("StateMachineDemo.sysml", content);
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let state_def_names: Vec<String> = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .filter(|node| node.element_kind == "state def")
        .map(|node| node.name.clone())
        .collect();
    assert!(
        state_def_names
            .iter()
            .any(|name| name == "TimerStateMachine"),
        "expected TimerStateMachine state def in graph, found {state_def_names:?}"
    );
    let machines = build_workspace_state_machines(&graph, &[uri]);
    let machine = machines
        .iter()
        .find(|machine| machine.name == "TimerStateMachine")
        .unwrap_or_else(|| {
            panic!(
                "TimerStateMachine not in extracted machines: {:?}",
                machines
                    .iter()
                    .map(|machine| (
                        &machine.name,
                        machine.states.len(),
                        machine.transitions.len()
                    ))
                    .collect::<Vec<_>>()
            );
        });
    assert_eq!(machine.states.len(), 4);
    assert!(
        machine.transitions.len() >= 6,
        "expected transition nodes for TimerStateMachine, got {}",
        machine.transitions.len()
    );
    assert!(
        machine
            .transitions
            .iter()
            .any(|transition| transition.accept.as_deref() == Some("StartPressed")),
        "expected accept trigger on at least one transition"
    );
}

#[test]
fn final_state_fixture_marks_initial_and_final_states() {
    let content = include_str!("fixtures/parser_wave/final-state.sysml");
    let doc = workspace_doc("final-state.sysml", content);
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let machines = build_workspace_state_machines(&graph, &[uri]);
    let machine = machines
        .iter()
        .find(|machine| machine.name == "DoneStates")
        .expect("DoneStates");
    assert_eq!(machine.states.len(), 2);
    assert!(
        machine.states.iter().all(|state| state.kind == "final"),
        "fixture only materializes final state nodes"
    );
}

#[test]
fn composite_state_machine_exposes_regions() {
    let content = r#"package Regions {
    state def CompositeMachine {
        state operating {
            state nominal;
            state fault;
        }
        transition start first nominal then fault;
    }
}"#;
    let doc = workspace_doc("regions.sysml", content);
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let machines = build_workspace_state_machines(&graph, &[uri]);
    let machine = machines
        .iter()
        .find(|machine| machine.name == "CompositeMachine")
        .expect("CompositeMachine");
    let composite = machine
        .states
        .iter()
        .find(|state| state.kind == "composite")
        .expect("composite state");
    assert!(
        machine
            .regions
            .iter()
            .any(|region| region.parent_id.as_deref() == Some(composite.id.as_str())),
        "expected explicit region for composite parent; regions={:?}",
        machine.regions
    );
    assert!(
        machine
            .states
            .iter()
            .filter(|state| state.parent_id.as_deref() == Some(composite.id.as_str()))
            .all(|state| state.region_id.is_some()),
        "nested states should reference a region id"
    );
}

#[test]
fn terminate_state_kind_is_distinct_from_final() {
    let content = r#"package TerminateDemo {
    state def Active;
    state def ShutdownMachine {
        state active : Active;
        state flowEnd : Terminate;
    }
}"#;
    let doc = workspace_doc("terminate.sysml", content);
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let machines = build_workspace_state_machines(&graph, &[uri]);
    let machine = machines
        .iter()
        .find(|machine| machine.name == "ShutdownMachine")
        .expect("ShutdownMachine");
    assert!(
        machine
            .states
            .iter()
            .any(|state| state.name == "flowEnd" && state.kind == "terminate"),
        "expected terminate kind for Terminate-typed state; states={:?}",
        machine
            .states
            .iter()
            .map(|state| (&state.name, &state.kind))
            .collect::<Vec<_>>()
    );
}

#[test]
fn exhibit_state_usage_does_not_duplicate_state_machine_selector_roots() {
    let content = r#"package ExhibitDemo {
    state def ModeMachine {
        state idle;
        state active;
        transition first idle then active;
    }
    part def Device {
        exhibit state deviceMode : ModeMachine {
            state idle;
            state active;
            transition first idle then active;
        }
    }
}"#;
    let doc = workspace_doc("exhibit-state.sysml", content);
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let machines = finalize_state_machines_for_response(build_workspace_state_machines(
        &graph,
        &[uri],
    ));
    assert_eq!(
        machines.len(),
        1,
        "expected one state-machine root (state def only), got: {:?}",
        machines
            .iter()
            .map(|machine| (&machine.name, &machine.id))
            .collect::<Vec<_>>()
    );
    assert_eq!(machines[0].name, "ModeMachine");
}

#[test]
fn finalized_state_machines_include_selector_labels() {
    let content = include_str!("fixtures/parser_wave/final-state.sysml");
    let doc = workspace_doc("final-state.sysml", content);
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let machines = finalize_state_machines_for_response(build_workspace_state_machines(
        &graph,
        &[uri],
    ));
    assert!(
        machines.iter().all(|machine| !machine.label.is_empty()),
        "expected selector labels on all machines: {:?}",
        machines
            .iter()
            .map(|machine| (&machine.name, &machine.label))
            .collect::<Vec<_>>()
    );
}
