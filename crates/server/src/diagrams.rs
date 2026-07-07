use std::path::{Path, PathBuf};

use sysml_model::SysmlVisualizationResultDto;
use workspace::HostWorkspaceSnapshot;

use crate::cli::{Cli, DiagramExportArgs, DiagramExportFormat};
use crate::headless_renderer::render_shared_svg;
use crate::host_snapshot::load_snapshot_for_paths;

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
    cli: &Cli,
    path: &Path,
    workspace_root: Option<&Path>,
    view: &str,
    selected_view: Option<&str>,
) -> Result<SysmlVisualizationResultDto, String> {
    let export_view = resolve_renderer_view(view)?;
    let snapshot = load_snapshot_for_paths(cli, path, workspace_root, false)?;
    build_diagram_payload_from_snapshot(&snapshot, export_view, selected_view)
}

pub fn build_diagram_payload_from_snapshot(
    snapshot: &HostWorkspaceSnapshot,
    view: &str,
    selected_view: Option<&str>,
) -> Result<SysmlVisualizationResultDto, String> {
    snapshot
        .prepare_view(view, selected_view)
        .map_err(|error| error.to_string())
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
            let raw = serde_json::to_string(payload).map_err(|err| {
                format!("Failed to serialize diagram payload for SVG export: {err}")
            })?;
            let svg = render_shared_svg(&raw)?;
            Ok((svg, "image/svg+xml"))
        }
    }
}

pub fn render_diagram_for_path(
    cli: &Cli,
    path: &Path,
    workspace_root: Option<&Path>,
    view: &str,
    selected_view: Option<&str>,
    format: DiagramExportFormat,
) -> Result<(String, &'static str), String> {
    let payload = build_diagram_payload(cli, path, workspace_root, view, selected_view)?;
    render_diagram(&payload, format)
}

pub fn export_diagrams(cli: &Cli, args: &DiagramExportArgs) -> Result<DiagramExportSummary, String> {
    std::fs::create_dir_all(&args.output)
        .map_err(|err| format!("Failed to create {}: {err}", args.output.display()))?;
    if args.view == "model-views" {
        return export_model_views(cli, args);
    }
    if args.selected_view.is_some() {
        return export_single_diagram(
            cli,
            args,
            args.view.as_str(),
            args.selected_view.as_deref(),
        );
    }
    let views = requested_renderer_views(args.view.as_str())?;
    let mut exported = 0;
    for view in views {
        export_single_diagram(cli, args, view, None)?;
        exported += 1;
    }
    Ok(DiagramExportSummary {
        output_dir: args.output.clone(),
        exported,
    })
}

fn export_model_views(cli: &Cli, args: &DiagramExportArgs) -> Result<DiagramExportSummary, String> {
    if args.selected_view.is_some() {
        return Err("--selected-view cannot be combined with --view model-views".to_string());
    }
    let snapshot = load_snapshot_for_paths(
        cli,
        args.path.as_path(),
        args.workspace_root.as_deref(),
        false,
    )?;
    let probe = build_diagram_payload_from_snapshot(&snapshot, "general-view", None)?;
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
        let payload = build_diagram_payload_from_snapshot(
            &snapshot,
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
    cli: &Cli,
    args: &DiagramExportArgs,
    view: &str,
    selected_view: Option<&str>,
) -> Result<DiagramExportSummary, String> {
    let payload = build_diagram_payload(
        cli,
        args.path.as_path(),
        args.workspace_root.as_deref(),
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

#[cfg(test)]
#[path = "legacy_elk_svg.rs"]
mod legacy_elk_svg;

#[cfg(test)]
mod tests {
    use super::*;
    use super::legacy_elk_svg::{elk_svg, native_svg};
    use sysml_model::{
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
            activity_diagram_candidates: None,
            sequence_diagrams: None,
            sequence_diagram_candidates: None,
            state_machines: None,
            state_machine_candidates: None,
            ibd: None,
            interconnection_scene: None,
            stats: None,
            projection_hints: None,
            prepared_view: None,
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
            activity_diagram_candidates: None,
            sequence_diagrams: None,
            sequence_diagram_candidates: None,
            state_machines: None,
            state_machine_candidates: None,
            ibd: None,
            interconnection_scene: None,
            stats: None,
            projection_hints: None,
            prepared_view: None,
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
            activity_diagram_candidates: None,
            sequence_diagrams: None,
            sequence_diagram_candidates: None,
            state_machines: None,
            state_machine_candidates: None,
            ibd: None,
            interconnection_scene: Some(scene),
            stats: None,
            projection_hints: None,
            prepared_view: None,
        };
        let svg = elk_svg(&payload, "interconnection-view").expect("interconnection svg");
        assert!(!svg.is_empty());
        assert!(svg.contains("diagram-edge"));
        assert!(svg.contains("data-layout-engine=\"elkjs-quickjs\""));
    }

    fn test_cli() -> Cli {
        Cli {
            config_path: None,
            library_paths: Vec::new(),
            stdlib_path: None,
            domain_libraries_path: None,
            no_stdlib: true,
            stdio: false,
            command: None,
        }
    }

    #[test]
    fn interconnection_export_matches_slim_scoped_lsp_contract_for_drone() {
        let repo_root =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/drone");
        assert!(
            repo_root.is_dir(),
            "expected drone example at {}",
            repo_root.display()
        );
        let payload = build_diagram_payload(
            &test_cli(),
            &repo_root.join("Views.sysml"),
            Some(repo_root.as_path()),
            "interconnection-view",
            Some("connections"),
        )
        .expect("interconnection diagram payload");

        assert_eq!(payload.view, "interconnection-view");
        assert_eq!(payload.selected_view_name.as_deref(), Some("connections"));
        assert!(
            payload.prepared_view.is_some(),
            "CLI interconnection export should include preparedView"
        );
        assert!(
            payload.interconnection_scene.is_none(),
            "CLI interconnection export should omit interconnectionScene when preparedView is present"
        );
        assert!(
            payload.ibd.is_none(),
            "CLI interconnection export should omit ibd for slim payload"
        );
        assert!(
            payload.graph.is_none(),
            "CLI interconnection export should omit graph for slim payload"
        );
        assert!(
            payload.general_view_graph.is_none(),
            "CLI interconnection export should omit generalViewGraph for slim payload"
        );
        assert!(
            !payload.view_candidates.is_empty(),
            "CLI interconnection export should retain viewCandidates"
        );

        let payload_bytes =
            serde_json::to_string(&payload).map(|raw| raw.len()).unwrap_or(0);
        // Budget bumped from 52_000: prepared nodes/ports now carry `uri`/`range` for
        // click-to-source (previously hardcoded to `None` in
        // `prepare_interconnection_prepared_view` — a real bug, not a size-budget trade-off),
        // which legitimately grows the slim payload by a few KB.
        const MAX_DRONE_SLIM_INTERCONNECTION_BYTES: usize = 62_000;
        assert!(
            payload_bytes <= MAX_DRONE_SLIM_INTERCONNECTION_BYTES,
            "CLI slim interconnection payload should stay under {MAX_DRONE_SLIM_INTERCONNECTION_BYTES} bytes, got {payload_bytes}"
        );

        let (svg, content_type) =
            render_diagram(&payload, DiagramExportFormat::Svg).expect("drone interconnection svg");
        assert_eq!(content_type, "image/svg+xml");
        assert!(!svg.is_empty());
        assert!(
            !svg.contains("data-layout-engine=\"elkjs-quickjs\""),
            "runtime interconnection SVG must use shared renderer, not legacy Rust ELK"
        );
    }
}
