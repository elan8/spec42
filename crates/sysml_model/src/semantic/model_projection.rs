use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};
use url::Url;

use crate::semantic::dto::{range_to_dto, GraphEdgeDto, GraphNodeDto, SysmlGraphDto};
use crate::semantic::element_kind_classify::{is_attribute_like, is_parameter_like};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::kinds::is_port_like_str as is_port_like;

pub fn canonical_general_view_graph(
    graph: &SysmlGraphDto,
    _include_all_roots: bool,
) -> SysmlGraphDto {
    let filtered_graph = fold_general_view_leaf_details_into_owners(graph);

    let mut node_by_id: HashMap<String, GraphNodeDto> = HashMap::new();
    for node in &filtered_graph.nodes {
        node_by_id
            .entry(node.id.clone())
            .or_insert_with(|| node.clone());
    }

    let mut edge_keys: HashSet<(String, String, String, Option<String>)> = HashSet::new();
    let mut out_edges: Vec<GraphEdgeDto> = Vec::new();
    for edge in &filtered_graph.edges {
        let key = (
            edge.source.clone(),
            edge.target.clone(),
            edge.rel_type.to_lowercase(),
            edge.name.clone(),
        );
        if edge_keys.insert(key) {
            out_edges.push(edge.clone());
        }
    }

    let mut out_nodes: Vec<GraphNodeDto> = node_by_id.into_values().collect();
    out_nodes.sort_by(|a, b| a.id.cmp(&b.id));
    out_edges.sort_by(|a, b| {
        (
            a.source.as_str(),
            a.target.as_str(),
            a.rel_type.to_lowercase(),
            a.name.as_deref().unwrap_or(""),
        )
            .cmp(&(
                b.source.as_str(),
                b.target.as_str(),
                b.rel_type.to_lowercase(),
                b.name.as_deref().unwrap_or(""),
            ))
    });
    SysmlGraphDto {
        nodes: out_nodes,
        edges: out_edges,
    }
}

mod general_view_fold;
mod workspace_dto;
pub(crate) use general_view_fold::*;
pub use workspace_dto::*;

#[cfg(test)]
mod tests;
