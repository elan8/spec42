//! Action-flow, state, and sequence prepared views.

use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};

use crate::semantic::dto::{RangeDto, SysmlVisualizationResultDto};
use crate::semantic::extracted_model::{
    ActivityDiagramDto, ControlFlowDto, SequenceDiagramDto, StateMachineDto, StateNodeDto,
    StateTransitionDto,
};
use crate::semantic::prepared_view::dto::{PreparedEdgeDto, PreparedNodeDto, PreparedViewDto};
use crate::semantic::prepared_view::preparers::graph::prepare_graph_from_dto;

fn normalize_diagram_key(value: &str) -> String {
    value.replace("::", ".").trim().to_lowercase()
}

fn diagram_simple_name(value: &str) -> String {
    let normalized = value.replace("::", ".");
    normalized
        .split('.')
        .filter(|segment| !segment.is_empty())
        .last()
        .map(str::to_string)
        .unwrap_or(normalized)
}

fn diagram_matches_selection(
    diagram_id: &str,
    diagram_name: &str,
    package_path: Option<&str>,
    selected_name: Option<&str>,
    selected_view_id: Option<&str>,
) -> bool {
    let selectors: Vec<&str> = [selected_name, selected_view_id]
        .into_iter()
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .collect();
    if selectors.is_empty() {
        return false;
    }
    let candidates = [
        diagram_id.to_string(),
        diagram_name.to_string(),
        format!(
            "{}::{}",
            package_path.unwrap_or(""),
            diagram_name
        )
        .trim_start_matches("::")
        .to_string(),
    ];
    selectors.iter().any(|selector| {
        let selector_key = normalize_diagram_key(selector);
        let selector_simple = diagram_simple_name(selector).to_lowercase();
        candidates.iter().any(|candidate| {
            let candidate_key = normalize_diagram_key(candidate);
            let candidate_simple = diagram_simple_name(candidate).to_lowercase();
            candidate_key == selector_key
                || candidate_simple == selector_simple
                || candidate_key.ends_with(&format!(".{selector_key}"))
                || selector_key.ends_with(&format!(".{candidate_key}"))
                || candidate_key.contains(&selector_simple)
                || selector_key.contains(&candidate_simple)
        })
    })
}

fn select_activity_diagram<'a>(
    diagrams: &'a [ActivityDiagramDto],
    selected_name: Option<&str>,
    selected_view_id: Option<&str>,
) -> Option<&'a ActivityDiagramDto> {
    if diagrams.is_empty() {
        return None;
    }
    if let Some(matched) = diagrams.iter().find(|diagram| {
        diagram_matches_selection(
            &diagram.id,
            &diagram.name,
            Some(&diagram.package_path),
            selected_name,
            selected_view_id,
        )
    }) {
        return Some(matched);
    }
    if selected_name.is_none() && selected_view_id.is_none() {
        return None;
    }
    if diagrams.len() == 1 {
        return Some(&diagrams[0]);
    }
    None
}

fn best_activity_diagram(diagrams: &[ActivityDiagramDto]) -> Option<&ActivityDiagramDto> {
    diagrams.iter().max_by_key(|diagram| {
        let nodes = diagram.actions.len() + diagram.decisions.len() + diagram.states.len();
        let edges = diagram.flows.len();
        nodes * 10 + edges
    })
}

fn build_behavior_node(
    id: &str,
    label: String,
    kind: &str,
    uri: Option<String>,
    range: Option<RangeDto>,
    extra: Value,
) -> PreparedNodeDto {
    PreparedNodeDto {
        id: id.to_string(),
        label: label.to_string(),
        kind: kind.to_string(),
        source_path: uri.clone(),
        uri,
        range,
        attributes: Some(extra),
    }
}

fn normalize_activity_kind(kind: &str) -> &str {
    let kind = kind.to_lowercase();
    if kind.contains("perform") {
        "perform"
    } else if kind.contains("decision") {
        "decision"
    } else if kind.contains("merge") {
        "merge"
    } else if kind.contains("fork") {
        "fork"
    } else if kind.contains("join") {
        "join"
    } else if kind.contains("assign") {
        "assign"
    } else if kind.contains("for-loop") || kind.contains("forloop") {
        "for-loop"
    } else if kind.contains("terminate") {
        "terminate"
    } else if kind.contains("accept") {
        "accept"
    } else if kind.contains("send") {
        "send"
    } else if kind.contains("initial") {
        "initial"
    } else if kind.contains("final") {
        "final"
    } else {
        "action"
    }
}

fn collect_activity_nodes(diagram: &ActivityDiagramDto) -> Vec<PreparedNodeDto> {
    let allowed: HashSet<&str> = [
        "action", "perform", "assign", "for-loop", "decision", "merge", "fork", "join", "initial",
        "final", "terminate", "accept", "send",
    ]
    .into_iter()
    .collect();
    let mut nodes = Vec::new();
    for (index, decision) in diagram.decisions.iter().enumerate() {
        nodes.push(build_behavior_node(
            &format!("decision-{index}"),
            "Decision".to_string(),
            "decision",
            None,
            Some(to_prepared_range(&decision.range)),
            json!({ "name": decision.name, "condition": decision.condition }),
        ));
    }
    for (index, state) in diagram.states.iter().enumerate() {
        let kind = state.state_type.to_lowercase();
        if ![
            "initial", "final", "decision", "merge", "fork", "join", "assign", "for-loop",
            "terminate", "accept", "send",
        ]
        .iter()
        .any(|token| kind.contains(token))
        {
            continue;
        }
        nodes.push(build_behavior_node(
            &format!("state-{index}"),
            format!("State {}", index + 1),
            kind.as_str(),
            None,
            Some(to_prepared_range(&state.range)),
            json!({ "name": state.name, "type": state.state_type }),
        ));
    }
    for (index, action) in diagram.actions.iter().enumerate() {
        let kind = normalize_activity_kind(action.action_type.as_str());
        let mut attrs = json!({
            "name": action.name,
            "type": action.action_type,
        });
        if let Some(swim_lane) = action.swim_lane.as_ref() {
            attrs["swimLane"] = json!(swim_lane);
        }
        nodes.push(build_behavior_node(
            action
                .id
                .clone()
                .unwrap_or_else(|| format!("action-{index}"))
                .as_str(),
            if action.name.is_empty() {
                format!("Action {}", index + 1)
            } else {
                action.name.clone()
            },
            kind,
            action.uri.clone(),
            action.range.as_ref().map(to_prepared_range),
            attrs,
        ));
    }
    nodes
        .into_iter()
        .filter(|node| allowed.contains(node.kind.as_str()))
        .collect()
}

fn build_activity_node_alias_map(nodes: &[PreparedNodeDto]) -> HashMap<String, String> {
    let mut aliases = HashMap::new();
    let mut register = |alias: &str, node_id: &str| {
        let key = alias.trim();
        if key.is_empty() {
            return;
        }
        aliases.entry(key.to_string()).or_insert_with(|| node_id.to_string());
        let normalized = key.replace("::", ".");
        aliases
            .entry(normalized.clone())
            .or_insert_with(|| node_id.to_string());
        if let Some(last) = normalized.split('.').filter(|s| !s.is_empty()).last() {
            aliases
                .entry(last.to_string())
                .or_insert_with(|| node_id.to_string());
        }
    };
    for node in nodes {
        register(&node.id, &node.id);
        register(&node.label, &node.id);
        if let Some(qn) = node
            .attributes
            .as_ref()
            .and_then(|attrs| attrs.get("qualifiedName"))
            .and_then(|v| v.as_str())
        {
            register(qn, &node.id);
        }
    }
    aliases
}

fn resolve_activity_node_ref(value: &str, aliases: &HashMap<String, String>) -> String {
    let key = value.trim();
    if key.is_empty() {
        return String::new();
    }
    let normalized = key.replace("::", ".");
    let segments: Vec<&str> = normalized.split('.').filter(|s| !s.is_empty()).collect();
    let last = segments.last().copied().unwrap_or("");
    let first = segments.first().copied().unwrap_or("");
    aliases
        .get(key)
        .or_else(|| aliases.get(&normalized))
        .or_else(|| if last.is_empty() { None } else { aliases.get(last) })
        .or_else(|| if first.is_empty() { None } else { aliases.get(first) })
        .cloned()
        .unwrap_or_else(|| key.to_string())
}

fn activity_edges(diagram: &ActivityDiagramDto, nodes: &[PreparedNodeDto]) -> Vec<PreparedEdgeDto> {
    let node_ids: HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let aliases = build_activity_node_alias_map(nodes);
    diagram
        .flows
        .iter()
        .enumerate()
        .filter_map(|(index, flow)| activity_flow_to_edge(flow, index, &aliases, &node_ids))
        .collect()
}

fn activity_flow_to_edge(
    flow: &ControlFlowDto,
    index: usize,
    aliases: &HashMap<String, String>,
    node_ids: &HashSet<String>,
) -> Option<PreparedEdgeDto> {
    let source = resolve_activity_node_ref(&flow.from, aliases);
    let target = resolve_activity_node_ref(&flow.to, aliases);
    if source.is_empty()
        || target.is_empty()
        || source == target
        || !node_ids.contains(&source)
        || !node_ids.contains(&target)
    {
        return None;
    }
    let guard = flow.guard.clone().unwrap_or_default();
    let condition = flow.condition.clone().unwrap_or_default();
    let guard_lower = guard.to_lowercase();
    let succession =
        guard_lower == "flow" || guard_lower == "first" || guard_lower == "succession";
    let conditional = !condition.is_empty()
        || (!guard.is_empty()
            && !["flow", "first", "bind", "perform", "succession"]
                .contains(&guard_lower.as_str()));
    Some(PreparedEdgeDto {
        id: format!("flow-{index}"),
        source,
        target,
        label: if !condition.is_empty() {
            condition.clone()
        } else if !guard.is_empty() {
            guard.clone()
        } else {
            String::new()
        },
        edge_kind: None,
        attributes: Some(json!({
            "guard": if guard.is_empty() { Value::Null } else { json!(guard) },
            "condition": if condition.is_empty() { Value::Null } else { json!(condition) },
            "succession": succession,
            "conditional": conditional,
        })),
    })
}

pub fn prepare_activity_prepared_view(response: &SysmlVisualizationResultDto) -> PreparedViewDto {
    let diagrams = response.activity_diagrams.clone().unwrap_or_default();
    let selected = select_activity_diagram(
        &diagrams,
        response.selected_view_name.as_deref(),
        response.selected_view.as_deref(),
    );
    let Some(diagram) = selected.or_else(|| best_activity_diagram(&diagrams)).cloned() else {
        return PreparedViewDto {
            title: response
                .selected_view_name
                .clone()
                .unwrap_or_else(|| "Action Flow View".to_string()),
            view: "action-flow-view".to_string(),
            nodes: Vec::new(),
            edges: Vec::new(),
            meta: None,
        };
    };
    let nodes = collect_activity_nodes(&diagram);
    let edges = activity_edges(&diagram, &nodes);
    let swim_lanes: Vec<String> = nodes
        .iter()
        .filter_map(|node| {
            node.attributes
                .as_ref()
                .and_then(|attrs| attrs.get("swimLane"))
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    PreparedViewDto {
        title: if diagram.name.is_empty() {
            response
                .selected_view_name
                .clone()
                .unwrap_or_else(|| "Action Flow View".to_string())
        } else {
            diagram.name.clone()
        },
        view: "action-flow-view".to_string(),
        nodes,
        edges,
        meta: Some(json!({
            "selectedDiagramId": diagram.id,
            "nodeCount": diagram.actions.len(),
            "edgeCount": diagram.flows.len(),
            "layoutDirection": "vertical",
            "activityDiagram": diagram,
            "parentContext": diagram.name,
            "swimLanes": swim_lanes,
        })),
    }
}

fn format_state_transition_label(transition: &StateTransitionDto) -> String {
    let mut parts = Vec::new();
    if let Some(guard) = transition.guard.as_ref().filter(|value| !value.is_empty()) {
        parts.push(format!("[{guard}]"));
    }
    if let Some(effect) = transition.effect.as_ref().filter(|value| !value.is_empty()) {
        parts.push(effect.clone());
    }
    if let Some(accept) = transition.accept.as_ref().filter(|value| !value.is_empty()) {
        parts.push(format!("accept {accept}"));
    }
    if let Some(send) = transition.send.as_ref().filter(|value| !value.is_empty()) {
        parts.push(format!("send {send}"));
    }
    if !parts.is_empty() {
        parts.join(" / ")
    } else {
        transition
            .label
            .clone()
            .or_else(|| transition.name.clone())
            .unwrap_or_default()
    }
}

fn state_kind(state: &StateNodeDto) -> &str {
    let kind = state.kind.to_lowercase();
    if kind.contains("initial") {
        "initial"
    } else if kind.contains("terminate") {
        "terminate"
    } else if kind.contains("final") {
        "final"
    } else if kind.contains("composite") {
        "composite"
    } else {
        "state"
    }
}

fn collect_state_machine_nodes(machine: &StateMachineDto) -> Vec<PreparedNodeDto> {
    machine
        .states
        .iter()
        .enumerate()
        .map(|(index, state)| {
            let kind = state_kind(state);
            let mut attrs = json!({
                "qualifiedName": state.id,
                "entry": state.entry,
                "do": state.do_action,
                "exit": state.exit,
                "regionId": state.region_id,
            });
            build_behavior_node(
                &state.id,
                if state.name.is_empty() {
                    "State".to_string()
                } else {
                    state.name.clone()
                },
                kind,
                state.element.uri.clone(),
                Some(to_prepared_range(&state.element.range)),
                attrs,
            )
        })
        .collect()
}

fn select_state_machine<'a>(
    machines: &'a [StateMachineDto],
    selected_name: Option<&str>,
    selected_view_id: Option<&str>,
) -> Option<&'a StateMachineDto> {
    if machines.is_empty() {
        return None;
    }
    machines
        .iter()
        .find(|machine| {
            diagram_matches_selection(
                &machine.id,
                &machine.name,
                None,
                selected_name,
                selected_view_id,
            )
        })
        .or_else(|| machines.first())
}

pub fn prepare_state_prepared_view(response: &SysmlVisualizationResultDto) -> PreparedViewDto {
    let machines = response.state_machines.clone().unwrap_or_default();
    if let Some(machine) = select_state_machine(
        &machines,
        response.selected_view_name.as_deref(),
        response.selected_view.as_deref(),
    ) {
        let nodes = collect_state_machine_nodes(machine);
        let node_ids: HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let aliases = build_activity_node_alias_map(&nodes);
        let edges: Vec<PreparedEdgeDto> = machine
            .transitions
            .iter()
            .enumerate()
            .filter_map(|(index, transition)| {
                let source = resolve_activity_node_ref(&transition.source, &aliases);
                let target = resolve_activity_node_ref(&transition.target, &aliases);
                if source.is_empty() || target.is_empty() {
                    return None;
                }
                if !node_ids.contains(&source) || !node_ids.contains(&target) {
                    return None;
                }
                Some(PreparedEdgeDto {
                    id: if transition.id.is_empty() {
                        format!("transition-{index}")
                    } else {
                        transition.id.clone()
                    },
                    source: source.clone(),
                    target,
                    label: format_state_transition_label(transition),
                    edge_kind: None,
                    attributes: Some(json!({
                        "selfLoop": transition.self_loop || source == transition.target,
                        "guard": transition.guard,
                        "effect": transition.effect,
                        "accept": transition.accept,
                        "send": transition.send,
                    })),
                })
            })
            .collect();
        return PreparedViewDto {
            title: if machine.name.is_empty() {
                response
                    .selected_view_name
                    .clone()
                    .unwrap_or_else(|| "State Transition View".to_string())
            } else {
                machine.name.clone()
            },
            view: "state-transition-view".to_string(),
            nodes,
            edges,
            meta: Some(json!({
                "selectedDiagramId": machine.id,
                "selectedDiagramName": machine.name,
                "layoutDirection": "horizontal",
                "stateMachine": machine,
                "parentContext": machine.name,
            })),
        };
    }
    if let Some(graph) = response.graph.clone() {
        return prepare_graph_from_dto(&graph, response);
    }
    PreparedViewDto {
        title: response
            .selected_view_name
            .clone()
            .unwrap_or_else(|| "State Transition View".to_string()),
        view: "state-transition-view".to_string(),
        nodes: Vec::new(),
        edges: Vec::new(),
        meta: None,
    }
}

fn to_prepared_range(range: &crate::semantic::extracted_model::RangeDto) -> RangeDto {
    RangeDto {
        start: crate::semantic::dto::PositionDto {
            line: range.start.line,
            character: range.start.character,
        },
        end: crate::semantic::dto::PositionDto {
            line: range.end.line,
            character: range.end.character,
        },
    }
}

fn diagram_to_prepared(
    diagram: &SequenceDiagramDto,
    view: &str,
    fallback_title: &str,
) -> PreparedViewDto {
    let nodes: Vec<PreparedNodeDto> = diagram
        .lifelines
        .iter()
        .enumerate()
        .map(|(index, lifeline)| {
            build_behavior_node(
                if lifeline.id.is_empty() {
                    format!("lifeline-{index}")
                } else {
                    lifeline.id.clone()
                }
                .as_str(),
                if lifeline.name.is_empty() {
                    format!("Lifeline {}", index + 1)
                } else {
                    lifeline.name.clone()
                },
                "lifeline",
                lifeline.uri.clone(),
                Some(to_prepared_range(&lifeline.range)),
                json!({ "name": lifeline.name }),
            )
        })
        .collect();
    let mut edges: Vec<PreparedEdgeDto> = diagram
        .messages
        .iter()
        .enumerate()
        .map(|(index, message)| PreparedEdgeDto {
            id: if message.id.is_empty() {
                format!("message-{index}")
            } else {
                message.id.clone()
            },
            source: message.from.clone(),
            target: message.to.clone(),
            label: message
                .label
                .clone()
                .unwrap_or_else(|| message.name.clone()),
            edge_kind: None,
            attributes: None,
        })
        .collect();
    let node_ids: HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
    edges.retain(|edge| node_ids.contains(&edge.source) && node_ids.contains(&edge.target));
    PreparedViewDto {
        title: if diagram.name.is_empty() {
            fallback_title.to_string()
        } else {
            diagram.name.clone()
        },
        view: view.to_string(),
        nodes,
        edges,
        meta: None,
    }
}

pub fn prepare_sequence_prepared_view(response: &SysmlVisualizationResultDto) -> PreparedViewDto {
    let diagrams = response.sequence_diagrams.clone().unwrap_or_default();
    let selected = diagrams.iter().find(|diagram| {
        diagram_matches_selection(
            &diagram.id,
            &diagram.name,
            Some(&diagram.package_path),
            response.selected_view_name.as_deref(),
            response.selected_view.as_deref(),
        )
    });
    let effective = selected.or_else(|| diagrams.first());
    if let Some(diagram) = effective {
        let mut prepared = diagram_to_prepared(diagram, "sequence-view", "Sequence View");
        prepared.meta = Some(json!({
            "selectedDiagramName": diagram.name,
            "sequenceDiagram": diagram,
            "parentContext": diagram.name,
        }));
        return prepared;
    }
    if let Some(graph) = response.general_view_graph.clone().or(response.graph.clone()) {
        return prepare_graph_from_dto(&graph, response);
    }
    PreparedViewDto {
        title: response
            .selected_view_name
            .clone()
            .unwrap_or_else(|| "Sequence View".to_string()),
        view: "sequence-view".to_string(),
        nodes: Vec::new(),
        edges: Vec::new(),
        meta: None,
    }
}
