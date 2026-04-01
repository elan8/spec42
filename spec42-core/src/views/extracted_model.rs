//! Activity and sequence diagram extraction for sysml/model response.

use crate::syntax::ast_util::identification_name;
use serde::Serialize;
use sysml_parser::ast::{
    ActionDefBody, ActionDefBodyElement, PackageBody, PackageBodyElement, RootElement,
};
use sysml_parser::{RootNamespace, Span};

fn expr_to_string(n: &sysml_parser::Node<sysml_parser::Expression>) -> String {
    use sysml_parser::Expression;
    match &n.value {
        Expression::FeatureRef(s) => s.clone(),
        Expression::MemberAccess(base, member) => {
            let b = expr_to_string(base);
            if b.is_empty() {
                member.clone()
            } else {
                format!("{b}.{member}")
            }
        }
        Expression::Index { base, index } => {
            let b = expr_to_string(base);
            let i = expr_to_string(index);
            if b.is_empty() {
                String::new()
            } else if i.is_empty() {
                format!("{b}#()")
            } else {
                format!("{b}#({i})")
            }
        }
        Expression::Bracket(inner) => expr_to_string(inner),
        Expression::LiteralString(s) => s.clone(),
        Expression::LiteralInteger(i) => i.to_string(),
        Expression::LiteralReal(s) => s.clone(),
        Expression::LiteralBoolean(b) => b.to_string(),
        Expression::LiteralWithUnit { value, unit } => {
            let v = expr_to_string(value);
            let u = expr_to_string(unit);
            if u.is_empty() {
                v
            } else {
                format!("{v} [{u}]")
            }
        }
        Expression::BinaryOp { op, left, right } => {
            format!(
                "({} {} {})",
                expr_to_string(left),
                op,
                expr_to_string(right)
            )
        }
        Expression::UnaryOp { op, operand } => format!("({}{})", op, expr_to_string(operand)),
        Expression::Null => String::new(),
    }
}

/// Position DTO for JSON (matches vscode sysmlModelTypes)
#[derive(Debug, Clone, Serialize)]
pub struct PositionDto {
    pub line: u32,
    pub character: u32,
}

/// Range DTO for JSON
#[derive(Debug, Clone, Serialize)]
pub struct RangeDto {
    pub start: PositionDto,
    pub end: PositionDto,
}

fn span_to_range_dto(span: &Span) -> RangeDto {
    let (start_line, start_char, end_line, end_char) = span.to_lsp_range();
    RangeDto {
        start: PositionDto {
            line: start_line,
            character: start_char,
        },
        end: PositionDto {
            line: end_line,
            character: end_char,
        },
    }
}

// ---------------------------------------------------------------------------
// Activity diagrams
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDiagramDto {
    pub name: String,
    pub actions: Vec<ActivityActionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<ActivityInterfaceDto>,
    pub decisions: Vec<DecisionNodeDto>,
    pub flows: Vec<ControlFlowDto>,
    pub states: Vec<ActivityStateDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityActionDto {
    pub name: String,
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityInterfaceDto {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionNodeDto {
    pub name: String,
    pub condition: String,
    pub branches: Vec<BranchDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct BranchDto {
    pub condition: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlFlowDto {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityStateDto {
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
    pub range: RangeDto,
}

// ---------------------------------------------------------------------------
// Extraction
// ---------------------------------------------------------------------------

fn collect_action_defs_from_elements(
    elements: &[sysml_parser::Node<PackageBodyElement>],
) -> Vec<&sysml_parser::Node<sysml_parser::ast::ActionDef>> {
    use sysml_parser::ast::PackageBodyElement as PBE;
    let mut out = Vec::new();
    for node in elements {
        match &node.value {
            PBE::ActionDef(ad) => out.push(ad),
            PBE::Package(p) => {
                if let PackageBody::Brace { elements: inner } = &p.body {
                    out.extend(collect_action_defs_from_elements(inner));
                }
            }
            PBE::LibraryPackage(lp) => {
                if let PackageBody::Brace { elements: inner } = &lp.body {
                    out.extend(collect_action_defs_from_elements(inner));
                }
            }
            _ => {}
        }
    }
    out
}

/// Extracts activity diagrams from ActionDef nodes.
/// Each ActionDef becomes one ActivityDiagramDto; sysml-parser ActionDefBody has InOutDecl only (no statements).
pub fn extract_activity_diagrams(root: &RootNamespace) -> Vec<ActivityDiagramDto> {
    let mut out = Vec::new();
    for node in &root.elements {
        let elements = match &node.value {
            RootElement::Package(p) => match &p.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::Namespace(n) => match &n.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::LibraryPackage(lp) => match &lp.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::Import(_) => continue,
        };
        for action in collect_action_defs_from_elements(elements) {
            out.push(extract_activity_from_action(action));
        }
    }
    out
}

fn extract_activity_from_action(
    node: &sysml_parser::Node<sysml_parser::ast::ActionDef>,
) -> ActivityDiagramDto {
    let name = identification_name(&node.identification);
    let range = span_to_range_dto(&node.span);
    let mut actions = Vec::new();
    let mut flows = Vec::new();
    let mut states = Vec::new();
    let mut interface_inputs = Vec::new();
    let mut interface_outputs = Vec::new();
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
                        sysml_parser::ast::InOut::In => interface_inputs.push(param_name),
                        sysml_parser::ast::InOut::Out => interface_outputs.push(param_name),
                        sysml_parser::ast::InOut::InOut => {
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
                        name: perform_name,
                        action_type: "action".to_string(),
                        kind: Some("perform".to_string()),
                        inputs: None,
                        outputs: None,
                        range: Some(span_to_range_dto(&perform.span)),
                    });
                }
                ActionDefBodyElement::ActionUsage(usage) => {
                    let u = usage.as_ref();
                    let mut inputs = Vec::new();
                    if let Some((ref accept_name, _accept_type)) = &u.accept {
                        inputs.push(accept_name.clone());
                    }
                    actions.push(ActivityActionDto {
                        name: u.name.clone(),
                        action_type: "action".to_string(),
                        // VS Code Action Flow view filters allowed node kinds. Use a compatible kind
                        // for action usages so they appear as regular action nodes.
                        kind: Some("action".to_string()),
                        inputs: if inputs.is_empty() {
                            None
                        } else {
                            Some(inputs)
                        },
                        outputs: None,
                        range: Some(span_to_range_dto(&u.span)),
                    });
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
                ActionDefBodyElement::Flow(flow) => {
                    let from = expr_to_string(&flow.value.from);
                    let to = expr_to_string(&flow.value.to);
                    if !from.is_empty() && !to.is_empty() {
                        flows.push(ControlFlowDto {
                            from,
                            to,
                            condition: None,
                            guard: Some("flow".to_string()),
                            range: span_to_range_dto(&flow.span),
                        });
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
            name: step,
            action_type: "action".to_string(),
            kind: Some("action".to_string()),
            inputs: None,
            outputs: None,
            range: None,
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
        name: if name.is_empty() {
            "action".to_string()
        } else {
            name
        },
        actions,
        interface,
        decisions: vec![],
        flows,
        states,
        range,
    }
}

#[cfg(test)]
mod tests {
    use super::extract_activity_diagrams;
    use sysml_parser::parse;

    #[test]
    fn extract_activity_diagrams_exposes_in_out_as_interface_metadata() {
        let input = r#"
            package P {
                action def UpdateDisplay {
                    in currentTime : TimeValue;
                    out displayText : String;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "UpdateDisplay")
            .expect("diagram");

        assert!(
            diagram.actions.is_empty(),
            "interface declarations should not become flow steps"
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["currentTime".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["displayText".to_string()])
        );
        assert!(
            diagram.flows.is_empty(),
            "should not synthesize pseudo-flows"
        );
    }

    #[test]
    fn extract_activity_diagrams_includes_perform_steps() {
        let input = r#"
            package P {
                action def ExecuteMission {
                    in route : Route;
                    perform action captureVideo : CaptureVideo;
                    out report : MissionReport;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");
        let action_names: Vec<_> = diagram.actions.iter().map(|a| a.name.as_str()).collect();

        assert_eq!(action_names, vec!["captureVideo"]);
        assert_eq!(diagram.actions[0].kind.as_deref(), Some("perform"));
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["route".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["report".to_string()])
        );
        assert!(
            diagram.flows.is_empty(),
            "perform-only diagrams should not invent ordering edges"
        );
    }

    #[test]
    fn extract_activity_diagrams_includes_usage_bind_and_flows() {
        let input = r#"
            package P {
                action def ExecuteMission {
                    in route : Route;
                    action captureVideo : CaptureVideo;
                    bind route = captureVideo;
                    flow captureVideo to route;
                    first captureVideo then route;
                    merge route;
                    out report : MissionReport;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");

        assert!(
            diagram
                .actions
                .iter()
                .any(|a| a.name == "captureVideo" && a.kind.as_deref() == Some("action")),
            "expected action usage step to be emitted as a regular action node kind"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("bind")),
            "expected bind to be represented as a guarded flow edge"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("flow")),
            "expected flow statement edge"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("first")),
            "expected first/then edge"
        );
        assert!(
            diagram.states.iter().any(|s| s.state_type == "merge"),
            "expected merge node"
        );
    }

    #[test]
    fn extract_activity_diagrams_with_only_interface_have_no_behavior_nodes() {
        let input = r#"
            package P {
                action def ValidateRoute {
                    in route : Route;
                    out isValid : Boolean;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ValidateRoute")
            .expect("diagram");

        assert!(diagram.actions.is_empty());
        assert!(diagram.flows.is_empty());
        assert!(diagram.states.is_empty());
        assert!(diagram.interface.is_some());
    }

    #[test]
    fn extract_activity_diagrams_finds_action_defs_in_library_package() {
        let input = r#"
            standard library package P {
                action def ExecuteMission {
                    perform action captureVideo : CaptureVideo;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        assert!(
            diagrams.iter().any(|d| d.name == "ExecuteMission"),
            "expected action def inside library package to be discovered; diagrams: {:?}",
            diagrams.iter().map(|d| d.name.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn extract_activity_diagrams_synthesizes_nodes_referenced_by_first_then() {
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

        assert!(
            diagram.actions.iter().any(|a| a.name == "validateRoute"),
            "expected referenced step node validateRoute to exist"
        );
        assert!(
            diagram.actions.iter().any(|a| a.name == "startMission"),
            "expected referenced step node startMission to exist"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("first")
                    && f.from == "validateRoute"
                    && f.to == "startMission"),
            "expected first/then flow edge"
        );
    }

    #[test]
    fn extract_activity_diagrams_does_not_synthesize_interface_parameters_as_step_nodes() {
        let input = r#"
            package P {
                action def ExecutePatrol {
                    in route : String;
                    out status : String;

                    action finishMission { out missionStatus : String; };
                    bind status = finishMission::missionStatus;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecutePatrol")
            .expect("diagram");

        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["route".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["status".to_string()])
        );

        assert!(
            diagram
                .actions
                .iter()
                .all(|a| a.name != "route" && a.name != "status"),
            "interface parameters should not be synthesized into action nodes; actions={:?}",
            diagram
                .actions
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
        );
    }
}
