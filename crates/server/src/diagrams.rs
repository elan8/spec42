use std::path::{Path, PathBuf};

use kernel::build_sysml_visualization_for_paths;
use semantic_core::{GraphNodeDto, SysmlGraphDto, SysmlVisualizationResultDto};

use crate::cli::{DiagramExportArgs, DiagramExportFormat};

const EXPORTABLE_VIEWS: &[&str] = &[
    "general-view",
    "interconnection-view",
    "action-flow-view",
    "state-transition-view",
    "sequence-view",
    "browser-view",
    "grid-view",
    "geometry-view",
];

#[derive(Debug, Clone)]
pub struct DiagramExportSummary {
    pub output_dir: PathBuf,
    pub exported: usize,
}

pub fn export_diagrams(
    args: &DiagramExportArgs,
    library_paths: &[PathBuf],
) -> Result<DiagramExportSummary, String> {
    std::fs::create_dir_all(&args.output)
        .map_err(|err| format!("Failed to create {}: {err}", args.output.display()))?;
    let views = requested_views(args.view.as_str())?;
    let mut exported = 0;
    for view in views {
        let payload = build_sysml_visualization_for_paths(
            args.path.as_path(),
            args.workspace_root.as_deref(),
            library_paths,
            view,
            None,
        )?;
        let extension = match args.format {
            DiagramExportFormat::Json => "json",
            DiagramExportFormat::Svg => "svg",
        };
        let output_path = args
            .output
            .join(format!("{}.{}", safe_file_stem(view), extension));
        match args.format {
            DiagramExportFormat::Json => write_json(output_path.as_path(), &payload)?,
            DiagramExportFormat::Svg => write_svg(output_path.as_path(), &payload)?,
        }
        exported += 1;
    }
    Ok(DiagramExportSummary {
        output_dir: args.output.clone(),
        exported,
    })
}

fn requested_views(view: &str) -> Result<Vec<&'static str>, String> {
    if view == "all" {
        return Ok(EXPORTABLE_VIEWS.to_vec());
    }
    EXPORTABLE_VIEWS
        .iter()
        .copied()
        .find(|candidate| *candidate == view)
        .map(|view| vec![view])
        .ok_or_else(|| {
            format!(
                "Unsupported export view '{view}'. Expected one of: all, {}",
                EXPORTABLE_VIEWS.join(", ")
            )
        })
}

fn write_json(path: &Path, payload: &SysmlVisualizationResultDto) -> Result<(), String> {
    let raw = serde_json::to_string_pretty(payload)
        .map_err(|err| format!("Failed to serialize diagram payload: {err}"))?;
    std::fs::write(path, raw).map_err(|err| format!("Failed to write {}: {err}", path.display()))
}

fn write_svg(path: &Path, payload: &SysmlVisualizationResultDto) -> Result<(), String> {
    let svg = deterministic_svg(payload);
    std::fs::write(path, svg).map_err(|err| format!("Failed to write {}: {err}", path.display()))
}

fn deterministic_svg(payload: &SysmlVisualizationResultDto) -> String {
    let graph = graph_for_payload(payload);
    let mut nodes = graph.map(|graph| graph.nodes.clone()).unwrap_or_default();
    nodes.sort_by(|a, b| a.id.cmp(&b.id).then_with(|| a.name.cmp(&b.name)));

    let width = 960;
    let row_height = 52;
    let height = ((nodes.len().max(1) + 2) as i32 * row_height).max(180);
    let mut out = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" data-spec42-view="{}">"#,
        xml_escape(&payload.view)
    );
    out.push_str(r#"<style>text{font-family:Arial,sans-serif;font-size:13px}.title{font-size:18px;font-weight:700}.node{fill:#fff;stroke:#24292f;stroke-width:1.4}.muted{fill:#57606a}</style>"#);
    out.push_str(&format!(
        r#"<text class="title" x="24" y="34">{}</text>"#,
        xml_escape(
            payload
                .selected_view_name
                .as_deref()
                .unwrap_or(payload.view.as_str())
        )
    ));
    out.push_str(&format!(
        r#"<text class="muted" x="24" y="56">view: {}</text>"#,
        xml_escape(&payload.view)
    ));

    if nodes.is_empty() {
        out.push_str(&format!(
            r#"<text class="muted" x="24" y="96">{}</text>"#,
            xml_escape(
                payload
                    .empty_state_message
                    .as_deref()
                    .unwrap_or("No diagram nodes were available for this view.")
            )
        ));
        out.push_str("</svg>");
        return out;
    }

    for (index, node) in nodes.iter().enumerate() {
        append_node_row(&mut out, node, 88 + (index as i32 * row_height));
    }
    out.push_str("</svg>");
    out
}

fn append_node_row(out: &mut String, node: &GraphNodeDto, y: i32) {
    out.push_str(&format!(
        r#"<g class="diagram-node" data-element-id="{}" data-element-name="{}" data-qualified-name="{}">"#,
        xml_escape(&node.id),
        xml_escape(&node.name),
        xml_escape(&node.id)
    ));
    out.push_str(&format!(
        r#"<rect class="node" x="24" y="{y}" width="912" height="40" rx="4"/>"#
    ));
    out.push_str(&format!(
        r#"<text x="40" y="{}">{}</text>"#,
        y + 17,
        xml_escape(&node.name)
    ));
    out.push_str(&format!(
        r#"<text class="muted" x="40" y="{}">{}</text>"#,
        y + 33,
        xml_escape(&node.element_type)
    ));
    out.push_str("</g>");
}

fn graph_for_payload(payload: &SysmlVisualizationResultDto) -> Option<&SysmlGraphDto> {
    payload
        .graph
        .as_ref()
        .or(payload.general_view_graph.as_ref())
}

fn safe_file_stem(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use semantic_core::{GraphNodeDto, PositionDto, RangeDto, SysmlGraphDto};
    use std::collections::HashMap;

    fn zero_range() -> RangeDto {
        RangeDto {
            start: PositionDto {
                line: 0,
                character: 0,
            },
            end: PositionDto {
                line: 0,
                character: 0,
            },
        }
    }

    #[test]
    fn svg_preserves_source_element_ids() {
        let payload = SysmlVisualizationResultDto {
            version: 1,
            view: "general-view".to_string(),
            workspace_root_uri: "file:///demo".to_string(),
            model_ready: true,
            view_candidates: Vec::new(),
            selected_view: None,
            selected_view_name: Some("General".to_string()),
            empty_state_message: None,
            package_groups: None,
            graph: Some(SysmlGraphDto {
                nodes: vec![GraphNodeDto {
                    id: "P::x".to_string(),
                    element_type: "part".to_string(),
                    name: "x".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                }],
                edges: Vec::new(),
            }),
            general_view_graph: None,
            workspace_model: None,
            activity_diagrams: None,
            sequence_diagrams: None,
            ibd: None,
            stats: None,
        };
        let svg = deterministic_svg(&payload);
        assert!(svg.contains("data-element-id=\"P::x\""));
        assert!(svg.contains("data-spec42-view=\"general-view\""));
    }

    #[test]
    fn requested_views_rejects_unknown_view() {
        let err = requested_views("unknown").expect_err("unknown view should fail");
        assert!(err.contains("Unsupported export view"));
    }
}
