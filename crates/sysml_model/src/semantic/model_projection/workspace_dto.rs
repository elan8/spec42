use super::*;

pub fn build_workspace_graph_dto(
    semantic_graph: &SemanticGraph,
    library_paths: &[Url],
) -> SysmlGraphDto {
    let sg_nodes = semantic_graph.workspace_nodes_excluding_libraries(library_paths);
    let nodes: Vec<GraphNodeDto> = sg_nodes
        .iter()
        .filter(|n| n.element_kind != crate::semantic::model::ElementKind::Diagnostic)
        .map(|n| GraphNodeDto {
            id: n.id.qualified_name.clone(),
            element_type: n.element_kind.as_str().to_string(),
            name: n.name.clone(),
            uri: Some(n.id.uri.as_str().to_string()),
            parent_id: n.parent_id.as_ref().map(|p| p.qualified_name.clone()),
            range: range_to_dto(n.range),
            attributes: n.attributes.clone(),
        })
        .collect();

    let mut edges: Vec<GraphEdgeDto> = semantic_graph
        .edges_for_workspace_as_strings(library_paths)
        .into_iter()
        .map(|(src, tgt, kind, name)| GraphEdgeDto {
            source: src,
            target: tgt,
            rel_type: kind.as_str().to_string(),
            name,
        })
        .collect();

    let node_ids: HashSet<String> = nodes.iter().map(|n| n.id.clone()).collect();
    for n in &nodes {
        if let Some(ref pid) = n.parent_id {
            if node_ids.contains(pid) {
                edges.push(GraphEdgeDto {
                    source: pid.clone(),
                    target: n.id.clone(),
                    rel_type: "contains".to_string(),
                    name: None,
                });
            }
        }
    }

    SysmlGraphDto { nodes, edges }
}

pub fn strip_synthetic_nodes(graph: &SysmlGraphDto) -> SysmlGraphDto {
    let synthetic_ids: HashSet<String> = graph
        .nodes
        .iter()
        .filter(|node| {
            node.element_type == "diagnostic"
                || node
                    .attributes
                    .get("synthetic")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
        })
        .map(|n| n.id.clone())
        .collect();
    let concrete_nodes: Vec<GraphNodeDto> = graph
        .nodes
        .iter()
        .filter(|node| !synthetic_ids.contains(&node.id))
        .cloned()
        .collect();
    let concrete_edges: Vec<GraphEdgeDto> = graph
        .edges
        .iter()
        .filter(|edge| {
            !synthetic_ids.contains(&edge.source) && !synthetic_ids.contains(&edge.target)
        })
        .cloned()
        .collect();
    SysmlGraphDto {
        nodes: concrete_nodes,
        edges: concrete_edges,
    }
}

pub fn workspace_visualization_enabled(scope: &[String]) -> bool {
    scope.iter().any(|s| s == "workspaceVisualization")
}
