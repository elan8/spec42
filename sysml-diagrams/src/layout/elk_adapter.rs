use std::collections::{HashMap, HashSet};

use elk_core::{
    EdgeEndpoint, ElementLayoutOptions, Graph as ElkGraph, LayerConstraint,
    LayoutDirection as ElkLayoutDirection, LayoutEngine, LayoutOptions, NodeAlignment,
    Padding as ElkPadding, Point as ElkPoint, PortConstraint, PortSide as ElkPortSide,
    Rect as ElkRect, Size as ElkSize, Spacing as ElkSpacing, ViewProfile as ElkViewProfile,
};
use elk_layered::LayeredLayoutEngine;

use crate::layout::{
    Bounds, DiagramEdge, DiagramGraph, DiagramLayout, DiagramNode, DiagramPort, EdgeLayout,
    LayerDirection, LayoutConfig, LayoutViewProfile, NodeLayout, Point, PortLayout, PortSide,
    Result,
};

pub(crate) struct LayoutComputation {
    pub layout: DiagramLayout,
    pub report: elk_core::LayoutReport,
    pub warnings: Vec<String>,
}

pub(crate) fn compute_layout(
    graph: &DiagramGraph,
    config: &LayoutConfig,
) -> Result<LayoutComputation> {
    let mut elk_graph = ElkGraph::new();
    let ordered_nodes = sort_nodes_for_hierarchy(&graph.nodes)?;
    let mut node_ids = HashMap::new();
    let mut port_ids = HashMap::new();

    for (index, node) in ordered_nodes.iter().enumerate() {
        let elk_node_id = add_node(&mut elk_graph, node, &node_ids)?;
        apply_node_hints(&mut elk_graph, elk_node_id, node, index, config);
        node_ids.insert(node.id.clone(), elk_node_id);
    }

    for node in &ordered_nodes {
        let elk_node_id = *node_ids
            .get(node.id.as_str())
            .ok_or_else(|| crate::layout::DiagramError::MissingNode(node.id.clone()))?;
        for (index, port) in node.ports.iter().enumerate() {
            let elk_port_id = elk_graph.add_port(
                elk_node_id,
                map_port_side(&port.side),
                ElkSize::new(8.0, 8.0),
            );
            elk_graph.port_mut(elk_port_id).layout.model_order = Some(index);
            port_ids.insert(port.id.clone(), elk_port_id);
            let normalized = normalize_port_id(&port.id);
            if normalized != port.id {
                port_ids.insert(normalized, elk_port_id);
            }
        }
    }

    for edge in &graph.edges {
        let source = map_endpoint(
            edge.source_node.as_str(),
            edge.source_port.as_deref(),
            &node_ids,
            &port_ids,
        )?;
        let target = map_endpoint(
            edge.target_node.as_str(),
            edge.target_port.as_deref(),
            &node_ids,
            &port_ids,
        )?;
        let edge_id = elk_graph.add_edge(source, target);
        let model_order = elk_graph.edge(edge_id).id.index();
        let layout = &mut elk_graph.edge_mut(edge_id).layout;
        layout.model_order = Some(model_order);
        layout.edge_bundle_key = Some(edge_bundle_key(&edge.kind));
    }

    apply_graph_hints(&mut elk_graph, graph, &node_ids, config);

    let mut options = map_layout_options(config);

    // For Interconnection View, try both main directions and keep the one that yields a
    // closer-to-square canvas. This is a pragmatic stopgap until routing/packing parity
    // improves enough that a single fixed direction always looks good.
    let (report, layout, warnings) = if matches!(config.view_profile, LayoutViewProfile::InterconnectionView)
    {
        let candidates = [ElkLayoutDirection::TopToBottom, ElkLayoutDirection::LeftToRight];
        let mut best: Option<(f32, ElkGraph, elk_core::LayoutReport)> = None;
        for dir in candidates {
            let mut g = elk_graph.clone();
            options.layered.direction = dir;
            let report = LayeredLayoutEngine::new()
                .layout(&mut g, &options)
                .map_err(|error| crate::layout::DiagramError::LayoutFailure(error.to_string()))?;
            let b = g.bounds;
            let ar = if b.size.height > 0.0 {
                b.size.width / b.size.height
            } else {
                f32::MAX
            };
            let score = (ar.ln()).abs(); // 0 is perfect square
            if best.as_ref().is_none_or(|(best_score, _, _)| score < *best_score) {
                best = Some((score, g, report));
            }
        }
        let (_score, best_graph, report) = best.expect("interconnection candidates");
        let layout = map_layout_back(graph, &best_graph, &node_ids, &port_ids)?;
        let warnings = collect_warnings(graph, &report);
        (report, layout, warnings)
    } else {
        let report = LayeredLayoutEngine::new()
            .layout(&mut elk_graph, &options)
            .map_err(|error| crate::layout::DiagramError::LayoutFailure(error.to_string()))?;
        let layout = map_layout_back(graph, &elk_graph, &node_ids, &port_ids)?;
        let warnings = collect_warnings(graph, &report);
        (report, layout, warnings)
    };

    Ok(LayoutComputation {
        layout,
        report,
        warnings,
    })
}

fn sort_nodes_for_hierarchy<'a>(nodes: &'a [DiagramNode]) -> Result<Vec<&'a DiagramNode>> {
    let node_by_id: HashMap<&str, &DiagramNode> =
        nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut depths = HashMap::new();
    for node in nodes {
        let depth = node_depth(node, &node_by_id, &mut depths, &mut HashSet::new())?;
        depths.insert(node.id.as_str(), depth);
    }

    let mut ordered = nodes.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| {
        depths
            .get(left.id.as_str())
            .copied()
            .unwrap_or_default()
            .cmp(&depths.get(right.id.as_str()).copied().unwrap_or_default())
            .then_with(|| left.id.cmp(&right.id))
    });
    Ok(ordered)
}

fn node_depth<'a>(
    node: &'a DiagramNode,
    node_by_id: &HashMap<&'a str, &'a DiagramNode>,
    depths: &mut HashMap<&'a str, usize>,
    visiting: &mut HashSet<&'a str>,
) -> Result<usize> {
    if let Some(depth) = depths.get(node.id.as_str()).copied() {
        return Ok(depth);
    }
    if !visiting.insert(node.id.as_str()) {
        return Ok(0);
    }
    let depth = match node.parent_id.as_deref() {
        Some(parent_id) => {
            let parent = node_by_id
                .get(parent_id)
                .copied()
                .ok_or_else(|| crate::layout::DiagramError::MissingParent(parent_id.to_string()))?;
            node_depth(parent, node_by_id, depths, visiting)? + 1
        }
        None => 0,
    };
    visiting.remove(node.id.as_str());
    depths.insert(node.id.as_str(), depth);
    Ok(depth)
}

fn add_node(
    elk_graph: &mut ElkGraph,
    node: &DiagramNode,
    node_ids: &HashMap<String, elk_core::NodeId>,
) -> Result<elk_core::NodeId> {
    let width = if node.width <= 0.0 { 1.0 } else { node.width };
    let height = if node.height <= 0.0 { 1.0 } else { node.height };
    let size = ElkSize::new(width, height);
    Ok(match node.parent_id.as_deref() {
        Some(parent_id) => {
            let parent = *node_ids
                .get(parent_id)
                .ok_or_else(|| crate::layout::DiagramError::MissingParent(parent_id.to_string()))?;
            elk_graph.add_child_node(parent, size)
        }
        None => elk_graph.add_node(size),
    })
}

fn apply_node_hints(
    elk_graph: &mut ElkGraph,
    node_id: elk_core::NodeId,
    node: &DiagramNode,
    index: usize,
    _config: &LayoutConfig,
) {
    let layout = &mut elk_graph.node_mut(node_id).layout;
    layout.model_order = Some(index);
    if node.kind == "layout-branch" && node.parent_id.is_none() {
        layout.layer_constraint = Some(LayerConstraint::First);
    }
}

fn map_endpoint(
    node_id: &str,
    port_id: Option<&str>,
    node_ids: &HashMap<String, elk_core::NodeId>,
    port_ids: &HashMap<String, elk_core::PortId>,
) -> Result<EdgeEndpoint> {
    let node = *node_ids
        .get(node_id)
        .ok_or_else(|| crate::layout::DiagramError::MissingNode(node_id.to_string()))?;
    Ok(match port_id {
        Some(port_id) => {
            if let Some(port) = port_ids
                .get(port_id)
                .or_else(|| port_ids.get(normalize_port_id(port_id).as_str()))
                .copied()
            {
                EdgeEndpoint::port(node, port)
            } else {
                return Err(crate::layout::DiagramError::MissingPort(port_id.to_string()));
            }
        }
        None => EdgeEndpoint::node(node),
    })
}

fn map_port_side(side: &PortSide) -> ElkPortSide {
    match side {
        PortSide::Left => ElkPortSide::West,
        PortSide::Right => ElkPortSide::East,
        PortSide::Top => ElkPortSide::North,
        PortSide::Bottom => ElkPortSide::South,
    }
}

fn map_layout_options(config: &LayoutConfig) -> LayoutOptions {
    let mut options = LayoutOptions::default();
    options.view_profile = map_view_profile(&config.view_profile);
    options.apply_view_profile_defaults();
    options.layered.direction = map_direction(config);
    options.layered.node_alignment = match config.view_profile {
        LayoutViewProfile::GeneralView => NodeAlignment::Start,
        LayoutViewProfile::InterconnectionView => NodeAlignment::Balanced,
        LayoutViewProfile::Default => NodeAlignment::Balanced,
    };
    options.layered.padding = ElkPadding {
        top: config.root_gap_y,
        right: config.root_gap_x,
        bottom: config.root_gap_y,
        left: config.root_gap_x,
    };
    options.layered.spacing = ElkSpacing {
        node_spacing: config.node_gap_x.min(config.node_gap_y),
        layer_spacing: config.node_gap_y.max(config.node_gap_x),
        edge_spacing: match config.view_profile {
            LayoutViewProfile::GeneralView => 32.0,
            LayoutViewProfile::InterconnectionView => 30.0,
            LayoutViewProfile::Default => 24.0,
        },
        segment_spacing: match config.view_profile {
            LayoutViewProfile::GeneralView => 28.0,
            LayoutViewProfile::InterconnectionView => 24.0,
            LayoutViewProfile::Default => 20.0,
        },
        label_spacing: 10.0,
        port_label_spacing: 8.0,
        component_spacing: config.root_gap_x.max(config.root_gap_y),
        label_clearance: 12.0,
    };
    if matches!(config.view_profile, LayoutViewProfile::GeneralView) {
        options.layered.component_packing = false;
        options.layered.compactness = 0.45;
        options.layered.preferred_connector_lanes = 4;
    }
    if matches!(config.view_profile, LayoutViewProfile::InterconnectionView) {
        options.layered.compactness = 0.66;
        options.layered.preferred_connector_lanes = 6;
    }
    options.element_defaults = ElementLayoutOptions {
        port_constraint: Some(PortConstraint::FixedSide),
        padding: Some(ElkPadding {
            top: config.top_padding,
            right: config.container_padding,
            bottom: config.container_padding,
            left: config.container_padding,
        }),
        ..ElementLayoutOptions::default()
    };
    options
}

fn map_view_profile(profile: &LayoutViewProfile) -> ElkViewProfile {
    match profile {
        LayoutViewProfile::Default => ElkViewProfile::Default,
        LayoutViewProfile::GeneralView => ElkViewProfile::GeneralView,
        LayoutViewProfile::InterconnectionView => ElkViewProfile::InterconnectionView,
    }
}

fn apply_graph_hints(
    elk_graph: &mut ElkGraph,
    graph: &DiagramGraph,
    node_ids: &HashMap<String, elk_core::NodeId>,
    config: &LayoutConfig,
) {
    if !matches!(config.view_profile, LayoutViewProfile::GeneralView) {
        return;
    }

    let mut incoming = HashMap::<&str, usize>::new();
    let mut outgoing = HashMap::<&str, usize>::new();
    for edge in &graph.edges {
        *outgoing.entry(edge.source_node.as_str()).or_default() += 1;
        *incoming.entry(edge.target_node.as_str()).or_default() += 1;
    }

    for node in &graph.nodes {
        let Some(elk_node_id) = node_ids.get(node.id.as_str()).copied() else {
            continue;
        };
        let kind = node.kind.trim().to_ascii_lowercase();
        let layout = &mut elk_graph.node_mut(elk_node_id).layout;
        let in_count = incoming.get(node.id.as_str()).copied().unwrap_or_default();
        let out_count = outgoing.get(node.id.as_str()).copied().unwrap_or_default();
        if kind.contains("part def") && in_count == 0 && out_count > 0 {
            layout.layer_constraint = Some(LayerConstraint::First);
        } else if !kind.contains("part") && out_count == 0 && in_count > 0 {
            layout.layer_constraint = Some(LayerConstraint::Last);
        }
    }
}

fn map_direction(config: &LayoutConfig) -> ElkLayoutDirection {
    match config.view_profile {
        LayoutViewProfile::GeneralView => ElkLayoutDirection::TopToBottom,
        // Interconnection views tend to get excessively wide with LTR layering due to dense
        // port-to-port connectors. Prefer TTB as a more stable default; auto-direction selection
        // can further refine this in the future.
        LayoutViewProfile::InterconnectionView => ElkLayoutDirection::TopToBottom,
        LayoutViewProfile::Default => match (&config.root_layer_direction, &config.layer_direction) {
            (LayerDirection::HorizontalRows, _) => ElkLayoutDirection::TopToBottom,
            (LayerDirection::VerticalColumns, LayerDirection::HorizontalRows) => {
                ElkLayoutDirection::LeftToRight
            }
            _ => ElkLayoutDirection::TopToBottom,
        },
    }
}

fn map_layout_back(
    graph: &DiagramGraph,
    elk_graph: &ElkGraph,
    node_ids: &HashMap<String, elk_core::NodeId>,
    port_ids: &HashMap<String, elk_core::PortId>,
) -> Result<DiagramLayout> {
    let mut nodes = Vec::with_capacity(graph.nodes.len());
    for node in &graph.nodes {
        let elk_node = elk_graph.node(
            *node_ids
                .get(node.id.as_str())
                .ok_or_else(|| crate::layout::DiagramError::MissingNode(node.id.clone()))?,
        );
        nodes.push(NodeLayout {
            id: node.id.clone(),
            label: node.label.clone(),
            kind: node.kind.clone(),
            detail_lines: node.detail_lines.clone(),
            bounds: map_rect(elk_node.bounds),
            parent_id: node.parent_id.clone(),
            ports: node
                .ports
                .iter()
                .map(|port| map_port_layout(node.id.as_str(), port, elk_graph, port_ids))
                .collect::<Result<Vec<_>>>()?,
        });
    }

    // Debug: detect systematic edge endpoint offsets from ports after layout mapping.
    // This should be empty in normal operation; any entries indicate a routing/compound bug
    // (e.g. hierarchical ports not restored, or missing subtree edge translation).
    let debug_enabled = std::env::var("SPEC42_ELK_DEBUG").as_deref() == Ok("1");
    let mut port_pos_by_id: HashMap<&str, Point> = HashMap::new();
    for node in &nodes {
        for port in &node.ports {
            port_pos_by_id.insert(port.id.as_str(), port.position);
        }
    }

    let mut edges = Vec::with_capacity(graph.edges.len());
    for (index, edge) in graph.edges.iter().enumerate() {
        let elk_edge = &elk_graph.edges[index];
        let points = if let Some(section) = elk_edge.sections.first() {
            section_points(section)
        } else {
            fallback_edge_points(elk_graph, elk_edge)
        };
        if debug_enabled && points.len() >= 2 {
            if let Some(src_port) = edge.source_port.as_deref() {
                let normalized = normalize_port_id(src_port);
                if let Some(expected) = port_pos_by_id
                    .get(src_port)
                    .or_else(|| port_pos_by_id.get(normalized.as_str()))
                    .copied()
                {
                    let got = points[0];
                    let dx = (got.x - expected.x).abs();
                    let dy = (got.y - expected.y).abs();
                    if dx > 1.0 || dy > 1.0 {
                        eprintln!(
                            "sysml-diagrams: edge {} source_port_offset port={} expected=({:.1},{:.1}) got=({:.1},{:.1}) dx={:.1} dy={:.1}",
                            edge.id, src_port, expected.x, expected.y, got.x, got.y, dx, dy
                        );
                    }
                }
            }
            if let Some(tgt_port) = edge.target_port.as_deref() {
                let normalized = normalize_port_id(tgt_port);
                if let Some(expected) = port_pos_by_id
                    .get(tgt_port)
                    .or_else(|| port_pos_by_id.get(normalized.as_str()))
                    .copied()
                {
                    let got = *points.last().unwrap();
                    let dx = (got.x - expected.x).abs();
                    let dy = (got.y - expected.y).abs();
                    if dx > 1.0 || dy > 1.0 {
                        eprintln!(
                            "sysml-diagrams: edge {} target_port_offset port={} expected=({:.1},{:.1}) got=({:.1},{:.1}) dx={:.1} dy={:.1}",
                            edge.id, tgt_port, expected.x, expected.y, got.x, got.y, dx, dy
                        );
                    }
                }
            }
        }
        edges.push(EdgeLayout {
            id: edge.id.clone(),
            source_node: edge.source_node.clone(),
            target_node: edge.target_node.clone(),
            kind: edge.kind.clone(),
            label: edge.label.clone(),
            points,
        });
    }

    let canvas = canvas_bounds(&nodes, &edges, elk_graph.bounds);

    Ok(DiagramLayout {
        width: canvas.width,
        height: canvas.height,
        nodes,
        edges,
    })
}

fn map_rect(rect: ElkRect) -> Bounds {
    Bounds {
        x: rect.origin.x,
        y: rect.origin.y,
        width: rect.size.width,
        height: rect.size.height,
    }
}

fn map_port_layout(
    node_id: &str,
    port: &DiagramPort,
    elk_graph: &ElkGraph,
    port_ids: &HashMap<String, elk_core::PortId>,
) -> Result<PortLayout> {
    let elk_port = elk_graph.port(
        *port_ids
            .get(port.id.as_str())
            .ok_or_else(|| crate::layout::DiagramError::MissingPort(port.id.clone()))?,
    );
    let center = elk_port.bounds.center();
    Ok(PortLayout {
        id: port.id.clone(),
        name: port.name.clone(),
        node_id: node_id.to_string(),
        side: port.side.clone(),
        position: Point {
            x: center.x,
            y: center.y,
        },
    })
}

fn section_points(section: &elk_core::EdgeSection) -> Vec<Point> {
    let mut points = Vec::with_capacity(section.bend_points.len() + 2);
    points.push(map_point(section.start));
    points.extend(section.bend_points.iter().copied().map(map_point));
    points.push(map_point(section.end));
    points
}

fn fallback_edge_points(elk_graph: &ElkGraph, edge: &elk_core::Edge) -> Vec<Point> {
    vec![
        endpoint_center(elk_graph, edge.source),
        endpoint_center(elk_graph, edge.target),
    ]
}

fn endpoint_center(elk_graph: &ElkGraph, endpoint: EdgeEndpoint) -> Point {
    if let Some(port_id) = endpoint.port {
        return map_point(elk_graph.port(port_id).bounds.center());
    }
    map_point(elk_graph.node(endpoint.node).bounds.center())
}

fn map_point(point: ElkPoint) -> Point {
    Point {
        x: point.x,
        y: point.y,
    }
}

fn canvas_bounds(nodes: &[NodeLayout], edges: &[EdgeLayout], fallback: ElkRect) -> Bounds {
    let mut max_x = fallback.origin.x + fallback.size.width;
    let mut max_y = fallback.origin.y + fallback.size.height;

    for node in nodes {
        max_x = max_x.max(node.bounds.right());
        max_y = max_y.max(node.bounds.bottom());
    }

    for edge in edges {
        for point in &edge.points {
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }
        if edge.label.is_some() && edge.points.len() >= 2 {
            let label = crate::layout::svg::label_bounds(&edge.points);
            max_x = max_x.max(label.x + 80.0);
            max_y = max_y.max(label.y + 16.0);
        }
    }

    Bounds {
        x: 0.0,
        y: 0.0,
        width: max_x.max(0.0).ceil(),
        height: max_y.max(0.0).ceil(),
    }
}

fn collect_warnings(graph: &DiagramGraph, report: &elk_core::LayoutReport) -> Vec<String> {
    let mut warnings = report.warnings.clone();
    if graph.edges.iter().any(|edge| {
        edge.label
            .as_deref()
            .is_some_and(|label| !label.trim().is_empty())
    }) {
        warnings.push(
            "elk-rust edge label placement is still partial; SVG labels are rendered by sysml-language-server.".to_string(),
        );
    }
    if graph
        .edges
        .iter()
        .any(|edge| edge_crosses_hierarchy(edge, graph))
    {
        warnings.push(
            "elk-rust compound routing is still maturing for cross-hierarchy edges; inspect nested diagrams visually.".to_string(),
        );
    }
    warnings.sort();
    warnings.dedup();
    warnings
}

fn edge_crosses_hierarchy(edge: &DiagramEdge, graph: &DiagramGraph) -> bool {
    let node_by_id: HashMap<&str, &DiagramNode> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let Some(source) = node_by_id.get(edge.source_node.as_str()).copied() else {
        return false;
    };
    let Some(target) = node_by_id.get(edge.target_node.as_str()).copied() else {
        return false;
    };
    source.parent_id != target.parent_id
        && (source.parent_id.is_some() || target.parent_id.is_some())
}

fn normalize_port_id(value: &str) -> String {
    value.replace("::", ".")
}

/// Hash edge kind to a u32 bundle key so same-type edges share connection points.
fn edge_bundle_key(kind: &str) -> u32 {
    let mut h: u32 = 0u32.wrapping_sub(1);
    for b in kind.bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    h
}

