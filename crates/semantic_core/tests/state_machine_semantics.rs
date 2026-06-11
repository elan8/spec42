use semantic_core::{
    build_semantic_graph_from_documents, build_workspace_state_machines, SysmlDocument,
    SysmlDocumentSourceKind,
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
