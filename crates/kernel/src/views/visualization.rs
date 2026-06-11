use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use semantic_core::{
    build_semantic_graph_with_provider, build_sysml_visualization_workspace,
    FileSystemDocumentProvider, SysmlGraphDto, SysmlModelStatsDto, SysmlVisualizationResultDto,
    WorkspaceParsedDocument,
};
use tower_lsp::lsp_types::Url;

use crate::semantic;
use crate::workspace::state::{IndexEntry, ParseMetadata};

mod activity_views;

pub(crate) use activity_views::parse_sysml_visualization_params;

pub fn build_sysml_visualization_for_paths(
    target: &Path,
    workspace_root: Option<&Path>,
    library_paths: &[PathBuf],
    view: &str,
    selected_view: Option<&str>,
) -> Result<SysmlVisualizationResultDto, String> {
    let build_start = Instant::now();
    let workspace_root = workspace_root.map(Path::to_path_buf).unwrap_or_else(|| {
        if target.is_dir() {
            target.to_path_buf()
        } else {
            target
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("."))
        }
    });
    let workspace_root_uri = path_to_url(&workspace_root)?;
    let library_root_urls = library_paths
        .iter()
        .map(|path| path_to_url(path))
        .collect::<Result<Vec<_>, _>>()?;
    let provider = FileSystemDocumentProvider::new(
        target.to_path_buf(),
        Some(workspace_root.clone()),
        library_paths.to_vec(),
    );
    let (semantic_graph, parsed_docs) = build_semantic_graph_with_provider(&provider)?;
    let index = parsed_docs
        .into_iter()
        .map(|doc| {
            (
                doc.uri,
                IndexEntry {
                    content: doc.content,
                    parsed: Some(doc.parsed),
                    parse_metadata: ParseMetadata {
                        parse_time_ms: doc.parse_time_ms,
                        parse_cached: doc.parse_cached,
                    },
                    include_in_semantic_graph: true,
                },
            )
        })
        .collect::<HashMap<Url, IndexEntry>>();
    Ok(build_sysml_visualization_response(
        &semantic_graph,
        &index,
        &workspace_root_uri,
        &library_root_urls,
        view,
        selected_view,
        build_start,
    ))
}

fn path_to_url(path: &Path) -> Result<Url, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|err| format!("Failed to resolve current directory: {err}"))?
            .join(path)
    };
    let canonical = std::fs::canonicalize(&absolute).unwrap_or(absolute);
    if canonical.is_dir() {
        Url::from_directory_path(&canonical)
    } else {
        Url::from_file_path(&canonical)
    }
    .map_err(|_| {
        format!(
            "Failed to convert path to file URI: {}",
            canonical.display()
        )
    })
}

fn workspace_parsed_documents_for_visualization(
    index: &HashMap<Url, IndexEntry>,
    workspace_uris: &[Url],
) -> Vec<WorkspaceParsedDocument> {
    workspace_uris
        .iter()
        .filter_map(|uri| {
            let entry = index.get(uri)?;
            Some(WorkspaceParsedDocument {
                uri: uri.clone(),
                content: entry.content.clone(),
                parsed: entry.parsed.as_ref()?.clone(),
                parse_time_ms: entry.parse_metadata.parse_time_ms,
                parse_cached: entry.parse_metadata.parse_cached,
            })
        })
        .collect()
}

pub(crate) fn build_sysml_visualization_response(
    semantic_graph: &semantic::SemanticGraph,
    index: &HashMap<Url, IndexEntry>,
    workspace_root_uri: &Url,
    library_paths: &[Url],
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
) -> SysmlVisualizationResultDto {
    let workspace_uris =
        semantic_core::workspace_uris_for_root(semantic_graph, library_paths, workspace_root_uri);
    let viz_docs = workspace_parsed_documents_for_visualization(index, &workspace_uris);
    let empty_graph = SysmlGraphDto {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    build_sysml_visualization_workspace(
        semantic_graph,
        &viz_docs,
        library_paths,
        workspace_root_uri,
        view,
        selected_view,
        build_start,
    )
    .unwrap_or_else(|message| SysmlVisualizationResultDto {
        version: 0,
        model_ready: true,
        view: view.to_string(),
        workspace_root_uri: workspace_root_uri.as_str().to_string(),
        view_candidates: Vec::new(),
        selected_view: None,
        selected_view_name: None,
        empty_state_message: Some(message),
        package_groups: Some(Vec::new()),
        graph: Some(empty_graph.clone()),
        general_view_graph: Some(empty_graph),
        workspace_model: None,
        activity_diagrams: None,
        sequence_diagrams: None,
        state_machines: None,
        ibd: None,
        stats: Some(SysmlModelStatsDto {
            total_elements: 0,
            resolved_elements: 0,
            unresolved_elements: 0,
            parse_time_ms: 0,
            model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
            parse_cached: false,
        }),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;
    use std::time::Instant;

    use super::{build_sysml_visualization_for_paths, parse_sysml_visualization_params};
    use semantic_core::semantic::ibd::{IbdDataDto, IbdPartDto, IbdRootViewDto};
    use semantic_core::{
        attach_ibd_package_container_groups, build_ibd_package_container_groups,
        build_package_groups_from_graph, build_workspace_activity_diagrams,
        select_interconnection_ibd_scope, GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto,
        SysmlGraphDto, SysmlVisualizationPackageCandidateDto, WorkspaceParsedDocument,
    };
    use sysml_v2_parser::parse;
    use tower_lsp::lsp_types::Url;

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
    fn interconnection_scope_falls_back_to_visible_ids_when_root_prefix_misses() {
        let full_ibd = IbdDataDto {
            parts: vec![IbdPartDto {
                id: "WebShopArchitecture::WebShopSystem::checkoutService".to_string(),
                name: "checkoutService".to_string(),
                qualified_name: "WebShopArchitecture.WebShopSystem.checkoutService".to_string(),
                uri: None,
                container_id: Some("WebShopArchitecture.WebShopSystem".to_string()),
                element_type: "part".to_string(),
                attributes: HashMap::new(),
            }],
            ports: vec![
                semantic_core::semantic::ibd::IbdPortDto {
                    id: "WebShopArchitecture::WebShopSystem::checkoutService::apiIn".to_string(),
                    name: "apiIn".to_string(),
                    parent_id: "WebShopArchitecture.WebShopSystem.checkoutService".to_string(),
                    direction: None,
                    port_type: None,
                    port_side: None,
                },
                semantic_core::semantic::ibd::IbdPortDto {
                    id: "WebShopArchitecture::WebShopSystem::apiGateway::checkoutApiOut"
                        .to_string(),
                    name: "checkoutApiOut".to_string(),
                    parent_id: "WebShopArchitecture.WebShopSystem.apiGateway".to_string(),
                    direction: None,
                    port_type: None,
                    port_side: None,
                },
            ],
            connectors: vec![semantic_core::semantic::ibd::IbdConnectorDto {
                source: "WebShopArchitecture::WebShopSystem::checkoutService::apiIn".to_string(),
                target: "WebShopArchitecture::WebShopSystem::apiGateway::checkoutApiOut"
                    .to_string(),
                source_id: "WebShopArchitecture.WebShopSystem.checkoutService.apiIn".to_string(),
                target_id: "WebShopArchitecture.WebShopSystem.apiGateway.checkoutApiOut"
                    .to_string(),
                source_part_id: None,
                target_part_id: None,
                rel_type: "connection".to_string(),
            }],
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: Vec::new(),
            default_root: None,
            root_views: HashMap::new(),
        };
        let selected_ids: HashSet<String> = HashSet::from([
            "WebShopArchitecture::WebShopSystem::checkoutService".to_string(),
            "WebShopArchitecture::WebShopSystem::checkoutService::apiIn".to_string(),
            "WebShopArchitecture::WebShopSystem::apiGateway::checkoutApiOut".to_string(),
        ]);
        let selected_exposed_ids: HashSet<String> =
            HashSet::from(["WebShopExample::webshopSystem".to_string()]);

        let scoped =
            select_interconnection_ibd_scope(&full_ibd, &selected_ids, Some(&selected_exposed_ids));

        assert!(
            !scoped.parts.is_empty(),
            "expected visible-id fallback to keep part payload when root-prefix scoping misses"
        );
        assert!(
            !scoped.connectors.is_empty(),
            "expected visible-id fallback to keep connectors when root-prefix scoping misses"
        );
    }

    #[test]
    fn parse_visualization_params_accepts_workspace_root_and_selected_view() {
        let params = serde_json::json!({
            "workspaceRootUri": "file:///C:/demo",
            "view": "general-view",
            "selectedView": "Demo::Pkg::VehicleView"
        });

        let (workspace_root_uri, view, selected_view) =
            parse_sysml_visualization_params(&params).expect("parse visualization params");
        assert_eq!(workspace_root_uri.as_str(), "file:///c:/demo");
        assert_eq!(view, "general-view");
        assert_eq!(selected_view.as_deref(), Some("Demo::Pkg::VehicleView"));
    }

    #[test]
    fn parse_visualization_params_accepts_array_shape() {
        let params = serde_json::json!([
            {
                "workspaceRootUri": "file:///C:/demo",
                "view": "interconnection-view",
                "selectedView": "Demo::VehicleConnections"
            }
        ]);

        let (workspace_root_uri, view, selected_view) =
            parse_sysml_visualization_params(&params).expect("parse visualization params");
        assert_eq!(workspace_root_uri.as_str(), "file:///c:/demo");
        assert_eq!(view, "interconnection-view");
        assert_eq!(selected_view.as_deref(), Some("Demo::VehicleConnections"));
    }

    #[test]
    fn workspace_activity_diagrams_include_performer_contexts_and_support_package_filtering() {
        let uri_a = Url::parse("file:///C:/demo/Logical.sysml").expect("uri a");
        let uri_b = Url::parse("file:///C:/demo/Function.sysml").expect("uri b");
        let parsed_a = parse(
            r#"
                package LogicalComponentsPackage {
                    part def LaunchSystem {
                        perform action provideStage1Thrust : ProvideStage1Thrust;
                        perform action provideStage2Thrust : ProvideStage2Thrust;
                    }
                }
            "#,
        )
        .expect("parse logical");
        let parsed_b = parse(
            r#"
                package FunctionsPackage {
                    action def LaunchToOrbit {
                        action countdown: ExecuteTerminalCountdown;
                        action provideThrust1: ProvideStage1Thrust;
                    }
                }
            "#,
        )
        .expect("parse function");

        let docs = vec![
            WorkspaceParsedDocument {
                uri: uri_a.clone(),
                content: String::new(),
                parsed: parsed_a,
                parse_time_ms: 0,
                parse_cached: false,
            },
            WorkspaceParsedDocument {
                uri: uri_b.clone(),
                content: String::new(),
                parsed: parsed_b,
                parse_time_ms: 0,
                parse_cached: false,
            },
        ];

        let all_diagrams =
            build_workspace_activity_diagrams(&docs, &[uri_a.clone(), uri_b.clone()], None);
        assert!(
            all_diagrams
                .iter()
                .any(|diagram| diagram.name == "LaunchSystem" && diagram.source_kind == "performer"),
            "expected performer diagram to be aggregated from workspace files"
        );
        assert!(
            all_diagrams.iter().any(
                |diagram| diagram.name == "LaunchToOrbit" && diagram.source_kind == "actionDef"
            ),
            "expected action-def diagram to be aggregated from workspace files"
        );

        let filtered_diagrams = build_workspace_activity_diagrams(
            &docs,
            &[uri_a, uri_b],
            Some(("LogicalComponentsPackage", Some("LogicalComponentsPackage"))),
        );
        assert_eq!(filtered_diagrams.len(), 1);
        assert_eq!(filtered_diagrams[0].name, "LaunchSystem");
        assert_eq!(filtered_diagrams[0].source_kind, "performer");
    }

    #[test]
    fn package_groups_are_built_from_contains_hierarchy() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "P".to_string(),
                    element_type: "package".to_string(),
                    name: "P".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "P::Inner".to_string(),
                    element_type: "package".to_string(),
                    name: "Inner".to_string(),
                    uri: None,
                    parent_id: Some("P".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "P::Inner::x".to_string(),
                    element_type: "part".to_string(),
                    name: "x".to_string(),
                    uri: None,
                    parent_id: Some("P::Inner".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![
                GraphEdgeDto {
                    source: "P".to_string(),
                    target: "P::Inner".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "P::Inner".to_string(),
                    target: "P::Inner::x".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
            ],
        };

        let groups = build_package_groups_from_graph(&graph);
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().any(|group| group.id == "P"));
        assert!(groups
            .iter()
            .any(|group| group.id == "P::Inner" && group.parent_id.as_deref() == Some("P")));
        assert!(groups.iter().any(|group| group
            .node_ids
            .iter()
            .any(|node_id| node_id == "P::Inner::x")));
    }

    #[test]
    fn ibd_package_container_groups_follow_package_membership() {
        let parts = vec![
            IbdPartDto {
                id: "Drone::Vehicle".to_string(),
                name: "Vehicle".to_string(),
                qualified_name: "Drone.Vehicle".to_string(),
                uri: None,
                container_id: None,
                element_type: "part def".to_string(),
                attributes: HashMap::new(),
            },
            IbdPartDto {
                id: "Timer::TimerSystem".to_string(),
                name: "TimerSystem".to_string(),
                qualified_name: "Timer.TimerSystem".to_string(),
                uri: None,
                container_id: None,
                element_type: "part def".to_string(),
                attributes: HashMap::new(),
            },
        ];
        let groups = build_ibd_package_container_groups(
            &parts,
            &[
                SysmlVisualizationPackageCandidateDto {
                    id: "Drone".to_string(),
                    name: "Drone".to_string(),
                },
                SysmlVisualizationPackageCandidateDto {
                    id: "Timer".to_string(),
                    name: "Timer".to_string(),
                },
            ],
            None,
        );
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().any(|group| {
            group.id == "package:Drone"
                && group.member_part_ids == vec!["Drone::Vehicle".to_string()]
        }));
        assert!(groups.iter().any(|group| {
            group.id == "package:Timer"
                && group.member_part_ids == vec!["Timer::TimerSystem".to_string()]
        }));
    }

    #[test]
    fn attach_ibd_package_container_groups_populates_root_views_for_selected_package() {
        let payload = IbdDataDto {
            parts: vec![IbdPartDto {
                id: "Drone::Vehicle".to_string(),
                name: "Vehicle".to_string(),
                qualified_name: "Drone.Vehicle".to_string(),
                uri: None,
                container_id: None,
                element_type: "part def".to_string(),
                attributes: HashMap::new(),
            }],
            ports: Vec::new(),
            connectors: Vec::new(),
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: vec!["Vehicle".to_string()],
            default_root: Some("Vehicle".to_string()),
            root_views: HashMap::from([(
                "Vehicle".to_string(),
                IbdRootViewDto {
                    parts: vec![IbdPartDto {
                        id: "Drone::Vehicle".to_string(),
                        name: "Vehicle".to_string(),
                        qualified_name: "Drone.Vehicle".to_string(),
                        uri: None,
                        container_id: None,
                        element_type: "part def".to_string(),
                        attributes: HashMap::new(),
                    }],
                    ports: Vec::new(),
                    connectors: Vec::new(),
                    container_groups: Vec::new(),
                    package_container_groups: Vec::new(),
                },
            )]),
        };

        let attached = attach_ibd_package_container_groups(
            payload,
            &[SysmlVisualizationPackageCandidateDto {
                id: "Drone".to_string(),
                name: "Drone".to_string(),
            }],
            Some(("Drone", Some("Drone"))),
        );

        assert_eq!(attached.package_container_groups.len(), 1);
        assert_eq!(attached.package_container_groups[0].id, "package:Drone");
        assert_eq!(
            attached
                .root_views
                .get("Vehicle")
                .expect("root view")
                .package_container_groups
                .len(),
            1
        );
    }

    #[test]
    fn webshop_sequence_view_includes_sequence_diagrams() {
        let workspace_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/webshop");
        let response = build_sysml_visualization_for_paths(
            &workspace_path,
            Some(&workspace_path),
            &[],
            "sequence-view",
            Some("checkoutFlow"),
        )
        .expect("build visualization response");

        assert_eq!(response.view, "sequence-view");
        let diagrams = response
            .sequence_diagrams
            .expect("sequence diagrams payload should be present");
        assert!(
            !diagrams.is_empty(),
            "expected webshop SequenceView to produce at least one sequence diagram"
        );
    }

    #[test]
    fn semantic_core_graph_first_output_matches_kernel_on_core_view_metadata() {
        let workspace_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/webshop");
        let requested_view = "general-view";
        let requested_selected_view = Some("GeneralView");

        let kernel_response = build_sysml_visualization_for_paths(
            &workspace_path,
            Some(&workspace_path),
            &[],
            requested_view,
            requested_selected_view,
        )
        .expect("kernel visualization response");

        let provider = semantic_core::FileSystemDocumentProvider::new(
            workspace_path.clone(),
            Some(workspace_path.clone()),
            Vec::new(),
        );
        let (graph, docs) = semantic_core::build_semantic_graph_with_provider(&provider)
            .expect("semantic graph for workspace");
        let workspace_root_uri = super::path_to_url(&workspace_path).expect("workspace root uri");
        let semantic_core_response = semantic_core::build_sysml_visualization_workspace(
            &graph,
            &docs,
            &[],
            &workspace_root_uri,
            requested_view,
            requested_selected_view,
            Instant::now(),
        )
        .expect("semantic_core workspace visualization response");

        assert_eq!(kernel_response.view, semantic_core_response.view);
        assert!(
            kernel_response.selected_view.is_some(),
            "kernel should resolve a selected view"
        );
        assert!(
            kernel_response.selected_view_name.is_some(),
            "kernel should resolve a selected view name"
        );
        assert_eq!(
            semantic_core_response.selected_view,
            kernel_response.selected_view
        );
        assert_eq!(
            semantic_core_response.selected_view_name,
            kernel_response.selected_view_name
        );
        assert!(
            !kernel_response.view_candidates.is_empty(),
            "kernel should surface at least one candidate"
        );
        assert!(
            !semantic_core_response.view_candidates.is_empty(),
            "semantic_core should surface at least one candidate"
        );
        assert_eq!(
            kernel_response
                .view_candidates
                .iter()
                .map(|candidate| candidate.id.clone())
                .collect::<Vec<_>>(),
            semantic_core_response
                .view_candidates
                .iter()
                .map(|candidate| candidate.id.clone())
                .collect::<Vec<_>>(),
            "semantic_core graph-first API should align with kernel view candidates"
        );
    }

    #[test]
    fn interconnection_fixture_with_views_returns_ibd_connectors() {
        let workspace_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../vscode/testFixture/workspaces/interconnection-drone");
        let response = build_sysml_visualization_for_paths(
            &workspace_path,
            Some(&workspace_path),
            &[],
            "interconnection-view",
            Some("Views::droneConnections"),
        )
        .expect("build interconnection visualization");

        assert!(
            response.empty_state_message.is_none(),
            "expected views in fixture, got empty state: {:?}",
            response.empty_state_message
        );
        assert_eq!(
            response.selected_view.as_deref(),
            Some("Views::droneConnections")
        );
        let ibd = response.ibd.expect("ibd payload");
        assert!(
            !ibd.connectors.is_empty(),
            "expected SurveillanceDrone IBD connectors, got: {ibd:#?}"
        );
    }
}
