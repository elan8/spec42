//! Server-side finalization of behavior-view DTOs before LSP/CLI serialization.
//!
//! Replaces semantic shaping that previously lived in `normalize-payload.ts`.

use crate::semantic::extracted_model::{
    ActivityDiagramDto, SequenceDiagramDto, StateMachineDto,
};
use crate::semantic::dto::{
    ActivityDiagramCandidateDto, SequenceDiagramCandidateDto, StateMachineCandidateDto,
};

/// Selector label shown in view pickers: `name - packagePath`.
pub fn build_selector_label(name: &str, package_path: &str) -> String {
    if package_path.is_empty() {
        name.to_string()
    } else {
        format!("{name} - {package_path}")
    }
}

fn rank_state_machine(machine: &StateMachineDto) -> (usize, usize) {
    (machine.states.len(), machine.transitions.len())
}

/// Apply labels and stable sort order for state-transition responses.
pub fn finalize_state_machines_for_response(
    mut machines: Vec<StateMachineDto>,
) -> Vec<StateMachineDto> {
    for machine in &mut machines {
        if machine.label.is_empty() {
            machine.label = build_selector_label(&machine.name, &machine.package_path);
        }
    }
    machines.sort_by(|left, right| {
        let (left_states, left_transitions) = rank_state_machine(left);
        let (right_states, right_transitions) = rank_state_machine(right);
        right_states
            .cmp(&left_states)
            .then_with(|| right_transitions.cmp(&left_transitions))
            .then_with(|| {
                left.label
                    .cmp(&right.label)
                    .then_with(|| left.name.cmp(&right.name))
            })
    });
    machines.retain(|machine| !machine.states.is_empty());
    machines
}

fn activity_node_count(diagram: &ActivityDiagramDto) -> usize {
    diagram.actions.len() + diagram.decisions.len() + diagram.states.len()
}

fn activity_diagram_is_renderable(diagram: &ActivityDiagramDto) -> bool {
    !diagram.flows.is_empty() && activity_node_count(diagram) > 0
}

fn rank_activity_diagram(diagram: &ActivityDiagramDto) -> i64 {
    let flow_count = diagram.flows.len() as i64;
    let node_count = activity_node_count(diagram) as i64;
    let source_bonus = match diagram.source_kind.as_str() {
        "actionDef" => 10_000,
        "performer" => 5_000,
        _ => 0,
    };
    source_bonus + flow_count * 100 + node_count * 10
}

fn last_path_segment(value: &str) -> &str {
    value
        .split(['.', ':'])
        .filter(|segment| !segment.is_empty())
        .last()
        .unwrap_or(value)
}

fn resolve_activity_flow_endpoint<'a>(
    value: &str,
    aliases: &std::collections::HashMap<String, String>,
) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let normalized = trimmed.replace("::", ".");
    let last = last_path_segment(&normalized);
    let first = normalized.split('.').next().unwrap_or("");
    aliases
        .get(trimmed)
        .or_else(|| aliases.get(&normalized))
        .or_else(|| aliases.get(last))
        .or_else(|| aliases.get(first))
        .cloned()
        .unwrap_or_else(|| value.to_string())
}

fn register_activity_aliases(
    aliases: &mut std::collections::HashMap<String, String>,
    alias: &str,
    node_id: &str,
) {
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
}

fn finalize_activity_diagram(diagram: &mut ActivityDiagramDto) {
    if diagram.label.is_empty() {
        diagram.label = build_selector_label(&diagram.name, &diagram.package_path);
    }

    let mut aliases = std::collections::HashMap::new();
    for action in &diagram.actions {
        let node_id = action
            .id
            .clone()
            .unwrap_or_else(|| action.name.clone());
        register_activity_aliases(&mut aliases, &node_id, &node_id);
        register_activity_aliases(&mut aliases, &action.name, &node_id);
    }
    for decision in &diagram.decisions {
        register_activity_aliases(&mut aliases, &decision.name, &decision.name);
    }
    for state in &diagram.states {
        register_activity_aliases(&mut aliases, &state.name, &state.name);
    }

    let node_ids: std::collections::HashSet<String> = aliases.values().cloned().collect();
    diagram.flows.retain(|flow| {
        flow.from != flow.to
            && node_ids.contains(&resolve_activity_flow_endpoint(&flow.from, &aliases))
            && node_ids.contains(&resolve_activity_flow_endpoint(&flow.to, &aliases))
    });
    for flow in &mut diagram.flows {
        flow.from = resolve_activity_flow_endpoint(&flow.from, &aliases);
        flow.to = resolve_activity_flow_endpoint(&flow.to, &aliases);
    }
}

/// Filter, rank, label, and normalize flow endpoints for action-flow responses.
pub fn finalize_activity_diagrams_for_response(
    mut diagrams: Vec<ActivityDiagramDto>,
) -> Vec<ActivityDiagramDto> {
    for diagram in &mut diagrams {
        finalize_activity_diagram(diagram);
    }
    diagrams.retain(activity_diagram_is_renderable);
    diagrams.sort_by(|left, right| {
        rank_activity_diagram(right)
            .cmp(&rank_activity_diagram(left))
            .then_with(|| {
                left.label
                    .cmp(&right.label)
                    .then_with(|| left.name.cmp(&right.name))
            })
    });
    diagrams
}

fn sequence_diagram_is_renderable(diagram: &SequenceDiagramDto) -> bool {
    !diagram.lifelines.is_empty() && !diagram.messages.is_empty()
}

fn rank_sequence_diagram(diagram: &SequenceDiagramDto) -> usize {
    diagram.messages.len()
}

/// Filter, rank, and label sequence diagrams for sequence-view responses.
pub fn finalize_sequence_diagrams_for_response(
    mut diagrams: Vec<SequenceDiagramDto>,
) -> Vec<SequenceDiagramDto> {
    for diagram in &mut diagrams {
        if diagram.label.is_empty() {
            diagram.label = build_selector_label(&diagram.name, &diagram.package_path);
        }
    }
    diagrams.retain(sequence_diagram_is_renderable);
    diagrams.sort_by(|left, right| {
        rank_sequence_diagram(right)
            .cmp(&rank_sequence_diagram(left))
            .then_with(|| {
                left.label
                    .cmp(&right.label)
                    .then_with(|| left.name.cmp(&right.name))
            })
    });
    diagrams
}

pub fn finalize_activity_diagram_candidates_for_response(
    diagrams: &[ActivityDiagramDto],
) -> Vec<ActivityDiagramCandidateDto> {
    diagrams
        .iter()
        .map(|diagram| ActivityDiagramCandidateDto {
            id: diagram.id.clone(),
            name: diagram.name.clone(),
            label: if diagram.label.is_empty() {
                build_selector_label(&diagram.name, &diagram.package_path)
            } else {
                diagram.label.clone()
            },
            package_path: diagram.package_path.clone(),
            source_kind: diagram.source_kind.clone(),
            node_count: activity_node_count(diagram) as u32,
            flow_count: diagram.flows.len() as u32,
        })
        .collect()
}

pub fn finalize_state_machine_candidates_for_response(
    machines: &[StateMachineDto],
) -> Vec<StateMachineCandidateDto> {
    machines
        .iter()
        .map(|machine| StateMachineCandidateDto {
            id: machine.id.clone(),
            name: machine.name.clone(),
            label: if machine.label.is_empty() {
                build_selector_label(&machine.name, &machine.package_path)
            } else {
                machine.label.clone()
            },
            package_path: machine.package_path.clone(),
            state_count: machine.states.len() as u32,
            transition_count: machine.transitions.len() as u32,
        })
        .collect()
}

pub fn finalize_sequence_diagram_candidates_for_response(
    diagrams: &[SequenceDiagramDto],
) -> Vec<SequenceDiagramCandidateDto> {
    diagrams
        .iter()
        .map(|diagram| SequenceDiagramCandidateDto {
            id: diagram.id.clone(),
            name: diagram.name.clone(),
            label: if diagram.label.is_empty() {
                build_selector_label(&diagram.name, &diagram.package_path)
            } else {
                diagram.label.clone()
            },
            package_path: diagram.package_path.clone(),
            message_count: diagram.messages.len() as u32,
            lifeline_count: diagram.lifelines.len() as u32,
        })
        .collect()
}

/// Log when behavior views would have relied on the TypeScript AST fallback.
pub fn warn_if_behavior_payload_missing(view: &str, state_count: usize, activity_count: usize) {
    match view {
        "state-transition-view" if state_count == 0 => {
            eprintln!(
                "spec42: state-transition-view response has empty stateMachines; \
                 TypeScript AST fallback is removed — ensure graph-first extraction populates DTOs"
            );
        }
        "action-flow-view" if activity_count == 0 => {
            eprintln!(
                "spec42: action-flow-view response has empty activityDiagrams; \
                 TypeScript AST fallback is removed — ensure graph-first extraction populates DTOs"
            );
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::extracted_model::{
        ActivityActionDto, ControlFlowDto, RangeDto, SequenceLifelineDto, SequenceMessageDto,
        StateNodeDto, StateNodeElementDto,
    };

    fn empty_range() -> RangeDto {
        RangeDto {
            start: crate::semantic::extracted_model::PositionDto {
                line: 0,
                character: 0,
            },
            end: crate::semantic::extracted_model::PositionDto {
                line: 0,
                character: 0,
            },
        }
    }

    #[test]
    fn activity_renderability_requires_flows_and_nodes() {
        let empty_flows = ActivityDiagramDto {
            id: "a".into(),
            name: "Empty".into(),
            package_path: String::new(),
            label: String::new(),
            source_kind: "actionDef".into(),
            uri: None,
            actions: vec![ActivityActionDto {
                id: Some("step".into()),
                name: "step".into(),
                action_type: "action".into(),
                kind: None,
                inputs: None,
                outputs: None,
                range: None,
                uri: None,
                swim_lane: None,
            }],
            interface: None,
            decisions: vec![],
            flows: vec![],
            states: vec![],
            range: empty_range(),
        };
        assert!(!activity_diagram_is_renderable(&empty_flows));

        let renderable = ActivityDiagramDto {
            flows: vec![ControlFlowDto {
                from: "step".into(),
                to: "step".into(),
                condition: None,
                guard: None,
                range: empty_range(),
            }],
            ..empty_flows
        };
        assert!(activity_diagram_is_renderable(&renderable));
    }

    #[test]
    fn activity_ranking_prefers_action_def_over_performer() {
        let performer = ActivityDiagramDto {
            id: "p".into(),
            name: "Performer".into(),
            package_path: "Pkg".into(),
            label: String::new(),
            source_kind: "performer".into(),
            uri: None,
            actions: vec![
                ActivityActionDto {
                    id: Some("a".into()),
                    name: "a".into(),
                    action_type: "action".into(),
                    kind: None,
                    inputs: None,
                    outputs: None,
                    range: None,
                    uri: None,
                    swim_lane: None,
                },
                ActivityActionDto {
                    id: Some("b".into()),
                    name: "b".into(),
                    action_type: "action".into(),
                    kind: None,
                    inputs: None,
                    outputs: None,
                    range: None,
                    uri: None,
                    swim_lane: None,
                },
            ],
            interface: None,
            decisions: vec![],
            flows: vec![ControlFlowDto {
                from: "a".into(),
                to: "b".into(),
                condition: None,
                guard: None,
                range: empty_range(),
            }],
            states: vec![],
            range: empty_range(),
        };
        let action_def = ActivityDiagramDto {
            source_kind: "actionDef".into(),
            name: "ActionDef".into(),
            ..performer.clone()
        };
        let ranked = finalize_activity_diagrams_for_response(vec![performer, action_def]);
        assert_eq!(ranked[0].source_kind, "actionDef");
    }

    #[test]
    fn state_machines_receive_selector_labels() {
        let machines = finalize_state_machines_for_response(vec![StateMachineDto {
            id: "Pkg::M".into(),
            name: "M".into(),
            label: String::new(),
            package_path: "Pkg".into(),
            uri: None,
            states: vec![StateNodeDto {
                id: "s1".into(),
                name: "s1".into(),
                kind: "state".into(),
                parent_id: None,
                region_id: None,
                entry: None,
                do_action: None,
                exit: None,
                element: StateNodeElementDto {
                    id: "s1".into(),
                    name: "s1".into(),
                    element_type: "state".into(),
                    uri: None,
                    range: empty_range(),
                },
            }],
            transitions: vec![],
            regions: vec![],
            range: empty_range(),
        }]);
        assert_eq!(machines[0].label, "M - Pkg");
    }

    #[test]
    fn sequence_diagrams_filter_empty() {
        let kept = finalize_sequence_diagrams_for_response(vec![
            SequenceDiagramDto {
                id: "empty".into(),
                name: "empty".into(),
                package_path: String::new(),
                label: String::new(),
                source_kind: "scenario".into(),
                uri: None,
                lifelines: vec![],
                messages: vec![],
                activations: vec![],
                fragments: vec![],
                range: empty_range(),
            },
            SequenceDiagramDto {
                id: "ok".into(),
                name: "ok".into(),
                package_path: "Pkg".into(),
                label: String::new(),
                source_kind: "scenario".into(),
                uri: None,
                lifelines: vec![SequenceLifelineDto {
                    id: "l1".into(),
                    name: "l1".into(),
                    uri: None,
                    range: empty_range(),
                }],
                messages: vec![SequenceMessageDto {
                    id: "m1".into(),
                    name: "m1".into(),
                    from: "l1".into(),
                    to: "l1".into(),
                    kind: "sync".into(),
                    order: 1,
                    label: None,
                    uri: None,
                    range: empty_range(),
                }],
                activations: vec![],
                fragments: vec![],
                range: empty_range(),
            },
        ]);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].label, "ok - Pkg");
    }
}
