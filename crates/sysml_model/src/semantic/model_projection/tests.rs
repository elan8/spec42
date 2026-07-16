
use super::*;
use crate::semantic::dto::{GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto};

fn range() -> RangeDto {
    RangeDto {
        start: PositionDto {
            line: 0,
            character: 0,
        },
        end: PositionDto {
            line: 0,
            character: 1,
        },
    }
}

#[test]
fn canonical_general_view_graph_preserves_subject_edges_for_retained_nodes() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Drone".to_string(),
                element_type: "part def".to_string(),
                name: "Drone".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Req".to_string(),
                element_type: "requirement def".to_string(),
                name: "Req".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Root".to_string(),
                element_type: "part def".to_string(),
                name: "Root".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
        ],
        edges: vec![
            GraphEdgeDto {
                source: "Pkg::Root".to_string(),
                target: "Pkg::Drone".to_string(),
                rel_type: "contains".to_string(),
                name: None,
            },
            GraphEdgeDto {
                source: "Pkg::Req".to_string(),
                target: "Pkg::Drone".to_string(),
                rel_type: "subject".to_string(),
                name: None,
            },
        ],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    assert!(
        canonical.nodes.iter().any(|node| node.id == "Pkg::Req"),
        "subject source node should be pulled into the canonical General View"
    );
    assert!(
        canonical.edges.iter().any(|edge| edge.rel_type == "subject"
            && edge.source == "Pkg::Req"
            && edge.target == "Pkg::Drone"),
        "subject edge should survive canonical General View projection"
    );
}

#[test]
fn canonical_general_view_graph_preserves_requirement_constraints_as_inline_attributes() {
    let graph = SysmlGraphDto {
        nodes: vec![GraphNodeDto {
            id: "Pkg::Req".to_string(),
            element_type: "requirement def".to_string(),
            name: "Req".to_string(),
            uri: None,
            parent_id: None,
            range: range(),
            attributes: serde_json::json!({
                "requirementConstraints": ["  flightTime >= 25 min."]
            })
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        }],
        edges: vec![],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    let owner = canonical
        .nodes
        .iter()
        .find(|node| node.id == "Pkg::Req")
        .unwrap();
    assert_eq!(
        owner.attributes.get("generalViewDirectAttributes"),
        Some(&serde_json::json!([{
            "name": "flightTime >= 25 min.",
            "displayText": "flightTime >= 25 min."
        }])),
        "requirement constraints should be exposed through generalViewDirectAttributes"
    );
}

#[test]
fn canonical_general_view_graph_filters_require_constraint_child_nodes() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Req".to_string(),
                element_type: "requirement def".to_string(),
                name: "Req".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: serde_json::json!({
                    "requirementConstraints": ["  flightTime >= 25 min."]
                })
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            },
            GraphNodeDto {
                id: "Pkg::Req::_requireConstraint_0".to_string(),
                element_type: "require constraint".to_string(),
                name: "_requireConstraint_0".to_string(),
                uri: None,
                parent_id: Some("Pkg::Req".to_string()),
                range: range(),
                attributes: Default::default(),
            },
        ],
        edges: vec![],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    assert_eq!(
        canonical.nodes.len(),
        1,
        "require constraint children should be filtered from General View"
    );
    let owner = &canonical.nodes[0];
    assert_eq!(owner.id, "Pkg::Req");
    assert_eq!(
        owner.attributes.get("generalViewDirectAttributes"),
        Some(&serde_json::json!([{
            "name": "flightTime >= 25 min.",
            "displayText": "flightTime >= 25 min."
        }])),
        "inline constraint summary should remain on the requirement owner"
    );
}

#[test]
fn strip_synthetic_nodes_removes_auto_expanded_instantiation_content() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Office::my_computer".to_string(),
                element_type: "part".to_string(),
                name: "my_computer".to_string(),
                uri: None,
                parent_id: Some("Office".to_string()),
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Office::my_computer::laptop".to_string(),
                element_type: "part".to_string(),
                name: "laptop".to_string(),
                uri: None,
                parent_id: Some("Office::my_computer".to_string()),
                range: range(),
                attributes: serde_json::json!({ "synthetic": true })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
            GraphNodeDto {
                id: "Office::Laptop".to_string(),
                element_type: "part def".to_string(),
                name: "Laptop".to_string(),
                uri: None,
                parent_id: Some("Office".to_string()),
                range: range(),
                attributes: Default::default(),
            },
        ],
        edges: vec![
            GraphEdgeDto {
                source: "Office::my_computer".to_string(),
                target: "Office::my_computer::laptop".to_string(),
                rel_type: "contains".to_string(),
                name: None,
            },
            GraphEdgeDto {
                source: "Office::my_computer::laptop".to_string(),
                target: "Office::Laptop".to_string(),
                rel_type: "typing".to_string(),
                name: None,
            },
        ],
    };

    let stripped = strip_synthetic_nodes(&graph);

    assert!(
        stripped
            .nodes
            .iter()
            .all(|node| node.id != "Office::my_computer::laptop"),
        "synthetic instantiation-expanded parts should be removed from general-view input"
    );
    assert!(
        stripped.edges.is_empty(),
        "edges touching synthetic instantiation-expanded content should be removed too: {:?}",
        stripped
            .edges
            .iter()
            .map(|edge| (&edge.source, &edge.target, &edge.rel_type))
            .collect::<Vec<_>>()
    );
}

#[test]
fn strip_synthetic_nodes_removes_builder_diagnostic_nodes() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "WebShopExample".to_string(),
                element_type: "package".to_string(),
                name: "WebShopExample".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "WebShopExample::unresolved_allocate_source".to_string(),
                element_type: "diagnostic".to_string(),
                name: "unresolved_allocate_source".to_string(),
                uri: None,
                parent_id: Some("WebShopExample".to_string()),
                range: range(),
                attributes: Default::default(),
            },
        ],
        edges: vec![GraphEdgeDto {
            source: "WebShopExample".to_string(),
            target: "WebShopExample::unresolved_allocate_source".to_string(),
            rel_type: "contains".to_string(),
            name: None,
        }],
    };

    let stripped = strip_synthetic_nodes(&graph);
    assert!(
        stripped
            .nodes
            .iter()
            .all(|n| n.element_type != "diagnostic"),
        "diagnostic nodes must not appear in model explorer graphs: {:?}",
        stripped.nodes
    );
    assert!(stripped.edges.is_empty());
}

#[test]
fn canonical_general_view_graph_inlines_ports_and_attributes_into_owner_nodes() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Laptop".to_string(),
                element_type: "part def".to_string(),
                name: "Laptop".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Laptop::voltage".to_string(),
                element_type: "attribute".to_string(),
                name: "voltage".to_string(),
                uri: None,
                parent_id: Some("Pkg::Laptop".to_string()),
                range: range(),
                attributes: serde_json::json!({ "dataType": "ScalarValues::Volt" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
            GraphNodeDto {
                id: "Pkg::Laptop::powerIn".to_string(),
                element_type: "port".to_string(),
                name: "powerIn".to_string(),
                uri: None,
                parent_id: Some("Pkg::Laptop".to_string()),
                range: range(),
                attributes: serde_json::json!({ "portType": "PowerPort" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
        ],
        edges: vec![
            GraphEdgeDto {
                source: "Pkg::Laptop".to_string(),
                target: "Pkg::Laptop::voltage".to_string(),
                rel_type: "contains".to_string(),
                name: None,
            },
            GraphEdgeDto {
                source: "Pkg::Laptop".to_string(),
                target: "Pkg::Laptop::powerIn".to_string(),
                rel_type: "contains".to_string(),
                name: None,
            },
        ],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    assert_eq!(
        canonical.nodes.len(),
        1,
        "port and attribute nodes should be filtered from General View"
    );
    let owner = canonical
        .nodes
        .iter()
        .find(|node| node.id == "Pkg::Laptop")
        .expect("owner node");
    assert_eq!(
        owner.attributes.get("generalViewDirectAttributes"),
        Some(&serde_json::json!([{
            "name": "voltage",
            "typeName": "Volt",
            "valueText": null,
            "declaredIn": null,
            "displayText": "voltage : Volt"
        }])),
        "attribute should be preserved in owner node compartments"
    );
    assert_eq!(
        owner.attributes.get("generalViewDirectPorts"),
        Some(&serde_json::json!([{
            "name": "powerIn",
            "typeName": "PowerPort",
            "valueText": null,
            "declaredIn": null,
            "displayText": "powerIn : PowerPort"
        }])),
        "port should be preserved in owner node compartments"
    );
    assert!(
        canonical.edges.is_empty(),
        "contains edges to inlined details should be removed from General View"
    );
}

#[test]
fn canonical_general_view_graph_display_text_includes_multiplicity_direction_and_redefines() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::DriveModule".to_string(),
                element_type: "part def".to_string(),
                name: "DriveModule".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::DriveModule::phaseLeftIn".to_string(),
                element_type: "port".to_string(),
                name: "phaseLeftIn".to_string(),
                uri: None,
                parent_id: Some("Pkg::DriveModule".to_string()),
                range: range(),
                attributes: serde_json::json!({
                    "portType": "ThreePhaseMotorPort",
                    "direction": "in",
                    "multiplicity": "1",
                })
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            },
            GraphNodeDto {
                id: "Pkg::DriveModule::cylinders".to_string(),
                element_type: "part".to_string(),
                name: "cylinders".to_string(),
                uri: None,
                parent_id: Some("Pkg::DriveModule".to_string()),
                range: range(),
                attributes: serde_json::json!({
                    "partType": "Cylinder",
                    "multiplicity": "4",
                    "redefines": "Engine::cylinders",
                })
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            },
        ],
        edges: vec![
            GraphEdgeDto {
                source: "Pkg::DriveModule".to_string(),
                target: "Pkg::DriveModule::phaseLeftIn".to_string(),
                rel_type: "contains".to_string(),
                name: None,
            },
            GraphEdgeDto {
                source: "Pkg::DriveModule".to_string(),
                target: "Pkg::DriveModule::cylinders".to_string(),
                rel_type: "contains".to_string(),
                name: None,
            },
        ],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    let owner = canonical
        .nodes
        .iter()
        .find(|node| node.id == "Pkg::DriveModule")
        .expect("owner node");
    assert_eq!(
        owner.attributes.get("generalViewDirectPorts"),
        Some(&serde_json::json!([{
            "name": "phaseLeftIn",
            "typeName": "ThreePhaseMotorPort",
            "valueText": null,
            "declaredIn": null,
            "displayText": "in phaseLeftIn [1] : ThreePhaseMotorPort"
        }])),
        "port row should show direction prefix and multiplicity bracket"
    );
    assert_eq!(
        owner.attributes.get("generalViewDirectParts"),
        Some(&serde_json::json!([{
            "name": "cylinders",
            "typeName": "Cylinder",
            "valueText": null,
            "declaredIn": null,
            "displayText": "cylinders [4] : Cylinder redefines cylinders"
        }])),
        "part row should show multiplicity bracket and redefines annotation"
    );
}

#[test]
fn canonical_general_view_graph_groups_direct_and_inherited_member_details() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Vehicle".to_string(),
                element_type: "part def".to_string(),
                name: "Vehicle".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Vehicle::mass".to_string(),
                element_type: "attribute".to_string(),
                name: "mass".to_string(),
                uri: None,
                parent_id: Some("Pkg::Vehicle".to_string()),
                range: range(),
                attributes:
                    serde_json::json!({ "dataType": "ScalarValues::Kilogram", "value": "1200" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
            },
            GraphNodeDto {
                id: "Pkg::Vehicle::engine".to_string(),
                element_type: "part".to_string(),
                name: "engine".to_string(),
                uri: None,
                parent_id: Some("Pkg::Vehicle".to_string()),
                range: range(),
                attributes: serde_json::json!({ "type": "Engine" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
            GraphNodeDto {
                id: "Pkg::Car".to_string(),
                element_type: "part def".to_string(),
                name: "Car".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Car::wheels".to_string(),
                element_type: "part".to_string(),
                name: "wheels".to_string(),
                uri: None,
                parent_id: Some("Pkg::Car".to_string()),
                range: range(),
                attributes: serde_json::json!({ "type": "WheelSet" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
            GraphNodeDto {
                id: "Pkg::Car::mass".to_string(),
                element_type: "attribute".to_string(),
                name: "mass".to_string(),
                uri: None,
                parent_id: Some("Pkg::Car".to_string()),
                range: range(),
                attributes:
                    serde_json::json!({ "dataType": "ScalarValues::Kilogram", "value": "1300" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
            },
        ],
        edges: vec![GraphEdgeDto {
            source: "Pkg::Car".to_string(),
            target: "Pkg::Vehicle".to_string(),
            rel_type: "specializes".to_string(),
            name: None,
        }],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    let owner = canonical
        .nodes
        .iter()
        .find(|node| node.id == "Pkg::Car")
        .expect("owner node");
    assert_eq!(
        owner.attributes.get("generalViewDirectAttributes"),
        Some(&serde_json::json!([{
            "name": "mass",
            "typeName": "Kilogram",
            "valueText": "1300",
            "declaredIn": null,
            "displayText": "mass : Kilogram = 1300"
        }])),
        "direct attributes should preserve type and value display text"
    );
    assert_eq!(
        owner.attributes.get("generalViewDirectParts"),
        Some(&serde_json::json!([{
            "name": "wheels",
            "typeName": "WheelSet",
            "valueText": null,
            "declaredIn": null,
            "displayText": "wheels : WheelSet"
        }])),
        "direct parts should remain in the owner node payload"
    );
    assert_eq!(
            owner.attributes.get("generalViewInheritedAttributes"),
            None,
            "redefined direct attributes should suppress inherited duplicates instead of emitting an empty compartment"
        );
    assert_eq!(
        owner.attributes.get("generalViewInheritedParts"),
        Some(&serde_json::json!([{
            "name": "engine",
            "typeName": "Engine",
            "valueText": null,
            "declaredIn": "Vehicle",
            "displayText": "engine : Engine"
        }])),
        "inherited parts should be grouped separately with provenance"
    );
}

#[test]
fn canonical_general_view_graph_moves_redefined_parts_into_direct_parts() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Stage".to_string(),
                element_type: "part def".to_string(),
                name: "Stage".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Stage::engine".to_string(),
                element_type: "part".to_string(),
                name: "engine".to_string(),
                uri: None,
                parent_id: Some("Pkg::Stage".to_string()),
                range: range(),
                attributes: serde_json::json!({ "type": "Engine" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
            GraphNodeDto {
                id: "Pkg::SIC".to_string(),
                element_type: "part def".to_string(),
                name: "S-IC".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::SIC::engine-redef".to_string(),
                element_type: "part".to_string(),
                name: "".to_string(),
                uri: None,
                parent_id: Some("Pkg::SIC".to_string()),
                range: range(),
                attributes: serde_json::json!({ "redefines": "Pkg::Stage::engine" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
        ],
        edges: vec![GraphEdgeDto {
            source: "Pkg::SIC".to_string(),
            target: "Pkg::Stage".to_string(),
            rel_type: "specializes".to_string(),
            name: None,
        }],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    let owner = canonical
        .nodes
        .iter()
        .find(|node| node.id == "Pkg::SIC")
        .expect("owner node");
    assert_eq!(
        owner.attributes.get("generalViewDirectParts"),
        Some(&serde_json::json!([{
            "name": "engine",
            "typeName": null,
            "valueText": null,
            "declaredIn": null,
            "displayText": "engine"
        }])),
        "redefined part rows should be surfaced as direct parts for the current owner"
    );
    assert_eq!(
        owner.attributes.get("generalViewInheritedParts"),
        None,
        "redefined inherited part should not stay in inherited compartment"
    );
}

#[test]
fn canonical_general_view_graph_moves_redefined_attributes_into_direct_attributes() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Stage".to_string(),
                element_type: "part def".to_string(),
                name: "Stage".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Stage::mass".to_string(),
                element_type: "attribute".to_string(),
                name: "mass".to_string(),
                uri: None,
                parent_id: Some("Pkg::Stage".to_string()),
                range: range(),
                attributes: serde_json::json!({ "dataType": "ScalarValues::Kilogram" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
            GraphNodeDto {
                id: "Pkg::SIC".to_string(),
                element_type: "part def".to_string(),
                name: "S-IC".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::SIC::mass-redef".to_string(),
                element_type: "attribute".to_string(),
                name: "".to_string(),
                uri: None,
                parent_id: Some("Pkg::SIC".to_string()),
                range: range(),
                attributes: serde_json::json!({
                    "redefines": "Pkg::Stage::mass",
                    "dataType": "ScalarValues::Kilogram",
                    "value": "28500"
                })
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            },
        ],
        edges: vec![GraphEdgeDto {
            source: "Pkg::SIC".to_string(),
            target: "Pkg::Stage".to_string(),
            rel_type: "specializes".to_string(),
            name: None,
        }],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    let owner = canonical
        .nodes
        .iter()
        .find(|node| node.id == "Pkg::SIC")
        .expect("owner node");
    assert_eq!(
        owner.attributes.get("generalViewDirectAttributes"),
        Some(&serde_json::json!([{
            "name": "mass",
            "typeName": "Kilogram",
            "valueText": "28500",
            "declaredIn": null,
            "displayText": "mass : Kilogram = 28500"
        }])),
        "redefined attribute rows should be surfaced as direct attributes for the current owner"
    );
    assert_eq!(
        owner.attributes.get("generalViewInheritedAttributes"),
        None,
        "redefined inherited attribute should not stay in inherited compartment"
    );
}

#[test]
fn canonical_general_view_graph_filters_parameter_nodes() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Operate".to_string(),
                element_type: "action def".to_string(),
                name: "Operate".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Operate::p".to_string(),
                element_type: "in out parameter".to_string(),
                name: "p".to_string(),
                uri: None,
                parent_id: Some("Pkg::Operate".to_string()),
                range: range(),
                attributes: serde_json::json!({ "parameterType": "Signal" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
        ],
        edges: vec![GraphEdgeDto {
            source: "Pkg::Operate".to_string(),
            target: "Pkg::Operate::p".to_string(),
            rel_type: "contains".to_string(),
            name: None,
        }],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    assert_eq!(
        canonical.nodes.len(),
        1,
        "parameter nodes should be filtered from General View"
    );
    assert!(
        canonical
            .nodes
            .iter()
            .all(|node| !node.element_type.to_lowercase().contains("parameter")),
        "parameter nodes should not remain in generalViewGraph"
    );
    assert!(
        canonical.edges.is_empty(),
        "contains edges to filtered parameter nodes should be removed too"
    );
}

#[test]
fn canonical_general_view_graph_filters_anonymous_redefinition_stubs() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Vehicle".to_string(),
                element_type: "part def".to_string(),
                name: "Vehicle".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::Vehicle::engines-redef".to_string(),
                element_type: "part".to_string(),
                name: "".to_string(),
                uri: None,
                parent_id: Some("Pkg::Vehicle".to_string()),
                range: range(),
                attributes: serde_json::json!({ "redefines": "engines" })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            },
        ],
        edges: vec![GraphEdgeDto {
            source: "Pkg::Vehicle".to_string(),
            target: "Pkg::Vehicle::engines-redef".to_string(),
            rel_type: "contains".to_string(),
            name: None,
        }],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    assert_eq!(
        canonical.nodes.len(),
        1,
        "anonymous redefinition stubs should not remain in General View"
    );
    assert!(
        canonical
            .nodes
            .iter()
            .all(|node| node.id != "Pkg::Vehicle::engines-redef"),
        "redefinition stub should be filtered out"
    );
    assert!(
        canonical.edges.is_empty(),
        "contains edges to redefinition stubs should be removed too"
    );
}

#[test]
fn canonical_general_view_graph_retains_def_usage_ref_nodes() {
    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Tree".to_string(),
                element_type: "part def".to_string(),
                name: "Tree".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::tree".to_string(),
                element_type: "part".to_string(),
                name: "tree".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
            GraphNodeDto {
                id: "Pkg::sharedBranch".to_string(),
                element_type: "ref".to_string(),
                name: "sharedBranch".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: Default::default(),
            },
        ],
        edges: vec![GraphEdgeDto {
            source: "Pkg::tree".to_string(),
            target: "Pkg::Tree".to_string(),
            rel_type: "typing".to_string(),
            name: None,
        }],
    };

    let canonical = canonical_general_view_graph(&graph, false);
    let types: Vec<_> = canonical
        .nodes
        .iter()
        .map(|node| node.element_type.as_str())
        .collect();
    assert!(types.iter().any(|t| t.contains("part def")));
    assert!(types.contains(&"part"));
    assert!(types.contains(&"ref"));
    assert!(
        canonical
            .edges
            .iter()
            .any(|edge| edge.rel_type.eq_ignore_ascii_case("typing")),
        "typing edge should survive canonicalization"
    );
}
