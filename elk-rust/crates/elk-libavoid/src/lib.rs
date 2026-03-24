#![forbid(unsafe_code)]
#![doc = "Obstacle-avoiding edge routing (ELK libavoid baseline, native Rust)."]

use elk_core::{LayoutError, LayoutOptions, LayoutReport, Point, PortSide, Rect, Size};
use std::collections::BTreeSet;

use elk_graph::{ElkGraph, EdgeId, EdgeEndpoint, NodeId};
use elk_alg_common::orthogonal::{
    build_shared_orthogonal_trunk, point_along_tangent, sanitize_orthogonal_path,
    simplify_orthogonal_points,
};

mod router;

use router::{Obstacle, OccupiedSegment, RoutingFailure, route_with_debug_with_penalties};
use router::path_is_clear;

const DEFAULT_CLEARANCE: f32 = 4.0;
const DEFAULT_SEGMENT_PENALTY: f32 = 1.0;
const DEFAULT_BEND_PENALTY: f32 = 6.0;
const DEFAULT_FIXED_SHARED_PATH_PENALTY: f32 = 8.0;
const DEFAULT_NUDGE_SHARED_PATHS_WITH_COMMON_ENDPOINT: bool = true;
const DEFAULT_REVERSE_DIRECTION_PENALTY: f32 = 0.0;
const TERMINAL_NORMAL_OFFSET: f32 = 16.0;
const TERMINAL_TANGENT_SPACING: f32 = 18.0;
const SHARED_SOURCE_SPLIT_KEY: &str = "spec42.shared.split.source";
const SHARED_TARGET_SPLIT_KEY: &str = "spec42.shared.split.target";

#[derive(Clone, Copy, Debug)]
struct RoutePlan {
    actual_start_abs: Point,
    actual_end_abs: Point,
    start_lead_abs: Point,
    end_lead_abs: Point,
    route_start_abs: Point,
    route_end_abs: Point,
    route_start_local: Point,
    route_end_local: Point,
    source_side: Option<PortSide>,
    target_side: Option<PortSide>,
    shared_source_split: Option<f32>,
    shared_target_split: Option<f32>,
}

fn read_penalty_f32(
    by_key: &std::collections::BTreeMap<String, &elk_graph::PropertyValue>,
    keys: &[&str],
    default: f32,
    min: f32,
) -> f32 {
    for key in keys {
        if let Some(v) = by_key.get(&key.to_ascii_lowercase()) {
            if let Some(x) = elk_alg_common::options::value_to_f32(v) {
                return x.max(min);
            }
        }
    }
    default.max(min)
}

fn read_bool_option(
    by_key: &std::collections::BTreeMap<String, &elk_graph::PropertyValue>,
    keys: &[&str],
    default: bool,
) -> bool {
    for key in keys {
        if let Some(v) = by_key.get(&key.to_ascii_lowercase()) {
            match v {
                elk_graph::PropertyValue::Bool(value) => return *value,
                elk_graph::PropertyValue::Int(value) => return *value != 0,
                elk_graph::PropertyValue::String(value) => {
                    if value.eq_ignore_ascii_case("true") {
                        return true;
                    }
                    if value.eq_ignore_ascii_case("false") {
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    default
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LibavoidLayoutEngine;

impl LibavoidLayoutEngine {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Route edges only; does not move nodes. Use after a layout that set node positions.
    pub fn layout(
        &self,
        graph: &mut ElkGraph,
        _options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        graph.validate().map_err(|e| LayoutError::Validation(format!("{e:?}")))?;

        let by_key = elk_alg_common::options::casefold_map(&graph.properties);
        let clearance = read_penalty_f32(
            &by_key,
            &[
                "elk.libavoid.clearance",
                "org.eclipse.elk.libavoid.clearance",
            ],
            DEFAULT_CLEARANCE,
            0.0,
        );
        let segment_penalty = read_penalty_f32(
            &by_key,
            &[
                "elk.libavoid.segmentpenalty",
                "org.eclipse.elk.libavoid.segmentPenalty",
                "org.eclipse.elk.libavoid.segmentpenalty",
            ],
            DEFAULT_SEGMENT_PENALTY,
            1e-6,
        );
        let bend_penalty = read_penalty_f32(
            &by_key,
            &[
                "elk.libavoid.bendpenalty",
                "org.eclipse.elk.libavoid.bendPenalty",
                "org.eclipse.elk.libavoid.bendpenalty",
            ],
            DEFAULT_BEND_PENALTY,
            0.0,
        );
        let fixed_shared_path_penalty = read_penalty_f32(
            &by_key,
            &[
                "elk.libavoid.fixedsharedpathpenalty",
                "org.eclipse.elk.alg.libavoid.fixedSharedPathPenalty",
                "org.eclipse.elk.alg.libavoid.fixedsharedpathpenalty",
            ],
            DEFAULT_FIXED_SHARED_PATH_PENALTY,
            0.0,
        );
        let reverse_direction_penalty = read_penalty_f32(
            &by_key,
            &[
                "elk.libavoid.reversedirectionpenalty",
                "org.eclipse.elk.alg.libavoid.reverseDirectionPenalty",
                "org.eclipse.elk.alg.libavoid.reversedirectionpenalty",
            ],
            DEFAULT_REVERSE_DIRECTION_PENALTY,
            0.0,
        );
        let penalise_shared_paths_at_conn_ends = read_bool_option(
            &by_key,
            &[
                "elk.libavoid.penaliseorthogonalsharedpathsatconnends",
                "org.eclipse.elk.alg.libavoid.penaliseOrthogonalSharedPathsAtConnEnds",
                "org.eclipse.elk.alg.libavoid.penaliseorthogonalsharedpathsatconnends",
            ],
            false,
        );
        let nudge_shared_paths_with_common_endpoint = read_bool_option(
            &by_key,
            &[
                "elk.libavoid.nudgesharedpathswithcommonendpoint",
                "org.eclipse.elk.alg.libavoid.nudgeSharedPathsWithCommonEndPoint",
                "org.eclipse.elk.alg.libavoid.nudgesharedpathswithcommonendpoint",
            ],
            DEFAULT_NUDGE_SHARED_PATHS_WITH_COMMON_ENDPOINT,
        );
        let shared_path_penalty = if penalise_shared_paths_at_conn_ends
            || nudge_shared_paths_with_common_endpoint
        {
            if fixed_shared_path_penalty > 0.0 {
                fixed_shared_path_penalty
            } else {
                clearance.max(4.0)
            }
        } else {
            0.0
        };

        let mut edge_ids: Vec<EdgeId> = graph.edges.iter().map(|e| e.id).collect();
        edge_ids.sort_by_key(|e| e.index());
        let mut occupied_segments = Vec::new();

        for edge_id in edge_ids {
            let plan = build_route_plan(graph, edge_id, Point::new(0.0, 0.0), clearance);
            let excluded = excluded_obstacle_nodes(graph, edge_id, plan.source_side, plan.target_side);
            let obstacles = collect_obstacles(
                graph,
                clearance,
                &excluded,
                Point::new(0.0, 0.0),
                None,
            );
            let shared_path =
                build_shared_route_path_local(&plan, Point::new(0.0, 0.0), &obstacles);
            let simple_paths =
                build_simple_route_candidates_local(&plan, Point::new(0.0, 0.0), &obstacles);
            let routed_path = route_with_debug_with_penalties(
                plan.route_start_local,
                plan.route_end_local,
                &obstacles,
                segment_penalty,
                bend_penalty,
                reverse_direction_penalty,
                shared_path_penalty,
                &occupied_segments,
            )
            .map(|value| value.0)
            .map_err(|e| routing_failure_to_layout_error(edge_id, e))?;
            let path = choose_preferred_route_path(
                shared_path,
                simple_paths,
                routed_path,
                segment_penalty,
                bend_penalty,
                shared_path_penalty,
                &occupied_segments,
            );
            let path = finalize_route_path(
                path,
                plan.actual_start_abs,
                plan.actual_end_abs,
                plan.start_lead_abs,
                plan.end_lead_abs,
                plan.route_start_local,
                plan.route_end_local,
                Point::new(0.0, 0.0),
            )
                .map_err(|m| LayoutError::Routing(format!("edge {:?}: {m}", edge_id)))?;
            let bends = if path.len() >= 2 {
                path[1..path.len() - 1].to_vec()
            } else {
                Vec::new()
            };
            let start_pt = path.first().copied().unwrap_or(plan.actual_start_abs);
            let end_pt = path.last().copied().unwrap_or(plan.actual_end_abs);
            remember_route_segments(&mut occupied_segments, &path, Point::new(0.0, 0.0));
            graph.edges[edge_id.index()].sections.clear();
            let _ = graph.add_edge_section(edge_id, start_pt, bends, end_pt);
        }

        Ok(LayoutReport::default())
    }
}

pub fn layout(graph: &mut ElkGraph, options: &LayoutOptions) -> Result<LayoutReport, LayoutError> {
    LibavoidLayoutEngine::new().layout(graph, options)
}

/// Route only the given edges; does not move nodes. Reads options from `graph.properties` (root).
/// Use after a layout that set node positions. Keeps routing local to the current graph (obstacles
/// from its nodes only).
pub fn route_edges(
    graph: &mut ElkGraph,
    edge_ids: &[EdgeId],
) -> Result<(), LayoutError> {
    route_edges_with_diagnostics(graph, edge_ids).map(|_| ())
}

/// Same as `route_edges`, but returns structured debug lines when `SPEC42_ELK_DEBUG` is set.
pub fn route_edges_with_diagnostics(
    graph: &mut ElkGraph,
    edge_ids: &[EdgeId],
) -> Result<Vec<String>, LayoutError> {
    route_edges_with_diagnostics_in_scope(graph, edge_ids, Point::new(0.0, 0.0), None)
}

/// Route only `edge_ids` using a local frame origin and optional obstacle-node subset.
/// Inputs/outputs to the router are scope-local; edge sections are written back in absolute coords.
pub fn route_edges_with_diagnostics_in_scope(
    graph: &mut ElkGraph,
    edge_ids: &[EdgeId],
    scope_origin_abs: Point,
    scope_nodes: Option<&BTreeSet<NodeId>>,
) -> Result<Vec<String>, LayoutError> {
    graph.validate().map_err(|e| LayoutError::Validation(format!("{e:?}")))?;
    let by_key = elk_alg_common::options::casefold_map(&graph.properties);
    let root_clearance = read_penalty_f32(
        &by_key,
        &[
            "elk.libavoid.clearance",
            "org.eclipse.elk.libavoid.clearance",
        ],
        DEFAULT_CLEARANCE,
        0.0,
    );
    let root_segment_penalty = read_penalty_f32(
        &by_key,
        &[
            "elk.libavoid.segmentpenalty",
            "org.eclipse.elk.libavoid.segmentPenalty",
            "org.eclipse.elk.libavoid.segmentpenalty",
        ],
        DEFAULT_SEGMENT_PENALTY,
        1e-6,
    );
    let root_bend_penalty = read_penalty_f32(
        &by_key,
        &[
            "elk.libavoid.bendpenalty",
            "org.eclipse.elk.libavoid.bendPenalty",
            "org.eclipse.elk.libavoid.bendpenalty",
        ],
        DEFAULT_BEND_PENALTY,
        0.0,
    );
    let root_fixed_shared_path_penalty = read_penalty_f32(
        &by_key,
        &[
            "elk.libavoid.fixedsharedpathpenalty",
            "org.eclipse.elk.alg.libavoid.fixedSharedPathPenalty",
            "org.eclipse.elk.alg.libavoid.fixedsharedpathpenalty",
        ],
        DEFAULT_FIXED_SHARED_PATH_PENALTY,
        0.0,
    );
    let root_reverse_direction_penalty = read_penalty_f32(
        &by_key,
        &[
            "elk.libavoid.reversedirectionpenalty",
            "org.eclipse.elk.alg.libavoid.reverseDirectionPenalty",
            "org.eclipse.elk.alg.libavoid.reversedirectionpenalty",
        ],
        DEFAULT_REVERSE_DIRECTION_PENALTY,
        0.0,
    );
    let root_penalise_shared_paths_at_conn_ends = read_bool_option(
        &by_key,
        &[
            "elk.libavoid.penaliseorthogonalsharedpathsatconnends",
            "org.eclipse.elk.alg.libavoid.penaliseOrthogonalSharedPathsAtConnEnds",
            "org.eclipse.elk.alg.libavoid.penaliseorthogonalsharedpathsatconnends",
        ],
        false,
    );
    let root_nudge_shared_paths_with_common_endpoint = read_bool_option(
        &by_key,
        &[
            "elk.libavoid.nudgesharedpathswithcommonendpoint",
            "org.eclipse.elk.alg.libavoid.nudgeSharedPathsWithCommonEndPoint",
            "org.eclipse.elk.alg.libavoid.nudgesharedpathswithcommonendpoint",
        ],
        DEFAULT_NUDGE_SHARED_PATHS_WITH_COMMON_ENDPOINT,
    );
    let root_shared_path_penalty = if root_penalise_shared_paths_at_conn_ends
        || root_nudge_shared_paths_with_common_endpoint
    {
        if root_fixed_shared_path_penalty > 0.0 {
            root_fixed_shared_path_penalty
        } else {
            root_clearance.max(4.0)
        }
    } else {
        0.0
    };

    let mut sorted_ids: Vec<EdgeId> = edge_ids.to_vec();
    sorted_ids.sort_by_key(|e| e.index());
    let debug_enabled = std::env::var_os("SPEC42_ELK_DEBUG").is_some();
    let mut diagnostics = Vec::new();
    let mut occupied_segments = Vec::new();

    for edge_id in sorted_ids {
        let clearance = root_clearance;
        let segment_penalty = root_segment_penalty;
        let bend_penalty = root_bend_penalty;
        let reverse_direction_penalty = root_reverse_direction_penalty;
        let shared_path_penalty = root_shared_path_penalty;
        let plan = build_route_plan(graph, edge_id, scope_origin_abs, clearance);
        let excluded = excluded_obstacle_nodes(graph, edge_id, plan.source_side, plan.target_side);
        let obstacles = collect_obstacles(graph, clearance, &excluded, scope_origin_abs, scope_nodes);
        let shared_path = build_shared_route_path_local(&plan, scope_origin_abs, &obstacles);
        let simple_paths = build_simple_route_candidates_local(&plan, scope_origin_abs, &obstacles);
        let (routed_path, route_dbg) = if debug_enabled {
            route_with_debug_with_penalties(
                plan.route_start_local,
                plan.route_end_local,
                &obstacles,
                segment_penalty,
                bend_penalty,
                reverse_direction_penalty,
                shared_path_penalty,
                &occupied_segments,
            )
                .map_err(|e| routing_failure_to_layout_error(edge_id, e))?
        } else {
            (
                route_with_debug_with_penalties(
                    plan.route_start_local,
                    plan.route_end_local,
                    &obstacles,
                    segment_penalty,
                    bend_penalty,
                    reverse_direction_penalty,
                    shared_path_penalty,
                    &occupied_segments,
                )
                    .map(|value| value.0)
                    .map_err(|e| routing_failure_to_layout_error(edge_id, e))?,
                router::RouteDebug::default(),
            )
        };
        let shared_path_score = shared_path.as_ref().map(|path| {
            route_path_score(
                path,
                segment_penalty,
                bend_penalty,
                0.0,
                &occupied_segments,
            )
        });
        let simple_path_scores = simple_paths
            .iter()
            .map(|path| {
                route_path_score(
                    path,
                    segment_penalty,
                    bend_penalty,
                    shared_path_penalty,
                    &occupied_segments,
                )
            })
            .collect::<Vec<_>>();
        let routed_path_score = route_path_score(
            &routed_path,
            segment_penalty,
            bend_penalty,
            shared_path_penalty,
            &occupied_segments,
        );
        let path = choose_preferred_route_path(
            shared_path.clone(),
            simple_paths.clone(),
            routed_path,
            segment_penalty,
            bend_penalty,
            shared_path_penalty,
            &occupied_segments,
        );
        let path = finalize_route_path(
            path,
            plan.actual_start_abs,
            plan.actual_end_abs,
            plan.start_lead_abs,
            plan.end_lead_abs,
            plan.route_start_local,
            plan.route_end_local,
            scope_origin_abs,
        )
            .map_err(|m| LayoutError::Routing(format!("edge {:?}: {m}", edge_id)))?;
        let bends = if path.len() >= 2 {
            path[1..path.len() - 1].to_vec()
        } else {
            Vec::new()
        };
        let start_pt = path.first().copied().unwrap_or(plan.actual_start_abs);
        let end_pt = path.last().copied().unwrap_or(plan.actual_end_abs);
        let bends_abs = bends;
        let start_abs_pt = start_pt;
        let end_abs_pt = end_pt;
        if debug_enabled {
            let start_contract_ok = nearly_same_point(start_abs_pt, plan.actual_start_abs, 1.0);
            let end_contract_ok = nearly_same_point(end_abs_pt, plan.actual_end_abs, 1.0);
            diagnostics.push(format!(
                "elk-libavoid: edge={:?} obstacles={} excluded_nodes={:?} scope_origin=({:.1},{:.1}) endpoint_contract=start:{} end:{} start_abs=({:.1},{:.1})->({:.1},{:.1}) end_abs=({:.1},{:.1})->({:.1},{:.1}) candidates={} expanded={} accepted={} blocked={} found={}",
                edge_id,
                obstacles.len(),
                excluded,
                scope_origin_abs.x,
                scope_origin_abs.y,
                start_contract_ok,
                end_contract_ok,
                plan.actual_start_abs.x,
                plan.actual_start_abs.y,
                start_abs_pt.x,
                start_abs_pt.y,
                plan.actual_end_abs.x,
                plan.actual_end_abs.y,
                end_abs_pt.x,
                end_abs_pt.y,
                route_dbg.candidate_points,
                route_dbg.expanded_states,
                route_dbg.accepted_neighbors,
                route_dbg.blocked_neighbors,
                route_dbg.path_found
            ));
            diagnostics.push(format!(
                "elk-libavoid: edge={:?} obstacle_nodes={:?}",
                edge_id,
                obstacles.iter().map(|o| o.node_index).collect::<Vec<_>>()
            ));
            if let Some(shared) = &shared_path {
                diagnostics.push(format!(
                    "elk-libavoid: edge={:?} shared_points={}",
                    edge_id,
                    shared
                        .iter()
                        .map(|p| format!("({:.1},{:.1})", p.x + scope_origin_abs.x, p.y + scope_origin_abs.y))
                        .collect::<Vec<_>>()
                        .join(" -> ")
                ));
            }
            if let Some(obstacle) = obstacles.iter().find(|o| o.node_index == 5) {
                diagnostics.push(format!(
                    "elk-libavoid: edge={:?} obstacle_node_5=({:.1},{:.1})-({:.1},{:.1})",
                    edge_id,
                    obstacle.rect.origin.x + scope_origin_abs.x,
                    obstacle.rect.origin.y + scope_origin_abs.y,
                    obstacle.rect.origin.x + obstacle.rect.size.width + scope_origin_abs.x,
                    obstacle.rect.origin.y + obstacle.rect.size.height + scope_origin_abs.y
                ));
            }
            diagnostics.push(format!(
                "elk-libavoid: edge={:?} shared_source_split={:?} shared_target_split={:?} shared_candidate={} shared_score={:?} simple_candidates={} simple_scores={:?} routed_score={:.1} chosen_points={}",
                edge_id,
                plan.shared_source_split,
                plan.shared_target_split,
                shared_path.is_some(),
                shared_path_score.map(|score| (score * 10.0).round() / 10.0),
                simple_paths.len(),
                simple_path_scores
                    .iter()
                    .map(|score| (score * 10.0).round() / 10.0)
                    .collect::<Vec<_>>(),
                (routed_path_score * 10.0).round() / 10.0,
                path.len()
            ));
            diagnostics.push(format!(
                "elk-libavoid: edge={:?} chosen_path={}",
                edge_id,
                path.iter()
                    .map(|p| format!("({:.1},{:.1})", p.x, p.y))
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ));
        }
        remember_route_segments(&mut occupied_segments, &path, scope_origin_abs);
        graph.edges[edge_id.index()].sections.clear();
        let _ = graph.add_edge_section(edge_id, start_abs_pt, bends_abs, end_abs_pt);
    }
    Ok(diagnostics)
}

fn nearly_same_point(a: Point, b: Point, eps: f32) -> bool {
    (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps
}

fn build_route_plan(
    graph: &ElkGraph,
    edge_id: EdgeId,
    scope_origin_abs: Point,
    clearance: f32,
) -> RoutePlan {
    let edge = &graph.edges[edge_id.index()];
    let source = edge.sources.first().copied();
    let target = edge.targets.first().copied();
    let other_end_abs = target
        .map(|ep| endpoint_center(graph, ep))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let other_start_abs = source
        .map(|ep| endpoint_center(graph, ep))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let source_side = source.map(|ep| endpoint_side_for_routing(graph, ep, other_end_abs));
    let target_side = target.map(|ep| endpoint_side_for_routing(graph, ep, other_start_abs));
    let source_slot = endpoint_slot(graph, edge_id, true);
    let target_slot = endpoint_slot(graph, edge_id, false);
    let lead = (clearance + TERMINAL_NORMAL_OFFSET).max(clearance + 1.0);
    let source_lead = lead + source_slot.abs() as f32 * (TERMINAL_TANGENT_SPACING * 0.75);
    let target_lead = lead + target_slot.abs() as f32 * (TERMINAL_TANGENT_SPACING * 0.75);
    let actual_start_abs = source
        .zip(source_side)
        .map(|(ep, side)| endpoint_anchor_for_routing(graph, ep, side, source_slot))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let actual_end_abs = target
        .zip(target_side)
        .map(|(ep, side)| endpoint_anchor_for_routing(graph, ep, side, target_slot))
        .unwrap_or_else(|| Point::new(0.0, 0.0));
    let start_lead_abs = source_side
        .map(|side| point_along_outward_normal(actual_start_abs, side, source_lead))
        .unwrap_or(actual_start_abs);
    let end_lead_abs = target_side
        .map(|side| point_along_outward_normal(actual_end_abs, side, target_lead))
        .unwrap_or(actual_end_abs);
    let route_start_abs = source_side
        .map(|side| point_along_tangent(start_lead_abs, side, tangent_route_offset(source, source_slot)))
        .unwrap_or(start_lead_abs);
    let route_end_abs = target_side
        .map(|side| point_along_tangent(end_lead_abs, side, tangent_route_offset(target, target_slot)))
        .unwrap_or(end_lead_abs);
    RoutePlan {
        actual_start_abs,
        actual_end_abs,
        start_lead_abs,
        end_lead_abs,
        route_start_abs,
        route_end_abs,
        route_start_local: to_local(route_start_abs, scope_origin_abs),
        route_end_local: to_local(route_end_abs, scope_origin_abs),
        source_side,
        target_side,
        shared_source_split: read_shared_split(graph, edge_id, true),
        shared_target_split: read_shared_split(graph, edge_id, false),
    }
}

fn finalize_route_path(
    route_path_local: Vec<Point>,
    actual_start_abs: Point,
    actual_end_abs: Point,
    start_lead_abs: Point,
    end_lead_abs: Point,
    route_start_local: Point,
    route_end_local: Point,
    scope_origin_abs: Point,
) -> Result<Vec<Point>, String> {
    let route_start_abs = to_abs(route_start_local, scope_origin_abs);
    let route_end_abs = to_abs(route_end_local, scope_origin_abs);
    let path_abs: Vec<Point> = route_path_local
        .into_iter()
        .map(|point| to_abs(point, scope_origin_abs))
        .collect();
    sanitize_orthogonal_path(
        path_abs,
        actual_start_abs,
        actual_end_abs,
        start_lead_abs,
        end_lead_abs,
        route_start_abs,
        route_end_abs,
    )
}

fn endpoint_slot(graph: &ElkGraph, edge_id: EdgeId, is_source: bool) -> i32 {
    let key = elk_graph::PropertyKey::from(if is_source {
        "spec42.endpoint.slot.source"
    } else {
        "spec42.endpoint.slot.target"
    });
    graph.edges[edge_id.index()]
        .properties
        .get(&key)
        .and_then(|value| value.as_i64())
        .unwrap_or(0) as i32
}

fn read_shared_split(
    graph: &ElkGraph,
    edge_id: EdgeId,
    is_source: bool,
) -> Option<f32> {
    let key = elk_graph::PropertyKey::from(if is_source {
        SHARED_SOURCE_SPLIT_KEY
    } else {
        SHARED_TARGET_SPLIT_KEY
    });
    graph.edges[edge_id.index()]
        .properties
        .get(&key)
        .and_then(|value| value.as_f64())
        .map(|value| value as f32)
}

fn build_shared_route_path_local(
    plan: &RoutePlan,
    scope_origin_abs: Point,
    obstacles: &[Obstacle],
) -> Option<Vec<Point>> {
    let source_side = plan.source_side?;
    let target_side = plan.target_side?;
    let mut candidates = shared_route_bend_candidates(
        plan.route_start_abs,
        plan.route_end_abs,
        source_side,
        target_side,
        plan.shared_source_split,
        plan.shared_target_split,
    );
    candidates.extend(shared_route_detour_bend_candidates(
        plan.route_start_abs,
        plan.route_end_abs,
        source_side,
        target_side,
        plan.shared_source_split,
        plan.shared_target_split,
        scope_origin_abs,
        obstacles,
    ));
    for bends in candidates {
        let route_abs = std::iter::once(plan.route_start_abs)
            .chain(bends.into_iter())
            .chain(std::iter::once(plan.route_end_abs))
            .collect::<Vec<_>>();
        let route_local = route_abs
            .into_iter()
            .map(|point| to_local(point, scope_origin_abs))
            .collect::<Vec<_>>();
        if path_is_clear(&route_local, obstacles)
            && shared_path_is_strictly_clear(&route_local, obstacles)
        {
            return Some(route_local);
        }
    }
    None
}

fn shared_path_is_strictly_clear(points: &[Point], obstacles: &[Obstacle]) -> bool {
    if points.len() < 2 {
        return false;
    }
    let start = points[0];
    let end = *points.last().unwrap_or(&start);
    points.windows(2).all(|pair| {
        obstacles.iter().all(|obstacle| {
            let allows_terminal_touch =
                rect_contains_local(&obstacle.rect, start) || rect_contains_local(&obstacle.rect, end);
            if allows_terminal_touch {
                true
            } else {
                !segment_hits_rect(pair[0], pair[1], &obstacle.rect)
            }
        })
    })
}

fn rect_contains_local(rect: &Rect, point: Point) -> bool {
    point.x >= rect.origin.x - 1e-4
        && point.x <= rect.origin.x + rect.size.width + 1e-4
        && point.y >= rect.origin.y - 1e-4
        && point.y <= rect.origin.y + rect.size.height + 1e-4
}

fn segment_hits_rect(a: Point, b: Point, rect: &Rect) -> bool {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);
    !(max_x < rect.origin.x
        || min_x > rect.origin.x + rect.size.width
        || max_y < rect.origin.y
        || min_y > rect.origin.y + rect.size.height)
}

fn shared_route_detour_bend_candidates(
    start_abs: Point,
    end_abs: Point,
    source_side: PortSide,
    target_side: PortSide,
    source_split: Option<f32>,
    target_split: Option<f32>,
    scope_origin_abs: Point,
    obstacles: &[Obstacle],
) -> Vec<Vec<Point>> {
    match (source_side, target_side) {
        (PortSide::East, PortSide::West) | (PortSide::West, PortSide::East) => {
            dual_shared_horizontal_detour_candidates(
                to_local(start_abs, scope_origin_abs),
                to_local(end_abs, scope_origin_abs),
                source_split.map(|v| v - scope_origin_abs.x),
                target_split.map(|v| v - scope_origin_abs.x),
                obstacles,
            )
        }
        (PortSide::South, PortSide::North) | (PortSide::North, PortSide::South) => {
            dual_shared_vertical_detour_candidates(
                to_local(start_abs, scope_origin_abs),
                to_local(end_abs, scope_origin_abs),
                source_split.map(|v| v - scope_origin_abs.y),
                target_split.map(|v| v - scope_origin_abs.y),
                obstacles,
            )
        }
        _ => Vec::new(),
    }
}

fn shared_route_bend_candidates(
    start: Point,
    end: Point,
    source_side: PortSide,
    target_side: PortSide,
    source_split: Option<f32>,
    target_split: Option<f32>,
) -> Vec<Vec<Point>> {
    let mut candidates = Vec::new();

    if let Some(bends) = build_dual_shared_route_bends(
        start,
        end,
        source_side,
        target_side,
        source_split,
        target_split,
    ) {
        candidates.push(bends);
    }

    for split in [source_split, target_split].into_iter().flatten() {
        if let Some(bends) =
            build_shared_orthogonal_trunk(start, end, source_side, target_side, split)
        {
            candidates.push(bends);
        }
    }

    candidates
}

fn build_dual_shared_route_bends(
    start: Point,
    end: Point,
    source_side: PortSide,
    target_side: PortSide,
    source_split: Option<f32>,
    target_split: Option<f32>,
) -> Option<Vec<Point>> {
    match (source_side, target_side) {
        (PortSide::East, PortSide::West) | (PortSide::West, PortSide::East) => {
            build_dual_shared_horizontal_bends(start, end, source_split, target_split)
        }
        (PortSide::South, PortSide::North) | (PortSide::North, PortSide::South) => {
            build_dual_shared_vertical_bends(start, end, source_split, target_split)
        }
        _ => None,
    }
}

fn build_dual_shared_horizontal_bends(
    start: Point,
    end: Point,
    source_split: Option<f32>,
    target_split: Option<f32>,
) -> Option<Vec<Point>> {
    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let valid_split = |split: f32| split > min_x && split < max_x;
    let source_split = source_split.filter(|split| valid_split(*split));
    let target_split = target_split.filter(|split| valid_split(*split));

    match (source_split, target_split) {
        (Some(source_split), Some(target_split)) => Some(simplify_orthogonal_points(vec![
            Point::new(source_split, start.y),
            Point::new(target_split, start.y),
            Point::new(target_split, end.y),
        ])),
        _ => None,
    }
}

fn build_dual_shared_vertical_bends(
    start: Point,
    end: Point,
    source_split: Option<f32>,
    target_split: Option<f32>,
) -> Option<Vec<Point>> {
    let min_y = start.y.min(end.y);
    let max_y = start.y.max(end.y);
    let valid_split = |split: f32| split > min_y && split < max_y;
    let source_split = source_split.filter(|split| valid_split(*split));
    let target_split = target_split.filter(|split| valid_split(*split));

    match (source_split, target_split) {
        (Some(source_split), Some(target_split)) => Some(simplify_orthogonal_points(vec![
            Point::new(start.x, source_split),
            Point::new(start.x, target_split),
            Point::new(end.x, target_split),
        ])),
        _ => None,
    }
}

fn dual_shared_horizontal_detour_candidates(
    start: Point,
    end: Point,
    source_split: Option<f32>,
    target_split: Option<f32>,
    obstacles: &[Obstacle],
) -> Vec<Vec<Point>> {
    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let valid_split = |split: f32| split > min_x && split < max_x;
    let Some(source_split) = source_split.filter(|split| valid_split(*split)) else {
        return Vec::new();
    };
    let Some(target_split) = target_split.filter(|split| valid_split(*split)) else {
        return Vec::new();
    };

    let mut seen = Vec::<f32>::new();
    let mut candidates = Vec::<(f32, Vec<Point>)>::new();
    for route_y in transverse_axis_candidates(start.y, end.y, obstacles, true) {
        if seen.iter().any(|value| (value - route_y).abs() <= 1e-3) {
            continue;
        }
        seen.push(route_y);
        let bends = simplify_orthogonal_points(vec![
            Point::new(source_split, start.y),
            Point::new(source_split, route_y),
            Point::new(target_split, route_y),
            Point::new(target_split, end.y),
        ]);
        let route = std::iter::once(start)
            .chain(bends.iter().copied())
            .chain(std::iter::once(end))
            .collect::<Vec<_>>();
        if !path_is_clear(&route, obstacles) {
            continue;
        }
        let cost =
            (route_y - start.y).abs() + (route_y - end.y).abs() + 0.25 * (target_split - source_split).abs();
        candidates.push((cost, bends));
    }
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    candidates.into_iter().map(|(_, bends)| bends).collect()
}

fn dual_shared_vertical_detour_candidates(
    start: Point,
    end: Point,
    source_split: Option<f32>,
    target_split: Option<f32>,
    obstacles: &[Obstacle],
) -> Vec<Vec<Point>> {
    let min_y = start.y.min(end.y);
    let max_y = start.y.max(end.y);
    let valid_split = |split: f32| split > min_y && split < max_y;
    let Some(source_split) = source_split.filter(|split| valid_split(*split)) else {
        return Vec::new();
    };
    let Some(target_split) = target_split.filter(|split| valid_split(*split)) else {
        return Vec::new();
    };

    let mut seen = Vec::<f32>::new();
    let mut candidates = Vec::<(f32, Vec<Point>)>::new();
    for route_x in transverse_axis_candidates(start.x, end.x, obstacles, false) {
        if seen.iter().any(|value| (value - route_x).abs() <= 1e-3) {
            continue;
        }
        seen.push(route_x);
        let bends = simplify_orthogonal_points(vec![
            Point::new(start.x, source_split),
            Point::new(route_x, source_split),
            Point::new(route_x, target_split),
            Point::new(end.x, target_split),
        ]);
        let route = std::iter::once(start)
            .chain(bends.iter().copied())
            .chain(std::iter::once(end))
            .collect::<Vec<_>>();
        if !path_is_clear(&route, obstacles) {
            continue;
        }
        let cost =
            (route_x - start.x).abs() + (route_x - end.x).abs() + 0.25 * (target_split - source_split).abs();
        candidates.push((cost, bends));
    }
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    candidates.into_iter().map(|(_, bends)| bends).collect()
}

fn transverse_axis_candidates(
    start_axis: f32,
    end_axis: f32,
    obstacles: &[Obstacle],
    horizontal_detour: bool,
) -> Vec<f32> {
    let mut values = vec![start_axis, end_axis];
    for obstacle in obstacles {
        if horizontal_detour {
            values.push(obstacle.rect.origin.y);
            values.push(obstacle.rect.origin.y + obstacle.rect.size.height);
        } else {
            values.push(obstacle.rect.origin.x);
            values.push(obstacle.rect.origin.x + obstacle.rect.size.width);
        }
    }
    values.sort_by(|a, b| {
        let da = (a - start_axis).abs() + (a - end_axis).abs();
        let db = (b - start_axis).abs() + (b - end_axis).abs();
        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
    });
    values
}

fn choose_preferred_route_path(
    shared_path: Option<Vec<Point>>,
    simple_paths: Vec<Vec<Point>>,
    routed_path: Vec<Point>,
    segment_penalty: f32,
    bend_penalty: f32,
    shared_path_penalty: f32,
    occupied_segments: &[OccupiedSegment],
) -> Vec<Point> {
    let mut best_path = routed_path;
    let mut best_score = route_path_score(
        &best_path,
        segment_penalty,
        bend_penalty,
        shared_path_penalty,
        occupied_segments,
    );

    if let Some(candidate) = shared_path {
        let candidate_score = route_path_score(
            &candidate,
            segment_penalty,
            bend_penalty,
            0.0,
            occupied_segments,
        );
        if candidate_score <= best_score + 1e-3 {
            best_score = candidate_score;
            best_path = candidate;
        }
    }

    for candidate in simple_paths {
        let candidate_score = route_path_score(
            &candidate,
            segment_penalty,
            bend_penalty,
            shared_path_penalty,
            occupied_segments,
        );
        if candidate_score <= best_score + 1e-3 {
            best_score = candidate_score;
            best_path = candidate;
        }
    }

    best_path
}

fn route_path_score(
    path: &[Point],
    segment_penalty: f32,
    bend_penalty: f32,
    shared_path_penalty: f32,
    occupied_segments: &[OccupiedSegment],
) -> f32 {
    let mut cost = 0.0;
    let mut previous_axis: Option<bool> = None;
    for pair in path.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let dx = (b.x - a.x).abs();
        let dy = (b.y - a.y).abs();
        let length = dx + dy;
        cost += length * segment_penalty.max(1e-6);
        let is_vertical = dx <= 1e-5;
        if let Some(prev_vertical) = previous_axis {
            if prev_vertical != is_vertical {
                cost += bend_penalty.max(1.0);
            }
        }
        previous_axis = Some(is_vertical);
        if shared_path_penalty > 0.0 {
            let shared_overlap = route_segment_shared_overlap_length(a, b, occupied_segments);
            if shared_overlap > 0.0 {
                cost += shared_overlap * shared_path_penalty;
            }
        }
    }
    cost
}

fn route_segment_shared_overlap_length(
    a: Point,
    b: Point,
    occupied_segments: &[OccupiedSegment],
) -> f32 {
    occupied_segments
        .iter()
        .map(|occupied| route_segment_overlap_length(a, b, occupied.start, occupied.end))
        .fold(0.0, f32::max)
}

fn route_segment_overlap_length(a: Point, b: Point, c: Point, d: Point) -> f32 {
    let a_vertical = (a.x - b.x).abs() < 1e-5;
    let c_vertical = (c.x - d.x).abs() < 1e-5;
    if a_vertical && c_vertical {
        if (a.x - c.x).abs() >= 1e-5 {
            return 0.0;
        }
        let a_min = a.y.min(b.y);
        let a_max = a.y.max(b.y);
        let c_min = c.y.min(d.y);
        let c_max = c.y.max(d.y);
        return (a_max.min(c_max) - a_min.max(c_min)).max(0.0);
    }

    let a_horizontal = (a.y - b.y).abs() < 1e-5;
    let c_horizontal = (c.y - d.y).abs() < 1e-5;
    if a_horizontal && c_horizontal {
        if (a.y - c.y).abs() >= 1e-5 {
            return 0.0;
        }
        let a_min = a.x.min(b.x);
        let a_max = a.x.max(b.x);
        let c_min = c.x.min(d.x);
        let c_max = c.x.max(d.x);
        return (a_max.min(c_max) - a_min.max(c_min)).max(0.0);
    }

    0.0
}

fn build_simple_route_candidates_local(
    plan: &RoutePlan,
    scope_origin_abs: Point,
    obstacles: &[Obstacle],
) -> Vec<Vec<Point>> {
    let start = to_local(plan.route_start_abs, scope_origin_abs);
    let end = to_local(plan.route_end_abs, scope_origin_abs);
    let mut candidates = Vec::new();
    for bends in [
        vec![Point::new(end.x, start.y)],
        vec![Point::new(start.x, end.y)],
    ] {
        let route = std::iter::once(start)
            .chain(bends.into_iter())
            .chain(std::iter::once(end))
            .collect::<Vec<_>>();
        let route = elk_alg_common::orthogonal::simplify_orthogonal_points(route);
        if path_is_clear(&route, obstacles) {
            candidates.push(route);
        }
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::{choose_preferred_route_path, shared_route_bend_candidates, OccupiedSegment};
    use elk_core::{Point, PortSide};

    #[test]
    fn prefers_shorter_routed_path_over_long_shared_detour() {
        let shared_path = vec![
            Point::new(0.0, 0.0),
            Point::new(40.0, 0.0),
            Point::new(40.0, 120.0),
            Point::new(20.0, 120.0),
            Point::new(20.0, -40.0),
            Point::new(200.0, -40.0),
        ];
        let routed_path = vec![
            Point::new(0.0, 0.0),
            Point::new(30.0, 0.0),
            Point::new(30.0, -40.0),
            Point::new(200.0, -40.0),
        ];

        let chosen = choose_preferred_route_path(
            Some(shared_path.clone()),
            Vec::new(),
            routed_path.clone(),
            1.0,
            6.0,
            0.0,
            &[],
        );

        assert_eq!(chosen, routed_path);
        assert_ne!(chosen, shared_path);
    }

    #[test]
    fn preserves_shared_candidate_when_it_is_no_worse() {
        let shared_path = vec![
            Point::new(0.0, 0.0),
            Point::new(40.0, 0.0),
            Point::new(40.0, 50.0),
            Point::new(100.0, 50.0),
        ];
        let routed_path = vec![
            Point::new(0.0, 0.0),
            Point::new(40.0, 0.0),
            Point::new(40.0, 50.0),
            Point::new(100.0, 50.0),
        ];

        let chosen = choose_preferred_route_path(
            Some(shared_path.clone()),
            Vec::new(),
            routed_path,
            1.0,
            6.0,
            4.0,
            &[OccupiedSegment {
                start: Point::new(40.0, 0.0),
                end: Point::new(40.0, 50.0),
            }],
        );

        assert_eq!(chosen, shared_path);
    }

    #[test]
    fn dual_split_shared_candidate_is_built_before_single_split_candidates() {
        let candidates = shared_route_bend_candidates(
            Point::new(20.0, 40.0),
            Point::new(220.0, 160.0),
            PortSide::East,
            PortSide::West,
            Some(60.0),
            Some(180.0),
        );

        assert!(!candidates.is_empty());
        assert_eq!(
            candidates[0],
            vec![
                Point::new(60.0, 40.0),
                Point::new(180.0, 40.0),
                Point::new(180.0, 160.0),
            ]
        );
    }

    #[test]
    fn shared_candidate_ignores_overlap_penalty_for_intended_bundle() {
        let shared_path = vec![
            Point::new(0.0, 0.0),
            Point::new(40.0, 0.0),
            Point::new(40.0, 120.0),
            Point::new(120.0, 120.0),
        ];
        let routed_path = vec![
            Point::new(0.0, 0.0),
            Point::new(80.0, 0.0),
            Point::new(80.0, 120.0),
            Point::new(120.0, 120.0),
        ];

        let chosen = choose_preferred_route_path(
            Some(shared_path.clone()),
            Vec::new(),
            routed_path,
            1.0,
            6.0,
            20.0,
            &[OccupiedSegment {
                start: Point::new(40.0, 0.0),
                end: Point::new(40.0, 120.0),
            }],
        );

        assert_eq!(chosen, shared_path);
    }
}

fn routing_failure_to_layout_error(edge_id: EdgeId, failure: RoutingFailure) -> LayoutError {
    let reason = match failure {
        RoutingFailure::NoCandidatePoints => "no_candidate_points",
        RoutingFailure::DegenerateEndpoints => "degenerate_endpoints",
        RoutingFailure::NoRouteFound => "no_route_found",
    };
    LayoutError::Routing(format!("edge {:?}: libavoid route failed ({reason})", edge_id))
}

fn node_rect(graph: &ElkGraph, n: NodeId) -> Rect {
    let g = &graph.nodes[n.index()].geometry;
    let o = node_abs_origin(graph, n);
    Rect::new(
        Point::new(o.x, o.y),
        Size::new(g.width.max(0.0), g.height.max(0.0)),
    )
}

fn node_abs_origin(graph: &ElkGraph, node_id: NodeId) -> Point {
    let n = &graph.nodes[node_id.index()];
    match n.parent {
        Some(parent) if parent != graph.root => {
            let p = node_abs_origin(graph, parent);
            Point::new(p.x + n.geometry.x, p.y + n.geometry.y)
        }
        _ => Point::new(n.geometry.x, n.geometry.y),
    }
}

fn collect_obstacles(
    graph: &ElkGraph,
    clearance: f32,
    excluded_nodes: &[NodeId],
    scope_origin_abs: Point,
    scope_nodes: Option<&BTreeSet<NodeId>>,
) -> Vec<Obstacle> {
    let mut out = Vec::new();
    for node in &graph.nodes {
        if node.id == graph.root {
            continue;
        }
        if scope_nodes.is_some_and(|s| !s.contains(&node.id)) {
            continue;
        }
        if excluded_nodes.contains(&node.id) {
            continue;
        }
        let r_abs = node_rect(graph, node.id);
        let r = Rect::new(
            to_local(r_abs.origin, scope_origin_abs),
            r_abs.size,
        );
        let expanded = Rect::new(
            Point::new(r.origin.x - clearance, r.origin.y - clearance),
            Size::new(r.size.width + 2.0 * clearance, r.size.height + 2.0 * clearance),
        );
        out.push(Obstacle {
            node_index: node.id.index(),
            rect: expanded,
        });
    }
    out.sort_by(|a, b| {
        a.rect.origin.x
            .partial_cmp(&b.rect.origin.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.rect.origin.y.partial_cmp(&b.rect.origin.y).unwrap_or(std::cmp::Ordering::Equal))
    });
    out
}

fn remember_route_segments(
    occupied_segments: &mut Vec<OccupiedSegment>,
    path_abs: &[Point],
    scope_origin_abs: Point,
) {
    if path_abs.len() < 2 {
        return;
    }

    remember_terminal_chain(occupied_segments, path_abs, scope_origin_abs, false);
}

fn remember_terminal_chain(
    occupied_segments: &mut Vec<OccupiedSegment>,
    path_abs: &[Point],
    scope_origin_abs: Point,
    from_start: bool,
) {
    let pairs = if from_start {
        path_abs.windows(2).collect::<Vec<_>>()
    } else {
        path_abs.windows(2).rev().collect::<Vec<_>>()
    };

    let mut remembered = 0usize;
    for pair in pairs {
        let start_abs = if from_start { pair[0] } else { pair[1] };
        let end_abs = if from_start { pair[1] } else { pair[0] };
        let start = to_local(start_abs, scope_origin_abs);
        let end = to_local(end_abs, scope_origin_abs);
        let is_orthogonal = (start.x - end.x).abs() < 1e-5 || (start.y - end.y).abs() < 1e-5;
        if !is_orthogonal {
            break;
        }
        occupied_segments.push(OccupiedSegment { start, end });
        remembered += 1;
        if remembered >= 2 {
            break;
        }
    }
}

fn excluded_obstacle_nodes(
    graph: &ElkGraph,
    edge_id: EdgeId,
    source_side: Option<PortSide>,
    target_side: Option<PortSide>,
) -> Vec<NodeId> {
    let edge = &graph.edges[edge_id.index()];
    let mut nodes = Vec::new();
    if let Some(source) = edge.sources.first() {
        if source.node != graph.root && source_side.is_none() && !nodes.contains(&source.node) {
            nodes.push(source.node);
        }
    }
    if let Some(target) = edge.targets.first() {
        if target.node != graph.root && target_side.is_none() && !nodes.contains(&target.node) {
            nodes.push(target.node);
        }
    }
    nodes
}

fn endpoint_center(graph: &ElkGraph, ep: EdgeEndpoint) -> Point {
    if let Some(port_id) = ep.port {
        let p = &graph.ports[port_id.index()];
        let o = node_abs_origin(graph, p.node);
        Point::new(
            o.x + p.geometry.x + p.geometry.width / 2.0,
            o.y + p.geometry.y + p.geometry.height / 2.0,
        )
    } else {
        let n = &graph.nodes[ep.node.index()];
        let o = node_abs_origin(graph, ep.node);
        Point::new(
            o.x + n.geometry.width / 2.0,
            o.y + n.geometry.height / 2.0,
        )
    }
}

fn endpoint_port_side(graph: &ElkGraph, ep: EdgeEndpoint) -> Option<PortSide> {
    ep.port.map(|port_id| graph.ports[port_id.index()].side)
}

fn endpoint_side_for_routing(graph: &ElkGraph, endpoint: EdgeEndpoint, toward: Point) -> PortSide {
    endpoint_port_side(graph, endpoint).unwrap_or_else(|| infer_node_side_for_target(graph, endpoint.node, toward))
}

fn infer_node_side_for_target(graph: &ElkGraph, node_id: NodeId, toward: Point) -> PortSide {
    let center = endpoint_center(graph, EdgeEndpoint::node(node_id));
    let dx = toward.x - center.x;
    let dy = toward.y - center.y;
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
}

fn endpoint_anchor_for_routing(
    graph: &ElkGraph,
    endpoint: EdgeEndpoint,
    side: PortSide,
    slot: i32,
) -> Point {
    if let Some(port_id) = endpoint.port {
        let _ = (port_id, side, slot);
        return endpoint_center(graph, endpoint);
    }

    let node = &graph.nodes[endpoint.node.index()];
    let origin = node_abs_origin(graph, endpoint.node);
    let center = Point::new(origin.x + node.geometry.width / 2.0, origin.y + node.geometry.height / 2.0);
    let tangent = slot as f32 * TERMINAL_TANGENT_SPACING;
    let margin = 12.0;
    let min_x = origin.x + margin;
    let max_x = (origin.x + node.geometry.width - margin).max(min_x);
    let min_y = origin.y + margin;
    let max_y = (origin.y + node.geometry.height - margin).max(min_y);

    match side {
        PortSide::North => Point::new((center.x + tangent).clamp(min_x, max_x), origin.y),
        PortSide::South => Point::new((center.x + tangent).clamp(min_x, max_x), origin.y + node.geometry.height),
        PortSide::East => Point::new(origin.x + node.geometry.width, (center.y + tangent).clamp(min_y, max_y)),
        PortSide::West => Point::new(origin.x, (center.y + tangent).clamp(min_y, max_y)),
    }
}

fn tangent_route_offset(endpoint: Option<EdgeEndpoint>, slot: i32) -> f32 {
    let _ = (endpoint, slot);
    0.0
}

fn point_along_outward_normal(center: Point, side: PortSide, delta: f32) -> Point {
    match side {
        PortSide::North => Point::new(center.x, center.y - delta),
        PortSide::South => Point::new(center.x, center.y + delta),
        PortSide::East => Point::new(center.x + delta, center.y),
        PortSide::West => Point::new(center.x - delta, center.y),
    }
}

fn to_local(abs: Point, origin_abs: Point) -> Point {
    Point::new(abs.x - origin_abs.x, abs.y - origin_abs.y)
}

fn to_abs(local: Point, origin_abs: Point) -> Point {
    Point::new(local.x + origin_abs.x, local.y + origin_abs.y)
}
