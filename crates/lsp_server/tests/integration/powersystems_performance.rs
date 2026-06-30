//! Report-only performance drill-down for interconnection visualization.
//!
//! Optional grid drill-down (requires `SYSML_POWERSYSTEMS_DIR`):
//! ```text
//! cargo test -p kernel --test lsp_integration integration::powersystems_performance::powersystems_system_context_performance_report -- --ignored --nocapture
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use lsp_server::build_sysml_visualization_for_paths;
use sysml_model::{
    build_ibd_for_uri, build_interconnection_scene, build_merged_workspace_ibd,
    build_semantic_graph_with_provider, build_sysml_visualization_workspace,
    build_sysml_visualization_workspace_with_meta, build_view_catalog,
    build_workspace_graph_dto_for_uris, evaluate_views, finalize_merged_ibd_connectors,
    merge_ibd_payloads, project_ids_for_renderer, select_interconnection_ibd_scope,
    FileSystemDocumentProvider, IbdBuildScope, VisualizationBuildOptions,
    WorkspaceParsedDocument, WorkspaceVisualizationRequest,
};
use tower_lsp::lsp_types::Url;

use super::harness::{next_id, read_message, send_message, spawn_server};
use super::perf_report::{
    collect_fixture_perf, emit_perf_report, graph_edge_count, graph_node_count, latest_perf_event,
    request_with_perf_capture, slowest_phase_entries, value_ms, visualization_model_build_time_ms,
    wait_for_startup_scan, workspace_loaded_files,
};

fn powersystems_repo_root() -> Option<PathBuf> {
    let repo_root = PathBuf::from(std::env::var_os("SYSML_POWERSYSTEMS_DIR")?);
    if repo_root.is_dir() {
        Some(repo_root)
    } else {
        None
    }
}

fn fixture_scan_root(repo_root: &Path) -> PathBuf {
    let nested = repo_root.join("sysml");
    if nested.is_dir() {
        nested
    } else {
        repo_root.to_path_buf()
    }
}

struct VisualizationPhaseBreakdown {
    semantic_graph_build_ms: u128,
    workspace_graph_dto_ms: u128,
    ibd_per_uri_ms: u128,
    ibd_merge_finalize_ms: u128,
    scoped_ibd_per_uri_ms: u128,
    scoped_uri_count: usize,
    view_catalog_ms: u128,
    evaluate_views_ms: u128,
    project_all_views_ms: u128,
    interconnection_scene_ms: u128,
    full_visualization_workspace_ms: u128,
    scoped_visualization_workspace_ms: u128,
    slim_payload_bytes: usize,
    cold_headless_visualization_ms: u128,
    workspace_file_count: usize,
    workspace_uri_count: usize,
    evaluated_view_count: usize,
    selected_view_part_count: usize,
    selected_view_connector_count: usize,
    selected_view_scene_edge_count: usize,
}

fn collect_visualization_phase_breakdown(
    repo_root: &Path,
    selected_view_name: &str,
) -> VisualizationPhaseBreakdown {
    let scan_root = fixture_scan_root(repo_root);
    let provider = FileSystemDocumentProvider::new(
        scan_root.clone(),
        Some(repo_root.to_path_buf()),
        Vec::new(),
    );

    let graph_start = Instant::now();
    let (semantic_graph, parsed_docs) =
        build_semantic_graph_with_provider(&provider).expect("semantic graph");
    let semantic_graph_build_ms = graph_start.elapsed().as_millis();

    let workspace_root_uri = Url::from_directory_path(
        repo_root
            .canonicalize()
            .unwrap_or_else(|_| repo_root.to_path_buf()),
    )
    .expect("workspace root uri");
    let workspace_uris =
        sysml_model::workspace_uris_for_root(&semantic_graph, &[], &workspace_root_uri);

    let graph_start = Instant::now();
    let workspace_graph = build_workspace_graph_dto_for_uris(&semantic_graph, &workspace_uris);
    let workspace_graph_dto_ms = graph_start.elapsed().as_millis();

    let ibd_start = Instant::now();
    let ibds: Vec<_> = workspace_uris
        .iter()
        .map(|uri| build_ibd_for_uri(&semantic_graph, uri))
        .collect();
    let ibd_per_uri_ms = ibd_start.elapsed().as_millis();

    let merge_start = Instant::now();
    let mut full_ibd = merge_ibd_payloads(ibds);
    finalize_merged_ibd_connectors(&semantic_graph, &workspace_uris, &mut full_ibd);
    let ibd_merge_finalize_ms = merge_start.elapsed().as_millis();

    let viz_docs: Vec<WorkspaceParsedDocument> = parsed_docs;

    let catalog_start = Instant::now();
    let catalog = build_view_catalog(&workspace_uris, &viz_docs);
    let view_catalog_ms = catalog_start.elapsed().as_millis();

    let evaluate_start = Instant::now();
    let evaluated_views = evaluate_views(&catalog, &semantic_graph, &workspace_graph);
    let evaluate_views_ms = evaluate_start.elapsed().as_millis();

    let project_start = Instant::now();
    for evaluated in &evaluated_views {
        let _ = project_ids_for_renderer(evaluated, &workspace_graph, "interconnection-view");
        let _ = project_ids_for_renderer(evaluated, &workspace_graph, "general-view");
        let _ = project_ids_for_renderer(evaluated, &workspace_graph, "state-transition-view");
    }
    let project_all_views_ms = project_start.elapsed().as_millis();

    let selected_view = evaluated_views
        .iter()
        .find(|view| view.name == selected_view_name)
        .unwrap_or_else(|| {
            panic!(
                "expected {selected_view_name} view; available: {:?}",
                evaluated_views
                    .iter()
                    .map(|view| view.name.as_str())
                    .collect::<Vec<_>>()
            )
        });
    let projected_ids =
        project_ids_for_renderer(selected_view, &workspace_graph, "interconnection-view");
    let scoped_ibd = select_interconnection_ibd_scope(
        &full_ibd,
        &projected_ids,
        Some(&selected_view.exposed_ids),
    );
    let root_ids = selected_view
        .exposed_ids
        .iter()
        .map(|id| id.replace("::", "."))
        .collect::<Vec<_>>();

    let scoped_uris = sysml_model::workspace_uris_for_ibd_scope(
        &workspace_uris,
        &semantic_graph,
        IbdBuildScope::ViewExposedPackages,
        &selected_view.exposed_ids,
    );
    let scoped_uri_count = scoped_uris.len();
    let scoped_ibd_start = Instant::now();
    let _scoped_merged_ibd = build_merged_workspace_ibd(&semantic_graph, &scoped_uris);
    let scoped_ibd_per_uri_ms = scoped_ibd_start.elapsed().as_millis();

    let scene_start = Instant::now();
    let scene = build_interconnection_scene(
        &scoped_ibd,
        &selected_view.id,
        &selected_view.name,
        &root_ids,
        None,
    );
    let interconnection_scene_ms = scene_start.elapsed().as_millis();

    let full_viz_start = Instant::now();
    let _full = build_sysml_visualization_workspace(
        &semantic_graph,
        &viz_docs,
        &[],
        &workspace_root_uri,
        "interconnection-view",
        Some(selected_view_name),
        full_viz_start,
    )
    .expect("full visualization");
    let full_visualization_workspace_ms = full_viz_start.elapsed().as_millis();

    let scoped_viz_start = Instant::now();
    let (scoped_viz, _) = build_sysml_visualization_workspace_with_meta(
        &semantic_graph,
        &viz_docs,
        WorkspaceVisualizationRequest {
            library_paths: &[],
            workspace_root_uri: &workspace_root_uri,
            view: "interconnection-view",
            selected_view: Some(selected_view_name),
            build_start: scoped_viz_start,
            options: VisualizationBuildOptions {
                slim_interconnection_payload: true,
                ibd_build_scope: IbdBuildScope::ViewExposedPackages,
            },
        },
    )
    .expect("scoped visualization");
    let scoped_visualization_workspace_ms = scoped_viz_start.elapsed().as_millis();
    let slim_payload_bytes =
        serde_json::to_string(&scoped_viz).map(|payload| payload.len()).unwrap_or(0);

    let cold_start = Instant::now();
    let _cold = build_sysml_visualization_for_paths(
        repo_root,
        Some(repo_root),
        &[],
        "interconnection-view",
        Some(selected_view_name),
    )
    .expect("cold headless visualization");
    let cold_headless_visualization_ms = cold_start.elapsed().as_millis();

    VisualizationPhaseBreakdown {
        semantic_graph_build_ms,
        workspace_graph_dto_ms,
        ibd_per_uri_ms,
        ibd_merge_finalize_ms,
        scoped_ibd_per_uri_ms,
        scoped_uri_count,
        view_catalog_ms,
        evaluate_views_ms,
        project_all_views_ms,
        interconnection_scene_ms,
        full_visualization_workspace_ms,
        scoped_visualization_workspace_ms,
        slim_payload_bytes,
        cold_headless_visualization_ms,
        workspace_file_count: viz_docs.len(),
        workspace_uri_count: workspace_uris.len(),
        evaluated_view_count: evaluated_views.len(),
        selected_view_part_count: scoped_ibd.parts.len(),
        selected_view_connector_count: scoped_ibd.connectors.len(),
        selected_view_scene_edge_count: scene.edges.len(),
    }
}

struct InterconnectionPerfReportConfig<'a> {
    fixture_name: &'a str,
    report_file: &'a str,
    repo_root: &'a Path,
    selected_view: &'a str,
    views_uri: Url,
    warm_cache_budget_ms: u128,
    /// Skip the warm cache-hit assertion. Use for small fixtures where the
    /// server may finish a re-index between the two visualization requests.
    skip_warm_cache_assert: bool,
}

fn run_interconnection_lsp_performance_report(config: InterconnectionPerfReportConfig<'_>) {
    let scan_root = fixture_scan_root(config.repo_root);
    let fixture_perf = collect_fixture_perf(&scan_root);
    let phase_breakdown =
        collect_visualization_phase_breakdown(config.repo_root, config.selected_view);

    let root_uri = url::Url::from_directory_path(
        config
            .repo_root
            .canonicalize()
            .unwrap_or_else(|_| config.repo_root.to_path_buf()),
    )
    .expect("workspace root uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "initializationOptions": {
                "performanceLogging": { "enabled": true },
                "workspace": { "maxFilesPerPattern": 1000 }
            },
            "clientInfo": { "name": "interconnection-perf-report", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    let workspace_model_params = serde_json::json!({
        "textDocument": { "uri": config.views_uri.as_str() },
        "scope": ["graph", "stats", "workspaceVisualization"]
    });
    let workspace_model_capture = {
        let wait_start = Instant::now();
        loop {
            let capture = request_with_perf_capture(
                &mut stdin,
                &mut stdout,
                "sysml/model",
                workspace_model_params.clone(),
            );
            let loaded_files = workspace_loaded_files(&capture.json);
            let graph_nodes = graph_node_count(&capture.json);
            if loaded_files > 0 && graph_nodes > 0 {
                break capture;
            }
            if wait_start.elapsed() >= Duration::from_secs(120) {
                panic!(
                    "workspace model did not become ready within 120s; last response: {:#?}",
                    capture.json
                );
            }
            std::thread::sleep(Duration::from_millis(250));
        }
    };

    let mut perf_events = workspace_model_capture.perf_events.clone();
    perf_events.extend(wait_for_startup_scan(
        &mut stdin,
        &mut stdout,
        &perf_events,
        Duration::from_secs(120),
    ));
    let visualization_capture = request_with_perf_capture(
        &mut stdin,
        &mut stdout,
        "sysml/visualization",
        serde_json::json!({
            "workspaceRootUri": root_uri.as_str(),
            "view": "interconnection-view",
            "selectedView": config.selected_view
        }),
    );
    perf_events.extend(visualization_capture.perf_events.clone());

    let warm_visualization_capture = request_with_perf_capture(
        &mut stdin,
        &mut stdout,
        "sysml/visualization",
        serde_json::json!({
            "workspaceRootUri": root_uri.as_str(),
            "view": "interconnection-view",
            "selectedView": config.selected_view
        }),
    );
    perf_events.extend(warm_visualization_capture.perf_events.clone());
    let warm_visualization_event = latest_perf_event(
        &warm_visualization_capture.perf_events,
        "backend:sysmlVisualizationRequest",
    );
    if !config.skip_warm_cache_assert {
        assert!(
            warm_visualization_event
                .and_then(|event| event.get("cacheHit"))
                .and_then(|value| value.as_bool())
                .unwrap_or(false),
            "expected warm sysml/visualization cache hit, got {warm_visualization_event:#?}"
        );
    }
    assert!(
        warm_visualization_capture.elapsed_ms < config.warm_cache_budget_ms,
        "expected warm visualization request under {}ms, got {}ms",
        config.warm_cache_budget_ms,
        warm_visualization_capture.elapsed_ms
    );

    let startup_event = latest_perf_event(&perf_events, "backend:startupScanPhases");
    let workspace_response_event = latest_perf_event(
        &workspace_model_capture.perf_events,
        "backend:buildSysmlModelResponse",
    );
    let visualization_event = latest_perf_event(
        &visualization_capture.perf_events,
        "backend:sysmlVisualizationRequest",
    );

    let visualization_result = &visualization_capture.json["result"];
    let has_ibd = visualization_result.get("ibd").is_some_and(|value| !value.is_null());
    let ibd_parts = visualization_result["ibd"]["parts"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0);
    let ibd_connectors = visualization_result["ibd"]["connectors"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0);
    let scene_edges = visualization_result["preparedView"]["edges"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0);
    let has_prepared_view = visualization_result
        .get("preparedView")
        .is_some_and(|value| !value.is_null());
    let view_candidates = visualization_result["viewCandidates"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0);

    let phases = HashMap::from([
        ("fixtureScan", fixture_perf.scan_ms),
        ("fixtureParseTotal", fixture_perf.parse.total_ms),
        (
            "phaseSemanticGraphBuild",
            phase_breakdown.semantic_graph_build_ms,
        ),
        (
            "phaseWorkspaceGraphDto",
            phase_breakdown.workspace_graph_dto_ms,
        ),
        ("phaseIbdPerUri", phase_breakdown.ibd_per_uri_ms),
        (
            "phaseIbdMergeFinalize",
            phase_breakdown.ibd_merge_finalize_ms,
        ),
        (
            "phaseScopedIbdPerUri",
            phase_breakdown.scoped_ibd_per_uri_ms,
        ),
        ("phaseViewCatalog", phase_breakdown.view_catalog_ms),
        ("phaseEvaluateViews", phase_breakdown.evaluate_views_ms),
        ("phaseProjectAllViews", phase_breakdown.project_all_views_ms),
        (
            "phaseInterconnectionScene",
            phase_breakdown.interconnection_scene_ms,
        ),
        (
            "phaseFullVisualizationWorkspace",
            phase_breakdown.full_visualization_workspace_ms,
        ),
        (
            "phaseScopedVisualizationWorkspace",
            phase_breakdown.scoped_visualization_workspace_ms,
        ),
        (
            "phaseColdHeadlessVisualization",
            phase_breakdown.cold_headless_visualization_ms,
        ),
        (
            "startupParseWorkers",
            value_ms(startup_event, "parseWorkersMs"),
        ),
        ("relinkTotal", value_ms(startup_event, "relinkTotalMs")),
        ("diagnostics", value_ms(startup_event, "diagnosticsMs")),
        ("workspaceModelRequest", workspace_model_capture.elapsed_ms),
        (
            "workspaceModelIbd",
            value_ms(workspace_response_event, "ibdMs"),
        ),
        (
            "workspaceModelBuild",
            value_ms(workspace_response_event, "totalMs"),
        ),
        ("visualizationRequest", visualization_capture.elapsed_ms),
        (
            "visualizationModelBuild",
            visualization_model_build_time_ms(&visualization_capture.json),
        ),
    ]);

    let report = serde_json::json!({
        "schemaVersion": 3,
        "fixture": {
            "name": config.fixture_name,
            "path": config.repo_root.to_string_lossy(),
            "scanRoot": scan_root.to_string_lossy(),
            "files": fixture_perf.files,
            "totalBytes": fixture_perf.total_bytes,
            "localScanParse": fixture_perf,
        },
        "context": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "ci": std::env::var_os("CI").is_some(),
            "profile": "debug-test"
        },
        "phaseBreakdown": {
            "semanticGraphBuildMs": phase_breakdown.semantic_graph_build_ms,
            "workspaceGraphDtoMs": phase_breakdown.workspace_graph_dto_ms,
            "ibdPerUriMs": phase_breakdown.ibd_per_uri_ms,
            "ibdMergeFinalizeMs": phase_breakdown.ibd_merge_finalize_ms,
            "scopedIbdPerUriMs": phase_breakdown.scoped_ibd_per_uri_ms,
            "scopedUriCount": phase_breakdown.scoped_uri_count,
            "workspaceUriCount": phase_breakdown.workspace_uri_count,
            "viewCatalogMs": phase_breakdown.view_catalog_ms,
            "evaluateViewsMs": phase_breakdown.evaluate_views_ms,
            "projectAllViewsMs": phase_breakdown.project_all_views_ms,
            "interconnectionSceneMs": phase_breakdown.interconnection_scene_ms,
            "fullVisualizationWorkspaceMs": phase_breakdown.full_visualization_workspace_ms,
            "scopedVisualizationWorkspaceMs": phase_breakdown.scoped_visualization_workspace_ms,
            "slimPayloadBytes": phase_breakdown.slim_payload_bytes,
            "coldHeadlessVisualizationMs": phase_breakdown.cold_headless_visualization_ms,
            "workspaceFileCount": phase_breakdown.workspace_file_count,
            "evaluatedViewCount": phase_breakdown.evaluated_view_count,
            "selectedViewParts": phase_breakdown.selected_view_part_count,
            "selectedViewConnectors": phase_breakdown.selected_view_connector_count,
            "selectedViewSceneEdges": phase_breakdown.selected_view_scene_edge_count,
        },
        "phases": {
            "startup": startup_event.cloned().unwrap_or_else(|| serde_json::json!({})),
            "workspaceModelResponse": workspace_response_event.cloned().unwrap_or_else(|| serde_json::json!({})),
            "visualizationResponse": visualization_event.cloned().unwrap_or_else(|| serde_json::json!({}))
        },
        "modelRequests": {
            "workspace": {
                "elapsedMs": workspace_model_capture.elapsed_ms,
                "responseBytes": workspace_model_capture.raw.len(),
                "loadedFiles": workspace_loaded_files(&workspace_model_capture.json),
                "graphNodes": graph_node_count(&workspace_model_capture.json),
                "graphEdges": graph_edge_count(&workspace_model_capture.json),
                "ibdMs": value_ms(workspace_response_event, "ibdMs"),
            }
        },
        "visualization": {
            "view": "interconnection-view",
            "selectedView": config.selected_view,
            "elapsedMs": visualization_capture.elapsed_ms,
            "responseBytes": visualization_capture.raw.len(),
            "modelBuildTimeMs": visualization_model_build_time_ms(&visualization_capture.json),
            "viewCandidates": view_candidates,
            "hasIbd": has_ibd,
            "hasPreparedView": has_prepared_view,
            "ibdParts": ibd_parts,
            "ibdConnectors": ibd_connectors,
            "sceneEdges": scene_edges,
            "event": visualization_event.cloned().unwrap_or_else(|| serde_json::json!({}))
        },
        "visualizationWarm": {
            "elapsedMs": warm_visualization_capture.elapsed_ms,
            "responseBytes": warm_visualization_capture.raw.len(),
            "modelBuildTimeMs": visualization_model_build_time_ms(&warm_visualization_capture.json),
            "event": warm_visualization_event.cloned().unwrap_or_else(|| serde_json::json!({}))
        },
        "counts": {
            "indexedDocuments": workspace_loaded_files(&workspace_model_capture.json),
            "workspaceGraphNodes": graph_node_count(&workspace_model_capture.json),
            "workspaceGraphEdges": graph_edge_count(&workspace_model_capture.json),
            "visualizationGraphNodes": visualization_result["graph"]["nodes"].as_array().map(Vec::len).unwrap_or(0),
            "visualizationGraphEdges": visualization_result["graph"]["edges"].as_array().map(Vec::len).unwrap_or(0),
            "viewCandidates": view_candidates,
            "perfEvents": perf_events.len()
        },
        "budgets": {
            "mode": "report-only",
            "workspaceModelRequestMs": 5000,
            "visualizationRequestMs": 1500,
            "visualizationModelBuildMs": 1500,
            "warmVisualizationCacheHitMs": config.warm_cache_budget_ms
        },
        "bottlenecks": {
            "slowestPhases": slowest_phase_entries(&phases),
            "slowestFilesByParse": fixture_perf.slowest_files_by_parse,
            "largestFiles": fixture_perf.largest_files
        },
        "events": perf_events
    });

    emit_perf_report(&report, config.report_file);

    assert!(
        workspace_loaded_files(&workspace_model_capture.json) > 0,
        "expected indexed workspace documents"
    );
    assert_eq!(
        visualization_result["selectedViewName"].as_str(),
        Some(config.selected_view)
    );
    assert!(
        has_prepared_view,
        "expected preparedView in slim LSP payload"
    );
    assert!(
        !has_ibd,
        "expected slim LSP payload to omit ibd when preparedView is present"
    );
    let has_graph = visualization_result
        .get("graph")
        .is_some_and(|value| !value.is_null());
    let has_general_view_graph = visualization_result
        .get("generalViewGraph")
        .is_some_and(|value| !value.is_null());
    assert!(
        !has_graph,
        "expected slim LSP payload to omit graph when preparedView is present"
    );
    assert!(
        !has_general_view_graph,
        "expected slim LSP payload to omit generalViewGraph when preparedView is present"
    );
    const MAX_DRONE_SLIM_INTERCONNECTION_BYTES: usize = 52_000;
    assert!(
        visualization_capture.raw.len() <= MAX_DRONE_SLIM_INTERCONNECTION_BYTES,
        "expected slim LSP payload under {MAX_DRONE_SLIM_INTERCONNECTION_BYTES} bytes on drone, got {}",
        visualization_capture.raw.len()
    );
    assert!(
        phase_breakdown.slim_payload_bytes <= MAX_DRONE_SLIM_INTERCONNECTION_BYTES,
        "expected scoped workspace slim payload under {MAX_DRONE_SLIM_INTERCONNECTION_BYTES} bytes, got {}",
        phase_breakdown.slim_payload_bytes
    );
    assert!(scene_edges > 0, "expected non-empty prepared interconnection view");
    assert!(
        phase_breakdown.scoped_uri_count <= phase_breakdown.workspace_uri_count,
        "scoped URI count should not exceed workspace URI count"
    );

    let _ = child.kill();
}

#[test]
fn drone_interconnection_performance_smoke_report() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/drone");
    assert!(
        repo_root.is_dir(),
        "expected drone example at {}; ensure the examples submodule is checked out",
        repo_root.display()
    );

    let views_uri =
        url::Url::from_file_path(repo_root.join("Views.sysml")).expect("drone Views.sysml uri");

    run_interconnection_lsp_performance_report(InterconnectionPerfReportConfig {
        fixture_name: "drone-interconnection",
        report_file: "drone-interconnection-performance.json",
        repo_root: &repo_root,
        selected_view: "connections",
        views_uri,
        warm_cache_budget_ms: 500,
        skip_warm_cache_assert: true,
    });
}

#[test]
#[ignore = "report-only drill-down; set SYSML_POWERSYSTEMS_DIR to an external grid fixture checkout"]
fn powersystems_system_context_performance_report() {
    let Some(repo_root) = powersystems_repo_root() else {
        eprintln!(
            "Skipping powersystems_system_context_performance_report: SYSML_POWERSYSTEMS_DIR is unset or not a directory"
        );
        return;
    };

    let views_uri = url::Url::from_file_path(
        repo_root
            .join("sysml")
            .join("projects")
            .join("regional-grid-expansion")
            .join("Views.sysml"),
    )
    .expect("Views.sysml uri");

    run_interconnection_lsp_performance_report(InterconnectionPerfReportConfig {
        fixture_name: "grid-system-context",
        report_file: "grid-system-context-performance.json",
        repo_root: &repo_root,
        selected_view: "systemContext",
        views_uri,
        warm_cache_budget_ms: 500,
        skip_warm_cache_assert: false,
    });
}
