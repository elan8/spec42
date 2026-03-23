use std::collections::{HashMap, HashSet, VecDeque};

use crate::layout::{DiagramGraph, LayerDirection, LayoutConfig, LayoutViewProfile};

use crate::{
    shared::{build_rendered_diagram_with_config, default_node, detail_lines, edge},
    GraphEdgeInput, GraphNodeInput, RenderedDiagram, Result,
};

pub fn render(nodes: &[GraphNodeInput], edges: &[GraphEdgeInput]) -> Result<RenderedDiagram> {
    let node_by_id: HashMap<&str, &GraphNodeInput> =
        nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let structural_mode = prefer_structural_general_view(nodes);
    let kept_ids = collect_kept_ids(nodes, edges, &node_by_id, structural_mode);

    let diagram_nodes = nodes
        .iter()
        .filter(|node| kept_ids.contains(node.id.as_str()))
        .map(|node| {
            let detail = general_detail_lines(node);
            default_node(
                node.id.clone(),
                node.name.clone(),
                node.element_type.clone(),
                None,
                detail,
                Vec::new(),
                220.0,
                64.0 + node.attributes.len().min(2) as f32 * 14.0,
            )
        })
        .collect::<Vec<_>>();

    let diagram_edges = collect_general_edges(edges, &kept_ids, structural_mode);

    build_rendered_diagram_with_config(
        DiagramGraph {
            nodes: diagram_nodes,
            edges: diagram_edges,
        },
        "general-view",
        None,
        LayoutConfig {
            root_layer_direction: LayerDirection::HorizontalRows,
            layer_direction: LayerDirection::HorizontalRows,
            node_gap_x: 44.0,
            node_gap_y: 72.0,
            root_gap_x: 80.0,
            root_gap_y: 96.0,
            view_profile: LayoutViewProfile::GeneralView,
            ..LayoutConfig::default()
        },
    )
}

fn collect_kept_ids<'a>(
    nodes: &'a [GraphNodeInput],
    edges: &'a [GraphEdgeInput],
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
    structural_mode: bool,
) -> HashSet<&'a str> {
    if structural_mode {
        return collect_structural_kept_ids(nodes, edges, node_by_id);
    }

    let keepable = nodes
        .iter()
        .filter(|node| should_keep_node(node, node_by_id))
        .collect::<Vec<_>>();
    let relevant_edges = edges
        .iter()
        .filter(|edge| include_general_relation(edge.rel_type.as_str()))
        .collect::<Vec<_>>();
    let roots = select_general_roots(nodes, &relevant_edges, node_by_id);
    let mut kept = HashSet::new();
    let mut queue = VecDeque::new();

    for root in roots {
        if kept.insert(root) {
            queue.push_back((root, 0usize));
        }
    }

    while let Some((node_id, depth)) = queue.pop_front() {
        for edge in relevant_edges
            .iter()
            .copied()
            .filter(|edge| edge.source.as_str() == node_id || edge.target.as_str() == node_id)
        {
            if depth >= max_depth_for_relation(edge.rel_type.as_str()) {
                continue;
            }

            for next_id in [edge.source.as_str(), edge.target.as_str()] {
                if next_id == node_id || kept.contains(next_id) {
                    continue;
                }
                let Some(next_node) = node_by_id.get(next_id).copied() else {
                    continue;
                };
                if !should_keep_node(next_node, node_by_id) {
                    continue;
                }
                kept.insert(next_id);
                queue.push_back((next_id, depth + 1));
            }
        }
    }

    if keepable.len() <= 16 {
        for node in &keepable {
            if always_keep_kind(&node.element_type) {
                kept.insert(node.id.as_str());
            }
        }
    }

    if kept.is_empty() {
        keepable
            .into_iter()
            .filter(|node| always_keep_kind(&node.element_type))
            .take(24)
            .map(|node| node.id.as_str())
            .collect()
    } else {
        kept
    }
}

fn collect_structural_kept_ids<'a>(
    nodes: &'a [GraphNodeInput],
    edges: &'a [GraphEdgeInput],
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
) -> HashSet<&'a str> {
    let typed_part_ids = edges
        .iter()
        .filter(|edge| edge.rel_type == "typing")
        .map(|edge| edge.source.as_str())
        .collect::<HashSet<_>>();

    nodes
        .iter()
        .filter(|node| should_keep_structural_node(node))
        .filter(|node| !has_hidden_structural_ancestor(node, node_by_id, &typed_part_ids))
        .map(|node| node.id.as_str())
        .collect()
}

fn collect_general_edges(
    edges: &[GraphEdgeInput],
    kept_ids: &HashSet<&str>,
    structural_mode: bool,
) -> Vec<crate::layout::DiagramEdge> {
    let mut unique = HashSet::new();
    let mut out = Vec::new();

    for edge_item in edges.iter().filter(|edge_item| {
        if structural_mode {
            include_structural_relation(edge_item.rel_type.as_str())
        } else {
            include_general_relation(edge_item.rel_type.as_str())
        }
    }) {
        if !kept_ids.contains(edge_item.source.as_str())
            || !kept_ids.contains(edge_item.target.as_str())
        {
            continue;
        }

        let key = (
            edge_item.source.clone(),
            edge_item.target.clone(),
            edge_item.rel_type.clone(),
        );
        if !unique.insert(key) {
            continue;
        }

        let index = out.len();
        out.push(edge(
            format!("general-edge-{index}"),
            edge_item.source.clone(),
            edge_item.target.clone(),
            None,
            None,
            edge_item
                .name
                .clone()
                .or_else(|| Some(edge_item.rel_type.clone())),
            edge_item.rel_type.clone(),
        ));
    }

    out
}

fn prefer_structural_general_view(nodes: &[GraphNodeInput]) -> bool {
    let structural = nodes
        .iter()
        .filter(|node| should_keep_structural_node(node))
        .count();
    let total_non_package = nodes
        .iter()
        .filter(|node| normalized_kind(&node.element_type) != "package")
        .count();

    structural >= 8 || (structural >= 4 && structural * 2 >= total_non_package.max(1))
}

fn should_keep_structural_node(node: &GraphNodeInput) -> bool {
    let kind = normalized_kind(&node.element_type);
    kind == "part" || kind.contains("part def")
}

fn include_structural_relation(rel_type: &str) -> bool {
    matches!(rel_type, "contains" | "typing" | "specializes")
}

fn select_general_roots<'a>(
    nodes: &'a [GraphNodeInput],
    edges: &[&'a GraphEdgeInput],
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
) -> Vec<&'a str> {
    let node_refs = nodes.iter().collect::<Vec<_>>();
    select_general_roots_from_refs(&node_refs, edges, node_by_id)
}

fn select_general_roots_from_refs<'a>(
    nodes: &[&'a GraphNodeInput],
    edges: &[&'a GraphEdgeInput],
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
) -> Vec<&'a str> {
    let mut outgoing = HashMap::<&str, usize>::new();
    let mut incoming = HashMap::<&str, usize>::new();
    let mut outgoing_contains = HashMap::<&str, usize>::new();
    let mut outgoing_behavior = HashMap::<&str, usize>::new();

    for edge in edges {
        *outgoing.entry(edge.source.as_str()).or_default() += 1;
        *incoming.entry(edge.target.as_str()).or_default() += 1;
        if edge.rel_type == "contains" {
            *outgoing_contains.entry(edge.source.as_str()).or_default() += 1;
        }
        if matches!(edge.rel_type.as_str(), "perform" | "allocate" | "satisfy") {
            *outgoing_behavior.entry(edge.source.as_str()).or_default() += 1;
        }
    }

    let mut scored = nodes
        .iter()
        .copied()
        .filter(|node| should_keep_node(node, node_by_id))
        .filter(|node| {
            outgoing.contains_key(node.id.as_str()) || incoming.contains_key(node.id.as_str())
        })
        .map(|node| {
            let id = node.id.as_str();
            let name = node.name.to_ascii_lowercase();
            let kind = normalized_kind(&node.element_type);
            let mut score = 0isize;

            score += outgoing_contains.get(id).copied().unwrap_or_default() as isize * 12;
            score += outgoing_behavior.get(id).copied().unwrap_or_default() as isize * 8;
            score += outgoing.get(id).copied().unwrap_or_default() as isize * 2;
            score -= incoming.get(id).copied().unwrap_or_default() as isize;

            if kind.contains("part def") {
                score += 20;
            } else if kind == "part" {
                score += 12;
            }
            if name.contains("drone") {
                score += 24;
            }
            if name.contains("instance") {
                score += 10;
            }
            if kind.contains("requirement") || kind.contains("action") || kind.contains("state") {
                score -= 8;
            }

            (id, score)
        })
        .collect::<Vec<_>>();

    scored.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(right.0)));
    scored
        .into_iter()
        .filter(|(_, score)| *score > 0)
        .take(3)
        .map(|(id, _)| id)
        .collect()
}

fn include_general_relation(rel_type: &str) -> bool {
    matches!(
        rel_type,
        "contains"
            | "typing"
            | "specializes"
            | "perform"
            | "allocate"
            | "allocation"
            | "satisfy"
            | "verify"
            | "transition"
    )
}

fn max_depth_for_relation(rel_type: &str) -> usize {
    match rel_type {
        "contains" | "typing" => 3,
        "specializes" | "perform" | "allocate" | "allocation" | "satisfy" | "verify" => 2,
        _ => 1,
    }
}

fn always_keep_kind(value: &str) -> bool {
    let kind = normalized_kind(value);
    kind.contains("part")
        || kind.contains("action")
        || kind.contains("state")
        || kind.contains("use case")
        || kind.contains("requirement")
        || kind.contains("actor")
        || kind.contains("interface")
        || kind.contains("constraint")
        || kind.contains("enumeration")
        || kind.contains("item")
}

fn should_keep_node<'a>(
    node: &'a GraphNodeInput,
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
) -> bool {
    let kind = normalized_kind(&node.element_type);
    if kind == "package" {
        return false;
    }
    if kind.contains("attribute") || kind.contains("metadata") || kind.contains("port") {
        return false;
    }

    let depth = non_package_depth(node, node_by_id);
    if kind.contains("part")
        || kind.contains("interface")
        || kind.contains("requirement")
        || kind.contains("use case")
        || kind.contains("action")
        || kind.contains("state")
        || kind.contains("enumeration")
        || kind.contains("constraint")
        || kind.contains("allocation")
        || kind.contains("item")
        || kind.contains("actor")
        || kind.contains("analysis")
        || kind.contains("verification")
    {
        return depth <= 4;
    }

    depth <= 3
}

fn is_instance_root<'a>(
    node: &'a GraphNodeInput,
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
) -> bool {
    if normalized_kind(&node.element_type) != "part" {
        return false;
    }

    let Some(parent_id) = node.parent_id.as_deref() else {
        return true;
    };
    let Some(parent) = node_by_id.get(parent_id).copied() else {
        return true;
    };

    normalized_kind(&parent.element_type) == "package"
}

fn has_hidden_structural_ancestor<'a>(
    node: &'a GraphNodeInput,
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
    typed_part_ids: &HashSet<&'a str>,
) -> bool {
    let mut current = node.parent_id.as_deref();
    while let Some(parent_id) = current {
        let Some(parent) = node_by_id.get(parent_id).copied() else {
            break;
        };
        if is_hidden_structural_owner(parent, node_by_id, typed_part_ids) {
            return true;
        }
        current = parent.parent_id.as_deref();
    }
    false
}

fn is_hidden_structural_owner<'a>(
    node: &'a GraphNodeInput,
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
    typed_part_ids: &HashSet<&'a str>,
) -> bool {
    is_instance_root(node, node_by_id)
        || (normalized_kind(&node.element_type) == "part" && typed_part_ids.contains(node.id.as_str()))
}

fn normalized_kind(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn non_package_depth<'a>(
    node: &'a GraphNodeInput,
    node_by_id: &HashMap<&'a str, &'a GraphNodeInput>,
) -> usize {
    let mut depth = 0usize;
    let mut current = node.parent_id.as_deref();
    while let Some(parent_id) = current {
        let Some(parent) = node_by_id.get(parent_id).copied() else {
            break;
        };
        if normalized_kind(&parent.element_type) != "package" {
            depth += 1;
        }
        current = parent.parent_id.as_deref();
    }
    depth
}

fn general_detail_lines(node: &GraphNodeInput) -> Vec<String> {
    let mut extras = vec![format!("type: {}", node.element_type)];
    if let Some((key, value)) = node.attributes.iter().find(|(key, _)| {
        !matches!(
            key.as_str(),
            "declaredName" | "effectiveName" | "name" | "type"
        )
    }) {
        extras.push(format!("{key}: {value}"));
    }
    detail_lines(&[], &extras)
}
