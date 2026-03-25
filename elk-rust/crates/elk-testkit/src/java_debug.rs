use serde_json::Value;

fn edge_endpoint_signature(edge: &Value) -> Option<String> {
    let sources = edge.get("sources")?.as_array()?;
    let targets = edge.get("targets")?.as_array()?;
    let endpoint_id = |v: &Value| -> Option<String> {
        if let Some(s) = v.as_str() {
            return Some(s.to_string());
        }
        let obj = v.as_object()?;
        if let Some(port) = obj.get("port").and_then(Value::as_str) {
            return Some(format!("port:{port}"));
        }
        if let Some(node) = obj.get("node").and_then(Value::as_str) {
            return Some(format!("node:{node}"));
        }
        if let Some(id) = obj.get("id").and_then(Value::as_str) {
            return Some(id.to_string());
        }
        None
    };
    let src = sources
        .iter()
        .filter_map(endpoint_id)
        .collect::<Vec<_>>()
        .join(",");
    let dst = targets
        .iter()
        .filter_map(endpoint_id)
        .collect::<Vec<_>>()
        .join(",");
    if src.is_empty() || dst.is_empty() {
        return None;
    }
    Some(format!("{src}->{dst}"))
}

fn edge_polyline_points(edge: &Value) -> Vec<(f64, f64)> {
    let mut pts = Vec::new();
    let sections = edge.get("sections").and_then(Value::as_array);
    if let Some(sections) = sections {
        for sec in sections {
            let start = sec.get("startPoint").and_then(Value::as_object);
            let end = sec.get("endPoint").and_then(Value::as_object);
            let bends = sec.get("bendPoints").and_then(Value::as_array);
            let mut push_point = |obj: Option<&serde_json::Map<String, Value>>| {
                if let Some(o) = obj {
                    if let (Some(x), Some(y)) = (o.get("x").and_then(Value::as_f64), o.get("y").and_then(Value::as_f64)) {
                        pts.push((x, y));
                    }
                }
            };
            push_point(start);
            if let Some(bends) = bends {
                for bp in bends {
                    push_point(bp.as_object());
                }
            }
            push_point(end);
        }
    }
    // de-dup consecutive identical points
    pts.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-9 && (a.1 - b.1).abs() < 1e-9);
    pts
}

fn is_non_orth(a: (f64, f64), b: (f64, f64), eps: f64) -> bool {
    (a.0 - b.0).abs() > eps && (a.1 - b.1).abs() > eps
}

/// Build a human-readable debug report from Java ELK layout JSON output.
///
/// Focuses on the same issues we care about when eyeballing the SVG: detours, non-orth segments,
/// and crossings/bend-heavy edges.
pub fn build_java_layout_debug(java_json: &Value, edge_limit: usize) -> String {
    let mut out = String::new();

    fn walk_counts(v: &Value, node_count: &mut usize, edge_count: &mut usize) {
        if let Some(obj) = v.as_object() {
            if obj.contains_key("id") {
                *node_count += 1;
            }
            if let Some(edges) = obj.get("edges").and_then(Value::as_array) {
                *edge_count += edges.len();
            }
            if let Some(children) = obj.get("children").and_then(Value::as_array) {
                for c in children {
                    walk_counts(c, node_count, edge_count);
                }
            }
        }
    }

    fn walk_edges(v: &Value, out: &mut Vec<Value>) {
        if let Some(obj) = v.as_object() {
            if let Some(edges) = obj.get("edges").and_then(Value::as_array) {
                out.extend(edges.iter().cloned());
            }
            if let Some(children) = obj.get("children").and_then(Value::as_array) {
                for c in children {
                    walk_edges(c, out);
                }
            }
        }
    }

    let mut node_count = 0usize;
    let mut edge_count = 0usize;
    walk_counts(java_json, &mut node_count, &mut edge_count);
    out.push_str("java-elk layout debug\n");
    out.push_str(&format!("node_count={node_count}\n"));
    out.push_str(&format!("edge_count={edge_count}\n"));

    // Header overlap heuristic for container nodes.
    // In our SVG, container headers are ~44px tall; in the adapter we currently target
    // `container_header_height + 10px` for top padding. Use the same threshold here so the
    // debug report directly confirms whether the padding took effect in Java output.
    const HEADER_CLEARANCE_HEIGHT_FALLBACK: f64 = 54.0;
    fn get_xy(obj: &serde_json::Map<String, Value>) -> (f64, f64) {
        (
            obj.get("x").and_then(Value::as_f64).unwrap_or(0.0),
            obj.get("y").and_then(Value::as_f64).unwrap_or(0.0),
        )
    }
    fn parse_padding_top(s: &str) -> Option<f64> {
        // Accepts a few common ELK string forms, best-effort:
        // - "[top=54.0,left=40.0,bottom=40.0,right=40.0]"
        // - "[54.0,40.0,40.0,40.0]"
        let s = s.trim();
        if let Some(i) = s.find("top=") {
            let rest = &s[i + "top=".len()..];
            let num = rest
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+')
                .collect::<String>();
            return num.parse::<f64>().ok();
        }
        if s.starts_with('[') && s.ends_with(']') {
            let inner = &s[1..s.len() - 1];
            let first = inner.split(',').next()?.trim();
            return first.parse::<f64>().ok();
        }
        None
    }
    fn container_clearance_height(obj: &serde_json::Map<String, Value>) -> (f64, &'static str) {
        // Prefer actual top padding if exported by Java ELK, otherwise fallback to header height.
        let lo = obj.get("layoutOptions").and_then(Value::as_object);
        if let Some(lo) = lo {
            // ELK may export either key depending on config.
            let keys = ["elk.padding", "org.eclipse.elk.padding"];
            for k in keys {
                if let Some(s) = lo.get(k).and_then(Value::as_str) {
                    if let Some(top) = parse_padding_top(s) {
                        return (top, k);
                    }
                }
            }
        }
        (HEADER_CLEARANCE_HEIGHT_FALLBACK, "<fallback>")
    }
    fn count_header_overlaps(v: &Value, overlaps: &mut Vec<String>) {
        let Some(obj) = v.as_object() else { return };
        let Some(children) = obj.get("children").and_then(Value::as_array) else { return };
        let container_id = obj
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<root>");
        let is_root_container = container_id == "root" || container_id == "<root>";
        // In ELK JSON, child `x`/`y` are typically *parent-relative* (even when exporting with
        // shapeCoords/edgeCoords ROOT in some configurations). For header clearance we care about
        // child placement inside the container's content box, so compare local child y against
        // the container's top clearance directly.
        let (clearance_h, clearance_src) = container_clearance_height(obj);
        let threshold_y = clearance_h;
        for c in children {
            if let Some(child_obj) = c.as_object() {
                if let (Some(child_id), Some(y)) = (
                    child_obj.get("id").and_then(Value::as_str),
                    child_obj.get("y").and_then(Value::as_f64),
                ) {
                    if !is_root_container && y < threshold_y {
                        overlaps.push(format!(
                            "container={container_id} clearance_h={clearance_h:.1} clearance_src={clearance_src} child={child_id} child_local_y={y:.1} < threshold_local_y={threshold_y:.1}"
                        ));
                    }
                }
            }
            count_header_overlaps(c, overlaps);
        }
    }
    let mut header_overlaps = Vec::new();
    count_header_overlaps(java_json, &mut header_overlaps);
    out.push_str(&format!(
        "header_overlap_count={}\n",
        header_overlaps.len()
    ));
    for line in header_overlaps.iter().take(40) {
        out.push_str(&format!("header_overlap: {line}\n"));
    }

    let mut edge_rows = Vec::new();
    let mut edges = Vec::new();
    walk_edges(java_json, &mut edges);
    for e in &edges {
        let sig = edge_endpoint_signature(e).unwrap_or_else(|| "<missing-sig>".to_string());
        let pts = edge_polyline_points(e);
        let bends = pts.len().saturating_sub(2);
        let mut non_orth = 0usize;
        for w in pts.windows(2) {
            if is_non_orth(w[0], w[1], 1e-6) {
                non_orth += 1;
            }
        }
        edge_rows.push((non_orth, bends, sig, pts));
    }

    edge_rows.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)).then_with(|| a.2.cmp(&b.2)));

    let total_non_orth: usize = edge_rows.iter().map(|r| r.0).sum();
    let total_bends: usize = edge_rows.iter().map(|r| r.1).sum();
    out.push_str(&format!("total_non_orth_segments={total_non_orth}\n"));
    out.push_str(&format!("total_bend_points={total_bends}\n"));

    out.push_str("\nworst_edges:\n");
    for (idx, (non_orth, bends, sig, pts)) in edge_rows.iter().take(edge_limit).enumerate() {
        out.push_str(&format!(
            "{idx}: non_orth_segments={non_orth} bends={bends} sig={sig}\n"
        ));
        if !pts.is_empty() {
            out.push_str("  points:");
            for (x, y) in pts.iter().take(120) {
                out.push_str(&format!(" ({x:.1},{y:.1})"));
            }
            if pts.len() > 120 {
                out.push_str(" ...");
            }
            out.push('\n');
        }
    }

    out
}

