//! Renderer-owned nesting limit for interconnection views.
//!
//! An interconnection view projects every part below its exposed roots. Drawn in full, a root with
//! a few levels of parts cannot be read at any size a page or a screen offers. A nesting limit
//! draws the roots and that many levels of parts below them; a part with deeper content is drawn
//! closed and says how many parts it holds. This is presentation only: the projection, and the
//! product a consumer reads, still carry every part and connector.
//!
//! A connector that ends inside a closed part is drawn to that part, without a port, because the
//! port it ends on is not drawn. A connector that lies wholly inside one closed part is not drawn.

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

    let mut closed: HashMap<String, usize> = HashMap::new();
    for id in by_id.keys().filter(|id| !drawn.contains(*id)) {
        if let Some(owner) = drawn_owner(id) {
            *closed.entry(owner).or_default() += 1;
        }
    }
    let kept_nodes: Vec<Value> = nodes
        .iter()
        .filter(|node| drawn.contains(&as_string(field(node, "id"), "")))
        .map(|node| {
            let mut node = node.clone();
            if let Some(count) = closed.get(&as_string(field(&node, "id"), "")) {
                node["attributes"]["closedParts"] = json!(count);
            }
            node
        })
        .collect();

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
        // Several connectors into one closed part read as one line once their ports are gone.
        let key = (
            drawn_source.clone(),
            source_port.to_string(),
            drawn_target.clone(),
            target_port.to_string(),
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
        edge["attributes"]["endsInClosedPart"] = json!(true);
        kept_edges.push(edge);
    }

    prepared["nodes"] = Value::Array(kept_nodes);
    prepared["edges"] = Value::Array(kept_edges);
    prepared["meta"]["nestingLevels"] = json!(levels);
    prepared["meta"]["closedPartCount"] = json!(nodes.len() - drawn.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part(id: &str, container: Option<&str>) -> Value {
        json!({ "id": id, "attributes": { "containerId": container } })
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
        // The connector inside the closed part is gone; the two that reach into it are one
        // line to the part, without the ports that are not drawn.
        assert_eq!(ids(&prepared, "edges"), ["between-modules", "deep-to-top"]);
        let moved = &as_array(field(&prepared, "edges"))[1];
        assert_eq!(field(moved, "source"), &json!("base"));
        assert_eq!(
            field(field(moved, "attributes"), "sourcePortId"),
            &Value::Null
        );
        assert_eq!(
            field(field(moved, "attributes"), "targetPortId"),
            &json!("p3")
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
    fn other_views_are_untouched() {
        let mut prepared = view();
        prepared["view"] = json!("general-view");
        let before = prepared.clone();
        apply_interconnection_nesting(&mut prepared, Nesting::Levels(0));
        assert_eq!(prepared, before);
    }
}
