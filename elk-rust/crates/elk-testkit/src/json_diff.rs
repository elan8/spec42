//! Stable-id keyed diff between two ELK layout JSON trees.
//!
//! This is intended for *diagnostics*, not strict gating: it reports the worst
//! node/port/edge geometry deltas to guide porting work from Java ELK.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

#[derive(Clone, Debug)]
struct Bounds {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Clone, Debug)]
struct NodeRec {
    id: String,
    bounds: Bounds,
}

#[derive(Clone, Debug)]
struct PortRec {
    id: String,
    parent_node_id: String,
    bounds: Bounds,
}

#[derive(Clone, Debug)]
struct EdgeRec {
    signature: String,
    points: Vec<(f32, f32)>,
}

#[derive(Clone, Debug)]
struct TreeIndex {
    nodes: BTreeMap<String, NodeRec>,
    ports: BTreeMap<String, PortRec>,
    edges: Vec<EdgeRec>,
}

fn f32_field(map: &serde_json::Map<String, Value>, key: &str) -> Option<f32> {
    map.get(key).and_then(|v| v.as_f64()).map(|v| v as f32)
}

fn str_field(map: &serde_json::Map<String, Value>, key: &str) -> Option<String> {
    map.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
}

fn bounds_of(map: &serde_json::Map<String, Value>) -> Option<Bounds> {
    Some(Bounds {
        x: f32_field(map, "x")?,
        y: f32_field(map, "y")?,
        w: f32_field(map, "width")?,
        h: f32_field(map, "height")?,
    })
}

fn section_object(v: &Value) -> Option<&serde_json::Map<String, Value>> {
    if let Some(arr) = v.as_array() {
        return arr.first().and_then(|v| v.as_object());
    }
    v.as_object()
}

fn polyline_points_from_edge(edge: &serde_json::Map<String, Value>) -> Vec<(f32, f32)> {
    let mut out = Vec::new();
    let sections = edge.get("sections").and_then(|v| v.as_array());
    let Some(sections) = sections else {
        return out;
    };
    for sec in sections {
        let Some(sec) = section_object(sec) else { continue };
        let start = sec.get("startPoint").and_then(|v| v.as_object());
        let end = sec.get("endPoint").and_then(|v| v.as_object());
        if let Some(s) = start {
            if let (Some(x), Some(y)) = (f32_field(s, "x"), f32_field(s, "y")) {
                out.push((x, y));
            }
        }
        if let Some(bends) = sec.get("bendPoints").and_then(|v| v.as_array()) {
            for bend in bends {
                if let Some(b) = bend.as_object() {
                    if let (Some(x), Some(y)) = (f32_field(b, "x"), f32_field(b, "y")) {
                        out.push((x, y));
                    }
                }
            }
        }
        if let Some(e) = end {
            if let (Some(x), Some(y)) = (f32_field(e, "x"), f32_field(e, "y")) {
                out.push((x, y));
            }
        }
    }

    // De-dupe consecutive equal points (some exporters repeat joints).
    out.dedup_by(|a, b| (a.0 - b.0).abs() <= f32::EPSILON && (a.1 - b.1).abs() <= f32::EPSILON);
    out
}

fn edge_signature(edge: &serde_json::Map<String, Value>) -> Option<String> {
    let sources = edge.get("sources").and_then(|v| v.as_array())?;
    let targets = edge.get("targets").and_then(|v| v.as_array())?;
    let s = sources
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join("|");
    let t = targets
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join("|");
    Some(format!("{s} -> {t}"))
}

fn index_tree(root: &Value) -> Result<TreeIndex, String> {
    let root = root
        .as_object()
        .ok_or_else(|| "layout json root is not an object".to_string())?;
    let mut idx = TreeIndex {
        nodes: BTreeMap::new(),
        ports: BTreeMap::new(),
        edges: Vec::new(),
    };

    fn walk_node(
        node: &serde_json::Map<String, Value>,
        idx: &mut TreeIndex,
    ) -> Result<(), String> {
        let id = str_field(node, "id").unwrap_or_else(|| "<missing-id>".to_string());
        if let Some(bounds) = bounds_of(node) {
            idx.nodes.insert(
                id.clone(),
                NodeRec {
                    id: id.clone(),
                    bounds,
                },
            );
        }

        if let Some(ports) = node.get("ports").and_then(|v| v.as_array()) {
            for port in ports {
                let Some(p) = port.as_object() else { continue };
                let pid = str_field(p, "id").unwrap_or_else(|| "<missing-port-id>".to_string());
                if let Some(bounds) = bounds_of(p) {
                    idx.ports.insert(
                        pid.clone(),
                        PortRec {
                            id: pid,
                            parent_node_id: id.clone(),
                            bounds,
                        },
                    );
                }
            }
        }

        if let Some(edges) = node.get("edges").and_then(|v| v.as_array()) {
            for edge in edges {
                let Some(e) = edge.as_object() else { continue };
                let Some(sig) = edge_signature(e) else { continue };
                idx.edges.push(EdgeRec {
                    signature: sig,
                    points: polyline_points_from_edge(e),
                });
            }
        }

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                let c = child
                    .as_object()
                    .ok_or_else(|| format!("child of node {id} is not an object"))?;
                walk_node(c, idx)?;
            }
        }
        Ok(())
    }

    walk_node(root, &mut idx)?;
    Ok(idx)
}

fn max_abs(a: f32, b: f32) -> f32 {
    a.abs().max(b.abs())
}

/// Build a human-readable diff report keyed by node/port ids and edge endpoint signatures.
///
/// `top_n` controls how many worst offenders to include per category.
pub fn build_layout_json_diff_report(
    java_layout_json: &Value,
    rust_layout_json: &Value,
    top_n: usize,
) -> Result<String, String> {
    let java = index_tree(java_layout_json)?;
    let rust = index_tree(rust_layout_json)?;

    let mut out = String::new();

    // Nodes.
    let java_node_ids: BTreeSet<_> = java.nodes.keys().cloned().collect();
    let rust_node_ids: BTreeSet<_> = rust.nodes.keys().cloned().collect();
    let missing_in_rust: Vec<_> = java_node_ids.difference(&rust_node_ids).cloned().collect();
    let extra_in_rust: Vec<_> = rust_node_ids.difference(&java_node_ids).cloned().collect();

    out.push_str("== Node bounds (key: node.id) ==\n");
    out.push_str(&format!(
        "java_nodes={} rust_nodes={} missing_in_rust={} extra_in_rust={}\n",
        java.nodes.len(),
        rust.nodes.len(),
        missing_in_rust.len(),
        extra_in_rust.len()
    ));

    let mut node_deltas: Vec<(f32, String)> = Vec::new();
    for (id, j) in &java.nodes {
        let Some(r) = rust.nodes.get(id) else { continue };
        let dx = r.bounds.x - j.bounds.x;
        let dy = r.bounds.y - j.bounds.y;
        let dw = r.bounds.w - j.bounds.w;
        let dh = r.bounds.h - j.bounds.h;
        let score = max_abs(dx, dy).max(max_abs(dw, dh));
        node_deltas.push((
            score,
            format!(
                "id={id} Δx={dx:.3} Δy={dy:.3} Δw={dw:.3} Δh={dh:.3} (java x/y/w/h={:.1}/{:.1}/{:.1}/{:.1} rust={:.1}/{:.1}/{:.1}/{:.1})\n",
                j.bounds.x, j.bounds.y, j.bounds.w, j.bounds.h,
                r.bounds.x, r.bounds.y, r.bounds.w, r.bounds.h,
            ),
        ));
    }
    node_deltas.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (_, line) in node_deltas.into_iter().take(top_n) {
        out.push_str(&line);
    }
    out.push('\n');

    // Ports.
    let java_port_ids: BTreeSet<_> = java.ports.keys().cloned().collect();
    let rust_port_ids: BTreeSet<_> = rust.ports.keys().cloned().collect();
    let missing_ports: Vec<_> = java_port_ids.difference(&rust_port_ids).cloned().collect();
    let extra_ports: Vec<_> = rust_port_ids.difference(&java_port_ids).cloned().collect();
    out.push_str("== Port bounds (key: port.id) ==\n");
    out.push_str(&format!(
        "java_ports={} rust_ports={} missing_in_rust={} extra_in_rust={}\n",
        java.ports.len(),
        rust.ports.len(),
        missing_ports.len(),
        extra_ports.len()
    ));

    let mut port_deltas: Vec<(f32, String)> = Vec::new();
    for (id, j) in &java.ports {
        let Some(r) = rust.ports.get(id) else { continue };
        let dx = r.bounds.x - j.bounds.x;
        let dy = r.bounds.y - j.bounds.y;
        let dw = r.bounds.w - j.bounds.w;
        let dh = r.bounds.h - j.bounds.h;
        let score = max_abs(dx, dy).max(max_abs(dw, dh));
        port_deltas.push((
            score,
            format!(
                "id={id} parent(java={}) parent(rust={}) Δx={dx:.3} Δy={dy:.3} Δw={dw:.3} Δh={dh:.3}\n",
                j.parent_node_id, r.parent_node_id
            ),
        ));
    }
    port_deltas.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (_, line) in port_deltas.into_iter().take(top_n) {
        out.push_str(&line);
    }
    out.push('\n');

    // Edges: group by signature; allow duplicates (parallel edges) by sorting in each group.
    out.push_str("== Edge polylines (key: sources/targets signature) ==\n");

    let mut java_edges_by_sig: BTreeMap<String, Vec<EdgeRec>> = BTreeMap::new();
    for e in java.edges {
        java_edges_by_sig.entry(e.signature.clone()).or_default().push(e);
    }
    let mut rust_edges_by_sig: BTreeMap<String, Vec<EdgeRec>> = BTreeMap::new();
    for e in rust.edges {
        rust_edges_by_sig.entry(e.signature.clone()).or_default().push(e);
    }

    let all_sigs: BTreeSet<_> = java_edges_by_sig
        .keys()
        .chain(rust_edges_by_sig.keys())
        .cloned()
        .collect();
    let missing_edge_sigs: Vec<_> = all_sigs
        .iter()
        .filter(|sig| !rust_edges_by_sig.contains_key(*sig))
        .cloned()
        .collect();
    let extra_edge_sigs: Vec<_> = all_sigs
        .iter()
        .filter(|sig| !java_edges_by_sig.contains_key(*sig))
        .cloned()
        .collect();
    out.push_str(&format!(
        "java_edges={} rust_edges={} missing_sigs_in_rust={} extra_sigs_in_rust={}\n",
        java_edges_by_sig.values().map(|v| v.len()).sum::<usize>(),
        rust_edges_by_sig.values().map(|v| v.len()).sum::<usize>(),
        missing_edge_sigs.len(),
        extra_edge_sigs.len()
    ));

    let mut edge_deltas: Vec<(f32, String)> = Vec::new();
    for sig in all_sigs {
        let Some(mut je) = java_edges_by_sig.get(&sig).cloned() else { continue };
        let Some(mut re) = rust_edges_by_sig.get(&sig).cloned() else { continue };

        // Sort by point count then by rough length (sum manhattan).
        fn score_edge(e: &EdgeRec) -> (usize, f32) {
            let mut len = 0.0f32;
            for w in e.points.windows(2) {
                len += (w[0].0 - w[1].0).abs() + (w[0].1 - w[1].1).abs();
            }
            (e.points.len(), len)
        }
        je.sort_by(|a, b| {
            let (al, ad) = score_edge(a);
            let (bl, bd) = score_edge(b);
            al.cmp(&bl).then_with(|| ad.total_cmp(&bd))
        });
        re.sort_by(|a, b| {
            let (al, ad) = score_edge(a);
            let (bl, bd) = score_edge(b);
            al.cmp(&bl).then_with(|| ad.total_cmp(&bd))
        });

        let pairs = je.len().min(re.len());
        for i in 0..pairs {
            let jp = &je[i].points;
            let rp = &re[i].points;
            let mut max_d = 0.0f32;
            let mut max_idx = None;
            if jp.len() == rp.len() {
                for (idx, ((jx, jy), (rx, ry))) in jp.iter().zip(rp.iter()).enumerate() {
                    let d = ((rx - jx).abs()).max((ry - jy).abs());
                    if d > max_d {
                        max_d = d;
                        max_idx = Some(idx);
                    }
                }
            } else {
                // Penalize length mismatch heavily, but still show counts.
                max_d = 1_000.0 + (jp.len() as i32 - rp.len() as i32).abs() as f32;
            }
            edge_deltas.push((
                max_d,
                format!(
                    "sig={sig} variant={i} java_points={} rust_points={} max_point_drift={max_d:.3} max_idx={:?}\n",
                    jp.len(),
                    rp.len(),
                    max_idx
                ),
            ));
        }
    }

    edge_deltas.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (_, line) in edge_deltas.into_iter().take(top_n) {
        out.push_str(&line);
    }

    Ok(out)
}

