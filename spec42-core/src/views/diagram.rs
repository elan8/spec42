use std::collections::{BTreeMap, HashMap, VecDeque};
use std::time::Instant;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{MessageType, Url};
use tower_lsp::Client;

use sysml_v2_parser::RootNamespace;

use crate::semantic_model;
use crate::views::dto::{
    DiagramBoundsDto, DiagramNodeCompartmentsDto, DiagramPointDto, DiagramSceneDto,
    GeneralDiagramEdgeDto, GeneralDiagramNodeDto, GeneralDiagramSceneDto, GraphEdgeDto,
    GraphNodeDto, IbdDiagramSceneDto, IbdSceneConnectorDto, IbdScenePartDto, IbdScenePortDto,
    IbdSceneRootDto, SysmlDiagramOptionsDto, SysmlDiagramResultDto, SysmlDiagramStatsDto,
    SysmlGraphDto,
};
use crate::views::ibd::{self, IbdConnectorDto, IbdDataDto, IbdPartDto, IbdPortDto};

#[path = "model_projection.rs"]
mod model_projection;

pub fn parse_sysml_diagram_params(
    v: &serde_json::Value,
) -> Result<(Url, String, SysmlDiagramOptionsDto)> {
    let (uri_str, kind, options_value) = if let Some(arr) = v.as_array() {
        let first = arr.first().ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params(
                "sysml/diagram params array must have at least one element",
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
        let kind = arr
            .get(1)
            .and_then(|value| value.as_str())
            .map(String::from)
            .or_else(|| {
                first
                    .as_object()
                    .and_then(|obj| obj.get("kind"))
                    .and_then(|value| value.as_str())
                    .map(String::from)
            });
        let options_value = arr.get(2).cloned().or_else(|| {
            first
                .as_object()
                .and_then(|obj| obj.get("options"))
                .cloned()
        });
        (uri_str, kind, options_value)
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
        let kind = obj
            .get("kind")
            .and_then(|value| value.as_str())
            .map(String::from);
        let options_value = obj.get("options").cloned();
        (uri_str, kind, options_value)
    } else {
        return Err(tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/diagram params must be an object or array",
        ));
    };

    let uri = uri_str.as_ref().ok_or_else(|| {
        tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/diagram requires 'uri' or 'textDocument.uri'",
        )
    })?;
    let kind = kind.ok_or_else(|| {
        tower_lsp::jsonrpc::Error::invalid_params("sysml/diagram requires 'kind'")
    })?;
    let uri = Url::parse(uri)
        .map_err(|_| tower_lsp::jsonrpc::Error::invalid_params("sysml/diagram: invalid URI"))?;
    let options = options_value
        .and_then(|value| serde_json::from_value::<SysmlDiagramOptionsDto>(value).ok())
        .unwrap_or_default();

    Ok((crate::common::util::normalize_file_uri(&uri), kind, options))
}

pub fn empty_diagram_response(
    kind: &str,
    uri: &Url,
    build_start: Instant,
) -> SysmlDiagramResultDto {
    SysmlDiagramResultDto {
        version: 0,
        kind: kind.to_string(),
        source_uri: uri.to_string(),
        scene: DiagramSceneDto {
            general_view: None,
            interconnection_view: None,
        },
        warnings: vec![],
        stats: Some(SysmlDiagramStatsDto {
            node_count: 0,
            edge_count: 0,
            build_time_ms: build_start.elapsed().as_millis() as u32,
        }),
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn build_sysml_diagram_response(
    content: &str,
    parsed: Option<&RootNamespace>,
    semantic_graph: &semantic_model::SemanticGraph,
    uri: &Url,
    library_paths: &[Url],
    kind: &str,
    options: &SysmlDiagramOptionsDto,
    build_start: Instant,
    client: &Client,
) -> SysmlDiagramResultDto {
    match kind {
        "general-view" => {
            let graph = build_general_source_graph(
                content,
                parsed,
                semantic_graph,
                uri,
                library_paths,
                options,
                client,
            )
            .await;
            let scene = build_general_scene(&graph);
            SysmlDiagramResultDto {
                version: 0,
                kind: kind.to_string(),
                source_uri: uri.to_string(),
                stats: Some(SysmlDiagramStatsDto {
                    node_count: scene.nodes.len() as u32,
                    edge_count: scene.edges.len() as u32,
                    build_time_ms: build_start.elapsed().as_millis() as u32,
                }),
                scene: DiagramSceneDto {
                    general_view: Some(scene),
                    interconnection_view: None,
                },
                warnings: vec![],
            }
        }
        "interconnection-view" => {
            let ibd = if options.workspace_visualization.unwrap_or(false) {
                let workspace_uris =
                    semantic_graph.workspace_uris_excluding_libraries(library_paths);
                let ibds = workspace_uris
                    .iter()
                    .map(|workspace_uri| ibd::build_ibd_for_uri(semantic_graph, workspace_uri))
                    .collect();
                ibd::merge_ibd_payloads(ibds)
            } else {
                ibd::build_ibd_for_uri(semantic_graph, uri)
            };
            let scene = build_ibd_scene(&ibd, options.root.as_deref());
            let edge_count: usize = scene.roots.values().map(|root| root.connectors.len()).sum();
            let node_count: usize = scene.roots.values().map(|root| root.parts.len()).sum();
            SysmlDiagramResultDto {
                version: 0,
                kind: kind.to_string(),
                source_uri: uri.to_string(),
                stats: Some(SysmlDiagramStatsDto {
                    node_count: node_count as u32,
                    edge_count: edge_count as u32,
                    build_time_ms: build_start.elapsed().as_millis() as u32,
                }),
                scene: DiagramSceneDto {
                    general_view: None,
                    interconnection_view: Some(scene),
                },
                warnings: vec![],
            }
        }
        _ => SysmlDiagramResultDto {
            version: 0,
            kind: kind.to_string(),
            source_uri: uri.to_string(),
            scene: DiagramSceneDto {
                general_view: None,
                interconnection_view: None,
            },
            warnings: vec![format!("Unsupported diagram kind: {kind}")],
            stats: Some(SysmlDiagramStatsDto {
                node_count: 0,
                edge_count: 0,
                build_time_ms: build_start.elapsed().as_millis() as u32,
            }),
        },
    }
}

async fn build_general_source_graph(
    content: &str,
    parsed: Option<&RootNamespace>,
    semantic_graph: &semantic_model::SemanticGraph,
    uri: &Url,
    library_paths: &[Url],
    options: &SysmlDiagramOptionsDto,
    client: &Client,
) -> SysmlGraphDto {
    let workspace_viz = options.workspace_visualization.unwrap_or(false);
    let raw_graph = if workspace_viz {
        model_projection::build_workspace_graph_dto(semantic_graph, library_paths)
    } else {
        let parsed_ok = parsed.is_some();
        if !parsed_ok {
            let errs = crate::common::util::parse_failure_diagnostics(content, 5);
            client
                .log_message(
                    MessageType::WARNING,
                    format!(
                        "sysml/diagram: document in index but parse failed (parsed_ok=false). uri={} parse_errors={}",
                        uri.as_str(),
                        errs.join("; "),
                    ),
                )
                .await;
        }
        let nodes: Vec<GraphNodeDto> = semantic_graph
            .nodes_for_uri(uri)
            .into_iter()
            .map(|n| GraphNodeDto {
                id: n.id.qualified_name.clone(),
                element_type: n.element_kind.clone(),
                name: n.name.clone(),
                parent_id: n.parent_id.as_ref().map(|p| p.qualified_name.clone()),
                range: crate::views::dto::range_to_dto(n.range),
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
    };
    let stripped = model_projection::strip_synthetic_nodes(&raw_graph);
    let canonical = model_projection::canonical_general_view_graph(&stripped, workspace_viz);
    canonical
}

fn build_general_scene(graph: &SysmlGraphDto) -> GeneralDiagramSceneDto {
    let node_by_id: HashMap<String, &GraphNodeDto> =
        graph.nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let contains_children = contains_children(graph);
    let typing_targets = typing_targets(graph);
    let category_order = [
        "system",
        "structure",
        "requirements",
        "behavior",
        "interfaces",
        "analysis",
        "other",
    ];
    let category_rank: HashMap<&str, usize> = category_order
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect();
    let mut hierarchy_adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let mut indegree: HashMap<String, usize> =
        graph.nodes.iter().map(|n| (n.id.clone(), 0)).collect();
    for edge in &graph.edges {
        let rel = edge.rel_type.to_lowercase();
        if matches!(rel.as_str(), "contains" | "typing" | "specializes") {
            hierarchy_adjacency
                .entry(edge.source.clone())
                .or_default()
                .push(edge.target.clone());
            *indegree.entry(edge.target.clone()).or_default() += 1;
        }
    }
    let mut roots: Vec<String> = indegree
        .iter()
        .filter_map(|(id, deg)| (*deg == 0).then_some(id.clone()))
        .collect();
    if roots.is_empty() {
        roots = graph.nodes.iter().map(|n| n.id.clone()).collect();
    }
    roots.sort();

    let mut depth_by_id: HashMap<String, usize> = HashMap::new();
    let mut queue: VecDeque<String> = roots.iter().cloned().collect();
    for root in &roots {
        depth_by_id.insert(root.clone(), 0);
    }
    while let Some(node_id) = queue.pop_front() {
        let depth = *depth_by_id.get(&node_id).unwrap_or(&0);
        for child in hierarchy_adjacency
            .get(&node_id)
            .cloned()
            .unwrap_or_default()
        {
            let next_depth = depth + 1;
            if !depth_by_id.contains_key(&child) {
                depth_by_id.insert(child.clone(), next_depth);
                queue.push_back(child);
            }
        }
    }
    for node in &graph.nodes {
        depth_by_id.entry(node.id.clone()).or_insert(roots.len());
    }

    let mut lanes: BTreeMap<usize, Vec<&GraphNodeDto>> = BTreeMap::new();
    for node in &graph.nodes {
        lanes
            .entry(*depth_by_id.get(&node.id).unwrap_or(&0))
            .or_default()
            .push(node);
    }

    let node_width = 220.0_f32;
    let horizontal_gap = 80.0_f32;
    let vertical_gap = 120.0_f32;
    let start_x = 80.0_f32;
    let start_y = 80.0_f32;
    let mut layout_by_id: HashMap<String, (f32, f32, f32, f32)> = HashMap::new();
    for (depth, lane_nodes) in &mut lanes {
        lane_nodes.sort_by(|a, b| {
            let a_cat = general_category(&a.element_type);
            let b_cat = general_category(&b.element_type);
            category_rank
                .get(a_cat.as_str())
                .unwrap_or(&999)
                .cmp(category_rank.get(b_cat.as_str()).unwrap_or(&999))
                .then(a.name.cmp(&b.name))
        });
        let mut cursor_x = start_x;
        let y = start_y + (*depth as f32) * vertical_gap;
        for node in lane_nodes.iter() {
            let compartments =
                build_general_compartments(node, &node_by_id, &contains_children, &typing_targets);
            let line_count = compartments.attributes.len()
                + compartments.parts.len()
                + compartments.ports.len()
                + compartments.other.len()
                + usize::from(compartments.typed_by_name.is_some());
            let height = (86.0_f32 + (line_count.min(10) as f32) * 12.0_f32).max(90.0_f32);
            layout_by_id.insert(node.id.clone(), (cursor_x, y, node_width, height));
            cursor_x += node_width + horizontal_gap;
        }
    }

    let mut scene_nodes: Vec<GeneralDiagramNodeDto> = graph
        .nodes
        .iter()
        .filter_map(|node| {
            let (x, y, width, height) = layout_by_id.get(&node.id).copied()?;
            let compartments =
                build_general_compartments(node, &node_by_id, &contains_children, &typing_targets);
            Some(GeneralDiagramNodeDto {
                id: node.id.clone(),
                name: node.name.clone(),
                qualified_name: node.id.clone(),
                element_type: node.element_type.clone(),
                x,
                y,
                width,
                height,
                category: general_category(&node.element_type),
                is_definition: node.element_type.to_lowercase().contains("def"),
                compartments,
            })
        })
        .collect();
    scene_nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut scene_edges: Vec<GeneralDiagramEdgeDto> = graph
        .edges
        .iter()
        .enumerate()
        .filter_map(|(idx, edge)| {
            let src = layout_by_id.get(&edge.source)?;
            let tgt = layout_by_id.get(&edge.target)?;
            let label = edge.name.clone().or_else(|| {
                (!matches!(
                    edge.rel_type.as_str(),
                    "contains" | "typing" | "specializes"
                ))
                .then_some(edge.rel_type.clone())
            });
            Some(GeneralDiagramEdgeDto {
                id: format!("general-edge-{idx}"),
                source: edge.source.clone(),
                target: edge.target.clone(),
                rel_type: edge.rel_type.clone(),
                label,
                points: orthogonal_points(*src, *tgt),
            })
        })
        .collect();
    scene_edges.sort_by(|a, b| a.id.cmp(&b.id));

    let bounds = compute_bounds(layout_by_id.values().copied().collect());
    GeneralDiagramSceneDto {
        nodes: scene_nodes,
        edges: scene_edges,
        bounds,
    }
}

fn build_ibd_scene(ibd: &IbdDataDto, selected_root: Option<&str>) -> IbdDiagramSceneDto {
    let top_level_by_name: HashMap<String, &IbdPartDto> = ibd
        .parts
        .iter()
        .filter(|part| part.container_id.is_none())
        .map(|part| (part.name.clone(), part))
        .collect();
    let mut root_candidates = ibd.root_candidates.clone();
    if root_candidates.is_empty() {
        let mut synthesized: Vec<String> = top_level_by_name.keys().cloned().collect();
        synthesized.sort();
        root_candidates = synthesized;
    }
    for root_name in ibd.root_views.keys() {
        if !root_candidates
            .iter()
            .any(|candidate| candidate == root_name)
        {
            root_candidates.push(root_name.clone());
        }
    }

    let mut roots = HashMap::new();
    for root_name in &root_candidates {
        if let Some(view) = ibd.root_views.get(root_name) {
            roots.insert(
                root_name.clone(),
                build_ibd_root_scene(root_name, &view.parts, &view.ports, &view.connectors),
            );
            continue;
        }

        let Some(root_part) = top_level_by_name.get(root_name) else {
            continue;
        };
        let root_prefix = root_part.qualified_name.as_str();
        let root_parts: Vec<IbdPartDto> = ibd
            .parts
            .iter()
            .filter(|part| {
                part.qualified_name == root_prefix
                    || part.qualified_name.starts_with(&format!("{root_prefix}."))
            })
            .cloned()
            .collect();
        let root_ports: Vec<IbdPortDto> = ibd
            .ports
            .iter()
            .filter(|port| {
                port.parent_id == root_prefix
                    || port.parent_id.starts_with(&format!("{root_prefix}."))
            })
            .cloned()
            .collect();
        let root_connectors: Vec<IbdConnectorDto> = ibd
            .connectors
            .iter()
            .filter(|connector| {
                let source_matches = connector.source_id == root_prefix
                    || connector.source_id.starts_with(&format!("{root_prefix}."));
                let target_matches = connector.target_id == root_prefix
                    || connector.target_id.starts_with(&format!("{root_prefix}."));
                source_matches || target_matches
            })
            .cloned()
            .collect();
        roots.insert(
            root_name.clone(),
            build_ibd_root_scene(root_name, &root_parts, &root_ports, &root_connectors),
        );
    }
    let default_root = ibd
        .default_root
        .clone()
        .filter(|root| roots.contains_key(root));
    let selected_root = selected_root
        .map(|root| root.to_string())
        .filter(|root| roots.contains_key(root))
        .or_else(|| default_root.clone())
        .or_else(|| {
            root_candidates
                .iter()
                .find(|root| roots.contains_key(*root))
                .cloned()
        });
    IbdDiagramSceneDto {
        root_candidates,
        default_root,
        selected_root,
        roots,
    }
}

fn build_ibd_root_scene(
    root_name: &str,
    parts: &[IbdPartDto],
    ports: &[IbdPortDto],
    connectors: &[IbdConnectorDto],
) -> IbdSceneRootDto {
    let part_by_qn: HashMap<String, &IbdPartDto> = parts
        .iter()
        .map(|part| (part.qualified_name.clone(), part))
        .collect();
    let mut children_by_container: HashMap<String, Vec<&IbdPartDto>> = HashMap::new();
    let mut root_parts: Vec<&IbdPartDto> = Vec::new();
    for part in parts {
        if let Some(container_id) = &part.container_id {
            children_by_container
                .entry(container_id.clone())
                .or_default()
                .push(part);
        } else {
            root_parts.push(part);
        }
    }
    root_parts.sort_by(|a, b| a.name.cmp(&b.name));

    let mut layout_by_qn: HashMap<String, (f32, f32, f32, f32, bool, u32)> = HashMap::new();
    let mut cursor_y = 100.0_f32;
    for root_part in &root_parts {
        let size = layout_ibd_part_tree(
            root_part,
            &children_by_container,
            ports,
            &mut layout_by_qn,
            100.0,
            cursor_y,
            0,
        );
        cursor_y += size.1 + 80.0;
    }

    let scene_parts: Vec<IbdScenePartDto> = parts
        .iter()
        .filter_map(|part| {
            let (x, y, width, height, is_container, depth) =
                layout_by_qn.get(&part.qualified_name).copied()?;
            Some(IbdScenePartDto {
                id: part.id.clone(),
                name: part.name.clone(),
                qualified_name: part.qualified_name.clone(),
                container_id: part.container_id.clone(),
                element_type: part.element_type.clone(),
                x,
                y,
                width,
                height,
                is_container,
                depth,
                attributes: part.attributes.clone(),
            })
        })
        .collect();

    let scene_ports: Vec<IbdScenePortDto> = ports
        .iter()
        .filter_map(|port| {
            let parent = if let Some(parent) = part_by_qn.get(&port.parent_id) {
                *parent
            } else {
                parts
                    .iter()
                    .find(|part| part.name == port.parent_id || part.id == port.parent_id)?
            };
            let (px, py) = ibd_port_anchor(parent, port, &layout_by_qn, ports);
            Some(IbdScenePortDto {
                id: port.id.clone(),
                name: port.name.clone(),
                parent_id: port.parent_id.clone(),
                x: px,
                y: py,
                direction: port.direction.clone(),
                port_type: port.port_type.clone(),
                port_side: port.port_side.clone(),
            })
        })
        .collect();

    let scene_connectors: Vec<IbdSceneConnectorDto> = connectors
        .iter()
        .enumerate()
        .map(|(idx, connector)| {
            let source_anchor =
                ibd_endpoint_anchor(connector.source_id.as_str(), parts, ports, &layout_by_qn);
            let target_anchor =
                ibd_endpoint_anchor(connector.target_id.as_str(), parts, ports, &layout_by_qn);
            IbdSceneConnectorDto {
                id: format!("ibd-edge-{idx}"),
                source: connector.source.clone(),
                target: connector.target.clone(),
                source_id: connector.source_id.clone(),
                target_id: connector.target_id.clone(),
                rel_type: connector.rel_type.clone(),
                points: orthogonal_connector_points(source_anchor, target_anchor),
            }
        })
        .collect();

    let bounds = compute_bounds(
        layout_by_qn
            .values()
            .map(|(x, y, width, height, _, _)| (*x, *y, *width, *height))
            .collect(),
    );
    IbdSceneRootDto {
        name: root_name.to_string(),
        parts: scene_parts,
        ports: scene_ports,
        connectors: scene_connectors,
        bounds,
    }
}

fn layout_ibd_part_tree(
    part: &IbdPartDto,
    children_by_container: &HashMap<String, Vec<&IbdPartDto>>,
    ports: &[IbdPortDto],
    layout_by_qn: &mut HashMap<String, (f32, f32, f32, f32, bool, u32)>,
    x: f32,
    y: f32,
    depth: u32,
) -> (f32, f32) {
    let children = children_by_container
        .get(&part.qualified_name)
        .cloned()
        .unwrap_or_default();
    let port_count = ports
        .iter()
        .filter(|port| {
            port.parent_id == part.qualified_name
                || port.parent_id == part.name
                || port.parent_id == part.id
        })
        .count();
    let width = (220.0_f32 + (port_count.min(4) as f32) * 20.0_f32).min(360.0_f32);
    let mut height = 90.0_f32 + (port_count as f32) * 22.0_f32;
    let is_container = !children.is_empty();
    if is_container {
        let mut child_y = y + 80.0_f32;
        let mut max_child_width = 0.0_f32;
        for child in children {
            let (child_width, child_height) = layout_ibd_part_tree(
                child,
                children_by_container,
                ports,
                layout_by_qn,
                x + 40.0,
                child_y,
                depth + 1,
            );
            max_child_width = max_child_width.max(child_width);
            child_y += child_height + 36.0;
        }
        height = (child_y - y + 20.0).max(height);
        let adjusted_width = width.max(max_child_width + 80.0);
        layout_by_qn.insert(
            part.qualified_name.clone(),
            (x, y, adjusted_width, height, true, depth),
        );
        (adjusted_width, height)
    } else {
        layout_by_qn.insert(
            part.qualified_name.clone(),
            (x, y, width, height, false, depth),
        );
        (width, height)
    }
}

fn contains_children(graph: &SysmlGraphDto) -> HashMap<String, Vec<String>> {
    let mut contains_children = HashMap::new();
    for edge in &graph.edges {
        if edge.rel_type.eq_ignore_ascii_case("contains") {
            contains_children
                .entry(edge.source.clone())
                .or_insert_with(Vec::new)
                .push(edge.target.clone());
        }
    }
    contains_children
}

fn typing_targets(graph: &SysmlGraphDto) -> HashMap<String, String> {
    let mut targets = HashMap::new();
    for edge in &graph.edges {
        if edge.rel_type.eq_ignore_ascii_case("typing") {
            targets.insert(edge.source.clone(), edge.target.clone());
        }
    }
    targets
}

fn build_general_compartments(
    node: &GraphNodeDto,
    node_by_id: &HashMap<String, &GraphNodeDto>,
    contains_children: &HashMap<String, Vec<String>>,
    typing_targets: &HashMap<String, String>,
) -> DiagramNodeCompartmentsDto {
    let mut attributes = Vec::new();
    let mut parts = Vec::new();
    let mut ports = Vec::new();
    let mut other = Vec::new();
    if let Some(children) = contains_children.get(&node.id) {
        for child_id in children {
            if let Some(child) = node_by_id.get(child_id) {
                let entry = if let Some(typed_id) = typing_targets.get(child_id) {
                    let typed_name = node_by_id
                        .get(typed_id)
                        .map(|typed| typed.name.as_str())
                        .unwrap_or(typed_id.as_str());
                    format!("{} : {}", child.name, typed_name)
                } else {
                    child.name.clone()
                };
                let type_lower = child.element_type.to_lowercase();
                if type_lower.contains("port") {
                    ports.push(entry);
                } else if type_lower.contains("part") {
                    parts.push(entry);
                } else if type_lower.contains("attribute") || type_lower.contains("property") {
                    attributes.push(entry);
                } else {
                    other.push(entry);
                }
            }
        }
    }

    DiagramNodeCompartmentsDto {
        stereotype: node.element_type.to_lowercase(),
        name: node.name.clone(),
        typed_by_name: typing_targets
            .get(&node.id)
            .and_then(|typed_id| node_by_id.get(typed_id).map(|typed| typed.name.clone())),
        attributes,
        parts,
        ports,
        other,
    }
}

fn general_category(element_type: &str) -> String {
    let lower = element_type.to_lowercase();
    if ["package", "namespace"].iter().any(|kw| lower.contains(kw)) {
        return "packages".to_string();
    }
    if ["requirement", "constraint"]
        .iter()
        .any(|kw| lower.contains(kw))
    {
        return "requirements".to_string();
    }
    if ["action", "state", "use case", "usecase", "mission"]
        .iter()
        .any(|kw| lower.contains(kw))
    {
        return "behavior".to_string();
    }
    if ["port", "interface", "connection"]
        .iter()
        .any(|kw| lower.contains(kw))
    {
        return "interfaces".to_string();
    }
    if ["analysis", "trade", "measure"]
        .iter()
        .any(|kw| lower.contains(kw))
    {
        return "analysis".to_string();
    }
    if ["system", "part def", "part usage", "part"]
        .iter()
        .any(|kw| lower.contains(kw))
    {
        return "structure".to_string();
    }
    if ["feature decl", "classifier decl"]
        .iter()
        .any(|kw| lower.contains(kw))
    {
        return "structure".to_string();
    }
    "other".to_string()
}

fn orthogonal_points(src: (f32, f32, f32, f32), tgt: (f32, f32, f32, f32)) -> Vec<DiagramPointDto> {
    let (sx, sy, sw, sh) = src;
    let (tx, ty, tw, _th) = tgt;
    let start = DiagramPointDto {
        x: sx + sw / 2.0,
        y: sy + sh,
    };
    let end = DiagramPointDto {
        x: tx + tw / 2.0,
        y: ty,
    };
    let mid_y = (start.y + end.y) / 2.0;
    vec![
        start.clone(),
        DiagramPointDto {
            x: start.x,
            y: mid_y,
        },
        DiagramPointDto { x: end.x, y: mid_y },
        end,
    ]
}

fn orthogonal_connector_points(src: (f32, f32), tgt: (f32, f32)) -> Vec<DiagramPointDto> {
    let mid_x = (src.0 + tgt.0) / 2.0;
    vec![
        DiagramPointDto { x: src.0, y: src.1 },
        DiagramPointDto { x: mid_x, y: src.1 },
        DiagramPointDto { x: mid_x, y: tgt.1 },
        DiagramPointDto { x: tgt.0, y: tgt.1 },
    ]
}

fn ibd_endpoint_anchor(
    endpoint_id: &str,
    parts: &[IbdPartDto],
    ports: &[IbdPortDto],
    layout_by_qn: &HashMap<String, (f32, f32, f32, f32, bool, u32)>,
) -> (f32, f32) {
    if let Some(port) = ports.iter().find(|port| {
        endpoint_id == port.id
            || endpoint_id == port.name
            || endpoint_id.ends_with(&format!(".{}", port.name))
            || endpoint_id.ends_with(&format!("::{}", port.name))
    }) {
        let parent = parts.iter().find(|part| {
            part.qualified_name == port.parent_id
                || part.name == port.parent_id
                || part.id == port.parent_id
        });
        if let Some(parent) = parent {
            return ibd_port_anchor(parent, port, layout_by_qn, ports);
        }
    }

    if let Some(part) = parts.iter().find(|part| {
        endpoint_id == part.id || endpoint_id == part.name || endpoint_id == part.qualified_name
    }) {
        if let Some((x, y, width, height, _, _)) = layout_by_qn.get(&part.qualified_name) {
            return (x + width / 2.0, y + height / 2.0);
        }
    }

    (0.0, 0.0)
}

fn ibd_port_anchor(
    part: &IbdPartDto,
    port: &IbdPortDto,
    layout_by_qn: &HashMap<String, (f32, f32, f32, f32, bool, u32)>,
    ports: &[IbdPortDto],
) -> (f32, f32) {
    let (x, y, width, height, _, _) = layout_by_qn
        .get(&part.qualified_name)
        .copied()
        .unwrap_or((0.0, 0.0, 220.0, 90.0, false, 0));
    let same_parent_ports: Vec<&IbdPortDto> = ports
        .iter()
        .filter(|candidate| {
            candidate.parent_id == port.parent_id || candidate.parent_id == part.qualified_name
        })
        .collect();
    let left_side = matches!(port.port_side.as_deref(), Some("left" | "west"))
        || matches!(port.direction.as_deref(), Some("in" | "input"));
    let index = same_parent_ports
        .iter()
        .position(|candidate| candidate.id == port.id)
        .unwrap_or(0);
    let denom = (same_parent_ports.len().max(1) + 1) as f32;
    let py = y + ((index as f32 + 1.0) * (height / denom));
    let px = if left_side { x } else { x + width };
    (px, py)
}

fn compute_bounds(rects: Vec<(f32, f32, f32, f32)>) -> DiagramBoundsDto {
    if rects.is_empty() {
        return DiagramBoundsDto {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        };
    }
    let min_x = rects
        .iter()
        .map(|(x, _, _, _)| *x)
        .fold(f32::INFINITY, f32::min);
    let min_y = rects
        .iter()
        .map(|(_, y, _, _)| *y)
        .fold(f32::INFINITY, f32::min);
    let max_x = rects
        .iter()
        .map(|(x, _, width, _)| x + width)
        .fold(f32::NEG_INFINITY, f32::max);
    let max_y = rects
        .iter()
        .map(|(_, y, _, height)| y + height)
        .fold(f32::NEG_INFINITY, f32::max);
    DiagramBoundsDto {
        x: min_x,
        y: min_y,
        width: max_x - min_x,
        height: max_y - min_y,
    }
}

#[cfg(test)]
mod tests {
    use super::{build_ibd_scene, parse_sysml_diagram_params};
    use crate::views::ibd::{IbdConnectorDto, IbdDataDto, IbdPartDto, IbdPortDto, IbdRootViewDto};

    #[test]
    fn build_ibd_scene_synthesizes_roots_from_flat_ibd_data() {
        let ibd = IbdDataDto {
            parts: vec![
                IbdPartDto {
                    id: "Demo::droneInstance".to_string(),
                    name: "droneInstance".to_string(),
                    qualified_name: "Demo.droneInstance".to_string(),
                    container_id: None,
                    element_type: "part".to_string(),
                    attributes: std::collections::HashMap::new(),
                },
                IbdPartDto {
                    id: "Demo::droneInstance::camera".to_string(),
                    name: "camera".to_string(),
                    qualified_name: "Demo.droneInstance.camera".to_string(),
                    container_id: Some("Demo.droneInstance".to_string()),
                    element_type: "part".to_string(),
                    attributes: std::collections::HashMap::new(),
                },
            ],
            ports: vec![IbdPortDto {
                id: "Demo::droneInstance::camera::videoOut".to_string(),
                name: "videoOut".to_string(),
                parent_id: "Demo.droneInstance.camera".to_string(),
                direction: Some("out".to_string()),
                port_type: None,
                port_side: Some("right".to_string()),
            }],
            connectors: vec![IbdConnectorDto {
                source: "Demo::droneInstance::camera::videoOut".to_string(),
                target: "Demo::droneInstance::camera::videoOut".to_string(),
                source_id: "Demo.droneInstance.camera.videoOut".to_string(),
                target_id: "Demo.droneInstance.camera.videoOut".to_string(),
                rel_type: "connection".to_string(),
            }],
            root_candidates: vec![],
            default_root: None,
            root_views: std::collections::HashMap::new(),
        };

        let scene = build_ibd_scene(&ibd, None);
        assert_eq!(scene.selected_root.as_deref(), Some("droneInstance"));
        assert_eq!(scene.root_candidates, vec!["droneInstance".to_string()]);
        let root = scene
            .roots
            .get("droneInstance")
            .expect("synthesized root scene");
        assert_eq!(root.parts.len(), 2);
        assert_eq!(root.ports.len(), 1);
        assert_eq!(root.connectors.len(), 1);
    }

    #[test]
    fn build_ibd_scene_uses_root_view_when_available() {
        let ibd = IbdDataDto {
            parts: vec![],
            ports: vec![],
            connectors: vec![],
            root_candidates: vec!["RootA".to_string()],
            default_root: Some("RootA".to_string()),
            root_views: std::collections::HashMap::from([(
                "RootA".to_string(),
                IbdRootViewDto {
                    parts: vec![IbdPartDto {
                        id: "Demo::RootA".to_string(),
                        name: "RootA".to_string(),
                        qualified_name: "Demo.RootA".to_string(),
                        container_id: None,
                        element_type: "part def".to_string(),
                        attributes: std::collections::HashMap::new(),
                    }],
                    ports: vec![],
                    connectors: vec![],
                },
            )]),
        };

        let scene = build_ibd_scene(&ibd, None);
        assert_eq!(scene.selected_root.as_deref(), Some("RootA"));
        assert_eq!(
            scene
                .roots
                .get("RootA")
                .expect("root scene from view")
                .parts
                .len(),
            1
        );
    }

    #[test]
    fn parse_sysml_diagram_params_accepts_top_level_uri_shape() {
        let params = serde_json::json!({
            "uri": "file:///C:/demo.sysml",
            "kind": "interconnection-view",
            "options": {
                "workspaceVisualization": true
            }
        });

        let (uri, kind, options) =
            parse_sysml_diagram_params(&params).expect("parse diagram params");
        assert_eq!(uri.as_str(), "file:///c:/demo.sysml");
        assert_eq!(kind, "interconnection-view");
        assert_eq!(options.workspace_visualization, Some(true));
    }
}
