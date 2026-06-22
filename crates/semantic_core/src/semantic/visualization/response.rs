//! Visualization response assembly (artifacts, slim payload, DTO output).

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use url::Url;

use crate::semantic::dto::{
    GraphEdgeDto, GraphNodeDto, SysmlGraphDto, SysmlModelStatsDto,
    SysmlVisualizationProjectionHintsDto, SysmlVisualizationResultDto, WorkspaceFileModelDto,
    WorkspaceModelDto, WorkspaceModelSummaryDto,
};
use crate::semantic::explicit_views;
use crate::semantic::extracted_model::{ActivityDiagramDto, SequenceDiagramDto, StateMachineDto};
use crate::semantic::ibd::{self, IbdDataDto, IbdRootViewDto};
use crate::semantic::interconnection_projection::occurrence_id_for_qualified_name;
use crate::semantic::interconnection_scene::build_interconnection_scene;
use crate::semantic::activity_graph::enrich_activity_diagrams_from_graph;
use crate::semantic::model_projection::{self, canonical_general_view_graph};
use crate::semantic::sequence_views::{
    build_workspace_sequence_diagrams, filter_sequence_diagrams_by_exposed_ids,
};
use crate::semantic::state_views::{
    build_workspace_state_machines, filter_state_machines_by_exposed_ids,
};
use crate::semantic::view_projection::{apply_edge_predicate, project_view};
use crate::semantic::visualization::ibd_scope::{
    filter_ibd_by_root_prefixes, filter_ibd_by_visible_ids, ibd_scope_trace_enabled,
    log_ibd_scope_trace, select_interconnection_ibd_scope, select_interconnection_ibd_scope_with_trace,
    IbdScopeTrace,
};
use crate::semantic::visualization::payload::{
    finalize_activity_diagram_candidates_for_response, finalize_activity_diagrams_for_response,
    finalize_sequence_diagram_candidates_for_response, finalize_sequence_diagrams_for_response,
    finalize_state_machine_candidates_for_response, finalize_state_machines_for_response,
    warn_if_behavior_payload_missing,
};
use crate::semantic::visualization::projection::{
    build_ibd_package_container_groups, build_package_groups_from_graph,
    build_workspace_activity_diagrams, build_workspace_graph_dto_for_uris,
    build_workspace_model_dto_from_graph, attach_ibd_package_container_groups,
    collect_package_candidates, diagram_matches_package_filter, filter_activity_diagrams_by_graph,
    graph_to_element_tree, merge_namespace_elements, normalize_package_path,
    no_defined_views_message, project_graph_by_ids, renderer_empty_state_message,
    top_level_package_for_node_id, unsupported_view_type_message,
    workspace_parsed_documents_for_uris,
};
use crate::semantic::visualization::scope::{
    workspace_uris_for_ibd_scope, workspace_uris_for_root, IbdArtifactMode, IbdBuildScope,
};
use crate::semantic::prepared_view::prepare_view_from_visualization;
use crate::semantic::workspace_graph::WorkspaceParsedDocument;
use crate::SemanticGraph;

fn finalize_visualization_response(
    mut response: SysmlVisualizationResultDto,
) -> SysmlVisualizationResultDto {
    if response.model_ready && response.empty_state_message.is_none() {
        response.prepared_view = prepare_view_from_visualization(&response).ok();
    }
    if response.prepared_view.is_some() && response.view == "interconnection-view" {
        response.interconnection_scene = None;
    }
    response
}

fn renderer_uses_activity_diagrams(renderer: &str) -> bool {
    renderer == "action-flow-view"
}

fn renderer_uses_sequence_diagrams(renderer: &str) -> bool {
    renderer == "sequence-view"
}

fn renderer_uses_state_machines(renderer: &str) -> bool {
    renderer == "state-transition-view"
}

/// Options for visualization response assembly (LSP / webview).
#[derive(Debug, Clone, Default)]
pub struct VisualizationBuildOptions {
    /// Omit workspace model and unused diagram families for interconnection responses.
    pub slim_interconnection_payload: bool,
    /// Restrict merged IBD construction to URIs exposed by the selected view.
    pub ibd_build_scope: IbdBuildScope,
}

/// Timing breakdown for visualization perf logging.
#[derive(Debug, Clone, Copy, Default)]
pub struct VisualizationBuildMeta {
    pub cache_hit: bool,
    pub ibd_ms: u32,
    pub view_eval_ms: u32,
    pub scene_ms: u32,
}

/// Expensive workspace inputs shared across model explorer and visualization requests.
#[derive(Debug, Clone)]
pub struct WorkspaceVisualizationArtifacts {
    pub workspace_root_uri: String,
    pub workspace_uris: Vec<Url>,
    pub graph: SysmlGraphDto,
    pub full_ibd: IbdDataDto,
    pub evaluated_views: Vec<explicit_views::EvaluatedView>,
    pub view_candidates: Vec<crate::semantic::dto::SysmlVisualizationViewCandidateDto>,
}

/// Build and merge IBD for all workspace URIs (parallel when multiple files).
pub fn build_merged_workspace_ibd(
    semantic_graph: &SemanticGraph,
    workspace_uris: &[Url],
) -> IbdDataDto {
    if workspace_uris.is_empty() {
        return IbdDataDto {
            parts: Vec::new(),
            ports: Vec::new(),
            connectors: Vec::new(),
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: Vec::new(),
            default_root: None,
            root_views: HashMap::new(),
        };
    }
    let worker_count = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
        .min(workspace_uris.len())
        .max(1);
    let ibds = std::thread::scope(|scope| {
        let mut buckets: Vec<Vec<Url>> = (0..worker_count).map(|_| Vec::new()).collect();
        for (index, uri) in workspace_uris.iter().cloned().enumerate() {
            buckets[index % worker_count].push(uri);
        }
        let mut handles = Vec::with_capacity(worker_count);
        for bucket in buckets {
            handles.push(scope.spawn(move || {
                bucket
                    .iter()
                    .map(|uri| ibd::build_ibd_for_uri(semantic_graph, uri))
                    .collect::<Vec<_>>()
            }));
        }
        handles
            .into_iter()
            .flat_map(|handle| handle.join().expect("ibd worker"))
            .collect::<Vec<_>>()
    });
    let mut full_ibd = ibd::merge_ibd_payloads(ibds);
    ibd::finalize_merged_ibd_connectors(semantic_graph, workspace_uris, &mut full_ibd);
    full_ibd
}

pub fn empty_merged_ibd() -> IbdDataDto {
    IbdDataDto {
        parts: Vec::new(),
        ports: Vec::new(),
        connectors: Vec::new(),
        container_groups: Vec::new(),
        package_container_groups: Vec::new(),
        root_candidates: Vec::new(),
        default_root: None,
        root_views: HashMap::new(),
    }
}

fn ibd_artifact_mode_for_options(view: &str, options: &VisualizationBuildOptions) -> IbdArtifactMode {
    if options.ibd_build_scope == IbdBuildScope::ViewExposedPackages
        && ((options.slim_interconnection_payload && view == "interconnection-view")
            || view == "general-view")
    {
        IbdArtifactMode::Deferred
    } else {
        IbdArtifactMode::FullWorkspace
    }
}

/// Build graph, IBD, evaluated views, and view candidates once per workspace snapshot.
pub fn build_workspace_visualization_artifacts(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    library_paths: &[Url],
    workspace_root_uri: &Url,
    ibd_artifact_mode: IbdArtifactMode,
) -> Result<WorkspaceVisualizationArtifacts, String> {
    let workspace_uris = workspace_uris_for_root(semantic_graph, library_paths, workspace_root_uri);
    let graph = model_projection::strip_synthetic_nodes(&build_workspace_graph_dto_for_uris(
        semantic_graph,
        &workspace_uris,
    ));
    let full_ibd = if ibd_artifact_mode == IbdArtifactMode::Deferred {
        empty_merged_ibd()
    } else {
        build_merged_workspace_ibd(semantic_graph, &workspace_uris)
    };
    let viz_docs = workspace_parsed_documents_for_uris(documents, &workspace_uris);
    let catalog = explicit_views::build_view_catalog(&workspace_uris, &viz_docs);
    let evaluated_views = if catalog.usages.is_empty() {
        Vec::new()
    } else {
        explicit_views::evaluate_views(&catalog, semantic_graph, &graph)
    };
    let view_candidates =
        explicit_views::build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
    Ok(WorkspaceVisualizationArtifacts {
        workspace_root_uri: workspace_root_uri.as_str().to_string(),
        workspace_uris,
        graph,
        full_ibd,
        evaluated_views,
        view_candidates,
    })
}

fn build_activity_diagrams_for_view(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    workspace_uris: &[Url],
    selected_graph: &SysmlGraphDto,
) -> Vec<ActivityDiagramDto> {
    let viz_docs = workspace_parsed_documents_for_uris(documents, workspace_uris);
    let mut diagrams = build_workspace_activity_diagrams(&viz_docs, workspace_uris, None);
    enrich_activity_diagrams_from_graph(&mut diagrams, semantic_graph, workspace_uris);
    finalize_activity_diagrams_for_response(filter_activity_diagrams_by_graph(
        &diagrams,
        selected_graph,
    ))
}

fn build_sequence_diagrams_for_view(
    semantic_graph: &SemanticGraph,
    workspace_uris: &[Url],
    exposed_ids: &HashSet<String>,
) -> Vec<SequenceDiagramDto> {
    let full_sequence_diagrams = build_workspace_sequence_diagrams(semantic_graph, workspace_uris);
    finalize_sequence_diagrams_for_response(filter_sequence_diagrams_by_exposed_ids(
        &full_sequence_diagrams,
        exposed_ids,
    ))
}

fn build_state_machines_for_view(
    semantic_graph: &SemanticGraph,
    graph: &SysmlGraphDto,
    evaluated: &explicit_views::EvaluatedView,
    workspace_uris: &[Url],
) -> Vec<StateMachineDto> {
    let state_ids =
        explicit_views::project_ids_for_renderer(evaluated, graph, "state-transition-view");
    let full_state_machines = build_workspace_state_machines(semantic_graph, workspace_uris);
    finalize_state_machines_for_response(filter_state_machines_by_exposed_ids(
        &full_state_machines,
        &state_ids,
    ))
}

fn select_view_candidate<'a>(
    view_candidates: &'a [crate::semantic::dto::SysmlVisualizationViewCandidateDto],
    view: &str,
    selected_view: Option<&str>,
) -> Option<&'a crate::semantic::dto::SysmlVisualizationViewCandidateDto> {
    selected_view
        .and_then(|selected| {
            view_candidates.iter().find(|candidate| {
                candidate.id == selected
                    || candidate.name == selected
                    || candidate.id.rsplit("::").next() == Some(selected)
            })
        })
        .or_else(|| {
            view_candidates.iter().find(|candidate| {
                candidate.supported && candidate.renderer_view.as_deref() == Some(view)
            })
        })
        .or_else(|| view_candidates.iter().find(|candidate| candidate.supported))
        .or_else(|| view_candidates.first())
}

/// Build a visualization response from precomputed workspace artifacts (lazy single-view projection).
pub fn build_sysml_visualization_from_artifacts(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    artifacts: &WorkspaceVisualizationArtifacts,
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
    options: VisualizationBuildOptions,
) -> Result<(SysmlVisualizationResultDto, VisualizationBuildMeta), String> {
    let mut meta = VisualizationBuildMeta::default();
    let workspace_root_uri = artifacts.workspace_root_uri.as_str();
    let workspace_uris = &artifacts.workspace_uris;
    let graph = &artifacts.graph;
    let full_ibd = &artifacts.full_ibd;
    let evaluated_views = &artifacts.evaluated_views;
    let view_candidates = artifacts.view_candidates.clone();
    let empty_graph = SysmlGraphDto {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    if view_candidates.is_empty() {
        return Ok((
            SysmlVisualizationResultDto {
                version: 0,
                model_ready: true,
                view: view.to_string(),
                workspace_root_uri: workspace_root_uri.to_string(),
                view_candidates: Vec::new(),
                selected_view: None,
                selected_view_name: None,
                empty_state_message: Some(no_defined_views_message()),
                package_groups: Some(Vec::new()),
                graph: Some(empty_graph.clone()),
                general_view_graph: Some(empty_graph.clone()),
                workspace_model: Some(build_workspace_model_dto_from_graph(
                    &empty_graph,
                    workspace_uris,
                )),
                activity_diagrams: Some(Vec::new()),
                activity_diagram_candidates: None,
                sequence_diagrams: Some(Vec::new()),
                sequence_diagram_candidates: None,
                state_machines: Some(Vec::new()),
                state_machine_candidates: None,
                ibd: Some(filter_ibd_by_visible_ids(full_ibd, &HashSet::new())),
                interconnection_scene: None,
                stats: Some(SysmlModelStatsDto {
                    total_elements: 0,
                    resolved_elements: 0,
                    unresolved_elements: 0,
                    parse_time_ms: 0,
                    model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
                    parse_cached: false,
                }),
                projection_hints: None,
                prepared_view: None,
            },
            meta,
        ));
    }

    let view_eval_start = Instant::now();
    let Some(selected_candidate) = select_view_candidate(&view_candidates, view, selected_view)
    else {
        return Ok((
            SysmlVisualizationResultDto {
                version: 0,
                model_ready: true,
                view: view.to_string(),
                workspace_root_uri: workspace_root_uri.to_string(),
                view_candidates,
                selected_view: None,
                selected_view_name: None,
                empty_state_message: Some(renderer_empty_state_message(view)),
                package_groups: Some(Vec::new()),
                graph: Some(empty_graph.clone()),
                general_view_graph: Some(empty_graph.clone()),
                workspace_model: Some(build_workspace_model_dto_from_graph(
                    &empty_graph,
                    workspace_uris,
                )),
                activity_diagrams: Some(Vec::new()),
                activity_diagram_candidates: None,
                sequence_diagrams: Some(Vec::new()),
                sequence_diagram_candidates: None,
                state_machines: Some(Vec::new()),
                state_machine_candidates: None,
                ibd: Some(filter_ibd_by_visible_ids(full_ibd, &HashSet::new())),
                interconnection_scene: None,
                stats: Some(SysmlModelStatsDto {
                    total_elements: 0,
                    resolved_elements: 0,
                    unresolved_elements: 0,
                    parse_time_ms: 0,
                    model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
                    parse_cached: false,
                }),
                projection_hints: None,
                prepared_view: None,
            },
            meta,
        ));
    };
    meta.view_eval_ms = view_eval_start.elapsed().as_millis().max(1) as u32;

    let selected_view_id = selected_candidate.id.clone();
    let selected_view_name = Some(selected_candidate.name.clone());
    let selected_view_type = selected_candidate.view_type.clone();

    if !selected_candidate.supported {
        return Ok((
            SysmlVisualizationResultDto {
                version: 0,
                model_ready: true,
                view: view.to_string(),
                workspace_root_uri: workspace_root_uri.to_string(),
                view_candidates,
                selected_view: Some(selected_view_id),
                selected_view_name,
                empty_state_message: Some(unsupported_view_type_message(
                    selected_view_type.as_deref(),
                )),
                package_groups: Some(Vec::new()),
                graph: Some(empty_graph.clone()),
                general_view_graph: Some(empty_graph.clone()),
                workspace_model: Some(build_workspace_model_dto_from_graph(
                    &empty_graph,
                    workspace_uris,
                )),
                activity_diagrams: Some(Vec::new()),
                activity_diagram_candidates: None,
                sequence_diagrams: Some(Vec::new()),
                sequence_diagram_candidates: None,
                state_machines: Some(Vec::new()),
                state_machine_candidates: None,
                ibd: Some(filter_ibd_by_visible_ids(full_ibd, &HashSet::new())),
                interconnection_scene: None,
                stats: Some(SysmlModelStatsDto {
                    total_elements: 0,
                    resolved_elements: 0,
                    unresolved_elements: 0,
                    parse_time_ms: 0,
                    model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
                    parse_cached: false,
                }),
                projection_hints: None,
                prepared_view: None,
            },
            meta,
        ));
    }

    let resolved_view = selected_candidate
        .renderer_view
        .clone()
        .unwrap_or_else(|| view.to_string());

    let selected_evaluated = evaluated_views
        .iter()
        .find(|evaluated| evaluated.id == selected_view_id);
    let (projected_ids, edge_predicate, projection_hints) =
        if let Some(evaluated) = selected_evaluated {
            let projected = project_view(evaluated, graph);
            let hints = if projected.hints.grid_layout.is_some()
                || projected.hints.grid_subtype.is_some()
                || projected.hints.browser_layout.is_some()
                || !projected.hints.tree_roots.is_empty()
                || projected.hints.geometry_mode.is_some()
                || projected.hints.geometry_projection.is_some()
            {
                Some(SysmlVisualizationProjectionHintsDto {
                    grid_layout: projected.hints.grid_layout,
                    grid_subtype: projected.hints.grid_subtype,
                    browser_layout: projected.hints.browser_layout,
                    tree_roots: projected.hints.tree_roots,
                    geometry_mode: projected.hints.geometry_mode,
                    geometry_projection: projected.hints.geometry_projection,
                })
            } else {
                None
            };
            (projected.node_ids, projected.edge_predicate, hints)
        } else {
            (
                HashSet::new(),
                crate::semantic::view_projection::EdgePredicate::All,
                None,
            )
        };
    let selected_graph = project_graph_by_ids(graph, &projected_ids);
    let general_view_graph = apply_edge_predicate(
        &canonical_general_view_graph(&selected_graph, true),
        edge_predicate,
    );
    let package_groups = Some(build_package_groups_from_graph(&general_view_graph));
    let workspace_model = build_workspace_model_dto_from_graph(&selected_graph, workspace_uris);
    let mut package_candidates = Vec::new();
    let mut seen_packages = HashSet::new();
    collect_package_candidates(
        &workspace_model.semantic,
        &mut seen_packages,
        &mut package_candidates,
    );
    package_candidates.sort_by(|left, right| left.name.cmp(&right.name));
    let selected_ids: HashSet<String> = selected_graph
        .nodes
        .iter()
        .map(|node| node.id.clone())
        .collect();
    let mut interconnection_scope_trace: Option<IbdScopeTrace> = None;
    let ibd_source = if options.ibd_build_scope == IbdBuildScope::ViewExposedPackages {
        if resolved_view == "interconnection-view" {
            if let Some(evaluated) = selected_evaluated {
                let scoped_uris = workspace_uris_for_ibd_scope(
                    workspace_uris,
                    semantic_graph,
                    IbdBuildScope::ViewExposedPackages,
                    &evaluated.exposed_ids,
                );
                build_merged_workspace_ibd(semantic_graph, &scoped_uris)
            } else {
                full_ibd.clone()
            }
        } else if resolved_view == "general-view" {
            let scoped_uris = workspace_uris_for_ibd_scope(
                workspace_uris,
                semantic_graph,
                IbdBuildScope::ViewExposedPackages,
                &selected_ids,
            );
            build_merged_workspace_ibd(semantic_graph, &scoped_uris)
        } else {
            full_ibd.clone()
        }
    } else {
        full_ibd.clone()
    };
    let filtered_ibd = attach_ibd_package_container_groups(
        if resolved_view == "interconnection-view" {
            let (scoped, scope_trace) = select_interconnection_ibd_scope_with_trace(
                &ibd_source,
                &selected_ids,
                selected_evaluated.map(|evaluated| &evaluated.exposed_ids),
            );
            if ibd_scope_trace_enabled() {
                log_ibd_scope_trace(&scope_trace);
            }
            interconnection_scope_trace = Some(scope_trace);
            scoped
        } else {
            filter_ibd_by_visible_ids(&ibd_source, &selected_ids)
        },
        &package_candidates,
        None,
    );
    let scene_start = Instant::now();
    let interconnection_scene = if resolved_view == "interconnection-view" {
        let root_ids = selected_evaluated
            .map(|evaluated| {
                evaluated
                    .exposed_ids
                    .iter()
                    .map(|id| occurrence_id_for_qualified_name(id))
                    .collect::<Vec<_>>()
            })
            .filter(|ids| !ids.is_empty())
            .unwrap_or_else(|| filtered_ibd.root_candidates.clone());
        Some(build_interconnection_scene(
            &filtered_ibd,
            &selected_view_id,
            selected_view_name.as_deref().unwrap_or(""),
            &root_ids,
            interconnection_scope_trace.as_ref(),
        ))
    } else {
        None
    };
    meta.scene_ms = scene_start.elapsed().as_millis().max(1) as u32;

    let activity_diagrams = if renderer_uses_activity_diagrams(resolved_view.as_str()) {
        build_activity_diagrams_for_view(semantic_graph, documents, workspace_uris, &selected_graph)
    } else {
        Vec::new()
    };
    let sequence_diagrams = if renderer_uses_sequence_diagrams(resolved_view.as_str()) {
        selected_evaluated
            .map(|evaluated| {
                build_sequence_diagrams_for_view(
                    semantic_graph,
                    workspace_uris,
                    &evaluated.exposed_ids,
                )
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let state_machines = if renderer_uses_state_machines(resolved_view.as_str()) {
        selected_evaluated
            .map(|evaluated| {
                build_state_machines_for_view(semantic_graph, graph, evaluated, workspace_uris)
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    warn_if_behavior_payload_missing(
        resolved_view.as_str(),
        state_machines.len(),
        activity_diagrams.len(),
    );

    let slim = options.slim_interconnection_payload
        && resolved_view == "interconnection-view"
        && interconnection_scene.is_some();
    let slim_scene_node_count = interconnection_scene
        .as_ref()
        .map(|scene| scene.nodes.len() as u32)
        .unwrap_or(0);
    let activity_diagram_candidates = if resolved_view == "action-flow-view" && !slim {
        Some(finalize_activity_diagram_candidates_for_response(&activity_diagrams))
    } else {
        None
    };
    let state_machine_candidates = if resolved_view == "state-transition-view" && !slim {
        Some(finalize_state_machine_candidates_for_response(&state_machines))
    } else {
        None
    };
    let sequence_diagram_candidates = if resolved_view == "sequence-view" && !slim {
        Some(finalize_sequence_diagram_candidates_for_response(&sequence_diagrams))
    } else {
        None
    };

    Ok((
        finalize_visualization_response(SysmlVisualizationResultDto {
            version: 0,
            model_ready: true,
            view: resolved_view,
            workspace_root_uri: workspace_root_uri.to_string(),
            view_candidates,
            selected_view: Some(selected_view_id),
            selected_view_name,
            empty_state_message: None,
            package_groups: if slim { None } else { package_groups },
            graph: if slim { None } else { Some(selected_graph.clone()) },
            general_view_graph: if slim { None } else { Some(general_view_graph) },
            workspace_model: if slim { None } else { Some(workspace_model) },
            activity_diagrams: if slim || activity_diagrams.is_empty() {
                None
            } else {
                Some(activity_diagrams)
            },
            activity_diagram_candidates,
            sequence_diagrams: if slim || sequence_diagrams.is_empty() {
                None
            } else {
                Some(sequence_diagrams)
            },
            sequence_diagram_candidates,
            state_machines: if slim || state_machines.is_empty() {
                None
            } else {
                Some(state_machines)
            },
            state_machine_candidates,
            ibd: if slim {
                None
            } else {
                Some(filtered_ibd)
            },
            interconnection_scene,
            stats: Some(SysmlModelStatsDto {
                total_elements: if slim {
                    slim_scene_node_count
                } else {
                    selected_graph.nodes.len() as u32
                },
                resolved_elements: 0,
                unresolved_elements: 0,
                parse_time_ms: 0,
                model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
                parse_cached: false,
            }),
            projection_hints,
            prepared_view: None,
        }),
        meta,
    ))
}

/// Full visualization response aligned with the Spec42 LSP kernel.
pub fn build_sysml_visualization_workspace(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    library_paths: &[Url],
    workspace_root_uri: &Url,
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
) -> Result<SysmlVisualizationResultDto, String> {
    let options = interconnection_build_options(view);
    let (response, _meta) = build_sysml_visualization_workspace_with_meta(
        semantic_graph,
        documents,
        WorkspaceVisualizationRequest {
            library_paths,
            workspace_root_uri,
            view,
            selected_view,
            build_start,
            options,
        },
    )?;
    Ok(response)
}

pub fn interconnection_build_options(view: &str) -> VisualizationBuildOptions {
    if view == "interconnection-view" {
        VisualizationBuildOptions {
            slim_interconnection_payload: true,
            ibd_build_scope: IbdBuildScope::ViewExposedPackages,
        }
    } else {
        VisualizationBuildOptions::default()
    }
}

/// Visualization build options for a renderer view, including scoped IBD where supported.
pub fn visualization_build_options(view: &str) -> VisualizationBuildOptions {
    match view {
        "interconnection-view" => interconnection_build_options(view),
        "general-view" => VisualizationBuildOptions {
            slim_interconnection_payload: false,
            ibd_build_scope: IbdBuildScope::ViewExposedPackages,
        },
        _ => VisualizationBuildOptions::default(),
    }
}

pub struct WorkspaceVisualizationRequest<'a> {
    pub library_paths: &'a [Url],
    pub workspace_root_uri: &'a Url,
    pub view: &'a str,
    pub selected_view: Option<&'a str>,
    pub build_start: Instant,
    pub options: VisualizationBuildOptions,
}

/// Full visualization response with perf metadata (tests and LSP).
pub fn build_sysml_visualization_workspace_with_meta(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    request: WorkspaceVisualizationRequest<'_>,
) -> Result<(SysmlVisualizationResultDto, VisualizationBuildMeta), String> {
    let WorkspaceVisualizationRequest {
        library_paths,
        workspace_root_uri,
        view,
        selected_view,
        build_start,
        options,
    } = request;
    let ibd_start = Instant::now();
    let ibd_artifact_mode = ibd_artifact_mode_for_options(view, &options);
    let artifacts = build_workspace_visualization_artifacts(
        semantic_graph,
        documents,
        library_paths,
        workspace_root_uri,
        ibd_artifact_mode,
    )?;
    let mut meta = VisualizationBuildMeta {
        ibd_ms: ibd_start.elapsed().as_millis().max(1) as u32,
        ..VisualizationBuildMeta::default()
    };
    let (response, build_meta) = build_sysml_visualization_from_artifacts(
        semantic_graph,
        documents,
        &artifacts,
        view,
        selected_view,
        build_start,
        options,
    )?;
    meta.view_eval_ms = build_meta.view_eval_ms;
    meta.scene_ms = build_meta.scene_ms;
    Ok((response, meta))
}

pub(crate) fn infer_workspace_root_uri(documents: &[WorkspaceParsedDocument]) -> Result<Url, String> {
    let mut uris: Vec<Url> = documents
        .iter()
        .map(|d| d.uri.clone())
        .filter(|u| {
            let p = u.path().to_ascii_lowercase();
            !p.starts_with("/library/")
        })
        .collect();
    if uris.is_empty() {
        return Url::parse("file:///").map_err(|e| e.to_string());
    }
    uris.sort();
    let first = uris.into_iter().next().expect("non-empty after sort");
    let path = first.path().to_string();
    if let Some(pos) = path.rfind('/') {
        let parent = if pos == 0 { "/" } else { &path[..=pos] };
        let mut base = first;
        base.set_path(parent);
        Ok(base)
    } else {
        Ok(first)
    }
}
