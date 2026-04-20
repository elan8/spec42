//! sysml/model request parsing and response building.

#[path = "model_params.rs"]
mod model_params;
#[path = "model_projection.rs"]
mod model_projection;

use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{MessageType, Url};
use tower_lsp::Client;

use sysml_v2_parser::RootNamespace;

use crate::common::util;
use crate::semantic_model;
use crate::views::dto::{
    range_to_dto, GraphEdgeDto, GraphNodeDto, RelationshipDto, SysmlElementDto, SysmlGraphDto,
    SysmlModelResultDto, SysmlModelStatsDto, WorkspaceFileModelDto, WorkspaceModelDto,
    WorkspaceModelSummaryDto,
};
use crate::views::extracted_model as model;
use crate::views::ibd;

pub fn parse_sysml_model_params(v: &serde_json::Value) -> Result<(Url, Vec<String>)> {
    model_params::parse_sysml_model_params(v)
}

pub fn empty_model_response(build_start: Instant) -> SysmlModelResultDto {
    model_params::empty_model_response(build_start)
}

fn canonical_general_view_graph(graph: &SysmlGraphDto, include_all_roots: bool) -> SysmlGraphDto {
    model_projection::canonical_general_view_graph(graph, include_all_roots)
}

fn build_workspace_graph_dto(
    semantic_graph: &semantic_model::SemanticGraph,
    library_paths: &[Url],
) -> SysmlGraphDto {
    model_projection::build_workspace_graph_dto(semantic_graph, library_paths)
}

fn strip_synthetic_nodes(graph: &SysmlGraphDto) -> SysmlGraphDto {
    model_projection::strip_synthetic_nodes(graph)
}

fn build_document_graph_dto(
    semantic_graph: &semantic_model::SemanticGraph,
    uri: &Url,
) -> SysmlGraphDto {
    let nodes: Vec<GraphNodeDto> = semantic_graph
        .nodes_for_uri(uri)
        .into_iter()
        .map(|n| GraphNodeDto {
            id: n.id.qualified_name.clone(),
            element_type: n.element_kind.clone(),
            name: n.name.clone(),
            uri: Some(n.id.uri.as_str().to_string()),
            parent_id: n.parent_id.as_ref().map(|p| p.qualified_name.clone()),
            range: range_to_dto(n.range),
            attributes: n.attributes.clone(),
        })
        .collect();

    let mut edges: Vec<GraphEdgeDto> = semantic_graph
        .edges_for_uri_as_strings(uri)
        .into_iter()
        .map(|(src, tgt, kind, name)| GraphEdgeDto {
            source: src,
            target: tgt,
            rel_type: kind.as_str().to_string(),
            name,
        })
        .collect();

    for n in semantic_graph.nodes_for_uri(uri) {
        if let Some(ref pid) = n.parent_id {
            edges.push(GraphEdgeDto {
                source: pid.qualified_name.clone(),
                target: n.id.qualified_name.clone(),
                rel_type: "contains".to_string(),
                name: None,
            });
        }
    }

    SysmlGraphDto { nodes, edges }
}

fn graph_to_element_tree(graph: &SysmlGraphDto, uri: &Url) -> Vec<SysmlElementDto> {
    let contains_targets: std::collections::HashSet<&str> = graph
        .edges
        .iter()
        .filter(|edge| edge.rel_type.eq_ignore_ascii_case("contains"))
        .map(|edge| edge.target.as_str())
        .collect();

    fn build_element(
        node_id: &str,
        uri: &Url,
        nodes_by_id: &std::collections::HashMap<&str, &GraphNodeDto>,
        child_ids_by_parent: &std::collections::HashMap<&str, Vec<&str>>,
        outgoing_relationships: &std::collections::HashMap<&str, Vec<RelationshipDto>>,
    ) -> Option<SysmlElementDto> {
        let node = nodes_by_id.get(node_id)?;
        let children = child_ids_by_parent
            .get(node_id)
            .into_iter()
            .flatten()
            .filter_map(|child_id| {
                build_element(
                    child_id,
                    uri,
                    nodes_by_id,
                    child_ids_by_parent,
                    outgoing_relationships,
                )
            })
            .collect();
        Some(SysmlElementDto {
            id: Some(node.id.clone()),
            element_type: node.element_type.clone(),
            name: node.name.clone(),
            uri: Some(uri.as_str().to_string()),
            range: node.range.clone(),
            children,
            attributes: node.attributes.clone(),
            relationships: outgoing_relationships
                .get(node_id)
                .cloned()
                .unwrap_or_default(),
            errors: None,
        })
    }

    let nodes_by_id: std::collections::HashMap<&str, &GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let mut child_ids_by_parent: std::collections::HashMap<&str, Vec<&str>> =
        std::collections::HashMap::new();
    let mut outgoing_relationships: std::collections::HashMap<&str, Vec<RelationshipDto>> =
        std::collections::HashMap::new();

    for edge in &graph.edges {
        if edge.rel_type.eq_ignore_ascii_case("contains") {
            child_ids_by_parent
                .entry(edge.source.as_str())
                .or_default()
                .push(edge.target.as_str());
            continue;
        }
        outgoing_relationships
            .entry(edge.source.as_str())
            .or_default()
            .push(RelationshipDto {
                rel_type: edge.rel_type.clone(),
                source: edge.source.clone(),
                target: edge.target.clone(),
                name: edge.name.clone(),
            });
    }

    graph
        .nodes
        .iter()
        .filter(|node| !contains_targets.contains(node.id.as_str()))
        .filter_map(|node| {
            build_element(
                node.id.as_str(),
                uri,
                &nodes_by_id,
                &child_ids_by_parent,
                &outgoing_relationships,
            )
        })
        .collect()
}

fn clone_element(element: &SysmlElementDto) -> SysmlElementDto {
    SysmlElementDto {
        id: element.id.clone(),
        element_type: element.element_type.clone(),
        name: element.name.clone(),
        uri: element.uri.clone(),
        range: element.range.clone(),
        children: element.children.iter().map(clone_element).collect(),
        attributes: element.attributes.clone(),
        relationships: element.relationships.clone(),
        errors: element.errors.clone(),
    }
}

fn merge_namespace_elements(elements: &[SysmlElementDto]) -> Vec<SysmlElementDto> {
    let namespace_types = ["package"];
    let mut merged_by_key: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let mut merged: Vec<SysmlElementDto> = Vec::new();

    for element in elements {
        let key = format!("{}::{}", element.element_type, element.name);
        if namespace_types.contains(&element.element_type.as_str()) {
            if let Some(existing_index) = merged_by_key.get(&key).copied() {
                let next = merge_two_elements(&merged[existing_index], element);
                merged[existing_index] = next;
            } else {
                merged_by_key.insert(key, merged.len());
                merged.push(clone_element(element));
            }
        } else {
            merged.push(clone_element(element));
        }
    }

    merged
}

fn merge_two_elements(a: &SysmlElementDto, b: &SysmlElementDto) -> SysmlElementDto {
    let namespace_types = ["package"];
    let mut child_by_key: std::collections::HashMap<String, SysmlElementDto> = a
        .children
        .iter()
        .map(|child| {
            (
                format!("{}::{}", child.element_type, child.name),
                clone_element(child),
            )
        })
        .collect();

    for child in &b.children {
        let key = format!("{}::{}", child.element_type, child.name);
        if namespace_types.contains(&child.element_type.as_str()) {
            if let Some(existing_child) = child_by_key.get(&key).cloned() {
                child_by_key.insert(key, merge_two_elements(&existing_child, child));
            } else {
                child_by_key.insert(key, clone_element(child));
            }
        } else {
            child_by_key
                .entry(key)
                .or_insert_with(|| clone_element(child));
        }
    }

    let mut relationship_keys: std::collections::HashSet<String> = a
        .relationships
        .iter()
        .map(|rel| format!("{}::{}::{}", rel.rel_type, rel.source, rel.target))
        .collect();
    let mut relationships = a.relationships.clone();
    for relationship in &b.relationships {
        let key = format!(
            "{}::{}::{}",
            relationship.rel_type, relationship.source, relationship.target
        );
        if relationship_keys.insert(key) {
            relationships.push(relationship.clone());
        }
    }

    let mut attributes = a.attributes.clone();
    attributes.extend(b.attributes.clone());

    SysmlElementDto {
        id: a.id.clone().or_else(|| b.id.clone()),
        element_type: a.element_type.clone(),
        name: a.name.clone(),
        uri: a.uri.clone().or_else(|| b.uri.clone()),
        range: a.range.clone(),
        children: child_by_key.into_values().collect(),
        attributes,
        relationships,
        errors: a.errors.clone().or_else(|| b.errors.clone()),
    }
}

fn build_workspace_model_dto(
    semantic_graph: &semantic_model::SemanticGraph,
    library_paths: &[Url],
) -> WorkspaceModelDto {
    let workspace_uris = semantic_graph.workspace_uris_excluding_libraries(library_paths);
    let mut files = Vec::with_capacity(workspace_uris.len());
    let mut all_elements = Vec::new();

    for workspace_uri in workspace_uris {
        let graph =
            strip_synthetic_nodes(&build_document_graph_dto(semantic_graph, &workspace_uri));
        let elements = graph_to_element_tree(&graph, &workspace_uri);
        all_elements.extend(elements.iter().map(clone_element));
        files.push(WorkspaceFileModelDto {
            uri: workspace_uri.as_str().to_string(),
            elements,
        });
    }

    files.sort_by(|left, right| left.uri.cmp(&right.uri));

    WorkspaceModelDto {
        summary: WorkspaceModelSummaryDto {
            scanned_files: files.len(),
            loaded_files: files.len(),
            failures: 0,
            truncated: false,
        },
        semantic: merge_namespace_elements(&all_elements),
        files,
    }
}

fn workspace_visualization_enabled(scope: &[String]) -> bool {
    model_projection::workspace_visualization_enabled(scope)
}

fn ibd_requested(scope: &[String]) -> bool {
    scope.is_empty() || scope.iter().any(|s| s == "graph" || s == "ibd")
}

fn elapsed_ms(start: Instant) -> u32 {
    start.elapsed().as_millis().max(1) as u32
}

async fn log_perf(client: &Client, enabled: bool, event: &str, fields: Vec<(&str, String)>) {
    if !enabled {
        return;
    }
    let details = fields
        .into_iter()
        .map(|(key, value)| format!("\"{}\":{}", key, value))
        .collect::<Vec<_>>()
        .join(",");
    client
        .log_message(
            MessageType::INFO,
            format!("[SysML][perf] {{\"event\":\"{}\",{}}}", event, details),
        )
        .await;
}

const TYPING_ATTRIBUTE_KEYS: &[&str] = &[
    "partType",
    "attributeType",
    "portType",
    "actionType",
    "actorType",
    "itemType",
    "occurrenceType",
    "flowType",
    "allocationType",
    "stateType",
    "requirementType",
    "useCaseType",
    "concernType",
    "endType",
    "refType",
    "parameterType",
];

fn node_expects_resolution(node: &semantic_model::SemanticNode) -> bool {
    TYPING_ATTRIBUTE_KEYS
        .iter()
        .any(|key| node.attributes.get(*key).and_then(|v| v.as_str()).is_some())
        || node
            .attributes
            .get("specializes")
            .and_then(|v| v.as_str())
            .is_some()
}

fn count_resolution_stats(semantic_graph: &semantic_model::SemanticGraph, uri: &Url) -> (u32, u32) {
    let mut resolved = 0_u32;
    let mut unresolved = 0_u32;

    for node in semantic_graph.nodes_for_uri(uri) {
        if !node_expects_resolution(node) {
            continue;
        }

        if semantic_graph
            .outgoing_typing_or_specializes_targets(node)
            .is_empty()
        {
            unresolved += 1;
        } else {
            resolved += 1;
        }
    }

    (resolved, unresolved)
}

#[allow(clippy::too_many_arguments)]
pub async fn build_sysml_model_response(
    content: &str,
    parsed: Option<&RootNamespace>,
    parse_time_ms: u32,
    parse_cached: bool,
    semantic_graph: &semantic_model::SemanticGraph,
    uri: &Url,
    library_paths: &[Url],
    scope: &[String],
    build_start: Instant,
    perf_logging_enabled: bool,
    client: &Client,
) -> SysmlModelResultDto {
    let request_phase_start = Instant::now();
    let want_graph = scope.is_empty()
        || scope.iter().any(|s| s == "graph")
        || scope.iter().any(|s| s == "elements")
        || scope.iter().any(|s| s == "relationships");
    let want_general_view_graph =
        scope.is_empty() || scope.iter().any(|s| s == "generalViewGraph") || want_graph;
    let want_stats = scope.is_empty() || scope.iter().any(|s| s == "stats");
    let want_activity_diagrams = scope.is_empty() || scope.iter().any(|s| s == "activityDiagrams");
    let want_ibd = ibd_requested(scope);
    let scope_eval_ms = request_phase_start.elapsed().as_millis().max(1);

    let workspace_viz = workspace_visualization_enabled(scope);
    let graph_start = Instant::now();
    let raw_graph = if want_graph && workspace_viz {
        let graph = build_workspace_graph_dto(semantic_graph, library_paths);
        client
            .log_message(
                MessageType::INFO,
                format!(
                    "sysml/model: workspaceVisualization=true uri={} scope={:?} -> graph nodes={} edges={}",
                    uri.as_str(),
                    scope,
                    graph.nodes.len(),
                    graph.edges.len(),
                ),
            )
            .await;
        Some(graph)
    } else if want_graph {
        let sg_nodes = semantic_graph.nodes_for_uri(uri);
        let node_count = sg_nodes.len();
        let graph_uris = semantic_graph.uris_with_nodes();
        let parsed_ok = parsed.is_some();
        if !parsed_ok {
            let errs = util::parse_failure_diagnostics(content, 5);
            client
                .log_message(
                    MessageType::WARNING,
                    format!(
                        "sysml/model: document in index but parse failed (parsed_ok=false). uri={} parse_errors={}",
                        uri.as_str(),
                        errs.join("; "),
                    ),
                )
                .await;
        }
        client
            .log_message(
                MessageType::INFO,
                format!(
                    "sysml/model: req_uri={} index_ok=true parsed_ok={} semantic_nodes={} graph_uris_count={} graph_uris_sample={:?}",
                    uri.as_str(),
                    parsed_ok,
                    node_count,
                    graph_uris.len(),
                    graph_uris.iter().take(3).collect::<Vec<_>>(),
                ),
            )
            .await;
        Some(build_document_graph_dto(semantic_graph, uri))
    } else {
        None
    };
    let raw_graph_ms = graph_start.elapsed().as_millis().max(1);
    let strip_start = Instant::now();
    let graph = raw_graph.as_ref().map(strip_synthetic_nodes);
    let strip_ms = strip_start.elapsed().as_millis().max(1);
    let general_view_start = Instant::now();
    let general_view_graph = if want_general_view_graph {
        graph
            .as_ref()
            .map(|g| canonical_general_view_graph(g, workspace_viz))
    } else {
        None
    };
    let general_view_ms = general_view_start.elapsed().as_millis().max(1);
    let workspace_model = if workspace_viz {
        Some(build_workspace_model_dto(semantic_graph, library_paths))
    } else {
        None
    };

    let doc = parsed;
    let activity_diagrams_start = Instant::now();
    let activity_diagrams = if want_activity_diagrams {
        Some(
            doc.map(model::extract_activity_diagrams)
                .unwrap_or_default(),
        )
    } else {
        None
    };
    let activity_diagrams_ms = activity_diagrams_start.elapsed().as_millis().max(1);

    let stats_start = Instant::now();
    let stats = if want_stats {
        let total = graph.as_ref().map(|g| g.nodes.len() as u32).unwrap_or(0);
        let (resolved_elements, unresolved_elements) = count_resolution_stats(semantic_graph, uri);
        Some(SysmlModelStatsDto {
            total_elements: total,
            resolved_elements,
            unresolved_elements,
            parse_time_ms,
            model_build_time_ms: elapsed_ms(build_start),
            parse_cached,
        })
    } else {
        None
    };
    let stats_ms = stats_start.elapsed().as_millis().max(1);

    let node_count = graph.as_ref().map(|g| g.nodes.len()).unwrap_or(0);
    let edge_count = graph.as_ref().map(|g| g.edges.len()).unwrap_or(0);
    let gv_node_count = general_view_graph
        .as_ref()
        .map(|g| g.nodes.len())
        .unwrap_or(0);
    let gv_edge_count = general_view_graph
        .as_ref()
        .map(|g| g.edges.len())
        .unwrap_or(0);
    client
        .log_message(
            MessageType::INFO,
            format!(
                "sysml/model: uri={} scope={:?} -> graph nodes={} edges={} generalViewGraph nodes={} edges={}",
                uri.as_str(),
                scope,
                node_count,
                edge_count,
                gv_node_count,
                gv_edge_count,
            ),
        )
        .await;

    let ibd_start = Instant::now();
    let ibd = if want_ibd && want_graph && graph.is_some() && workspace_viz {
        let workspace_uris = semantic_graph.workspace_uris_excluding_libraries(library_paths);
        let ibds = workspace_uris
            .iter()
            .map(|workspace_uri| ibd::build_ibd_for_uri(semantic_graph, workspace_uri))
            .collect();
        Some(ibd::merge_ibd_payloads(ibds))
    } else if want_ibd && want_graph && graph.is_some() {
        Some(ibd::build_ibd_for_uri(semantic_graph, uri))
    } else {
        None
    };
    let ibd_ms = ibd_start.elapsed().as_millis().max(1);
    let total_ms = request_phase_start.elapsed().as_millis().max(1);
    log_perf(
        client,
        perf_logging_enabled,
        "backend:buildSysmlModelResponse",
        vec![
            ("uri", format!("{:?}", uri.as_str())),
            ("scope", format!("{:?}", scope)),
            ("workspaceVisualization", workspace_viz.to_string()),
            ("wantIbd", want_ibd.to_string()),
            ("scopeEvalMs", scope_eval_ms.to_string()),
            ("rawGraphMs", raw_graph_ms.to_string()),
            ("stripSyntheticMs", strip_ms.to_string()),
            ("generalViewMs", general_view_ms.to_string()),
            ("activityDiagramsMs", activity_diagrams_ms.to_string()),
            ("statsMs", stats_ms.to_string()),
            ("ibdMs", ibd_ms.to_string()),
            ("graphNodes", node_count.to_string()),
            ("graphEdges", edge_count.to_string()),
            ("generalViewNodes", gv_node_count.to_string()),
            ("generalViewEdges", gv_edge_count.to_string()),
            ("totalMs", total_ms.to_string()),
        ],
    )
    .await;

    SysmlModelResultDto {
        version: 0,
        graph,
        package_groups: None,
        general_view_graph,
        workspace_model,
        stats,
        activity_diagrams,
        ibd,
    }
}

#[cfg(test)]
mod tests {
    use super::ibd_requested;

    #[test]
    fn ibd_is_only_requested_when_scope_explicitly_includes_it() {
        assert!(ibd_requested(&[]));
        assert!(ibd_requested(&["graph".to_string(), "stats".to_string()]));
        assert!(ibd_requested(&["graph".to_string(), "ibd".to_string()]));
        assert!(ibd_requested(&["ibd".to_string()]));
    }
}
