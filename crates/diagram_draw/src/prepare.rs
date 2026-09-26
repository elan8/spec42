//! Port of `prepareViewData` and every shipped-view prepare path (general, interconnection,
//! sequence, action-flow, state-transition, browser, grid, geometry), including schema-5 typed
//! products.

use std::collections::{HashMap, HashSet};

use serde_json::{json, Map, Value};

use crate::graph_normalization::{
    is_overview_visual_element_type, is_package_element_type, normalize_edge_kind,
};
use crate::json_util::{as_array, as_array_opt, as_object, as_string, field, first_present};

pub fn prepare_view_data(input: &Value) -> Result<Value, String> {
    if let Some(typed) = prepare_typed_diagram_product(input) {
        return typed;
    }
    Ok(prepare_legacy_view_data(input))
}

fn prepare_legacy_view_data(input: &Value) -> Value {
    let passthrough = field(input, "preparedView");
    if passthrough.is_object() {
        let view = field(passthrough, "view");
        let nodes = field(passthrough, "nodes");
        let edges = field(passthrough, "edges");
        if view.is_string() && nodes.is_array() && edges.is_array() {
            return passthrough.clone();
        }
    }
    let visualization = normalize_visualization_payload(input);
    let view = as_string(field(&visualization, "view"), "general-view");
    match view.as_str() {
        "interconnection-view" => prepare_interconnection(&visualization),
        "action-flow-view" => prepare_activity(&visualization),
        "state-transition-view" => prepare_state(&visualization),
        "sequence-view" => prepare_sequence(&visualization),
        "browser-view" => prepare_browser(&visualization),
        "grid-view" => prepare_grid(&visualization),
        "geometry-view" => prepare_geometry(&visualization),
        _ => prepare_graph(
            if field(&visualization, "generalViewGraph").is_object() {
                field(&visualization, "generalViewGraph")
            } else {
                field(&visualization, "graph")
            },
            &visualization,
        ),
    }
}

fn normalize_visualization_payload(data: &Value) -> Value {
    if data.is_null() {
        return Value::Null;
    }
    let view = as_string(field(data, "view"), "general-view");
    match view.as_str() {
        "general-view" => data.clone(),
        "interconnection-view" => {
            if field(data, "interconnectionScene").is_object() {
                data.clone()
            } else {
                let mut object = as_object(data);
                object.insert("elements".into(), json!([]));
                object.insert("parts".into(), json!([]));
                object.insert("ports".into(), json!([]));
                object.insert("connectors".into(), json!([]));
                object.insert("containerGroups".into(), json!([]));
                object.insert("packageContainerGroups".into(), json!([]));
                Value::Object(object)
            }
        }
        "action-flow-view" => {
            let diagrams: Vec<Value> = as_array(field(data, "activityDiagrams"))
                .iter()
                .map(|diagram| {
                    let mut object = as_object(diagram);
                    let nodes = if object.contains_key("nodes") {
                        object.get("nodes").cloned().unwrap_or(json!([]))
                    } else {
                        object.get("actions").cloned().unwrap_or(json!([]))
                    };
                    let flow_count = as_array_opt(object.get("flows")).len();
                    let node_count = as_array(&nodes).len();
                    object.insert("nodes".into(), nodes);
                    object.insert("hasBehavioralFlow".into(), json!(flow_count > 0));
                    object.insert(
                        "hasRenderableContent".into(),
                        json!(flow_count > 0 && node_count > 0),
                    );
                    Value::Object(object)
                })
                .collect();
            let mut object = as_object(data);
            object.insert("diagrams".into(), Value::Array(diagrams));
            Value::Object(object)
        }
        "state-transition-view" => {
            let state_machines = as_array(field(data, "stateMachines")).to_vec();
            let states: Vec<Value> = state_machines
                .iter()
                .flat_map(|machine| as_array(field(machine, "states")).iter().cloned())
                .collect();
            let transitions: Vec<Value> = state_machines
                .iter()
                .flat_map(|machine| as_array(field(machine, "transitions")).iter().cloned())
                .collect();
            let mut object = as_object(data);
            object.insert("stateMachines".into(), Value::Array(state_machines));
            object.insert("states".into(), Value::Array(states));
            object.insert("transitions".into(), Value::Array(transitions));
            Value::Object(object)
        }
        "sequence-view" => {
            let mut object = as_object(data);
            object.insert("diagrams".into(), field(data, "sequenceDiagrams").clone());
            Value::Object(object)
        }
        _ => data.clone(),
    }
}

fn element_type_of(node: &Value) -> String {
    let attrs = field(node, "attributes");
    as_string(
        first_present([
            field(node, "type"),
            field(node, "element_type"),
            field(node, "element_kind"),
            field(attrs, "element_type"),
            field(attrs, "element_kind"),
            field(attrs, "elementKind"),
        ]),
        "",
    )
}

fn is_package(node: &Value) -> bool {
    is_package_element_type(&element_type_of(node))
}

fn node_uri(node: &Value) -> Value {
    match js_nonempty_string(first_present([
        field(node, "uri"),
        field(node, "sourcePath"),
        field(node, "source_path"),
    ])) {
        Some(uri) => Value::String(uri),
        None => Value::Null,
    }
}

fn js_nonempty_string(value: &Value) -> Option<String> {
    let text = as_string(value, "");
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn is_synthetic_package(node: &Value) -> bool {
    if !is_package(node) {
        return false;
    }
    let attrs = field(node, "attributes");
    bool_flag(field(node, "synthetic"))
        || bool_flag(field(node, "isSynthetic"))
        || bool_flag(field(attrs, "synthetic"))
        || bool_flag(field(attrs, "isSyntheticContainer"))
}

fn bool_flag(value: &Value) -> bool {
    match value {
        Value::Bool(true) => true,
        Value::String(s) if s == "true" => true,
        _ => false,
    }
}

fn legacy_notation_role(kind: &str) -> &'static str {
    let normalized = kind.trim().to_ascii_lowercase();
    if normalized == "ref" || normalized.ends_with("-ref") || normalized.ends_with(" ref") {
        "reference-usage"
    } else if normalized.contains(" def")
        || normalized.contains("_def")
        || normalized.contains("definition")
    {
        "definition"
    } else if normalized == "package" {
        "namespace"
    } else {
        "usage"
    }
}

fn node_range(node: &Value) -> Value {
    let range = field(node, "range");
    if range.is_null() {
        Value::Null
    } else {
        range.clone()
    }
}

fn build_behavior_node(node: &Value, defaults: (&str, &str, &str)) -> Value {
    let attrs = as_object(field(node, "attributes"));
    let qualified_name = as_string(
        first_present([
            field(node, "qualifiedName"),
            field(field(node, "attributes"), "qualifiedName"),
            field(node, "id"),
        ]),
        "",
    );
    let mut attributes = attrs;
    if !qualified_name.is_empty() {
        attributes.insert("qualifiedName".into(), Value::String(qualified_name));
    }
    if !field(node, "parentId").is_null() {
        attributes.insert("parentId".into(), field(node, "parentId").clone());
    }
    if !field(node, "parent").is_null() {
        attributes.insert("parent".into(), field(node, "parent").clone());
    }
    json!({
        "id": as_string(first_present([field(node, "id"), field(node, "name")]), defaults.0),
        "label": as_string(
            first_present([field(node, "name"), field(node, "label"), field(node, "id")]),
            defaults.1,
        ),
        "kind": defaults.2,
        "sourcePath": node_uri(node),
        "uri": node_uri(node),
        "range": node_range(node),
        "attributes": attributes,
    })
}

fn build_general_package_container_groups(nodes: &[Value]) -> Vec<Value> {
    let mut order: Vec<String> = Vec::new();
    let mut by_package: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes {
        let attrs = field(node, "attributes");
        let qn = {
            let from_attr = as_string(field(attrs, "qualifiedName"), "");
            if from_attr.is_empty() {
                as_string(field(node, "id"), "")
            } else {
                from_attr
            }
        };
        let Some(sep) = qn.find("::") else {
            continue;
        };
        if sep == 0 {
            continue;
        }
        let pkg = qn[..sep].to_string();
        by_package
            .entry(pkg.clone())
            .or_insert_with(|| {
                order.push(pkg.clone());
                Vec::new()
            })
            .push(as_string(field(node, "id"), ""));
    }
    if order.len() < 2 {
        return Vec::new();
    }
    order
        .into_iter()
        .map(|name| {
            let member_ids = by_package.remove(&name).unwrap_or_default();
            json!({
                "id": format!("package:{name}"),
                "name": name,
                "memberIds": member_ids,
            })
        })
        .collect()
}

fn prepare_graph(graph_input: &Value, visualization: &Value) -> Value {
    let raw_nodes = as_array(field(graph_input, "nodes"));
    let source_nodes: Vec<&Value> = raw_nodes
        .iter()
        .filter(|node| {
            if is_synthetic_package(node) {
                return false;
            }
            is_overview_visual_element_type(&element_type_of(node))
        })
        .collect();
    let node_ids: HashSet<String> = source_nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let nodes: Vec<Value> = source_nodes
        .iter()
        .map(|node| {
            let mut attributes = as_object(field(node, "attributes"));
            attributes.insert(
                "qualifiedName".into(),
                Value::String(as_string(
                    first_present([
                        field(node, "qualifiedName"),
                        field(field(node, "attributes"), "qualifiedName"),
                    ]),
                    "",
                )),
            );
            attributes.insert("isPackage".into(), json!(is_package(node)));
            attributes.insert(
                "notationRole".into(),
                Value::String(
                    legacy_notation_role(&as_string(
                        first_present([field(node, "type"), field(node, "element_type")]),
                        "",
                    ))
                    .to_string(),
                ),
            );
            let kind = {
                let kind = element_type_of(node);
                if kind.is_empty() {
                    "Element".to_string()
                } else {
                    kind
                }
            };
            json!({
                "id": as_string(field(node, "id"), ""),
                "label": as_string(
                    first_present([field(node, "name"), field(node, "qualifiedName"), field(node, "id")]),
                    "Unnamed",
                ),
                "kind": kind,
                "sourcePath": js_nonempty_string(first_present([
                    field(node, "sourcePath"),
                    field(node, "source_path"),
                ])).map(Value::String).unwrap_or(Value::Null),
                "uri": node_uri(node),
                "range": node_range(node),
                "attributes": attributes,
            })
        })
        .collect();
    let edges: Vec<Value> = as_array(field(graph_input, "edges"))
        .iter()
        .enumerate()
        .filter(|(_, edge)| {
            node_ids.contains(&as_string(field(edge, "source"), ""))
                && node_ids.contains(&as_string(field(edge, "target"), ""))
        })
        .map(|(index, edge)| {
            let relation_type = as_string(
                first_present([
                    field(edge, "type"),
                    field(edge, "rel_type"),
                    field(edge, "relationType"),
                    field(edge, "name"),
                ]),
                "",
            );
            let label = as_string(
                first_present([
                    field(edge, "name"),
                    field(edge, "label"),
                    field(edge, "type"),
                    field(edge, "rel_type"),
                ]),
                "",
            );
            let kind = normalize_edge_kind(&relation_type);
            let mut attributes = as_object(field(edge, "attributes"));
            attributes.insert("relationType".into(), Value::String(kind.clone()));
            json!({
                "id": as_string(field(edge, "id"), &format!("edge-{index}")),
                "source": as_string(field(edge, "source"), ""),
                "target": as_string(field(edge, "target"), ""),
                "label": label,
                "edgeKind": kind,
                "attributes": attributes,
            })
        })
        .collect();
    let package_container_groups = build_general_package_container_groups(&nodes);
    let mut prepared = json!({
        "title": as_string(field(visualization, "selectedViewName"), "SysML View"),
        "view": as_string(field(visualization, "view"), "general-view"),
        "nodes": nodes,
        "edges": edges,
    });
    if !package_container_groups.is_empty() {
        prepared["meta"] = json!({ "packageContainerGroups": package_container_groups });
    }
    prepared
}

fn normalize_diagram_key(value: &str) -> String {
    value.replace("::", ".").trim().to_ascii_lowercase()
}

fn diagram_simple_name(value: &str) -> String {
    let normalized = value.replace("::", ".");
    normalized
        .split('.')
        .rfind(|segment| !segment.is_empty())
        .unwrap_or(&normalized)
        .to_string()
}

fn diagram_matches_selection(diagram: &Value, selected_name: &str, selected_view_id: &str) -> bool {
    let selectors: Vec<&str> = [selected_name, selected_view_id]
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect();
    if selectors.is_empty() {
        return false;
    }
    let package_path = as_string(field(diagram, "package_path"), "");
    let name = as_string(field(diagram, "name"), "");
    let qualified = format!("{package_path}::{name}")
        .trim_start_matches(':')
        .to_string();
    let diagram_keys = [as_string(field(diagram, "id"), ""), name, qualified]
        .into_iter()
        .filter(|key| !key.is_empty())
        .collect::<Vec<_>>();
    selectors.iter().any(|selector| {
        let selector_key = normalize_diagram_key(selector);
        let selector_simple = diagram_simple_name(selector).to_ascii_lowercase();
        diagram_keys.iter().any(|candidate| {
            let candidate_key = normalize_diagram_key(candidate);
            let candidate_simple = diagram_simple_name(candidate).to_ascii_lowercase();
            candidate_key == selector_key
                || candidate_simple == selector_simple
                || candidate_key.ends_with(&format!(".{selector_key}"))
                || selector_key.ends_with(&format!(".{candidate_key}"))
                || candidate_key.contains(&selector_simple)
                || selector_key.contains(&candidate_simple)
        })
    })
}

fn select_named_diagram(
    diagrams_input: &Value,
    selected_name: &str,
    selected_view_id: &str,
) -> Option<Value> {
    let diagrams: Vec<Value> = as_array(diagrams_input).to_vec();
    if diagrams.is_empty() {
        return None;
    }
    if selected_name.is_empty() && selected_view_id.is_empty() {
        return None;
    }
    if let Some(matched) = diagrams
        .iter()
        .find(|diagram| diagram_matches_selection(diagram, selected_name, selected_view_id))
    {
        return Some(matched.clone());
    }
    if diagrams.len() == 1 {
        Some(diagrams[0].clone())
    } else {
        None
    }
}

fn behavior_diagram_score(diagram: &Value) -> usize {
    let nodes = as_array(first_present([
        field(diagram, "nodes"),
        field(diagram, "actions"),
        field(diagram, "steps"),
    ]))
    .len();
    let edges = as_array(first_present([
        field(diagram, "edges"),
        field(diagram, "flows"),
        field(diagram, "transitions"),
    ]))
    .len();
    nodes * 10 + edges
}

fn best_behavior_diagram(diagrams: &[Value]) -> Option<Value> {
    diagrams
        .iter()
        .fold(None, |best: Option<&Value>, diagram| match best {
            None => Some(diagram),
            Some(current) if behavior_diagram_score(diagram) > behavior_diagram_score(current) => {
                Some(diagram)
            }
            Some(current) => Some(current),
        })
        .cloned()
}

fn diagram_to_prepared(diagram_input: &Value, view: &str, fallback_title: &str) -> Value {
    let mut nodes: Vec<Value> = as_array(first_present([
        field(diagram_input, "nodes"),
        field(diagram_input, "states"),
    ]))
    .iter()
    .enumerate()
    .map(|(index, node)| {
        let kind = as_string(
            first_present([field(node, "type"), field(node, "kind")]),
            view,
        );
        build_behavior_node(
            node,
            (
                &format!("node-{index}"),
                &format!("Node {}", index + 1),
                &kind,
            ),
        )
    })
    .collect();
    let mut edges: Vec<Value> = as_array(first_present([
        field(diagram_input, "edges"),
        field(diagram_input, "transitions"),
    ]))
    .iter()
    .enumerate()
    .map(|(index, edge)| {
        json!({
            "id": as_string(field(edge, "id"), &format!("edge-{index}")),
            "source": as_string(first_present([
                field(edge, "source"),
                field(edge, "from"),
                field(edge, "sourceId"),
            ]), ""),
            "target": as_string(first_present([
                field(edge, "target"),
                field(edge, "to"),
                field(edge, "targetId"),
            ]), ""),
            "label": as_string(first_present([
                field(edge, "name"),
                field(edge, "label"),
                field(edge, "type"),
            ]), ""),
        })
    })
    .collect();
    if view == "sequence-view" && nodes.is_empty() {
        nodes = as_array(field(diagram_input, "lifelines"))
            .iter()
            .enumerate()
            .map(|(index, lifeline)| {
                build_behavior_node(
                    lifeline,
                    (
                        &format!("lifeline-{index}"),
                        &format!("Lifeline {}", index + 1),
                        "lifeline",
                    ),
                )
            })
            .collect();
        edges = as_array(field(diagram_input, "messages"))
            .iter()
            .enumerate()
            .map(|(index, message)| {
                json!({
                    "id": as_string(field(message, "id"), &format!("message-{index}")),
                    "source": as_string(first_present([
                        field(message, "source"),
                        field(message, "from"),
                        field(message, "sourceId"),
                    ]), ""),
                    "target": as_string(first_present([
                        field(message, "target"),
                        field(message, "to"),
                        field(message, "targetId"),
                    ]), ""),
                    "label": as_string(first_present([
                        field(message, "name"),
                        field(message, "label"),
                        field(message, "type"),
                    ]), ""),
                    "attributes": {
                        "messageKind": as_string(first_present([
                            field(message, "kind"),
                            field(message, "type"),
                        ]), ""),
                        "order": field(message, "order"),
                    },
                })
            })
            .collect();
    }
    let ids: HashSet<String> = nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    edges.retain(|edge| {
        ids.contains(&as_string(field(edge, "source"), ""))
            && ids.contains(&as_string(field(edge, "target"), ""))
    });
    json!({
        "title": as_string(field(diagram_input, "name"), fallback_title),
        "view": view,
        "nodes": nodes,
        "edges": edges,
    })
}

fn build_activity_node_alias_map(nodes: &[Value]) -> HashMap<String, String> {
    let mut aliases = HashMap::new();
    let mut register = |alias: &Value, node_id: &str| {
        let key = as_string(alias, "").trim().to_string();
        if key.is_empty() {
            return;
        }
        aliases
            .entry(key.clone())
            .or_insert_with(|| node_id.to_string());
        let normalized = key.replace("::", ".");
        aliases
            .entry(normalized.clone())
            .or_insert_with(|| node_id.to_string());
        if let Some(last) = normalized.split('.').rfind(|s| !s.is_empty()) {
            aliases
                .entry(last.to_string())
                .or_insert_with(|| node_id.to_string());
        }
    };
    for node in nodes {
        let id = as_string(field(node, "id"), "");
        register(field(node, "id"), &id);
        register(field(node, "label"), &id);
        register(field(field(node, "attributes"), "qualifiedName"), &id);
    }
    aliases
}

fn resolve_activity_node_ref(value: &Value, aliases: &HashMap<String, String>) -> String {
    let key = as_string(value, "").trim().to_string();
    if key.is_empty() {
        return String::new();
    }
    let normalized = key.replace("::", ".");
    let segments: Vec<&str> = normalized.split('.').filter(|s| !s.is_empty()).collect();
    let last = segments.last().copied().unwrap_or("");
    let first = segments.first().copied().unwrap_or("");
    aliases
        .get(&key)
        .or_else(|| aliases.get(&normalized))
        .or_else(|| {
            if last.is_empty() {
                None
            } else {
                aliases.get(last)
            }
        })
        .or_else(|| {
            if first.is_empty() {
                None
            } else {
                aliases.get(first)
            }
        })
        .cloned()
        .unwrap_or(key)
}

fn prepare_sequence(visualization: &Value) -> Value {
    let selected_name = as_string(field(visualization, "selectedViewName"), "");
    let selected_view = as_string(field(visualization, "selectedView"), "");
    let selected = select_named_diagram(
        field(visualization, "sequenceDiagrams"),
        &selected_name,
        &selected_view,
    );
    let fallback = as_array(field(visualization, "sequenceDiagrams"))
        .first()
        .cloned();
    let effective = selected.or(fallback);
    if let Some(effective) = effective {
        let mut prepared = diagram_to_prepared(&effective, "sequence-view", "Sequence View");
        prepared["meta"] = json!({
            "selectedDiagramName": as_string(field(&effective, "name"), ""),
            "sequenceDiagram": effective,
            "parentContext": as_string(field(&effective, "name"), ""),
        });
        prepared
    } else {
        prepare_graph(field(visualization, "graph"), visualization)
    }
}

fn collect_activity_nodes(diagram: &Value) -> Vec<Value> {
    let allowed: HashSet<&str> = [
        "action",
        "perform",
        "assign",
        "for-loop",
        "decision",
        "merge",
        "fork",
        "join",
        "initial",
        "final",
        "terminate",
        "accept",
        "send",
    ]
    .into_iter()
    .collect();
    let decisions: Vec<Value> = as_array(field(diagram, "decisions"))
        .iter()
        .enumerate()
        .map(|(index, node)| {
            build_behavior_node(node, (&format!("decision-{index}"), "Decision", "decision"))
        })
        .collect();
    let control_tokens = [
        "initial",
        "final",
        "decision",
        "merge",
        "fork",
        "join",
        "assign",
        "for-loop",
        "terminate",
        "accept",
        "send",
    ];
    let states: Vec<Value> = as_array(field(diagram, "states"))
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let kind = as_string(
                first_present([
                    field(node, "type"),
                    field(node, "stateType"),
                    field(node, "kind"),
                ]),
                "state",
            )
            .to_ascii_lowercase();
            build_behavior_node(
                node,
                (
                    &format!("state-{index}"),
                    &format!("State {}", index + 1),
                    &kind,
                ),
            )
        })
        .filter(|node| {
            let kind = as_string(field(node, "kind"), "");
            control_tokens.iter().any(|token| kind.contains(token))
        })
        .collect();
    let actions: Vec<Value> = as_array(first_present([
        field(diagram, "nodes"),
        field(diagram, "actions"),
        field(diagram, "steps"),
    ]))
    .iter()
    .enumerate()
    .map(|(index, node)| {
        let kind = as_string(
            first_present([
                field(node, "kind"),
                field(node, "type"),
                field(node, "action_type"),
            ]),
            "action",
        )
        .to_ascii_lowercase();
        let normalized_kind = if kind.contains("perform") {
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
        };
        build_behavior_node(
            node,
            (
                &format!("action-{index}"),
                &format!("Action {}", index + 1),
                normalized_kind,
            ),
        )
    })
    .collect();
    let mut enriched: Vec<Value> = actions.into_iter().chain(decisions).chain(states).collect();
    for node in &mut enriched {
        let attrs = field(node, "attributes");
        let swim_lane = as_string(
            first_present([field(attrs, "swimLane"), field(attrs, "swim_lane")]),
            "",
        );
        if !swim_lane.is_empty() {
            let mut attributes = as_object(attrs);
            attributes.insert("swimLane".into(), Value::String(swim_lane));
            node["attributes"] = Value::Object(attributes);
        }
    }
    enriched
        .into_iter()
        .filter(|node| allowed.contains(as_string(field(node, "kind"), "").as_str()))
        .collect()
}

fn prepare_activity(visualization: &Value) -> Value {
    let catalog: Vec<Value> = {
        let diagrams = as_array(field(visualization, "diagrams"));
        if !diagrams.is_empty() {
            diagrams.to_vec()
        } else {
            as_array(field(visualization, "activityDiagrams")).to_vec()
        }
    };
    let selected_name = as_string(field(visualization, "selectedViewName"), "");
    let selected_view = as_string(field(visualization, "selectedView"), "");
    let selected = select_named_diagram(
        &Value::Array(catalog.clone()),
        &selected_name,
        &selected_view,
    );
    let effective = selected.or_else(|| best_behavior_diagram(&catalog));
    let diagram = effective.clone().unwrap_or(json!({}));
    let nodes = collect_activity_nodes(&diagram);
    let node_ids: HashSet<String> = nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let aliases = build_activity_node_alias_map(&nodes);
    let edges: Vec<Value> = as_array(first_present([
        field(&diagram, "flows"),
        field(&diagram, "edges"),
        field(&diagram, "transitions"),
    ]))
    .iter()
    .enumerate()
    .filter_map(|(index, edge)| {
        let source = resolve_activity_node_ref(
            first_present([
                field(edge, "from"),
                field(edge, "source"),
                field(edge, "sourceId"),
            ]),
            &aliases,
        );
        let target = resolve_activity_node_ref(
            first_present([
                field(edge, "to"),
                field(edge, "target"),
                field(edge, "targetId"),
            ]),
            &aliases,
        );
        let guard = as_string(
            first_present([field(edge, "guard"), field(edge, "type")]),
            "",
        );
        let condition = as_string(field(edge, "condition"), "");
        let guard_lower = guard.to_ascii_lowercase();
        let succession = guard_lower == "first"
            || guard_lower == "succession"
            || guard_lower == "succession flow";
        let streaming_flow = guard_lower == "flow";
        let conditional = !condition.is_empty()
            || (!guard.is_empty()
                && !["flow", "first", "bind", "perform", "succession"]
                    .contains(&guard_lower.as_str()));
        if source.is_empty()
            || target.is_empty()
            || source == target
            || !node_ids.contains(&source)
            || !node_ids.contains(&target)
        {
            return None;
        }
        let mut attributes = Map::new();
        if !guard.is_empty() {
            attributes.insert("guard".into(), Value::String(guard.clone()));
        }
        if !condition.is_empty() {
            attributes.insert("condition".into(), Value::String(condition.clone()));
        }
        attributes.insert("succession".into(), json!(succession));
        attributes.insert("streamingFlow".into(), json!(streaming_flow));
        attributes.insert(
            "flowKind".into(),
            Value::String(if succession {
                "succession".into()
            } else if streaming_flow {
                "streaming".into()
            } else {
                "other".into()
            }),
        );
        attributes.insert("conditional".into(), json!(conditional));
        Some(json!({
            "id": as_string(field(edge, "id"), &format!("flow-{index}")),
            "source": source,
            "target": target,
            "label": as_string(first_present([
                field(edge, "name"),
                field(edge, "label"),
                field(edge, "condition"),
                field(edge, "guard"),
            ]), ""),
            "attributes": attributes,
        }))
    })
    .collect();
    let mut swim_lanes = Vec::new();
    let mut seen_lanes = HashSet::new();
    for node in &nodes {
        let lane = as_string(field(field(node, "attributes"), "swimLane"), "");
        if !lane.is_empty() && seen_lanes.insert(lane.clone()) {
            swim_lanes.push(lane);
        }
    }
    json!({
        "title": as_string(
            first_present([field(&diagram, "name"), field(visualization, "selectedViewName")]),
            "Action Flow View",
        ),
        "view": "action-flow-view",
        "nodes": nodes,
        "edges": edges,
        "meta": {
            "selectedDiagramId": as_string(field(&diagram, "id"), ""),
            "nodeCount": nodes.len(),
            "edgeCount": edges.len(),
            "layoutDirection": as_string(field(visualization, "activityLayoutDirection"), "vertical"),
            "activityDiagram": effective,
            "parentContext": as_string(field(&diagram, "name"), ""),
            "swimLanes": swim_lanes,
        },
    })
}

fn attach_composite_regions(nodes: Vec<Value>) -> Vec<Value> {
    let mut children_by_region: HashMap<String, Vec<Value>> = HashMap::new();
    for node in &nodes {
        let region_id = as_string(field(field(node, "attributes"), "regionId"), "");
        if region_id.is_empty() {
            continue;
        }
        children_by_region
            .entry(region_id)
            .or_default()
            .push(node.clone());
    }
    nodes
        .into_iter()
        .map(|mut node| {
            if !as_string(field(&node, "kind"), "").contains("composite") {
                return node;
            }
            let id = as_string(field(&node, "id"), "");
            let Some(children) = children_by_region.get(&id) else {
                return node;
            };
            if children.is_empty() {
                return node;
            }
            let mut attributes = as_object(field(&node, "attributes"));
            attributes.insert(
                "regions".into(),
                json!(children
                    .iter()
                    .map(|child| json!({
                        "name": field(child, "label"),
                        "id": field(child, "id"),
                    }))
                    .collect::<Vec<_>>()),
            );
            node["attributes"] = Value::Object(attributes);
            node
        })
        .collect()
}

fn attach_explicit_regions(nodes: Vec<Value>, machine: &Value) -> Vec<Value> {
    let regions = as_array(field(machine, "regions"));
    if regions.is_empty() {
        return attach_composite_regions(nodes);
    }
    let mut regions_by_parent: HashMap<String, Vec<Value>> = HashMap::new();
    for region in regions {
        let parent_id = as_string(
            first_present([field(region, "parentId"), field(region, "parent_id")]),
            "",
        );
        if parent_id.is_empty() {
            continue;
        }
        regions_by_parent
            .entry(parent_id)
            .or_default()
            .push(region.clone());
    }
    nodes
        .into_iter()
        .map(|mut node| {
            if !as_string(field(&node, "kind"), "").contains("composite") {
                return node;
            }
            let id = as_string(field(&node, "id"), "");
            let Some(explicit) = regions_by_parent.get(&id) else {
                return node;
            };
            if explicit.is_empty() {
                return node;
            }
            let mut attributes = as_object(field(&node, "attributes"));
            attributes.insert(
                "regions".into(),
                json!(explicit
                    .iter()
                    .enumerate()
                    .map(|(index, region)| json!({
                        "id": as_string(field(region, "id"), &format!("region-{index}")),
                        "name": as_string(field(region, "name"), &format!("region {}", index + 1)),
                    }))
                    .collect::<Vec<_>>()),
            );
            node["attributes"] = Value::Object(attributes);
            node
        })
        .collect()
}

fn format_state_transition_label(edge: &Value) -> String {
    let mut parts = Vec::new();
    let guard = as_string(field(edge, "guard"), "").trim().to_string();
    let effect = as_string(field(edge, "effect"), "").trim().to_string();
    let accept = as_string(field(edge, "accept"), "").trim().to_string();
    let send = as_string(field(edge, "send"), "").trim().to_string();
    let label = as_string(field(edge, "label"), "").trim().to_string();
    if !guard.is_empty() {
        parts.push(format!("[{guard}]"));
    }
    if !effect.is_empty() {
        parts.push(effect);
    }
    if !accept.is_empty() {
        parts.push(format!("accept {accept}"));
    }
    if !send.is_empty() {
        parts.push(format!("send {send}"));
    }
    if !parts.is_empty() {
        parts.join(" / ")
    } else {
        label
    }
}

fn collect_state_machine_nodes(machine: &Value) -> Vec<Value> {
    let nodes: Vec<Value> = as_array(field(machine, "states"))
        .iter()
        .enumerate()
        .map(|(index, state)| {
            let element = field(state, "element");
            let mut merged = as_object(element);
            if let Some(object) = state.as_object() {
                for (key, value) in object {
                    merged.insert(key.clone(), value.clone());
                }
            }
            merged.insert(
                "id".into(),
                first_present([field(state, "id"), field(element, "id")]).clone(),
            );
            merged.insert(
                "name".into(),
                first_present([field(state, "name"), field(element, "name")]).clone(),
            );
            merged.insert(
                "range".into(),
                first_present([field(element, "range"), field(state, "range")]).clone(),
            );
            merged.insert(
                "uri".into(),
                first_present([
                    field(element, "uri"),
                    field(state, "uri"),
                    field(element, "sourcePath"),
                    field(state, "sourcePath"),
                ])
                .clone(),
            );
            merged.insert(
                "qualifiedName".into(),
                first_present([
                    field(state, "qualifiedName"),
                    field(element, "qualifiedName"),
                    field(state, "id"),
                ])
                .clone(),
            );
            for key in ["entry", "do", "exit"] {
                merged.insert(
                    key.into(),
                    first_present([field(state, key), field(element, key)]).clone(),
                );
            }
            merged.insert(
                "parentId".into(),
                first_present([
                    field(state, "parentId"),
                    field(state, "parent_id"),
                    field(element, "parentId"),
                ])
                .clone(),
            );
            merged.insert(
                "regionId".into(),
                first_present([
                    field(state, "regionId"),
                    field(state, "region_id"),
                    field(element, "regionId"),
                ])
                .clone(),
            );
            let merged_value = Value::Object(merged);
            let kind = as_string(
                first_present([
                    field(state, "kind"),
                    field(state, "type"),
                    field(element, "type"),
                ]),
                "state",
            )
            .to_ascii_lowercase();
            let normalized_kind = if kind.contains("initial") {
                "initial"
            } else if kind.contains("terminate") {
                "terminate"
            } else if kind.contains("final") {
                "final"
            } else if kind.contains("composite") {
                "composite"
            } else {
                "state"
            };
            let mut behavior_node = build_behavior_node(
                &merged_value,
                (&format!("state-{index}"), "State", normalized_kind),
            );
            let entry = as_string(field(&merged_value, "entry"), "");
            let do_action = as_string(field(&merged_value, "do"), "");
            let exit = as_string(field(&merged_value, "exit"), "");
            let region_id = as_string(field(&merged_value, "regionId"), "");
            if !entry.is_empty()
                || !do_action.is_empty()
                || !exit.is_empty()
                || !region_id.is_empty()
            {
                let mut attributes = as_object(field(&behavior_node, "attributes"));
                if !entry.is_empty() {
                    attributes.insert("entry".into(), Value::String(entry));
                }
                if !do_action.is_empty() {
                    attributes.insert("do".into(), Value::String(do_action));
                }
                if !exit.is_empty() {
                    attributes.insert("exit".into(), Value::String(exit));
                }
                if !region_id.is_empty() {
                    attributes.insert("regionId".into(), Value::String(region_id));
                }
                behavior_node["attributes"] = Value::Object(attributes);
            }
            behavior_node
        })
        .collect();
    attach_explicit_regions(nodes, machine)
}

fn prepare_state_machine(machine: &Value, visualization: &Value) -> Value {
    let nodes = collect_state_machine_nodes(machine);
    let node_ids: HashSet<String> = nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let aliases = build_activity_node_alias_map(&nodes);
    let edges: Vec<Value> = as_array(field(machine, "transitions"))
        .iter()
        .enumerate()
        .filter_map(|(index, edge)| {
            let source = resolve_activity_node_ref(
                first_present([
                    field(edge, "source"),
                    field(edge, "sourceName"),
                    field(edge, "from"),
                ]),
                &aliases,
            );
            let target = resolve_activity_node_ref(
                first_present([
                    field(edge, "target"),
                    field(edge, "targetName"),
                    field(edge, "to"),
                ]),
                &aliases,
            );
            if source.is_empty()
                || target.is_empty()
                || !node_ids.contains(&source)
                || !node_ids.contains(&target)
            {
                return None;
            }
            let self_loop = match field(edge, "selfLoop") {
                Value::Bool(value) => *value,
                _ => source == target,
            };
            Some(json!({
                "id": as_string(field(edge, "id"), &format!("transition-{index}")),
                "source": source,
                "target": target,
                "label": format_state_transition_label(edge),
                "attributes": {
                    "selfLoop": self_loop,
                    "relationType": "transition",
                    "trigger": field(edge, "trigger"),
                    "guard": field(edge, "guard"),
                    "effect": field(edge, "effect"),
                    "accept": field(edge, "accept"),
                    "send": field(edge, "send"),
                },
            }))
        })
        .collect();
    json!({
        "title": as_string(
            first_present([field(machine, "name"), field(visualization, "selectedViewName")]),
            "State Transition View",
        ),
        "view": "state-transition-view",
        "nodes": nodes,
        "edges": edges,
        "meta": {
            "selectedDiagramId": as_string(field(machine, "id"), ""),
            "selectedDiagramName": as_string(field(machine, "name"), ""),
            "layoutDirection": as_string(field(visualization, "stateLayoutDirection"), "horizontal"),
            "stateMachine": machine,
            "parentContext": as_string(field(machine, "name"), ""),
        },
    })
}

fn prepare_state(visualization: &Value) -> Value {
    let catalog: Vec<Value> = {
        let machines = as_array(field(visualization, "stateMachines"));
        if !machines.is_empty() {
            machines.to_vec()
        } else {
            as_array(field(visualization, "stateDiagrams")).to_vec()
        }
    };
    if !catalog.is_empty() {
        let selected_name = as_string(field(visualization, "selectedViewName"), "");
        let selected_view = as_string(field(visualization, "selectedView"), "");
        let selected = select_named_diagram(
            &Value::Array(catalog.clone()),
            &selected_name,
            &selected_view,
        );
        if let Some(effective) = selected.or_else(|| catalog.first().cloned()) {
            return prepare_state_machine(&effective, visualization);
        }
    }
    let selected_name = as_string(field(visualization, "selectedViewName"), "");
    let selected_view = as_string(field(visualization, "selectedView"), "");
    if let Some(selected_state_diagram) = select_named_diagram(
        field(visualization, "stateDiagrams"),
        &selected_name,
        &selected_view,
    ) {
        let mut prepared = diagram_to_prepared(
            &selected_state_diagram,
            "state-transition-view",
            "State Transition View",
        );
        prepared["meta"] = json!({
            "selectedDiagramId": as_string(field(&selected_state_diagram, "id"), ""),
            "selectedDiagramName": as_string(field(&selected_state_diagram, "name"), ""),
            "layoutDirection": as_string(field(visualization, "stateLayoutDirection"), "horizontal"),
            "stateDiagram": selected_state_diagram,
        });
        return prepared;
    }
    let graph = field(visualization, "graph");
    let state_nodes: Vec<&Value> = as_array(field(graph, "nodes"))
        .iter()
        .filter(|node| {
            as_string(
                first_present([field(node, "type"), field(node, "element_type")]),
                "",
            )
            .to_ascii_lowercase()
            .contains("state")
        })
        .collect();
    let ids: HashSet<String> = state_nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let nodes: Vec<Value> = state_nodes
        .iter()
        .map(|node| {
            json!({
                "id": as_string(field(node, "id"), ""),
                "label": as_string(first_present([field(node, "name"), field(node, "id")]), "State"),
                "kind": as_string(first_present([field(node, "type"), field(node, "element_type")]), "state"),
                "sourcePath": js_nonempty_string(field(node, "sourcePath")).map(Value::String).unwrap_or(Value::Null),
                "range": node_range(node),
                "attributes": field(node, "attributes"),
            })
        })
        .collect();
    let edges: Vec<Value> = as_array(field(graph, "edges"))
        .iter()
        .enumerate()
        .filter_map(|(index, edge)| {
            let source = as_string(field(edge, "source"), "");
            let target = as_string(field(edge, "target"), "");
            if !ids.contains(&source) || !ids.contains(&target) {
                return None;
            }
            Some(json!({
                "id": format!("transition-{index}"),
                "source": source,
                "target": target,
                "label": as_string(first_present([
                    field(edge, "name"),
                    field(edge, "type"),
                    field(edge, "rel_type"),
                ]), ""),
            }))
        })
        .collect();
    let synthesize_initial = field(visualization, "synthesizeInitialState") == &json!(true);
    let has_initial = nodes.iter().any(|node| {
        as_string(field(node, "kind"), "")
            .to_ascii_lowercase()
            .contains("initial")
            || as_string(field(node, "label"), "").eq_ignore_ascii_case("initial")
    });
    let mut with_synthetic = nodes.clone();
    if synthesize_initial && !has_initial && !nodes.is_empty() {
        with_synthetic.insert(
            0,
            json!({
                "id": "__synthetic_initial__",
                "label": "Initial",
                "kind": "initial",
                "attributes": { "synthetic": true },
            }),
        );
    }
    let ids_with_initial: HashSet<String> = with_synthetic
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let mut edges_with_initial = edges;
    if !has_initial && with_synthetic.len() > 1 {
        edges_with_initial.insert(
            0,
            json!({
                "id": "transition-synthetic-initial",
                "source": "__synthetic_initial__",
                "target": field(&with_synthetic[1], "id"),
                "label": "initial",
            }),
        );
    }
    json!({
        "title": as_string(field(visualization, "selectedViewName"), "State Transition View"),
        "view": "state-transition-view",
        "nodes": with_synthetic.into_iter().filter(|node| ids_with_initial.contains(&as_string(field(node, "id"), ""))).collect::<Vec<_>>(),
        "edges": edges_with_initial.into_iter().filter(|edge| {
            ids_with_initial.contains(&as_string(field(edge, "source"), ""))
                && ids_with_initial.contains(&as_string(field(edge, "target"), ""))
        }).collect::<Vec<_>>(),
        "meta": {
            "syntheticInitial": synthesize_initial && !has_initial && !nodes.is_empty(),
        },
    })
}

fn prepare_interconnection(visualization: &Value) -> Value {
    let scene = field(visualization, "interconnectionScene");
    let schema_version = field(scene, "schemaVersion").as_f64().unwrap_or(0.0);
    if scene.is_object() && schema_version >= 2.0 {
        return prepare_interconnection_scene(scene, visualization);
    }
    json!({
        "title": as_string(field(visualization, "selectedViewName"), "Interconnection View"),
        "view": "interconnection-view",
        "nodes": [],
        "edges": [],
        "meta": {
            "diagnostics": [{
                "severity": "error",
                "code": "missing_interconnection_scene",
                "message": "Interconnection view requires interconnectionScene from the language server.",
            }],
        },
    })
}

fn prepare_interconnection_scene(scene: &Value, visualization: &Value) -> Value {
    let selected_root = select_root(scene);
    let selected_root_has_coverage = selected_root
        .as_ref()
        .is_some_and(|root| root_coverage(root, scene) > 0);
    let mut node_ids: HashSet<String> = as_array(field(scene, "nodes"))
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let mut nodes: Vec<Value> = as_array(field(scene, "nodes"))
        .iter()
        .map(|node| {
            let node_id = as_string(field(node, "id"), "");
            let port_details: Vec<Value> = as_array(field(scene, "ports"))
                .iter()
                .filter(|port| as_string(field(port, "ownerNodeId"), "") == node_id)
                .map(map_port_detail)
                .collect();
            let ports: Vec<Value> = port_details
                .iter()
                .map(|port| field(port, "name").clone())
                .collect();
            json!({
                "id": node_id,
                "label": as_string(field(node, "name"), ""),
                "kind": "part",
                "uri": field(node, "uri"),
                "range": field(node, "range"),
                "attributes": {
                    "containerId": field(node, "parentId"),
                    "qualifiedName": field(node, "qualifiedName"),
                    "semanticId": field(node, "semanticId"),
                    "definitionId": field(node, "definitionId"),
                    "partType": field(node, "typeName"),
                    "ports": ports,
                    "portDetails": port_details,
                    "notationRole": legacy_notation_role(&as_string(field(node, "kind"), "")),
                    "sceneNodeId": field(node, "id"),
                },
            })
        })
        .collect();
    for container in as_array(field(scene, "containers")) {
        let id = as_string(field(container, "id"), "");
        let in_selected_scope = !selected_root_has_coverage
            || selected_root
                .as_ref()
                .is_none_or(|root| id == *root || id.starts_with(&format!("{root}.")));
        if node_ids.contains(&id) || !in_selected_scope {
            continue;
        }
        nodes.push(json!({
            "id": id,
            "label": field(container, "label"),
            "kind": "package",
            "attributes": {
                "isSyntheticContainer": true,
                "containerId": field(container, "parentId"),
                "qualifiedName": field(container, "label"),
                "memberNodeIds": field(container, "memberNodeIds"),
                "layoutDepth": field(container, "depth"),
            },
        }));
        node_ids.insert(as_string(field(container, "id"), ""));
    }
    let edges: Vec<Value> = as_array(field(scene, "edges"))
        .iter()
        .filter(|edge| {
            node_ids.contains(&as_string(field(edge, "sourceNodeId"), ""))
                && node_ids.contains(&as_string(field(edge, "targetNodeId"), ""))
        })
        .map(|edge| {
            json!({
                "id": field(edge, "id"),
                "source": field(edge, "sourceNodeId"),
                "target": field(edge, "targetNodeId"),
                "label": as_string(
                    first_present([field(edge, "label"), field(edge, "kind")]),
                    "",
                ),
                "edgeKind": normalize_edge_kind(&as_string(field(edge, "kind"), "")),
                "attributes": {
                    "sourceId": field(edge, "sourcePortId"),
                    "targetId": field(edge, "targetPortId"),
                    "sourcePortId": field(edge, "sourcePortId"),
                    "targetPortId": field(edge, "targetPortId"),
                    "sourceNodeId": field(edge, "sourceNodeId"),
                    "targetNodeId": field(edge, "targetNodeId"),
                    "semanticId": field(edge, "semanticId"),
                    "sourceExpression": field(edge, "sourceExpression"),
                    "targetExpression": field(edge, "targetExpression"),
                    "relationType": field(edge, "kind"),
                    "canonicalScene": true,
                },
            })
        })
        .collect();
    let title = {
        let name = as_string(field(field(scene, "view"), "name"), "");
        if name.is_empty() {
            as_string(
                field(visualization, "selectedViewName"),
                "Interconnection View",
            )
        } else {
            name
        }
    };
    json!({
        "title": title,
        "view": "interconnection-view",
        "nodes": nodes,
        "edges": edges,
        "meta": {
            "canonicalScene": true,
            "schemaVersion": field(scene, "schemaVersion"),
            "selectedRoot": selected_root,
            "rootCandidates": field(field(scene, "view"), "rootIds"),
            "diagnostics": field(scene, "diagnostics"),
        },
    })
}

fn map_port_detail(port: &Value) -> Value {
    let side_hint = as_string(field(port, "sideHint"), "");
    let port_side = match side_hint.as_str() {
        "west" => Some("left"),
        "east" => Some("right"),
        _ => None,
    };
    let multiplicity = {
        let value = as_string(field(port, "multiplicity"), "");
        if value.is_empty() {
            "[1]".to_string()
        } else {
            value
        }
    };
    json!({
        "id": field(port, "id"),
        "name": field(port, "name"),
        "direction": field(port, "direction"),
        "conjugated": field(port, "conjugated") == &json!(true),
        "semanticId": field(port, "semanticId"),
        "multiplicity": multiplicity,
        "portType": field(port, "typeName"),
        "portSide": port_side,
        "uri": field(port, "uri"),
        "range": field(port, "range"),
        "attributes": {
            "parentId": field(port, "ownerNodeId"),
            "scenePortId": field(port, "id"),
            "sideHint": field(port, "sideHint"),
        },
    })
}

fn root_coverage(root_id: &str, scene: &Value) -> usize {
    let prefix = format!("{root_id}.");
    as_array(field(scene, "nodes"))
        .iter()
        .filter(|node| {
            let id = as_string(field(node, "id"), "");
            id == root_id || id.starts_with(&prefix)
        })
        .count()
}

fn select_root(scene: &Value) -> Option<String> {
    let mut root_ids: Vec<String> = as_array(field(field(scene, "view"), "rootIds"))
        .iter()
        .map(|id| as_string(id, ""))
        .collect();
    root_ids.sort_by(|left, right| {
        let coverage = root_coverage(right, scene).cmp(&root_coverage(left, scene));
        if coverage != std::cmp::Ordering::Equal {
            return coverage;
        }
        let depth = left.split('.').count().cmp(&right.split('.').count());
        if depth != std::cmp::Ordering::Equal {
            return depth;
        }
        left.cmp(right)
    });
    root_ids.into_iter().next()
}

fn sequence_diagram_from_scene(name: &str, nodes: &[Value], scene: &Value) -> Option<Value> {
    if as_string(field(scene, "kind"), "") != "sequence" {
        return None;
    }
    let as_index = |value: &Value| -> Option<usize> {
        value.as_u64().and_then(|index| {
            let index = index as usize;
            if nodes.get(index).is_some() {
                Some(index)
            } else {
                None
            }
        })
    };
    let lifelines: Vec<Value> = as_array(field(scene, "lifelines"))
        .iter()
        .filter_map(as_index)
        .map(|index| {
            let name = {
                let label = as_string(field(&nodes[index], "label"), "");
                if label.is_empty() {
                    field(&nodes[index], "kind").clone()
                } else {
                    Value::String(label)
                }
            };
            json!({
                "id": field(&nodes[index], "id"),
                "name": name,
            })
        })
        .collect();
    let messages: Vec<Value> = as_array(field(scene, "messages"))
        .iter()
        .filter_map(|message| {
            let index = as_index(field(message, "node"));
            let source = field(message, "source");
            let target = field(message, "target");
            let order = field(message, "order");
            let source_index = if as_string(field(source, "status"), "") == "resolved" {
                as_index(field(source, "lifeline"))
            } else {
                None
            };
            let target_index = if as_string(field(target, "status"), "") == "resolved" {
                as_index(field(target, "lifeline"))
            } else {
                None
            };
            let order_value = if as_string(field(order, "status"), "") == "resolved" {
                field(order, "value").as_f64()
            } else {
                None
            };
            if index.is_none()
                || source_index.is_none()
                || target_index.is_none()
                || order_value.is_none()
            {
                return None;
            }
            Some(json!({
                "id": field(&nodes[index.unwrap()], "id"),
                "name": if field(message, "label").is_string() {
                    field(message, "label").clone()
                } else {
                    field(&nodes[index.unwrap()], "label").clone()
                },
                "source": field(&nodes[source_index.unwrap()], "id"),
                "target": field(&nodes[target_index.unwrap()], "id"),
                "kind": field(&nodes[index.unwrap()], "kind"),
                "order": order_value.unwrap(),
            }))
        })
        .collect();
    Some(json!({
        "name": name,
        "lifelines": lifelines,
        "messages": messages,
        "activations": [],
        "fragments": [],
    }))
}

fn is_part_metaclass(metaclass: &str) -> bool {
    let normalized = metaclass.to_ascii_lowercase();
    normalized == "partdefinition" || normalized == "partusage"
}

fn is_port_metaclass(metaclass: &str) -> bool {
    let normalized = metaclass.to_ascii_lowercase();
    normalized == "portusage"
        || normalized == "portdefinition"
        || normalized == "conjugatedportdefinition"
}

fn is_interconnection_edge_kind(kind: &str) -> bool {
    matches!(
        normalize_edge_kind(kind).as_str(),
        "connection" | "flow" | "bind" | "interface"
    )
}

type NavigationFn<'a> = Box<dyn Fn(&Value) -> Value + 'a>;

fn prepare_interconnection_from_typed_projection(
    name: &str,
    nodes: &[Value],
    edges: &[Value],
    exposed_roots: &Value,
    metadata: &Value,
    references: &[Value],
    navigation: &NavigationFn<'_>,
) -> Value {
    let scene = interconnection_scene_from_typed_projection(
        name,
        nodes,
        edges,
        exposed_roots,
        metadata,
        references,
        navigation,
    );
    let mut prepared = prepare_interconnection_scene(&scene, &json!({ "selectedViewName": name }));
    prepared["meta"]["typedProjection"] = json!(true);
    let port_type_keys: HashMap<String, String> = nodes
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            if !is_port_metaclass(&as_string(field(node, "metaclass"), "")) {
                return None;
            }
            let typing = field(node, "typing");
            if field(typing, "status") != "resolved" {
                return None;
            }
            let types = as_array(field(typing, "types"));
            if types.len() != 1 {
                return None;
            }
            let reference = field(&types[0], "reference")
                .as_u64()
                .and_then(|index| references.get(index as usize))?;
            let qualified_name = field(reference, "qualifiedName").as_str()?;
            Some((format!("n:{index}"), qualified_name.to_string()))
        })
        .collect();
    if let Some(edges) = prepared.get_mut("edges").and_then(Value::as_array_mut) {
        for edge in edges {
            edge["attributes"]["typedProjection"] = json!(true);
            let attributes = &edge["attributes"];
            let source_type = field(attributes, "sourcePortId")
                .as_str()
                .and_then(|id| port_type_keys.get(id));
            let target_type = field(attributes, "targetPortId")
                .as_str()
                .and_then(|id| port_type_keys.get(id));
            if let (Some(source_type), Some(target_type)) = (source_type, target_type) {
                if source_type == target_type {
                    edge["attributes"]["portTypeIdentity"] = json!(source_type);
                }
            }
        }
    }
    prepared
}

fn interconnection_scene_from_typed_projection(
    name: &str,
    raw_nodes: &[Value],
    edges: &[Value],
    exposed_roots: &Value,
    metadata: &Value,
    references: &[Value],
    navigation: &NavigationFn<'_>,
) -> Value {
    let index_set = |value: &Value| -> HashSet<usize> {
        as_array(value)
            .iter()
            .filter_map(|index| {
                index.as_u64().and_then(|i| {
                    let i = i as usize;
                    if raw_nodes.get(i).is_some() {
                        Some(i)
                    } else {
                        None
                    }
                })
            })
            .collect()
    };
    let mut part_indexes = index_set(field(metadata, "parts"));
    let mut port_indexes = index_set(field(metadata, "ports"));
    if part_indexes.is_empty() {
        for (index, element) in raw_nodes.iter().enumerate() {
            if is_part_metaclass(&as_string(field(element, "metaclass"), "")) {
                part_indexes.insert(index);
            }
        }
    }
    if port_indexes.is_empty() {
        for (index, element) in raw_nodes.iter().enumerate() {
            if is_port_metaclass(&as_string(field(element, "metaclass"), "")) {
                port_indexes.insert(index);
            }
        }
    }
    let id_for = |index: usize| format!("n:{index}");
    let qualified_name = |index: usize| -> String {
        let element = &raw_nodes[index];
        if let Some(reference) = field(element, "reference")
            .as_u64()
            .and_then(|i| references.get(i as usize))
        {
            let qualified = as_string(field(reference, "qualifiedName"), "");
            if !qualified.is_empty() {
                return qualified;
            }
        }
        as_string(field(element, "name"), &id_for(index))
    };
    let type_name = |element: &Value| -> Option<String> {
        let typing = field(element, "typing");
        let labels: Vec<String> = as_array(field(typing, "types"))
            .iter()
            .filter_map(|item| {
                item.get("label")
                    .and_then(Value::as_str)
                    .map(str::to_string)
            })
            .collect();
        if labels.is_empty() {
            None
        } else {
            Some(labels.join(" & "))
        }
    };
    let location = |element: &Value| -> (Value, Value) {
        let source = navigation(field(element, "source"));
        let range = field(&source, "range");
        let start = field(range, "start");
        let end = field(range, "end");
        let placed_range = if field(start, "line").as_f64().is_some()
            && field(start, "character").as_f64().is_some()
            && field(end, "line").as_f64().is_some()
            && field(end, "character").as_f64().is_some()
        {
            json!({
                "start": { "line": field(start, "line"), "character": field(start, "character") },
                "end": { "line": field(end, "line"), "character": field(end, "character") },
            })
        } else {
            Value::Null
        };
        (field(&source, "uri").clone(), placed_range)
    };
    let scene_kind = |element: &Value| -> &'static str {
        match as_string(field(element, "notationRole"), "").as_str() {
            "reference-usage" => "ref",
            "definition" => "def",
            _ => "part",
        }
    };
    let mut part_list: Vec<usize> = part_indexes.iter().copied().collect();
    part_list.sort_unstable();
    let nodes: Vec<Value> = part_list
        .iter()
        .map(|&index| {
            let element = &raw_nodes[index];
            let owner = field(element, "owner").as_u64().map(|i| i as usize);
            let parent_id = owner.and_then(|owner| {
                if part_indexes.contains(&owner) {
                    Some(id_for(owner))
                } else {
                    None
                }
            });
            let (uri, range) = location(element);
            json!({
                "id": id_for(index),
                "semanticId": qualified_name(index),
                "qualifiedName": qualified_name(index),
                "name": as_string(field(element, "name"), &id_for(index)),
                "kind": scene_kind(element),
                "typeName": type_name(element),
                "parentId": parent_id,
                "uri": uri,
                "range": range,
            })
        })
        .collect();
    let mut port_list: Vec<usize> = port_indexes.iter().copied().collect();
    port_list.sort_unstable();
    let ports: Vec<Value> = port_list
        .iter()
        .filter_map(|&index| {
            let element = &raw_nodes[index];
            let owner = field(element, "owner").as_u64().map(|i| i as usize)?;
            if !part_indexes.contains(&owner) {
                return None;
            }
            let (uri, range) = location(element);
            Some(json!({
                "id": id_for(index),
                "semanticId": qualified_name(index),
                "ownerNodeId": id_for(owner),
                "name": as_string(field(element, "name"), &id_for(index)),
                "typeName": type_name(element),
                "direction": field(element, "direction").as_str().map(str::to_string),
                "conjugated": field(element, "conjugated") == &json!(true),
                "sideHint": "",
                "uri": uri,
                "range": range,
            }))
        })
        .collect();
    let port_owner: HashMap<String, String> = ports
        .iter()
        .map(|port| {
            (
                as_string(field(port, "id"), ""),
                as_string(field(port, "ownerNodeId"), ""),
            )
        })
        .collect();
    let part_ids: HashSet<String> = nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let endpoint = |index: &Value| -> Option<(String, String)> {
        let index = index.as_u64()? as usize;
        let id = id_for(index);
        if let Some(owner) = port_owner.get(&id) {
            return Some((owner.clone(), id));
        }
        if part_ids.contains(&id) {
            return Some((id, String::new()));
        }
        None
    };
    let scene_edges: Vec<Value> = edges
        .iter()
        .enumerate()
        .filter_map(|(index, edge)| {
            let kind = as_string(field(edge, "kind"), "");
            if !is_interconnection_edge_kind(&kind) {
                return None;
            }
            let source = endpoint(field(edge, "source"))?;
            let target = endpoint(field(edge, "target"))?;
            Some(json!({
                "id": format!("e:{index}"),
                "kind": kind,
                "sourcePortId": source.1,
                "targetPortId": target.1,
                "sourceNodeId": source.0,
                "targetNodeId": target.0,
            }))
        })
        .collect();
    let root_ids: Vec<String> = as_array(exposed_roots)
        .iter()
        .filter_map(|index| {
            let index = index.as_u64()? as usize;
            if part_indexes.contains(&index) {
                Some(id_for(index))
            } else {
                None
            }
        })
        .collect();
    json!({
        "schemaVersion": 2,
        "view": {
            "id": name,
            "name": name,
            "type": "InterconnectionView",
            "rootIds": root_ids,
        },
        "nodes": nodes,
        "ports": ports,
        "edges": scene_edges,
        "containers": [],
        "diagnostics": [],
    })
}

fn required_scene_vertex_index(
    transition: &Value,
    field_name: &str,
    vertex_count: usize,
) -> Result<usize, String> {
    let value = field(transition, field_name);
    let Some(index) = value.as_u64().map(|index| index as usize) else {
        return Err(format!(
            "state-transition {field_name} must be a vertex index, got {value}"
        ));
    };
    if index >= vertex_count {
        return Err(format!(
            "state-transition {field_name} index {index} is out of range ({vertex_count} vertices)"
        ));
    }
    Ok(index)
}

fn graph_for_standard_view(visualization: &Value) -> &Value {
    let general = field(visualization, "generalViewGraph");
    if general.is_object() {
        general
    } else {
        field(visualization, "graph")
    }
}

fn projection_hints(visualization: &Value) -> &Value {
    field(visualization, "projectionHints")
}

fn nonempty_string(value: &Value) -> Option<String> {
    let text = as_string(value, "");
    if text.trim().is_empty() {
        None
    } else {
        Some(text)
    }
}

fn first_nonempty_string<'a>(values: impl IntoIterator<Item = &'a Value>) -> Option<String> {
    values.into_iter().find_map(nonempty_string)
}

fn qualified_name_of(node: &Value) -> String {
    as_string(
        first_present([
            field(node, "id"),
            field(node, "qualifiedName"),
            field(field(node, "attributes"), "qualifiedName"),
            field(node, "name"),
        ]),
        "",
    )
}

fn optional_uri_value(node: &Value) -> Value {
    match js_nonempty_string(first_present([
        field(node, "uri"),
        field(node, "sourcePath"),
        field(node, "source_path"),
    ])) {
        Some(uri) => Value::String(uri),
        None => Value::Null,
    }
}

struct BrowserGraphNode {
    id: String,
    label: String,
    kind: String,
    parent_id: String,
    qualified_name: String,
    visibility: Option<String>,
    uri: Value,
    range: Value,
}

fn standard_graph_nodes(visualization: &Value) -> Vec<BrowserGraphNode> {
    as_array(field(graph_for_standard_view(visualization), "nodes"))
        .iter()
        .map(|node| BrowserGraphNode {
            id: as_string(field(node, "id"), ""),
            label: as_string(
                first_present([
                    field(node, "name"),
                    field(node, "qualifiedName"),
                    field(node, "id"),
                ]),
                "Unnamed",
            ),
            kind: {
                let kind = element_type_of(node);
                if kind.is_empty() {
                    "element".to_string()
                } else {
                    kind
                }
            },
            parent_id: first_nonempty_string([
                field(node, "parent_id"),
                field(node, "parentId"),
                field(field(node, "attributes"), "parentId"),
            ])
            .unwrap_or_default(),
            qualified_name: qualified_name_of(node),
            visibility: nonempty_string(field(field(node, "attributes"), "visibility")),
            uri: optional_uri_value(node),
            range: node_range(node),
        })
        .collect()
}

fn browser_row_value(node: &BrowserGraphNode, depth: usize, has_children: bool) -> Value {
    let mut row = json!({
        "id": node.id,
        "label": node.label,
        "kind": node.kind,
        "parentId": node.parent_id,
        "qualifiedName": node.qualified_name,
        "depth": depth,
        "hasChildren": has_children,
        "uri": node.uri,
        "range": node.range,
    });
    if let Some(visibility) = &node.visibility {
        row["visibility"] = json!(visibility);
    }
    row
}

fn build_hierarchy_rows(graph_nodes: &[BrowserGraphNode], tree_roots: &[String]) -> Vec<Value> {
    let by_id: HashMap<&str, &BrowserGraphNode> = graph_nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let mut children_by_parent: HashMap<&str, Vec<&BrowserGraphNode>> = HashMap::new();
    for node in graph_nodes {
        if node.parent_id.is_empty() || !by_id.contains_key(node.parent_id.as_str()) {
            continue;
        }
        children_by_parent
            .entry(node.parent_id.as_str())
            .or_default()
            .push(node);
    }
    let roots: Vec<&BrowserGraphNode> = if tree_roots.is_empty() {
        graph_nodes
            .iter()
            .filter(|node| {
                node.parent_id.is_empty() || !by_id.contains_key(node.parent_id.as_str())
            })
            .collect()
    } else {
        tree_roots
            .iter()
            .filter_map(|id| by_id.get(id.as_str()).copied())
            .collect()
    };
    let mut rows = Vec::new();
    fn visit(
        node: &BrowserGraphNode,
        depth: usize,
        children_by_parent: &HashMap<&str, Vec<&BrowserGraphNode>>,
        rows: &mut Vec<Value>,
    ) {
        let children = children_by_parent
            .get(node.id.as_str())
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        rows.push(browser_row_value(node, depth, !children.is_empty()));
        for child in children {
            visit(child, depth + 1, children_by_parent, rows);
        }
    }
    for root in roots {
        visit(root, 0, &children_by_parent, &mut rows);
    }
    rows
}

fn prepare_browser(visualization: &Value) -> Value {
    let graph_nodes = standard_graph_nodes(visualization);
    let hints = projection_hints(visualization);
    let hierarchy_layout = as_string(field(hints, "browserLayout"), "") == "hierarchy";
    let tree_roots: Vec<String> = as_array(field(hints, "treeRoots"))
        .iter()
        .map(|value| as_string(value, ""))
        .filter(|value| !value.is_empty())
        .collect();
    let mut rows = if hierarchy_layout {
        build_hierarchy_rows(&graph_nodes, &tree_roots)
    } else {
        let mut rows: Vec<Value> = graph_nodes
            .iter()
            .map(|node| browser_row_value(node, 0, false))
            .collect();
        rows.sort_by(|left, right| {
            as_string(field(left, "qualifiedName"), "")
                .cmp(&as_string(field(right, "qualifiedName"), ""))
        });
        rows
    };
    let nodes: Vec<Value> = rows
        .iter_mut()
        .enumerate()
        .map(|(index, row)| {
            let id = as_string(field(row, "id"), "");
            if id.is_empty() {
                row["id"] = json!(format!("browser-row-{index}"));
            }
            json!({
                "id": field(row, "id"),
                "label": field(row, "label"),
                "kind": field(row, "kind"),
                "uri": field(row, "uri"),
                "range": field(row, "range"),
                "attributes": row,
            })
        })
        .collect();
    json!({
        "title": as_string(field(visualization, "selectedViewName"), "Browser View"),
        "view": "browser-view",
        "nodes": nodes,
        "edges": [],
        "meta": {
            "rows": rows,
            "hierarchyLayout": hierarchy_layout,
            "provisional": !hierarchy_layout,
        },
    })
}

fn build_relationship_matrix(node_ids: &[String], graph_edges: &[Value]) -> Vec<Value> {
    let mut edge_by_pair: HashMap<String, Vec<String>> = HashMap::new();
    for edge in graph_edges {
        let source = as_string(field(edge, "source"), "");
        let target = as_string(field(edge, "target"), "");
        if source.is_empty() || target.is_empty() {
            continue;
        }
        let label = as_string(
            first_present([
                field(edge, "name"),
                field(edge, "label"),
                field(edge, "type"),
                field(edge, "rel_type"),
            ]),
            "",
        );
        if label.is_empty() {
            continue;
        }
        edge_by_pair
            .entry(format!("{source}::{target}"))
            .or_default()
            .push(label);
    }
    let mut cells = Vec::new();
    for source in node_ids {
        for target in node_ids {
            let labels = edge_by_pair
                .get(&format!("{source}::{target}"))
                .cloned()
                .unwrap_or_default();
            cells.push(json!({
                "source": source,
                "target": target,
                "present": !labels.is_empty(),
                "labels": labels,
            }));
        }
    }
    cells
}

fn prepare_grid(visualization: &Value) -> Value {
    let graph = graph_for_standard_view(visualization);
    let hints = projection_hints(visualization);
    let relationship_matrix = as_string(field(hints, "gridSubtype"), "") == "relationship_matrix";
    let mut cells: Vec<Value> = as_array(field(graph, "nodes"))
        .iter()
        .map(|node| {
            let attrs = field(node, "attributes");
            let qualified_name = qualified_name_of(node);
            let kind = element_type_of(node);
            json!({
                "id": as_string(field(node, "id"), ""),
                "name": as_string(
                    first_present([
                        field(node, "name"),
                        field(node, "qualifiedName"),
                        field(node, "id"),
                    ]),
                    "Unnamed",
                ),
                "kind": if kind.is_empty() { "element" } else { kind.as_str() },
                "qualifiedName": qualified_name,
                "attributeCount": as_array(field(attrs, "attributes")).len(),
                "partCount": as_array(field(attrs, "parts")).len(),
                "portCount": as_array(field(attrs, "ports")).len(),
                "unsupportedRendering": "Unsupported rendering",
                "uri": optional_uri_value(node),
                "range": node_range(node),
            })
        })
        .collect();
    cells.sort_by(|left, right| {
        as_string(field(left, "qualifiedName"), "")
            .cmp(&as_string(field(right, "qualifiedName"), ""))
    });
    let node_ids: Vec<String> = cells
        .iter()
        .map(|cell| as_string(field(cell, "id"), ""))
        .filter(|id| !id.is_empty())
        .collect();
    let matrix_cells = if relationship_matrix {
        build_relationship_matrix(&node_ids, as_array(field(graph, "edges")))
    } else {
        Vec::new()
    };
    let column_views: Vec<Value> = as_array(field(hints, "columnViews"))
        .iter()
        .map(|entry| {
            let rendering_type = as_string(field(entry, "renderingType"), "");
            json!({
                "label": as_string(field(entry, "label"), "Column"),
                "renderingType": rendering_type,
                "supported": rendering_type == "asTextualNotation",
            })
        })
        .collect();
    let columns = if column_views.is_empty() {
        Value::Null
    } else {
        Value::Array(
            column_views
                .iter()
                .map(|column| {
                    let supported = field(column, "supported") == &json!(true);
                    json!({
                        "key": if supported { "name" } else { "unsupportedRendering" },
                        "label": field(column, "label"),
                        "renderingType": field(column, "renderingType"),
                        "notationStatus": if supported { "normative" } else { "unsupported" },
                    })
                })
                .collect(),
        )
    };
    let nodes: Vec<Value> = cells
        .iter()
        .enumerate()
        .map(|(index, cell)| {
            let id = as_string(field(cell, "id"), "");
            json!({
                "id": if id.is_empty() {
                    Value::String(format!("grid-row-{index}"))
                } else {
                    field(cell, "id").clone()
                },
                "label": field(cell, "name"),
                "kind": field(cell, "kind"),
                "uri": field(cell, "uri"),
                "range": field(cell, "range"),
                "attributes": cell,
            })
        })
        .collect();
    let mut meta = json!({
        "cells": cells,
        "relationshipMatrix": relationship_matrix,
        "matrixRowIds": if relationship_matrix { json!(node_ids) } else { json!([]) },
        "matrixColIds": if relationship_matrix { json!(node_ids) } else { json!([]) },
        "matrixCells": matrix_cells,
        "provisional": column_views.iter().any(|column| field(column, "supported") != &json!(true)),
    });
    if !columns.is_null() {
        meta["columns"] = columns;
    }
    json!({
        "title": as_string(field(visualization, "selectedViewName"), "Grid View"),
        "view": "grid-view",
        "nodes": nodes,
        "edges": [],
        "meta": meta,
    })
}

fn prepare_geometry(visualization: &Value) -> Value {
    let graph = graph_for_standard_view(visualization);
    let hints = projection_hints(visualization);
    let elements: Vec<Value> = as_array(field(graph, "nodes"))
        .iter()
        .map(|node| {
            let kind = element_type_of(node);
            json!({
                "id": as_string(field(node, "id"), ""),
                "label": as_string(
                    first_present([
                        field(node, "name"),
                        field(node, "qualifiedName"),
                        field(node, "id"),
                    ]),
                    "Unnamed",
                ),
                "kind": if kind.is_empty() { "element" } else { kind.as_str() },
                "qualifiedName": qualified_name_of(node),
                "uri": optional_uri_value(node),
                "range": node_range(node),
            })
        })
        .collect();
    let node_ids: HashSet<String> = elements
        .iter()
        .map(|element| as_string(field(element, "id"), ""))
        .collect();
    let nodes: Vec<Value> = elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            let id = as_string(field(element, "id"), "");
            json!({
                "id": if id.is_empty() {
                    Value::String(format!("geometry-node-{index}"))
                } else {
                    field(element, "id").clone()
                },
                "label": field(element, "label"),
                "kind": field(element, "kind"),
                "uri": field(element, "uri"),
                "range": field(element, "range"),
                "attributes": element,
            })
        })
        .collect();
    let edges: Vec<Value> = as_array(field(graph, "edges"))
        .iter()
        .enumerate()
        .filter_map(|(index, edge)| {
            let source = as_string(field(edge, "source"), "");
            let target = as_string(field(edge, "target"), "");
            if !node_ids.contains(&source) || !node_ids.contains(&target) {
                return None;
            }
            Some(json!({
                "id": as_string(field(edge, "id"), &format!("geometry-edge-{index}")),
                "source": source,
                "target": target,
                "label": as_string(
                    first_present([
                        field(edge, "name"),
                        field(edge, "label"),
                        field(edge, "type"),
                        field(edge, "rel_type"),
                    ]),
                    "",
                ),
            }))
        })
        .collect();
    json!({
        "title": as_string(field(visualization, "selectedViewName"), "Geometry View"),
        "view": "geometry-view",
        "nodes": nodes,
        "edges": edges,
        "meta": {
            "elements": elements,
            "geometryMode": as_string(field(hints, "geometryMode"), "2d"),
            "geometryProjection": as_string(field(hints, "geometryProjection"), "orthographic"),
            "provisional": true,
        },
    })
}

fn typed_owner_node_id(node: &Value) -> String {
    match field(field(node, "attributes"), "owner") {
        Value::Number(number) => number
            .as_u64()
            .map(|index| format!("n:{index}"))
            .or_else(|| number.as_i64().map(|index| format!("n:{index}")))
            .unwrap_or_default(),
        Value::String(value) => value.clone(),
        _ => String::new(),
    }
}

fn enrich_typed_catalog_product(selected_kind: &str, product: &mut Value) {
    match selected_kind {
        "browser-view" => {
            let nodes = as_array(field(product, "nodes"));
            let metadata = field(field(product, "meta"), "viewMetadata");
            let tree_roots: Vec<String> = as_array(field(metadata, "roots"))
                .iter()
                .map(|index| format!("n:{}", as_string(index, "")))
                .filter(|id| id != "n:")
                .collect();
            let graph_nodes: Vec<BrowserGraphNode> = nodes
                .iter()
                .map(|node| BrowserGraphNode {
                    id: as_string(field(node, "id"), ""),
                    label: as_string(field(node, "label"), "Unnamed"),
                    kind: as_string(field(node, "kind"), "element"),
                    parent_id: typed_owner_node_id(node),
                    qualified_name: as_string(field(node, "id"), ""),
                    visibility: nonempty_string(field(field(node, "attributes"), "visibility")),
                    uri: field(node, "uri").clone(),
                    range: field(node, "range").clone(),
                })
                .collect();
            let by_id: HashSet<String> = graph_nodes.iter().map(|node| node.id.clone()).collect();
            let hierarchy_layout = graph_nodes
                .iter()
                .any(|node| !node.parent_id.is_empty() && by_id.contains(&node.parent_id));
            let rows = if hierarchy_layout {
                build_hierarchy_rows(&graph_nodes, &tree_roots)
            } else {
                graph_nodes
                    .iter()
                    .map(|node| browser_row_value(node, 0, false))
                    .collect()
            };
            let mut meta = as_object(field(product, "meta"));
            meta.insert("rows".into(), Value::Array(rows));
            meta.insert("hierarchyLayout".into(), json!(hierarchy_layout));
            meta.insert("provisional".into(), json!(!hierarchy_layout));
            product["meta"] = Value::Object(meta);
        }
        "grid-view" => {
            let nodes = as_array(field(product, "nodes")).to_vec();
            let metadata = field(field(product, "meta"), "viewMetadata");
            let grid_rows: Vec<usize> = as_array(field(metadata, "rows"))
                .iter()
                .filter_map(|value| value.as_u64().map(|index| index as usize))
                .filter(|index| *index < nodes.len())
                .collect();
            let grid_columns: Vec<String> = as_array(field(metadata, "columns"))
                .iter()
                .map(|value| as_string(value, ""))
                .filter(|value| !value.is_empty())
                .collect();
            let grid_relationships = as_array(field(metadata, "cells"));
            let cells: Vec<Value> = grid_rows
                .iter()
                .map(|node_index| {
                    let node = &nodes[*node_index];
                    let mut cell = json!({
                        "id": field(node, "id"),
                        "name": field(node, "label"),
                        "kind": field(node, "kind"),
                    });
                    for column in &grid_columns {
                        let present = grid_relationships.iter().any(|entry| {
                            entry.get("row").and_then(Value::as_u64) == Some(*node_index as u64)
                                && as_string(field(entry, "column"), "") == *column
                        });
                        cell[format!("relationship:{column}")] =
                            json!(if present { "✓" } else { "" });
                    }
                    cell
                })
                .collect();
            let mut columns = vec![json!({
                "key": "name",
                "label": "Element",
                "notationStatus": "normative",
            })];
            columns.extend(grid_columns.iter().map(|column| {
                json!({
                    "key": format!("relationship:{column}"),
                    "label": column,
                    "notationStatus": "normative",
                })
            }));
            let mut meta = as_object(field(product, "meta"));
            meta.insert("cells".into(), Value::Array(cells));
            meta.insert("columns".into(), Value::Array(columns));
            meta.insert("provisional".into(), json!(false));
            product["meta"] = Value::Object(meta);
        }
        "geometry-view" => {
            let elements: Vec<Value> = as_array(field(product, "nodes"))
                .iter()
                .map(|node| {
                    json!({
                        "id": field(node, "id"),
                        "label": field(node, "label"),
                        "kind": field(node, "kind"),
                    })
                })
                .collect();
            let mut meta = as_object(field(product, "meta"));
            meta.insert("elements".into(), Value::Array(elements));
            meta.insert("geometryMode".into(), json!("2d"));
            meta.insert("geometryProjection".into(), json!("orthographic"));
            meta.insert("provisional".into(), json!(true));
            product["meta"] = Value::Object(meta);
        }
        _ => {}
    }
}

/// The typed projection includes declarations and parameter references for inspection. Only
/// projected action/control members are activity vertices; containment is not an action flow.
fn prepare_typed_action_flow(
    name: &str,
    raw_nodes: &[Value],
    raw_edges: &[Value],
    metadata: &Value,
    references: &[Value],
    navigation: &NavigationFn<'_>,
) -> Result<Value, String> {
    if !field(metadata, "actions").is_array() || !field(metadata, "controlNodes").is_array() {
        return Err("action-flow metadata must include actions and controlNodes arrays".into());
    }
    let member_indexes: Vec<usize> = as_array(field(metadata, "actions"))
        .iter()
        .chain(as_array(field(metadata, "controlNodes")))
        .map(|value| {
            let index = value
                .as_u64()
                .ok_or_else(|| "action-flow member index must be an integer".to_string())?
                as usize;
            if index >= raw_nodes.len() {
                return Err(format!("action-flow member index {index} is out of range"));
            }
            Ok(index)
        })
        .collect::<Result<_, String>>()?;
    let action_indexes: HashSet<usize> = member_indexes.into_iter().collect();
    let parent_actions: HashSet<usize> = action_indexes
        .iter()
        .filter_map(|index| field(&raw_nodes[*index], "owner").as_u64())
        .map(|index| index as usize)
        .filter(|index| action_indexes.contains(index))
        .collect();
    let visible_indexes: HashSet<usize> = action_indexes
        .difference(&parent_actions)
        .copied()
        .collect();
    let mut ordered_indexes: Vec<usize> = visible_indexes.iter().copied().collect();
    ordered_indexes.sort_unstable();
    let nodes: Vec<Value> = ordered_indexes
        .iter()
        .map(|&index| {
            let element = &raw_nodes[index];
            let source = navigation(field(element, "source"));
            let semantic_reference = field(element, "reference")
                .as_u64()
                .and_then(|reference| references.get(reference as usize));
            json!({
                "id": format!("n:{index}"),
                "label": as_string(field(element, "name"), &as_string(field(element, "metaclass"), "Action")),
                "kind": as_string(field(element, "metaclass"), "ActionUsage"),
                "uri": field(&source, "uri"),
                "range": field(&source, "range"),
                "attributes": {
                    "semanticReference": semantic_reference,
                    "notationRole": field(element, "notationRole"),
                    "owner": field(element, "owner"),
                },
            })
        })
        .collect();
    let edges: Vec<Value> = raw_edges
        .iter()
        .enumerate()
        .filter_map(|(index, edge)| {
            let kind = field(edge, "kind").as_str()?;
            if kind != "succession" && kind != "flow" {
                return None;
            }
            let source = field(edge, "source").as_u64()? as usize;
            let target = field(edge, "target").as_u64()? as usize;
            if !visible_indexes.contains(&source) || !visible_indexes.contains(&target) {
                return None;
            }
            Some(json!({
                "id": format!("e:{index}"),
                "source": format!("n:{source}"),
                "target": format!("n:{target}"),
                "label": "",
                "attributes": {
                    "succession": kind == "succession",
                    "streamingFlow": kind == "flow",
                    "typedProjection": true,
                    "provenance": field(edge, "provenance"),
                    "sourceNavigation": navigation(field(edge, "navigation")),
                    "semanticReference": field(edge, "reference")
                        .as_u64()
                        .and_then(|reference| references.get(reference as usize)),
                },
            }))
        })
        .collect();
    Ok(json!({
        "title": name,
        "view": "action-flow-view",
        "nodes": nodes,
        "edges": edges,
        "meta": { "sceneKind": "action-flow", "layoutDirection": "vertical", "typedProjection": true },
    }))
}

fn prepare_typed_diagram_product(input: &Value) -> Option<Result<Value, String>> {
    if field(input, "schemaVersion").as_u64() != Some(5) {
        return None;
    }
    let selected = field(input, "selectedView");
    let projection = field(input, "projection");
    let documents: Vec<Value> = as_array(field(input, "documents")).to_vec();
    let sources: Vec<Value> = as_array(field(input, "sources")).to_vec();
    let references: Vec<Value> = as_array(field(input, "references")).to_vec();
    let selected_kind = selected.get("kind")?.as_str()?;
    if field(projection, "kind").as_str() != Some(selected_kind) {
        return None;
    }
    let selected_name = selected.get("name")?.as_str()?;
    let projection_nodes = projection.get("nodes")?.as_array()?;
    let projection_edges = projection.get("edges")?.as_array()?;
    let navigation: NavigationFn<'_> = Box::new(move |index: &Value| {
        let source = index
            .as_u64()
            .and_then(|i| sources.get(i as usize))
            .cloned()
            .unwrap_or(Value::Null);
        let document = field(&source, "document")
            .as_u64()
            .and_then(|i| documents.get(i as usize))
            .cloned()
            .unwrap_or(Value::Null);
        let range = as_array(field(&source, "range"));
        json!({
            "uri": field(&document, "uri").as_str().map(str::to_string),
            "range": if range.len() == 4 {
                json!({
                    "start": { "line": range[0], "character": range[1] },
                    "end": { "line": range[2], "character": range[3] },
                })
            } else {
                json!({})
            },
        })
    });
    if selected_kind == "action-flow-view" {
        return Some(prepare_typed_action_flow(
            selected_name,
            projection_nodes,
            projection_edges,
            field(projection, "metadata"),
            &references,
            &navigation,
        ));
    }
    if selected_kind == "state-transition-view" {
        let scene = field(projection, "scene");
        if as_string(field(scene, "kind"), "") != "state-transition"
            || !field(scene, "vertices").is_array()
            || !field(scene, "transitions").is_array()
        {
            return None;
        }
        let frame = field(scene, "frame");
        let nodes: Vec<Value> = as_array(field(scene, "vertices"))
            .iter()
            .enumerate()
            .map(|(index, vertex)| {
                let source = navigation(field(vertex, "navigation"));
                let semantic_id = {
                    let id = as_string(field(vertex, "id"), "");
                    if id.is_empty() {
                        index.to_string()
                    } else {
                        id
                    }
                };
                json!({
                    "id": format!("state:{semantic_id}"),
                    "label": as_string(field(vertex, "label"), ""),
                    "kind": as_string(field(vertex, "kind"), "state"),
                    "uri": field(&source, "uri"),
                    "range": field(&source, "range"),
                    "attributes": { "semanticSceneId": field(vertex, "id") },
                })
            })
            .collect();
        let feature_label = |value: &Value| -> String {
            if as_string(field(value, "status"), "") == "supported" {
                as_string(field(value, "label"), "")
            } else {
                String::new()
            }
        };
        let trigger_label = |value: &Value| -> String {
            if as_string(field(value, "status"), "") == "accept" {
                as_string(field(value, "label"), "")
            } else {
                String::new()
            }
        };
        let mut edges = Vec::new();
        for (index, transition) in as_array(field(scene, "transitions")).iter().enumerate() {
            let source_index = match required_scene_vertex_index(transition, "source", nodes.len())
            {
                Ok(index) => index,
                Err(err) => return Some(Err(err)),
            };
            let target_index = match required_scene_vertex_index(transition, "target", nodes.len())
            {
                Ok(index) => index,
                Err(err) => return Some(Err(err)),
            };
            let source = &nodes[source_index];
            let target = &nodes[target_index];
            let trigger = trigger_label(field(transition, "trigger"));
            let guard = feature_label(field(transition, "guard"));
            let effect = feature_label(field(transition, "effect"));
            let mut parts = Vec::new();
            if !trigger.is_empty() {
                parts.push(trigger.clone());
            }
            if !guard.is_empty() {
                parts.push(format!("[{guard}]"));
            }
            if !effect.is_empty() {
                parts.push(effect.clone());
            }
            let label = if parts.is_empty() {
                as_string(field(transition, "label"), "")
            } else {
                parts.join(" / ")
            };
            edges.push(json!({
                "id": format!("transition:{index}"),
                "source": field(source, "id"),
                "target": field(target, "id"),
                "label": label,
                "edgeKind": "transition",
                "attributes": {
                    "semanticSceneId": field(transition, "id"),
                    "relationType": "transition",
                    "selfLoop": field(transition, "source") == field(transition, "target"),
                    "trigger": trigger,
                    "guard": guard,
                    "effect": effect,
                    "provenance": field(transition, "provenance"),
                    "sourceNavigation": navigation(field(transition, "navigation")),
                },
            }));
        }
        let title = {
            let label = as_string(field(frame, "label"), "");
            if label.is_empty() {
                selected_name.to_string()
            } else {
                label
            }
        };
        return Some(Ok(json!({
            "title": title,
            "view": selected_kind,
            "nodes": nodes,
            "edges": edges,
            "meta": {
                "sceneKind": field(scene, "kind"),
                "frame": frame,
                "layoutDirection": "horizontal",
            },
        })));
    }
    if selected_kind == "interconnection-view" {
        return Some(Ok(prepare_interconnection_from_typed_projection(
            selected_name,
            projection_nodes,
            projection_edges,
            field(projection, "exposedRoots"),
            field(projection, "metadata"),
            &references,
            &navigation,
        )));
    }
    let mut nodes: Vec<Value> = projection_nodes
        .iter()
        .enumerate()
        .map(|(index, element)| {
            let source = navigation(field(element, "source"));
            let typing = field(element, "typing");
            let type_labels: Vec<String> = as_array(field(typing, "types"))
                .iter()
                .filter_map(|item| {
                    item.get("label")
                        .and_then(Value::as_str)
                        .map(str::to_string)
                })
                .collect();
            let typing_status = as_string(field(typing, "status"), "");
            let typed_by_name = if (typing_status == "resolved" || typing_status == "partial")
                && !type_labels.is_empty()
            {
                Some(type_labels.join(" & "))
            } else {
                None
            };
            let semantic_reference = field(element, "reference")
                .as_u64()
                .and_then(|i| references.get(i as usize))
                .cloned();
            json!({
                "id": format!("n:{index}"),
                "label": if field(element, "name").is_string() {
                    field(element, "name").clone()
                } else {
                    Value::String(as_string(field(element, "metaclass"), ""))
                },
                "kind": as_string(field(element, "metaclass"), "Unrecognized"),
                "uri": field(&source, "uri"),
                "range": field(&source, "range"),
                "attributes": {
                    "notationRole": field(element, "notationRole"),
                    "semanticReference": semantic_reference,
                    "owner": field(element, "owner"),
                    "typingStatus": field(typing, "status"),
                    "typedByName": typed_by_name,
                },
            })
        })
        .collect();
    for (owner_index, raw) in projection_nodes.iter().enumerate() {
        let compartments: Vec<Value> = as_array(field(raw, "compartments"))
            .iter()
            .map(|compartment| {
                let members: Vec<Value> = as_array(field(compartment, "members"))
                    .iter()
                    .filter_map(|member| {
                        let index = member.as_u64()? as usize;
                        let node = nodes.get(index)?;
                        Some(json!({
                            "id": field(node, "id"),
                            "name": field(node, "label"),
                            "kind": field(node, "kind"),
                            "typeName": field(field(node, "attributes"), "typedByName")
                                .as_str()
                                .map(str::to_string),
                        }))
                    })
                    .collect();
                json!({
                    "kind": as_string(field(compartment, "kind"), "members"),
                    "provenance": as_string(field(compartment, "provenance"), "direct"),
                    "members": members,
                })
            })
            .collect();
        let mut attributes = as_object(field(&nodes[owner_index], "attributes"));
        attributes.insert("typedCompartments".into(), Value::Array(compartments));
        nodes[owner_index]["attributes"] = Value::Object(attributes);
    }
    let edges: Vec<Value> = projection_edges
        .iter()
        .enumerate()
        .map(|(index, edge)| {
            let origin = field(edge, "origin").as_u64().map(|i| format!("n:{i}"));
            json!({
                "id": format!("e:{index}"),
                "source": format!("n:{}", as_string(field(edge, "source"), "")),
                "target": format!("n:{}", as_string(field(edge, "target"), "")),
                "label": "",
                "edgeKind": normalize_edge_kind(&as_string(field(edge, "kind"), "relationship")),
                "attributes": {
                    "originNodeId": origin,
                    "semanticReference": field(edge, "reference")
                        .as_u64()
                        .and_then(|i| references.get(i as usize))
                        .cloned(),
                    "provenance": field(edge, "provenance"),
                    "sourceNavigation": if field(edge, "navigation").is_null() {
                        Value::Null
                    } else {
                        navigation(field(edge, "navigation"))
                    },
                },
            })
        })
        .collect();
    let metadata = field(projection, "metadata");
    let sequence_diagram = if selected_kind == "sequence-view" {
        sequence_diagram_from_scene(selected_name, &nodes, field(projection, "scene"))
    } else {
        None
    };
    let mut meta = json!({
        "selectedDiagramReference": field(selected, "reference")
            .as_u64()
            .and_then(|i| references.get(i as usize))
            .cloned(),
        "exposedRoots": as_array(field(projection, "exposedRoots"))
            .iter()
            .map(|index| format!("n:{}", as_string(index, "")))
            .collect::<Vec<_>>(),
        "viewMetadata": metadata,
    });
    if let Some(sequence_diagram) = sequence_diagram {
        meta["sequenceDiagram"] = sequence_diagram;
    }
    let mut prepared = json!({
        "title": selected_name,
        "view": selected_kind,
        "nodes": nodes,
        "edges": edges,
        "meta": meta,
    });
    enrich_typed_catalog_product(selected_kind, &mut prepared);
    Some(Ok(prepared))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_container_groups_keep_encounter_order() {
        let groups = build_general_package_container_groups(&[
            json!({ "id": "B::one", "attributes": {} }),
            json!({ "id": "A::two", "attributes": {} }),
        ]);
        assert_eq!(
            groups
                .iter()
                .map(|group| as_string(field(group, "name"), ""))
                .collect::<Vec<_>>(),
            vec!["B".to_string(), "A".to_string()]
        );
    }

    #[test]
    fn best_behavior_diagram_keeps_the_first_equal_score() {
        let diagrams = vec![
            json!({ "name": "first", "nodes": [{}, {}], "edges": [{}] }),
            json!({ "name": "second", "nodes": [{}, {}], "edges": [{}] }),
        ];
        let best = best_behavior_diagram(&diagrams).expect("catalog is non-empty");
        assert_eq!(best["name"], "first");
    }

    #[test]
    fn browser_hierarchy_preserves_tree_roots_and_sibling_order() {
        let prepared = prepare_view_data(&json!({
            "view": "browser-view",
            "selectedViewName": "Structure Browser",
            "projectionHints": {
                "browserLayout": "hierarchy",
                "treeRoots": ["zRoot", "aRoot"]
            },
            "generalViewGraph": {
                "nodes": [
                    { "id": "zRoot", "name": "zRoot", "type": "part", "parentId": "" },
                    { "id": "aRoot", "name": "aRoot", "type": "part", "parentId": "" },
                    { "id": "zChild", "name": "zChild", "type": "part", "parentId": "zRoot" },
                    { "id": "aChild", "name": "aChild", "type": "part", "parentId": "zRoot" }
                ],
                "edges": []
            }
        }))
        .expect("prepare");
        let rows = as_array(field(field(&prepared, "meta"), "rows"));
        assert_eq!(field(&prepared, "view"), "browser-view");
        assert_eq!(field(field(&prepared, "meta"), "hierarchyLayout"), true);
        assert_eq!(
            rows.iter()
                .map(|row| as_string(field(row, "id"), ""))
                .collect::<Vec<_>>(),
            vec!["zRoot", "zChild", "aChild", "aRoot"]
        );
        assert_eq!(field(&rows[0], "hasChildren"), true);
        assert_eq!(field(&rows[1], "parentId"), "zRoot");
    }

    #[test]
    fn grid_relationship_matrix_keeps_every_kind_between_a_pair() {
        let prepared = prepare_view_data(&json!({
            "view": "grid-view",
            "projectionHints": { "gridSubtype": "relationship_matrix" },
            "generalViewGraph": {
                "nodes": [
                    { "id": "a", "name": "a", "type": "part" },
                    { "id": "b", "name": "b", "type": "part" }
                ],
                "edges": [
                    { "source": "a", "target": "b", "type": "Dependency" },
                    { "source": "a", "target": "b", "type": "Satisfy" }
                ]
            }
        }))
        .expect("prepare");
        assert_eq!(field(field(&prepared, "meta"), "relationshipMatrix"), true);
        let cells = as_array(field(field(&prepared, "meta"), "matrixCells"));
        let cell = cells
            .iter()
            .find(|entry| {
                as_string(field(entry, "source"), "") == "a"
                    && as_string(field(entry, "target"), "") == "b"
            })
            .expect("a→b cell");
        assert_eq!(field(cell, "present"), true);
        assert_eq!(
            field(cell, "labels").clone(),
            json!(["Dependency", "Satisfy"])
        );
    }
}
