//! Enrich AST-extracted activity diagrams with semantic graph facts (P1 action body projection).

use std::collections::{HashMap, HashSet};

use url::Url;

use crate::semantic::dto::{PositionDto, RangeDto};
use crate::semantic::extracted_model::{
    ActivityActionDto, ActivityDiagramDto, ActivityStateDto, ControlFlowDto, DecisionNodeDto,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, RelationshipKind, SemanticNode};
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

fn is_action_step_kind(kind: &crate::ElementKind) -> bool {
    matches!(kind, ElementKind::Action | ElementKind::Perform)
}

fn normalized_type_name(type_name: &str) -> String {
    type_name
        .split("::")
        .last()
        .unwrap_or(type_name)
        .replace([' ', '_'], "")
        .to_lowercase()
}

fn control_state_type(type_name: &str) -> Option<&'static str> {
    match normalized_type_name(type_name).as_str() {
        "decision" => Some("decision"),
        "merge" => Some("merge"),
        "fork" => Some("fork"),
        "join" => Some("join"),
        "terminate" => Some("terminate"),
        "accept" => Some("accept"),
        "send" => Some("send"),
        _ => None,
    }
}

fn control_kind_from_graph_node(node: &SemanticNode) -> Option<&'static str> {
    match node.element_kind {
        ElementKind::Merge => Some("merge"),
        ElementKind::Assign => Some("assign"),
        ElementKind::ForLoop => Some("for-loop"),
        ElementKind::Action | ElementKind::Perform => node
            .attributes
            .get("actionType")
            .and_then(|value| value.as_str())
            .and_then(control_state_type),
        _ => None,
    }
}

fn is_activity_step_node(node: &SemanticNode) -> bool {
    is_action_step_kind(&node.element_kind)
        || matches!(
            node.element_kind,
            ElementKind::Assign | ElementKind::Merge | ElementKind::ForLoop
        )
        || control_kind_from_graph_node(node).is_some()
}

fn activity_step_name(node: &SemanticNode) -> String {
    if let Some(kind) = control_kind_from_graph_node(node) {
        match kind {
            "assign" => {
                let lhs = node
                    .attributes
                    .get("lhs")
                    .and_then(|value| value.as_str())
                    .unwrap_or("assign");
                format!("assign_{lhs}")
            }
            "for-loop" => format!("for_{}", node.name),
            "merge" => node
                .attributes
                .get("mergeTarget")
                .and_then(|value| value.as_str())
                .filter(|target| !target.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| node.name.clone()),
            _ => node.name.clone(),
        }
    } else {
        node.name.clone()
    }
}

fn default_swim_lane(diagram: &ActivityDiagramDto) -> Option<String> {
    if diagram.source_kind == "performer" && !diagram.name.is_empty() {
        return Some(diagram.name.clone());
    }
    let segment = diagram
        .package_path
        .split('.')
        .rfind(|part| !part.is_empty())
        .map(str::to_string);
    segment.filter(|value| !value.is_empty())
}

fn swim_lane_for_action(node: &SemanticNode, default_lane: Option<&str>) -> Option<String> {
    for key in ["performer", "owner", "performedBy"] {
        if let Some(value) = node.attributes.get(key).and_then(|v| v.as_str()) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    default_lane.map(str::to_string)
}

fn find_action_def_for_diagram<'a>(
    graph: &'a SemanticGraph,
    diagram: &ActivityDiagramDto,
) -> Option<&'a SemanticNode> {
    let uri_str = diagram.uri.as_deref()?;
    let uri = Url::parse(uri_str).ok()?;
    let candidates: Vec<&SemanticNode> = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .filter(|node| node.element_kind == ElementKind::ActionDef && node.name == diagram.name)
        .collect();
    if candidates.is_empty() {
        return None;
    }
    if candidates.len() == 1 {
        return Some(candidates[0]);
    }
    let package = diagram.package_path.replace('.', "::");
    if package.is_empty() {
        return candidates.into_iter().next();
    }
    let qualified_suffix = format!("{package}::{}", diagram.name);
    candidates
        .into_iter()
        .find(|node| node.id.qualified_name == qualified_suffix)
}

fn collect_action_step_nodes<'a>(
    graph: &'a SemanticGraph,
    parent: &'a SemanticNode,
    out: &mut HashMap<String, &'a SemanticNode>,
) {
    for child in graph.children_of(parent) {
        if is_activity_step_node(child) {
            let step_name = activity_step_name(child);
            out.insert(step_name, child);
            collect_action_step_nodes(graph, child, out);
        }
    }
}

fn flow_guard_for_kind(kind: &RelationshipKind) -> &'static str {
    match kind {
        RelationshipKind::Flow => "flow",
        RelationshipKind::Perform => "perform",
        RelationshipKind::Bind => "bind",
        _ => "flow",
    }
}

fn merge_graph_flows(
    diagram: &mut ActivityDiagramDto,
    action_def: &SemanticNode,
    step_nodes: &HashMap<String, &SemanticNode>,
    graph: &SemanticGraph,
) {
    let mut existing: HashSet<(String, String, String)> = diagram
        .flows
        .iter()
        .map(|flow| {
            (
                flow.from.clone(),
                flow.to.clone(),
                flow.guard.clone().unwrap_or_default(),
            )
        })
        .collect();

    let mut push_flow = |from: &str, to: &str, guard: &str, range: RangeDto| {
        let key = (from.to_string(), to.to_string(), guard.to_string());
        if existing.insert(key) {
            diagram.flows.push(ControlFlowDto {
                from: from.to_string(),
                to: to.to_string(),
                condition: None,
                guard: Some(guard.to_string()),
                range,
            });
        }
    };

    let def_range = text_range_to_dto(action_def.range);
    for target in graph.outgoing_targets_by_kind(action_def, RelationshipKind::Perform) {
        if is_activity_step_node(target) {
            push_flow(
                &action_def.name,
                &activity_step_name(target),
                "perform",
                text_range_to_dto(target.range),
            );
        }
    }

    for node in step_nodes.values() {
        for (target, kind) in graph.outgoing_relationships(node) {
            let target_name = activity_step_name(target);
            if (kind == RelationshipKind::Flow || kind == RelationshipKind::Bind)
                && step_nodes.contains_key(&target_name)
            {
                push_flow(
                    &activity_step_name(node),
                    &target_name,
                    flow_guard_for_kind(&kind),
                    text_range_to_dto(node.range),
                );
            }
        }
    }

    let _ = def_range;
}

fn enrich_diagram(diagram: &mut ActivityDiagramDto, graph: &SemanticGraph) {
    let Some(action_def) = find_action_def_for_diagram(graph, diagram) else {
        return;
    };

    let uri_string = diagram.uri.clone().unwrap_or_default();
    let default_lane = default_swim_lane(diagram);
    let graph_actions: Vec<ActivityActionDto> = graph
        .children_of(action_def)
        .into_iter()
        .filter(|child| is_activity_step_node(child))
        .map(|child| {
            let step_kind = control_kind_from_graph_node(child).unwrap_or("action");
            ActivityActionDto {
                id: Some(child.id.qualified_name.clone()),
                name: activity_step_name(child),
                action_type: "action".to_string(),
                kind: Some(step_kind.to_string()),
                inputs: None,
                outputs: None,
                range: Some(text_range_to_dto(child.range)),
                uri: Some(uri_string.clone()),
                swim_lane: swim_lane_for_action(child, default_lane.as_deref()),
            }
        })
        .collect();

    let graph_action_names: HashSet<String> =
        graph_actions.iter().map(|a| a.name.clone()).collect();

    diagram
        .actions
        .retain(|action| action.range.is_some() || !graph_action_names.contains(&action.name));

    for graph_action in graph_actions {
        if let Some(existing) = diagram
            .actions
            .iter_mut()
            .find(|action| action.name == graph_action.name)
        {
            *existing = graph_action;
        } else {
            diagram.actions.push(graph_action);
        }
    }

    let mut step_nodes: HashMap<String, &SemanticNode> = HashMap::new();
    collect_action_step_nodes(graph, action_def, &mut step_nodes);
    enrich_control_nodes_from_graph(diagram, graph, action_def);
    merge_graph_flows(diagram, action_def, &step_nodes, graph);
    propagate_interface_parameters(diagram, action_def, graph);
}

fn enrich_control_nodes_from_graph(
    diagram: &mut ActivityDiagramDto,
    graph: &SemanticGraph,
    action_def: &SemanticNode,
) {
    collect_control_nodes_recursive(diagram, graph, action_def);
}

fn collect_control_nodes_recursive(
    diagram: &mut ActivityDiagramDto,
    graph: &SemanticGraph,
    parent: &SemanticNode,
) {
    for child in graph.children_of(parent) {
        if let Some(state_type) = control_kind_from_graph_node(child) {
            let name = activity_step_name(child);
            let range = text_range_to_dto(child.range);
            if !diagram
                .states
                .iter()
                .any(|state| state.name == name && state.state_type == state_type)
            {
                diagram.states.push(ActivityStateDto {
                    name: name.clone(),
                    state_type: state_type.to_string(),
                    range: range.clone(),
                });
            }
            if state_type == "decision"
                && !diagram
                    .decisions
                    .iter()
                    .any(|decision| decision.name == name)
            {
                diagram.decisions.push(DecisionNodeDto {
                    name,
                    condition: String::new(),
                    branches: Vec::new(),
                    range,
                });
            }
        }
        if is_activity_step_node(child) {
            collect_control_nodes_recursive(diagram, graph, child);
        }
    }
}

fn propagate_interface_parameters(
    diagram: &mut ActivityDiagramDto,
    action_def: &SemanticNode,
    graph: &SemanticGraph,
) {
    let mut inputs = diagram
        .interface
        .as_ref()
        .map(|interface| interface.inputs.clone())
        .unwrap_or_default();
    let mut outputs = diagram
        .interface
        .as_ref()
        .map(|interface| interface.outputs.clone())
        .unwrap_or_default();
    for child in graph.children_of(action_def) {
        if child.element_kind != ElementKind::InOutParameter {
            continue;
        }
        let direction = child
            .attributes
            .get("direction")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        match direction {
            "in" => inputs.push(child.name.clone()),
            "out" => outputs.push(child.name.clone()),
            "inout" => {
                inputs.push(child.name.clone());
                outputs.push(child.name.clone());
            }
            _ => {}
        }
    }
    inputs.sort();
    inputs.dedup();
    outputs.sort();
    outputs.dedup();
    if inputs.is_empty() && outputs.is_empty() {
        return;
    }
    diagram.interface =
        Some(crate::semantic::extracted_model::ActivityInterfaceDto { inputs, outputs });
}

/// Merges graph-backed action steps and control flows into AST-extracted activity diagrams.
pub fn enrich_activity_diagrams_from_graph(
    diagrams: &mut [ActivityDiagramDto],
    graph: &SemanticGraph,
    _workspace_uris: &[Url],
) {
    for diagram in diagrams.iter_mut() {
        enrich_diagram(diagram, graph);
    }
}
