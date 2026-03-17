//! sysml/model request parsing and response building.

use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{MessageType, Url};
use tower_lsp::Client;

use sysml_parser::RootNamespace;

use crate::dto::{
    range_to_dto, rendered_diagram_to_dto, GraphEdgeDto, GraphNodeDto, RenderedDiagramsDto,
    SysmlGraphDto, SysmlModelResultDto, SysmlModelStatsDto,
};
use crate::ibd;
use crate::model;
use crate::semantic_model;
use crate::util;

fn graph_node_input(node: &GraphNodeDto) -> sysml_diagrams::GraphNodeInput {
    sysml_diagrams::GraphNodeInput {
        id: node.id.clone(),
        element_type: node.element_type.clone(),
        name: node.name.clone(),
        parent_id: node.parent_id.clone(),
        range: sysml_diagrams::RangeInput {
            start_line: node.range.start.line,
            start_character: node.range.start.character,
            end_line: node.range.end.line,
            end_character: node.range.end.character,
        },
        attributes: node
            .attributes
            .iter()
            .map(|(key, value)| {
                (
                    key.clone(),
                    serde_json::to_string(value).unwrap_or_else(|_| String::new()),
                )
            })
            .collect(),
    }
}

fn graph_edge_input(edge: &GraphEdgeDto) -> sysml_diagrams::GraphEdgeInput {
    sysml_diagrams::GraphEdgeInput {
        source: edge.source.clone(),
        target: edge.target.clone(),
        rel_type: edge.rel_type.clone(),
        name: edge.name.clone(),
    }
}

fn ibd_input(ibd: &ibd::IbdDataDto) -> sysml_diagrams::IbdInput {
    sysml_diagrams::IbdInput {
        parts: ibd
            .parts
            .iter()
            .map(|part| sysml_diagrams::IbdPartInput {
                id: part.id.clone(),
                name: part.name.clone(),
                qualified_name: part.qualified_name.clone(),
                container_id: part.container_id.clone(),
                element_type: part.element_type.clone(),
                attributes: part
                    .attributes
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            serde_json::to_string(value).unwrap_or_else(|_| String::new()),
                        )
                    })
                    .collect(),
            })
            .collect(),
        ports: ibd
            .ports
            .iter()
            .map(|port| sysml_diagrams::IbdPortInput {
                id: port.id.clone(),
                name: port.name.clone(),
                parent_id: port.parent_id.clone(),
                direction: port.direction.clone(),
                port_type: port.port_type.clone(),
                port_side: port.port_side.clone(),
            })
            .collect(),
        connectors: ibd
            .connectors
            .iter()
            .map(|connector| sysml_diagrams::IbdConnectorInput {
                source: connector.source.clone(),
                target: connector.target.clone(),
                source_id: connector.source_id.clone(),
                target_id: connector.target_id.clone(),
                rel_type: connector.rel_type.clone(),
            })
            .collect(),
        root_candidates: ibd.root_candidates.clone(),
        default_root: ibd.default_root.clone(),
    }
}

/// Parse sysml/model params from JSON-RPC value. Accepts both object format
/// ({"textDocument":{"uri":"..."},"scope":[...]}) and positional array
/// (for clients that send params as array).
pub fn parse_sysml_model_params(v: &serde_json::Value) -> Result<(Url, Vec<String>)> {
    let (uri_str, scope_value) = if let Some(arr) = v.as_array() {
        let first = arr.get(0).ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params(
                "sysml/model params array must have at least one element",
            )
        })?;
        let uri_str = if let Some(s) = first.as_str() {
            Some(s.to_string())
        } else if let Some(obj) = first.as_object() {
            obj.get("uri")
                .and_then(|u| u.as_str())
                .map(String::from)
                .or_else(|| {
                    obj.get("textDocument")
                        .and_then(|td| td.get("uri"))
                        .and_then(|u| u.as_str())
                        .map(String::from)
                })
        } else {
            None
        };
        let scope_value = arr.get(1);
        (uri_str, scope_value)
    } else if let Some(obj) = v.as_object() {
        let uri_str = obj
            .get("uri")
            .and_then(|u| u.as_str())
            .map(String::from)
            .or_else(|| {
                obj.get("textDocument")
                    .and_then(|td| td.get("uri"))
                    .and_then(|u| u.as_str())
                    .map(String::from)
            });
        let scope_value = obj.get("scope");
        (uri_str, scope_value)
    } else {
        return Err(tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/model params must be an object or array",
        ));
    };

    let uri = uri_str.as_ref().ok_or_else(|| {
        tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/model requires 'uri' or 'textDocument.uri'",
        )
    })?;
    let uri = Url::parse(uri)
        .map_err(|_| tower_lsp::jsonrpc::Error::invalid_params("sysml/model: invalid URI"))?;
    let uri = util::normalize_file_uri(&uri);

    let scope: Vec<String> = scope_value
        .and_then(|s| serde_json::from_value(s.clone()).ok())
        .unwrap_or_default();

    Ok((uri, scope))
}

pub fn empty_model_response(build_start: Instant) -> SysmlModelResultDto {
    SysmlModelResultDto {
        version: 0,
        graph: Some(SysmlGraphDto {
            nodes: vec![],
            edges: vec![],
        }),
        activity_diagrams: None,
        sequence_diagrams: None,
        ibd: None,
        rendered_diagrams: None,
        stats: Some(SysmlModelStatsDto {
            total_elements: 0,
            resolved_elements: 0,
            unresolved_elements: 0,
            parse_time_ms: 0,
            model_build_time_ms: build_start.elapsed().as_millis() as u32,
            parse_cached: true,
        }),
    }
}

/// Build sysml/model response from semantic graph and parsed document.
pub async fn build_sysml_model_response(
    content: &str,
    parsed: Option<&RootNamespace>,
    semantic_graph: &semantic_model::SemanticGraph,
    uri: &Url,
    scope: &[String],
    build_start: Instant,
    client: &Client,
) -> SysmlModelResultDto {
    let want_graph = scope.is_empty()
        || scope.iter().any(|s| s == "graph")
        || scope.iter().any(|s| s == "elements")
        || scope.iter().any(|s| s == "relationships");
    let want_stats = scope.is_empty() || scope.iter().any(|s| s == "stats");
    let want_activity_diagrams = scope.is_empty() || scope.iter().any(|s| s == "activityDiagrams");
    let want_sequence_diagrams = scope.is_empty() || scope.iter().any(|s| s == "sequenceDiagrams");

    let graph = if want_graph {
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
        let nodes: Vec<GraphNodeDto> = sg_nodes
            .into_iter()
            .map(|n| GraphNodeDto {
                id: n.id.qualified_name.clone(),
                element_type: n.element_kind.clone(),
                name: n.name.clone(),
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

        Some(SysmlGraphDto { nodes, edges })
    } else {
        None
    };

    let doc = parsed;

    let activity_diagrams = if want_activity_diagrams {
        Some(
            doc.map(model::extract_activity_diagrams)
                .unwrap_or_default(),
        )
    } else {
        None
    };

    let sequence_diagrams = if want_sequence_diagrams {
        Some(
            doc.map(model::extract_sequence_diagrams)
                .unwrap_or_default(),
        )
    } else {
        None
    };

    let stats = if want_stats {
        let total = graph.as_ref().map(|g| g.nodes.len() as u32).unwrap_or(0);
        Some(SysmlModelStatsDto {
            total_elements: total,
            resolved_elements: 0,
            unresolved_elements: 0,
            parse_time_ms: 0,
            model_build_time_ms: build_start.elapsed().as_millis() as u32,
            parse_cached: true,
        })
    } else {
        None
    };

    let node_count = graph.as_ref().map(|g| g.nodes.len()).unwrap_or(0);
    let edge_count = graph.as_ref().map(|g| g.edges.len()).unwrap_or(0);
    client
        .log_message(
            MessageType::INFO,
            format!(
                "sysml/model: uri={} scope={:?} -> graph nodes={} edges={}",
                uri.as_str(),
                scope,
                node_count,
                edge_count,
            ),
        )
        .await;

    let ibd = if want_graph && graph.is_some() {
        Some(ibd::build_ibd_for_uri(semantic_graph, uri))
    } else {
        None
    };

    let rendered_diagrams = if let Some(graph_dto) = &graph {
        let general = sysml_diagrams::general_view::render(
            &graph_dto
                .nodes
                .iter()
                .map(graph_node_input)
                .collect::<Vec<_>>(),
            &graph_dto
                .edges
                .iter()
                .map(graph_edge_input)
                .collect::<Vec<_>>(),
        )
        .ok()
        .map(rendered_diagram_to_dto);
        let interconnection = ibd
            .as_ref()
            .and_then(|ibd_dto| {
                sysml_diagrams::interconnection_view::render(&ibd_input(ibd_dto)).ok()
            })
            .map(rendered_diagram_to_dto);
        Some(RenderedDiagramsDto {
            general_view: general,
            interconnection_view: interconnection,
        })
    } else {
        None
    };

    SysmlModelResultDto {
        version: 0,
        graph,
        activity_diagrams,
        sequence_diagrams,
        ibd,
        rendered_diagrams,
        stats,
    }
}
