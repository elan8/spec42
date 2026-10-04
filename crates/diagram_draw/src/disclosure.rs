//! Renderer-owned disclosure projection (spec42 #176 / #181).
//!
//! General View: port of the former TypeScript `visibleProjection` — which nested nodes are
//! visible, which memberships stay listed as compartments, and the `disclosure` /
//! `compartmentSectionState` attributes drawing reads.
//!
//! Browser View: `expanded_node_ids` is the set of **collapsed** row ids (the toggle set the
//! client already mutates). Empty means every row is expanded, matching the former TypeScript
//! local `collapsed` Set. Drawing reads `meta.collapsedRowIds`.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::json_util::{as_array, as_object, as_string, field};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisclosureState {
    #[serde(default)]
    pub expanded_node_ids: Vec<String>,
    #[serde(default)]
    pub section_states: Vec<SectionState>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionState {
    pub node_id: String,
    pub section_key: String,
    pub expanded: bool,
}

fn owner_of(node: &Value) -> Option<String> {
    match field(field(node, "attributes"), "owner") {
        Value::Number(number) => number
            .as_u64()
            .map(|index| format!("n:{index}"))
            .or_else(|| number.as_i64().map(|index| format!("n:{index}"))),
        Value::String(value) if !value.is_empty() => Some(value.clone()),
        _ => None,
    }
}

/// Applies renderer-owned expansion/compartment state onto a prepared view.
/// No-op for views that have no disclosure.
pub fn apply_general_disclosure(prepared: &mut Value, state: &DisclosureState) {
    match as_string(field(prepared, "view"), "").as_str() {
        "general-view" => apply_general_view_disclosure(prepared, state),
        "browser-view" => apply_browser_disclosure(prepared, state),
        _ => {}
    }
}

fn apply_browser_disclosure(prepared: &mut Value, state: &DisclosureState) {
    let mut meta = as_object(field(prepared, "meta"));
    meta.insert(
        "collapsedRowIds".into(),
        json!(state.expanded_node_ids.clone()),
    );
    prepared["meta"] = Value::Object(meta);
}

fn apply_general_view_disclosure(prepared: &mut Value, state: &DisclosureState) {
    let expanded: HashSet<String> = state.expanded_node_ids.iter().cloned().collect();
    let mut section_state: HashMap<(String, String), bool> = HashMap::new();
    for section in &state.section_states {
        section_state.insert(
            (section.node_id.clone(), section.section_key.clone()),
            section.expanded,
        );
    }

    let nodes: Vec<Value> = as_array(field(prepared, "nodes")).to_vec();
    let roots: HashSet<String> = as_array(field(field(prepared, "meta"), "exposedRoots"))
        .iter()
        .map(|value| as_string(value, ""))
        .filter(|value| !value.is_empty())
        .collect();
    let owners: HashSet<String> = nodes.iter().filter_map(owner_of).collect();

    let mut visible = HashSet::new();
    let mut visiting = HashSet::new();
    fn visit(
        node: &Value,
        nodes: &[Value],
        roots: &HashSet<String>,
        expanded: &HashSet<String>,
        visible: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
    ) -> bool {
        let id = as_string(field(node, "id"), "");
        if visible.contains(&id) {
            return true;
        }
        if !visiting.insert(id.clone()) {
            return false;
        }
        let Some(owner) = owner_of(node) else {
            if roots.is_empty() || roots.contains(&id) {
                visible.insert(id);
                return true;
            }
            return false;
        };
        let Some(parent) = nodes
            .iter()
            .find(|candidate| as_string(field(candidate, "id"), "") == owner)
        else {
            return false;
        };
        if !visit(parent, nodes, roots, expanded, visible, visiting) || !expanded.contains(&owner) {
            return false;
        }
        visible.insert(id);
        true
    }
    for node in &nodes {
        visit(node, &nodes, &roots, &expanded, &mut visible, &mut visiting);
    }

    let projected_nodes: Vec<Value> = nodes
        .iter()
        .filter(|node| visible.contains(&as_string(field(node, "id"), "")))
        .map(|node| {
            let id = as_string(field(node, "id"), "");
            let is_expanded = expanded.contains(&id);
            let mut attributes = as_object(field(node, "attributes"));
            if owners.contains(&id) {
                attributes.insert(
                    "disclosure".into(),
                    Value::String(if is_expanded {
                        "expanded".into()
                    } else {
                        "collapsed".into()
                    }),
                );
            }
            if is_expanded {
                let typed = as_array(attributes.get("typedCompartments").unwrap_or(&Value::Null));
                let filtered: Vec<Value> = typed
                    .iter()
                    .filter_map(|compartment| {
                        let members: Vec<Value> = as_array(field(compartment, "members"))
                            .iter()
                            .filter(|member| {
                                let member_id = as_string(field(member, "id"), "");
                                member_id.is_empty() || !visible.contains(&member_id)
                            })
                            .cloned()
                            .collect();
                        if members.is_empty() {
                            return None;
                        }
                        let mut next = as_object(compartment);
                        next.insert("members".into(), Value::Array(members));
                        Some(Value::Object(next))
                    })
                    .collect();
                attributes.insert("typedCompartments".into(), Value::Array(filtered));
            }
            let mut sections = Map::new();
            for ((node_id, key), value) in &section_state {
                if node_id == &id {
                    sections.insert(key.clone(), json!(value));
                }
            }
            if !sections.is_empty() {
                attributes.insert("compartmentSectionState".into(), Value::Object(sections));
            }
            let mut next = node.clone();
            next["attributes"] = Value::Object(attributes);
            next
        })
        .collect();

    let projected_edges: Vec<Value> = as_array(field(prepared, "edges"))
        .iter()
        .filter(|edge| {
            visible.contains(&as_string(field(edge, "source"), ""))
                && visible.contains(&as_string(field(edge, "target"), ""))
        })
        .cloned()
        .collect();

    prepared["nodes"] = Value::Array(projected_nodes);
    prepared["edges"] = Value::Array(projected_edges);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample() -> Value {
        json!({
            "view": "general-view",
            "nodes": [
                { "id": "n:0", "label": "root", "attributes": {} },
                { "id": "n:1", "label": "child", "attributes": { "owner": 0 } }
            ],
            "edges": [{ "id": "e", "source": "n:0", "target": "n:1" }],
            "meta": {}
        })
    }

    #[test]
    fn nested_child_is_hidden_until_the_owner_is_expanded() {
        let mut collapsed = sample();
        apply_general_disclosure(&mut collapsed, &DisclosureState::default());
        assert_eq!(as_array(field(&collapsed, "nodes")).len(), 1);
        assert_eq!(as_array(field(&collapsed, "edges")).len(), 0);
        assert_eq!(
            as_string(
                field(
                    field(&as_array(field(&collapsed, "nodes"))[0], "attributes"),
                    "disclosure"
                ),
                ""
            ),
            "collapsed"
        );

        let mut expanded = sample();
        apply_general_disclosure(
            &mut expanded,
            &DisclosureState {
                expanded_node_ids: vec!["n:0".into()],
                section_states: vec![],
            },
        );
        assert_eq!(as_array(field(&expanded, "nodes")).len(), 2);
        assert_eq!(as_array(field(&expanded, "edges")).len(), 1);
        assert_eq!(
            as_string(
                field(
                    field(&as_array(field(&expanded, "nodes"))[0], "attributes"),
                    "disclosure"
                ),
                ""
            ),
            "expanded"
        );
    }

    #[test]
    fn browser_collapsed_row_ids_are_the_toggle_set() {
        let mut prepared = json!({
            "view": "browser-view",
            "nodes": [],
            "edges": [],
            "meta": { "rows": [], "hierarchyLayout": true }
        });
        apply_general_disclosure(
            &mut prepared,
            &DisclosureState {
                expanded_node_ids: vec!["root".into()],
                section_states: vec![],
            },
        );
        assert_eq!(
            as_array(field(field(&prepared, "meta"), "collapsedRowIds")),
            &[json!("root")]
        );
    }
}
