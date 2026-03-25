//! Minimal port of ELK's `OrthogonalRoutingGenerator` slot assignment logic.
//! Reserved for future routing slot assignment; allow dead_code until wired in.
#![allow(dead_code)]

use std::collections::VecDeque;
/// The upstream implementation builds hyperedge segments, creates a dependency graph based on
/// conflict / crossing penalties, breaks cycles, and assigns routing slots using topological
/// numbering. We port the core mechanics here and keep bend-point generation in `routing.rs`.
/// This module is currently used as an InterconnectionView-only stabilizer to assign
/// deterministic routing slots (lanes) for dense port-to-port connectors.

const TOLERANCE: f32 = 1e-3;
const CONFLICT_THRESHOLD_FACTOR: f32 = 0.5;
const CRITICAL_CONFLICT_THRESHOLD_FACTOR: f32 = 0.2;
const CONFLICT_PENALTY: i32 = 1;
const CROSSING_PENALTY: i32 = 16;

#[derive(Clone, Debug)]
pub(crate) struct HyperEdgeSegment {
    pub id: usize,
    pub start_coordinate: f32,
    pub end_coordinate: f32,
    pub incoming_connection_coordinates: Vec<f32>,
    pub outgoing_connection_coordinates: Vec<f32>,
    pub routing_slot: i32,
    pub in_weight: i32,
    pub out_weight: i32,
    pub incoming: Vec<Dependency>,
    pub outgoing: Vec<Dependency>,
    // ELK splitting metadata (critical cycle resolution).
    pub split_partner: Option<usize>,
    pub split_by: Option<usize>,
    pub mark: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DependencyKind {
    Critical,
    Regular,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Dependency {
    pub source: usize,
    pub target: usize,
    pub weight: i32,
    pub kind: DependencyKind,
}

impl HyperEdgeSegment {
    fn is_straight(&self) -> bool {
        (self.start_coordinate - self.end_coordinate).abs() < TOLERANCE
    }

    fn represents_hyperedge(&self) -> bool {
        // Heuristic proxy for the upstream concept: segments that aggregate multiple
        // incoming/outgoing connections are more likely to represent a hyperedge.
        self.incoming_connection_coordinates.len() + self.outgoing_connection_coordinates.len() > 2
    }
}

pub(crate) fn assign_routing_slots(
    mut segments: Vec<HyperEdgeSegment>,
    edge_spacing: f32,
) -> Vec<i32> {
    if segments.is_empty() {
        return Vec::new();
    }

    // Sort and unique coordinates for min distance computation.
    let min_dist = minimum_horizontal_segment_distance(&segments);
    let critical_conflict_threshold = CRITICAL_CONFLICT_THRESHOLD_FACTOR * min_dist;
    let conflict_threshold = CONFLICT_THRESHOLD_FACTOR * edge_spacing;

    // Dependencies.
    for i in 0..segments.len().saturating_sub(1) {
        for j in (i + 1)..segments.len() {
            let deps = create_dependency_if_necessary(
                &segments[i],
                &segments[j],
                conflict_threshold,
                critical_conflict_threshold,
            );
            for dep in deps {
                add_dependency(&mut segments, dep);
            }
        }
    }

    // Break critical cycles by splitting segments (ELK escape hatch).
    break_critical_cycles_by_splitting(
        &mut segments,
        conflict_threshold,
        critical_conflict_threshold,
    );

    // Break non-critical cycles (ELK: remove weight=0, else reverse).
    break_non_critical_cycles(&mut segments);

    // Assign slots via topological numbering.
    topological_numbering(&mut segments);

    // Return per-id slots.
    let mut slots = vec![0i32; segments.len()];
    for seg in &segments {
        slots[seg.id] = seg.routing_slot;
    }
    slots
}

#[derive(Clone, Copy, Debug)]
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    fn next_u32(&mut self) -> u32 {
        // xorshift64*
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        (x.wrapping_mul(0x2545F4914F6CDD1D) >> 32) as u32
    }

    fn gen_range(&mut self, upper_exclusive: usize) -> usize {
        if upper_exclusive <= 1 {
            return 0;
        }
        (self.next_u32() as usize) % upper_exclusive
    }
}

fn break_critical_cycles_by_splitting(
    segments: &mut Vec<HyperEdgeSegment>,
    conflict_threshold: f32,
    critical_conflict_threshold: f32,
) {
    // Detect critical cycles using the upstream mark-based heuristic.
    // Java uses a Random from the graph; we use a deterministic seed to keep output stable.
    let mut rng = SimpleRng::new(1);
    let deps_to_resolve = detect_cycles_mark_heuristic(segments, true, &mut rng);
    if deps_to_resolve.is_empty() {
        return;
    }

    // Find free areas between connection coordinates.
    let mut free_areas = find_free_areas(segments, critical_conflict_threshold);

    // Decide which segments to split: choose one endpoint of each dependency, preferring the
    // non-hyperedge segment (Java prefers splitting regular edges) and then smaller extent.
    let mut to_split: Vec<usize> = Vec::new();
    for dep in deps_to_resolve {
        if segments[dep.source].split_partner.is_some() || segments[dep.target].split_partner.is_some() {
            continue;
        }
        let s_hyper = segments[dep.source].represents_hyperedge();
        let t_hyper = segments[dep.target].represents_hyperedge();
        let len_s = (segments[dep.source].end_coordinate - segments[dep.source].start_coordinate).abs();
        let len_t = (segments[dep.target].end_coordinate - segments[dep.target].start_coordinate).abs();
        let (seg_to_split, seg_causing) = if s_hyper && !t_hyper {
            (dep.target, dep.source)
        } else if t_hyper && !s_hyper {
            (dep.source, dep.target)
        } else if len_s <= len_t {
            (dep.source, dep.target)
        } else {
            (dep.target, dep.source)
        };
        segments[seg_to_split].split_by = Some(seg_causing);
        to_split.push(seg_to_split);
    }

    // Split from smallest to largest extent (ELK does that).
    to_split.sort_by(|&a, &b| {
        let la = (segments[a].end_coordinate - segments[a].start_coordinate).abs();
        let lb = (segments[b].end_coordinate - segments[b].start_coordinate).abs();
        la.partial_cmp(&lb).unwrap_or(std::cmp::Ordering::Equal)
    });

    for seg_id in to_split {
        if segments[seg_id].split_partner.is_some() {
            continue;
        }
        let split_pos = compute_split_position_and_use_area(seg_id, segments, &mut free_areas, critical_conflict_threshold);
        let new_id = split_at(seg_id, split_pos, segments);

        // Update dependencies: segment -> split_by -> partner
        if let Some(split_by) = segments[seg_id].split_by {
            add_dependency(
                segments,
                Dependency {
                    source: seg_id,
                    target: split_by,
                    weight: 1,
                    kind: DependencyKind::Critical,
                },
            );
            add_dependency(
                segments,
                Dependency {
                    source: split_by,
                    target: new_id,
                    weight: 1,
                    kind: DependencyKind::Critical,
                },
            );
        }

        // Reintroduce dependencies against all others (simplified: recompute pairwise).
        // First clear non-critical deps for the split pair; keep critical ones.
        for id in [seg_id, new_id] {
            segments[id].incoming.retain(|d| d.kind == DependencyKind::Critical);
            segments[id].outgoing.retain(|d| d.kind == DependencyKind::Critical);
        }
        for other in 0..segments.len() {
            if other == seg_id || other == new_id {
                continue;
            }
            let deps_a = create_dependency_if_necessary(
                &segments[other],
                &segments[seg_id],
                conflict_threshold,
                critical_conflict_threshold,
            );
            for d in deps_a {
                add_dependency(segments, d);
            }
            let deps_b = create_dependency_if_necessary(
                &segments[other],
                &segments[new_id],
                conflict_threshold,
                critical_conflict_threshold,
            );
            for d in deps_b {
                add_dependency(segments, d);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct FreeArea {
    start: f32,
    end: f32,
}

fn find_free_areas(segments: &[HyperEdgeSegment], critical_conflict_threshold: f32) -> Vec<FreeArea> {
    let mut coords: Vec<f32> = segments
        .iter()
        .flat_map(|s| {
            s.incoming_connection_coordinates
                .iter()
                .chain(s.outgoing_connection_coordinates.iter())
                .copied()
        })
        .filter(|v| v.is_finite())
        .collect();
    coords.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    coords.dedup_by(|a, b| (*a - *b).abs() < TOLERANCE);

    let mut free = Vec::new();
    for w in coords.windows(2) {
        let a = w[0];
        let b = w[1];
        if b - a >= 2.0 * critical_conflict_threshold {
            free.push(FreeArea {
                start: a + critical_conflict_threshold,
                end: b - critical_conflict_threshold,
            });
        }
    }
    free
}

fn compute_split_position_and_use_area(
    seg_id: usize,
    segments: &[HyperEdgeSegment],
    free: &mut Vec<FreeArea>,
    critical_conflict_threshold: f32,
) -> f32 {
    let s = &segments[seg_id];
    let center = (s.start_coordinate + s.end_coordinate) / 2.0;

    // Choose the first free area that intersects the segment extent; else use center.
    let (min_c, max_c) = if s.start_coordinate <= s.end_coordinate {
        (s.start_coordinate, s.end_coordinate)
    } else {
        (s.end_coordinate, s.start_coordinate)
    };
    for (idx, area) in free.iter().enumerate() {
        if area.end < min_c {
            continue;
        }
        if area.start > max_c {
            break;
        }
        let split = (area.start + area.end) / 2.0;
        // "Use" the area by shrinking it around the split point.
        let left = FreeArea {
            start: area.start,
            end: split - critical_conflict_threshold,
        };
        let right = FreeArea {
            start: split + critical_conflict_threshold,
            end: area.end,
        };
        // Replace current area with up to two smaller ones.
        free.remove(idx);
        if left.end > left.start {
            free.insert(idx, left);
        }
        if right.end > right.start {
            free.insert(idx + 1, right);
        }
        return split;
    }
    center
}

fn split_at(seg_id: usize, split_pos: f32, segments: &mut Vec<HyperEdgeSegment>) -> usize {
    let mut partner = segments[seg_id].clone();
    let new_id = segments.len();
    partner.id = new_id;
    partner.start_coordinate = split_pos;
    partner.end_coordinate = segments[seg_id].end_coordinate;
    partner.incoming_connection_coordinates = vec![split_pos];
    partner.outgoing_connection_coordinates = segments[seg_id].outgoing_connection_coordinates.clone();
    partner.incoming.clear();
    partner.outgoing.clear();
    partner.split_partner = Some(seg_id);
    partner.split_by = None;
    partner.mark = -1;

    segments[seg_id].end_coordinate = split_pos;
    segments[seg_id].outgoing_connection_coordinates = vec![split_pos];
    segments[seg_id].incoming_connection_coordinates = segments[seg_id].incoming_connection_coordinates.clone();
    segments[seg_id].incoming.clear();
    segments[seg_id].outgoing.clear();
    segments[seg_id].split_partner = Some(new_id);

    segments.push(partner);
    new_id
}

fn detect_cycles_mark_heuristic(
    segments: &[HyperEdgeSegment],
    critical_only: bool,
    rng: &mut SimpleRng,
) -> Vec<Dependency> {
    // Port of ELK's HyperEdgeCycleDetector mark heuristic.
    let n = segments.len();
    if n == 0 {
        return Vec::new();
    }

    let mut mark: Vec<i32> = (0..n).map(|i| -(i as i32) - 1).collect();
    let mut in_w = vec![0i32; n];
    let mut out_w = vec![0i32; n];
    let mut critical_in_w = vec![0i32; n];
    let mut critical_out_w = vec![0i32; n];

    let consider = |d: &Dependency, critical_only: bool| -> bool {
        !critical_only || d.kind == DependencyKind::Critical
    };

    for i in 0..n {
        critical_in_w[i] = segments[i]
            .incoming
            .iter()
            .filter(|d| d.kind == DependencyKind::Critical)
            .map(|d| d.weight)
            .sum();
        critical_out_w[i] = segments[i]
            .outgoing
            .iter()
            .filter(|d| d.kind == DependencyKind::Critical)
            .map(|d| d.weight)
            .sum();
        if critical_only {
            in_w[i] = critical_in_w[i];
            out_w[i] = critical_out_w[i];
        } else {
            in_w[i] = segments[i].incoming.iter().map(|d| d.weight).sum();
            out_w[i] = segments[i].outgoing.iter().map(|d| d.weight).sum();
        }
    }

    let mut sources: VecDeque<usize> = VecDeque::new();
    let mut sinks: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if out_w[i] == 0 {
            sinks.push_back(i);
        } else if in_w[i] == 0 {
            sources.push_back(i);
        }
    }

    let mut unprocessed = vec![true; n];
    let mark_base = n as i32;
    let mut next_sink_mark = mark_base - 1;
    let mut next_source_mark = mark_base + 1;

    fn update_neighbors(
        u: usize,
        segments: &[HyperEdgeSegment],
        critical_only: bool,
        unprocessed: &[bool],
        in_w: &mut [i32],
        out_w: &mut [i32],
        critical_in_w: &mut [i32],
        critical_out_w: &mut [i32],
        sources: &mut VecDeque<usize>,
        sinks: &mut VecDeque<usize>,
    ) {
        let consider = |d: &Dependency, critical_only: bool| -> bool {
            !critical_only || d.kind == DependencyKind::Critical
        };

        for dep in &segments[u].outgoing {
            if !consider(dep, critical_only) {
                continue;
            }
            let t = dep.target;
            if unprocessed[t] && dep.weight > 0 {
                in_w[t] -= dep.weight;
                if dep.kind == DependencyKind::Critical {
                    critical_in_w[t] -= dep.weight;
                }
                if in_w[t] <= 0 && out_w[t] > 0 {
                    sources.push_back(t);
                }
            }
        }

        for dep in &segments[u].incoming {
            if !consider(dep, critical_only) {
                continue;
            }
            let s = dep.source;
            if unprocessed[s] && dep.weight > 0 {
                out_w[s] -= dep.weight;
                if dep.kind == DependencyKind::Critical {
                    critical_out_w[s] -= dep.weight;
                }
                if out_w[s] <= 0 && in_w[s] > 0 {
                    sinks.push_back(s);
                }
            }
        }
    }

    while unprocessed.iter().any(|v| *v) {
        while let Some(u) = sinks.pop_front() {
            if !unprocessed[u] {
                continue;
            }
            unprocessed[u] = false;
            mark[u] = next_sink_mark;
            next_sink_mark -= 1;
            update_neighbors(
                u,
                segments,
                critical_only,
                &unprocessed,
                &mut in_w,
                &mut out_w,
                &mut critical_in_w,
                &mut critical_out_w,
                &mut sources,
                &mut sinks,
            );
        }

        while let Some(u) = sources.pop_front() {
            if !unprocessed[u] {
                continue;
            }
            unprocessed[u] = false;
            mark[u] = next_source_mark;
            next_source_mark += 1;
            update_neighbors(
                u,
                segments,
                critical_only,
                &unprocessed,
                &mut in_w,
                &mut out_w,
                &mut critical_in_w,
                &mut critical_out_w,
                &mut sources,
                &mut sinks,
            );
        }

        // Select among unprocessed nodes with maximal outflow, with the same critical guard
        // as upstream (ensure critical deps always point right when critical_only=false).
        let mut max_outflow = i32::MIN;
        let mut max_segments: Vec<usize> = Vec::new();
        for u in 0..n {
            if !unprocessed[u] {
                continue;
            }
            if !critical_only && critical_out_w[u] > 0 && critical_in_w[u] <= 0 {
                max_segments.clear();
                max_segments.push(u);
                break;
            }
            let outflow = out_w[u] - in_w[u];
            if outflow >= max_outflow {
                if outflow > max_outflow {
                    max_outflow = outflow;
                    max_segments.clear();
                }
                max_segments.push(u);
            }
        }

        if !max_segments.is_empty() {
            let pick = max_segments[rng.gen_range(max_segments.len())];
            if unprocessed[pick] {
                unprocessed[pick] = false;
                mark[pick] = next_source_mark;
                next_source_mark += 1;
                update_neighbors(
                    pick,
                    segments,
                    critical_only,
                    &unprocessed,
                    &mut in_w,
                    &mut out_w,
                    &mut critical_in_w,
                    &mut critical_out_w,
                    &mut sources,
                    &mut sinks,
                );
            }
        }
    }

    let shift_base = n as i32 + 1;
    for m in &mut mark {
        if *m < mark_base {
            *m += shift_base;
        }
    }

    let mut out = Vec::new();
    for seg in segments {
        for dep in &seg.outgoing {
            if !consider(dep, critical_only) {
                continue;
            }
            if mark[dep.source] > mark[dep.target] {
                out.push(*dep);
            }
        }
    }
    out
}

fn minimum_horizontal_segment_distance(segments: &[HyperEdgeSegment]) -> f32 {
    let min_in = minimum_difference(
        segments
            .iter()
            .flat_map(|s| s.incoming_connection_coordinates.iter().copied()),
    );
    let min_out = minimum_difference(
        segments
            .iter()
            .flat_map(|s| s.outgoing_connection_coordinates.iter().copied()),
    );
    min_in.min(min_out)
}

fn minimum_difference(values: impl Iterator<Item = f32>) -> f32 {
    let mut numbers: Vec<f32> = values.filter(|v| v.is_finite()).collect();
    if numbers.len() < 2 {
        return f32::MAX;
    }
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    numbers.dedup_by(|a, b| (*a - *b).abs() < TOLERANCE);

    if numbers.len() < 2 {
        return f32::MAX;
    }
    let mut iter = numbers.into_iter();
    let mut prev = iter.next().unwrap_or(0.0);
    let mut min_diff = f32::MAX;
    for cur in iter {
        min_diff = min_diff.min(cur - prev);
        prev = cur;
    }
    min_diff
}

fn create_dependency_if_necessary(
    he1: &HyperEdgeSegment,
    he2: &HyperEdgeSegment,
    conflict_threshold: f32,
    critical_conflict_threshold: f32,
) -> Vec<Dependency> {
    // Straight segments don't take a slot.
    if he1.is_straight() || he2.is_straight() {
        return Vec::new();
    }

    let conflicts1 = count_conflicts(
        &he1.outgoing_connection_coordinates,
        &he2.incoming_connection_coordinates,
        conflict_threshold,
        critical_conflict_threshold,
    );
    let conflicts2 = count_conflicts(
        &he2.outgoing_connection_coordinates,
        &he1.incoming_connection_coordinates,
        conflict_threshold,
        critical_conflict_threshold,
    );

    let critical_conflicts_detected = conflicts1.is_none() || conflicts2.is_none();

    if critical_conflicts_detected {
        let mut out = Vec::new();
        if conflicts1.is_none() {
            // he1 must not be left of he2 => he2 -> he1 (critical)
            out.push(Dependency {
                source: he2.id,
                target: he1.id,
                weight: 1,
                kind: DependencyKind::Critical,
            });
        }
        if conflicts2.is_none() {
            out.push(Dependency {
                source: he1.id,
                target: he2.id,
                weight: 1,
                kind: DependencyKind::Critical,
            });
        }
        return out;
    }

    let conflicts1 = conflicts1.unwrap_or(0);
    let conflicts2 = conflicts2.unwrap_or(0);

    let crossings1 = count_crossings(
        &he1.outgoing_connection_coordinates,
        he2.start_coordinate,
        he2.end_coordinate,
    ) + count_crossings(
        &he2.incoming_connection_coordinates,
        he1.start_coordinate,
        he1.end_coordinate,
    );
    let crossings2 = count_crossings(
        &he2.outgoing_connection_coordinates,
        he1.start_coordinate,
        he1.end_coordinate,
    ) + count_crossings(
        &he1.incoming_connection_coordinates,
        he2.start_coordinate,
        he2.end_coordinate,
    );

    let dep_value1 = CONFLICT_PENALTY * conflicts1 + CROSSING_PENALTY * crossings1;
    let dep_value2 = CONFLICT_PENALTY * conflicts2 + CROSSING_PENALTY * crossings2;

    if dep_value1 < dep_value2 {
        vec![Dependency {
            source: he1.id,
            target: he2.id,
            weight: dep_value2 - dep_value1,
            kind: DependencyKind::Regular,
        }]
    } else if dep_value1 > dep_value2 {
        vec![Dependency {
            source: he2.id,
            target: he1.id,
            weight: dep_value1 - dep_value2,
            kind: DependencyKind::Regular,
        }]
    } else if dep_value1 > 0 {
        // Create two dependencies with 0 weight.
        vec![
            Dependency {
                source: he1.id,
                target: he2.id,
                weight: 0,
                kind: DependencyKind::Regular,
            },
            Dependency {
                source: he2.id,
                target: he1.id,
                weight: 0,
                kind: DependencyKind::Regular,
            },
        ]
    } else {
        Vec::new()
    }
}

/// Returns Some(conflict_count) or None if a critical conflict is detected.
fn count_conflicts(
    posis1: &[f32],
    posis2: &[f32],
    conflict_threshold: f32,
    critical_conflict_threshold: f32,
) -> Option<i32> {
    if posis1.is_empty() || posis2.is_empty() {
        return Some(0);
    }
    let mut i = 0usize;
    let mut j = 0usize;
    let mut conflicts = 0i32;
    while i < posis1.len() && j < posis2.len() {
        let p1 = posis1[i];
        let p2 = posis2[j];
        if p1 > p2 - critical_conflict_threshold && p1 < p2 + critical_conflict_threshold {
            return None;
        }
        if p1 > p2 - conflict_threshold && p1 < p2 + conflict_threshold {
            conflicts += 1;
        }
        if p1 <= p2 {
            i += 1;
        } else {
            j += 1;
        }
    }
    Some(conflicts)
}

fn count_crossings(posis: &[f32], start: f32, end: f32) -> i32 {
    let (a, b) = if start <= end { (start, end) } else { (end, start) };
    let mut crossings = 0i32;
    for &p in posis {
        if p > b {
            break;
        }
        if p >= a {
            crossings += 1;
        }
    }
    crossings
}

fn add_dependency(segments: &mut [HyperEdgeSegment], dep: Dependency) {
    let s = dep.source;
    let t = dep.target;
    segments[s].outgoing.push(dep);
    segments[t].incoming.push(dep);
}

fn break_non_critical_cycles(segments: &mut [HyperEdgeSegment]) {
    // Port of ELK's HyperEdgeCycleDetector for non-critical cycle breaking.
    let mut rng = SimpleRng::new(1);
    let deps_to_resolve = detect_cycles_mark_heuristic(segments, false, &mut rng);
    for dep in deps_to_resolve {
        if dep.kind == DependencyKind::Critical {
            continue;
        }
        if dep.weight == 0 {
            remove_dependency(segments, dep);
        } else {
            reverse_dependency(segments, dep);
        }
    }
}

fn remove_dependency(segments: &mut [HyperEdgeSegment], dep: Dependency) {
    segments[dep.source]
        .outgoing
        .retain(|d| !(d.target == dep.target && d.kind == dep.kind && d.weight == dep.weight));
    segments[dep.target]
        .incoming
        .retain(|d| !(d.source == dep.source && d.kind == dep.kind && d.weight == dep.weight));
}

fn reverse_dependency(segments: &mut [HyperEdgeSegment], dep: Dependency) {
    remove_dependency(segments, dep);
    let reversed = Dependency {
        source: dep.target,
        target: dep.source,
        weight: dep.weight,
        kind: dep.kind,
    };
    add_dependency(segments, reversed);
}

fn topological_numbering(segments: &mut [HyperEdgeSegment]) {
    // initialize weights and sources
    let mut sources: VecDeque<usize> = VecDeque::new();
    let mut rightward_targets: VecDeque<usize> = VecDeque::new();
    for seg in segments.iter_mut() {
        seg.in_weight = seg.incoming.len() as i32;
        seg.out_weight = seg.outgoing.len() as i32;
        seg.routing_slot = 0;
    }
    for seg in segments.iter() {
        if seg.in_weight == 0 {
            sources.push_back(seg.id);
        }
        if seg.out_weight == 0 && seg.incoming_connection_coordinates.is_empty() {
            rightward_targets.push_back(seg.id);
        }
    }

    let mut max_rank = -1i32;
    while let Some(u) = sources.pop_front() {
        let slot_u = segments[u].routing_slot;
        let outgoing = segments[u].outgoing.clone();
        for dep in outgoing {
            let v = dep.target;
            segments[v].routing_slot = segments[v].routing_slot.max(slot_u + 1);
            max_rank = max_rank.max(segments[v].routing_slot);
            segments[v].in_weight -= 1;
            if segments[v].in_weight == 0 {
                sources.push_back(v);
            }
        }
    }

    // ELK adjustment: if a segment has no leftward horizontal segments (incoming coords empty),
    // move it as far right as possible to avoid pushing back edges too far away from targets.
    if max_rank > -1 {
        let targets: Vec<usize> = rightward_targets.iter().copied().collect();
        for node in targets {
            segments[node].routing_slot = max_rank;
        }

        while let Some(node) = rightward_targets.pop_front() {
            let incoming = segments[node].incoming.clone();
            for dep in incoming {
                let source = dep.source;
                if !segments[source].incoming_connection_coordinates.is_empty() {
                    continue;
                }

                segments[source].routing_slot =
                    segments[source].routing_slot.min(segments[node].routing_slot - 1);
                segments[source].out_weight -= 1;
                if segments[source].out_weight == 0 {
                    rightward_targets.push_back(source);
                }
            }
        }
    }
}
