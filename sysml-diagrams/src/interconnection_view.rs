use std::collections::HashSet;

use crate::layout::{DiagramGraph, DiagramPort, LayoutConfig, LayoutViewProfile, PortSide};

use crate::{
    shared::{
        build_rendered_diagram_with_config, default_node, detail_lines, edge, port_side_from_text,
    },
    IbdConnectorInput, IbdInput, IbdPartInput, IbdPortInput, RenderedDiagram, Result,
};

pub fn render(ibd: &IbdInput) -> Result<RenderedDiagram> {
    let default_root = ibd.default_root.clone();
    let filtered_parts = filter_parts(&ibd.parts, default_root.as_deref());
    let valid_ids: HashSet<String> = filtered_parts
        .iter()
        .map(|part| part.qualified_name.clone())
        .collect();

    const TITLE_HEIGHT: f32 = 44.0;
    const PORT_PITCH: f32 = 14.0;
    const MIN_NODE_WIDTH: f32 = 260.0;

    let diagram_nodes = filtered_parts
        .iter()
        .map(|part| {
            let ports = ports_for_part(part, &ibd.ports);
            let port_count = ports.len();
            let (left, right, top, bottom) = port_counts_per_side(&ports);
            let min_height = TITLE_HEIGHT
                + (left.max(right) as f32) * PORT_PITCH;
            let min_width = (top.max(bottom) as f32) * PORT_PITCH + 120.0;
            let width = MIN_NODE_WIDTH.max(min_width);
            let height = (100.0 + port_count.min(8) as f32 * 12.0).max(min_height);
            let detail = detail_lines(&part.attributes, &[format!("type: {}", part.element_type)]);
            let parent_id = part
                .container_id
                .as_ref()
                .map(|c| normalize_id(c));
            default_node(
                part.qualified_name.clone(),
                part.name.clone(),
                part.element_type.clone(),
                parent_id,
                detail,
                ports,
                width,
                height,
            )
        })
        .collect();

    let diagram_edges = ibd
        .connectors
        .iter()
        .enumerate()
        .filter_map(|(index, connector)| connector_edge(index, connector, &valid_ids))
        .collect();

    let use_java = std::env::var("SPEC42_ELK_USE_JAVA")
        .ok()
        .as_deref()
        .is_some_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));

    let mut layout_config = LayoutConfig {
        view_profile: LayoutViewProfile::InterconnectionView,
        ..LayoutConfig::default()
    };
    if use_java {
        // Java baseline readability tuning: prefer extra whitespace over collisions.
        layout_config.node_gap_x = 96.0;
        layout_config.node_gap_y = 96.0;
        layout_config.container_padding = 40.0;
        layout_config.container_header_height = 44.0;
        layout_config.top_padding = 64.0;
        layout_config.root_gap_x = 140.0;
        layout_config.root_gap_y = 140.0;
        layout_config.max_children_per_row = 2;
    }

    build_rendered_diagram_with_config(
        DiagramGraph {
            nodes: diagram_nodes,
            edges: diagram_edges,
        },
        "interconnection-view",
        default_root,
        layout_config,
    )
}

fn filter_parts<'a>(
    parts: &'a [IbdPartInput],
    default_root: Option<&str>,
) -> Vec<&'a IbdPartInput> {
    match default_root {
        Some(root) => parts
            .iter()
            .filter(|part| matches_root(part, root))
            .collect(),
        None => parts.iter().collect(),
    }
}

fn matches_root(part: &IbdPartInput, root: &str) -> bool {
    let qualified = normalize_id(&part.qualified_name);
    let root = normalize_id(root);
    part.name == root
        || qualified == root
        || qualified.ends_with(&format!(".{root}"))
        || qualified.starts_with(&format!("{root}."))
        || qualified.contains(&format!(".{root}."))
}

fn port_counts_per_side(ports: &[DiagramPort]) -> (usize, usize, usize, usize) {
    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;
    for port in ports {
        match port.side {
            PortSide::Left => left += 1,
            PortSide::Right => right += 1,
            PortSide::Top => top += 1,
            PortSide::Bottom => bottom += 1,
        }
    }
    (left, right, top, bottom)
}

fn ports_for_part(part: &IbdPartInput, ports: &[IbdPortInput]) -> Vec<DiagramPort> {
    ports
        .iter()
        .filter(|port| normalize_id(&port.parent_id) == normalize_id(&part.qualified_name))
        .map(|port| DiagramPort {
            id: normalize_id(&port.id),
            name: port.name.clone(),
            side: port_side_from_text(port.port_side.as_deref(), port.direction.as_deref()),
        })
        .collect()
}

fn connector_edge(
    index: usize,
    connector: &IbdConnectorInput,
    valid_ids: &HashSet<String>,
) -> Option<crate::layout::DiagramEdge> {
    let source_node = parent_from_port(&connector.source_id);
    let target_node = parent_from_port(&connector.target_id);
    if !valid_ids.contains(&source_node) || !valid_ids.contains(&target_node) {
        return None;
    }
    Some(edge(
        format!("ibd-edge-{index}"),
        source_node,
        target_node,
        Some(connector.source_id.clone()),
        Some(connector.target_id.clone()),
        Some(connector.rel_type.clone()),
        connector.rel_type.clone(),
    ))
}

fn parent_from_port(value: &str) -> String {
    if let Some((parent, _)) = value.rsplit_once('.') {
        return parent.to_string();
    }
    if let Some((parent, _)) = value.rsplit_once("::") {
        return parent.to_string();
    }
    value.to_string()
}

fn normalize_id(value: &str) -> String {
    value.replace("::", ".")
}
