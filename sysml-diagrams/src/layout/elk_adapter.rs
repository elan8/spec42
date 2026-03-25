use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use elk_core::{
    ElementLayoutOptions, LayoutDirection as ElkLayoutDirection, LayoutOptions, NodeAlignment,
    Padding as ElkPadding, Point as ElkPoint, PortConstraint, PortSide as ElkPortSide,
    Rect as ElkRect, Size as ElkSize, Spacing as ElkSpacing, ViewProfile as ElkViewProfile,
};
use elk_graph::{
    EdgeEndpoint, ElkGraph, NodeId, PortId, PropertyKey, PropertyValue, ShapeGeometry,
};
use elk_layered::LayeredLayoutEngine;
use elk_testkit::run_java_elk_json;

use crate::layout::{
    Bounds, DiagramEdge, DiagramGraph, DiagramLayout, DiagramNode, DiagramPort, EdgeLayout,
    LayerDirection, LayoutConfig, LayoutViewProfile, NodeLayout, Point, PortLayout, PortSide,
    Result,
};

#[derive(Default)]
struct Spec42JsonIdMaps {
    node_json_id_by_spec42: HashMap<String, String>,
    port_json_id_by_spec42: HashMap<String, String>,
    edge_json_id_by_spec42: HashMap<String, String>,
}

pub(crate) struct LayoutComputation {
    pub layout: DiagramLayout,
    pub report: elk_core::LayoutReport,
    pub warnings: Vec<String>,
}

pub(crate) fn compute_layout(
    graph: &DiagramGraph,
    config: &LayoutConfig,
) -> Result<LayoutComputation> {
    let use_java = std::env::var("SPEC42_ELK_USE_JAVA")
        .ok()
        .as_deref()
        .is_some_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));

    let mut elk_graph = ElkGraph::new();
    let ordered_nodes = sort_nodes_for_hierarchy(&graph.nodes)?;
    let container_node_ids: HashSet<&str> = ordered_nodes
        .iter()
        .filter_map(|node| node.parent_id.as_deref())
        .collect();
    let mut node_ids: HashMap<String, NodeId> = HashMap::new();
    let mut port_ids: HashMap<String, PortId> = HashMap::new();

    for (index, node) in ordered_nodes.iter().enumerate() {
        let elk_node_id = add_node(&mut elk_graph, node, &node_ids)?;
        let is_container = container_node_ids.contains(node.id.as_str());
        apply_node_hints(&mut elk_graph, elk_node_id, node, index, config, use_java, is_container);
        elk_graph.nodes[elk_node_id.index()].properties.insert(
            "spec42.node_id",
            PropertyValue::String(node.id.clone()),
        );
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
                ShapeGeometry {
                    x: 0.0,
                    y: 0.0,
                    width: 8.0,
                    height: 8.0,
                },
            );
            // ELK port placement: border offset from the node boundary.
            // Set on the port itself (in addition to the root), since some implementations read
            // port-specific values during port position calculation.
            elk_graph.ports[elk_port_id.index()].properties.insert(
                "org.eclipse.elk.port.borderOffset",
                PropertyValue::Float(-4.0),
            );
            elk_graph.ports[elk_port_id.index()].properties.insert(
                "elk.port.borderOffset",
                PropertyValue::Float(-4.0),
            );
            elk_graph.ports[elk_port_id.index()].properties.insert(
                "elk.port.index",
                PropertyValue::Int(index as i64),
            );
            elk_graph.ports[elk_port_id.index()].properties.insert(
                "spec42.port_id",
                PropertyValue::String(port.id.clone()),
            );
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
        let edge_id = elk_graph.add_edge(elk_graph.root, vec![source], vec![target]);
        elk_graph.edges[edge_id.index()].properties.insert(
            "elk.edge.bundle",
            PropertyValue::Int(edge_bundle_key(&edge.kind) as i64),
        );
        elk_graph.edges[edge_id.index()].properties.insert(
            "spec42.edge_id",
            PropertyValue::String(edge.id.clone()),
        );
    }

    apply_graph_hints(&mut elk_graph, graph, &node_ids, config);

    let options = map_layout_options(config);

    // Keep adapter-level behavior Java-aligned: do not try multiple directions or score candidates
    // here. Direction selection is controlled by the ELK options/view profile, as in upstream.
    let (report, layout, warnings) = if use_java {
        apply_java_layout_options_to_root_node(&mut elk_graph, &options);
        if let Ok(path) = std::env::var("SPEC42_ELK_JAVA_OPTIONS_OUT") {
            if let Some(parent) = Path::new(&path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            let root_props = &elk_graph.nodes[elk_graph.root.index()].properties;
            let mut s = String::new();
            for (k, v) in root_props.iter() {
                s.push_str(&format!("{k:?}={v:?}\n"));
            }
            let _ = fs::write(&path, s);
        }
        let input_json = elk_graph_json::export_elk_graph_to_value(&elk_graph);
        let spec42_json_ids = build_spec42_id_to_json_id_maps(&input_json);
        if let Ok(path) = std::env::var("SPEC42_ELK_GRAPH_JSON_OUT") {
            if let Some(parent) = Path::new(&path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            let pretty =
                serde_json::to_string_pretty(&input_json).unwrap_or_else(|_| "{}".to_string());
            let _ = fs::write(&path, pretty);
        }
        let java_json = run_java_elk_json(&input_json)
            .map_err(|error| crate::layout::DiagramError::LayoutFailure(error))?;

        if let Ok(path) = std::env::var("SPEC42_ELK_JAVA_JSON_OUT") {
            if let Some(parent) = Path::new(&path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            let pretty = serde_json::to_string_pretty(&java_json).unwrap_or_else(|_| "{}".to_string());
            let _ = fs::write(&path, pretty);
        }

        let java_str = serde_json::to_string(&java_json)
            .map_err(|e| crate::layout::DiagramError::LayoutFailure(e.to_string()))?;
        let imported = elk_graph_json::import_str(&java_str)
            .map_err(|e| crate::layout::DiagramError::LayoutFailure(format!("{e:?}")))?;
        let report = elk_core::LayoutReport::default();
        let (java_node_ids, java_port_ids, java_edge_index_by_id) =
            remap_ids_from_imported_graph(&imported.graph, &spec42_json_ids);
        let layout = map_layout_back(
            graph,
            &imported.graph,
            &java_node_ids.unwrap_or_else(|| node_ids.clone()),
            &java_port_ids.unwrap_or_else(|| port_ids.clone()),
            java_edge_index_by_id.as_ref(),
        )?;
        let mut warnings = collect_warnings(graph, &report);
        warnings.extend(imported.warnings.warnings);
        (report, layout, warnings)
    } else {
        let report = LayeredLayoutEngine::new()
            .layout(&mut elk_graph, &options)
            .map_err(|error| crate::layout::DiagramError::LayoutFailure(error.to_string()))?;
        let layout = map_layout_back(graph, &elk_graph, &node_ids, &port_ids, None)?;
        let warnings = collect_warnings(graph, &report);
        (report, layout, warnings)
    };

    Ok(LayoutComputation {
        layout,
        report,
        warnings,
    })
}

fn apply_java_layout_options_to_root_node(elk_graph: &mut ElkGraph, options: &LayoutOptions) {
    fn direction_str(dir: ElkLayoutDirection) -> &'static str {
        match dir {
            ElkLayoutDirection::LeftToRight => "RIGHT",
            ElkLayoutDirection::RightToLeft => "LEFT",
            ElkLayoutDirection::TopToBottom => "DOWN",
            ElkLayoutDirection::BottomToTop => "UP",
        }
    }
    fn node_alignment_str(alignment: NodeAlignment) -> &'static str {
        match alignment {
            NodeAlignment::Start => "START",
            NodeAlignment::Center => "CENTER",
            NodeAlignment::End => "END",
            NodeAlignment::Balanced => "BALANCED",
        }
    }

    let root = elk_graph.root;
    let props = &mut elk_graph.nodes[root.index()].properties;
    props.insert(
        "elk.algorithm",
        PropertyValue::String("org.eclipse.elk.layered".to_string()),
    );
    props.insert(
        "elk.direction",
        PropertyValue::String(direction_str(options.layered.direction).to_string()),
    );
    props.insert(
        "elk.edgeRouting",
        PropertyValue::String("ORTHOGONAL".to_string()),
    );
    // Ensure cross-hierarchy edges are routed in a single run for Java baseline readability.
    // Default (when unset) is effectively SEPARATE_CHILDREN which can leave cross edges unrouted.
    props.insert(
        "org.eclipse.elk.hierarchyHandling",
        PropertyValue::String("INCLUDE_CHILDREN".to_string()),
    );
    props.insert(
        "elk.nodeAlignment",
        PropertyValue::String(node_alignment_str(options.layered.node_alignment).to_string()),
    );
    if let Some(port_constraint) = options.element_defaults.port_constraint {
        let v = match port_constraint {
            PortConstraint::Free => "FREE",
            PortConstraint::FixedSide => "FIXED_SIDE",
            PortConstraint::FixedOrder => "FIXED_ORDER",
            PortConstraint::FixedPosition => "FIXED_POS",
        };
        props.insert("elk.portConstraints", PropertyValue::String(v.to_string()));
    }
    // Port placement: distance from the node border. Positive moves ports outward.
    props.insert(
        "org.eclipse.elk.port.borderOffset",
        PropertyValue::Float(-4.0),
    );
    props.insert(
        "elk.spacing.nodeNode",
        PropertyValue::Float(options.layered.spacing.node_spacing as f64),
    );
    props.insert(
        "elk.spacing.nodeNodeBetweenLayers",
        PropertyValue::Float(options.layered.spacing.layer_spacing as f64),
    );
    props.insert(
        "elk.spacing.edgeEdge",
        PropertyValue::Float(options.layered.spacing.edge_spacing as f64),
    );
    props.insert(
        "elk.spacing.edgeNode",
        PropertyValue::Float(options.layered.spacing.edge_spacing as f64),
    );
    props.insert(
        "elk.spacing.edgeLabel",
        PropertyValue::Float(options.layered.spacing.label_spacing as f64),
    );
    props.insert(
        "elk.spacing.labelLabel",
        PropertyValue::Float(options.layered.spacing.label_clearance as f64),
    );
    props.insert(
        "elk.spacing.componentComponent",
        PropertyValue::Float(options.layered.spacing.component_spacing as f64),
    );
    props.insert(
        "elk.padding",
        PropertyValue::String(format!(
            "[{:.1},{:.1},{:.1},{:.1}]",
            options.layered.padding.top,
            options.layered.padding.right,
            options.layered.padding.bottom,
            options.layered.padding.left
        )),
    );
    props.insert(
        "org.eclipse.elk.json.shapeCoords",
        PropertyValue::String("ROOT".to_string()),
    );
    props.insert(
        "org.eclipse.elk.json.edgeCoords",
        PropertyValue::String("ROOT".to_string()),
    );
}

fn sort_nodes_for_hierarchy(nodes: &[DiagramNode]) -> Result<Vec<&DiagramNode>> {
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
    node_ids: &HashMap<String, NodeId>,
) -> Result<NodeId> {
    let width = if node.width <= 0.0 { 1.0 } else { node.width };
    let height = if node.height <= 0.0 { 1.0 } else { node.height };
    let geometry = ShapeGeometry {
        x: 0.0,
        y: 0.0,
        width,
        height,
    };
    let parent = match node.parent_id.as_deref() {
        Some(parent_id) => *node_ids
            .get(parent_id)
            .ok_or_else(|| crate::layout::DiagramError::MissingParent(parent_id.to_string()))?,
        None => elk_graph.root,
    };
    Ok(elk_graph.add_node(parent, geometry))
}

fn apply_node_hints(
    elk_graph: &mut ElkGraph,
    node_id: NodeId,
    node: &DiagramNode,
    _index: usize,
    config: &LayoutConfig,
    use_java: bool,
    is_container: bool,
) {
    // Java baseline tuning for interconnection view: ensure container children never overlap the
    // visual header bar by forcing a larger top padding on container nodes.
    if use_java
        && matches!(config.view_profile, LayoutViewProfile::InterconnectionView)
        && is_container
    {
        // SVG header bars are effectively ~44px; keep a bit of breathing room.
        let header_margin = 10.0f32;
        let top = (config.container_header_height + header_margin).max(config.top_padding);
        let right = config.container_padding;
        let bottom = config.container_padding;
        let left = config.container_padding;
        // Java ELK expects the fully qualified key; keep the short alias too since elk-rust
        // historically used it in a few places.
        let padding_java = format!(
            "[top={top:.1},left={left:.1},bottom={bottom:.1},right={right:.1}]"
        );
        let padding_short = format!("[{top:.1},{right:.1},{bottom:.1},{left:.1}]");
        elk_graph.nodes[node_id.index()]
            .properties
            .insert("org.eclipse.elk.padding", PropertyValue::String(padding_java));
        elk_graph.nodes[node_id.index()]
            .properties
            .insert("elk.padding", PropertyValue::String(padding_short));
        // Keep containers in the same hierarchy layout run; otherwise children may be laid out in a
        // separate run with default padding, which defeats header clearance.
        elk_graph.nodes[node_id.index()].properties.insert(
            "org.eclipse.elk.hierarchyHandling",
            PropertyValue::String("INCLUDE_CHILDREN".to_string()),
        );
        // Let ELK grow containers to fit their children (avoids child overflow/overlaps).
        elk_graph.nodes[node_id.index()].properties.insert(
            "org.eclipse.elk.nodeSize.constraints",
            PropertyValue::String("[PORTS, NODE_LABELS, MINIMUM_SIZE]".to_string()),
        );
    }

    if node.kind == "layout-branch" && node.parent_id.is_none() {
        elk_graph.nodes[node_id.index()].properties.insert(
            "elk.layerConstraint",
            PropertyValue::String("FIRST".into()),
        );
    }
}

fn map_endpoint(
    node_id: &str,
    port_id: Option<&str>,
    node_ids: &HashMap<String, NodeId>,
    port_ids: &HashMap<String, PortId>,
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
    let mut options = LayoutOptions {
        view_profile: map_view_profile(&config.view_profile),
        ..LayoutOptions::default()
    };
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
        options.layered.compactness = 0.7;
        options.layered.preferred_connector_lanes = 7;
        options.layered.spacing.segment_spacing = 28.0;
        options.layered.spacing.edge_spacing = 32.0;
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
    node_ids: &HashMap<String, NodeId>,
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
        let in_count = incoming.get(node.id.as_str()).copied().unwrap_or_default();
        let out_count = outgoing.get(node.id.as_str()).copied().unwrap_or_default();
        let prop = if kind.contains("part def") && in_count == 0 && out_count > 0 {
            Some("FIRST")
        } else if !kind.contains("part") && out_count == 0 && in_count > 0 {
            Some("LAST")
        } else {
            None
        };
        if let Some(s) = prop {
            elk_graph.nodes[elk_node_id.index()].properties.insert(
                "elk.layerConstraint",
                PropertyValue::String(s.into()),
            );
        }
    }
}

fn map_direction(config: &LayoutConfig) -> ElkLayoutDirection {
    let use_java = std::env::var("SPEC42_ELK_USE_JAVA")
        .ok()
        .as_deref()
        .is_some_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
    match config.view_profile {
        LayoutViewProfile::GeneralView => ElkLayoutDirection::TopToBottom,
        // Interconnection views tend to get excessively wide with LTR layering due to dense
        // port-to-port connectors. Prefer TTB as a more stable default; auto-direction selection
        // can further refine this in the future.
        LayoutViewProfile::InterconnectionView => {
            // Java baseline preference: left-to-right reduces top/bottom port usage and is often
            // more readable for IBD interconnections.
            if use_java {
                ElkLayoutDirection::LeftToRight
            } else {
                ElkLayoutDirection::TopToBottom
            }
        }
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
    node_ids: &HashMap<String, NodeId>,
    port_ids: &HashMap<String, PortId>,
    edge_index_by_id: Option<&HashMap<String, usize>>,
) -> Result<DiagramLayout> {
    let node_absolute_origins = compute_absolute_node_origins(elk_graph);
    let edge_containers = edge_container_nodes(elk_graph);

    let mut nodes = Vec::with_capacity(graph.nodes.len());
    for node in &graph.nodes {
        let elk_node_id = *node_ids
            .get(node.id.as_str())
            .ok_or_else(|| crate::layout::DiagramError::MissingNode(node.id.clone()))?;
        let elk_node = &elk_graph.nodes[elk_node_id.index()];
        let abs_origin = node_absolute_origins[elk_node_id.index()];
        nodes.push(NodeLayout {
            id: node.id.clone(),
            label: node.label.clone(),
            kind: node.kind.clone(),
            detail_lines: node.detail_lines.clone(),
            bounds: Bounds {
                x: abs_origin.x,
                y: abs_origin.y,
                width: elk_node.geometry.width,
                height: elk_node.geometry.height,
            },
            parent_id: node.parent_id.clone(),
            ports: node
                .ports
                .iter()
                .map(|port| {
                    map_port_layout(
                        node.id.as_str(),
                        port,
                        elk_graph,
                        &port_ids,
                        &node_absolute_origins,
                    )
                })
                .collect::<Result<Vec<_>>>()?,
        });
    }
    // Debug: detect systematic edge endpoint offsets from ports after layout mapping.
    // This should be empty in normal operation; any entries indicate a routing/compound bug
    // (e.g. hierarchical ports not restored, or missing subtree edge translation).
    let debug_enabled = std::env::var("SPEC42_ELK_DEBUG").as_deref() == Ok("1");
    let mut max_src_dx = 0.0f32;
    let mut max_src_dy = 0.0f32;
    let mut max_tgt_dx = 0.0f32;
    let mut max_tgt_dy = 0.0f32;
    let mut src_mismatch_count = 0usize;
    let mut tgt_mismatch_count = 0usize;
    let mut port_pos_by_id: HashMap<&str, Point> = HashMap::new();
    for node in &nodes {
        for port in &node.ports {
            port_pos_by_id.insert(port.id.as_str(), port.position);
        }
    }

    let mut edges = Vec::with_capacity(graph.edges.len());
    for (index, edge) in graph.edges.iter().enumerate() {
        let elk_edge_index = edge_index_by_id
            .and_then(|m| m.get(edge.id.as_str()).copied())
            .unwrap_or(index);
        let elk_edge = elk_graph
            .edges
            .get(elk_edge_index)
            .unwrap_or_else(|| &elk_graph.edges[index.min(elk_graph.edges.len().saturating_sub(1))]);
        let points = if !elk_edge.sections.is_empty() {
            let raw = edge_section_points(elk_graph, elk_edge);
            translate_edge_points_for_container(
                &raw,
                edge_containers[elk_edge.id.index()],
                &node_absolute_origins,
            )
        } else {
            fallback_edge_points(elk_graph, elk_edge, &node_absolute_origins)
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
                        src_mismatch_count += 1;
                        max_src_dx = max_src_dx.max(dx);
                        max_src_dy = max_src_dy.max(dy);
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
                        tgt_mismatch_count += 1;
                        max_tgt_dx = max_tgt_dx.max(dx);
                        max_tgt_dy = max_tgt_dy.max(dy);
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
    if debug_enabled {
        let summary = format!(
            "endpoint_offset_summary source_mismatches={} source_max_dx={:.1} source_max_dy={:.1} target_mismatches={} target_max_dx={:.1} target_max_dy={:.1}",
            src_mismatch_count, max_src_dx, max_src_dy, tgt_mismatch_count, max_tgt_dx, max_tgt_dy
        );
        eprintln!("sysml-diagrams: {summary}");
        if let Ok(path) = std::env::var("SPEC42_ELK_ENDPOINT_OFFSETS_OUT") {
            if let Some(parent) = Path::new(&path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(path, summary);
        }
    }

    let root_geom = elk_graph.nodes[elk_graph.root.index()].geometry;
    let fallback_rect = ElkRect::new(
        ElkPoint::new(root_geom.x, root_geom.y),
        ElkSize::new(root_geom.width, root_geom.height),
    );
    let canvas = canvas_bounds(&nodes, &edges, fallback_rect);

    Ok(DiagramLayout {
        width: canvas.width,
        height: canvas.height,
        nodes,
        edges,
    })
}

fn map_port_layout(
    node_id: &str,
    port: &DiagramPort,
    elk_graph: &ElkGraph,
    port_ids: &HashMap<String, PortId>,
    node_absolute_origins: &[Point],
) -> Result<PortLayout> {
    let elk_port_id = *port_ids
        .get(port.id.as_str())
        .ok_or_else(|| crate::layout::DiagramError::MissingPort(port.id.clone()))?;
    let elk_port = &elk_graph.ports[elk_port_id.index()];
    let g = &elk_port.geometry;
    let origin = node_absolute_origins[elk_port.node.index()];
    // ELK edge section endpoints connect to the port's *anchor point* on its border (not the
    // visual center). Use the side to compute the correct anchor so polylines land on ports.
    let effective_side = match elk_port.side {
        ElkPortSide::West => PortSide::Left,
        ElkPortSide::East => PortSide::Right,
        ElkPortSide::North => PortSide::Top,
        ElkPortSide::South => PortSide::Bottom,
    };
    let (anchor_x, anchor_y) = match effective_side {
        PortSide::Left => (origin.x + g.x, origin.y + g.y + g.height / 2.0),
        PortSide::Right => (origin.x + g.x + g.width, origin.y + g.y + g.height / 2.0),
        PortSide::Top => (origin.x + g.x + g.width / 2.0, origin.y + g.y),
        PortSide::Bottom => (origin.x + g.x + g.width / 2.0, origin.y + g.y + g.height),
    };
    Ok(PortLayout {
        id: port.id.clone(),
        name: port.name.clone(),
        node_id: node_id.to_string(),
        side: effective_side,
        position: Point {
            x: anchor_x,
            y: anchor_y,
        },
    })
}

fn build_spec42_id_to_json_id_maps(input_json: &serde_json::Value) -> Spec42JsonIdMaps {
    fn opt_str(obj: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<String> {
        obj.get("layoutOptions")
            .and_then(|v| v.as_object())
            .and_then(|o| o.get(key))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }
    fn walk(v: &serde_json::Value, out: &mut Spec42JsonIdMaps) {
        let Some(obj) = v.as_object() else { return };
        let json_id = obj.get("id").and_then(|v| v.as_str()).unwrap_or("");
        if let Some(spec42_node) = opt_str(obj, "spec42.node_id") {
            if !json_id.is_empty() {
                out.node_json_id_by_spec42
                    .insert(spec42_node, json_id.to_string());
            }
        }
        if let Some(ports) = obj.get("ports").and_then(|v| v.as_array()) {
            for p in ports {
                let Some(pobj) = p.as_object() else { continue };
                let pid = pobj.get("id").and_then(|v| v.as_str()).unwrap_or("");
                if let Some(spec42_port) = opt_str(pobj, "spec42.port_id") {
                    if !pid.is_empty() {
                        out.port_json_id_by_spec42
                            .insert(spec42_port, pid.to_string());
                    }
                }
            }
        }
        if let Some(edges) = obj.get("edges").and_then(|v| v.as_array()) {
            for e in edges {
                let Some(eobj) = e.as_object() else { continue };
                let eid = eobj.get("id").and_then(|v| v.as_str()).unwrap_or("");
                if let Some(spec42_edge) = opt_str(eobj, "spec42.edge_id") {
                    if !eid.is_empty() {
                        out.edge_json_id_by_spec42
                            .insert(spec42_edge, eid.to_string());
                    }
                }
            }
        }
        if let Some(children) = obj.get("children").and_then(|v| v.as_array()) {
            for c in children {
                walk(c, out);
            }
        }
    }
    let mut out = Spec42JsonIdMaps::default();
    walk(input_json, &mut out);
    out
}

fn remap_ids_from_imported_graph(
    elk_graph: &ElkGraph,
    maps: &Spec42JsonIdMaps,
) -> (
    Option<HashMap<String, NodeId>>,
    Option<HashMap<String, PortId>>,
    Option<HashMap<String, usize>>,
) {
    let json_id_key = PropertyKey::from("spec42.original_json_id");

    let json_to_spec42_node: HashMap<&str, &str> = maps
        .node_json_id_by_spec42
        .iter()
        .map(|(k, v)| (v.as_str(), k.as_str()))
        .collect();
    let json_to_spec42_port: HashMap<&str, &str> = maps
        .port_json_id_by_spec42
        .iter()
        .map(|(k, v)| (v.as_str(), k.as_str()))
        .collect();
    let json_to_spec42_edge: HashMap<&str, &str> = maps
        .edge_json_id_by_spec42
        .iter()
        .map(|(k, v)| (v.as_str(), k.as_str()))
        .collect();

    let mut node_ids = HashMap::new();
    for node in &elk_graph.nodes {
        if let Some(json_id) = node.properties.get_str(&json_id_key) {
            if let Some(spec42) = json_to_spec42_node.get(json_id).copied() {
                node_ids.insert(spec42.to_string(), node.id);
            }
        }
    }

    let mut port_ids = HashMap::new();
    for port in &elk_graph.ports {
        if let Some(json_id) = port.properties.get_str(&json_id_key) {
            if let Some(spec42) = json_to_spec42_port.get(json_id).copied() {
                port_ids.insert(spec42.to_string(), port.id);
                let normalized = normalize_port_id(spec42);
                if normalized != spec42 {
                    port_ids.insert(normalized, port.id);
                }
            }
        }
    }

    let mut edge_index = HashMap::new();
    for (idx, edge) in elk_graph.edges.iter().enumerate() {
        if let Some(json_id) = edge.properties.get_str(&json_id_key) {
            if let Some(spec42) = json_to_spec42_edge.get(json_id).copied() {
                edge_index.insert(spec42.to_string(), idx);
            }
        }
    }

    (
        (!node_ids.is_empty()).then_some(node_ids),
        (!port_ids.is_empty()).then_some(port_ids),
        (!edge_index.is_empty()).then_some(edge_index),
    )
}

fn compute_absolute_node_origins(elk_graph: &ElkGraph) -> Vec<Point> {
    fn compute_for(
        node_id: NodeId,
        elk_graph: &ElkGraph,
        cache: &mut [Option<Point>],
    ) -> Point {
        if let Some(point) = cache[node_id.index()] {
            return point;
        }
        let node = &elk_graph.nodes[node_id.index()];
        let local = Point {
            x: node.geometry.x,
            y: node.geometry.y,
        };
        let absolute = match node.parent {
            Some(parent_id) if parent_id != elk_graph.root => {
                let parent = compute_for(parent_id, elk_graph, cache);
                Point {
                    x: parent.x + local.x,
                    y: parent.y + local.y,
                }
            }
            _ => local,
        };
        cache[node_id.index()] = Some(absolute);
        absolute
    }

    let mut cache = vec![None; elk_graph.nodes.len()];
    for node in &elk_graph.nodes {
        let _ = compute_for(node.id, elk_graph, &mut cache);
    }
    cache
        .into_iter()
        .map(|point| point.unwrap_or(Point { x: 0.0, y: 0.0 }))
        .collect()
}

fn section_points(section: &elk_graph::EdgeSection) -> Vec<Point> {
    let mut points = Vec::with_capacity(section.bend_points.len() + 2);
    points.push(map_point(section.start));
    points.extend(
        section
            .bend_points
            .iter()
            .copied()
            .map(map_point),
    );
    points.push(map_point(section.end));
    points
}

fn edge_section_points(
    elk_graph: &ElkGraph,
    edge: &elk_graph::Edge,
) -> Vec<Point> {
    let mut points = Vec::new();
    for section_id in &edge.sections {
        let section = &elk_graph.edge_sections[section_id.index()];
        points.extend(section_points(section));
    }
    points
}

fn edge_container_nodes(elk_graph: &ElkGraph) -> Vec<NodeId> {
    let mut containers = vec![elk_graph.root; elk_graph.edges.len()];
    for node in &elk_graph.nodes {
        for edge_id in &node.edges {
            containers[edge_id.index()] = node.id;
        }
    }
    containers
}

fn translate_edge_points_for_container(
    points: &[Point],
    edge_container: NodeId,
    node_absolute_origins: &[Point],
) -> Vec<Point> {
    let origin = node_absolute_origins[edge_container.index()];
    points
        .iter()
        .map(|p| Point {
            x: p.x + origin.x,
            y: p.y + origin.y,
        })
        .collect()
}

fn edge_index_by_spec42_id(elk_graph: &ElkGraph) -> HashMap<String, usize> {
    let mut out = HashMap::new();
    let key = PropertyKey::from("spec42.edge_id");
    for (idx, e) in elk_graph.edges.iter().enumerate() {
        if let Some(id) = e.properties.get_str(&key) {
            out.insert(id.to_string(), idx);
        }
    }
    out
}

fn fallback_edge_points(
    elk_graph: &ElkGraph,
    edge: &elk_graph::Edge,
    node_absolute_origins: &[Point],
) -> Vec<Point> {
    let src = edge
        .sources
        .first()
        .map(|e| endpoint_center(elk_graph, *e, node_absolute_origins));
    let tgt = edge
        .targets
        .first()
        .map(|e| endpoint_center(elk_graph, *e, node_absolute_origins));
    match (src, tgt) {
        (Some(s), Some(t)) => vec![s, t],
        (Some(s), None) => vec![s],
        (None, Some(t)) => vec![t],
        (None, None) => vec![],
    }
}

fn endpoint_center(
    elk_graph: &ElkGraph,
    endpoint: EdgeEndpoint,
    node_absolute_origins: &[Point],
) -> Point {
    if let Some(port_id) = endpoint.port {
        let p = &elk_graph.ports[port_id.index()];
        let n = node_absolute_origins[p.node.index()];
        let g = &p.geometry;
        return Point {
            x: n.x + g.x + g.width / 2.0,
            y: n.y + g.y + g.height / 2.0,
        };
    }
    let origin = node_absolute_origins[endpoint.node.index()];
    let g = &elk_graph.nodes[endpoint.node.index()].geometry;
    Point {
        x: origin.x + g.width / 2.0,
        y: origin.y + g.height / 2.0,
    }
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
/// Reserved for future use when elk-layered props support edge bundle keys.
#[allow(dead_code)]
fn edge_bundle_key(kind: &str) -> u32 {
    let mut h: u32 = 0u32.wrapping_sub(1);
    for b in kind.bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    h
}
