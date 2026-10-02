//! Visual experiment for #217. Uses the shipped, pinned ELK router unchanged.
//! cargo run -p diagram_draw --example port_nodes -- product.json output-directory
use std::collections::{BTreeMap, BTreeSet};
use std::{env, fs, path::Path};

use diagram_draw::svg::Element;
use serde_json::{json, Value};

fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}
fn number(v: &Value) -> f64 {
    v.as_f64().unwrap_or(0.0)
}
fn array(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}

fn options() -> Value {
    json!({
        "elk.algorithm": "layered", "elk.direction": "RIGHT",
        "elk.edgeRouting": "ORTHOGONAL", "elk.hierarchyHandling": "INCLUDE_CHILDREN",
        "elk.layered.mergeEdges": "false", "elk.layered.mergeHierarchyEdges": "false",
        "elk.spacing.nodeNode": "32", "elk.layered.spacing.nodeNodeBetweenLayers": "64",
        "elk.spacing.edgeNode": "16", "elk.layered.spacing.edgeNodeBetweenLayers": "16",
        "elk.spacing.edgeEdge": "12", "elk.layered.spacing.edgeEdgeBetweenLayers": "12",
        "elk.padding": "[top=38,left=18,bottom=18,right=18]",
        "elk.nodeSize.constraints": "MINIMUM_SIZE", "org.eclipse.elk.json.edgeCoords": "ROOT"
    })
}

fn graph(draw: &Value) -> Result<Value, String> {
    let nodes = array(&draw["nodes"]);
    let edges = array(&draw["edges"]);
    let ids: BTreeSet<_> = nodes.iter().map(|n| text(&n["id"])).collect();
    let mut ports = BTreeSet::new();
    let mut degrees: BTreeMap<&str, usize> = BTreeMap::new();
    for edge in edges {
        for key in ["sourcePortId", "targetPortId"] {
            if let Some(id) = edge["attributes"][key].as_str() {
                *degrees.entry(id).or_default() += 1;
            }
        }
    }
    fn node(
        n: &Value,
        nodes: &[Value],
        degrees: &BTreeMap<&str, usize>,
        ports: &mut BTreeSet<String>,
        ancestors: &mut BTreeSet<String>,
    ) -> Result<Value, String> {
        let id = text(&n["id"]);
        if !ancestors.insert(id.into()) {
            return Err(format!("containment cycle at {id}"));
        }
        let mut children = Vec::new();
        for child in nodes
            .iter()
            .filter(|child| text(&child["attributes"]["containerId"]) == id)
        {
            children.push(node(child, nodes, degrees, ports, ancestors)?);
        }
        for p in array(&n["attributes"]["portDetails"]) {
            let port_id = text(&p["id"]);
            if port_id.is_empty() || !ports.insert(port_id.into()) {
                return Err(format!("missing or duplicate port identity {port_id:?}"));
            }
            let label = text(&p["name"]);
            children.push(json!({"id": port_id, "width": (label.chars().count() as f64 * 6.5 + 24.0).max(32.0),
                "height": (degrees.get(port_id).copied().unwrap_or(0) as f64 * 6.0 + 12.0).max(26.0),
                "spikeKind": "port", "spikeLabel": label}));
        }
        ancestors.remove(id);
        let label = text(&n["label"]);
        let mut result = json!({"id": id, "width": (label.chars().count() as f64*7.0+36.0).max(120.0),
            "height": 64, "spikeKind": "part", "spikeLabel": label,
            "spikeType": n["attributes"]["partType"], "layoutOptions": options()});
        result["layoutOptions"]["elk.nodeSize.minimum"] = json!(format!(
            "({},64)",
            (label.chars().count() as f64 * 7.0 + 36.0).max(120.0)
        ));
        if !children.is_empty() {
            result["children"] = json!(children);
        }
        Ok(result)
    }
    let mut children = Vec::new();
    for n in nodes
        .iter()
        .filter(|n| !ids.contains(text(&n["attributes"]["containerId"])))
    {
        children.push(node(n, nodes, &degrees, &mut ports, &mut BTreeSet::new())?);
    }
    let mut elk_edges = Vec::new();
    for edge in edges {
        let endpoint = |port_key: &str, node_key: &str| -> Result<String, String> {
            if let Some(id) = edge["attributes"][port_key].as_str() {
                if !ports.contains(id) {
                    return Err(format!(
                        "edge {} references absent port {id}",
                        text(&edge["id"])
                    ));
                }
                return Ok(id.into());
            }
            let id = text(&edge[node_key]);
            if !ids.contains(id) {
                return Err(format!("absent endpoint {id}"));
            }
            Ok(id.into())
        };
        let label = text(&edge["label"]);
        let mut elk_edge = json!({"id": edge["id"], "sources": [endpoint("sourcePortId", "source")?],
            "targets": [endpoint("targetPortId", "target")?], "spikeLabel": label});
        if !label.is_empty() {
            elk_edge["labels"] = json!([{"text": label, "width": label.chars().count() as f64 * 6.0,
                "height": 14, "layoutOptions": {"elk.edgeLabels.placement": "CENTER"}}]);
        }
        elk_edges.push(elk_edge);
    }
    Ok(json!({"id": "root", "layoutOptions": options(), "children": children, "edges": elk_edges}))
}

#[derive(Clone, Copy, Debug)]
struct Point(f64, f64);
fn point(v: &Value) -> Point {
    Point(number(&v["x"]), number(&v["y"]))
}
fn routes(layout: &Value) -> Vec<Vec<Point>> {
    array(&layout["edges"])
        .iter()
        .map(|e| {
            array(&e["sections"])
                .iter()
                .flat_map(|s| {
                    let mut points = vec![point(&s["startPoint"])];
                    points.extend(array(&s["bendPoints"]).iter().map(point));
                    points.push(point(&s["endPoint"]));
                    points
                })
                .collect()
        })
        .collect()
}

fn overlap(routes: &[Vec<Point>]) -> (usize, f64) {
    let mut count = 0;
    let mut total = 0.0;
    for (i, a) in routes.iter().enumerate() {
        for b in &routes[i + 1..] {
            for aa in a.windows(2) {
                for bb in b.windows(2) {
                    let [Point(ax, ay), Point(bx, by)] = [aa[0], aa[1]];
                    let [Point(cx, cy), Point(dx, dy)] = [bb[0], bb[1]];
                    let length = if (ay - by).abs() < 1e-6
                        && (cy - dy).abs() < 1e-6
                        && (ay - cy).abs() < 1e-6
                    {
                        ax.max(bx).min(cx.max(dx)) - ax.min(bx).max(cx.min(dx))
                    } else if (ax - bx).abs() < 1e-6
                        && (cx - dx).abs() < 1e-6
                        && (ax - cx).abs() < 1e-6
                    {
                        ay.max(by).min(cy.max(dy)) - ay.min(by).max(cy.min(dy))
                    } else {
                        0.0
                    };
                    if length > 1e-6 {
                        count += 1;
                        total += length;
                    }
                }
            }
        }
    }
    (count, total)
}

fn svg(layout: &Value, input: &Value) -> String {
    fn metadata(n: &Value, map: &mut BTreeMap<String, Value>) {
        map.insert(text(&n["id"]).into(), n.clone());
        for child in array(&n["children"]) {
            metadata(child, map);
        }
    }
    let mut labels = BTreeMap::new();
    metadata(input, &mut labels);
    let mut containers = Vec::new();
    let mut leaves = Vec::new();
    fn walk(
        n: &Value,
        ox: f64,
        oy: f64,
        labels: &BTreeMap<String, Value>,
        containers: &mut Vec<Element>,
        leaves: &mut Vec<Element>,
    ) {
        let x = ox + number(&n["x"]);
        let y = oy + number(&n["y"]);
        let w = number(&n["width"]);
        let h = number(&n["height"]);
        let m = &labels[text(&n["id"])];
        let port = text(&m["spikeKind"]) == "port";
        let is_container = !array(&n["children"]).is_empty();
        let rect = Element::new("rect")
            .attr_f("x", x)
            .attr_f("y", y)
            .attr_f("width", w)
            .attr_f("height", h)
            .attr("fill", if port { "#dbeafe" } else { "#ffffff" })
            .attr("fill-opacity", if is_container { "0.4" } else { "1" })
            .attr("stroke", if port { "#2563eb" } else { "#94a3b8" })
            .attr("rx", "3");
        let name = Element::new("text")
            .attr_f("x", x + 8.0)
            .attr_f("y", y + if port { 17.0 } else { 23.0 })
            .attr("font-size", if port { "11" } else { "13" })
            .attr("font-weight", if port { "400" } else { "600" })
            .text(text(&m["spikeLabel"]));
        let group = Element::new("g").child(rect).child(name);
        if is_container {
            containers.push(group);
        } else {
            leaves.push(group);
        }
        for child in array(&n["children"]) {
            walk(child, x, y, labels, containers, leaves);
        }
    }
    for child in array(&layout["children"]) {
        walk(child, 0.0, 0.0, &labels, &mut containers, &mut leaves);
    }
    let colors = [
        "#2563eb", "#dc2626", "#059669", "#9333ea", "#d97706", "#0891b2",
    ];
    let mut lines = Vec::new();
    for (i, (edge, points)) in array(&layout["edges"])
        .iter()
        .zip(routes(layout))
        .enumerate()
    {
        let p = points
            .iter()
            .map(|Point(x, y)| format!("{x},{y}"))
            .collect::<Vec<_>>()
            .join(" ");
        let original = array(&input["edges"])
            .iter()
            .find(|e| e["id"] == edge["id"])
            .unwrap();
        let mut line = Element::new("g").child(
            Element::new("polyline")
                .attr("points", p)
                .attr("fill", "none")
                .attr("stroke", colors[i % colors.len()])
                .attr("stroke-width", "1.5")
                .child(Element::new("title").text(text(&original["spikeLabel"]))),
        );
        for label in array(&edge["labels"]) {
            line = line.child(
                Element::new("text")
                    .attr_f("x", number(&label["x"]))
                    .attr_f("y", number(&label["y"]) + 11.0)
                    .attr("font-size", "10")
                    .attr("fill", colors[i % colors.len()])
                    .text(text(&original["spikeLabel"])),
            );
        }
        lines.push(line);
    }
    Element::new("svg")
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr(
            "viewBox",
            format!(
                "0 0 {} {}",
                number(&layout["width"]) + 20.0,
                number(&layout["height"]) + 20.0
            ),
        )
        .attr_f("width", number(&layout["width"]) + 20.0)
        .attr_f("height", number(&layout["height"]) + 20.0)
        .attr(
            "style",
            "font-family:Segoe UI,sans-serif;background:#f8fafc",
        )
        .child(
            Element::new("rect")
                .attr("width", "100%")
                .attr("height", "100%")
                .attr("fill", "#f8fafc"),
        )
        .extend(containers)
        .extend(lines)
        .extend(leaves)
        .to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        return Err("usage: port_nodes product.json output-directory".into());
    }
    let payload: Value = serde_json::from_str(&fs::read_to_string(&args[1])?)?;
    let out = Path::new(&args[2]);
    fs::create_dir_all(out)?;
    let baseline = diagram_draw::pipeline::draw_input_from_payload(&payload)?;
    fs::write(
        out.join("baseline.svg"),
        diagram_draw::draw_input::render_svg_from_json(&baseline, 1200.0, 800.0)?,
    )?;
    fs::write(
        out.join("baseline.draw.json"),
        serde_json::to_string_pretty(&baseline)?,
    )?;
    let input = graph(&baseline)?;
    let layout = diagram_layout::layout_value(&input)?;
    if array(&layout["edges"]).len() != array(&baseline["edges"]).len() {
        return Err("layout lost connectors".into());
    }
    fs::write(out.join("port-nodes.svg"), svg(&layout, &input))?;
    fs::write(
        out.join("port-nodes.input.json"),
        serde_json::to_string_pretty(&input)?,
    )?;
    fs::write(
        out.join("port-nodes.layout.json"),
        serde_json::to_string_pretty(&layout)?,
    )?;
    let baseline_routes: Vec<_> = array(&baseline["interconnectionLayout"]["edges"])
        .iter()
        .map(|e| array(&e["routePoints"]).iter().map(point).collect())
        .collect();
    let (old_count, old_length) = overlap(&baseline_routes);
    let (new_count, new_length) = overlap(&routes(&layout));
    let report = json!({"elkrsRevision":diagram_layout::ELKRS_REVISION,"connectors":array(&layout["edges"]).len(),
        "baseline":{"sharedSegments":old_count,"pairwiseSharedLength":old_length},
        "portNodes":{"sharedSegments":new_count,"pairwiseSharedLength":new_length,"width":layout["width"],"height":layout["height"]}});
    println!("{}", serde_json::to_string_pretty(&report)?);
    fs::write(
        out.join("report.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    fs::write(
        out.join("comparison.html"),
        r#"<!doctype html>
<meta charset="utf-8"><title>Port nodes routing experiment</title>
<style>body{font:16px system-ui;margin:24px;background:#f8fafc;color:#172033}main{display:grid;grid-template-columns:1fr 1fr;gap:24px}img{width:100%;border:1px solid #cbd5e1}a{color:#2563eb} @media(max-width:900px){main{grid-template-columns:1fr}}</style>
<h1>Port nodes routing experiment</h1>
<p>Same semantic diagram, existing pinned elkrs. Click either drawing for full size.</p>
<main><section><h2>Baseline: current renderer</h2><a href="baseline.svg"><img src="baseline.svg" alt="Current diagram"></a></section>
<section><h2>Experiment: ports as child nodes</h2><a href="port-nodes.svg"><img src="port-nodes.svg" alt="Experimental diagram"></a></section></main>
<p>Experimental drawing uses simplified styling and colours per connector. Ports are nested boxes; routes and connector label positions come directly from ELK. Ports are not fixed to part borders.</p>
<p><a href="report.json">Shared-segment measurements</a> · <a href="port-nodes.input.json">ELK input</a> · <a href="port-nodes.layout.json">ELK output</a></p>"#,
    )?;
    Ok(())
}
