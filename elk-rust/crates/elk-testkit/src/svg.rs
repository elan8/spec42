use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use elk_core::{Graph, LayoutDirection, LayoutReport, Point, Rect};

const SNAPSHOT_ENV: &str = "ELK_TESTKIT_WRITE_SVG";

pub fn maybe_write_svg_snapshot(
    fixture: &str,
    direction: LayoutDirection,
    graph: &Graph,
    report: &LayoutReport,
) -> io::Result<Option<PathBuf>> {
    if !snapshot_enabled() {
        return Ok(None);
    }

    let dir = snapshot_dir()?;
    fs::create_dir_all(&dir)?;
    let path = dir.join(snapshot_file_name(fixture, direction));
    let svg = render_graph_svg(fixture, direction, graph, report);
    fs::write(&path, svg)?;
    Ok(Some(path))
}

pub fn render_graph_svg(
    fixture: &str,
    direction: LayoutDirection,
    graph: &Graph,
    report: &LayoutReport,
) -> String {
    let width = graph.bounds.size.width.max(240.0) + 48.0;
    let height = graph.bounds.size.height.max(160.0) + 96.0;
    let mut svg = String::new();
    svg.push_str(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {:.1} {:.1}" width="{:.1}" height="{:.1}" font-family="ui-monospace, SFMono-Regular, Consolas, monospace">"##,
        width, height, width, height
    ));
    svg.push_str(r##"<defs><pattern id="grid" width="24" height="24" patternUnits="userSpaceOnUse"><path d="M 24 0 L 0 0 0 24" fill="none" stroke="#e5e7eb" stroke-width="1"/></pattern></defs>"##);
    svg.push_str(r##"<rect width="100%" height="100%" fill="#f7f5ef"/>"##);
    svg.push_str(r##"<rect width="100%" height="100%" fill="url(#grid)" opacity="0.5"/>"##);
    svg.push_str(r##"<g transform="translate(24 28)">"##);
    svg.push_str(&format!(
        r##"<text x="0" y="-8" font-size="16" font-weight="700" fill="#111827">{} ({:?})</text>"##,
        escape_xml(fixture),
        direction
    ));
    svg.push_str(&format!(
        r##"<text x="0" y="12" font-size="11" fill="#475569">layers={} crossings={} bends={} components={} packed={} labels={}</text>"##,
        report.stats.layers,
        report.stats.crossings_after,
        report.stats.bend_points,
        report.stats.component_count,
        report.stats.packed_components,
        graph.labels.len()
    ));

    svg.push_str(r##"<g fill="none" stroke-linejoin="round" stroke-linecap="round">"##);
    for edge in &graph.edges {
        let stroke = if edge.labels.is_empty() { "#64748b" } else { "#0f766e" };
        let width = if edge.sections.iter().any(|section| section.bend_points.len() > 3) {
            2.5
        } else {
            2.0
        };
        for section in &edge.sections {
            let mut data = format!("M {:.1} {:.1}", section.start.x, section.start.y);
            for point in &section.bend_points {
                data.push_str(&format!(" L {:.1} {:.1}", point.x, point.y));
            }
            data.push_str(&format!(" L {:.1} {:.1}", section.end.x, section.end.y));
            svg.push_str(&format!(
                r##"<path d="{}" stroke="{}" stroke-width="{:.1}" opacity="0.95"/>"##,
                data, stroke, width
            ));
        }
    }
    svg.push_str("</g>");

    svg.push_str(r##"<g>"##);
    for node in &graph.nodes {
        let is_compound = !node.children.is_empty();
        let fill = if is_compound { "#eef2ff" } else { "#ffffff" };
        let stroke = if is_compound { "#4f46e5" } else { "#0f172a" };
        let dash = if is_compound { "6 4" } else { "" };
        if is_compound {
            svg.push_str(&format!(
                r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="10" fill="{}" stroke="{}" stroke-width="1.6" stroke-dasharray="{}"/>"##,
                node.bounds.origin.x,
                node.bounds.origin.y,
                node.bounds.size.width,
                node.bounds.size.height,
                fill,
                stroke,
                dash
            ));
        } else {
            svg.push_str(&format!(
                r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="10" fill="{}" stroke="{}" stroke-width="1.6"/>"##,
                node.bounds.origin.x,
                node.bounds.origin.y,
                node.bounds.size.width,
                node.bounds.size.height,
                fill,
                stroke
            ));
        }
        svg.push_str(&format!(
            r##"<text x="{:.1}" y="{:.1}" font-size="10" fill="#334155">{:?}</text>"##,
            node.bounds.origin.x + 8.0,
            node.bounds.origin.y + 14.0,
            node.id
        ));
    }
    svg.push_str("</g>");

    svg.push_str(r##"<g fill="#2563eb" stroke="#1d4ed8" stroke-width="1">"##);
    for port in &graph.ports {
        svg.push_str(&format!(
            r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="2"/>"##,
            port.bounds.origin.x,
            port.bounds.origin.y,
            port.bounds.size.width,
            port.bounds.size.height
        ));
    }
    svg.push_str("</g>");

    svg.push_str(r##"<g fill="#fff7d6" stroke="#d97706" stroke-width="1">"##);
    for label in &graph.labels {
        svg.push_str(&format!(
            r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="4"/>"##,
            label.position.x,
            label.position.y,
            label.size.width.max(24.0),
            label.size.height.max(14.0)
        ));
        svg.push_str(&format!(
            r##"<text x="{:.1}" y="{:.1}" font-size="10" fill="#78350f">{}</text>"##,
            label.position.x + 4.0,
            label.position.y + label.size.height.max(14.0) - 4.0,
            escape_xml(&label.text)
        ));
    }
    svg.push_str("</g>");

    svg.push_str("</g></svg>");
    svg
}

pub fn snapshot_file_name(fixture: &str, direction: LayoutDirection) -> String {
    format!("{}-{:?}.svg", sanitize(fixture), direction)
}

pub fn snapshot_dir() -> io::Result<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest_dir
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| io::Error::other("workspace root not found"))?;
    Ok(workspace.join("target").join("elk-testkit-snapshots"))
}

fn snapshot_enabled() -> bool {
    matches!(
        env::var(SNAPSHOT_ENV).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[allow(dead_code)]
fn _rect_to_debug(rect: Rect) -> (Point, Point) {
    (
        rect.origin,
        Point::new(
            rect.origin.x + rect.size.width,
            rect.origin.y + rect.size.height,
        ),
    )
}
