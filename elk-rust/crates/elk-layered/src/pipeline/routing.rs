use std::collections::{BTreeMap, BTreeSet};
use std::collections::HashMap;

use elk_core::{
    ContentAlignment, EdgeEndpoint, EdgeLabelPlacement, EdgeRouting, EdgeSection, Graph,
    LayoutDirection, LayoutOptions, LayoutStats, NodeId, NodeLabelPlacement, Point, PortConstraint,
    PortId, PortLabelPlacement, PortSide, Rect, Size, ViewProfile,
};

use crate::ir::{IrEdge, LayeredIr, NormalizedEdge};
use crate::pipeline::util::{
    dedup_points, ensure_orthogonal_path_prefer_major, simplify_orthogonal_points,
};
use crate::pipeline::orthogonal_routing_generator as ortho_gen;

pub(crate) fn export_to_graph(
    graph: &mut Graph,
    ir: &LayeredIr,
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
    warnings: &mut Vec<String>,
    stats: &mut LayoutStats,
) -> usize {
    let debug_enabled = std::env::var("SPEC42_ELK_DEBUG").as_deref() == Ok("1");
    let debug_profile = matches!(
        options.view_profile,
        ViewProfile::InterconnectionView | ViewProfile::GeneralView
    );
    if debug_enabled && debug_profile {
        warnings.push(format!(
            "elk-layered debug enabled (view_profile={:?}, direction={:?})",
            options.view_profile, options.layered.direction
        ));
    }
    for node in &ir.nodes {
        if let crate::ir::IrNodeKind::Real(node_id) = node.kind {
            let old_origin = graph.node(node_id).bounds.origin;
            let delta = Point::new(
                node.position.x - old_origin.x,
                node.position.y - old_origin.y,
            );
            graph.node_mut(node_id).bounds.origin = node.position;
            if delta.x != 0.0 || delta.y != 0.0 {
                translate_descendants(graph, node_id, delta);
            }
            layout_ports(graph, node_id, options, stats);
            layout_node_labels(graph, node_id, options, stats);
        }
    }

    let mut routed_segments = 0usize;
    let subgraph_bounds = graph_bounds_for_subgraph(graph, local_nodes);

    // For InterconnectionView, precompute deterministic slot assignments inspired by ELK's
    // OrthogonalRoutingGenerator, but do it per *layer gap segment* (ELK-style) using
    // `normalized_edges` instead of one segment per whole edge.
    let interconnection_slots: Option<Vec<Vec<Option<InterconnectionSlot>>>> =
        if options.layered.endpoint_fanout {
            Some(assign_interconnection_segment_slots(graph, ir, local_nodes, options))
        } else {
            None
        };

    // InterconnectionView: compute per-(port,bundle_key) stub/junction points so edges of the same
    // type at the same port overlap at the endpoint (ELK-style bundling).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct BundlePortKey {
        port: PortId,
        bundle_key: u32,
        // Distinguish source vs target endpoint so a port used on both sides doesn't mix.
        is_source: bool,
    }
    let mut bundle_stub_point: HashMap<BundlePortKey, Point> = HashMap::new();
    let mut bundle_stub_rank: HashMap<BundlePortKey, i32> = HashMap::new();
    if options.layered.endpoint_fanout {
        // First pass: collect bundle keys and representative slot per endpoint.
        let mut port_bundles: HashMap<(PortId, bool), HashMap<u32, i32>> = HashMap::new();
        for (edge_index, edge) in ir.edges.iter().enumerate() {
            if edge.self_loop {
                continue;
            }
            if !local_nodes.contains(&edge.effective_source) || !local_nodes.contains(&edge.effective_target) {
                continue;
            }
            let Some(bundle_key) = edge.bundle_key else {
                continue;
            };
            // For source endpoint use segment 0; for target use last segment.
            if let Some(port_id) = edge.source.port {
                let slot = interconnection_slots
                    .as_ref()
                    .and_then(|t| t.get(edge_index))
                    .and_then(|v| v.get(0).and_then(|s| *s))
                    .map(|s| s.slot)
                    .unwrap_or(0);
                port_bundles
                    .entry((port_id, true))
                    .or_default()
                    .entry(bundle_key)
                    .and_modify(|cur| *cur = (*cur).min(slot))
                    .or_insert(slot);
            }
            if let Some(port_id) = edge.target.port {
                let last_seg = edge_segments_for_edge(ir, edge_index).len().saturating_sub(1);
                let slot = interconnection_slots
                    .as_ref()
                    .and_then(|t| t.get(edge_index))
                    .and_then(|v| v.get(last_seg).and_then(|s| *s))
                    .map(|s| s.slot)
                    .unwrap_or(0);
                port_bundles
                    .entry((port_id, false))
                    .or_default()
                    .entry(bundle_key)
                    .and_modify(|cur| *cur = (*cur).min(slot))
                    .or_insert(slot);
            }
        }
        // Assign a compact rank per port that separates bundles, not edges within a bundle.
        for ((port_id, is_source), bundles) in port_bundles {
            let mut entries: Vec<(u32, i32)> = bundles.into_iter().collect();
            entries.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
            let center = (entries.len() as i32 - 1) / 2;
            for (idx, (bundle_key, _slot)) in entries.into_iter().enumerate() {
                let relative = idx as i32 - center;
                bundle_stub_rank.insert(
                    BundlePortKey {
                        port: port_id,
                        bundle_key,
                        is_source,
                    },
                    relative,
                );
            }
        }
    }
    for (edge_index, edge) in ir.edges.iter().enumerate() {
        if !local_nodes.contains(&edge.effective_source)
            || !local_nodes.contains(&edge.effective_target)
        {
            continue;
        }

        if edge.self_loop {
            route_self_loop(graph, edge, options);
            warnings.push(format!(
                "self-loop edge {} routed with stable loop fallback",
                edge.original_edge
            ));
            if debug_enabled && debug_profile && edge.original_edge.index() < 12 {
                warnings.push(format!(
                    "debug edge {} self_loop=true effective=({:?}->{:?}) source=(node={:?},port={:?}) target=(node={:?},port={:?})",
                    edge.original_edge,
                    edge.effective_source,
                    edge.effective_target,
                    edge.source.node,
                    edge.source.port,
                    edge.target.node,
                    edge.target.port
                ));
            }
            routed_segments += 1;
            let section = graph.edge(edge.original_edge).sections[0].clone();
            stats.bend_points += section.bend_points.len();
            place_edge_labels(graph, edge, &section, options, stats);
            continue;
        }

        let start_hint = endpoint_center(graph, edge.target);
        let end_hint = endpoint_center(graph, edge.source);
        let start = spread_anchor_point_towards(graph, ir, edge_index, edge.source, start_hint, options);
        let end = spread_anchor_point_towards(graph, ir, edge_index, edge.target, end_hint, options);
        let start_side = endpoint_anchor_side(graph, edge.source, start_hint, options);
        let end_side = endpoint_anchor_side(graph, edge.target, end_hint, options);
        let stub_len = options.layered.spacing.edge_spacing.clamp(12.0, 24.0);
        let interconnection = options.layered.endpoint_fanout;
        let fan_step = options.layered.spacing.edge_spacing.max(20.0) * 0.45;

        let routed_start = if interconnection {
            if let (Some(port_id), Some(bundle_key), Some(side)) = (
                edge.source.port,
                edge.bundle_key,
                start_side,
            ) {
                let key = BundlePortKey {
                    port: port_id,
                    bundle_key,
                    is_source: true,
                };
                *bundle_stub_point.entry(key).or_insert_with(|| {
                    let mut p = extend_from_side(start, side, stub_len);
                    let rank = bundle_stub_rank.get(&key).copied().unwrap_or(0) as f32;
                    let fan = rank * fan_step;
                    match side {
                        PortSide::East => p.x += fan,
                        PortSide::West => p.x -= fan,
                        PortSide::North => p.y -= fan,
                        PortSide::South => p.y += fan,
                    }
                    p
                })
            } else {
                start_side
                    .map(|side| extend_from_side(start, side, stub_len))
                    .unwrap_or(start)
            }
        } else {
            start_side
                .map(|side| extend_from_side(start, side, stub_len))
                .unwrap_or(start)
        };

        let routed_end = if interconnection {
            if let (Some(port_id), Some(bundle_key), Some(side)) = (
                edge.target.port,
                edge.bundle_key,
                end_side,
            ) {
                let key = BundlePortKey {
                    port: port_id,
                    bundle_key,
                    is_source: false,
                };
                *bundle_stub_point.entry(key).or_insert_with(|| {
                    let mut p = extend_from_side(end, side, stub_len);
                    let rank = bundle_stub_rank.get(&key).copied().unwrap_or(0) as f32;
                    let fan = rank * fan_step;
                    match side {
                        PortSide::East => p.x += fan,
                        PortSide::West => p.x -= fan,
                        PortSide::North => p.y -= fan,
                        PortSide::South => p.y += fan,
                    }
                    p
                })
            } else {
                end_side
                    .map(|side| extend_from_side(end, side, stub_len))
                    .unwrap_or(end)
            }
        } else {
            end_side
                .map(|side| extend_from_side(end, side, stub_len))
                .unwrap_or(end)
        };
        let bends = match edge_routing_for_edge(graph, edge, options) {
            EdgeRouting::Straight => join_with_endpoint_stubs(
                start,
                routed_start,
                straight_path(ir, edge, routed_start, routed_end),
                routed_end,
                end,
            ),
            EdgeRouting::Orthogonal => {
                let raw = orthogonal_path(
                    graph,
                    ir,
                    edge_index,
                    routed_start,
                    routed_end,
                    options,
                    interconnection_slots.as_ref(),
                    subgraph_bounds,
                );
                if debug_enabled && debug_profile && edge.original_edge.index() < 12 {
                    warnings.push(format!(
                        "debug edge {} nonloop effective=({:?}->{:?}) source=(node={:?},port={:?}) target=(node={:?},port={:?}) start_side={:?} end_side={:?} start={:?} routed_start={:?} routed_end={:?} end={:?} raw={:?}",
                        edge.original_edge,
                        edge.effective_source,
                        edge.effective_target,
                        edge.source.node,
                        edge.source.port,
                        edge.target.node,
                        edge.target.port,
                        start_side,
                        end_side,
                        start,
                        routed_start,
                        routed_end,
                        end,
                        raw
                    ));
                }
                let routed =
                    obstacle_aware_bends(graph, edge, routed_start, routed_end, raw, options);
                join_with_endpoint_stubs(start, routed_start, routed, routed_end, end)
            }
        };

        let full_path: Vec<Point> = std::iter::once(start)
            .chain(bends)
            .chain(std::iter::once(end))
            .collect();
        let orthogonal = ensure_orthogonal_path_prefer_major(full_path, options.layered.direction);
        // `ensure_orthogonal_path_*` fixes diagonal adjacency, but some upstream steps may still
        // produce non-orthogonal pairs due to numeric jitter or corner ordering. Run a final
        // enforcement pass to guarantee manhattan geometry.
        let orthogonal = crate::pipeline::util::ensure_orthogonal_path(orthogonal);
        let (start_orth, mid, end_orth) = if orthogonal.len() >= 2 {
            let start_orth = orthogonal[0];
            let end_orth = *orthogonal.last().unwrap();
            let bend_points: Vec<Point> = orthogonal[1..orthogonal.len() - 1].to_vec();
            (start_orth, bend_points, end_orth)
        } else {
            (start, vec![], end)
        };

        let bounds = subgraph_bounds;
        let margin = options.layered.spacing.edge_spacing.max(8.0);
        // When edges connect to ports, bend points often sit exactly on the port-aligned
        // boundary. Clamping with a large margin can shift bends away from the port axis and
        // reintroduce diagonal segments. Use a smaller clamp margin in that case.
        let bend_margin = if edge.source.port.is_some() || edge.target.port.is_some() {
            0.0
        } else {
            margin
        };
        // Keep port endpoints exact: clamping can shift section start/end away from the port center.
        // We still clamp bend points so detours stay on-canvas.
        let section_start = if edge.source.port.is_some() {
            start_orth
        } else {
            clamp_point_to_rect(start_orth, bounds, margin)
        };
        let section_end = if edge.target.port.is_some() {
            end_orth
        } else {
            clamp_point_to_rect(end_orth, bounds, margin)
        };
        let section = EdgeSection {
            start: section_start,
            bend_points: mid
                .into_iter()
                .map(|p| clamp_point_to_rect(p, bounds, bend_margin))
                .collect(),
            end: section_end,
        };
        // Final safety net: after clamping, enforce manhattan geometry again. Clamping can
        // move bend points off-axis relative to port endpoints.
        let mut section_points = Vec::with_capacity(section.bend_points.len() + 2);
        section_points.push(section.start);
        section_points.extend(section.bend_points.iter().copied());
        section_points.push(section.end);
        let enforced =
            crate::pipeline::util::ensure_orthogonal_path(crate::pipeline::util::dedup_points(
                section_points,
            ));
        let (start_final, bend_final, end_final) = if enforced.len() >= 2 {
            (
                enforced[0],
                enforced[1..enforced.len() - 1].to_vec(),
                *enforced.last().unwrap(),
            )
        } else {
            (section.start, Vec::new(), section.end)
        };
        let section = EdgeSection {
            start: start_final,
            bend_points: bend_final,
            end: end_final,
        };
        let section = enforce_port_perpendiculars(graph, edge, section);
        if debug_enabled && debug_profile {
            if let Some(port_id) = edge.source.port {
                let expected = graph.port(port_id).bounds.center();
                let dx = (section.start.x - expected.x).abs();
                let dy = (section.start.y - expected.y).abs();
                if dx > 1.0 || dy > 1.0 {
                    warnings.push(format!(
                        "edge {} source_port_mismatch port={:?} expected={:?} got_start={:?} (dx={:.1},dy={:.1})",
                        edge.original_edge, port_id, expected, section.start, dx, dy
                    ));
                }
            }
            if let Some(port_id) = edge.target.port {
                let expected = graph.port(port_id).bounds.center();
                let dx = (section.end.x - expected.x).abs();
                let dy = (section.end.y - expected.y).abs();
                if dx > 1.0 || dy > 1.0 {
                    warnings.push(format!(
                        "edge {} target_port_mismatch port={:?} expected={:?} got_end={:?} (dx={:.1},dy={:.1})",
                        edge.original_edge, port_id, expected, section.end, dx, dy
                    ));
                }
            }
        }
        if debug_enabled
            && options.layered.endpoint_fanout
            && section_has_diagonal(&section)
        {
            warnings.push(format!(
                "edge {} still has diagonal segment after enforcement: start={:?} first_bend={:?}",
                edge.original_edge,
                section.start,
                section.bend_points.first().copied().unwrap_or(section.end)
            ));
        }
        if debug_enabled && debug_profile && edge.original_edge.index() < 12 {
            let mut pts = Vec::with_capacity(section.bend_points.len() + 2);
            pts.push(section.start);
            pts.extend(section.bend_points.iter().copied());
            pts.push(section.end);
            warnings.push(format!(
                "debug edge {} final_section_points={:?} clamp_bounds={:?} margin={} bend_margin={}",
                edge.original_edge, pts, bounds, margin, bend_margin
            ));
        }
        let edge_mut = graph.edge_mut(edge.original_edge);
        edge_mut.was_reversed = edge.reversed;
        edge_mut.sections = vec![section.clone()];
        stats.bend_points += section.bend_points.len();
        place_edge_labels(graph, edge, &section, options, stats);
        routed_segments += edge_segments_for_edge(ir, edge_index).len().max(1);
    }

    routed_segments
}

fn assign_interconnection_segment_slots(
    graph: &Graph,
    ir: &LayeredIr,
    local_nodes: &BTreeSet<NodeId>,
    options: &LayoutOptions,
) -> Vec<Vec<Option<InterconnectionSlot>>> {
    use std::collections::{BTreeMap, HashMap};

    let direction = options.layered.direction;
    let minor_is_x = matches!(
        direction,
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop
    );
    let spacing = options.layered.spacing.segment_spacing.max(12.0);

    // Determine max segment order per edge (needed to recognize first/last segments).
    let mut max_order: Vec<usize> = vec![0; ir.edges.len()];
    for ne in &ir.normalized_edges {
        max_order[ne.edge_index] = max_order[ne.edge_index].max(ne.segment_order);
    }

    // slots[edge_index][segment_order] = slot info
    let mut slots: Vec<Vec<Option<InterconnectionSlot>>> = max_order
        .iter()
        .map(|&m| vec![None; m.saturating_add(1)])
        .collect();

    // Build per-gap segment lists. For port-incident segments, collapse by (gap, port, bundle_key)
    // so edges of the same type connected to the same port share the same slot (ELK-style bundling).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct GroupKey {
        gap: usize,
        port: PortId,
        bundle_key: u32,
        is_source: bool,
    }

    #[derive(Clone)]
    struct SegMeta {
        members: Vec<(usize, usize)>, // (edge_index, segment_order)
        segment: ortho_gen::HyperEdgeSegment,
    }

    let mut by_gap: BTreeMap<usize, Vec<SegMeta>> = BTreeMap::new();
    let mut group_index_by_gap: BTreeMap<usize, HashMap<GroupKey, usize>> = BTreeMap::new();

    for ne in &ir.normalized_edges {
        let edge = &ir.edges[ne.edge_index];
        if edge.self_loop {
            continue;
        }
        if !local_nodes.contains(&edge.effective_source) || !local_nodes.contains(&edge.effective_target) {
            continue;
        }

        let from_layer = ir.nodes[ne.from].layer;
        let to_layer = ir.nodes[ne.to].layer;
        let gap = from_layer.min(to_layer);

        // Compute endpoint coordinates on the minor axis.
        // First/last segment use actual port endpoints when available; intermediate segments use
        // dummy/real node centers.
        let is_first = ne.segment_order == 0;
        let is_last = ne.segment_order == max_order[ne.edge_index];

        let (start_pt, end_pt) = if is_first && is_last {
            (endpoint_center(graph, edge.source), endpoint_center(graph, edge.target))
        } else if is_first {
            (endpoint_center(graph, edge.source), ir.nodes[ne.to].center())
        } else if is_last {
            (ir.nodes[ne.from].center(), endpoint_center(graph, edge.target))
        } else {
            (ir.nodes[ne.from].center(), ir.nodes[ne.to].center())
        };

        let start_c = if minor_is_x { start_pt.x } else { start_pt.y };
        let end_c = if minor_is_x { end_pt.x } else { end_pt.y };

        // Decide whether to bundle this segment: only for segments incident to an actual port
        // (first/last) AND only when edge has a bundle key.
        let bundle = edge.bundle_key;
        let group_key = if is_first {
            edge.source.port.and_then(|p| bundle.map(|k| GroupKey { gap, port: p, bundle_key: k, is_source: true }))
        } else if is_last {
            edge.target.port.and_then(|p| bundle.map(|k| GroupKey { gap, port: p, bundle_key: k, is_source: false }))
        } else {
            None
        };

        if let Some(gk) = group_key {
            let idx_map = group_index_by_gap.entry(gap).or_default();
            let list = by_gap.entry(gap).or_default();
            let idx = *idx_map.entry(gk).or_insert_with(|| {
                // Create a new grouped segment meta.
                let seg = ortho_gen::HyperEdgeSegment {
                    id: 0, // reindexed per gap below
                    start_coordinate: start_c,
                    end_coordinate: end_c,
                    // Collect all opposite-side coordinates for the group.
                    incoming_connection_coordinates: vec![end_c],
                    outgoing_connection_coordinates: vec![start_c],
                    routing_slot: 0,
                    in_weight: 0,
                    out_weight: 0,
                    incoming: Vec::new(),
                    outgoing: Vec::new(),
                    split_partner: None,
                    split_by: None,
                    mark: -1,
                };
                list.push(SegMeta {
                    members: Vec::new(),
                    segment: seg,
                });
                list.len() - 1
            });

            let meta = &mut list[idx];
            meta.members.push((ne.edge_index, ne.segment_order));
            // Expand segment model to reflect all members.
            meta.segment.start_coordinate = meta.segment.start_coordinate.min(start_c);
            meta.segment.end_coordinate = meta.segment.end_coordinate.max(end_c);
            meta.segment.incoming_connection_coordinates.push(end_c);
            meta.segment.outgoing_connection_coordinates.push(start_c);
        } else {
            let seg = ortho_gen::HyperEdgeSegment {
                id: 0, // reindexed per gap below
                start_coordinate: start_c,
                end_coordinate: end_c,
                incoming_connection_coordinates: vec![end_c],
                outgoing_connection_coordinates: vec![start_c],
                routing_slot: 0,
                in_weight: 0,
                out_weight: 0,
                incoming: Vec::new(),
                outgoing: Vec::new(),
                split_partner: None,
                split_by: None,
                mark: -1,
            };
            by_gap.entry(gap).or_default().push(SegMeta {
                members: vec![(ne.edge_index, ne.segment_order)],
                segment: seg,
            });
        }
    }

    // Assign slots per gap and write into lookup table.
    for metas in by_gap.values_mut() {
        // Reindex ids densely for this gap so `assign_routing_slots` can return a compact vec.
        for (i, meta) in metas.iter_mut().enumerate() {
            meta.segment.id = i;
            meta.segment
                .incoming_connection_coordinates
                .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            meta.segment
                .outgoing_connection_coordinates
                .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        }
        let segs: Vec<ortho_gen::HyperEdgeSegment> =
            metas.iter().map(|m| m.segment.clone()).collect();
        let slot_vec = ortho_gen::assign_routing_slots(segs, spacing);
        let (min_slot, max_slot) = slot_vec.iter().fold((i32::MAX, i32::MIN), |acc, &s| {
            (acc.0.min(s), acc.1.max(s))
        });
        for meta in metas.iter() {
            let slot = slot_vec
                .get(meta.segment.id)
                .copied()
                .unwrap_or_default();
            for &(edge_index, segment_order) in &meta.members {
                if let Some(list) = slots.get_mut(edge_index) {
                    if let Some(cell) = list.get_mut(segment_order) {
                        *cell = Some(InterconnectionSlot {
                            slot,
                            min_slot,
                            max_slot,
                        });
                    }
                }
            }
        }
    }

    slots
}

#[derive(Clone, Copy, Debug)]
struct InterconnectionSlot {
    slot: i32,
    min_slot: i32,
    max_slot: i32,
}

fn enforce_port_perpendiculars(graph: &Graph, edge: &IrEdge, mut section: EdgeSection) -> EdgeSection {
    // Ensure the segment adjacent to a port is perpendicular to the port's owning side.
    // This is stricter than general Manhattan enforcement: it enforces *orientation* at ports.
    const EPS: f32 = 0.001;

    // Start (source)
    if let Some(port_id) = edge.source.port {
        let side = graph.port(port_id).side;
        let next = section.bend_points.first().copied().unwrap_or(section.end);
        let dx = (next.x - section.start.x).abs();
        let dy = (next.y - section.start.y).abs();
        let should_be_horizontal = matches!(side, PortSide::East | PortSide::West);
        let violates = if should_be_horizontal { dx <= EPS && dy > EPS } else { dy <= EPS && dx > EPS };
        if violates {
            let stub = if should_be_horizontal {
                Point::new(next.x, section.start.y)
            } else {
                Point::new(section.start.x, next.y)
            };
            if (stub.x - section.start.x).abs() > EPS || (stub.y - section.start.y).abs() > EPS {
                section.bend_points.insert(0, stub);
            }
        }
    }

    // End (target)
    if let Some(port_id) = edge.target.port {
        let side = graph.port(port_id).side;
        let prev = section.bend_points.last().copied().unwrap_or(section.start);
        let dx = (section.end.x - prev.x).abs();
        let dy = (section.end.y - prev.y).abs();
        let should_be_horizontal = matches!(side, PortSide::East | PortSide::West);
        let violates = if should_be_horizontal { dx <= EPS && dy > EPS } else { dy <= EPS && dx > EPS };
        if violates {
            let stub = if should_be_horizontal {
                Point::new(prev.x, section.end.y)
            } else {
                Point::new(section.end.x, prev.y)
            };
            if (stub.x - section.end.x).abs() > EPS || (stub.y - section.end.y).abs() > EPS {
                section.bend_points.push(stub);
            }
        }
    }

    // Final cleanup: keep it orthogonal and remove duplicates.
    let mut pts = Vec::with_capacity(section.bend_points.len() + 2);
    pts.push(section.start);
    pts.extend(section.bend_points.iter().copied());
    pts.push(section.end);
    let enforced = crate::pipeline::util::ensure_orthogonal_path(dedup_points(pts));
    if enforced.len() >= 2 {
        section.start = enforced[0];
        section.end = *enforced.last().unwrap();
        section.bend_points = enforced[1..enforced.len() - 1].to_vec();
    }
    section
}

fn section_has_diagonal(section: &EdgeSection) -> bool {
    let mut points = Vec::with_capacity(section.bend_points.len() + 2);
    points.push(section.start);
    points.extend(section.bend_points.iter().copied());
    points.push(section.end);
    points.windows(2).any(|w| (w[0].x - w[1].x).abs() > 0.1 && (w[0].y - w[1].y).abs() > 0.1)
}

fn edge_routing_for_edge(graph: &Graph, edge: &IrEdge, options: &LayoutOptions) -> EdgeRouting {
    graph
        .edge(edge.original_edge)
        .layout
        .edge_routing
        .or(graph.layout.edge_routing)
        .unwrap_or(options.layered.edge_routing)
}

fn straight_path(ir: &LayeredIr, edge: &IrEdge, start: Point, end: Point) -> Vec<Point> {
    let mut chain: Vec<Point> = edge
        .chain
        .iter()
        .map(|node_id| ir.nodes[*node_id].center())
        .collect();
    if edge.reversed {
        chain.reverse();
    }
    let mut points = vec![start];
    points.extend(chain);
    points.push(end);
    let deduped = dedup_points(points);
    if deduped.len() <= 2 {
        return Vec::new();
    }
    let simplified = simplify_orthogonal_points(deduped);
    if simplified.len() <= 2 {
        Vec::new()
    } else {
        simplified[1..simplified.len() - 1].to_vec()
    }
}

fn orthogonal_path(
    graph: &Graph,
    ir: &LayeredIr,
    edge_index: usize,
    start: Point,
    end: Point,
    options: &LayoutOptions,
    interconnection_slots: Option<&Vec<Vec<Option<InterconnectionSlot>>>>,
    bounds: Rect,
) -> Vec<Point> {
    let direction = options.layered.direction;
    let segment_spacing = options.layered.spacing.segment_spacing.max(12.0);
    let edge = &ir.edges[edge_index];
    let interconnection = options.layered.endpoint_fanout;
    let points = chain_points(ir, edge, start, end);
    let segments = edge_segments_for_edge(ir, edge_index);
    let mut bends = Vec::new();

    for (segment_index, window) in points.windows(2).enumerate() {
        // Slot-driven channel routing for InterconnectionView (ELK-inspired, per layer gap).
        if interconnection {
            let slot = interconnection_slots
                .and_then(|table| table.get(edge_index))
                .and_then(|per_edge| per_edge.get(segment_index).and_then(|v| *v));
            if let Some(slot) = slot {
                let corridor = between_layer_corridor(graph, ir, &segments, segment_index, direction, bounds);
                let channel_bends = interconnection_channel_bends(
                    window[0],
                    window[1],
                    slot,
                    direction,
                    corridor,
                    options.layered.spacing.edge_spacing.max(20.0),
                );
                bends.extend(channel_bends);
                continue;
            }
        }
        let lane = if interconnection
            && edge.effective_source == edge.effective_target
            && edge.source.port.is_some()
            && edge.target.port.is_some()
        {
            // ELK-style slot assignment (simplified): use precomputed slot if available,
            // otherwise fall back to deterministic distribution.
            let lane_id = interconnection_slots
                .and_then(|table| table.get(edge_index))
                .and_then(|per_edge| per_edge.get(segment_index).and_then(|v| *v))
                .map(|s| s.slot)
                .unwrap_or_else(|| {
                let lanes = options.layered.preferred_connector_lanes.max(2) as i32;
                let idx = (edge_index as i32).rem_euclid(lanes);
                idx - (lanes / 2)
            });
            alternating_lane_offset(lane_id, segment_spacing)
        } else if interconnection {
            // Temporary stabilization for other cases: prefer strict manhattan routing over
            // lane-offset bends until the full ELK-style slot router is implemented.
            0.0
        } else {
            segments
                .get(segment_index)
                .map(|segment| alternating_lane_offset(segment.lane, segment_spacing))
                .unwrap_or_default()
        };
        let mut segment_bends = orthogonal_segment(window[0], window[1], lane, direction);
        bends.append(&mut segment_bends);
    }

    let mut points = vec![start];
    points.extend(bends);
    points.push(end);
    let simplified = simplify_orthogonal_points(points);
    if simplified.len() <= 2 {
        Vec::new()
    } else {
        simplified[1..simplified.len() - 1].to_vec()
    }
}

fn interconnection_channel_bends(
    start: Point,
    end: Point,
    slot: InterconnectionSlot,
    direction: LayoutDirection,
    corridor: Rect,
    spacing: f32,
) -> Vec<Point> {
    match direction {
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => {
            // Route via a horizontal channel within the between-layer corridor.
            let min_y = corridor.origin.y + 2.0;
            let max_y = corridor.max_y() - 2.0;
            let usable = (max_y - min_y).max(0.0);
            let range = (slot.max_slot - slot.min_slot).abs().max(1) as f32;
            // Distribute lanes across the full corridor to avoid collapsing multiple slots
            // onto the same clamped coordinate when the corridor is tight.
            // We still respect a minimum step so lanes don't become identical due to rounding.
            let step = (usable / (range + 1.0)).max(2.0).min(spacing.max(2.0) * 3.0);
            let center_slot = (slot.min_slot + slot.max_slot) as f32 / 2.0;
            let offset = (slot.slot as f32 - center_slot) * step;
            let mid_y = corridor.origin.y + corridor.size.height / 2.0;
            let channel_y = (mid_y + offset).clamp(min_y, max_y);
            // Port escape/approach fanout is handled by the stub points in `export_to_graph`.
            dedup_points(vec![Point::new(start.x, channel_y), Point::new(end.x, channel_y)])
        }
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => {
            // Route via a vertical channel within the between-layer corridor.
            let min_x = corridor.origin.x + 2.0;
            let max_x = corridor.max_x() - 2.0;
            let usable = (max_x - min_x).max(0.0);
            let range = (slot.max_slot - slot.min_slot).abs().max(1) as f32;
            let step = (usable / (range + 1.0)).max(2.0).min(spacing.max(2.0) * 3.0);
            let center_slot = (slot.min_slot + slot.max_slot) as f32 / 2.0;
            let offset = (slot.slot as f32 - center_slot) * step;
            let mid_x = corridor.origin.x + corridor.size.width / 2.0;
            let channel_x = (mid_x + offset).clamp(min_x, max_x);
            dedup_points(vec![
                Point::new(channel_x, start.y),
                Point::new(channel_x, end.y),
            ])
        }
    }
}

fn between_layer_corridor(
    graph: &Graph,
    ir: &LayeredIr,
    segments: &[&NormalizedEdge],
    segment_index: usize,
    direction: LayoutDirection,
    fallback: Rect,
) -> Rect {
    // Determine which gap this segment traverses (best effort).
    let (from_layer, to_layer) = segments
        .get(segment_index)
        .map(|ne| (ir.nodes[ne.from].layer, ir.nodes[ne.to].layer))
        .unwrap_or_else(|| (0, 1));
    let l0 = from_layer.min(to_layer);
    let l1 = l0.saturating_add(1);
    if l0 >= ir.layers.len() || l1 >= ir.layers.len() {
        return fallback;
    }

    // Compute a corridor rectangle between the two adjacent layers along the major axis.
    // TTB/BTB: corridor is horizontal strip between bottom(layer0) and top(layer1).
    // LTR/RTL: corridor is vertical strip between right(layer0) and left(layer1).
    let mut layer0_extents: Option<(f32, f32)> = None;
    let mut layer1_extents: Option<(f32, f32)> = None;

    for &ir_id in &ir.layers[l0] {
        if let crate::ir::IrNodeKind::Real(node_id) = ir.nodes[ir_id].kind {
            let b = graph.node(node_id).bounds;
            let (start, end) = match direction {
                LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => (b.origin.y, b.max_y()),
                LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => (b.origin.x, b.max_x()),
            };
            layer0_extents = Some(match layer0_extents {
                None => (start, end),
                Some((s, e)) => (s.min(start), e.max(end)),
            });
        }
    }
    for &ir_id in &ir.layers[l1] {
        if let crate::ir::IrNodeKind::Real(node_id) = ir.nodes[ir_id].kind {
            let b = graph.node(node_id).bounds;
            let (start, end) = match direction {
                LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => (b.origin.y, b.max_y()),
                LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => (b.origin.x, b.max_x()),
            };
            layer1_extents = Some(match layer1_extents {
                None => (start, end),
                Some((s, e)) => (s.min(start), e.max(end)),
            });
        }
    }

    let (_l0_start, l0_end) = layer0_extents.unwrap_or((fallback.origin.y, fallback.max_y()));
    let (l1_start, _l1_end) = layer1_extents.unwrap_or((fallback.origin.y, fallback.max_y()));

    match direction {
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => {
            let y0 = l0_end;
            let y1 = l1_start;
            if y1 <= y0 + 1.0 {
                return fallback;
            }
            Rect::new(
                Point::new(fallback.origin.x, y0),
                Size::new(fallback.size.width, y1 - y0),
            )
        }
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => {
            let x0 = l0_end;
            let x1 = l1_start;
            if x1 <= x0 + 1.0 {
                return fallback;
            }
            Rect::new(
                Point::new(x0, fallback.origin.y),
                Size::new(x1 - x0, fallback.size.height),
            )
        }
    }
}

fn chain_points(ir: &LayeredIr, edge: &IrEdge, start: Point, end: Point) -> Vec<Point> {
    let mut points = vec![start];
    let mut chain: Vec<Point> = edge
        .chain
        .iter()
        .map(|node_id| ir.nodes[*node_id].center())
        .collect();
    if edge.reversed {
        chain.reverse();
    }
    points.extend(chain);
    points.push(end);
    points
}

fn edge_segments_for_edge(ir: &LayeredIr, edge_index: usize) -> Vec<&NormalizedEdge> {
    let mut segments: Vec<_> = ir
        .normalized_edges
        .iter()
        .filter(|segment| segment.edge_index == edge_index)
        .collect();
    segments.sort_by_key(|segment| segment.segment_order);
    if ir.edges[edge_index].reversed {
        segments.reverse();
    }
    segments
}

fn orthogonal_segment(
    start: Point,
    end: Point,
    lane_offset: f32,
    direction: LayoutDirection,
) -> Vec<Point> {
    if (start.x - end.x).abs() <= f32::EPSILON || (start.y - end.y).abs() <= f32::EPSILON {
        return Vec::new();
    }

    let horizontal_major = matches!(
        direction,
        LayoutDirection::LeftToRight | LayoutDirection::RightToLeft
    );

    if lane_offset.abs() <= f32::EPSILON {
        let simple = if horizontal_major {
            vec![Point::new(end.x, start.y)]
        } else {
            vec![Point::new(start.x, end.y)]
        };
        return dedup_points(simple);
    }

    let mut bends = Vec::new();
    if horizontal_major {
        let preferred_minor = (start.y + end.y) / 2.0 + lane_offset;
        bends.push(Point::new(start.x, preferred_minor));
        bends.push(Point::new(end.x, preferred_minor));
    } else {
        let preferred_minor = (start.x + end.x) / 2.0 + lane_offset;
        bends.push(Point::new(preferred_minor, start.y));
        bends.push(Point::new(preferred_minor, end.y));
    }

    dedup_points(bends)
}

fn alternating_lane_offset(lane: i32, spacing: f32) -> f32 {
    if lane == 0 {
        return 0.0;
    }

    let normalized = lane.unsigned_abs() as usize;
    let magnitude = normalized.div_ceil(2) as f32 * spacing;
    if normalized % 2 == 0 {
        -magnitude
    } else {
        magnitude
    }
}

fn join_with_endpoint_stubs(
    start: Point,
    routed_start: Point,
    bends: Vec<Point>,
    routed_end: Point,
    end: Point,
) -> Vec<Point> {
    let mut points = vec![start];
    if !same_point(start, routed_start) {
        points.push(routed_start);
    }
    points.extend(bends);
    if !same_point(routed_end, end) {
        points.push(routed_end);
    }
    points.push(end);
    let simplified = simplify_orthogonal_points(dedup_points(points));
    if simplified.len() <= 2 {
        Vec::new()
    } else {
        simplified[1..simplified.len() - 1].to_vec()
    }
}

fn obstacle_aware_bends(
    graph: &Graph,
    edge: &IrEdge,
    start: Point,
    end: Point,
    bends: Vec<Point>,
    options: &LayoutOptions,
) -> Vec<Point> {
    let mut points = vec![start];
    points.extend(bends.iter().copied());
    points.push(end);

    let endpoint_nodes = [
        endpoint_obstacle_node(graph, edge.source),
        endpoint_obstacle_node(graph, edge.target),
        endpoint_obstacle_node(graph, edge.routed_source),
        endpoint_obstacle_node(graph, edge.routed_target),
    ];

    for (segment_index, window) in points.windows(2).enumerate() {
        for node in &graph.nodes {
            if endpoint_nodes.contains(&node.id) {
                continue;
            }
            if segment_intersects_rect(
                window[0],
                window[1],
                inflate_rect(node.bounds, options.layered.spacing.edge_spacing),
            ) {
                let mut rerouted = splice_detour(
                    &points,
                    segment_index,
                    detour_around_rect(graph, &endpoint_nodes, window[0], window[1], node.bounds, options),
                );
                // Try a few additional local detours; the interconnection view often needs
                // multiple obstacle deflections to keep routes inside free space.
                let mut budget = 6usize;
                while budget > 0 {
                    budget -= 1;
                    let mut changed = false;
                    'outer: for (seg_idx, seg) in rerouted.windows(2).enumerate() {
                        for cand in &graph.nodes {
                            if endpoint_nodes.contains(&cand.id) {
                                continue;
                            }
                            if segment_intersects_rect(
                                seg[0],
                                seg[1],
                                inflate_rect(
                                    cand.bounds,
                                    options.layered.spacing.edge_spacing,
                                ),
                            ) {
                                rerouted = splice_detour(
                                    &rerouted,
                                    seg_idx,
                                    detour_around_rect(graph, &endpoint_nodes, seg[0], seg[1], cand.bounds, options),
                                );
                                changed = true;
                                break 'outer;
                            }
                        }
                    }
                    if !changed {
                        break;
                    }
                }
                return rerouted;
            }
        }

        for label in &graph.labels {
            let rect = Rect::new(label.position, label.size);
            if segment_intersects_rect(
                window[0],
                window[1],
                inflate_rect(rect, options.layered.spacing.label_clearance),
            ) {
                let detour_point = Point::new(
                    rect.max_x() + options.layered.spacing.label_clearance,
                    rect.max_y() + options.layered.spacing.label_clearance,
                );
                return splice_detour(
                    &points,
                    segment_index,
                    dedup_points(vec![
                        Point::new(detour_point.x, window[0].y),
                        Point::new(detour_point.x, detour_point.y),
                        Point::new(window[1].x, detour_point.y),
                    ]),
                );
            }
        }
    }

    let mut points = vec![start];
    points.extend(bends);
    points.push(end);
    let enforced = crate::pipeline::util::ensure_orthogonal_path(dedup_points(points));
    let simplified = simplify_orthogonal_points(enforced);
    if simplified.len() <= 2 {
        Vec::new()
    } else {
        simplified[1..simplified.len() - 1].to_vec()
    }
}

fn endpoint_obstacle_node(graph: &Graph, endpoint: EdgeEndpoint) -> NodeId {
    if let Some(port_id) = endpoint.port {
        graph.port(port_id).node
    } else {
        endpoint.node
    }
}

fn splice_detour(points: &[Point], segment_index: usize, detour: Vec<Point>) -> Vec<Point> {
    let mut rerouted = Vec::with_capacity(points.len() + detour.len());
    rerouted.extend_from_slice(&points[..=segment_index]);
    rerouted.extend(detour);
    rerouted.extend_from_slice(&points[segment_index + 1..]);
    let rerouted = crate::pipeline::util::ensure_orthogonal_path(dedup_points(rerouted));
    let rerouted = simplify_orthogonal_points(rerouted);
    if rerouted.len() <= 2 {
        Vec::new()
    } else {
        rerouted[1..rerouted.len() - 1].to_vec()
    }
}

fn detour_around_rect(
    graph: &Graph,
    endpoint_nodes: &[NodeId; 4],
    start: Point,
    end: Point,
    rect: Rect,
    options: &LayoutOptions,
) -> Vec<Point> {
    let gap = options.layered.spacing.edge_spacing.max(20.0);
    let direction = options.layered.direction;

    let obstacle_margin = options.layered.spacing.edge_spacing;
    let obstacles = || {
        graph
            .nodes
            .iter()
            .filter(|node| !endpoint_nodes.contains(&node.id))
            .map(|node| inflate_rect(node.bounds, obstacle_margin))
            .collect::<Vec<_>>()
    };

    if matches!(
        direction,
        LayoutDirection::TopToBottom | LayoutDirection::BottomToTop
    ) {
        // Interconnection diagrams are dense; large detours tend to create many additional
        // intrusions. Prefer *local* detours near the current segment.
        let mut detour_y = if end.y <= rect.origin.y {
            rect.origin.y - gap
        } else {
            rect.max_y() + gap
        };
        let local_min = start.y.min(end.y) - gap * 3.0;
        let local_max = start.y.max(end.y) + gap * 3.0;
        detour_y = detour_y.clamp(local_min, local_max);
        let candidates = [
            vec![
                Point::new(rect.origin.x - gap, start.y),
                Point::new(rect.origin.x - gap, detour_y),
                Point::new(end.x, detour_y),
            ],
            vec![
                Point::new(rect.max_x() + gap, start.y),
                Point::new(rect.max_x() + gap, detour_y),
                Point::new(end.x, detour_y),
            ],
        ];
        return best_detour_avoiding_obstacles(candidates, start, end, &obstacles());
    }

    let detour_x = if end.x <= rect.origin.x {
        rect.origin.x - gap
    } else {
        rect.max_x() + gap
    };
    // Same idea for LTR/RTL: clamp detour x locally to avoid huge escapes.
    let local_min = start.x.min(end.x) - gap * 3.0;
    let local_max = start.x.max(end.x) + gap * 3.0;
    let detour_x = detour_x.clamp(local_min, local_max);
    let candidates = [
        vec![
            Point::new(start.x, rect.origin.y - gap),
            Point::new(detour_x, rect.origin.y - gap),
            Point::new(detour_x, end.y),
        ],
        vec![
            Point::new(start.x, rect.max_y() + gap),
            Point::new(detour_x, rect.max_y() + gap),
            Point::new(detour_x, end.y),
        ],
    ];
    best_detour_avoiding_obstacles(candidates, start, end, &obstacles())
}

fn best_detour_avoiding_obstacles<const N: usize>(
    candidates: [Vec<Point>; N],
    start: Point,
    end: Point,
    obstacles: &[Rect],
) -> Vec<Point> {
    candidates
        .into_iter()
        .map(dedup_points)
        .min_by(|left, right| {
            let left_score = detour_score(left, start, end, obstacles);
            let right_score = detour_score(right, start, end, obstacles);
            left_score.cmp(&right_score)
        })
        .unwrap_or_default()
}

fn detour_score(points: &[Point], start: Point, end: Point, obstacles: &[Rect]) -> (usize, i32, i32) {
    let mut route = Vec::with_capacity(points.len() + 2);
    route.push(start);
    route.extend(points.iter().copied());
    route.push(end);

    let mut hits = 0usize;
    for seg in route.windows(2) {
        for rect in obstacles {
            if segment_intersects_rect(seg[0], seg[1], *rect) {
                hits += 1;
            }
        }
    }

    // Prefer fewer intersections first, then fewer bends, then shorter routes.
    let bends = route.len().saturating_sub(2) as i32;
    let length = route_length(&route).round() as i32;
    (hits, bends, length)
}

fn route_length(points: &[Point]) -> f32 {
    points
        .windows(2)
        .map(|window| (window[1].x - window[0].x).abs() + (window[1].y - window[0].y).abs())
        .sum()
}

fn place_edge_labels(
    graph: &mut Graph,
    edge: &IrEdge,
    section: &EdgeSection,
    options: &LayoutOptions,
    stats: &mut LayoutStats,
) {
    if edge.label_ids.is_empty() {
        return;
    }

    let (base, horizontal) = best_label_anchor(section);
    for (index, label_id) in edge.label_ids.iter().enumerate() {
        let placement = graph.labels[label_id.index()]
            .layout
            .edge_label_placement
            .unwrap_or(EdgeLabelPlacement::Center);
        let spacing = options.layered.spacing.label_spacing;
        let clearance = options.layered.spacing.label_clearance;
        let size = graph.labels[label_id.index()].size;
        let stacked = index as f32 * (size.height + 4.0);
        let mut position = match placement {
            EdgeLabelPlacement::Head => Point::new(
                section.start.x + spacing,
                section.start.y - size.height - spacing,
            ),
            EdgeLabelPlacement::Tail => Point::new(
                section.end.x - size.width - spacing,
                section.end.y + spacing,
            ),
            EdgeLabelPlacement::Center => {
                if horizontal {
                    Point::new(
                        base.x - size.width / 2.0,
                        base.y - size.height - spacing - stacked,
                    )
                } else {
                    Point::new(base.x + spacing, base.y - size.height / 2.0 + stacked)
                }
            }
        };

        for _ in 0..10 {
            let rect = inflate_rect(Rect::new(position, size), clearance);
            let node_clear = graph.nodes.iter().all(|node| !rect.intersects(node.bounds));
            let label_clear = graph
                .labels
                .iter()
                .filter(|other| other.id != *label_id)
                .all(|other| !rect.intersects(Rect::new(other.position, other.size)));
            if node_clear && label_clear {
                break;
            }
            position.y -= size.height + clearance;
            position.x += if horizontal { 0.0 } else { clearance / 2.0 };
        }

        graph.labels[label_id.index()].position = position;
        stats.label_displacements += distance(position, base);
    }
}

fn best_label_anchor(section: &EdgeSection) -> (Point, bool) {
    let mut points = vec![section.start];
    points.extend(section.bend_points.iter().copied());
    points.push(section.end);

    let mut best = (
        0.0f32,
        Point::new(
            (section.start.x + section.end.x) / 2.0,
            (section.start.y + section.end.y) / 2.0,
        ),
        true,
    );
    for window in points.windows(2) {
        let dx = (window[0].x - window[1].x).abs();
        let dy = (window[0].y - window[1].y).abs();
        let length = dx + dy;
        if length > best.0 {
            best = (
                length,
                Point::new(
                    (window[0].x + window[1].x) / 2.0,
                    (window[0].y + window[1].y) / 2.0,
                ),
                dx >= dy,
            );
        }
    }
    (best.1, best.2)
}

fn translate_descendants(graph: &mut Graph, parent: NodeId, delta: Point) {
    // When laying out compound graphs, child subgraphs may already have routed edges with
    // absolute section coordinates. If we later translate the whole subtree (e.g. because the
    // parent moved), we must translate those edge sections too; otherwise all edges appear with a
    // constant (dx, dy) offset relative to the moved nodes/ports.
    fn translate_edge_sections(graph: &mut Graph, node_set: &BTreeSet<NodeId>, delta: Point) {
        for edge in &mut graph.edges {
            if !node_set.contains(&edge.source.node) || !node_set.contains(&edge.target.node) {
                continue;
            }
            for section in &mut edge.sections {
                section.start.x += delta.x;
                section.start.y += delta.y;
                for p in &mut section.bend_points {
                    p.x += delta.x;
                    p.y += delta.y;
                }
                section.end.x += delta.x;
                section.end.y += delta.y;
            }
            for label_id in edge.labels.clone() {
                graph.labels[label_id.index()].position.x += delta.x;
                graph.labels[label_id.index()].position.y += delta.y;
            }
        }
    }

    fn collect_descendant_nodes(graph: &Graph, root: NodeId, out: &mut BTreeSet<NodeId>) {
        for child in graph.children_of(root) {
            if out.insert(*child) {
                collect_descendant_nodes(graph, *child, out);
            }
        }
    }

    fn translate_descendants_nodes_only(graph: &mut Graph, parent: NodeId, delta: Point) {
        let children = graph.children_of(parent).to_vec();
        for child in children {
            let (ports, labels) = {
                let node = graph.node_mut(child);
                node.bounds.origin.x += delta.x;
                node.bounds.origin.y += delta.y;
                (node.ports.clone(), node.labels.clone())
            };
            for port_id in ports {
                let label_ids = {
                    let port = graph.port_mut(port_id);
                    port.bounds.origin.x += delta.x;
                    port.bounds.origin.y += delta.y;
                    port.labels.clone()
                };
                for label_id in label_ids {
                    graph.labels[label_id.index()].position.x += delta.x;
                    graph.labels[label_id.index()].position.y += delta.y;
                }
            }
            for label_id in labels {
                graph.labels[label_id.index()].position.x += delta.x;
                graph.labels[label_id.index()].position.y += delta.y;
            }
            translate_descendants_nodes_only(graph, child, delta);
        }
    }

    // Translate edges exactly once for this subtree move (including sibling-to-sibling edges),
    // then translate nodes/ports/labels recursively. Avoid translating edges again in recursion,
    // which would produce a constant extra offset (e.g. +72,+72) for intra-subtree edges.
    let mut subtree: BTreeSet<NodeId> = BTreeSet::new();
    subtree.insert(parent);
    collect_descendant_nodes(graph, parent, &mut subtree);
    translate_edge_sections(graph, &subtree, delta);
    translate_descendants_nodes_only(graph, parent, delta);

    // Note: recursion is handled by `translate_descendants_nodes_only`.
}

fn layout_ports(
    graph: &mut Graph,
    node_id: NodeId,
    options: &LayoutOptions,
    stats: &mut LayoutStats,
) {
    let bounds = graph.node(node_id).bounds;
    let port_ids = graph.node(node_id).ports.clone();
    let node_layout = graph
        .node(node_id)
        .layout
        .clone()
        .inherit_from(&graph.layout);
    let respect_port_order = node_layout
        .respect_port_order
        .unwrap_or(options.layered.respect_port_order);

    let mut grouped: BTreeMap<PortSide, Vec<PortId>> = BTreeMap::new();
    for port_id in port_ids {
        let side = graph.port(port_id).side;
        grouped.entry(side).or_default().push(port_id);
    }

    for (side, mut ports) in grouped {
        if !respect_port_order {
            ports.sort_by_key(|port_id| {
                graph
                    .port(*port_id)
                    .layout
                    .model_order
                    .unwrap_or(port_id.index())
            });
        }
        let count = ports.len().max(1) as f32;
        for (index, port_id) in ports.into_iter().enumerate() {
            let port_layout = graph
                .port(port_id)
                .layout
                .clone()
                .inherit_from(&node_layout);
            if port_layout.port_constraint == Some(PortConstraint::FixedPosition) {
                layout_port_labels(graph, port_id, options, stats);
                continue;
            }
            let fraction = (index as f32 + 1.0) / (count + 1.0);
            let port = graph.port_mut(port_id);
            let size = port.bounds.size;
            port.bounds.origin = match side {
                PortSide::North => Point::new(
                    bounds.origin.x + bounds.size.width * fraction - size.width / 2.0,
                    bounds.origin.y - size.height / 2.0,
                ),
                PortSide::South => Point::new(
                    bounds.origin.x + bounds.size.width * fraction - size.width / 2.0,
                    bounds.max_y() - size.height / 2.0,
                ),
                PortSide::East => Point::new(
                    bounds.max_x() - size.width / 2.0,
                    bounds.origin.y + bounds.size.height * fraction - size.height / 2.0,
                ),
                PortSide::West => Point::new(
                    bounds.origin.x - size.width / 2.0,
                    bounds.origin.y + bounds.size.height * fraction - size.height / 2.0,
                ),
            };
            layout_port_labels(graph, port_id, options, stats);
        }
    }
}

fn layout_node_labels(
    graph: &mut Graph,
    node_id: NodeId,
    options: &LayoutOptions,
    stats: &mut LayoutStats,
) {
    let bounds = graph.node(node_id).bounds;
    let node_layout = graph
        .node(node_id)
        .layout
        .clone()
        .inherit_from(&graph.layout);
    let spacing = node_layout
        .spacing
        .unwrap_or(options.layered.spacing)
        .label_spacing;
    let labels = graph.node(node_id).labels.clone();
    if labels.is_empty() {
        return;
    }

    let total_height: f32 = labels
        .iter()
        .map(|label_id| graph.labels[label_id.index()].size.height)
        .sum();
    let content_alignment = node_layout
        .content_alignment
        .unwrap_or(ContentAlignment::Center);
    let inside_start_y = match content_alignment {
        ContentAlignment::Start => bounds.origin.y + spacing,
        ContentAlignment::Center => bounds.origin.y + (bounds.size.height - total_height) / 2.0,
        ContentAlignment::End => bounds.max_y() - total_height - spacing,
    };

    let mut cursor_y = 0.0;
    for label_id in labels {
        let placement = graph.labels[label_id.index()]
            .layout
            .node_label_placement
            .or(node_layout.node_label_placement)
            .unwrap_or(NodeLabelPlacement::OutsideTopCenter);
        let label = &mut graph.labels[label_id.index()];
        let anchor = Point::new(bounds.origin.x + bounds.size.width / 2.0, bounds.origin.y);
        label.position = match placement {
            NodeLabelPlacement::InsideTopLeft => Point::new(
                bounds.origin.x + spacing,
                bounds.origin.y + spacing + cursor_y,
            ),
            NodeLabelPlacement::InsideTopCenter => Point::new(
                bounds.origin.x + (bounds.size.width - label.size.width) / 2.0,
                bounds.origin.y + spacing + cursor_y,
            ),
            NodeLabelPlacement::InsideTopRight => Point::new(
                bounds.max_x() - label.size.width - spacing,
                bounds.origin.y + spacing + cursor_y,
            ),
            NodeLabelPlacement::OutsideBottomCenter => Point::new(
                bounds.origin.x + (bounds.size.width - label.size.width) / 2.0,
                bounds.max_y() + spacing + cursor_y,
            ),
            NodeLabelPlacement::OutsideTopCenter => Point::new(
                bounds.origin.x + (bounds.size.width - label.size.width) / 2.0,
                bounds.origin.y - label.size.height - spacing - cursor_y,
            ),
        };
        if matches!(
            placement,
            NodeLabelPlacement::InsideTopLeft
                | NodeLabelPlacement::InsideTopCenter
                | NodeLabelPlacement::InsideTopRight
        ) {
            label.position.y = inside_start_y + cursor_y;
        }
        stats.label_displacements += distance(label.position, anchor);
        cursor_y += label.size.height + spacing;
    }
}

fn layout_port_labels(
    graph: &mut Graph,
    port_id: PortId,
    options: &LayoutOptions,
    stats: &mut LayoutStats,
) {
    let port = graph.port(port_id).clone();
    let labels = port.labels.clone();
    if labels.is_empty() {
        return;
    }
    let spacing = options.layered.spacing.port_label_spacing;
    let mut cursor = 0.0;
    for label_id in labels {
        let placement = graph.labels[label_id.index()]
            .layout
            .port_label_placement
            .or(port.layout.port_label_placement)
            .unwrap_or(PortLabelPlacement::Outside);
        let label = &mut graph.labels[label_id.index()];
        let anchor = port.bounds.center();
        label.position = match placement {
            PortLabelPlacement::Inside => Point::new(
                port.bounds.origin.x + (port.bounds.size.width - label.size.width) / 2.0,
                port.bounds.origin.y + (port.bounds.size.height - label.size.height) / 2.0 + cursor,
            ),
            PortLabelPlacement::NextToPortIfPossible | PortLabelPlacement::Outside => {
                match port.side {
                    PortSide::North => Point::new(
                        port.bounds.origin.x,
                        port.bounds.origin.y - label.size.height - spacing - cursor,
                    ),
                    PortSide::South => {
                        Point::new(port.bounds.origin.x, port.bounds.max_y() + spacing + cursor)
                    }
                    PortSide::East => {
                        Point::new(port.bounds.max_x() + spacing, port.bounds.origin.y + cursor)
                    }
                    PortSide::West => Point::new(
                        port.bounds.origin.x - label.size.width - spacing,
                        port.bounds.origin.y + cursor,
                    ),
                }
            }
        };
        stats.label_displacements += distance(label.position, anchor);
        cursor += label.size.height + spacing;
    }
}

fn spread_anchor_point_towards(
    graph: &Graph,
    ir: &LayeredIr,
    edge_index: usize,
    endpoint: EdgeEndpoint,
    toward: Point,
    options: &LayoutOptions,
) -> Point {
    if let Some(port_id) = endpoint.port {
        return graph.port(port_id).bounds.center();
    }
    let bounds = endpoint_node_bounds(graph, endpoint.node);
    let side = choose_anchor_side(bounds, toward, options);
    let offset = incident_anchor_offset(graph, ir, edge_index, endpoint.node, side, options);
    base_anchor_point(bounds, side, offset)
}

fn endpoint_anchor_side(
    graph: &Graph,
    endpoint: EdgeEndpoint,
    toward: Point,
    options: &LayoutOptions,
) -> Option<PortSide> {
    if let Some(port_id) = endpoint.port {
        // Always exit/enter perpendicular to the port boundary.
        return Some(graph.port(port_id).side);
    }
    Some(choose_anchor_side(
        endpoint_node_bounds(graph, endpoint.node),
        toward,
        options,
    ))
}

fn choose_anchor_side(bounds: Rect, toward: Point, options: &LayoutOptions) -> PortSide {
    let center = bounds.center();
    let dx = toward.x - center.x;
    let dy = toward.y - center.y;

    if options.layered.anchor_side_by_delta {
        if dx.abs() >= dy.abs() {
            if dx >= 0.0 {
                PortSide::East
            } else {
                PortSide::West
            }
        } else if dy >= 0.0 {
            PortSide::South
        } else {
            PortSide::North
        }
    } else if dy >= 0.0 {
        PortSide::South
    } else {
        PortSide::North
    }
}

fn incident_anchor_offset(
    graph: &Graph,
    ir: &LayeredIr,
    edge_index: usize,
    node_id: NodeId,
    side: PortSide,
    options: &LayoutOptions,
) -> f32 {
    let bounds = endpoint_node_bounds(graph, node_id);
    let span = if matches!(side, PortSide::East | PortSide::West) {
        bounds.size.height
    } else {
        bounds.size.width
    };
    let spacing = 12.0f32;

    let mut incident: Vec<(usize, f32, Option<u32>)> = ir
        .edges
        .iter()
        .enumerate()
        .filter_map(|(index, edge)| {
            let endpoint = if edge.source.node == node_id {
                Some(edge.target)
            } else if edge.target.node == node_id {
                Some(edge.source)
            } else {
                None
            }?;
            if endpoint.port.is_some() {
                return None;
            }
            let toward = endpoint_center(graph, endpoint);
            let candidate_side = choose_anchor_side(bounds, toward, options);
            if candidate_side != side {
                return None;
            }
            let sort_key = if matches!(side, PortSide::East | PortSide::West) {
                toward.y
            } else {
                toward.x
            };
            Some((index, sort_key, edge.bundle_key))
        })
        .collect();

    if incident.len() <= 1 {
        return 0.0;
    }

    incident.sort_by(|left, right| {
        let left_group = (left.2, left.2.map(|_| 0).unwrap_or(left.0));
        let right_group = (right.2, right.2.map(|_| 0).unwrap_or(right.0));
        left_group
            .cmp(&right_group)
            .then_with(|| {
                left.1
                    .partial_cmp(&right.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| left.0.cmp(&right.0))
    });

    let mut group_of_edge: Option<i32> = None;
    let mut current_group_key: (Option<u32>, usize) = (None, usize::MAX);
    let mut group_index: i32 = -1;
    for (idx, _sort_key, bundle_key) in &incident {
        let group_key = (*bundle_key, bundle_key.map(|_| 0).unwrap_or(*idx));
        if current_group_key != group_key {
            current_group_key = group_key;
            group_index += 1;
        }
        if *idx == edge_index {
            group_of_edge = Some(group_index);
            break;
        }
    }
    let Some(edge_group) = group_of_edge else {
        return 0.0;
    };
    let total_groups = group_index + 1;
    let center_group = (total_groups as f32 - 1.0) / 2.0;
    let raw = (edge_group as f32 - center_group) * spacing;
    let limit = (span / 2.0 - 10.0).max(spacing);
    raw.clamp(-limit, limit)
}

fn base_anchor_point(bounds: Rect, side: PortSide, offset: f32) -> Point {
    let center = bounds.center();
    match side {
        PortSide::East => Point::new(
            bounds.max_x(),
            clamp(bounds.origin.y, bounds.max_y(), center.y + offset),
        ),
        PortSide::West => Point::new(
            bounds.origin.x,
            clamp(bounds.origin.y, bounds.max_y(), center.y + offset),
        ),
        PortSide::North => Point::new(
            clamp(bounds.origin.x, bounds.max_x(), center.x + offset),
            bounds.origin.y,
        ),
        PortSide::South => Point::new(
            clamp(bounds.origin.x, bounds.max_x(), center.x + offset),
            bounds.max_y(),
        ),
    }
}

fn extend_from_side(point: Point, side: PortSide, distance: f32) -> Point {
    match side {
        PortSide::East => Point::new(point.x + distance, point.y),
        PortSide::West => Point::new(point.x - distance, point.y),
        PortSide::North => Point::new(point.x, point.y - distance),
        PortSide::South => Point::new(point.x, point.y + distance),
    }
}

fn clamp(min: f32, max: f32, value: f32) -> f32 {
    value.max(min).min(max)
}

fn same_point(left: Point, right: Point) -> bool {
    (left.x - right.x).abs() <= 0.1 && (left.y - right.y).abs() <= 0.1
}

fn endpoint_node_bounds(graph: &Graph, node_id: NodeId) -> Rect {
    let node = graph.node(node_id);
    let origin = if let Some(parent) = node.parent {
        let parent_bounds = endpoint_node_bounds(graph, parent);
        let local_origin = node.preferred_position.unwrap_or(node.bounds.origin);
        Point::new(
            parent_bounds.origin.x + local_origin.x,
            parent_bounds.origin.y + local_origin.y,
        )
    } else {
        node.bounds.origin
    };
    Rect::new(origin, node.bounds.size)
}

fn endpoint_center(graph: &Graph, endpoint: EdgeEndpoint) -> Point {
    if let Some(port_id) = endpoint.port {
        graph.port(port_id).bounds.center()
    } else {
        endpoint_node_bounds(graph, endpoint.node).center()
    }
}

fn route_self_loop(graph: &mut Graph, edge: &IrEdge, options: &LayoutOptions) {
    let node_bounds = graph.node(edge.routed_source.node).bounds;
    let start = anchor_point_towards(
        graph,
        edge.routed_source,
        Point::new(node_bounds.max_x() + 1.0, node_bounds.origin.y),
        options,
    );
    let end = anchor_point_towards(
        graph,
        edge.routed_target,
        Point::new(node_bounds.origin.x, node_bounds.origin.y - 1.0),
        options,
    );
    let gap = options.layered.spacing.edge_spacing.max(18.0);
    let loop_right = Point::new(
        node_bounds.max_x() + gap,
        node_bounds.origin.y + node_bounds.size.height / 4.0,
    );
    let loop_top = Point::new(node_bounds.max_x() + gap, node_bounds.origin.y - gap);
    let loop_left = Point::new(
        node_bounds.origin.x + node_bounds.size.width / 4.0,
        node_bounds.origin.y - gap,
    );
    graph.edge_mut(edge.original_edge).sections = vec![EdgeSection {
        start,
        bend_points: vec![loop_right, loop_top, loop_left],
        end,
    }];
}

fn anchor_point_towards(
    graph: &Graph,
    endpoint: EdgeEndpoint,
    toward: Point,
    options: &LayoutOptions,
) -> Point {
    if let Some(port_id) = endpoint.port {
        return graph.port(port_id).bounds.center();
    }
    let bounds = endpoint_node_bounds(graph, endpoint.node);
    let side = choose_anchor_side(bounds, toward, options);
    base_anchor_point(bounds, side, 0.0)
}

fn inflate_rect(rect: Rect, amount: f32) -> Rect {
    Rect::new(
        Point::new(rect.origin.x - amount, rect.origin.y - amount),
        Size::new(
            rect.size.width + amount * 2.0,
            rect.size.height + amount * 2.0,
        ),
    )
}

/// Bounding rect of the current subgraph (local roots + descendants + their ports).
fn graph_bounds_for_subgraph(graph: &Graph, local_nodes: &BTreeSet<NodeId>) -> Rect {
    let mut stack: Vec<NodeId> = local_nodes.iter().copied().collect();
    let mut seen: BTreeSet<NodeId> = BTreeSet::new();
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    while let Some(node_id) = stack.pop() {
        if !seen.insert(node_id) {
            continue;
        }
        let node = graph.node(node_id);
        let r = node.bounds;
        min_x = min_x.min(r.origin.x);
        min_y = min_y.min(r.origin.y);
        max_x = max_x.max(r.max_x());
        max_y = max_y.max(r.max_y());

        for child in graph.children_of(node_id) {
            stack.push(*child);
        }
        for port_id in &node.ports {
            let pr = graph.port(*port_id).bounds;
            min_x = min_x.min(pr.origin.x);
            min_y = min_y.min(pr.origin.y);
            max_x = max_x.max(pr.max_x());
            max_y = max_y.max(pr.max_y());
        }
    }

    if min_x <= max_x && min_y <= max_y {
        Rect::new(
            Point::new(min_x, min_y),
            Size::new(max_x - min_x, max_y - min_y),
        )
    } else {
        Rect::new(Point::new(0.0, 0.0), Size::new(1000.0, 1000.0))
    }
}

/// Clamp a point to stay inside the rect, inset by margin (keeps edges on-canvas).
/// Uses at most half the rect size per axis so min <= max when rect is small.
fn clamp_point_to_rect(p: Point, rect: Rect, margin: f32) -> Point {
    let margin_x = margin.min(rect.size.width / 2.0).max(0.0);
    let margin_y = margin.min(rect.size.height / 2.0).max(0.0);
    let min_x = rect.origin.x + margin_x;
    let max_x = rect.max_x() - margin_x;
    let min_y = rect.origin.y + margin_y;
    let max_y = rect.max_y() - margin_y;
    Point::new(
        p.x.clamp(min_x, max_x),
        p.y.clamp(min_y, max_y),
    )
}

fn distance(a: Point, b: Point) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn segment_intersects_rect(start: Point, end: Point, rect: Rect) -> bool {
    if (start.x - end.x).abs() <= f32::EPSILON {
        let x = start.x;
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);
        x >= rect.origin.x && x <= rect.max_x() && max_y >= rect.origin.y && min_y <= rect.max_y()
    } else if (start.y - end.y).abs() <= f32::EPSILON {
        let y = start.y;
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        y >= rect.origin.y && y <= rect.max_y() && max_x >= rect.origin.x && min_x <= rect.max_x()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use elk_core::{EdgeEndpoint, Graph, LayoutOptions, Size, ViewProfile};

    use crate::pipeline::{
        break_cycles, export_to_graph, import_graph, normalize_edges, place_nodes, assign_layers,
    };

    fn assert_section_is_manhattan(section: &elk_core::EdgeSection) {
        let mut points = vec![section.start];
        points.extend(section.bend_points.iter().copied());
        points.push(section.end);
        for window in points.windows(2) {
            let a = window[0];
            let b = window[1];
            assert!(
                (a.x - b.x).abs() <= 0.1 || (a.y - b.y).abs() <= 0.1,
                "expected manhattan segment, got {a:?} -> {b:?} (dx={}, dy={}); full_points={points:?}",
                (a.x - b.x).abs(),
                (a.y - b.y).abs()
            );
        }
    }

    #[test]
    fn export_inserts_corners_for_interconnection_view_ports() {
        let mut graph = Graph::new();
        let a = graph.add_node(Size::new(120.0, 80.0));
        let b = graph.add_node(Size::new(120.0, 80.0));
        // Put the nodes at different x/y to force diagonal if corners are missing.
        graph.node_mut(a).preferred_position = Some(elk_core::Point::new(40.0, 40.0));
        graph.node_mut(b).preferred_position = Some(elk_core::Point::new(360.0, 200.0));
        let a_out = graph.add_port(a, elk_core::PortSide::East, elk_core::Size::new(8.0, 8.0));
        let b_in = graph.add_port(b, elk_core::PortSide::West, elk_core::Size::new(8.0, 8.0));
        graph.add_edge(EdgeEndpoint::port(a, a_out), EdgeEndpoint::port(b, b_in));

        let options = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);
        let nodes = graph.top_level_nodes();
        let local: BTreeSet<_> = nodes.iter().copied().collect();
        let mut ir = import_graph(&graph, &nodes, &local, &options);
        break_cycles(&mut ir);
        assign_layers(&mut ir, &options);
        normalize_edges(&mut ir, &options);
        let _ = place_nodes(&mut ir, &options);

        let mut graph_copy = graph.clone();
        let mut warnings = Vec::new();
        let mut stats = elk_core::LayoutStats::default();
        export_to_graph(
            &mut graph_copy,
            &ir,
            &local,
            &options,
            &mut warnings,
            &mut stats,
        );

        let section = &graph_copy.edges[0].sections[0];
        // Sanity check the utility: it must be able to orthogonalize diagonal pairs.
        let sanity = crate::pipeline::util::ensure_orthogonal_path(vec![
            elk_core::Point::new(144.0, 64.0),
            elk_core::Point::new(116.0, 232.0),
            elk_core::Point::new(24.0, 232.0),
        ]);
        assert!(
            sanity.len() >= 4,
            "expected ensure_orthogonal_path to insert a corner, got {sanity:?}"
        );
        assert_section_is_manhattan(section);
    }
}
