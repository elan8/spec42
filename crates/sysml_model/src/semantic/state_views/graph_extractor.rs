//! Graph-driven state-machine extraction from the workspace `SemanticGraph`.

use std::collections::{HashMap, HashSet};

use url::Url;

use crate::semantic::dto::{PositionDto, RangeDto};
use crate::semantic::extracted_model::{
    RegionDto, StateMachineDto, StateNodeDto, StateNodeElementDto, StateTransitionDto,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, NodeId, RelationshipKind, SemanticNode};
use crate::semantic::text_span::TextRange;

fn text_range_to_dto(range: TextRange) -> RangeDto {
    RangeDto {
        start: PositionDto {
            line: range.start.line,
            character: range.start.character,
        },
        end: PositionDto {
            line: range.end.line,
            character: range.end.character,
        },
    }
}

pub(super) fn extract_state_machines(
    graph: &SemanticGraph,
    workspace_uris: &[Url],
) -> Vec<StateMachineDto> {
    let mut machines = Vec::new();
    let mut seen_roots: HashSet<NodeId> = HashSet::new();

    for uri in workspace_uris {
        for node in graph.nodes_for_uri(uri) {
            if !is_state_machine_root(graph, node) {
                continue;
            }
            if !seen_roots.insert(node.id.clone()) {
                continue;
            }
            if let Some(machine) = build_machine(graph, node) {
                if !machine.states.is_empty() {
                    machines.push(machine);
                }
            }
        }
    }

    machines.sort_by(|left, right| {
        right
            .states
            .len()
            .cmp(&left.states.len())
            .then_with(|| right.transitions.len().cmp(&left.transitions.len()))
            .then_with(|| left.name.cmp(&right.name))
    });
    machines
}

fn is_state_behavior_element(kind: &crate::ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::State | ElementKind::FinalState | ElementKind::Transition
    )
}

fn is_state_machine_root(graph: &SemanticGraph, node: &SemanticNode) -> bool {
    node.element_kind == ElementKind::StateDef
        && graph
            .children_of(node)
            .iter()
            .any(|child| is_state_behavior_element(&child.element_kind))
}

fn build_machine(graph: &SemanticGraph, root: &SemanticNode) -> Option<StateMachineDto> {
    let initial_targets = initial_state_targets(graph, root);
    let mut state_nodes: Vec<StateNodeDto> = Vec::new();
    let mut state_ids: HashSet<String> = HashSet::new();

    collect_state_nodes(graph, root, None, &mut state_nodes, &mut state_ids);

    if state_nodes.is_empty() {
        return None;
    }

    let transitions = collect_transitions(graph, root, &state_ids, &initial_targets);
    apply_initial_state_kinds(&mut state_nodes, &transitions, &initial_targets);
    let regions = build_regions(&state_nodes);
    assign_region_ids(&mut state_nodes, &regions);

    Some(StateMachineDto {
        id: root.id.qualified_name.clone(),
        name: root.name.clone(),
        label: String::new(),
        package_path: package_path_for(graph, root),
        uri: Some(root.id.uri.as_str().to_string()),
        states: state_nodes,
        transitions,
        regions,
        range: text_range_to_dto(root.range),
    })
}

fn normalized_type_name(type_name: &str) -> String {
    type_name
        .split("::")
        .last()
        .unwrap_or(type_name)
        .replace([' ', '_'], "")
        .to_lowercase()
}

fn is_terminate_state(node: &SemanticNode) -> bool {
    attr_str(node, "stateType")
        .map(|value| normalized_type_name(&value))
        .is_some_and(|value| value == "terminate")
}

fn build_regions(states: &[StateNodeDto]) -> Vec<RegionDto> {
    states
        .iter()
        .filter(|state| state.kind == "composite")
        .map(|state| RegionDto {
            id: format!("{}::region_1", state.id),
            name: "1".to_string(),
            parent_id: Some(state.id.clone()),
        })
        .collect()
}

fn assign_region_ids(states: &mut [StateNodeDto], regions: &[RegionDto]) {
    for state in states.iter_mut() {
        let Some(parent_id) = state.parent_id.clone() else {
            continue;
        };
        if let Some(region) = regions
            .iter()
            .find(|region| region.parent_id.as_deref() == Some(parent_id.as_str()))
        {
            state.region_id = Some(region.id.clone());
        }
    }
}

fn initial_state_targets(graph: &SemanticGraph, root: &SemanticNode) -> HashSet<String> {
    let mut targets = HashSet::new();
    for (target, kind) in graph.outgoing_relationships(root) {
        if kind == RelationshipKind::InitialState {
            targets.insert(target.id.qualified_name.clone());
        }
    }
    for child in graph.children_of(root) {
        if child.element_kind != ElementKind::Transition {
            continue;
        }
        if child
            .attributes
            .get("isInitial")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            if let Some(source) = attr_str(child, "source") {
                if let Some(resolved) =
                    resolve_state_id_in_machine(graph, root, &source, &HashSet::new())
                {
                    targets.insert(resolved);
                }
            }
        }
    }
    targets
}

fn apply_initial_state_kinds(
    states: &mut [StateNodeDto],
    transitions: &[StateTransitionDto],
    initial_targets: &HashSet<String>,
) {
    if initial_targets.is_empty() {
        return;
    }
    let incoming_count = transitions
        .iter()
        .fold(HashMap::new(), |mut counts, transition| {
            *counts.entry(transition.target.clone()).or_insert(0) += 1;
            counts
        });
    if let Some(initial) = states
        .iter_mut()
        .filter(|state| initial_targets.contains(&state.id))
        .min_by_key(|state| {
            (
                incoming_count.get(&state.id).copied().unwrap_or(0),
                state.name.clone(),
            )
        })
    {
        initial.kind = "initial".to_string();
    }
}

fn collect_state_nodes(
    graph: &SemanticGraph,
    machine_root: &SemanticNode,
    parent: Option<&SemanticNode>,
    out: &mut Vec<StateNodeDto>,
    state_ids: &mut HashSet<String>,
) {
    let parent_node = parent.unwrap_or(machine_root);
    for child in graph.children_of(parent_node) {
        match child.element_kind {
            ElementKind::State | ElementKind::FinalState => {
                let id = child.id.qualified_name.clone();
                if !state_ids.insert(id.clone()) {
                    continue;
                }
                let has_nested_states = graph
                    .children_of(child)
                    .iter()
                    .any(|node| matches!(node.element_kind, ElementKind::State | ElementKind::FinalState));
                let kind = if child.element_kind == ElementKind::FinalState {
                    "final".to_string()
                } else if is_terminate_state(child) {
                    "terminate".to_string()
                } else if has_nested_states {
                    "composite".to_string()
                } else {
                    "state".to_string()
                };
                let entry = compartment_action(graph, child, "entry");
                let do_action = compartment_action(graph, child, "do");
                let exit = compartment_action(graph, child, "exit");
                let element_type = if child.element_kind == ElementKind::FinalState {
                    "final state"
                } else {
                    "state"
                }
                .to_string();
                out.push(StateNodeDto {
                    id: id.clone(),
                    name: child.name.clone(),
                    kind,
                    parent_id: parent
                        .filter(|node| node.id != machine_root.id)
                        .map(|node| node.id.qualified_name.clone()),
                    region_id: parent
                        .filter(|node| node.id != machine_root.id)
                        .map(|node| node.id.qualified_name.clone()),
                    entry,
                    do_action,
                    exit,
                    element: StateNodeElementDto {
                        id: id.clone(),
                        name: child.name.clone(),
                        element_type,
                        uri: Some(child.id.uri.as_str().to_string()),
                        range: text_range_to_dto(child.range),
                    },
                });
                collect_state_nodes(graph, machine_root, Some(child), out, state_ids);
            }
            _ => {}
        }
    }
}

fn compartment_action(
    graph: &SemanticGraph,
    state: &SemanticNode,
    compartment_kind: &str,
) -> Option<String> {
    for child in graph.children_of(state) {
        if child.element_kind != ElementKind::Action
            || attr_str(child, "compartment").as_deref() != Some(compartment_kind)
        {
            continue;
        }
        if let Some(action_name) = attr_str(child, "actionName") {
            if !action_name.trim().is_empty() {
                return Some(action_name);
            }
        }
        return Some(child.name.clone());
    }
    None
}

fn collect_transitions(
    graph: &SemanticGraph,
    root: &SemanticNode,
    state_ids: &HashSet<String>,
    _initial_targets: &HashSet<String>,
) -> Vec<StateTransitionDto> {
    let mut transitions = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    collect_transition_nodes(graph, root, root, state_ids, &mut transitions, &mut seen);
    transitions.sort_by(|left, right| left.id.cmp(&right.id));
    transitions
}

fn collect_transition_nodes(
    graph: &SemanticGraph,
    machine_root: &SemanticNode,
    parent: &SemanticNode,
    state_ids: &HashSet<String>,
    out: &mut Vec<StateTransitionDto>,
    seen: &mut HashSet<String>,
) {
    for child in graph.children_of(parent) {
        if child.element_kind == ElementKind::Transition {
            if let Some(transition) = transition_from_node(graph, machine_root, child, state_ids) {
                if seen.insert(transition.id.clone()) {
                    out.push(transition);
                }
            }
        }
        collect_transition_nodes(graph, machine_root, child, state_ids, out, seen);
    }
}

fn transition_from_node(
    graph: &SemanticGraph,
    machine_root: &SemanticNode,
    transition: &SemanticNode,
    state_ids: &HashSet<String>,
) -> Option<StateTransitionDto> {
    let source_raw = attr_str(transition, "source")?;
    let target_raw = attr_str(transition, "target")?;
    let source = resolve_state_id_in_machine(graph, machine_root, &source_raw, state_ids)?;
    let target = resolve_state_id_in_machine(graph, machine_root, &target_raw, state_ids)?;
    if !state_ids.contains(&source) || !state_ids.contains(&target) {
        return None;
    }

    let guard = attr_str(transition, "guardExpression");
    let effect = attr_str(transition, "effectExpression");
    let accept = attr_str(transition, "acceptName")
        .or_else(|| attr_str(transition, "acceptExpression"))
        .or_else(|| attr_str(transition, "payloadName"));
    let send = attr_str(transition, "sendName")
        .or_else(|| attr_str(transition, "sendExpression"))
        .or_else(|| effect.as_deref().and_then(send_from_effect_expression));
    let label = transition_label(
        transition,
        guard.as_deref(),
        accept.as_deref(),
        send.as_deref(),
        effect.as_deref(),
    );

    Some(StateTransitionDto {
        id: transition.id.qualified_name.clone(),
        source: source.clone(),
        target: target.clone(),
        name: Some(transition.name.clone()),
        label: Some(label),
        guard,
        effect,
        accept,
        send,
        self_loop: source == target,
        uri: Some(transition.id.uri.as_str().to_string()),
        range: text_range_to_dto(transition.range),
    })
}

fn send_from_effect_expression(effect: &str) -> Option<String> {
    let trimmed = effect.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("send ") {
        let payload = trimmed[5..].trim();
        if !payload.is_empty() {
            return Some(payload.to_string());
        }
    }
    None
}

fn transition_label(
    transition: &SemanticNode,
    guard: Option<&str>,
    accept: Option<&str>,
    send: Option<&str>,
    effect: Option<&str>,
) -> String {
    let name = transition.name.trim();
    if !name.is_empty() && !name.starts_with("transition_") {
        return name.to_string();
    }
    let mut parts = Vec::new();
    if let Some(guard) = guard.filter(|value| !value.trim().is_empty()) {
        parts.push(format!("[{guard}]"));
    }
    if let Some(effect) = effect.filter(|value| !value.trim().is_empty()) {
        if send.is_none() || !effect.to_ascii_lowercase().starts_with("send ") {
            parts.push(effect.to_string());
        }
    }
    if let Some(accept) = accept.filter(|value| !value.trim().is_empty()) {
        parts.push(format!("accept {accept}"));
    }
    if let Some(send) = send.filter(|value| !value.trim().is_empty()) {
        parts.push(format!("send {send}"));
    }
    if !parts.is_empty() {
        return parts.join(" / ");
    }
    name.to_string()
}

fn resolve_state_id_in_machine(
    graph: &SemanticGraph,
    machine_root: &SemanticNode,
    reference: &str,
    state_ids: &HashSet<String>,
) -> Option<String> {
    if state_ids.contains(reference) {
        return Some(reference.to_string());
    }
    if let Some(node) = graph.get_node(&NodeId::new(&machine_root.id.uri, reference)) {
        let id = node.id.qualified_name.clone();
        if state_ids.is_empty() || state_ids.contains(&id) {
            return Some(id);
        }
    }
    let suffix = reference.rsplit("::").next().unwrap_or(reference);
    let mut matches: Vec<String> = if state_ids.is_empty() {
        let mut discovered = Vec::new();
        walk_machine_states(graph, machine_root, |state| {
            if state.name == suffix || state.id.qualified_name.ends_with(&format!("::{suffix}")) {
                discovered.push(state.id.qualified_name.clone());
            }
        });
        discovered
    } else {
        state_ids
            .iter()
            .filter(|id| {
                id.rsplit("::").next() == Some(suffix) || id.ends_with(&format!("::{suffix}"))
            })
            .cloned()
            .collect()
    };
    matches.sort_by_key(|id| id.len());
    matches.into_iter().next()
}

fn walk_machine_states<F>(graph: &SemanticGraph, root: &SemanticNode, mut visit: F)
where
    F: FnMut(&SemanticNode),
{
    fn walk<F>(graph: &SemanticGraph, parent: &SemanticNode, visit: &mut F)
    where
        F: FnMut(&SemanticNode),
    {
        for child in graph.children_of(parent) {
            if matches!(child.element_kind, ElementKind::State | ElementKind::FinalState) {
                visit(child);
                walk(graph, child, visit);
            }
        }
    }
    walk(graph, root, &mut visit);
}

fn attr_str(node: &SemanticNode, key: &str) -> Option<String> {
    node.attributes
        .get(key)
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
        .filter(|value| !value.trim().is_empty())
}

fn package_path_for(graph: &SemanticGraph, root: &SemanticNode) -> String {
    let mut segments = Vec::new();
    for ancestor in graph.ancestors_of(root) {
        if matches!(&ancestor.element_kind, ElementKind::Package)
            || matches!(&ancestor.element_kind, ElementKind::Unknown(s) if s == "library package")
        {
            segments.push(ancestor.name.clone());
        }
    }
    segments.reverse();
    segments
        .into_iter()
        .filter(|segment| !segment.trim().is_empty())
        .collect::<Vec<_>>()
        .join("::")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::workspace_graph::build_semantic_graph_from_documents;
    use crate::SysmlDocument;
    use crate::SysmlDocumentSourceKind;

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
    fn extracts_transitions_with_accept_and_initial_state() {
        let content = include_str!("../../../tests/fixtures/parser_wave/final-state.sysml");
        let doc = workspace_doc("final-state.sysml", content);
        let uri = doc.uri.clone();
        let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let machines = extract_state_machines(&graph, &[uri]);
        let machine = machines
            .iter()
            .find(|machine| machine.name == "DoneStates")
            .expect("DoneStates machine");
        assert_eq!(machine.states.len(), 2);
        assert!(
            machine.states.iter().all(|state| state.kind == "final"),
            "fixture only declares final states; states={:?}",
            machine
                .states
                .iter()
                .map(|state| (&state.name, &state.kind))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn extracts_timer_state_machine_demo() {
        let content = r#"package StateMachineDemo {
    action def StartPressed;
    state def Idle;
    state def Running;
    state def TimerStateMachine {
        state idle : Idle;
        state running : Running;
        transition to_running first idle accept StartPressed then running;
    }
}"#;
        let doc = workspace_doc("timer.sysml", content);
        let uri = doc.uri.clone();
        let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let machines = extract_state_machines(&graph, &[uri]);
        let machine = machines
            .iter()
            .find(|machine| machine.name == "TimerStateMachine")
            .expect("TimerStateMachine");
        assert_eq!(machine.states.len(), 2);
        assert!(
            machine.transitions.iter().any(|transition| {
                transition.accept.as_deref() == Some("StartPressed")
                    && transition.label.as_deref() == Some("to_running")
            }),
            "expected accept trigger on transition; transitions={:?}",
            machine.transitions
        );
    }
}
