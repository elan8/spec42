use super::*;

pub(crate) fn extract_activity_from_action(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::ActionDef>,
    package_segments: &[String],
    parent_segments: &[String],
) -> ActivityDiagramDto {
    let name = identification_name(&node.identification);
    let qualified_segments = with_segment(parent_segments, name.clone());
    let range = span_to_range_dto(&node.span);
    let mut actions = Vec::new();
    let mut flows = Vec::new();
    let mut states = Vec::new();
    let mut decisions = Vec::new();
    let mut interface_inputs = Vec::new();
    let mut interface_outputs = Vec::new();
    let mut previous_then_action: Option<String> = None;
    if let ActionDefBody::Brace { elements } = &node.body {
        for (i, element) in elements.iter().enumerate() {
            match &element.value {
                ActionDefBodyElement::InOutDecl(in_out) => {
                    let param_name = if in_out.value.name.trim().is_empty() {
                        format!("param_{}", i)
                    } else {
                        in_out.value.name.clone()
                    };
                    match in_out.value.direction {
                        sysml_v2_parser::ast::InOut::In => interface_inputs.push(param_name),
                        sysml_v2_parser::ast::InOut::Out => interface_outputs.push(param_name),
                        sysml_v2_parser::ast::InOut::InOut => {
                            interface_inputs.push(param_name.clone());
                            interface_outputs.push(param_name);
                        }
                    }
                }
                ActionDefBodyElement::Perform(perform) => {
                    let perform_name = if perform.value.action_name.trim().is_empty() {
                        perform
                            .value
                            .type_name
                            .clone()
                            .unwrap_or_else(|| format!("perform_{}", i))
                    } else {
                        perform.value.action_name.clone()
                    };
                    actions.push(ActivityActionDto {
                        id: Some(format!(
                            "{}::{}",
                            join_segments(&qualified_segments),
                            perform_name
                        )),
                        name: perform_name,
                        action_type: "action".to_string(),
                        kind: Some("perform".to_string()),
                        inputs: None,
                        outputs: None,
                        range: Some(span_to_range_dto(&perform.span)),
                        uri: None,
                        swim_lane: None,
                    });
                }
                ActionDefBodyElement::ActionUsage(usage) => {
                    let u = usage.as_ref();
                    let control = control_state_type(&u.type_name);
                    let step_kind = control.unwrap_or("action");
                    let mut inputs = Vec::new();
                    if let Some(ref accept) = &u.accept {
                        inputs.push(accept.name.clone());
                    }
                    if control == Some("decision") {
                        decisions.push(DecisionNodeDto {
                            name: u.name.clone(),
                            condition: String::new(),
                            branches: Vec::new(),
                            range: span_to_range_dto(&u.span),
                        });
                    }
                    if control.is_some() {
                        states.push(ActivityStateDto {
                            name: u.name.clone(),
                            state_type: step_kind.to_string(),
                            range: span_to_range_dto(&u.span),
                        });
                    }
                    actions.push(ActivityActionDto {
                        id: Some(format!(
                            "{}::{}",
                            join_segments(&qualified_segments),
                            u.name
                        )),
                        name: u.name.clone(),
                        action_type: "action".to_string(),
                        kind: Some(step_kind.to_string()),
                        inputs: if inputs.is_empty() {
                            None
                        } else {
                            Some(inputs)
                        },
                        outputs: None,
                        range: Some(span_to_range_dto(&u.span)),
                        uri: None,
                        swim_lane: None,
                    });
                }
                ActionDefBodyElement::Assign(assign) => {
                    let value = &assign.value;
                    let step_name = format!("assign_{}", expression_to_debug_string(&value.lhs));
                    states.push(ActivityStateDto {
                        name: step_name.clone(),
                        state_type: "assign".to_string(),
                        range: span_to_range_dto(&assign.span),
                    });
                    actions.push(ActivityActionDto {
                        id: Some(format!(
                            "{}::{}",
                            join_segments(&qualified_segments),
                            step_name
                        )),
                        name: step_name.clone(),
                        action_type: "action".to_string(),
                        kind: Some("assign".to_string()),
                        inputs: None,
                        outputs: None,
                        range: Some(span_to_range_dto(&assign.span)),
                        uri: None,
                        swim_lane: None,
                    });
                    if value.is_then {
                        if let Some(previous) = previous_then_action.take() {
                            flows.push(ControlFlowDto {
                                from: previous,
                                to: step_name.clone(),
                                condition: Some(expr_to_string(&value.rhs)),
                                guard: Some("flow".to_string()),
                                range: span_to_range_dto(&assign.span),
                            });
                        }
                    }
                    previous_then_action = Some(step_name);
                }
                ActionDefBodyElement::ForLoop(for_loop) => {
                    let fl = &for_loop.value;
                    let loop_name = format!("for_{}", fl.var);
                    states.push(ActivityStateDto {
                        name: loop_name.clone(),
                        state_type: "for-loop".to_string(),
                        range: span_to_range_dto(&for_loop.span),
                    });
                    if let ActionDefBody::Brace { elements } = &fl.body {
                        for (j, inner) in elements.iter().enumerate() {
                            if let ActionDefBodyElement::ThenAction(then_action) = &inner.value {
                                let action = &then_action.value.action.value;
                                let perform_name = if action.name.trim().is_empty() {
                                    format!("for_body_{j}")
                                } else {
                                    action.name.clone()
                                };
                                if let Some(previous) = previous_then_action.take() {
                                    flows.push(ControlFlowDto {
                                        from: previous,
                                        to: perform_name.clone(),
                                        condition: None,
                                        guard: Some("flow".to_string()),
                                        range: span_to_range_dto(&then_action.span),
                                    });
                                }
                                flows.push(ControlFlowDto {
                                    from: loop_name.clone(),
                                    to: perform_name.clone(),
                                    condition: None,
                                    guard: Some("flow".to_string()),
                                    range: span_to_range_dto(&for_loop.span),
                                });
                                actions.push(ActivityActionDto {
                                    id: Some(format!(
                                        "{}::{}",
                                        join_segments(&qualified_segments),
                                        perform_name
                                    )),
                                    name: perform_name.clone(),
                                    action_type: "action".to_string(),
                                    kind: Some("action".to_string()),
                                    inputs: None,
                                    outputs: None,
                                    range: Some(span_to_range_dto(&then_action.span)),
                                    uri: None,
                                    swim_lane: None,
                                });
                                previous_then_action = Some(perform_name);
                            }
                        }
                    }
                }
                ActionDefBodyElement::Bind(bind) => {
                    let left = expr_to_string(&bind.value.left);
                    let right = expr_to_string(&bind.value.right);
                    if !left.is_empty() && !right.is_empty() {
                        flows.push(ControlFlowDto {
                            from: left,
                            to: right,
                            condition: None,
                            guard: Some("bind".to_string()),
                            range: span_to_range_dto(&bind.span),
                        });
                    }
                }
                ActionDefBodyElement::FlowUsage(flow) => {
                    if let (Some(from_expr), Some(to_expr)) = (&flow.value.from, &flow.value.to) {
                        let from = expr_to_string(from_expr);
                        let to = expr_to_string(to_expr);
                        if !from.is_empty() && !to.is_empty() {
                            let condition = flow
                                .value
                                .payload
                                .as_ref()
                                .map(|payload| payload_feature_to_string(&payload.value));
                            flows.push(ControlFlowDto {
                                from,
                                to,
                                condition,
                                guard: Some(flow_guard_for_usage(flow.value.kind).to_string()),
                                range: span_to_range_dto(&flow.span),
                            });
                        }
                    }
                }
                ActionDefBodyElement::FirstStmt(first) => {
                    let from = expr_to_string(&first.value.first);
                    let to = expr_to_string(&first.value.then);
                    if !from.is_empty() && !to.is_empty() {
                        flows.push(ControlFlowDto {
                            from,
                            to,
                            condition: None,
                            guard: Some("first".to_string()),
                            range: span_to_range_dto(&first.span),
                        });
                    }
                }
                ActionDefBodyElement::MergeStmt(merge) => {
                    let m = expr_to_string(&merge.value.merge);
                    states.push(ActivityStateDto {
                        name: if m.is_empty() {
                            format!("merge_{}", i)
                        } else {
                            m
                        },
                        state_type: "merge".to_string(),
                        range: span_to_range_dto(&merge.span),
                    });
                }
                ActionDefBodyElement::ThenAction(then_action) => {
                    let action = &then_action.value.action.value;
                    let perform_name = if action.name.trim().is_empty() {
                        format!("then_action_{}", i)
                    } else {
                        action.name.clone()
                    };
                    if let Some(previous) = previous_then_action.take() {
                        flows.push(ControlFlowDto {
                            from: previous,
                            to: perform_name.clone(),
                            condition: None,
                            guard: Some("flow".to_string()),
                            range: span_to_range_dto(&then_action.span),
                        });
                    }
                    actions.push(ActivityActionDto {
                        id: Some(format!(
                            "{}::{}",
                            join_segments(&qualified_segments),
                            perform_name
                        )),
                        name: perform_name.clone(),
                        action_type: "action".to_string(),
                        kind: Some("action".to_string()),
                        inputs: None,
                        outputs: None,
                        range: Some(span_to_range_dto(&then_action.span)),
                        uri: None,
                        swim_lane: None,
                    });
                    previous_then_action = Some(perform_name);
                }
                ActionDefBodyElement::StateUsage(state_usage) => {
                    states.push(ActivityStateDto {
                        name: state_usage.value.name.clone(),
                        state_type: "state".to_string(),
                        range: span_to_range_dto(&state_usage.span),
                    });
                }
                ActionDefBodyElement::Error(_) | ActionDefBodyElement::Doc(_) => {}
                _ => {}
            }
        }
    }

    // Synthesize action nodes referenced by flow endpoints so the UI can render sequencing like
    // `first validateRoute then startMission;` even when the parser doesn't surface those steps
    // as ActionUsage/Perform nodes.
    fn endpoint_to_step_name(endpoint: &str) -> Option<String> {
        let s = endpoint.trim();
        if s.is_empty() {
            return None;
        }
        // `foo::bar` -> `foo`, `foo.bar` -> `foo`
        let step = s
            .split_once("::")
            .map(|(head, _)| head)
            .or_else(|| s.split_once('.').map(|(head, _)| head))
            .unwrap_or(s)
            .trim();
        if step.is_empty() {
            None
        } else {
            Some(step.to_string())
        }
    }

    let existing_action_names: std::collections::HashSet<String> =
        actions.iter().map(|a| a.name.clone()).collect();
    let mut referenced_step_names: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    let interface_param_names: std::collections::HashSet<String> = interface_inputs
        .iter()
        .chain(interface_outputs.iter())
        .cloned()
        .collect();

    for f in &flows {
        if let Some(step) = endpoint_to_step_name(&f.from) {
            if !interface_param_names.contains(&step) {
                referenced_step_names.insert(step);
            }
        }
        if let Some(step) = endpoint_to_step_name(&f.to) {
            if !interface_param_names.contains(&step) {
                referenced_step_names.insert(step);
            }
        }
    }

    // Avoid turning the activity itself into a node (e.g., `ExecutePatrol::route`).
    referenced_step_names.remove(&name);

    for step in referenced_step_names {
        if existing_action_names.contains(&step) {
            continue;
        }
        actions.push(ActivityActionDto {
            id: Some(format!("{}::{}", join_segments(&qualified_segments), step)),
            name: step,
            action_type: "action".to_string(),
            kind: Some("action".to_string()),
            inputs: None,
            outputs: None,
            range: None,
            uri: None,
            swim_lane: None,
        });
    }

    let interface = if interface_inputs.is_empty() && interface_outputs.is_empty() {
        None
    } else {
        Some(ActivityInterfaceDto {
            inputs: interface_inputs,
            outputs: interface_outputs,
        })
    };
    ActivityDiagramDto {
        id: activity_diagram_id(&qualified_segments, "actionDef"),
        name: if name.is_empty() {
            "action".to_string()
        } else {
            name
        },
        package_path: package_path_from_segments(package_segments),
        label: String::new(),
        source_kind: "actionDef".to_string(),
        uri: None,
        actions,
        interface,
        decisions,
        flows,
        states,
        range,
    }
}
