use std::collections::BTreeMap;

use elk_core::{
    ContentAlignment, EdgeLabelPlacement, EdgeRouting, ElementLayoutOptions, LayoutDirection,
    LayerConstraint, NodeAlignment, NodeLabelPlacement, Padding, PortConstraint, PortLabelPlacement,
    Spacing,
};
use elk_graph::{PropertyBag, PropertyValue};

pub(crate) fn decode_layout_from_props(props: &PropertyBag) -> ElementLayoutOptions {
    let mut out = ElementLayoutOptions::default();
    apply_layout_from_props(props, &mut out);
    out
}

pub(crate) fn apply_layout_from_props(props: &PropertyBag, out: &mut ElementLayoutOptions) {
    let by_key: BTreeMap<String, &PropertyValue> = elk_alg_common::options::casefold_map(props);
    let meta = elk_meta::default_registry();

    // direction
    if let Some(PropertyValue::String(dir)) = elk_alg_common::options::find_option(
        &meta,
        &by_key,
        "elk.direction",
    ) {
        out.direction = parse_direction(dir).or(out.direction);
    }

    // edgeRouting
    if let Some(PropertyValue::String(r)) =
        elk_alg_common::options::find_option(&meta, &by_key, "elk.edgeRouting")
    {
        out.edge_routing = parse_edge_routing(r).or(out.edge_routing);
    }

    // portConstraints
    if let Some(PropertyValue::String(pc)) = find_value(
        &by_key,
        &[
            "elk.portconstraints",
            "elk.portConstraints",
            "org.eclipse.elk.portConstraints",
            "org.eclipse.elk.portconstraints",
        ],
    ) {
        out.port_constraint = parse_port_constraint(pc).or(out.port_constraint);
    }

    // layerConstraint
    if let Some(PropertyValue::String(lc)) = find_value(
        &by_key,
        &[
            "elk.layerconstraint",
            "elk.layerConstraint",
            "org.eclipse.elk.layerConstraint",
            "org.eclipse.elk.layerconstraint",
        ],
    ) {
        out.layer_constraint = parse_layer_constraint(lc).or(out.layer_constraint);
    }

    // nodeAlignment
    if let Some(PropertyValue::String(na)) = find_value(
        &by_key,
        &[
            "elk.nodealignment",
            "elk.nodeAlignment",
            "org.eclipse.elk.nodeAlignment",
            "org.eclipse.elk.nodealignment",
        ],
    ) {
        out.node_alignment = parse_node_alignment(na).or(out.node_alignment);
    }

    // contentAlignment
    if let Some(PropertyValue::String(ca)) = find_value(
        &by_key,
        &[
            "elk.contentalignment",
            "elk.contentAlignment",
            "org.eclipse.elk.contentAlignment",
            "org.eclipse.elk.contentalignment",
        ],
    ) {
        out.content_alignment = parse_content_alignment(ca).or(out.content_alignment);
    }

    // label placements
    if let Some(PropertyValue::String(v)) = find_value(
        &by_key,
        &[
            "elk.nodelabels.placement",
            "org.eclipse.elk.nodeLabels.placement",
        ],
    ) {
        out.node_label_placement = parse_node_label_placement(v).or(out.node_label_placement);
    }
    if let Some(PropertyValue::String(v)) = find_value(
        &by_key,
        &[
            "elk.portlabels.placement",
            "org.eclipse.elk.portLabels.placement",
        ],
    ) {
        out.port_label_placement = parse_port_label_placement(v).or(out.port_label_placement);
    }
    if let Some(PropertyValue::String(v)) = find_value(
        &by_key,
        &[
            "elk.edgelabels.placement",
            "org.eclipse.elk.edgeLabels.placement",
        ],
    ) {
        out.edge_label_placement = parse_edge_label_placement(v).or(out.edge_label_placement);
    }

    // padding (uniform)
    if let Some(value) = find_value(&by_key, &["elk.padding", "org.eclipse.elk.padding"]) {
        if let Some(p) = parse_padding(value) {
            out.padding = Some(p);
        }
    }

    // spacing subset
    if let Some(value) = find_value(&by_key, &["elk.spacing.nodenode", "org.eclipse.elk.spacing.nodenode"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.node_spacing = f));
        }
    }
    if let Some(value) = find_value(
        &by_key,
        &[
            "elk.spacing.nodenodebetweenlayers",
            "org.eclipse.elk.spacing.nodenodebetweenlayers",
            "org.eclipse.elk.alg.layered.spacing.nodenodebetweenlayers",
        ],
    ) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.layer_spacing = f));
        }
    }
    if let Some(value) = find_value(&by_key, &["elk.spacing.edgeedge", "org.eclipse.elk.spacing.edgeedge"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.edge_spacing = f));
        }
    }
    if let Some(value) = find_value(&by_key, &["elk.spacing.edgenode", "org.eclipse.elk.spacing.edgenode"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.edge_spacing = f));
        }
    }
    if let Some(value) = find_value(&by_key, &["elk.spacing.edgelabel", "org.eclipse.elk.spacing.edgelabel"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.label_spacing = f));
        }
    }
    if let Some(value) = find_value(&by_key, &["elk.spacing.labelnode", "org.eclipse.elk.spacing.labelnode"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.label_spacing = f));
        }
    }
    if let Some(value) = find_value(&by_key, &["elk.spacing.labellabel", "org.eclipse.elk.spacing.labellabel"]) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.label_clearance = f));
        }
    }
    if let Some(value) = find_value(
        &by_key,
        &["elk.spacing.componentcomponent", "org.eclipse.elk.spacing.componentcomponent"],
    ) {
        if let Some(f) = parse_f32(value) {
            out.spacing = Some(merge_spacing(out.spacing, |s| s.component_spacing = f));
        }
    }

    // ordering
    if let Some(value) = find_value(&by_key, &["elk.port.index", "org.eclipse.elk.port.index", "port.index"]) {
        if let Some(u) = parse_usize(value) {
            out.model_order = Some(u);
        }
    }
    if let Some(value) = find_value(
        &by_key,
        &[
            "elk.edge.bundle",
            "org.eclipse.elk.edge.bundle",
            "elk.layered.edgebundle",
        ],
    ) {
        if let Some(u) = parse_usize(value) {
            out.edge_bundle_key = Some(u as u32);
        }
    }
}

fn find_value<'a>(
    by_key: &'a BTreeMap<String, &'a PropertyValue>,
    keys: &[&str],
) -> Option<&'a PropertyValue> {
    for key in keys {
        if let Some(v) = by_key.get(&key.to_ascii_lowercase()) {
            return Some(*v);
        }
    }
    None
}

fn parse_direction(value: &str) -> Option<LayoutDirection> {
    match value.trim().to_ascii_uppercase().as_str() {
        "RIGHT" => Some(LayoutDirection::LeftToRight),
        "LEFT" => Some(LayoutDirection::RightToLeft),
        "DOWN" => Some(LayoutDirection::TopToBottom),
        "UP" => Some(LayoutDirection::BottomToTop),
        _ => None,
    }
}

fn parse_edge_routing(value: &str) -> Option<EdgeRouting> {
    match value.trim().to_ascii_uppercase().as_str() {
        "ORTHOGONAL" => Some(EdgeRouting::Orthogonal),
        "POLYLINE" | "SPLINES" => Some(EdgeRouting::Straight),
        _ => None,
    }
}

fn parse_port_constraint(value: &str) -> Option<PortConstraint> {
    match value.trim().to_ascii_uppercase().as_str() {
        "FREE" | "UNDEFINED" => Some(PortConstraint::Free),
        "FIXED_SIDE" => Some(PortConstraint::FixedSide),
        "FIXED_ORDER" | "FIXED_RATIO" => Some(PortConstraint::FixedOrder),
        "FIXED_POS" => Some(PortConstraint::FixedPosition),
        _ => None,
    }
}

fn parse_layer_constraint(value: &str) -> Option<LayerConstraint> {
    match value.trim().to_ascii_uppercase().as_str() {
        "FIRST" => Some(LayerConstraint::First),
        "LAST" => Some(LayerConstraint::Last),
        "NONE" => Some(LayerConstraint::None),
        _ => None,
    }
}

fn parse_node_alignment(value: &str) -> Option<NodeAlignment> {
    match value.trim().to_ascii_uppercase().as_str() {
        "START" => Some(NodeAlignment::Start),
        "CENTER" => Some(NodeAlignment::Center),
        "END" => Some(NodeAlignment::End),
        "BALANCED" => Some(NodeAlignment::Balanced),
        _ => None,
    }
}

fn parse_content_alignment(value: &str) -> Option<ContentAlignment> {
    match value.trim().to_ascii_uppercase().as_str() {
        "START" => Some(ContentAlignment::Start),
        "CENTER" => Some(ContentAlignment::Center),
        "END" => Some(ContentAlignment::End),
        _ => None,
    }
}

fn parse_node_label_placement(value: &str) -> Option<NodeLabelPlacement> {
    match value.trim().to_ascii_uppercase().as_str() {
        "OUTSIDE_TOP_CENTER" => Some(NodeLabelPlacement::OutsideTopCenter),
        "OUTSIDE_BOTTOM_CENTER" => Some(NodeLabelPlacement::OutsideBottomCenter),
        "INSIDE_TOP_LEFT" => Some(NodeLabelPlacement::InsideTopLeft),
        "INSIDE_TOP_CENTER" => Some(NodeLabelPlacement::InsideTopCenter),
        "INSIDE_TOP_RIGHT" => Some(NodeLabelPlacement::InsideTopRight),
        _ => None,
    }
}

fn parse_port_label_placement(value: &str) -> Option<PortLabelPlacement> {
    match value.trim().to_ascii_uppercase().as_str() {
        "INSIDE" => Some(PortLabelPlacement::Inside),
        "OUTSIDE" => Some(PortLabelPlacement::Outside),
        "NEXT_TO_PORT_IF_POSSIBLE" => Some(PortLabelPlacement::NextToPortIfPossible),
        _ => None,
    }
}

fn parse_edge_label_placement(value: &str) -> Option<EdgeLabelPlacement> {
    match value.trim().to_ascii_uppercase().as_str() {
        "HEAD" => Some(EdgeLabelPlacement::Head),
        "TAIL" => Some(EdgeLabelPlacement::Tail),
        "CENTER" => Some(EdgeLabelPlacement::Center),
        _ => None,
    }
}

fn parse_padding(value: &PropertyValue) -> Option<Padding> {
    match value {
        PropertyValue::Int(i) => Some(Padding::uniform(*i as f32)),
        PropertyValue::Float(f) => Some(Padding::uniform(*f as f32)),
        PropertyValue::String(s) => s.trim().parse::<f32>().ok().map(Padding::uniform),
        _ => None,
    }
}

fn parse_f32(value: &PropertyValue) -> Option<f32> {
    match value {
        PropertyValue::Int(i) => Some(*i as f32),
        PropertyValue::Float(f) => Some(*f as f32),
        PropertyValue::String(s) => s.trim().parse::<f32>().ok(),
        _ => None,
    }
}

fn parse_usize(value: &PropertyValue) -> Option<usize> {
    match value {
        PropertyValue::Int(i) => (*i).try_into().ok(),
        PropertyValue::Float(f) => (*f as i64).try_into().ok(),
        PropertyValue::String(s) => s.trim().parse::<usize>().ok(),
        _ => None,
    }
}

fn merge_spacing(current: Option<Spacing>, f: impl FnOnce(&mut Spacing)) -> Spacing {
    let mut s = current.unwrap_or_default();
    f(&mut s);
    s
}

