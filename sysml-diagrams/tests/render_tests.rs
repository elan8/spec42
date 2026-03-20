use sysml_diagrams::{
    general_view, interconnection_view, GraphEdgeInput, GraphNodeInput, IbdConnectorInput,
    IbdInput, IbdPartInput, IbdPortInput, RangeInput,
};

#[test]
fn general_view_renders_svg() {
    let nodes = vec![
        GraphNodeInput {
            id: "Pkg".to_string(),
            element_type: "package".to_string(),
            name: "Pkg".to_string(),
            parent_id: None,
            range: sample_range(),
            attributes: vec![("visibility".to_string(), "public".to_string())],
        },
        GraphNodeInput {
            id: "Pkg::Drone".to_string(),
            element_type: "part def".to_string(),
            name: "Drone".to_string(),
            parent_id: Some("Pkg".to_string()),
            range: sample_range(),
            attributes: vec![("partType".to_string(), "Vehicle".to_string())],
        },
    ];
    let edges = vec![GraphEdgeInput {
        source: "Pkg::Drone".to_string(),
        target: "Pkg".to_string(),
        rel_type: "typing".to_string(),
        name: Some("typing".to_string()),
    }];
    let rendered = general_view::render(&nodes, &edges).expect("render");
    assert!(rendered.svg.contains("general-view"));
    assert!(rendered.metrics.node_count >= 1);
}

#[test]
fn general_view_keeps_top_level_instance_without_expanding_instance_parts() {
    let nodes = vec![
        GraphNodeInput {
            id: "Pkg".to_string(),
            element_type: "package".to_string(),
            name: "Pkg".to_string(),
            parent_id: None,
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::Drone".to_string(),
            element_type: "part def".to_string(),
            name: "Drone".to_string(),
            parent_id: Some("Pkg".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::Drone::airframe".to_string(),
            element_type: "part".to_string(),
            name: "airframe".to_string(),
            parent_id: Some("Pkg::Drone".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::Airframe".to_string(),
            element_type: "part def".to_string(),
            name: "Airframe".to_string(),
            parent_id: Some("Pkg".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::droneInstance".to_string(),
            element_type: "part".to_string(),
            name: "droneInstance".to_string(),
            parent_id: Some("Pkg".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::droneInstance::airframe".to_string(),
            element_type: "part".to_string(),
            name: "airframe".to_string(),
            parent_id: Some("Pkg::droneInstance".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
    ];
    let edges = vec![
        GraphEdgeInput {
            source: "Pkg::Drone".to_string(),
            target: "Pkg::Drone::airframe".to_string(),
            rel_type: "contains".to_string(),
            name: Some("contains".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::Drone::airframe".to_string(),
            target: "Pkg::Airframe".to_string(),
            rel_type: "typing".to_string(),
            name: Some("typing".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::droneInstance".to_string(),
            target: "Pkg::Drone".to_string(),
            rel_type: "typing".to_string(),
            name: Some("instance of".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::droneInstance".to_string(),
            target: "Pkg::droneInstance::airframe".to_string(),
            rel_type: "contains".to_string(),
            name: Some("contains".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::droneInstance::airframe".to_string(),
            target: "Pkg::Airframe".to_string(),
            rel_type: "typing".to_string(),
            name: Some("typing".to_string()),
        },
    ];

    let rendered = general_view::render(&nodes, &edges).expect("render");

    assert!(rendered.svg.contains("data-element-id=\"Pkg::droneInstance\""));
    assert!(rendered.svg.contains("data-element-id=\"Pkg::Drone::airframe\""));
    assert!(!rendered
        .svg
        .contains("data-element-id=\"Pkg::droneInstance::airframe\""));
}

#[test]
fn general_view_keeps_typed_part_usage_without_expanding_typed_part_subtree() {
    let nodes = vec![
        GraphNodeInput {
            id: "Pkg".to_string(),
            element_type: "package".to_string(),
            name: "Pkg".to_string(),
            parent_id: None,
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::Power".to_string(),
            element_type: "part def".to_string(),
            name: "Power".to_string(),
            parent_id: Some("Pkg".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::Power::distribution".to_string(),
            element_type: "part".to_string(),
            name: "distribution".to_string(),
            parent_id: Some("Pkg::Power".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::PowerDistribution".to_string(),
            element_type: "part def".to_string(),
            name: "PowerDistribution".to_string(),
            parent_id: Some("Pkg".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::Drone".to_string(),
            element_type: "part def".to_string(),
            name: "Drone".to_string(),
            parent_id: Some("Pkg".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::Drone::power".to_string(),
            element_type: "part".to_string(),
            name: "power".to_string(),
            parent_id: Some("Pkg::Drone".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
        GraphNodeInput {
            id: "Pkg::Drone::power::distribution".to_string(),
            element_type: "part".to_string(),
            name: "distribution".to_string(),
            parent_id: Some("Pkg::Drone::power".to_string()),
            range: sample_range(),
            attributes: vec![],
        },
    ];
    let edges = vec![
        GraphEdgeInput {
            source: "Pkg::Power".to_string(),
            target: "Pkg::Power::distribution".to_string(),
            rel_type: "contains".to_string(),
            name: Some("contains".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::Power::distribution".to_string(),
            target: "Pkg::PowerDistribution".to_string(),
            rel_type: "typing".to_string(),
            name: Some("typing".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::Drone".to_string(),
            target: "Pkg::Drone::power".to_string(),
            rel_type: "contains".to_string(),
            name: Some("contains".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::Drone::power".to_string(),
            target: "Pkg::Power".to_string(),
            rel_type: "typing".to_string(),
            name: Some("typing".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::Drone::power".to_string(),
            target: "Pkg::Drone::power::distribution".to_string(),
            rel_type: "contains".to_string(),
            name: Some("contains".to_string()),
        },
        GraphEdgeInput {
            source: "Pkg::Drone::power::distribution".to_string(),
            target: "Pkg::PowerDistribution".to_string(),
            rel_type: "typing".to_string(),
            name: Some("typing".to_string()),
        },
    ];

    let rendered = general_view::render(&nodes, &edges).expect("render");

    assert!(rendered.svg.contains("data-element-id=\"Pkg::Drone::power\""));
    assert!(rendered.svg.contains("data-element-id=\"Pkg::Power::distribution\""));
    assert!(!rendered
        .svg
        .contains("data-element-id=\"Pkg::Drone::power::distribution\""));
}

#[test]
fn interconnection_view_renders_svg() {
    let ibd = IbdInput {
        parts: vec![
            IbdPartInput {
                id: "root".to_string(),
                name: "root".to_string(),
                qualified_name: "Root".to_string(),
                container_id: None,
                element_type: "part".to_string(),
                attributes: vec![],
            },
            IbdPartInput {
                id: "child".to_string(),
                name: "sensor".to_string(),
                qualified_name: "Root.sensor".to_string(),
                container_id: Some("Root".to_string()),
                element_type: "part".to_string(),
                attributes: vec![],
            },
        ],
        ports: vec![
            IbdPortInput {
                id: "Root.sensor.out".to_string(),
                name: "out".to_string(),
                parent_id: "Root.sensor".to_string(),
                direction: Some("out".to_string()),
                port_type: None,
                port_side: Some("right".to_string()),
            },
            IbdPortInput {
                id: "Root.in".to_string(),
                name: "in".to_string(),
                parent_id: "Root".to_string(),
                direction: Some("in".to_string()),
                port_type: None,
                port_side: Some("left".to_string()),
            },
        ],
        connectors: vec![IbdConnectorInput {
            source: "out".to_string(),
            target: "in".to_string(),
            source_id: "Root.sensor.out".to_string(),
            target_id: "Root.in".to_string(),
            rel_type: "flow".to_string(),
        }],
        root_candidates: vec!["Root".to_string()],
        default_root: Some("Root".to_string()),
    };
    let rendered = interconnection_view::render(&ibd).expect("render");
    assert!(rendered.svg.contains("interconnection-view"));
    assert!(rendered.metrics.edge_count >= 1);
}

#[test]
fn interconnection_view_omits_internal_attributes_from_properties() {
    let ibd = IbdInput {
        parts: vec![
            IbdPartInput {
                id: "root".to_string(),
                name: "root".to_string(),
                qualified_name: "Root".to_string(),
                container_id: None,
                element_type: "part".to_string(),
                attributes: vec![],
            },
            IbdPartInput {
                id: "child".to_string(),
                name: "sensor".to_string(),
                qualified_name: "Root.sensor".to_string(),
                container_id: Some("Root".to_string()),
                element_type: "part".to_string(),
                attributes: vec![
                    ("synthetic".to_string(), "true".to_string()),
                    ("originRange".to_string(), "{\"line\":1}".to_string()),
                    ("voltage".to_string(), "5V".to_string()),
                ],
            },
        ],
        ports: vec![],
        connectors: vec![],
        root_candidates: vec!["Root".to_string()],
        default_root: Some("Root".to_string()),
    };
    let rendered = interconnection_view::render(&ibd).expect("render");
    assert!(rendered.svg.contains("voltage: 5V"));
    assert!(!rendered.svg.contains("synthetic"));
    assert!(!rendered.svg.contains("originRange"));
}

fn sample_range() -> RangeInput {
    RangeInput {
        start_line: 0,
        start_character: 0,
        end_line: 0,
        end_character: 1,
    }
}
