use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagramEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    pub edge_kind: String,
}

pub fn normalize_edge_kind(relationship_type: &str) -> String {
    let type_name = relationship_type.trim().to_lowercase();
    if type_name.is_empty() {
        return "relationship".to_string();
    }
    if type_name.contains("connection") || type_name == "connect" {
        return "connection".to_string();
    }
    if type_name == "satisfy" {
        return "satisfy".to_string();
    }
    if type_name == "verify" {
        return "verify".to_string();
    }
    if type_name == "typing" {
        return "typing".to_string();
    }
    if type_name == "specializes" || type_name == "specialization" {
        return "specializes".to_string();
    }
    if type_name == "bind" || type_name == "binding" {
        return "bind".to_string();
    }
    if type_name == "allocate" || type_name == "allocation" {
        return "allocate".to_string();
    }
    if type_name == "transition" {
        return "transition".to_string();
    }
    if type_name == "hierarchy" || type_name == "contains" || type_name == "owns" {
        return "hierarchy".to_string();
    }
    let normalized = type_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string();
    if normalized.is_empty() {
        "relationship".to_string()
    } else {
        normalized
    }
}

pub fn deduplicate_edges(edges: Vec<DiagramEdge>) -> Vec<DiagramEdge> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();
    for edge in edges {
        let key = format!("{}\0{}\0{}", edge.source, edge.target, edge.edge_kind);
        if seen.insert(key) {
            unique.push(edge);
        }
    }

    let semantic_pairs: HashSet<String> = unique
        .iter()
        .filter(|edge| edge.edge_kind == "typing" || edge.edge_kind == "specializes")
        .map(|edge| edge_pair_key(&edge.source, &edge.target))
        .collect();

    unique
        .into_iter()
        .filter(|edge| {
            edge.edge_kind != "hierarchy"
                || !semantic_pairs.contains(&edge_pair_key(&edge.source, &edge.target))
        })
        .collect()
}

pub fn edge_pair_key(source: &str, target: &str) -> String {
    if source < target {
        format!("{source}\0{target}")
    } else {
        format!("{target}\0{source}")
    }
}

pub fn has_edge_between(edges: &[DiagramEdge], a: &str, b: &str) -> bool {
    edges
        .iter()
        .any(|edge| (edge.source == a && edge.target == b) || (edge.source == b && edge.target == a))
}

#[cfg(test)]
mod tests {
    use super::{deduplicate_edges, normalize_edge_kind, DiagramEdge};

    #[test]
    fn normalize_edge_kind_maps_common_types() {
        assert_eq!(normalize_edge_kind("typing"), "typing");
        assert_eq!(normalize_edge_kind("specialization"), "specializes");
        assert_eq!(normalize_edge_kind("owns"), "hierarchy");
        assert_eq!(normalize_edge_kind(""), "relationship");
    }

    #[test]
    fn deduplicate_edges_keeps_semantic_edge_over_hierarchy() {
        let edges = vec![
            DiagramEdge {
                id: "h".to_string(),
                source: "a".to_string(),
                target: "b".to_string(),
                label: "contains".to_string(),
                edge_kind: "hierarchy".to_string(),
            },
            DiagramEdge {
                id: "t".to_string(),
                source: "a".to_string(),
                target: "b".to_string(),
                label: "typing".to_string(),
                edge_kind: "typing".to_string(),
            },
        ];
        let deduped = deduplicate_edges(edges);
        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0].edge_kind, "typing");
    }
}
