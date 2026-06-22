//! Robot-vacuum embedding-host performance harness (report-only).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use semantic_core::{
    build_ibd_for_uri, build_render_snapshot, build_semantic_graph_with_provider,
    build_sysml_visualization_workspace, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, finalize_merged_ibd_connectors, merge_ibd_payloads, project_ids_for_renderer,
    FileSystemDocumentProvider, SemanticGraph, WorkspaceParsedDocument,
};
use serde::Serialize;
use url::Url;
use walkdir::WalkDir;

use crate::{
    EngineBuilder, HostContext, HostFilesystemProvider, HostPipelinePhase, HostWorkspaceSnapshot,
    Spec42Engine, ValidationTiming, WorkspaceLoadRequest,
};

const SELECTED_VIEW: &str = "productStructure";
const RENDERER_VIEW: &str = "general-view";

/// Release-profile median ceilings for the view-first embedding path (deferred validation).
#[derive(Debug, Clone, Copy)]
pub struct PerfRegressionThresholds {
    pub load_workspace_ms: u128,
    pub prepare_view_ms: u128,
    pub total_ms: u128,
}

/// Release-profile ceilings for time-to-completed-validation scenarios.
#[derive(Debug, Clone, Copy)]
pub struct ValidationPerfThresholds {
    pub eager_at_load_ms: u128,
    pub deferred_ensure_ms: u128,
    pub view_then_validation_ms: u128,
}

pub fn release_perf_thresholds() -> PerfRegressionThresholds {
    PerfRegressionThresholds {
        load_workspace_ms: 3_000,
        prepare_view_ms: 2_500,
        total_ms: 5_500,
    }
}

pub fn release_validation_perf_thresholds() -> ValidationPerfThresholds {
    ValidationPerfThresholds {
        eager_at_load_ms: 3_500,
        deferred_ensure_ms: 3_000,
        view_then_validation_ms: 5_500,
    }
}

pub fn assert_release_perf_thresholds(phases: &HostPhaseTimings) {
    if cfg!(debug_assertions) {
        return;
    }
    let thresholds = release_perf_thresholds();
    assert!(
        phases.load_workspace_total_ms <= thresholds.load_workspace_ms,
        "load_workspace {}ms exceeded ceiling {}ms",
        phases.load_workspace_total_ms,
        thresholds.load_workspace_ms
    );
    assert!(
        phases.prepare_view_ms <= thresholds.prepare_view_ms,
        "prepare_view {}ms exceeded ceiling {}ms",
        phases.prepare_view_ms,
        thresholds.prepare_view_ms
    );
    assert!(
        phases.total_ms <= thresholds.total_ms,
        "total {}ms exceeded ceiling {}ms",
        phases.total_ms,
        thresholds.total_ms
    );
}

pub fn assert_release_validation_perf_thresholds(phases: &HostPhaseTimings) {
    if cfg!(debug_assertions) {
        return;
    }
    let thresholds = release_validation_perf_thresholds();
    if phases.time_to_completed_validation_ms == 0 {
        return;
    }
    let ceiling = match phases.validation_mode {
        ValidationPerfMode::EagerAtLoad => thresholds.eager_at_load_ms,
        ValidationPerfMode::DeferredEnsure => thresholds.deferred_ensure_ms,
        ValidationPerfMode::ViewThenValidation => thresholds.view_then_validation_ms,
        ValidationPerfMode::ViewFirst => return,
    };
    assert!(
        phases.time_to_completed_validation_ms <= ceiling,
        "time_to_completed_validation {}ms exceeded ceiling {}ms (mode {:?})",
        phases.time_to_completed_validation_ms,
        ceiling,
        phases.validation_mode
    );
}

/// Performance scenario configuration.
#[derive(Debug, Clone, Serialize)]
pub struct PerfConfig {
    pub label: String,
    pub no_stdlib: bool,
    pub include_prepare_view: bool,
    pub release_build: bool,
    #[serde(default)]
    pub validation_mode: ValidationPerfMode,
}

/// What “time to completed validation” measures in a perf run.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ValidationPerfMode {
    /// View-first: deferred validation at load; no `ensure_validation()` in the run.
    #[default]
    ViewFirst,
    /// Validation collected during `load_workspace` (`ValidationTiming::Eager`).
    EagerAtLoad,
    /// Deferred load, then `ensure_validation()` only.
    DeferredEnsure,
    /// Deferred load, `prepare_view`, then `ensure_validation()`.
    ViewThenValidation,
}

/// Wall-clock duration per host pipeline phase.
#[derive(Debug, Clone, Serialize)]
pub struct HostPhaseTimings {
    pub engine_build_ms: u128,
    pub loading_documents_ms: u128,
    pub building_graph_ms: u128,
    pub building_language_workspace_ms: u128,
    pub building_view_catalog_ms: u128,
    pub collecting_validation_ms: u128,
    pub projecting_model_ms: u128,
    pub load_workspace_total_ms: u128,
    pub prepare_view_ms: u128,
    pub ensure_validation_ms: u128,
    pub time_to_completed_validation_ms: u128,
    pub validation_mode: ValidationPerfMode,
    pub total_ms: u128,
}

impl Default for HostPhaseTimings {
    fn default() -> Self {
        Self {
            engine_build_ms: 0,
            loading_documents_ms: 0,
            building_graph_ms: 0,
            building_language_workspace_ms: 0,
            building_view_catalog_ms: 0,
            collecting_validation_ms: 0,
            projecting_model_ms: 0,
            load_workspace_total_ms: 0,
            prepare_view_ms: 0,
            ensure_validation_ms: 0,
            time_to_completed_validation_ms: 0,
            validation_mode: ValidationPerfMode::default(),
            total_ms: 0,
        }
    }
}

/// In-process semantic/visualization phase breakdown (post-graph or cold).
#[derive(Debug, Clone, Default, Serialize)]
pub struct VisualizationPhaseBreakdown {
    pub semantic_graph_build_ms: u128,
    pub workspace_graph_dto_ms: u128,
    pub ibd_per_uri_ms: u128,
    pub ibd_merge_finalize_ms: u128,
    pub view_catalog_ms: u128,
    pub evaluate_views_ms: u128,
    pub project_all_views_ms: u128,
    pub build_render_snapshot_ms: u128,
    pub prepare_view_product_structure_ms: u128,
    pub cold_headless_visualization_ms: u128,
    pub workspace_file_count: usize,
    pub workspace_uri_count: usize,
    pub evaluated_view_count: usize,
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct FixtureSummary {
    pub files: usize,
    pub total_bytes: u64,
    pub scan_ms: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct RobotVacuumPerfReport {
    pub scenario: PerfConfig,
    pub fixture: FixtureSummary,
    pub host_phases: HostPhaseTimings,
    pub post_snapshot_visualization: VisualizationPhaseBreakdown,
    pub cold_one_shot_visualization_ms: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct RobotVacuumPerfMatrixReport {
    pub fixture_root: String,
    pub model_dir: String,
    pub runs_per_scenario: usize,
    pub scenarios: Vec<ScenarioMedianReport>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScenarioMedianReport {
    pub scenario: PerfConfig,
    pub median_host_phases: HostPhaseTimings,
    pub median_post_snapshot_visualization: VisualizationPhaseBreakdown,
    pub median_cold_one_shot_visualization_ms: u128,
    pub raw_run_totals_ms: Vec<u128>,
}

pub fn spec42_repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join("config").join("robot-vacuum-cleaner.json").is_file())
        .expect("spec42 repository root")
        .to_path_buf()
}

pub fn robot_vacuum_fixture_root() -> PathBuf {
    if let Ok(override_dir) = std::env::var("SYSML_ROBOT_VACUUM_DIR") {
        return PathBuf::from(override_dir);
    }
    spec42_repo_root().join("third_party/sysml-robot-vacuum-cleaner")
}

pub fn require_robot_vacuum_fixture() -> (PathBuf, PathBuf) {
    let root = robot_vacuum_fixture_root();
    let model_dir = root.join("model");
    if !model_dir.is_dir() {
        panic!(
            "robot vacuum fixture missing at {} — run: bash scripts/fetch-robot-vacuum-cleaner.sh",
            root.display()
        );
    }
    (root, model_dir)
}

pub fn perf_output_dir() -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| spec42_repo_root().join("target"))
        .join("spec42-perf")
}

pub fn emit_perf_report<T: Serialize>(report: &T, filename: &str) -> PathBuf {
    let dir = perf_output_dir();
    fs::create_dir_all(&dir).expect("create perf output dir");
    let path = dir.join(filename);
    let json = serde_json::to_string_pretty(report).expect("serialize perf report");
    fs::write(&path, json).expect("write perf report");
    path
}

fn collect_fixture_summary(model_dir: &Path) -> FixtureSummary {
    let scan_start = Instant::now();
    let mut files = 0usize;
    let mut total_bytes = 0u64;
    for entry in WalkDir::new(model_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let ext = entry.path().extension().and_then(|ext| ext.to_str());
        if ext != Some("sysml") && ext != Some("kerml") {
            continue;
        }
        files += 1;
        total_bytes += fs::metadata(entry.path())
            .map(|meta| meta.len())
            .unwrap_or(0);
    }
    FixtureSummary {
        files,
        total_bytes,
        scan_ms: scan_start.elapsed().as_millis(),
    }
}

struct PhaseRecorder {
    last_phase: Option<HostPipelinePhase>,
    last_at: Instant,
    durations: HashMap<String, u128>,
}

impl PhaseRecorder {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_phase: None,
            last_at: now,
            durations: HashMap::new(),
        }
    }

    fn on_phase(&mut self, phase: HostPipelinePhase) {
        let now = Instant::now();
        if let Some(prev) = self.last_phase {
            let key = phase_key(prev);
            *self.durations.entry(key).or_default() += self.last_at.elapsed().as_millis();
        }
        self.last_phase = Some(phase);
        self.last_at = now;
    }

    fn take_durations(&mut self) -> HashMap<String, u128> {
        if let Some(prev) = self.last_phase {
            let key = phase_key(prev);
            *self.durations.entry(key).or_default() += self.last_at.elapsed().as_millis();
        }
        std::mem::take(&mut self.durations)
    }
}

fn phase_key(phase: HostPipelinePhase) -> String {
    match phase {
        HostPipelinePhase::LoadingDocuments => "loadingDocuments".into(),
        HostPipelinePhase::BuildingGraph => "buildingGraph".into(),
        HostPipelinePhase::BuildingLanguageWorkspace => "buildingLanguageWorkspace".into(),
        HostPipelinePhase::BuildingViewCatalog => "buildingViewCatalog".into(),
        HostPipelinePhase::CollectingValidation => "collectingValidation".into(),
        HostPipelinePhase::ProjectingModel => "projectingModel".into(),
    }
}

fn duration_for(durations: &HashMap<String, u128>, key: &str) -> u128 {
    durations.get(key).copied().unwrap_or(0)
}

fn build_engine(cache_dir: &Path, no_stdlib: bool) -> (Spec42Engine, u128) {
    let start = Instant::now();
    let mut builder = EngineBuilder::default().cache_dir(cache_dir);
    if no_stdlib {
        builder = builder.no_stdlib(true);
    }
    let engine = builder.build().expect("engine build");
    (engine, start.elapsed().as_millis())
}

fn load_snapshot_with_phases(
    engine: &Spec42Engine,
    root: &Path,
    model_dir: &Path,
    validation_timing: ValidationTiming,
) -> (Arc<HostWorkspaceSnapshot>, HashMap<String, u128>, u128) {
    let provider = HostFilesystemProvider::from_paths(model_dir, Some(root), &[]);
    let request = WorkspaceLoadRequest::single_target(model_dir.to_path_buf())
        .with_workspace_root(Some(root.to_path_buf()))
        .with_validation_timing(validation_timing);

    let recorder = Arc::new(Mutex::new(PhaseRecorder::new()));
    let progress_recorder = Arc::clone(&recorder);
    let context = HostContext::default().with_progress(Arc::new(move |phase| {
        if let Ok(mut locked) = progress_recorder.lock() {
            locked.on_phase(phase);
        }
    }));

    let load_start = Instant::now();
    let snapshot = engine
        .load_workspace(provider, request, context)
        .expect("load workspace");
    let load_ms = load_start.elapsed().as_millis();

    let mut locked = recorder.lock().expect("phase recorder");
    let durations = locked.take_durations();
    drop(locked);
    (snapshot, durations, load_ms)
}

fn collect_post_snapshot_visualization(
    snapshot: &HostWorkspaceSnapshot,
) -> VisualizationPhaseBreakdown {
    let semantic_graph = snapshot.semantic_graph();
    let parsed_documents = snapshot.parsed_documents();
    let library_urls = snapshot.library_urls();
    let workspace_root_uri = snapshot.workspace_root_uri();

    let workspace_uris = semantic_core::workspace_uris_for_root(
        semantic_graph,
        library_urls,
        workspace_root_uri,
    );

    let graph_start = Instant::now();
    let workspace_graph =
        build_workspace_graph_dto_for_uris(semantic_graph, &workspace_uris);
    let workspace_graph_dto_ms = graph_start.elapsed().as_millis();

    let ibd_start = Instant::now();
    let ibds: Vec<_> = workspace_uris
        .iter()
        .map(|uri| build_ibd_for_uri(semantic_graph, uri))
        .collect();
    let ibd_per_uri_ms = ibd_start.elapsed().as_millis();

    let merge_start = Instant::now();
    let mut full_ibd = merge_ibd_payloads(ibds);
    finalize_merged_ibd_connectors(semantic_graph, &workspace_uris, &mut full_ibd);
    let ibd_merge_finalize_ms = merge_start.elapsed().as_millis();

    let catalog_start = Instant::now();
    let catalog = build_view_catalog(&workspace_uris, parsed_documents);
    let view_catalog_ms = catalog_start.elapsed().as_millis();

    let evaluate_start = Instant::now();
    let evaluated_views = evaluate_views(&catalog, semantic_graph, &workspace_graph);
    let evaluate_views_ms = evaluate_start.elapsed().as_millis();

    let project_start = Instant::now();
    for evaluated in &evaluated_views {
        let _ = project_ids_for_renderer(evaluated, &workspace_graph, RENDERER_VIEW);
    }
    let project_all_views_ms = project_start.elapsed().as_millis();

    let render_start = Instant::now();
    let _render_snapshot = build_render_snapshot(
        semantic_graph,
        parsed_documents,
        library_urls,
        workspace_root_uri,
        1,
    )
    .expect("render snapshot");
    let build_render_snapshot_ms = render_start.elapsed().as_millis();

    let prepare_start = Instant::now();
    let _prepared = snapshot
        .prepare_view(RENDERER_VIEW, Some(SELECTED_VIEW))
        .expect("prepare view");
    let prepare_view_product_structure_ms = prepare_start.elapsed().as_millis();

    VisualizationPhaseBreakdown {
        semantic_graph_build_ms: 0,
        workspace_graph_dto_ms,
        ibd_per_uri_ms,
        ibd_merge_finalize_ms,
        view_catalog_ms,
        evaluate_views_ms,
        project_all_views_ms,
        build_render_snapshot_ms,
        prepare_view_product_structure_ms,
        cold_headless_visualization_ms: 0,
        workspace_file_count: snapshot.documents().len(),
        workspace_uri_count: workspace_uris.len(),
        evaluated_view_count: evaluated_views.len(),
        graph_node_count: workspace_graph.nodes.len(),
        graph_edge_count: workspace_graph.edges.len(),
    }
}

fn collect_cold_one_shot_visualization(root: &Path, model_dir: &Path) -> (u128, VisualizationPhaseBreakdown) {
    let provider = FileSystemDocumentProvider::new(
        model_dir.to_path_buf(),
        Some(root.to_path_buf()),
        Vec::new(),
    );

    let graph_start = Instant::now();
    let (semantic_graph, parsed_docs) =
        build_semantic_graph_with_provider(&provider).expect("semantic graph");
    let semantic_graph_build_ms = graph_start.elapsed().as_millis();

    let workspace_root_uri = Url::from_directory_path(
        root.canonicalize().unwrap_or_else(|_| root.to_path_buf()),
    )
    .expect("workspace root uri");
    let workspace_uris =
        semantic_core::workspace_uris_for_root(&semantic_graph, &[], &workspace_root_uri);

    let viz_start = Instant::now();
    let _ = build_sysml_visualization_workspace(
        &semantic_graph,
        &parsed_docs,
        &[],
        &workspace_root_uri,
        RENDERER_VIEW,
        Some(SELECTED_VIEW),
        viz_start,
    )
    .expect("cold visualization");
    let cold_headless_visualization_ms = viz_start.elapsed().as_millis();

    let mut breakdown = collect_post_snapshot_from_graph(
        &semantic_graph,
        &parsed_docs,
        &[],
        &workspace_root_uri,
        &workspace_uris,
    );
    breakdown.semantic_graph_build_ms = semantic_graph_build_ms;
    breakdown.cold_headless_visualization_ms = cold_headless_visualization_ms;
    (cold_headless_visualization_ms, breakdown)
}

fn collect_post_snapshot_from_graph(
    semantic_graph: &SemanticGraph,
    parsed_documents: &[WorkspaceParsedDocument],
    _library_urls: &[Url],
    _workspace_root_uri: &Url,
    workspace_uris: &[Url],
) -> VisualizationPhaseBreakdown {
    let workspace_graph =
        build_workspace_graph_dto_for_uris(semantic_graph, workspace_uris);
    let catalog = build_view_catalog(workspace_uris, parsed_documents);
    let evaluated_views = evaluate_views(&catalog, semantic_graph, &workspace_graph);
    VisualizationPhaseBreakdown {
        workspace_uri_count: workspace_uris.len(),
        evaluated_view_count: evaluated_views.len(),
        graph_node_count: workspace_graph.nodes.len(),
        graph_edge_count: workspace_graph.edges.len(),
        workspace_file_count: parsed_documents.len(),
        ..Default::default()
    }
}

pub fn run_robot_vacuum_perf(
    config: &PerfConfig,
    cache_dir: &Path,
) -> RobotVacuumPerfReport {
    let (root, model_dir) = require_robot_vacuum_fixture();
    let fixture = collect_fixture_summary(&model_dir);

    let validation_timing = match config.validation_mode {
        ValidationPerfMode::EagerAtLoad => ValidationTiming::Eager,
        ValidationPerfMode::ViewFirst
        | ValidationPerfMode::DeferredEnsure
        | ValidationPerfMode::ViewThenValidation => ValidationTiming::Deferred,
    };
    let include_prepare_view = config.include_prepare_view
        || config.validation_mode == ValidationPerfMode::ViewThenValidation;

    let (engine, engine_build_ms) = build_engine(cache_dir, config.no_stdlib);
    let (snapshot, phase_durations, load_workspace_total_ms) =
        load_snapshot_with_phases(&engine, &root, &model_dir, validation_timing);

    let mut prepare_view_ms = 0;
    if include_prepare_view {
        let prepare_start = Instant::now();
        let _ = snapshot
            .prepare_view(RENDERER_VIEW, Some(SELECTED_VIEW))
            .expect("prepare view");
        prepare_view_ms = prepare_start.elapsed().as_millis();
    }

    let mut ensure_validation_ms = 0;
    if matches!(
        config.validation_mode,
        ValidationPerfMode::DeferredEnsure | ValidationPerfMode::ViewThenValidation
    ) {
        let ensure_start = Instant::now();
        let _ = snapshot.ensure_validation().expect("ensure validation");
        ensure_validation_ms = ensure_start.elapsed().as_millis();
    }

    let time_to_completed_validation_ms = match config.validation_mode {
        ValidationPerfMode::ViewFirst => 0,
        ValidationPerfMode::EagerAtLoad => load_workspace_total_ms,
        ValidationPerfMode::DeferredEnsure => load_workspace_total_ms + ensure_validation_ms,
        ValidationPerfMode::ViewThenValidation => {
            load_workspace_total_ms + prepare_view_ms + ensure_validation_ms
        }
    };

    let post_snapshot_visualization = collect_post_snapshot_visualization(snapshot.as_ref());
    let (cold_one_shot_visualization_ms, _) = collect_cold_one_shot_visualization(&root, &model_dir);

    let measured_total_ms = engine_build_ms
        + load_workspace_total_ms
        + prepare_view_ms
        + ensure_validation_ms;

    let host_phases = HostPhaseTimings {
        engine_build_ms,
        loading_documents_ms: duration_for(&phase_durations, "loadingDocuments"),
        building_graph_ms: duration_for(&phase_durations, "buildingGraph"),
        building_language_workspace_ms: duration_for(&phase_durations, "buildingLanguageWorkspace"),
        building_view_catalog_ms: duration_for(&phase_durations, "buildingViewCatalog"),
        collecting_validation_ms: duration_for(&phase_durations, "collectingValidation"),
        projecting_model_ms: duration_for(&phase_durations, "projectingModel"),
        load_workspace_total_ms,
        prepare_view_ms,
        ensure_validation_ms,
        time_to_completed_validation_ms,
        validation_mode: config.validation_mode,
        total_ms: measured_total_ms,
    };

    RobotVacuumPerfReport {
        scenario: config.clone(),
        fixture,
        host_phases,
        post_snapshot_visualization,
        cold_one_shot_visualization_ms,
    }
}

pub fn median_u128(values: &[u128]) -> u128 {
    if values.is_empty() {
        return 0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    sorted[sorted.len() / 2]
}

fn median_host_phases(reports: &[RobotVacuumPerfReport]) -> HostPhaseTimings {
    HostPhaseTimings {
        engine_build_ms: median_u128(&reports.iter().map(|r| r.host_phases.engine_build_ms).collect::<Vec<_>>()),
        loading_documents_ms: median_u128(&reports.iter().map(|r| r.host_phases.loading_documents_ms).collect::<Vec<_>>()),
        building_graph_ms: median_u128(&reports.iter().map(|r| r.host_phases.building_graph_ms).collect::<Vec<_>>()),
        building_language_workspace_ms: median_u128(&reports.iter().map(|r| r.host_phases.building_language_workspace_ms).collect::<Vec<_>>()),
        building_view_catalog_ms: median_u128(&reports.iter().map(|r| r.host_phases.building_view_catalog_ms).collect::<Vec<_>>()),
        collecting_validation_ms: median_u128(&reports.iter().map(|r| r.host_phases.collecting_validation_ms).collect::<Vec<_>>()),
        projecting_model_ms: median_u128(&reports.iter().map(|r| r.host_phases.projecting_model_ms).collect::<Vec<_>>()),
        load_workspace_total_ms: median_u128(&reports.iter().map(|r| r.host_phases.load_workspace_total_ms).collect::<Vec<_>>()),
        prepare_view_ms: median_u128(&reports.iter().map(|r| r.host_phases.prepare_view_ms).collect::<Vec<_>>()),
        ensure_validation_ms: median_u128(&reports.iter().map(|r| r.host_phases.ensure_validation_ms).collect::<Vec<_>>()),
        time_to_completed_validation_ms: median_u128(
            &reports
                .iter()
                .map(|r| r.host_phases.time_to_completed_validation_ms)
                .collect::<Vec<_>>(),
        ),
        validation_mode: reports
            .first()
            .map(|r| r.host_phases.validation_mode)
            .unwrap_or_default(),
        total_ms: median_u128(&reports.iter().map(|r| r.host_phases.total_ms).collect::<Vec<_>>()),
    }
}

fn median_visualization(reports: &[RobotVacuumPerfReport]) -> VisualizationPhaseBreakdown {
    let pick = |f: fn(&VisualizationPhaseBreakdown) -> u128| {
        median_u128(
            &reports
                .iter()
                .map(|r| f(&r.post_snapshot_visualization))
                .collect::<Vec<_>>(),
        )
    };
    let pick_usize = |f: fn(&VisualizationPhaseBreakdown) -> usize| {
        reports
            .iter()
            .map(|r| f(&r.post_snapshot_visualization))
            .max()
            .unwrap_or(0)
    };
    VisualizationPhaseBreakdown {
        semantic_graph_build_ms: pick(|v| v.semantic_graph_build_ms),
        workspace_graph_dto_ms: pick(|v| v.workspace_graph_dto_ms),
        ibd_per_uri_ms: pick(|v| v.ibd_per_uri_ms),
        ibd_merge_finalize_ms: pick(|v| v.ibd_merge_finalize_ms),
        view_catalog_ms: pick(|v| v.view_catalog_ms),
        evaluate_views_ms: pick(|v| v.evaluate_views_ms),
        project_all_views_ms: pick(|v| v.project_all_views_ms),
        build_render_snapshot_ms: pick(|v| v.build_render_snapshot_ms),
        prepare_view_product_structure_ms: pick(|v| v.prepare_view_product_structure_ms),
        cold_headless_visualization_ms: pick(|v| v.cold_headless_visualization_ms),
        workspace_file_count: pick_usize(|v| v.workspace_file_count),
        workspace_uri_count: pick_usize(|v| v.workspace_uri_count),
        evaluated_view_count: pick_usize(|v| v.evaluated_view_count),
        graph_node_count: pick_usize(|v| v.graph_node_count),
        graph_edge_count: pick_usize(|v| v.graph_edge_count),
    }
}

pub fn run_perf_matrix(
    scenarios: &[PerfConfig],
    runs_per_scenario: usize,
    cache_root: &Path,
) -> RobotVacuumPerfMatrixReport {
    let (root, model_dir) = require_robot_vacuum_fixture();
    let mut scenario_reports = Vec::new();

    for scenario in scenarios {
        let mut runs = Vec::new();
        for run_index in 0..runs_per_scenario {
            let cache_dir = cache_root.join(format!(
                "{}-run-{run_index}",
                scenario.label.replace('/', "_")
            ));
            fs::create_dir_all(&cache_dir).expect("cache dir");
            runs.push(run_robot_vacuum_perf(scenario, &cache_dir));
        }
        scenario_reports.push(ScenarioMedianReport {
            scenario: scenario.clone(),
            median_host_phases: median_host_phases(&runs),
            median_post_snapshot_visualization: median_visualization(&runs),
            median_cold_one_shot_visualization_ms: median_u128(
                &runs
                    .iter()
                    .map(|r| r.cold_one_shot_visualization_ms)
                    .collect::<Vec<_>>(),
            ),
            raw_run_totals_ms: runs.iter().map(|r| r.host_phases.total_ms).collect(),
        });
    }

    RobotVacuumPerfMatrixReport {
        fixture_root: root.display().to_string(),
        model_dir: model_dir.display().to_string(),
        runs_per_scenario,
        scenarios: scenario_reports,
    }
}

pub fn default_matrix_scenarios(release_build: bool) -> Vec<PerfConfig> {
    vec![
        PerfConfig {
            label: "no_stdlib_load_only".into(),
            no_stdlib: true,
            include_prepare_view: false,
            release_build,
            validation_mode: ValidationPerfMode::ViewFirst,
        },
        PerfConfig {
            label: "no_stdlib_load_and_prepare_view".into(),
            no_stdlib: true,
            include_prepare_view: true,
            release_build,
            validation_mode: ValidationPerfMode::ViewFirst,
        },
        PerfConfig {
            label: "embedded_libs_load_and_prepare_view".into(),
            no_stdlib: false,
            include_prepare_view: true,
            release_build,
            validation_mode: ValidationPerfMode::ViewFirst,
        },
        PerfConfig {
            label: "validation_eager_at_load".into(),
            no_stdlib: true,
            include_prepare_view: false,
            release_build,
            validation_mode: ValidationPerfMode::EagerAtLoad,
        },
        PerfConfig {
            label: "validation_deferred_ensure".into(),
            no_stdlib: true,
            include_prepare_view: false,
            release_build,
            validation_mode: ValidationPerfMode::DeferredEnsure,
        },
        PerfConfig {
            label: "view_then_validation".into(),
            no_stdlib: true,
            include_prepare_view: true,
            release_build,
            validation_mode: ValidationPerfMode::ViewThenValidation,
        },
    ]
}

pub fn validation_matrix_scenarios(release_build: bool) -> Vec<PerfConfig> {
    default_matrix_scenarios(release_build)
        .into_iter()
        .filter(|scenario| scenario.validation_mode != ValidationPerfMode::ViewFirst)
        .collect()
}
