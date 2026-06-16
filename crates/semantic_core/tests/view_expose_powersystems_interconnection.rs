use std::path::{Path, PathBuf};

use semantic_core::semantic::dto::SysmlGraphDto;
use semantic_core::semantic::ibd::IbdDataDto;
use semantic_core::{
    build_ibd_for_uri, build_interconnection_scene, build_semantic_graph_from_documents,
    build_view_catalog, build_workspace_graph_dto_for_uris, evaluate_views,
    finalize_merged_ibd_connectors, merge_ibd_payloads, project_ids_for_renderer,
    select_interconnection_ibd_scope, EvaluatedView, InterconnectionSceneDto, SemanticGraph,
    SysmlDocument, SysmlDocumentSourceKind, WorkspaceParsedDocument,
};

type PowersystemsWorkspace = (
    Vec<SysmlDocument>,
    Vec<url::Url>,
    SemanticGraph,
    Vec<WorkspaceParsedDocument>,
);

fn optional_external_grid_fixture_sysml_root() -> Option<PathBuf> {
    let repo_root = PathBuf::from(std::env::var_os("SYSML_POWERSYSTEMS_DIR")?);
    let nested = repo_root.join("sysml");
    if nested.is_dir() {
        Some(nested)
    } else if repo_root.is_dir() {
        Some(repo_root)
    } else {
        None
    }
}

fn load_powersystems_workspace() -> Option<PowersystemsWorkspace> {
    let workspace_root = optional_external_grid_fixture_sysml_root()?;

    let mut documents = Vec::new();
    let mut uris = Vec::new();
    for entry in walkdir::WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sysml"))
    {
        let path = entry.path();
        let content = std::fs::read_to_string(path).expect("read power systems model");
        let doc = SysmlDocument::from_memory_path(
            "powersystems",
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("model.sysml"),
            content,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document uri");
        uris.push(doc.uri.clone());
        documents.push(doc);
    }

    let (graph, parsed) =
        build_semantic_graph_from_documents(&documents).expect("semantic graph should build");
    Some((documents, uris, graph, parsed))
}

fn powersystems_interconnection_fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("shared")
        .join("diagram-renderer")
        .join("test-fixtures")
        .join("interconnection")
        .join(name)
}

fn build_powersystems_scene(
    full_ibd: &IbdDataDto,
    graph_dto: &SysmlGraphDto,
    view: &EvaluatedView,
) -> InterconnectionSceneDto {
    let projected = project_ids_for_renderer(view, graph_dto, "interconnection-view");
    let scoped_ibd =
        select_interconnection_ibd_scope(full_ibd, &projected, Some(&view.exposed_ids));
    let root_ids = view
        .exposed_ids
        .iter()
        .map(|id| id.replace("::", "."))
        .collect::<Vec<_>>();
    build_interconnection_scene(&scoped_ibd, &view.id, &view.name, &root_ids, None)
}

fn write_scene_fixture(scene: &InterconnectionSceneDto, filename: &str) {
    let fixture_path = powersystems_interconnection_fixture_path(filename);
    if let Some(parent) = fixture_path.parent() {
        std::fs::create_dir_all(parent).expect("create fixture directory");
    }
    std::fs::write(
        &fixture_path,
        serde_json::to_string_pretty(scene).expect("serialize scene"),
    )
    .expect("write grid scene fixture fixture");
    eprintln!("wrote {}", fixture_path.display());
}

#[test]
#[ignore = "optional local drill-down; set SYSML_POWERSYSTEMS_DIR to an external grid fixture checkout"]
fn powersystems_grid_connections_ibd_includes_feeder_and_cable_connectors() {
    let Some(workspace_root) = optional_external_grid_fixture_sysml_root() else {
        return;
    };

    let mut documents = Vec::new();
    let mut uris = Vec::new();
    for entry in walkdir::WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sysml"))
    {
        let path = entry.path();
        let content = std::fs::read_to_string(path).expect("read power systems model");
        let doc = SysmlDocument::from_memory_path(
            "powersystems",
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("model.sysml"),
            content,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document uri");
        uris.push(doc.uri.clone());
        documents.push(doc);
    }

    let (graph, parsed) =
        build_semantic_graph_from_documents(&documents).expect("semantic graph should build");
    assert!(
        graph.nodes_for_uri(&uris[0]).iter().any(|node| {
            node.id
                .qualified_name
                .ends_with("northSouthRing::ringSegmentBtoC")
        }) || uris.iter().any(|uri| {
            graph.nodes_for_uri(uri).iter().any(|node| {
                node.id
                    .qualified_name
                    .ends_with("northSouthRing::ringSegmentBtoC")
            })
        }),
        "semantic graph should contain inline nested part northSouthRing::ringSegmentBtoC"
    );
    let mut full_ibd = merge_ibd_payloads(
        uris.iter()
            .map(|uri| build_ibd_for_uri(&graph, uri))
            .collect(),
    );
    finalize_merged_ibd_connectors(&graph, &uris, &mut full_ibd);
    assert!(
        full_ibd
            .parts
            .iter()
            .any(|part| part.qualified_name.ends_with("northSouthRing.ringSegmentBtoC")),
        "full merged IBD should contain inline nested part northSouthRing.ringSegmentBtoC; parts: {:?}",
        full_ibd.parts
    );
    let catalog = build_view_catalog(&uris, &parsed);
    let graph_dto = build_workspace_graph_dto_for_uris(&graph, &uris);
    let evaluated = evaluate_views(&catalog, &graph, &graph_dto);
    let view = evaluated
        .iter()
        .find(|view| view.name == "gridConnections")
        .expect("gridConnections view");
    let projected = project_ids_for_renderer(view, &graph_dto, "interconnection-view");
    let architecture_connectors: Vec<_> = full_ibd
        .connectors
        .iter()
        .filter(|connector| {
            connector
                .source_id
                .contains("regionalExpansionProject.architecture")
        })
        .collect();
    assert!(
        architecture_connectors.len() >= 15,
        "merged IBD should include architecture instance connectors, got {}: {:?}",
        architecture_connectors.len(),
        architecture_connectors
    );
    assert!(
        full_ibd.ports.iter().any(|port| {
            port.parent_id.contains("feederNorth")
                && (port.name == "source" || port.name == "outgoing")
        }),
        "full IBD feederNorth ports: {:?}",
        full_ibd
            .ports
            .iter()
            .filter(|port| port.parent_id.contains("feederNorth"))
            .collect::<Vec<_>>()
    );

    let ibd = select_interconnection_ibd_scope(&full_ibd, &projected, Some(&view.exposed_ids));

    assert!(
        !ibd.connectors.iter().any(|connector| {
            connector.source_id.contains(".Variants.")
                || connector.target_id.contains(".Variants.")
                || connector.source_id.contains(".expansionAlternatives.")
                || connector.target_id.contains(".expansionAlternatives.")
        }),
        "gridConnections should not include variant or alternative connectors, got {:?}",
        ibd.connectors
    );
    assert!(
        ibd.connectors.iter().all(|connector| {
            connector
                .source_id
                .contains("regionalExpansionProject.architecture")
                && connector
                    .target_id
                    .contains("regionalExpansionProject.architecture")
        }),
        "gridConnections connectors should stay under architecture instance paths, got {:?}",
        ibd.connectors
    );
    assert!(
        ibd.connectors.len() >= 15,
        "gridConnections should expose the full architecture connect set, got {} scoped connectors (full={}): {:?}",
        ibd.connectors.len(),
        full_ibd.connectors.len(),
        ibd.connectors
    );
    assert!(
        ibd.ports.iter().any(|port| {
            port.parent_id.contains("feederNorth")
                && (port.name == "source" || port.name == "outgoing")
        }),
        "feederNorth should inherit MediumVoltageFeeder ports, got ports: {:?}",
        ibd.ports
    );
    assert!(
        ibd.connectors.iter().any(|connector| {
            connector.source_id.contains("feederNorth") && connector.target_id.contains("cable01")
        }),
        "expected feederNorth to cable01 connection, got: {:?}",
        ibd.connectors
    );
    assert!(
        ibd.parts
            .iter()
            .any(|part| part.qualified_name.ends_with("northSouthRing.ringSegmentBtoC")),
        "northSouthRing.ringSegmentBtoC should be included because gridConnections references its ports; parts: {:?}",
        ibd.parts
    );
    assert!(
        ibd.parts
            .iter()
            .any(|part| part.qualified_name.ends_with("northSouthRing.noTiePoint")),
        "northSouthRing.noTiePoint should be included because gridConnections references its ports; parts: {:?}",
        ibd.parts
    );
    assert!(
        ibd.ports.iter().any(|port| {
            port.parent_id.ends_with("northSouthRing.ringSegmentBtoC")
                && (port.name == "a" || port.name == "b")
        }),
        "ringSegmentBtoC should expose inherited segment ports, got ports: {:?}",
        ibd.ports
    );
    assert!(
        ibd.ports.iter().any(|port| {
            port.parent_id.ends_with("northSouthRing.noTiePoint")
                && (port.name == "incoming" || port.name == "outgoing")
        }),
        "noTiePoint should expose inherited switchgear ports, got ports: {:?}",
        ibd.ports
    );

    let system_context = evaluated
        .iter()
        .find(|view| view.name == "systemContext")
        .expect("systemContext view");
    let system_projected =
        project_ids_for_renderer(system_context, &graph_dto, "interconnection-view");
    let system_ibd = select_interconnection_ibd_scope(
        &full_ibd,
        &system_projected,
        Some(&system_context.exposed_ids),
    );
    assert!(
        system_ibd.connectors.iter().any(|connector| {
            connector.source_id.contains("tennetConnection")
                && connector.target_id.contains("primarySubstation")
        }),
        "systemContext should include tennetConnection -> primarySubstation connector, got {:?}",
        system_ibd.connectors
    );
    assert!(
        system_ibd.connectors.iter().any(|connector| {
            connector.source_id.contains("txStationA")
                && connector.target_id.contains("residentialAreaA")
        }),
        "systemContext should include txStationA -> residentialAreaA connector, got {:?}",
        system_ibd.connectors
    );
    assert!(
        system_ibd.connectors.iter().any(|connector| {
            connector.source_id.contains("txStationC")
                && connector.target_id.contains("industrialClusterA")
        }),
        "systemContext should include txStationC -> industrialClusterA connector, got {:?}",
        system_ibd.connectors
    );
    let expected_paths = [
        (
            "tennetConnection.connection",
            "primarySubstation.hvConnection",
        ),
        (
            "primarySubstation.mvBusbar.northFeederTap",
            "northFeederBay.incoming",
        ),
        (
            "primarySubstation.mvBusbar.southFeederTap",
            "southFeederBay.incoming",
        ),
        ("northFeederBay.outgoing", "feederNorth.source"),
        ("southFeederBay.outgoing", "feederSouth.source"),
        ("feederNorth.outgoing", "cable01.a"),
        ("cable01.b", "txStationA.mvConnection"),
        ("feederNorth.outgoing", "cable02.a"),
        ("cable02.b", "txStationB.mvConnection"),
        ("feederSouth.outgoing", "cable03.a"),
        ("cable03.b", "txStationC.mvConnection"),
        (
            "txStationB.mvConnection",
            "northSouthRing.ringSegmentBtoC.a",
        ),
        (
            "northSouthRing.ringSegmentBtoC.b",
            "northSouthRing.noTiePoint.incoming",
        ),
        (
            "northSouthRing.noTiePoint.outgoing",
            "txStationC.mvConnection",
        ),
        ("txStationA.lvConnection", "residentialAreaA.gridConnection"),
        ("txStationB.lvConnection", "residentialAreaB.gridConnection"),
        (
            "txStationC.lvConnection",
            "industrialClusterA.gridConnection",
        ),
    ];
    for (source_suffix, target_suffix) in expected_paths {
        let connector = system_ibd
            .connectors
            .iter()
            .find(|connector| {
                connector.source_id.contains("regionalExpansionProject.architecture")
                    && !connector.source_id.contains(".Variants.")
                    && !connector.source_id.contains(".expansionAlternatives.")
                    && connector.source_id.ends_with(source_suffix)
                    && connector.target_id.ends_with(target_suffix)
            })
            .unwrap_or_else(|| {
                panic!(
                    "systemContext missing expected connector {source_suffix} -> {target_suffix}; got {:?}",
                    system_ibd.connectors
                )
            });
        assert!(
            connector
                .source_port_id
                .as_deref()
                .is_some_and(|id| id.ends_with(source_suffix)),
            "expected sourcePortId for {source_suffix} -> {target_suffix}, got {:?}",
            connector
        );
        assert!(
            connector
                .target_port_id
                .as_deref()
                .is_some_and(|id| id.ends_with(target_suffix)),
            "expected targetPortId for {source_suffix} -> {target_suffix}, got {:?}",
            connector
        );
    }

    let scene = build_interconnection_scene(
        &system_ibd,
        &system_context.id,
        &system_context.name,
        &["RegionalGridExpansion.architecture".to_string()],
        None,
    );
    assert_eq!(scene.schema_version, 2);
    assert!(
        scene.edges.len() >= expected_paths.len(),
        "systemContext scene should expose at least {} connectors, got {}",
        expected_paths.len(),
        scene.edges.len()
    );
    assert!(
        scene.edges.iter().any(|edge| {
            edge.target_node_id.ends_with("ringSegmentBtoC")
                || edge
                    .semantic_id
                    .as_deref()
                    .is_some_and(|id| id.contains("ringSegmentBtoC.a"))
        }),
        "scene should route nested ring segment endpoints to the nested owner"
    );
}

#[test]
#[ignore = "fixture export; set SYSML_POWERSYSTEMS_DIR to regenerate grid scene JSON"]
fn export_powersystems_system_context_scene() {
    let Some((_documents, uris, graph, parsed)) = load_powersystems_workspace() else {
        return;
    };
    let mut full_ibd = merge_ibd_payloads(
        uris.iter()
            .map(|uri| build_ibd_for_uri(&graph, uri))
            .collect(),
    );
    finalize_merged_ibd_connectors(&graph, &uris, &mut full_ibd);
    let catalog = build_view_catalog(&uris, &parsed);
    let graph_dto = build_workspace_graph_dto_for_uris(&graph, &uris);
    let evaluated = evaluate_views(&catalog, &graph, &graph_dto);
    let system_context = evaluated
        .iter()
        .find(|view| view.name == "systemContext")
        .expect("systemContext view");
    let scene = build_powersystems_scene(&full_ibd, &graph_dto, system_context);
    write_scene_fixture(&scene, "grid-system-context-scene.json");
}

#[test]
#[ignore = "fixture export; set SYSML_POWERSYSTEMS_DIR to regenerate grid scene JSON"]
fn export_powersystems_grid_connections_scene() {
    let Some((_documents, uris, graph, parsed)) = load_powersystems_workspace() else {
        return;
    };
    let mut full_ibd = merge_ibd_payloads(
        uris.iter()
            .map(|uri| build_ibd_for_uri(&graph, uri))
            .collect(),
    );
    finalize_merged_ibd_connectors(&graph, &uris, &mut full_ibd);
    let catalog = build_view_catalog(&uris, &parsed);
    let graph_dto = build_workspace_graph_dto_for_uris(&graph, &uris);
    let evaluated = evaluate_views(&catalog, &graph, &graph_dto);
    let grid_connections = evaluated
        .iter()
        .find(|view| view.name == "gridConnections")
        .expect("gridConnections view");
    let scene = build_powersystems_scene(&full_ibd, &graph_dto, grid_connections);

    assert!(
        !scene.edges.iter().any(|edge| {
            edge.semantic_id.as_deref().is_some_and(|id| {
                id.contains(".Variants.") || id.contains(".expansionAlternatives.")
            })
        }),
        "gridConnections scene should not include variant connectors, got {:?}",
        scene.edges
    );
    assert!(
        scene.edges.iter().any(|edge| {
            edge.semantic_id
                .as_deref()
                .is_some_and(|id| id.contains("feederNorth.outgoing") && id.contains("cable01.a"))
        }),
        "gridConnections scene should include feederNorth to cable01 connector, got {:?}",
        scene.edges
    );
    assert!(
        scene.edges.iter().any(|edge| {
            edge.target_node_id.ends_with("ringSegmentBtoC")
                || edge
                    .semantic_id
                    .as_deref()
                    .is_some_and(|id| id.contains("ringSegmentBtoC.a"))
        }),
        "gridConnections scene should include nested ring segment endpoints, got {:?}",
        scene.edges
    );
    assert!(
        scene.edges.len() >= 15,
        "gridConnections scene should expose the architecture connector set, got {}",
        scene.edges.len()
    );

    write_scene_fixture(&scene, "grid-connections-scene.json");
}
