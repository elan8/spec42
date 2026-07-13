    use std::collections::HashMap;

    use url::Url;

    use super::DefInstanceMappingDto;

    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
    use crate::semantic::workspace_graph::build_semantic_graph_from_documents;

    use crate::semantic::ibd::connectors::enrich_connector_endpoint_refs;
    use crate::semantic::ibd::dto::{
        IbdConnectorDto, IbdDataDto, IbdPartDto, IbdPortDto,
    };
    use crate::semantic::ibd::{
        build_ibd_for_uri, finalize_merged_ibd_connectors, merge_ibd_payloads,
        normalize_ibd_to_instance_paths,
    };
    use super::{
        build_container_groups, infer_port_side, prune_ibd_payload_to_connected_scope,
        prune_interconnection_definition_parts,
    };

    fn test_part(
        id: &str,
        name: &str,
        qualified_name: &str,
        container_id: Option<&str>,
        element_type: &str,
    ) -> IbdPartDto {
        IbdPartDto {
            id: id.to_string(),
            node_id: qualified_name.to_string(),
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            uri: None,
            container_id: container_id.map(String::from),
            element_type: element_type.to_string(),
            attributes: HashMap::new(),
            range: None,
        }
    }

    fn test_port(id: &str, name: &str, parent_id: &str) -> IbdPortDto {
        IbdPortDto {
            id: id.to_string(),
            port_id: format!("{parent_id}.{name}"),
            name: name.to_string(),
            parent_id: parent_id.to_string(),
            direction: None,
            port_type: None,
            port_side: None,
            uri: None,
            range: None,
        }
    }

    fn test_connector(source_id: &str, target_id: &str) -> IbdConnectorDto {
        IbdConnectorDto {
            source: source_id.to_string(),
            target: target_id.to_string(),
            source_id: source_id.to_string(),
            target_id: target_id.to_string(),
            source_part_id: None,
            target_part_id: None,
            source_port_id: None,
            target_port_id: None,
            rel_type: "connection".to_string(),
        }
    }

    #[test]
    fn infer_port_side_prefers_direction() {
        assert_eq!(
            infer_port_side("power_out", Some("in"), Some("PowerPort")),
            Some("left".to_string())
        );
        assert_eq!(
            infer_port_side("sensor_in", Some("out"), Some("SensorPort")),
            Some("right".to_string())
        );
    }

    #[test]
    fn infer_port_side_uses_generic_name_hints() {
        assert_eq!(
            infer_port_side("camera_input", None, None),
            Some("left".to_string())
        );
        assert_eq!(
            infer_port_side("telemetryOutput", None, None),
            Some("right".to_string())
        );
        assert_eq!(
            infer_port_side("fuel_in", None, None),
            Some("left".to_string())
        );
        assert_eq!(
            infer_port_side("payload_out", None, None),
            Some("right".to_string())
        );
    }

    #[test]
    fn infer_port_side_does_not_use_model_specific_type_names() {
        assert_eq!(infer_port_side("status", None, Some("PowerPort")), None);
        assert_eq!(
            infer_port_side("status", None, Some("~TelemetryPort")),
            None
        );
    }

    #[test]
    fn prune_ibd_keeps_unconnected_parts_under_same_composite() {
        let parts = vec![
            test_part("O::Desk", "desk", "O.Desk", None, "part"),
            test_part(
                "O::Desk::connected",
                "connected",
                "O.Desk.connected",
                Some("O.Desk"),
                "part",
            ),
            test_part(
                "O::Desk::orphan",
                "orphan",
                "O.Desk.orphan",
                Some("O.Desk"),
                "part",
            ),
            test_part(
                "O::Desk::orphan::nested",
                "nested",
                "O.Desk.orphan.nested",
                Some("O.Desk.orphan"),
                "part",
            ),
        ];
        let ports = vec![
            test_port("O.Desk.connected.p1", "p1", "O.Desk.connected"),
            test_port("O.Desk.connected.p2", "p2", "O.Desk.connected"),
        ];
        let connectors = vec![test_connector("O.Desk.connected.p1", "O.Desk.connected.p2")];

        let (parts, _ports, _connectors) =
            prune_ibd_payload_to_connected_scope(parts, ports, connectors);

        let qns: Vec<&str> = parts.iter().map(|p| p.qualified_name.as_str()).collect();
        assert!(qns.contains(&"O.Desk"));
        assert!(qns.contains(&"O.Desk.connected"));
        assert!(
            qns.contains(&"O.Desk.orphan"),
            "sibling part with no connectors should remain in IBD payload"
        );
        assert!(
            qns.contains(&"O.Desk.orphan.nested"),
            "nested parts under an unconnected sibling should remain"
        );
    }

    #[test]
    fn container_groups_are_derived_from_part_qualified_names() {
        let parts = vec![
            test_part("P::Inner::a", "a", "P.Inner.a", None, "part"),
            test_part("P::Inner::b", "b", "P.Inner.b", None, "part"),
        ];
        let groups = build_container_groups(&parts, &|_| false);
        assert!(groups
            .iter()
            .any(|group| group.qualified_name == "P" && group.member_part_ids.len() == 2));
        assert!(groups
            .iter()
            .any(|group| group.qualified_name == "P.Inner" && group.member_part_ids.len() == 2));
    }

    #[test]
    fn connector_endpoint_refs_use_nested_port_owner() {
        let parts = vec![
            test_part(
                "Grid::northSouthRing",
                "northSouthRing",
                "Grid.northSouthRing",
                None,
                "part",
            ),
            test_part(
                "Grid::northSouthRing::ringSegmentBtoC",
                "ringSegmentBtoC",
                "Grid.northSouthRing.ringSegmentBtoC",
                Some("Grid.northSouthRing"),
                "part",
            ),
            test_part(
                "Grid::txStationB",
                "txStationB",
                "Grid.txStationB",
                None,
                "part",
            ),
        ];
        let ports = vec![
            test_port(
                "Grid.northSouthRing.ringSegmentBtoC.a",
                "a",
                "Grid.northSouthRing.ringSegmentBtoC",
            ),
            test_port(
                "Grid.txStationB.mvConnection",
                "mvConnection",
                "Grid.txStationB",
            ),
        ];
        let mut connectors = vec![IbdConnectorDto {
            source: "Grid.txStationB.mvConnection".to_string(),
            target: "Grid.northSouthRing.ringSegmentBtoC.a".to_string(),
            source_id: "Grid.txStationB.mvConnection".to_string(),
            target_id: "Grid.northSouthRing.ringSegmentBtoC.a".to_string(),
            source_part_id: Some("Grid.txStationB".to_string()),
            target_part_id: Some("Grid.northSouthRing".to_string()),
            source_port_id: None,
            target_port_id: None,
            rel_type: "connection".to_string(),
        }];

        enrich_connector_endpoint_refs(&mut connectors, &parts, &ports);

        assert_eq!(
            connectors[0].target_part_id.as_deref(),
            Some("Grid.northSouthRing.ringSegmentBtoC")
        );
        assert_eq!(
            connectors[0].target_port_id.as_deref(),
            Some("Grid.northSouthRing.ringSegmentBtoC.a")
        );
        assert_eq!(
            connectors[0].source_port_id.as_deref(),
            Some("Grid.txStationB.mvConnection")
        );
    }

    #[test]
    fn build_ibd_mirrors_definition_connections_onto_cross_file_instance() {
        let architecture = r#"
            package WebShopArchitecture {
                part def Storefront {
                    port checkoutApiOut;
                }
                part def ApiGateway {
                    port publicCheckoutIn;
                }
                part def WebShopSystem {
                    part storefront : Storefront;
                    part apiGateway : ApiGateway;
                    connect storefront.checkoutApiOut to apiGateway.publicCheckoutIn;
                }
            }
        "#;
        let instance = r#"
            package WebShopExample {
                import WebShopArchitecture::*;
                part webshopSystem : WebShopSystem;
            }
        "#;

        let arch_doc = SysmlDocument::from_memory_path(
            "workspace",
            "WebShopArchitecture.sysml",
            architecture.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("arch doc");
        let instance_doc = SysmlDocument::from_memory_path(
            "workspace",
            "webshop.sysml",
            instance.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("instance doc");
        let architecture_uri = arch_doc.uri.clone();
        let instance_uri = instance_doc.uri.clone();

        let (graph, _parsed) =
            build_semantic_graph_from_documents(&[arch_doc, instance_doc]).expect("graph");

        let merged = merge_ibd_payloads(vec![
            build_ibd_for_uri(&graph, &architecture_uri),
            build_ibd_for_uri(&graph, &instance_uri),
        ]);

        let root_view = merged
            .root_views
            .get("webshopSystem")
            .expect("webshopSystem root view");
        assert!(
            root_view.connectors.iter().any(|connector| {
                connector.source_id.contains("webshopSystem.storefront")
                    && connector.source_id.contains("checkoutApiOut")
                    && connector.target_id.contains("webshopSystem.apiGateway")
                    && connector.target_id.contains("publicCheckoutIn")
            }),
            "expected mirrored storefrontâ†’gateway connector, got {:?}",
            root_view.connectors
        );
    }

    #[test]
    fn finalize_merged_ibd_remapped_definition_connects_to_typed_instance() {
        let architecture = SysmlDocument::from_memory_path(
            "powersystems",
            "Architecture.sysml",
            r#"package RegionalGridExpansion::Architecture {
    part def RegionalGridArchitecture {
        part feederNorth { port outgoing; }
        part cable01 { port a; port b; }
        connect feederNorth.outgoing to cable01.a;
    }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("architecture uri");
        let project = SysmlDocument::from_memory_path(
            "powersystems",
            "Project.sysml",
            r#"package RegionalGridExpansion {
    public import RegionalGridExpansion::Architecture::*;
    part regionalExpansionProject {
        part architecture : RegionalGridArchitecture;
    }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("project uri");
        let uris = [architecture.uri.clone(), project.uri.clone()];
        let (graph, _) =
            build_semantic_graph_from_documents(&[architecture, project]).expect("graph");
        let mut merged = merge_ibd_payloads(
            uris.iter()
                .map(|uri| build_ibd_for_uri(&graph, uri))
                .collect(),
        );
        finalize_merged_ibd_connectors(&graph, &uris, &mut merged);
        assert!(
            merged.connectors.iter().any(|connector| {
                connector
                    .source_id
                    .contains("regionalExpansionProject.architecture.feederNorth")
                    && connector
                        .target_id
                        .contains("regionalExpansionProject.architecture.cable01")
            }),
            "expected definition-level connect mirrored to project architecture instance, got {:?}",
            merged.connectors
        );
    }

    #[test]
    fn normalize_ibd_remaps_non_regional_architecture_ports_to_instance_paths() {
        let mut ibd = IbdDataDto {
            parts: vec![
                test_part(
                    "StedinRijnmondGridExpansion::Architecture::RijnmondGridArchitecture::feederNorth",
                    "feederNorth",
                    "StedinRijnmondGridExpansion.Architecture.RijnmondGridArchitecture.feederNorth",
                    None,
                    "part",
                ),
                test_part(
                    "StedinRijnmondGridExpansion::rijnmondExpansionProject::architecture::feederNorth",
                    "feederNorth",
                    "StedinRijnmondGridExpansion.rijnmondExpansionProject.architecture.feederNorth",
                    None,
                    "part",
                ),
            ],
            ports: vec![test_port(
                "StedinRijnmondGridExpansion.Architecture.RijnmondGridArchitecture.feederNorth.outgoing",
                "outgoing",
                "StedinRijnmondGridExpansion.Architecture.RijnmondGridArchitecture.feederNorth",
            )],
            connectors: vec![IbdConnectorDto {
                source: "StedinRijnmondGridExpansion.rijnmondExpansionProject.architecture.feederNorth.outgoing".to_string(),
                target: "StedinRijnmondGridExpansion.rijnmondExpansionProject.architecture.feederNorth.outgoing".to_string(),
                source_id: "StedinRijnmondGridExpansion.rijnmondExpansionProject.architecture.feederNorth.outgoing".to_string(),
                target_id: "StedinRijnmondGridExpansion.rijnmondExpansionProject.architecture.feederNorth.outgoing".to_string(),
                source_part_id: None,
                target_part_id: None,
                source_port_id: None,
                target_port_id: None,
                rel_type: "connection".to_string(),
            }],
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: Vec::new(),
            default_root: None,
            root_views: std::collections::HashMap::new(),
            // In production this is always populated by `build_instance_def_mappings` from real
            // typing edges (see `ibd/connectors.rs`); this hand-built fixture supplies the
            // equivalent mapping directly since it doesn't go through `build_ibd_for_uri`.
            def_instance_mappings: vec![DefInstanceMappingDto {
                def_root: "StedinRijnmondGridExpansion.Architecture.RijnmondGridArchitecture"
                    .to_string(),
                instance_root: "StedinRijnmondGridExpansion.rijnmondExpansionProject.architecture"
                    .to_string(),
            }],
        };

        normalize_ibd_to_instance_paths(&mut ibd);

        assert!(ibd.ports.iter().any(|port| {
            port.port_id
                == "StedinRijnmondGridExpansion.rijnmondExpansionProject.architecture.feederNorth.outgoing"
        }));
    }

    #[test]
    fn build_ibd_materializes_inline_children_of_typed_part_usage_on_instance() {
        let architecture = SysmlDocument::from_memory_path(
            "workspace",
            "Architecture.sysml",
            r#"package GridArchitecture {
    part def Segment {
        port a;
        port b;
    }
    part def TiePoint {
        port incoming;
        port outgoing;
    }
    part def Ring;
    part def System {
        part ring : Ring {
            part segment : Segment;
            part tie : TiePoint;
        }
        connect ring.segment.b to ring.tie.incoming;
    }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("architecture uri");
        let project = SysmlDocument::from_memory_path(
            "workspace",
            "Project.sysml",
            r#"package Project {
    import GridArchitecture::*;
    part system : System;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("project uri");
        let uris = [architecture.uri.clone(), project.uri.clone()];
        let (graph, _) =
            build_semantic_graph_from_documents(&[architecture, project]).expect("graph");
        let mut merged = merge_ibd_payloads(
            uris.iter()
                .map(|uri| build_ibd_for_uri(&graph, uri))
                .collect(),
        );
        finalize_merged_ibd_connectors(&graph, &uris, &mut merged);

        assert!(
            merged
                .parts
                .iter()
                .any(|part| part.qualified_name == "Project.system.ring.segment"),
            "expected inline nested segment materialized under instance path, got {:?}",
            merged.parts
        );
        assert!(
            merged
                .parts
                .iter()
                .any(|part| part.qualified_name == "Project.system.ring.tie"),
            "expected inline nested tie materialized under instance path, got {:?}",
            merged.parts
        );
        assert!(
            merged.ports.iter().any(|port| {
                port.parent_id == "Project.system.ring.segment" && port.name == "b"
            }),
            "expected segment ports under instance path, got {:?}",
            merged.ports
        );
        assert!(
            merged.connectors.iter().any(|connector| {
                connector.source_id == "Project.system.ring.segment.b"
                    && connector.target_id == "Project.system.ring.tie.incoming"
            }),
            "expected connector to instance inline nested ports, got {:?}",
            merged.connectors
        );
    }

    #[test]
    fn build_ibd_includes_ports_inherited_from_generalized_part_def() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            r#"package Grid {
    part def MediumVoltageFeeder {
        port source;
        port outgoing;
    }
    part def DutchMVFeeder :> MediumVoltageFeeder;
    part def System {
        part feederNorth : DutchMVFeeder;
    }
    part system : System;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document uri");
        let uri = doc.uri.clone();
        let (graph, _) =
            build_semantic_graph_from_documents(&[doc]).expect("semantic graph should build");
        let ibd = build_ibd_for_uri(&graph, &uri);
        let feeder_ports: Vec<_> = ibd
            .ports
            .iter()
            .filter(|port| port.parent_id.contains("feederNorth"))
            .map(|port| port.name.as_str())
            .collect();
        assert!(
            feeder_ports.contains(&"source") && feeder_ports.contains(&"outgoing"),
            "expected inherited feeder ports on instance path, got {:?}",
            feeder_ports
        );
    }

    #[test]
    fn build_ibd_expands_library_typed_part_usage() {
        let library = SysmlDocument::from_memory_path(
            "library",
            "Domain.sysml",
            r#"package Domain {
  part def Robot {
    part motor;
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Library,
            None,
            None,
        )
        .expect("library doc");
        let workspace = SysmlDocument::from_memory_path(
            "workspace",
            "Architecture.sysml",
            r#"package Architecture {
  part robot : Domain::Robot;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let uri = workspace.uri.clone();
        let (graph, _) = build_semantic_graph_from_documents(&[workspace, library])
            .expect("semantic graph should build");
        let ibd = build_ibd_for_uri(&graph, &uri);
        assert!(
            ibd.root_candidates.iter().any(|root| root == "robot"),
            "expected robot as IBD root, got {:?}",
            ibd.root_candidates
        );
        assert!(
            ibd.parts.iter().any(|part| part.name == "motor"),
            "expected library-defined motor part in expanded tree, got {:?}",
            ibd.parts.iter().map(|part| part.name.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn build_ibd_materializes_pending_connection_endpoints_for_untyped_connects() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            r#"package Architecture {
  part def PowerSubsystem {
    port powerOut;
  }
  part def ControlSoftware {
    port powerIn;
  }
  part AutonomousFloorCleaningRobot {
    part power : PowerSubsystem;
    part control : ControlSoftware;
    connect power.powerOut to control.powerIn;
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/model.sysml").expect("uri");
        let ibd = build_ibd_for_uri(&graph, &uri);
        assert!(
            ibd.connectors.iter().any(|connector| {
                connector.source_id == "Architecture.AutonomousFloorCleaningRobot.power.powerOut"
                    && connector.target_id
                        == "Architecture.AutonomousFloorCleaningRobot.control.powerIn"
            }),
            "expected pending connect endpoints to materialize as IBD connector: {:?}",
            ibd.connectors
        );
    }

    #[test]
    fn build_ibd_surveillance_drone_instance_has_nested_parts_and_connectors() {
        let fixture = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../lsp_server/tests/fixtures/surveillance_drone_full.sysml"),
        )
        .expect("read surveillance drone fixture");
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "surveillance_drone_full.sysml",
            fixture,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/surveillance_drone_full.sysml").expect("uri");
        let ibd = build_ibd_for_uri(&graph, &uri);

        assert_eq!(
            ibd.default_root.as_deref(),
            Some("droneInstance"),
            "expected drone instance as default root, got {:?}",
            ibd.default_root
        );
        assert!(
            ibd.connectors.len() >= 14,
            "expected drone connector set, got {:?}",
            ibd.connectors.len()
        );
        assert!(
            ibd.parts
                .iter()
                .any(|part| { part.qualified_name.ends_with("propulsion.propulsionUnit4") }),
            "expected expanded propulsion unit in IBD, got {:?}",
            ibd.parts
                .iter()
                .map(|p| &p.qualified_name)
                .collect::<Vec<_>>()
        );
        assert!(
            ibd.connectors.iter().any(|connector| {
                connector.source_id.ends_with("flightController.motorCmd")
                    && connector.target_id.ends_with("propulsionUnit1.cmd")
            }),
            "expected motor command connector under drone instance, got {:?}",
            ibd.connectors
        );
        for unit in ["propulsionUnit2", "propulsionUnit3", "propulsionUnit4"] {
            assert!(
                ibd.connectors.iter().any(|connector| {
                    connector.source_id.ends_with("flightController.motorCmd")
                        && connector.target_id.ends_with(&format!("{unit}.cmd"))
                }),
                "expected motor command connector to {unit}, got {:?}",
                ibd.connectors
            );
            assert!(
                ibd.connectors
                    .iter()
                    .any(|connector| { connector.target_id.ends_with(&format!("{unit}.pwr")) }),
                "expected power connector to {unit}, got {:?}",
                ibd.connectors
            );
        }

        let default_root = ibd.default_root.as_deref().expect("default root");
        let root_view = ibd.root_views.get(default_root).expect("default root view");
        assert!(
            root_view.connectors.len() >= 14,
            "expected default root view to include connector set, got {} in {:?}: {:?}",
            root_view.connectors.len(),
            default_root,
            root_view.connectors
        );
        for part in &ibd.parts {
            assert!(
                !part.element_type.to_lowercase().contains(" def"),
                "IBD parts must not include definitions: {:?}",
                part
            );
        }
        for part in &root_view.parts {
            assert!(
                !part.element_type.to_lowercase().contains(" def"),
                "scoped IBD parts must not include definitions: {:?}",
                part
            );
        }
    }

    #[test]
    fn ibd_payload_excludes_definitions_from_connected_scope() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "parts_tree.sysml",
            r#"package PartsTree {
  part def Tree {
    part branch;
  }
  part def Vehicle {
    part tree : Tree;
  }
  part vehicle : Vehicle;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/parts_tree.sysml").expect("uri");
        let ibd = build_ibd_for_uri(&graph, &uri);

        assert!(
            ibd.parts
                .iter()
                .all(|part| !part.element_type.to_lowercase().contains(" def")),
            "expected no part definitions in IBD payload, got {:?}",
            ibd.parts
                .iter()
                .map(|part| (&part.name, &part.element_type))
                .collect::<Vec<_>>()
        );
        for view in ibd.root_views.values() {
            assert!(
                view.parts
                    .iter()
                    .all(|part| !part.element_type.to_lowercase().contains(" def")),
                "scoped IBD must not include definitions: {:?}",
                view.parts
            );
        }
    }

    #[test]
    fn build_ibd_resolves_connectors_with_bare_own_port_endpoint_and_no_phantom_package_box() {
        // Regression test for a real-world model (sysml-robot-vacuum-cleaner) where every
        // connector inside `part def DriveModule` names its own port endpoint bare
        // (e.g. `phaseLeftIn`) while the other endpoint is a two-segment member chain
        // (e.g. `leftMotor.phaseIn`). All 5 connectors were previously dropped because the
        // two-segment endpoint contains `::` internally and was mistaken for an already
        // fully-qualified path, so it never got prefixed with the enclosing definition. The
        // enclosing package (`PhysicalArchitecture`) was also incorrectly rendered as an empty
        // container box alongside the legitimate `AutonomousFloorCleaningRobot` part-def box.
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            r#"package PhysicalArchitecture {
  port def ThreePhaseMotorPort;
  port def QuadratureEncoderPort;
  port def GpioPort;

  part def BrushlessDriveMotor {
    port phaseIn : ThreePhaseMotorPort;
  }
  part def WheelEncoder {
    port odometryOut : QuadratureEncoderPort;
  }
  part def BumperSwitchArray {
    port hazardOut : GpioPort;
  }
  part def DriveModule {
    port phaseLeftIn : ThreePhaseMotorPort;
    port phaseRightIn : ThreePhaseMotorPort;
    port leftEncoderOut : QuadratureEncoderPort;
    port rightEncoderOut : QuadratureEncoderPort;
    port bumperHazardOut : GpioPort;
    part leftMotor : BrushlessDriveMotor;
    part rightMotor : BrushlessDriveMotor;
    part leftEncoder : WheelEncoder;
    part rightEncoder : WheelEncoder;
    part bumperSwitches : BumperSwitchArray;
    connect leftMotor.phaseIn to phaseLeftIn;
    connect rightMotor.phaseIn to phaseRightIn;
    connect leftEncoder.odometryOut to leftEncoderOut;
    connect rightEncoder.odometryOut to rightEncoderOut;
    connect bumperSwitches.hazardOut to bumperHazardOut;
  }

  part def AutonomousFloorCleaningRobot {
    part driveModule : DriveModule;
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/model.sysml").expect("uri");
        let ibd = build_ibd_for_uri(&graph, &uri);

        let view = ibd
            .root_views
            .get("driveModule")
            .expect("expected a driveModule root view");

        assert_eq!(
            view.connectors.len(),
            5,
            "expected all 5 connectors to resolve, got {:?}",
            view.connectors
        );
        for (source_suffix, target_suffix) in [
            ("leftMotor.phaseIn", "phaseLeftIn"),
            ("rightMotor.phaseIn", "phaseRightIn"),
            ("leftEncoder.odometryOut", "leftEncoderOut"),
            ("rightEncoder.odometryOut", "rightEncoderOut"),
            ("bumperSwitches.hazardOut", "bumperHazardOut"),
        ] {
            assert!(
                view.connectors.iter().any(|connector| {
                    connector.source_id.ends_with(source_suffix)
                        && connector.target_id.ends_with(target_suffix)
                }),
                "expected connector {source_suffix} -> {target_suffix}, got {:?}",
                view.connectors
            );
        }

        assert!(
            view.container_groups
                .iter()
                .all(|group| group.label != "PhysicalArchitecture"),
            "package PhysicalArchitecture should not render as a container box, got {:?}",
            view.container_groups
        );
        assert!(
            view.container_groups
                .iter()
                .any(|group| group.label == "AutonomousFloorCleaningRobot"),
            "expected the enclosing part-def container box to still render, got {:?}",
            view.container_groups
        );
        for group in &view.container_groups {
            if let Some(parent_id) = &group.parent_id {
                assert!(
                    view.container_groups.iter().any(|g| &g.id == parent_id),
                    "container group {:?} has a dangling parent_id {parent_id}",
                    group
                );
            }
        }
    }

    #[test]
    fn prune_interconnection_definition_parts_normalizes_reference_metadata() {
        let parts = vec![IbdPartDto {
            id: "PartsTree::sharedBranch".to_string(),
            node_id: "PartsTree.sharedBranch".to_string(),
            name: "sharedBranch".to_string(),
            qualified_name: "PartsTree.sharedBranch".to_string(),
            uri: None,
            container_id: Some("PartsTree.tree".to_string()),
            element_type: "ref".to_string(),
            attributes: HashMap::new(),
            range: None,
        }];
        let (parts, ports, connectors) =
            prune_interconnection_definition_parts(parts, Vec::new(), Vec::new());
        assert!(ports.is_empty());
        assert!(connectors.is_empty());
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].element_type, "ref");
        assert_eq!(
            parts[0].attributes.get("isReference"),
            Some(&serde_json::json!(true))
        );
        assert_eq!(
            parts[0].attributes.get("isDefinition"),
            Some(&serde_json::json!(false))
        );
    }
