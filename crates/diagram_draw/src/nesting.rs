//! Renderer-owned nesting limit for interconnection and general views.
//!
//! An interconnection view projects every part below its exposed roots. Drawn in full, a root with
//! a few levels of parts cannot be read at any size a page or a screen offers. A nesting limit
//! draws the roots and that many levels of parts below them; a part with deeper content is drawn
//! closed, with the ellipsis that marks left-out content (SysML 8.2.3.2). This is presentation only: the projection, and the
//! product a consumer reads, still carry every part and connector.
//!
//! A connector that ends inside a closed part is drawn to a proxy on that part's boundary that
//! names the feature it ends on (SysML 8.2.3.12). A connector that lies wholly inside one closed
//! part is not drawn.

use std::collections::{BTreeSet, HashMap};

use serde_json::{json, Value};

use crate::json_util::{as_array, as_string, field};

/// How many levels of parts are drawn below the exposed roots of an interconnection view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Nesting {
    /// Every level the view projects.
    #[default]
    All,
    /// The roots and this many levels of parts below them. `Levels(1)` is a root with its
    /// direct parts as closed boxes.
    Levels(usize),
}

fn container_of(node: &Value) -> Option<String> {
    match field(field(node, "attributes"), "containerId") {
        Value::String(id) if !id.is_empty() => Some(id.clone()),
        _ => None,
    }
}

fn is_synthetic(node: &Value) -> bool {
    field(field(node, "attributes"), "isSyntheticContainer") == &json!(true)
}

/// Applies `nesting` to a prepared view: an interconnection view closes its deeper parts, a
/// general view keeps its deeper members in the compartments of the last level drawn. Other
/// views are left as they are.
pub fn apply_nesting(prepared: &mut Value, nesting: Nesting) {
    match as_string(field(prepared, "view"), "").as_str() {
        "interconnection-view" => apply_interconnection_nesting(prepared, nesting),
        "general-view" => apply_general_nesting(prepared, nesting),
        _ => {}
    }
}

/// Draws the exposed elements of a general view and `levels` of members below them as boxes.
/// A member further down stays listed in the compartment of its owner, which is how the editor
/// shows an element that has not been expanded.
fn apply_general_nesting(prepared: &mut Value, nesting: Nesting) {
    let Nesting::Levels(levels) = nesting else {
        return;
    };
    let owner_id = |node: &Value| -> Option<String> {
        match field(field(node, "attributes"), "owner") {
            Value::Number(index) => index.as_u64().map(|index| format!("n:{index}")),
            Value::String(id) if !id.is_empty() => Some(id.clone()),
            _ => None,
        }
    };
    let nodes = as_array(field(prepared, "nodes")).to_vec();
    let owner: HashMap<String, String> = nodes
        .iter()
        .filter_map(|node| Some((as_string(field(node, "id"), ""), owner_id(node)?)))
        .collect();
    let level = |id: &str| -> usize {
        let mut level = 0;
        let mut current = id;
        while let Some(above) = owner.get(current) {
            level += 1;
            current = above;
            if level > nodes.len() {
                break;
            }
        }
        level
    };
    // An owner is expanded when its members are within the levels drawn.
    let mut expanded: Vec<String> = owner
        .values()
        .filter(|id| level(id) < levels)
        .cloned()
        .collect();
    expanded.sort();
    expanded.dedup();
    crate::disclosure::apply_general_disclosure(
        prepared,
        &crate::disclosure::DisclosureState {
            expanded_node_ids: expanded,
            section_states: Vec::new(),
        },
    );
    prepared["meta"]["nestingLevels"] = json!(levels);
}

/// Applies `nesting` to a prepared interconnection view. Other views are left as they are.
pub fn apply_interconnection_nesting(prepared: &mut Value, nesting: Nesting) {
    let Nesting::Levels(levels) = nesting else {
        return;
    };
    if as_string(field(prepared, "view"), "") != "interconnection-view" {
        return;
    }
    let nodes = as_array(field(prepared, "nodes")).to_vec();
    let by_id: HashMap<String, &Value> = nodes
        .iter()
        .map(|node| (as_string(field(node, "id"), ""), node))
        .collect();
    let parent = |id: &str| -> Option<String> {
        container_of(by_id.get(id)?).filter(|parent| by_id.contains_key(parent))
    };
    // The level of a part: the number of parts above it. A package the scene adds to group
    // roots is not a level.
    let level = |id: &str| -> usize {
        let mut level = 0;
        let mut current = id.to_owned();
        let mut guard = nodes.len();
        while let Some(above) = parent(&current) {
            if by_id
                .get(above.as_str())
                .is_some_and(|node| !is_synthetic(node))
            {
                level += 1;
            }
            current = above;
            guard = guard.saturating_sub(1);
            if guard == 0 {
                break;
            }
        }
        level
    };
    let drawn: BTreeSet<String> = by_id
        .keys()
        .filter(|id| level(id) <= levels)
        .cloned()
        .collect();
    if drawn.len() == nodes.len() {
        return;
    }
    // The drawn part a part belongs to: itself, or its nearest drawn ancestor.
    let drawn_owner = |id: &str| -> Option<String> {
        let mut current = id.to_owned();
        let mut guard = nodes.len() + 1;
        while !drawn.contains(&current) {
            current = parent(&current)?;
            guard = guard.saturating_sub(1);
            if guard == 0 {
                return None;
            }
        }
        Some(current)
    };

    // The path of a feature inside a closed part, from that part: the names of the parts down
    // to the one that owns the port, then the port (`motor.shaft`).
    let feature_path = |closed_part: &str, owner: &str, port: &Value| -> String {
        let mut names = Vec::new();
        let mut current = owner.to_owned();
        while current != closed_part {
            names.push(as_string(field(by_id[current.as_str()], "label"), ""));
            match parent(&current) {
                Some(above) => current = above,
                None => break,
            }
        }
        names.reverse();
        let port_id = as_string(port, "");
        let port_name = as_array(field(field(by_id[owner], "attributes"), "portDetails"))
            .iter()
            .find(|detail| as_string(field(detail, "id"), "") == port_id)
            .map(|detail| as_string(field(detail, "name"), ""))
            .unwrap_or_default();
        if !port_name.is_empty() {
            names.push(port_name);
        }
        names.join(".")
    };

    let mut closed: HashMap<String, usize> = HashMap::new();
    for id in by_id.keys().filter(|id| !drawn.contains(*id)) {
        if let Some(owner) = drawn_owner(id) {
            *closed.entry(owner).or_default() += 1;
        }
    }
    // The proxies each drawn part gets, by the path of the feature they stand for.
    let mut proxies: HashMap<String, BTreeSet<String>> = HashMap::new();
    let proxy_id = |part: &str, path: &str| format!("proxy:{part}:{path}");
    let mut seen = BTreeSet::new();
    let mut kept_edges = Vec::new();
    for edge in as_array(field(prepared, "edges")) {
        let source = as_string(field(edge, "source"), "");
        let target = as_string(field(edge, "target"), "");
        let (Some(drawn_source), Some(drawn_target)) = (drawn_owner(&source), drawn_owner(&target))
        else {
            continue;
        };
        let moved_source = drawn_source != source;
        let moved_target = drawn_target != target;
        if !moved_source && !moved_target {
            kept_edges.push(edge.clone());
            continue;
        }
        if drawn_source == drawn_target {
            continue;
        }
        let mut edge = edge.clone();
        let attributes = field(&edge, "attributes").clone();
        let port = |key: &str, moved: bool| {
            if moved {
                Value::Null
            } else {
                field(&attributes, key).clone()
            }
        };
        let source_port = port("sourcePortId", moved_source);
        let target_port = port("targetPortId", moved_target);
        let proxy_source = moved_source
            .then(|| feature_path(&drawn_source, &source, field(&attributes, "sourcePortId")));
        let proxy_target = moved_target
            .then(|| feature_path(&drawn_target, &target, field(&attributes, "targetPortId")));
        // Each distinct inner feature keeps its own line and proxy; two connectors between the
        // same two ends would draw the same line twice.
        let key = (
            drawn_source.clone(),
            proxy_source
                .clone()
                .unwrap_or_else(|| source_port.to_string()),
            drawn_target.clone(),
            proxy_target
                .clone()
                .unwrap_or_else(|| target_port.to_string()),
        );
        if !seen.insert(key) {
            continue;
        }
        edge["source"] = json!(drawn_source);
        edge["target"] = json!(drawn_target);
        edge["attributes"]["sourceNodeId"] = json!(drawn_source);
        edge["attributes"]["targetNodeId"] = json!(drawn_target);
        edge["attributes"]["sourcePortId"] = source_port.clone();
        edge["attributes"]["sourceId"] = source_port;
        edge["attributes"]["targetPortId"] = target_port.clone();
        edge["attributes"]["targetId"] = target_port;
        // A hidden end attaches to a proxy of the drawn part, which the layout places and
        // labels like a port.
        if let Some(path) = proxy_source {
            let id = proxy_id(&drawn_source, &path);
            edge["attributes"]["sourcePortId"] = json!(id);
            edge["attributes"]["sourceId"] = json!(id);
            edge["attributes"]["proxySource"] = json!(path);
            proxies
                .entry(drawn_source.clone())
                .or_default()
                .insert(path);
        }
        if let Some(path) = proxy_target {
            let id = proxy_id(&drawn_target, &path);
            edge["attributes"]["targetPortId"] = json!(id);
            edge["attributes"]["targetId"] = json!(id);
            edge["attributes"]["proxyTarget"] = json!(path);
            proxies
                .entry(drawn_target.clone())
                .or_default()
                .insert(path);
        }
        kept_edges.push(edge);
    }

    let kept_nodes: Vec<Value> = nodes
        .iter()
        .filter(|node| drawn.contains(&as_string(field(node, "id"), "")))
        .map(|node| {
            let mut node = node.clone();
            let id = as_string(field(&node, "id"), "");
            if let Some(count) = closed.get(&id) {
                node["attributes"]["closedParts"] = json!(count);
            }
            if let Some(paths) = proxies.get(&id) {
                let mut details =
                    as_array(field(field(&node, "attributes"), "portDetails")).to_vec();
                let mut names = as_array(field(field(&node, "attributes"), "ports")).to_vec();
                for path in paths {
                    details.push(json!({ "id": proxy_id(&id, path), "name": path, "proxy": true }));
                    names.push(json!(path));
                }
                node["attributes"]["portDetails"] = Value::Array(details);
                node["attributes"]["ports"] = Value::Array(names);
            }
            node
        })
        .collect();

    prepared["nodes"] = Value::Array(kept_nodes);
    prepared["edges"] = Value::Array(kept_edges);
    prepared["meta"]["nestingLevels"] = json!(levels);
    prepared["meta"]["closedPartCount"] = json!(nodes.len() - drawn.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part(id: &str, container: Option<&str>) -> Value {
        json!({
            "id": id,
            "label": id,
            "attributes": {
                "containerId": container,
                "portDetails": [{ "id": "shaft", "name": "shaft" }, { "id": "out", "name": "out" }],
            },
        })
    }

    fn connector(id: &str, source: (&str, &str), target: (&str, &str)) -> Value {
        json!({
            "id": id,
            "source": source.0,
            "target": target.0,
            "attributes": {
                "sourceNodeId": source.0, "sourcePortId": source.1, "sourceId": source.1,
                "targetNodeId": target.0, "targetPortId": target.1, "targetId": target.1,
            },
        })
    }

    fn view() -> Value {
        json!({
            "view": "interconnection-view",
            "nodes": [
                part("robot", None),
                part("base", Some("robot")),
                part("top", Some("robot")),
                part("motor", Some("base")),
                part("gear", Some("base")),
                part("rotor", Some("motor")),
            ],
            "edges": [
                connector("between-modules", ("base", "p1"), ("top", "p2")),
                connector("inside-base", ("motor", "out"), ("gear", "in")),
                connector("deep-to-top", ("rotor", "shaft"), ("top", "p3")),
                connector("deep-to-top-again", ("gear", "out"), ("top", "p3")),
            ],
            "meta": {},
        })
    }

    fn ids(prepared: &Value, key: &str) -> Vec<String> {
        as_array(field(prepared, key))
            .iter()
            .map(|item| as_string(field(item, "id"), ""))
            .collect()
    }

    #[test]
    fn parts_below_the_limit_close_into_their_parent() {
        let mut prepared = view();
        apply_interconnection_nesting(&mut prepared, Nesting::Levels(1));
        assert_eq!(ids(&prepared, "nodes"), ["robot", "base", "top"]);
        let base = &as_array(field(&prepared, "nodes"))[1];
        assert_eq!(field(field(base, "attributes"), "closedParts"), &json!(3));
        // The connector inside the closed part is gone; each one that reaches into it ends on
        // the part, without the port that is not drawn.
        assert_eq!(
            ids(&prepared, "edges"),
            ["between-modules", "deep-to-top", "deep-to-top-again"]
        );
        let moved = &as_array(field(&prepared, "edges"))[1];
        assert_eq!(field(moved, "source"), &json!("base"));
        assert_eq!(
            field(field(moved, "attributes"), "sourcePortId"),
            &json!("proxy:base:motor.rotor.shaft")
        );
        // The closed part carries the proxies as ports of its own, so they are laid out.
        let proxy_ports: Vec<String> = as_array(field(field(base, "attributes"), "portDetails"))
            .iter()
            .filter(|detail| field(detail, "proxy") == &json!(true))
            .map(|detail| as_string(field(detail, "name"), ""))
            .collect();
        assert_eq!(proxy_ports, ["gear.out", "motor.rotor.shaft"]);
        assert_eq!(
            field(field(moved, "attributes"), "targetPortId"),
            &json!("p3")
        );
        // The hidden end is remembered by its path from the closed part, for the proxy label.
        assert_eq!(
            field(field(moved, "attributes"), "proxySource"),
            &json!("motor.rotor.shaft")
        );
        assert_eq!(
            field(field(moved, "attributes"), "proxyTarget"),
            &Value::Null
        );
        assert_eq!(
            field(field(&prepared, "meta"), "closedPartCount"),
            &json!(3)
        );
    }

    #[test]
    fn a_limit_that_hides_nothing_changes_nothing() {
        for nesting in [Nesting::All, Nesting::Levels(3)] {
            let mut prepared = view();
            apply_interconnection_nesting(&mut prepared, nesting);
            assert_eq!(prepared, view());
        }
    }

    #[test]
    fn a_general_view_keeps_deeper_members_in_compartments() {
        let member =
            |id: &str, owner: Option<u64>| json!({ "id": id, "attributes": { "owner": owner } });
        let view = json!({
            "view": "general-view",
            "nodes": [member("n:0", None), member("n:1", Some(0)), member("n:2", Some(1))],
            "edges": [
                { "id": "a", "source": "n:0", "target": "n:1" },
                { "id": "b", "source": "n:1", "target": "n:2" },
            ],
            "meta": {},
        });
        let drawn = |levels: usize| {
            let mut prepared = view.clone();
            apply_nesting(&mut prepared, Nesting::Levels(levels));
            (ids(&prepared, "nodes"), ids(&prepared, "edges"))
        };
        assert_eq!(drawn(0), (vec!["n:0".to_owned()], Vec::<String>::new()));
        assert_eq!(
            drawn(1),
            (
                vec!["n:0".to_owned(), "n:1".to_owned()],
                vec!["a".to_owned()]
            )
        );
        assert_eq!(drawn(2).0.len(), 3);

        // The last level drawn is marked collapsed, so the drawing lists its members.
        let mut prepared = view.clone();
        apply_nesting(&mut prepared, Nesting::Levels(1));
        let nodes = as_array(field(&prepared, "nodes"));
        assert_eq!(
            field(field(&nodes[0], "attributes"), "disclosure"),
            &json!("expanded")
        );
        assert_eq!(
            field(field(&nodes[1], "attributes"), "disclosure"),
            &json!("collapsed")
        );

        let mut everything = view.clone();
        apply_nesting(&mut everything, Nesting::All);
        assert_eq!(everything, view);
    }

    #[test]
    fn other_views_are_untouched() {
        let mut prepared = view();
        prepared["view"] = json!("state-transition-view");
        let before = prepared.clone();
        apply_nesting(&mut prepared, Nesting::Levels(0));
        assert_eq!(prepared, before);
    }
}
