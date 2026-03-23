//! Java-vs-Rust JSON parity comparison with tolerant numeric checks.
//!
//! Compares layout output (node positions, edge sections) between two ELK graph JSON
//! trees. Used to validate Rust output against checked-in expected outputs (from Java ELK
//! or from a previous Rust run).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Compares two ELK graph JSON root objects. Returns `Ok(())` if structure matches
/// and all numeric layout fields are within `coord_eps`. Order of children and edges
/// must match (same indices); node/edge ids must match.
pub fn compare_layout_json(
    actual: &serde_json::Value,
    expected: &serde_json::Value,
    coord_eps: f32,
) -> Result<(), String> {
    let a = actual
        .as_object()
        .ok_or_else(|| "actual root is not an object".to_string())?;
    let e = expected
        .as_object()
        .ok_or_else(|| "expected root is not an object".to_string())?;
    compare_node_object(a, e, coord_eps, "root")
}

fn compare_node_object(
    actual: &serde_json::Map<String, serde_json::Value>,
    expected: &serde_json::Map<String, serde_json::Value>,
    eps: f32,
    path: &str,
) -> Result<(), String> {
    compare_str_key(actual, expected, "id", path)?;
    // Some ELK exporters omit root x/y while Rust export keeps explicit zero origin.
    if path == "root" {
        compare_f32_key_if_both_present(actual, expected, "x", eps, path)?;
        compare_f32_key_if_both_present(actual, expected, "y", eps, path)?;
    } else {
        compare_f32_key(actual, expected, "x", eps, path)?;
        compare_f32_key(actual, expected, "y", eps, path)?;
    }
    compare_f32_key(actual, expected, "width", eps, path)?;
    compare_f32_key(actual, expected, "height", eps, path)?;

    let a_children = actual.get("children").and_then(|v| v.as_array());
    let e_children = expected.get("children").and_then(|v| v.as_array());
    match (a_children, e_children) {
        (None, None) => {}
        (Some(ac), Some(ec)) => {
            if ac.len() != ec.len() {
                return Err(format!(
                    "{}: children length {} != {}",
                    path,
                    ac.len(),
                    ec.len()
                ));
            }
            for (i, (ac_obj, ec_obj)) in ac.iter().zip(ec.iter()).enumerate() {
                let ac_map = ac_obj
                    .as_object()
                    .ok_or_else(|| format!("{}: actual child {} is not object", path, i))?;
                let ec_map = ec_obj
                    .as_object()
                    .ok_or_else(|| format!("{}: expected child {} is not object", path, i))?;
                let child_id = ec_map
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("?");
                compare_node_object(ac_map, ec_map, eps, &format!("{}.children[{}](id={})", path, i, child_id))?;
            }
        }
        _ => {
            return Err(format!(
                "{}: children presence mismatch (actual has children: {}, expected: {})",
                path,
                a_children.is_some(),
                e_children.is_some()
            ));
        }
    }

    let a_edges = actual.get("edges").and_then(|v| v.as_array());
    let e_edges = expected.get("edges").and_then(|v| v.as_array());
    match (a_edges, e_edges) {
        (None, None) => {}
        (Some(ae), Some(ee)) => {
            if ae.len() != ee.len() {
                return Err(format!(
                    "{}: edges length {} != {}",
                    path,
                    ae.len(),
                    ee.len()
                ));
            }
            for (i, (ae_obj, ee_obj)) in ae.iter().zip(ee.iter()).enumerate() {
                compare_edge_object(ae_obj, ee_obj, eps, &format!("{}.edges[{}]", path, i))?;
            }
        }
        _ => {
            return Err(format!(
                "{}: edges presence mismatch (actual has edges: {}, expected: {})",
                path,
                a_edges.is_some(),
                e_edges.is_some()
            ));
        }
    }

    Ok(())
}

fn compare_edge_object(
    actual: &serde_json::Value,
    expected: &serde_json::Value,
    eps: f32,
    path: &str,
) -> Result<(), String> {
    let a = actual
        .as_object()
        .ok_or_else(|| format!("{}: actual edge is not object", path))?;
    let e = expected
        .as_object()
        .ok_or_else(|| format!("{}: expected edge is not object", path))?;
    // Edge ids are not stable between Java and Rust exports.
    compare_str_array_key(a, e, "sources", path)?;
    compare_str_array_key(a, e, "targets", path)?;

    let a_sections = a.get("sections").and_then(|v| v.as_array());
    let e_sections = e.get("sections").and_then(|v| v.as_array());
    match (a_sections, e_sections) {
        (None, None) => {}
        (Some(asec), Some(esec)) => {
            if asec.len() != esec.len() {
                return Err(format!(
                    "{}: section count {} != {}",
                    path,
                    asec.len(),
                    esec.len()
                ));
            }
            for (i, (asec_val, esec_val)) in asec.iter().zip(esec.iter()).enumerate() {
                compare_section_value(asec_val, esec_val, eps, &format!("{}.sections[{}]", path, i))?;
            }
        }
        _ => {
            return Err(format!(
                "{}: sections presence mismatch",
                path
            ));
        }
    }

    Ok(())
}

fn compare_section_value(
    actual: &serde_json::Value,
    expected: &serde_json::Value,
    eps: f32,
    path: &str,
) -> Result<(), String> {
    // ELK JSON: section is object with startPoint {x,y}, endPoint {x,y}, bendPoints [{x,y},...]
    // or wrapped in array of one element (legacy).
    let a_sec = section_object(actual).ok_or_else(|| format!("{}: actual section not object/array", path))?;
    let e_sec = section_object(expected).ok_or_else(|| format!("{}: expected section not object/array", path))?;
    compare_point_key(a_sec, e_sec, "startPoint", eps, path)?;
    compare_point_key(a_sec, e_sec, "endPoint", eps, path)?;
    let a_bends = a_sec.get("bendPoints").and_then(|v| v.as_array());
    let e_bends = e_sec.get("bendPoints").and_then(|v| v.as_array());
    match (a_bends, e_bends) {
        (None, None) => {}
        (Some(ab), Some(eb)) => {
            if ab.len() != eb.len() {
                return Err(format!("{}: bend point count {} != {}", path, ab.len(), eb.len()));
            }
            for (i, (pb_a, pb_e)) in ab.iter().zip(eb.iter()).enumerate() {
                let pa = pb_a.as_object().ok_or_else(|| format!("{}: bend[{}] not object", path, i))?;
                let pe = pb_e.as_object().ok_or_else(|| format!("{}: expected bend[{}] not object", path, i))?;
                compare_f32_key(pa, pe, "x", eps, &format!("{}.bend[{}]", path, i))?;
                compare_f32_key(pa, pe, "y", eps, &format!("{}.bend[{}]", path, i))?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn compare_point_key(
    actual: &serde_json::Map<String, serde_json::Value>,
    expected: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    eps: f32,
    path: &str,
) -> Result<(), String> {
    let a_pt = actual.get(key).and_then(|v| v.as_object());
    let e_pt = expected.get(key).and_then(|v| v.as_object());
    match (a_pt, e_pt) {
        (Some(ap), Some(ep)) => {
            compare_f32_key(ap, ep, "x", eps, &format!("{}.{}", path, key))?;
            compare_f32_key(ap, ep, "y", eps, &format!("{}.{}", path, key))?;
        }
        (None, None) => {}
        _ => return Err(format!("{}: {} presence mismatch", path, key)),
    }
    Ok(())
}

fn section_object(v: &serde_json::Value) -> Option<&serde_json::Map<String, serde_json::Value>> {
    if let Some(arr) = v.as_array() {
        if let Some(first) = arr.first() {
            return first.as_object();
        }
    }
    v.as_object()
}

fn compare_str_key(
    actual: &serde_json::Map<String, serde_json::Value>,
    expected: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    path: &str,
) -> Result<(), String> {
    let a = actual.get(key).and_then(|v| v.as_str());
    let e = expected.get(key).and_then(|v| v.as_str());
    match (a, e) {
        (Some(aa), Some(ee)) if aa == ee => Ok(()),
        (Some(aa), Some(ee)) => Err(format!("{}: {} {:?} != {:?}", path, key, aa, ee)),
        (None, None) => Ok(()),
        _ => Err(format!("{}: {} presence mismatch", path, key)),
    }
}

fn compare_f32_key(
    actual: &serde_json::Map<String, serde_json::Value>,
    expected: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    eps: f32,
    path: &str,
) -> Result<(), String> {
    let a = get_f32(actual, key);
    let e = get_f32(expected, key);
    match (a, e) {
        (Some(aa), Some(ee)) => {
            if (aa - ee).abs() <= eps {
                Ok(())
            } else {
                Err(format!("{}: {} {} != {} (eps {})", path, key, aa, ee, eps))
            }
        }
        (None, None) => Ok(()),
        _ => Err(format!("{}: {} presence mismatch", path, key)),
    }
}

fn compare_f32_key_if_both_present(
    actual: &serde_json::Map<String, serde_json::Value>,
    expected: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    eps: f32,
    path: &str,
) -> Result<(), String> {
    let a = get_f32(actual, key);
    let e = get_f32(expected, key);
    match (a, e) {
        (Some(aa), Some(ee)) => {
            if (aa - ee).abs() <= eps {
                Ok(())
            } else {
                Err(format!("{}: {} {} != {} (eps {})", path, key, aa, ee, eps))
            }
        }
        _ => Ok(()),
    }
}

fn compare_str_array_key(
    actual: &serde_json::Map<String, serde_json::Value>,
    expected: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    path: &str,
) -> Result<(), String> {
    let a = actual.get(key).and_then(|v| v.as_array());
    let e = expected.get(key).and_then(|v| v.as_array());
    match (a, e) {
        (Some(aa), Some(ee)) => {
            if aa.len() != ee.len() {
                return Err(format!("{}: {} length {} != {}", path, key, aa.len(), ee.len()));
            }
            for (i, (av, ev)) in aa.iter().zip(ee.iter()).enumerate() {
                let as_ = av.as_str();
                let es_ = ev.as_str();
                if as_ != es_ {
                    return Err(format!(
                        "{}: {}[{}] {:?} != {:?}",
                        path, key, i, as_, es_
                    ));
                }
            }
            Ok(())
        }
        (None, None) => Ok(()),
        _ => Err(format!("{}: {} presence mismatch", path, key)),
    }
}

fn get_f32(m: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<f32> {
    let v = m.get(key)?;
    match v {
        serde_json::Value::Number(n) => n.as_f64().map(|f| f as f32),
        serde_json::Value::String(s) => s.trim().parse::<f32>().ok(),
        _ => None,
    }
}

/// Collects all node ids (as strings) from a root JSON object for structure checks.
pub fn node_ids_from_json(root: &serde_json::Value) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    collect_node_ids(root, &mut set);
    set
}

fn collect_node_ids(v: &serde_json::Value, out: &mut BTreeSet<String>) {
    let Some(obj) = v.as_object() else { return };
    if let Some(id) = obj.get("id").and_then(|x| x.as_str()) {
        out.insert(id.to_string());
    }
    if let Some(children) = obj.get("children").and_then(|x| x.as_array()) {
        for c in children {
            collect_node_ids(c, out);
        }
    }
}

/// Relaxed Java-vs-Rust parity check:
/// - node/edge/port counts must match exactly
/// - non-root nodes must expose finite x/y/width/height numbers in both trees
pub fn compare_layout_json_relaxed(
    actual: &serde_json::Value,
    expected: &serde_json::Value,
) -> Result<(), String> {
    let a_stats = graph_stats(actual)?;
    let e_stats = graph_stats(expected)?;
    if a_stats != e_stats {
        return Err(format!(
            "graph stats differ: actual(nodes={}, edges={}, ports={}) expected(nodes={}, edges={}, ports={})",
            a_stats.0, a_stats.1, a_stats.2, e_stats.0, e_stats.1, e_stats.2
        ));
    }

    check_node_geometry(actual, true, "actual")?;
    check_node_geometry(expected, true, "expected")?;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityFixtureKind {
    Layered,
    Interconnection,
    Libavoid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityRootCause {
    LayeredSlotAssignment,
    LayeredBendGeneration,
    HierarchicalPortRouting,
    DummyRestoration,
    LibavoidOptionMapping,
    LibavoidCompoundClusterModeling,
    PostRouteNormalizationDrift,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityGraphStats {
    pub nodes: usize,
    pub edges: usize,
    pub ports: usize,
    pub edge_sections: usize,
    pub routed_edges: usize,
    pub bend_points: usize,
    pub orthogonal_violations: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityDiffSummary {
    pub rust: ParityGraphStats,
    pub java: ParityGraphStats,
    pub node_delta: isize,
    pub edge_delta: isize,
    pub port_delta: isize,
    pub section_delta: isize,
    pub routed_edge_delta: isize,
    pub bend_point_delta: isize,
    pub orthogonal_violation_delta: isize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityCaseReport {
    pub fixture: String,
    pub kind: ParityFixtureKind,
    pub status: String,
    pub skip_reason: Option<String>,
    pub relaxed_error: Option<String>,
    pub bend_complexity_error: Option<String>,
    pub root_causes: Vec<ParityRootCause>,
    pub diff: ParityDiffSummary,
}

pub fn parity_graph_stats(root: &serde_json::Value) -> Result<ParityGraphStats, String> {
    fn visit_node(
        value: &serde_json::Value,
        nodes: &mut usize,
        edges: &mut usize,
        ports: &mut usize,
        edge_sections: &mut usize,
        routed_edges: &mut usize,
        bend_points: &mut usize,
        orthogonal_violations: &mut usize,
    ) -> Result<(), String> {
        let Some(obj) = value.as_object() else {
            return Ok(());
        };
        *nodes += 1;

        if let Some(port_arr) = obj.get("ports").and_then(serde_json::Value::as_array) {
            *ports += port_arr.len();
        }

        if let Some(edge_arr) = obj.get("edges").and_then(serde_json::Value::as_array) {
            *edges += edge_arr.len();
            for edge in edge_arr {
                let Some(edge_obj) = edge.as_object() else {
                    continue;
                };
                let sections = edge_obj.get("sections").and_then(serde_json::Value::as_array);
                if let Some(sections) = sections {
                    if !sections.is_empty() {
                        *routed_edges += 1;
                    }
                    *edge_sections += sections.len();
                    for (section_idx, section) in sections.iter().enumerate() {
                        let Some(sec_obj) = section_object(section) else {
                            continue;
                        };
                        let points = section_points(sec_obj)?;
                        if points.len() >= 2 {
                            for pair in points.windows(2) {
                                let a = pair[0];
                                let b = pair[1];
                                let dx = (a.0 - b.0).abs();
                                let dy = (a.1 - b.1).abs();
                                if dx > 1e-3 && dy > 1e-3 {
                                    *orthogonal_violations += 1;
                                }
                            }
                        }
                        *bend_points += sec_obj
                            .get("bendPoints")
                            .and_then(serde_json::Value::as_array)
                            .map_or(0, |points| points.len());
                        let _ = section_idx;
                    }
                }
            }
        }

        if let Some(children) = obj.get("children").and_then(serde_json::Value::as_array) {
            for child in children {
                visit_node(
                    child,
                    nodes,
                    edges,
                    ports,
                    edge_sections,
                    routed_edges,
                    bend_points,
                    orthogonal_violations,
                )?;
            }
        }
        Ok(())
    }

    let mut nodes = 0usize;
    let mut edges = 0usize;
    let mut ports = 0usize;
    let mut edge_sections = 0usize;
    let mut routed_edges = 0usize;
    let mut bend_points = 0usize;
    let mut orthogonal_violations = 0usize;
    visit_node(
        root,
        &mut nodes,
        &mut edges,
        &mut ports,
        &mut edge_sections,
        &mut routed_edges,
        &mut bend_points,
        &mut orthogonal_violations,
    )?;
    Ok(ParityGraphStats {
        nodes,
        edges,
        ports,
        edge_sections,
        routed_edges,
        bend_points,
        orthogonal_violations,
    })
}

pub fn build_parity_case_report(
    fixture: &str,
    kind: ParityFixtureKind,
    rust: &serde_json::Value,
    java: &serde_json::Value,
    relaxed_error: Option<String>,
    bend_complexity_error: Option<String>,
) -> Result<ParityCaseReport, String> {
    let rust_stats = parity_graph_stats(rust)?;
    let java_stats = parity_graph_stats(java)?;
    let root_causes = classify_parity_root_causes(
        kind,
        &rust_stats,
        &java_stats,
        relaxed_error.as_deref(),
        bend_complexity_error.as_deref(),
    );
    let status = if relaxed_error.is_none() && bend_complexity_error.is_none() {
        "passed"
    } else {
        "failed"
    };
    Ok(ParityCaseReport {
        fixture: fixture.to_string(),
        kind,
        status: status.to_string(),
        skip_reason: None,
        relaxed_error,
        bend_complexity_error,
        root_causes,
        diff: ParityDiffSummary {
            node_delta: rust_stats.nodes as isize - java_stats.nodes as isize,
            edge_delta: rust_stats.edges as isize - java_stats.edges as isize,
            port_delta: rust_stats.ports as isize - java_stats.ports as isize,
            section_delta: rust_stats.edge_sections as isize - java_stats.edge_sections as isize,
            routed_edge_delta: rust_stats.routed_edges as isize - java_stats.routed_edges as isize,
            bend_point_delta: rust_stats.bend_points as isize - java_stats.bend_points as isize,
            orthogonal_violation_delta:
                rust_stats.orthogonal_violations as isize - java_stats.orthogonal_violations as isize,
            rust: rust_stats,
            java: java_stats,
        },
    })
}

pub fn build_skipped_parity_case_report(
    fixture: &str,
    kind: ParityFixtureKind,
    skip_reason: impl Into<String>,
) -> ParityCaseReport {
    let empty = ParityGraphStats {
        nodes: 0,
        edges: 0,
        ports: 0,
        edge_sections: 0,
        routed_edges: 0,
        bend_points: 0,
        orthogonal_violations: 0,
    };
    ParityCaseReport {
        fixture: fixture.to_string(),
        kind,
        status: "skipped".to_string(),
        skip_reason: Some(skip_reason.into()),
        relaxed_error: None,
        bend_complexity_error: None,
        root_causes: Vec::new(),
        diff: ParityDiffSummary {
            rust: empty.clone(),
            java: empty,
            node_delta: 0,
            edge_delta: 0,
            port_delta: 0,
            section_delta: 0,
            routed_edge_delta: 0,
            bend_point_delta: 0,
            orthogonal_violation_delta: 0,
        },
    }
}

fn classify_parity_root_causes(
    kind: ParityFixtureKind,
    rust: &ParityGraphStats,
    java: &ParityGraphStats,
    relaxed_error: Option<&str>,
    bend_complexity_error: Option<&str>,
) -> Vec<ParityRootCause> {
    let mut causes = BTreeSet::new();

    if rust.nodes != java.nodes || rust.edges != java.edges || rust.ports != java.ports {
        if rust.ports != java.ports {
            causes.insert(ParityRootCause::HierarchicalPortRouting);
            causes.insert(ParityRootCause::DummyRestoration);
        } else {
            causes.insert(ParityRootCause::DummyRestoration);
        }
    }

    if rust.routed_edges != java.routed_edges || rust.edge_sections != java.edge_sections {
        match kind {
            ParityFixtureKind::Libavoid => {
                causes.insert(ParityRootCause::LibavoidCompoundClusterModeling);
                causes.insert(ParityRootCause::LibavoidOptionMapping);
            }
            ParityFixtureKind::Interconnection => {
                causes.insert(ParityRootCause::HierarchicalPortRouting);
                causes.insert(ParityRootCause::DummyRestoration);
            }
            ParityFixtureKind::Layered => {
                causes.insert(ParityRootCause::LayeredBendGeneration);
            }
        }
    }

    if rust.bend_points != java.bend_points || bend_complexity_error.is_some() {
        match kind {
            ParityFixtureKind::Libavoid => {
                causes.insert(ParityRootCause::LibavoidOptionMapping);
                causes.insert(ParityRootCause::PostRouteNormalizationDrift);
            }
            _ => {
                causes.insert(ParityRootCause::LayeredSlotAssignment);
                causes.insert(ParityRootCause::LayeredBendGeneration);
            }
        }
    }

    if rust.orthogonal_violations != java.orthogonal_violations {
        causes.insert(ParityRootCause::PostRouteNormalizationDrift);
    }

    if let Some(err) = relaxed_error {
        let err = err.to_ascii_lowercase();
        if err.contains("graph stats differ") {
            causes.insert(ParityRootCause::DummyRestoration);
        }
        if err.contains("missing") || err.contains("presence mismatch") {
            causes.insert(ParityRootCause::HierarchicalPortRouting);
        }
    }

    if causes.is_empty() {
        causes.insert(match kind {
            ParityFixtureKind::Libavoid => ParityRootCause::LibavoidOptionMapping,
            ParityFixtureKind::Interconnection => ParityRootCause::HierarchicalPortRouting,
            ParityFixtureKind::Layered => ParityRootCause::LayeredBendGeneration,
        });
    }

    causes.into_iter().collect()
}

fn section_points(
    section: &serde_json::Map<String, serde_json::Value>,
) -> Result<Vec<(f32, f32)>, String> {
    let mut out = Vec::new();
    if let Some(start) = section.get("startPoint").and_then(serde_json::Value::as_object) {
        out.push((
            get_f32(start, "x").ok_or_else(|| "section startPoint missing x".to_string())?,
            get_f32(start, "y").ok_or_else(|| "section startPoint missing y".to_string())?,
        ));
    }
    if let Some(bends) = section.get("bendPoints").and_then(serde_json::Value::as_array) {
        for bend in bends {
            let Some(bend_obj) = bend.as_object() else {
                continue;
            };
            out.push((
                get_f32(bend_obj, "x").ok_or_else(|| "bend missing x".to_string())?,
                get_f32(bend_obj, "y").ok_or_else(|| "bend missing y".to_string())?,
            ));
        }
    }
    if let Some(end) = section.get("endPoint").and_then(serde_json::Value::as_object) {
        out.push((
            get_f32(end, "x").ok_or_else(|| "section endPoint missing x".to_string())?,
            get_f32(end, "y").ok_or_else(|| "section endPoint missing y".to_string())?,
        ));
    }
    Ok(out)
}

fn graph_stats(v: &serde_json::Value) -> Result<(usize, usize, usize), String> {
    let Some(obj) = v.as_object() else {
        return Ok((0, 0, 0));
    };
    let mut nodes = 1usize;
    let mut edges = 0usize;
    let mut ports = 0usize;

    if let Some(edge_arr) = obj.get("edges").and_then(|e| e.as_array()) {
        edges += edge_arr.len();
    }

    if let Some(ps) = obj.get("ports").and_then(|p| p.as_array()) {
        ports += ps.len();
    }

    if let Some(children) = obj.get("children").and_then(|c| c.as_array()) {
        for child in children {
            let (cn, ce, cp) = graph_stats(child)?;
            nodes += cn;
            edges += ce;
            ports += cp;
        }
    }
    Ok((nodes, edges, ports))
}

fn check_node_geometry(v: &serde_json::Value, is_root: bool, side: &str) -> Result<(), String> {
    let Some(obj) = v.as_object() else { return Ok(()) };
    let id = obj.get("id").and_then(|x| x.as_str()).unwrap_or("?");
    let path = format!("{} node {}", side, id);

    if !is_root {
        for key in ["x", "y", "width", "height"] {
            let n = get_f32(obj, key).ok_or_else(|| format!("{}: missing {}", path, key))?;
            if !n.is_finite() {
                return Err(format!("{}: {} is not finite", path, key));
            }
        }
    }

    if let Some(children) = obj.get("children").and_then(|c| c.as_array()) {
        for child in children {
            check_node_geometry(child, false, side)?;
        }
    }

    Ok(())
}
