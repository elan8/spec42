//! sysml/model request parsing and response building.
//! Diagram output is collected from [crate::config::DiagramProvider] implementations.

use std::sync::Arc;
use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{MessageType, Url};
use tower_lsp::Client;

use sysml_parser::RootNamespace;

use crate::config::{DiagramContext, DiagramProvider};
use crate::dto::{
    range_to_dto, rendered_diagram_to_dto, GraphEdgeDto, GraphNodeDto, RenderedDiagramsDto,
    SysmlGraphDto, SysmlModelResultDto, SysmlModelStatsDto,
};
use crate::ibd;
use crate::model;
use crate::semantic_model;
use crate::util;

/// Parse sysml/model params from JSON-RPC value.
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

/// Build sysml/model response. Uses `diagram_providers` to fill `rendered_diagrams` (generalView, interconnectionView by diagram_id).
pub async fn build_sysml_model_response(
    content: &str,
    parsed: Option<&RootNamespace>,
    semantic_graph: &semantic_model::SemanticGraph,
    uri: &Url,
    scope: &[String],
    build_start: Instant,
    client: &Client,
    diagram_providers: &[Arc<dyn DiagramProvider>],
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

    let rendered_diagrams = if graph.is_some() || ibd.is_some() {
        let context = DiagramContext {
            graph: graph.as_ref(),
            ibd: ibd.as_ref(),
            uri,
        };
        let mut diagrams = std::collections::HashMap::new();
        for provider in diagram_providers {
            if let Some(diagram) = provider.render(&context) {
                let dto = rendered_diagram_to_dto(diagram);
                diagrams.insert(provider.diagram_id().to_string(), dto);
            }
        }
        Some(RenderedDiagramsDto(diagrams))
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
