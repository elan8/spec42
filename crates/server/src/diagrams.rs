#![allow(dead_code)] // Legacy Rust SVG helpers are retained for internal layout probes while public SVG uses shared renderer.

use std::path::{Path, PathBuf};

use kernel::build_sysml_visualization_for_paths;
use semantic_core::{
    build_elk_graph_from_scene, GraphNodeDto, RangeDto, SysmlGraphDto, SysmlVisualizationResultDto,
};
use serde::{Deserialize, Serialize};

use crate::cli::{DiagramExportArgs, DiagramExportFormat};
use crate::headless_renderer::render_shared_svg;
use crate::elk_layout::layout_elk_graph;

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

pub fn build_diagram_payload(
    path: &Path,
    workspace_root: Option<&Path>,
    library_paths: &[PathBuf],
    view: &str,
    selected_view: Option<&str>,
) -> Result<SysmlVisualizationResultDto, String> {
    let export_view = resolve_renderer_view(view)?;
    build_sysml_visualization_for_paths(
        path,
        workspace_root,
        library_paths,
        export_view,
        selected_view,
    )
}

pub fn render_diagram(
    payload: &SysmlVisualizationResultDto,
    format: DiagramExportFormat,
) -> Result<(String, &'static str), String> {
    match format {
        DiagramExportFormat::Json => {
            let raw = serde_json::to_string_pretty(payload)
                .map_err(|err| format!("Failed to serialize diagram payload: {err}"))?;
            Ok((raw, "application/json"))
        }
        DiagramExportFormat::Svg => {
            let raw = serde_json::to_string(payload)
                .map_err(|err| format!("Failed to serialize diagram payload for SVG export: {err}"))?;
            let svg = render_shared_svg(&raw)?;
            Ok((svg, "image/svg+xml"))
        }
    }
}

pub fn render_diagram_for_path(
    path: &Path,
    workspace_root: Option<&Path>,
    library_paths: &[PathBuf],
    view: &str,
    selected_view: Option<&str>,
    format: DiagramExportFormat,
) -> Result<(String, &'static str), String> {
    let payload = build_diagram_payload(path, workspace_root, library_paths, view, selected_view)?;
    render_diagram(&payload, format)
}

pub fn export_diagrams(
    args: &DiagramExportArgs,
    library_paths: &[PathBuf],
) -> Result<DiagramExportSummary, String> {
    std::fs::create_dir_all(&args.output)
        .map_err(|err| format!("Failed to create {}: {err}", args.output.display()))?;
    if args.view == "model-views" {
        return export_model_views(args, library_paths);
    }
    if args.selected_view.is_some() {
        return export_single_diagram(
            args,
            library_paths,
            args.view.as_str(),
            args.selected_view.as_deref(),
        );
    }
    let views = requested_renderer_views(args.view.as_str())?;
    let mut exported = 0;
    for view in views {
        export_single_diagram(args, library_paths, view, None)?;
        exported += 1;
    }
    Ok(DiagramExportSummary {
        output_dir: args.output.clone(),
        exported,
    })
}

fn export_model_views(
    args: &DiagramExportArgs,
    library_paths: &[PathBuf],
) -> Result<DiagramExportSummary, String> {
    if args.selected_view.is_some() {
        return Err("--selected-view cannot be combined with --view model-views".to_string());
    }
    let probe = build_sysml_visualization_for_paths(
        args.path.as_path(),
        args.workspace_root.as_deref(),
        library_paths,
        "general-view",
        None,
    )?;
    let candidates: Vec<_> = probe
        .view_candidates
        .iter()
        .filter(|candidate| candidate.supported)
        .collect();
    if candidates.is_empty() {
        return Err("No supported explicit model views were found in the workspace".to_string());
    }
    let extension = match args.format {
        DiagramExportFormat::Json => "json",
        DiagramExportFormat::Svg => "svg",
    };
    let mut exported = 0;
    for candidate in candidates {
        let renderer_view = candidate
            .renderer_view
            .as_deref()
            .ok_or_else(|| format!("View '{}' has no renderer mapping", candidate.name))?;
        let payload = build_sysml_visualization_for_paths(
            args.path.as_path(),
            args.workspace_root.as_deref(),
            library_paths,
            renderer_view,
            Some(candidate.name.as_str()),
        )?;
        let output_path =
            args.output
                .join(format!("{}.{}", safe_file_stem(&candidate.name), extension));
        let (body, _) = render_diagram(&payload, args.format)?;
        std::fs::write(&output_path, body)
            .map_err(|err| format!("Failed to write {}: {err}", output_path.display()))?;
        exported += 1;
    }
    Ok(DiagramExportSummary {
        output_dir: args.output.clone(),
        exported,
    })
}

fn export_single_diagram(
    args: &DiagramExportArgs,
    library_paths: &[PathBuf],
    view: &str,
    selected_view: Option<&str>,
) -> Result<DiagramExportSummary, String> {
    let payload = build_diagram_payload(
        args.path.as_path(),
        args.workspace_root.as_deref(),
        library_paths,
        view,
        selected_view,
    )?;
    let extension = match args.format {
        DiagramExportFormat::Json => "json",
        DiagramExportFormat::Svg => "svg",
    };
    let file_stem = payload
        .selected_view_name
        .as_deref()
        .map(safe_file_stem)
        .unwrap_or_else(|| safe_file_stem(payload.view.as_str()));
    let output_path = args.output.join(format!("{file_stem}.{extension}"));
    let (body, _) = render_diagram(&payload, args.format)?;
    std::fs::write(&output_path, body)
        .map_err(|err| format!("Failed to write {}: {err}", output_path.display()))?;
    Ok(DiagramExportSummary {
        output_dir: args.output.clone(),
        exported: 1,
    })
}

fn resolve_renderer_view(view: &str) -> Result<&'static str, String> {
    if view == "model-views" {
        return Err(
            "Use diagrams export --view model-views without --selected-view to export all explicit views"
                .to_string(),
        );
    }
    requested_renderer_views(view)?
        .into_iter()
        .next()
        .ok_or_else(|| format!("No renderer view resolved for '{view}'"))
}

fn requested_renderer_views(view: &str) -> Result<Vec<&'static str>, String> {
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
                "Unsupported export view '{view}'. Expected one of: all, model-views, {}",
                EXPORTABLE_VIEWS.join(", ")
            )
        })
}

fn native_svg(payload: &SysmlVisualizationResultDto, export_view: &str) -> String {
    let graph = graph_for_payload(payload);
    let mut nodes = graph.map(|graph| graph.nodes.clone()).unwrap_or_default();
    nodes.sort_by(|a, b| a.id.cmp(&b.id).then_with(|| a.name.cmp(&b.name)));

    let width = 960;
    let row_height = 52;
    let height = ((nodes.len().max(1) + 2) as i32 * row_height).max(180);
    let mut out = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" data-spec42-view="{}" data-layout-engine="native">"#,
        xml_escape(export_view)
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

fn elk_svg(payload: &SysmlVisualizationResultDto, export_view: &str) -> Result<String, String> {
    let source = build_elk_source(payload, export_view)?;
    if source.graph.children.is_empty() {
        return Ok(empty_elk_svg(payload, export_view));
    }
    let graph_json = serde_json::to_string(&source.graph)
        .map_err(|err| format!("Failed to serialize ELK graph: {err}"))?;
    let layouted_json = layout_elk_graph(&graph_json)?;
    let layouted: ElkGraph = serde_json::from_str(&layouted_json)
        .map_err(|err| format!("ELK returned invalid graph JSON: {err}"))?;
    render_elk_svg(payload, export_view, &layouted, &source)
}

fn empty_elk_svg(payload: &SysmlVisualizationResultDto, export_view: &str) -> String {
    let mut out = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="960" height="180" viewBox="0 0 960 180" data-spec42-view="{}" data-layout-engine="elkjs-quickjs">"#,
        xml_escape(export_view)
    );
    out.push_str(r#"<style>text{font-family:Arial,sans-serif;font-size:13px}.title{font-size:18px;font-weight:700}.muted{fill:#57606a}</style>"#);
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
        r#"<text class="muted" x="24" y="80">{}</text></svg>"#,
        xml_escape(
            payload
                .empty_state_message
                .as_deref()
                .unwrap_or("No diagram nodes were available for this view.")
        )
    ));
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkGraph {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<ElkNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    edges: Vec<ElkEdge>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    ports: Vec<ElkPort>,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    layout_options: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkNode {
    id: String,
    width: f64,
    height: f64,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<ElkNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    ports: Vec<ElkPort>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    edges: Vec<ElkEdge>,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    layout_options: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkPort {
    id: String,
    width: f64,
    height: f64,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    layout_options: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkEdge {
    id: String,
    sources: Vec<String>,
    targets: Vec<String>,
    #[serde(default)]
    sections: Vec<ElkSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElkSection {
    start_point: ElkPoint,
    end_point: ElkPoint,
    #[serde(default)]
    bend_points: Vec<ElkPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ElkPoint {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct ElkSource {
    graph: ElkGraph,
    nodes: std::collections::HashMap<String, SvgNodeMeta>,
    edges: std::collections::HashMap<String, SvgEdgeMeta>,
}

#[derive(Debug, Clone)]
struct SvgNodeMeta {
    id: String,
    name: String,
    element_type: String,
    qualified_name: String,
    uri: Option<String>,
    range: Option<RangeDto>,
}

#[derive(Debug, Clone)]
struct SvgEdgeMeta {
    id: String,
    label: Option<String>,
    rel_type: String,
}

fn build_elk_source(
    payload: &SysmlVisualizationResultDto,
    export_view: &str,
) -> Result<ElkSource, String> {
    match export_view {
        "interconnection-view" => build_interconnection_elk_source(payload),
        "action-flow-view" => build_activity_elk_source(payload),
        "state-transition-view" => build_state_elk_source(payload),
        _ => build_graph_elk_source(payload),
    }
}

fn build_graph_elk_source(payload: &SysmlVisualizationResultDto) -> Result<ElkSource, String> {
    let graph = graph_for_payload(payload).ok_or_else(|| {
        format!(
            "ELK-backed SVG export for {} requires a graph payload",
            payload.view
        )
    })?;
    let mut children = Vec::new();
    let mut nodes = std::collections::HashMap::new();
    for node in &graph.nodes {
        children.push(ElkNode {
            id: node.id.clone(),
            width: node_width(node),
            height: node_height(node),
            x: 0.0,
            y: 0.0,
            children: Vec::new(),
            ports: Vec::new(),
            edges: Vec::new(),
            layout_options: std::collections::BTreeMap::new(),
        });
        nodes.insert(
            node.id.clone(),
            SvgNodeMeta {
                id: node.id.clone(),
                name: node.name.clone(),
                element_type: node.element_type.clone(),
                qualified_name: node.id.clone(),
                uri: node.uri.clone(),
                range: Some(node.range.clone()),
            },
        );
    }
    let node_ids: std::collections::HashSet<&str> =
        graph.nodes.iter().map(|node| node.id.as_str()).collect();
    let mut edge_meta = std::collections::HashMap::new();
    let mut edges = Vec::new();
    for (index, edge) in graph.edges.iter().enumerate() {
        if !node_ids.contains(edge.source.as_str()) || !node_ids.contains(edge.target.as_str()) {
            continue;
        }
        let id = format!(
            "edge-{index}-{}-{}",
            safe_file_stem(&edge.source),
            safe_file_stem(&edge.target)
        );
        edges.push(ElkEdge {
            id: id.clone(),
            sources: vec![edge.source.clone()],
            targets: vec![edge.target.clone()],
            sections: Vec::new(),
        });
        edge_meta.insert(
            id.clone(),
            SvgEdgeMeta {
                id,
                label: edge.name.clone(),
                rel_type: edge.rel_type.clone(),
            },
        );
    }
    Ok(ElkSource {
        graph: ElkGraph {
            id: "spec42-root".to_string(),
            width: None,
            height: None,
            x: None,
            y: None,
            children,
            edges,
            ports: Vec::new(),
            layout_options: elk_layout_options(payload.view.as_str()),
        },
        nodes,
        edges: edge_meta,
    })
}

fn sanitize_elk_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn build_interconnection_elk_source(
    payload: &SysmlVisualizationResultDto,
) -> Result<ElkSource, String> {
    let scene = payload.interconnection_scene.as_ref().ok_or_else(|| {
        "ELK-backed interconnection export requires interconnectionScene on the visualization payload"
            .to_string()
    })?;
    let graph: ElkGraph = serde_json::from_value(build_elk_graph_from_scene(scene))
        .map_err(|err| format!("Failed to decode interconnection ELK graph: {err}"))?;
    let mut nodes = std::collections::HashMap::new();
    for node in &scene.nodes {
        let elk_id = sanitize_elk_id(&node.id);
        nodes.insert(
            elk_id,
            SvgNodeMeta {
                id: node.id.clone(),
                name: node.name.clone(),
                element_type: node.kind.clone(),
                qualified_name: node.qualified_name.clone(),
                uri: None,
                range: None,
            },
        );
    }
    for container in &scene.containers {
        let elk_id = sanitize_elk_id(&container.id);
        nodes.insert(
            elk_id,
            SvgNodeMeta {
                id: container.id.clone(),
                name: container.label.clone(),
                element_type: "package".to_string(),
                qualified_name: container.label.clone(),
                uri: None,
                range: None,
            },
        );
    }
    let edge_meta = scene
        .edges
        .iter()
        .map(|edge| {
            (
                edge.id.clone(),
                SvgEdgeMeta {
                    id: edge.id.clone(),
                    label: edge.label.clone(),
                    rel_type: edge.kind.clone(),
                },
            )
        })
        .collect();
    Ok(ElkSource {
        graph,
        nodes,
        edges: edge_meta,
    })
}

fn build_activity_elk_source(payload: &SysmlVisualizationResultDto) -> Result<ElkSource, String> {
    let diagrams = payload.activity_diagrams.as_ref().ok_or_else(|| {
        "ELK-backed action-flow export requires activity diagram payload".to_string()
    })?;
    let Some(diagram) = diagrams.first() else {
        return Ok(empty_elk_source("spec42-action-root", "action-flow-view"));
    };
    let mut children = Vec::new();
    let mut nodes = std::collections::HashMap::new();
    for action in &diagram.actions {
        let id = action
            .id
            .clone()
            .unwrap_or_else(|| format!("{}::{}", diagram.id, action.name));
        children.push(ElkNode {
            id: id.clone(),
            width: 210.0,
            height: 76.0,
            x: 0.0,
            y: 0.0,
            children: Vec::new(),
            ports: Vec::new(),
            edges: Vec::new(),
            layout_options: std::collections::BTreeMap::new(),
        });
        nodes.insert(
            id.clone(),
            SvgNodeMeta {
                id: id.clone(),
                name: action.name.clone(),
                element_type: action.action_type.clone(),
                qualified_name: id,
                uri: action.uri.clone(),
                range: None,
            },
        );
    }
    let mut edge_meta = std::collections::HashMap::new();
    let node_ids: std::collections::HashSet<&str> = nodes.keys().map(String::as_str).collect();
    let mut edges = Vec::new();
    for (index, flow) in diagram.flows.iter().enumerate() {
        if !node_ids.contains(flow.from.as_str()) || !node_ids.contains(flow.to.as_str()) {
            continue;
        }
        let id = format!("flow-{index}");
        edges.push(ElkEdge {
            id: id.clone(),
            sources: vec![flow.from.clone()],
            targets: vec![flow.to.clone()],
            sections: Vec::new(),
        });
        edge_meta.insert(
            id.clone(),
            SvgEdgeMeta {
                id,
                label: flow.condition.clone().or_else(|| flow.guard.clone()),
                rel_type: "control-flow".to_string(),
            },
        );
    }
    Ok(ElkSource {
        graph: ElkGraph {
            id: "spec42-action-root".to_string(),
            width: None,
            height: None,
            x: None,
            y: None,
            children,
            edges,
            ports: Vec::new(),
            layout_options: elk_layout_options("action-flow-view"),
        },
        nodes,
        edges: edge_meta,
    })
}

fn build_state_elk_source(payload: &SysmlVisualizationResultDto) -> Result<ElkSource, String> {
    let Some(machine) = payload
        .state_machines
        .as_ref()
        .and_then(|machines| machines.first())
    else {
        return build_graph_elk_source(payload);
    };

    let mut children = Vec::new();
    let mut nodes = std::collections::HashMap::new();
    for state in &machine.states {
        children.push(ElkNode {
            id: state.id.clone(),
            width: 220.0,
            height: if state.kind == "composite" {
                120.0
            } else {
                84.0
            },
            x: 0.0,
            y: 0.0,
            children: Vec::new(),
            ports: Vec::new(),
            edges: Vec::new(),
            layout_options: std::collections::BTreeMap::new(),
        });
        nodes.insert(
            state.id.clone(),
            SvgNodeMeta {
                id: state.id.clone(),
                name: state.name.clone(),
                element_type: state.element.element_type.clone(),
                qualified_name: state.id.clone(),
                uri: state.element.uri.clone(),
                range: None,
            },
        );
    }

    let state_ids: std::collections::HashSet<&str> = nodes.keys().map(String::as_str).collect();
    let mut edges = Vec::new();
    let mut edge_meta = std::collections::HashMap::new();
    for (index, transition) in machine.transitions.iter().enumerate() {
        if !state_ids.contains(transition.source.as_str())
            || !state_ids.contains(transition.target.as_str())
        {
            continue;
        }
        let id = transition.id.clone();
        let edge_id = if id.is_empty() {
            format!("transition-{index}")
        } else {
            id.clone()
        };
        edges.push(ElkEdge {
            id: edge_id.clone(),
            sources: vec![transition.source.clone()],
            targets: vec![transition.target.clone()],
            sections: Vec::new(),
        });
        edge_meta.insert(
            edge_id.clone(),
            SvgEdgeMeta {
                id: edge_id,
                label: transition
                    .label
                    .clone()
                    .or_else(|| transition.guard.clone())
                    .or_else(|| transition.accept.clone()),
                rel_type: "transition".to_string(),
            },
        );
    }

    Ok(ElkSource {
        graph: ElkGraph {
            id: "spec42-state-root".to_string(),
            width: None,
            height: None,
            x: None,
            y: None,
            children,
            edges,
            ports: Vec::new(),
            layout_options: elk_layout_options("state-transition-view"),
        },
        nodes,
        edges: edge_meta,
    })
}

fn empty_elk_source(id: &str, view: &str) -> ElkSource {
    ElkSource {
        graph: ElkGraph {
            id: id.to_string(),
            width: None,
            height: None,
            x: None,
            y: None,
            children: Vec::new(),
            edges: Vec::new(),
            ports: Vec::new(),
            layout_options: elk_layout_options(view),
        },
        nodes: std::collections::HashMap::new(),
        edges: std::collections::HashMap::new(),
    }
}

fn elk_layout_options(view: &str) -> std::collections::BTreeMap<String, String> {
    let mut options = std::collections::BTreeMap::from([
        ("elk.algorithm".to_string(), "layered".to_string()),
        ("elk.edgeRouting".to_string(), "ORTHOGONAL".to_string()),
        (
            "elk.layered.nodePlacement.strategy".to_string(),
            "NETWORK_SIMPLEX".to_string(),
        ),
        (
            "elk.separateConnectedComponents".to_string(),
            "true".to_string(),
        ),
        (
            "org.eclipse.elk.json.edgeCoords".to_string(),
            "ROOT".to_string(),
        ),
    ]);
    match view {
        "interconnection-view" => {
            options.insert(
                "elk.hierarchyHandling".to_string(),
                "INCLUDE_CHILDREN".to_string(),
            );
            options.insert("elk.direction".to_string(), "RIGHT".to_string());
            options.insert("elk.spacing.nodeNode".to_string(), "150".to_string());
            options.insert(
                "elk.layered.spacing.nodeNodeBetweenLayers".to_string(),
                "220".to_string(),
            );
            options.insert("elk.spacing.edgeNode".to_string(), "110".to_string());
            options.insert("elk.spacing.edgeEdge".to_string(), "90".to_string());
            options.insert(
                "elk.layered.crossingMinimization.strategy".to_string(),
                "LAYER_SWEEP".to_string(),
            );
            options.insert(
                "elk.padding".to_string(),
                "[top=70,left=70,bottom=70,right=70]".to_string(),
            );
            options.insert(
                "org.eclipse.elk.portConstraints".to_string(),
                "FIXED_ORDER".to_string(),
            );
            options.insert(
                "org.eclipse.elk.portAlignment.default".to_string(),
                "CENTER".to_string(),
            );
        }
        "action-flow-view" => {
            options.insert("elk.direction".to_string(), "RIGHT".to_string());
            options.insert("elk.spacing.nodeNode".to_string(), "120".to_string());
            options.insert(
                "elk.layered.spacing.nodeNodeBetweenLayers".to_string(),
                "180".to_string(),
            );
            options.insert("elk.spacing.edgeNode".to_string(), "80".to_string());
            options.insert("elk.spacing.edgeEdge".to_string(), "70".to_string());
            options.insert(
                "elk.padding".to_string(),
                "[top=80,left=80,bottom=80,right=80]".to_string(),
            );
        }
        "state-transition-view" => {
            options.insert("elk.direction".to_string(), "RIGHT".to_string());
            options.insert("elk.spacing.nodeNode".to_string(), "130".to_string());
            options.insert(
                "elk.layered.spacing.nodeNodeBetweenLayers".to_string(),
                "190".to_string(),
            );
            options.insert("elk.spacing.edgeNode".to_string(), "90".to_string());
            options.insert("elk.spacing.edgeEdge".to_string(), "80".to_string());
            options.insert(
                "elk.padding".to_string(),
                "[top=100,left=90,bottom=90,right=90]".to_string(),
            );
        }
        _ => {
            options.insert("elk.direction".to_string(), "DOWN".to_string());
            options.insert("elk.spacing.nodeNode".to_string(), "220".to_string());
            options.insert(
                "elk.layered.spacing.nodeNodeBetweenLayers".to_string(),
                "280".to_string(),
            );
            options.insert("elk.spacing.edgeNode".to_string(), "120".to_string());
            options.insert("elk.spacing.edgeEdge".to_string(), "120".to_string());
            options.insert("elk.aspectRatio".to_string(), "1.4".to_string());
            options.insert(
                "elk.padding".to_string(),
                "[top=100,left=100,bottom=100,right=100]".to_string(),
            );
        }
    }
    options
}

fn node_width(node: &GraphNodeDto) -> f64 {
    let name_width = (node.name.chars().count() as f64 * 7.0) + 56.0;
    name_width.clamp(180.0, 320.0)
}

fn node_height(node: &GraphNodeDto) -> f64 {
    if node.attributes.is_empty() {
        64.0
    } else {
        82.0
    }
}

fn render_elk_svg(
    payload: &SysmlVisualizationResultDto,
    export_view: &str,
    layouted: &ElkGraph,
    source: &ElkSource,
) -> Result<String, String> {
    for edge in collect_edges(layouted) {
        if edge.sections.is_empty() {
            return Err(format!(
                "ELK did not return routed sections for edge '{}'; refusing heuristic SVG fallback",
                edge.id
            ));
        }
    }
    let width = layouted.width.unwrap_or(960.0).max(260.0) + 40.0;
    let height = layouted.height.unwrap_or(180.0).max(120.0) + 40.0;
    let mut out = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{:.0}" height="{:.0}" viewBox="0 0 {:.0} {:.0}" data-spec42-view="{}" data-layout-engine="elkjs-quickjs">"#,
        width,
        height,
        width,
        height,
        xml_escape(export_view)
    );
    out.push_str(r#"<style>text{font-family:Arial,sans-serif;font-size:13px}.title{font-size:18px;font-weight:700}.node{fill:#fff;stroke:#24292f;stroke-width:1.4}.port{fill:#fff;stroke:#57606a;stroke-width:1.2}.edge{fill:none;stroke:#0969da;stroke-width:1.6}.edge-label,.muted{fill:#57606a}.type{fill:#57606a;font-size:11px}</style>"#);
    out.push_str(&format!(
        r#"<text class="title" x="20" y="30">{}</text>"#,
        xml_escape(
            payload
                .selected_view_name
                .as_deref()
                .unwrap_or(payload.view.as_str())
        )
    ));
    out.push_str(r#"<g transform="translate(20 44)">"#);
    for edge in collect_edges(layouted) {
        append_elk_edge(&mut out, edge, source)?;
    }
    append_elk_nodes(&mut out, &layouted.children, source, 0.0, 0.0);
    out.push_str("</g></svg>");
    Ok(out)
}

fn collect_edges(graph: &ElkGraph) -> Vec<&ElkEdge> {
    let mut edges: Vec<&ElkEdge> = graph.edges.iter().collect();
    for child in &graph.children {
        collect_node_edges(child, &mut edges);
    }
    edges
}

fn collect_node_edges<'a>(node: &'a ElkNode, edges: &mut Vec<&'a ElkEdge>) {
    edges.extend(node.edges.iter());
    for child in &node.children {
        collect_node_edges(child, edges);
    }
}

fn append_elk_edge(out: &mut String, edge: &ElkEdge, source: &ElkSource) -> Result<(), String> {
    let meta = source.edges.get(&edge.id);
    out.push_str(&format!(
        r#"<g class="diagram-edge" data-edge-id="{}" data-edge-type="{}">"#,
        xml_escape(
            meta.map(|meta| meta.id.as_str())
                .unwrap_or(edge.id.as_str())
        ),
        xml_escape(meta.map(|meta| meta.rel_type.as_str()).unwrap_or(""))
    ));
    for section in &edge.sections {
        let mut path = format!(
            "M {:.1} {:.1}",
            section.start_point.x, section.start_point.y
        );
        for point in &section.bend_points {
            path.push_str(&format!(" L {:.1} {:.1}", point.x, point.y));
        }
        path.push_str(&format!(
            " L {:.1} {:.1}",
            section.end_point.x, section.end_point.y
        ));
        out.push_str(&format!(
            r#"<path class="edge" d="{}"/>"#,
            xml_escape(&path)
        ));
    }
    if let (Some(label), Some(section)) = (
        meta.and_then(|meta| meta.label.as_ref()),
        edge.sections.first(),
    ) {
        let x = (section.start_point.x + section.end_point.x) / 2.0;
        let y = (section.start_point.y + section.end_point.y) / 2.0 - 6.0;
        out.push_str(&format!(
            r#"<text class="edge-label" x="{x:.1}" y="{y:.1}">{}</text>"#,
            xml_escape(label)
        ));
    }
    out.push_str("</g>");
    Ok(())
}

fn append_elk_nodes(
    out: &mut String,
    nodes: &[ElkNode],
    source: &ElkSource,
    offset_x: f64,
    offset_y: f64,
) {
    for node in nodes {
        let x = offset_x + node.x;
        let y = offset_y + node.y;
        let meta = source.nodes.get(&node.id);
        out.push_str(&format!(
            r#"<g class="diagram-node" data-element-id="{}" data-element-name="{}" data-qualified-name="{}"{}{} transform="translate({x:.1} {y:.1})">"#,
            xml_escape(meta.map(|meta| meta.id.as_str()).unwrap_or(node.id.as_str())),
            xml_escape(meta.map(|meta| meta.name.as_str()).unwrap_or(node.id.as_str())),
            xml_escape(meta.map(|meta| meta.qualified_name.as_str()).unwrap_or(node.id.as_str())),
            source_uri_attr(meta.and_then(|meta| meta.uri.as_deref())),
            source_range_attr(meta.and_then(|meta| meta.range.as_ref())),
        ));
        out.push_str(&format!(
            r#"<rect class="node" x="0" y="0" width="{:.1}" height="{:.1}" rx="4"/>"#,
            node.width, node.height
        ));
        out.push_str(&format!(
            r#"<text x="12" y="22">{}</text>"#,
            xml_escape(
                meta.map(|meta| meta.name.as_str())
                    .unwrap_or(node.id.as_str())
            )
        ));
        out.push_str(&format!(
            r#"<text class="type" x="12" y="40">{}</text>"#,
            xml_escape(meta.map(|meta| meta.element_type.as_str()).unwrap_or(""))
        ));
        for port in &node.ports {
            out.push_str(&format!(
                r#"<rect class="port" data-port-id="{}" x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="2"/>"#,
                xml_escape(&port.id),
                port.x,
                port.y,
                port.width,
                port.height
            ));
        }
        if !node.children.is_empty() {
            append_elk_nodes(out, &node.children, source, 0.0, 0.0);
        }
        out.push_str("</g>");
    }
}

fn source_uri_attr(uri: Option<&str>) -> String {
    uri.map(|uri| format!(r#" data-source-uri="{}""#, xml_escape(uri)))
        .unwrap_or_default()
}

fn source_range_attr(range: Option<&RangeDto>) -> String {
    range
        .map(|range| {
            format!(
                r#" data-source-start-line="{}" data-source-start-character="{}" data-source-end-line="{}" data-source-end-character="{}""#,
                range.start.line,
                range.start.character,
                range.end.line,
                range.end.character
            )
        })
        .unwrap_or_default()
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
    use semantic_core::{
        GraphEdgeDto, GraphNodeDto, InterconnectionSceneDto, PositionDto, RangeDto, SysmlGraphDto,
    };
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
            state_machines: None,
            ibd: None,
            interconnection_scene: None,
            stats: None,
            projection_hints: None,
        };
        let svg = native_svg(&payload, "general-view");
        assert!(svg.contains("data-element-id=\"P::x\""));
        assert!(svg.contains("data-spec42-view=\"general-view\""));
        assert!(svg.contains("data-layout-engine=\"native\""));
    }

    #[test]
    fn public_svg_export_uses_shared_renderer_markup() {
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
                nodes: vec![
                    GraphNodeDto {
                        id: "P::Vehicle".to_string(),
                        element_type: "part def".to_string(),
                        name: "Vehicle".to_string(),
                        uri: None,
                        parent_id: None,
                        range: zero_range(),
                        attributes: HashMap::new(),
                    },
                    GraphNodeDto {
                        id: "P::vehicle".to_string(),
                        element_type: "part".to_string(),
                        name: "vehicle".to_string(),
                        uri: None,
                        parent_id: None,
                        range: zero_range(),
                        attributes: HashMap::new(),
                    },
                ],
                edges: vec![GraphEdgeDto {
                    source: "P::vehicle".to_string(),
                    target: "P::Vehicle".to_string(),
                    rel_type: "typing".to_string(),
                    name: Some("typing".to_string()),
                }],
            }),
            general_view_graph: None,
            workspace_model: None,
            activity_diagrams: None,
            sequence_diagrams: None,
            state_machines: None,
            ibd: None,
            interconnection_scene: None,
            stats: None,
            projection_hints: None,
        };
        let (svg, content_type) =
            render_diagram(&payload, DiagramExportFormat::Svg).expect("shared renderer svg");
        assert_eq!(content_type, "image/svg+xml");
        assert!(svg.contains("viz-node--definition"));
        assert!(svg.contains("viz-node--usage"));
        assert!(svg.contains("general-d3-specializes"));
        assert!(!svg.contains("data-layout-engine=\"elkjs-quickjs\""));
    }

    #[test]
    fn requested_renderer_views_rejects_unknown_view() {
        let err = requested_renderer_views("unknown").expect_err("unknown view should fail");
        assert!(err.contains("Unsupported export view"));
    }

    #[test]
    fn interconnection_elk_svg_from_scene_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../shared/diagram-renderer/test-fixtures/interconnection/scene-two-part-chain.json"
        ))
        .expect("parse scene fixture");
        let scene: InterconnectionSceneDto = serde_json::from_value(
            fixture
                .get("interconnectionScene")
                .cloned()
                .expect("interconnectionScene"),
        )
        .expect("deserialize scene");
        let payload = SysmlVisualizationResultDto {
            version: 1,
            view: "interconnection-view".to_string(),
            workspace_root_uri: "file:///demo".to_string(),
            model_ready: true,
            view_candidates: Vec::new(),
            selected_view: None,
            selected_view_name: Some("TwoPartChain".to_string()),
            empty_state_message: None,
            package_groups: None,
            graph: None,
            general_view_graph: None,
            workspace_model: None,
            activity_diagrams: None,
            sequence_diagrams: None,
            state_machines: None,
            ibd: None,
            interconnection_scene: Some(scene),
            stats: None,
            projection_hints: None,
        };
        let svg = elk_svg(&payload, "interconnection-view").expect("interconnection svg");
        assert!(!svg.is_empty());
        assert!(svg.contains("diagram-edge"));
        assert!(svg.contains("data-layout-engine=\"elkjs-quickjs\""));
    }
}
