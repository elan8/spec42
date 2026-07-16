
use super::{
    build_view_candidates, build_view_catalog, parse_filter_text, project_ids_for_renderer,
    EvaluatedView, FilterExpr,
};
use crate::semantic::dto::{GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto, SysmlGraphDto};
use crate::semantic::workspace_graph::WorkspaceParsedDocument;
use std::collections::{HashMap, HashSet};
use sysml_v2_parser::parse;
use url::Url;

#[test]
fn extracts_view_definitions_and_usages_with_filters_and_expose() {
    let uri = Url::parse("file:///C:/demo/model.sysml").expect("uri");
    let content = r#"
            package Demo {
                view def StructuralView {
                    filter @SysML::PartUsage and not @SysML::ConnectionUsage;
                }

                view VehicleView : StructuralView {
                    expose Demo::Vehicle::**[not @SysML::PortUsage];
                }
            }
        "#;
    let parsed = parse(content).expect("parse");
    let doc = WorkspaceParsedDocument {
        uri: uri.clone(),
        content: content.to_string(),
        parsed,
        parse_time_ms: 1,
        parse_cached: false,
    };

    let catalog = build_view_catalog(std::slice::from_ref(&uri), std::slice::from_ref(&doc));
    assert_eq!(catalog.definitions.len(), 1);
    assert_eq!(catalog.usages.len(), 1);
    assert_eq!(
        catalog.usages[0].definition_ref.as_deref(),
        Some("StructuralView")
    );
    assert_eq!(catalog.usages[0].exposes.len(), 1);
    assert!(catalog.usages[0].exposes[0].filter.is_some());
}

#[test]
fn parses_supported_filter_subset() {
    let parsed = parse_filter_text(
        "@SysML::PartUsage and not (@SysML::ConnectionUsage or @SysML::PortUsage)",
    );
    match parsed {
        FilterExpr::And(_, right) => match *right {
            FilterExpr::Not(_) => {}
            other => panic!("expected unary not, got {other:?}"),
        },
        other => panic!("expected conjunction, got {other:?}"),
    }
}

#[test]
fn includes_unsupported_view_types_in_candidates() {
    let evaluated_views = vec![
        EvaluatedView {
            id: "Demo::Supported".to_string(),
            name: "Supported".to_string(),
            effective_view_type: Some("GeneralView".to_string()),
            exposed_ids: HashSet::new(),
            conforms_to: Vec::new(),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        },
        EvaluatedView {
            id: "Demo::Safety".to_string(),
            name: "Safety".to_string(),
            effective_view_type: Some("SafetyView".to_string()),
            exposed_ids: HashSet::new(),
            conforms_to: Vec::new(),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        },
    ];

    let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates[0].name, "Safety");
    assert!(!candidates[0].supported);
    assert_eq!(candidates[0].renderer_view, None);
    assert_eq!(candidates[1].name, "Supported");
    assert!(candidates[1].supported);
    assert_eq!(candidates[1].renderer_view.as_deref(), Some("general-view"));
}

#[test]
fn sequence_view_type_maps_to_sequence_renderer() {
    let evaluated_views = vec![EvaluatedView {
        id: "Demo::CheckoutSequence".to_string(),
        name: "Checkout Sequence".to_string(),
        effective_view_type: Some("SequenceView".to_string()),
        exposed_ids: HashSet::new(),
        conforms_to: Vec::new(),
        filters: Vec::new(),
        visible_ids: HashSet::new(),
        issues: Vec::new(),
    }];

    let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
    assert_eq!(candidates.len(), 1);
    assert!(candidates[0].supported);
    assert_eq!(
        candidates[0].renderer_view.as_deref(),
        Some("sequence-view")
    );
}

#[test]
fn standard_view_types_map_to_shared_renderers() {
    let cases = [
        ("GeneralView", Some("general-view")),
        ("InterconnectionView", Some("interconnection-view")),
        ("ActionFlowView", Some("action-flow-view")),
        ("SequenceView", Some("sequence-view")),
        ("StateTransitionView", Some("state-transition-view")),
        ("BrowserView", Some("browser-view")),
        ("GridView", Some("grid-view")),
        ("GeometryView", Some("geometry-view")),
        ("RequirementView", None),
        ("CaseView", None),
        ("SafetyView", None),
    ];
    for (view_type, expected) in cases {
        assert_eq!(
            super::renderer_view_for_view_type(Some(view_type)),
            expected,
            "{view_type}"
        );
    }
}

#[test]
fn general_view_projection_expands_exposed_roots_to_owned_members() {
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

    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Office::OfficeDeskSetup".to_string(),
                element_type: "part def".to_string(),
                name: "OfficeDeskSetup".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Office::OfficeDeskSetup::laptop".to_string(),
                element_type: "part".to_string(),
                name: "laptop".to_string(),
                uri: None,
                parent_id: Some("Office::OfficeDeskSetup".to_string()),
                range: zero_range(),
                attributes: HashMap::new(),
            },
        ],
        edges: Vec::new(),
    };
    let evaluated = EvaluatedView {
        id: "Office::structure".to_string(),
        name: "structure".to_string(),
        effective_view_type: Some("GeneralView".to_string()),
        exposed_ids: HashSet::from(["Office::OfficeDeskSetup".to_string()]),
        conforms_to: Vec::new(),
        filters: Vec::new(),
        visible_ids: HashSet::new(),
        issues: Vec::new(),
    };

    let projected = project_ids_for_renderer(&evaluated, &graph, "general-view");
    assert!(projected.contains("Office::OfficeDeskSetup"));
    assert!(projected.contains("Office::OfficeDeskSetup::laptop"));
}

#[test]
fn structural_projection_recursively_expands_typed_part_definitions() {
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

    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::System".to_string(),
                element_type: "part def".to_string(),
                name: "System".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::System::engine".to_string(),
                element_type: "part".to_string(),
                name: "engine".to_string(),
                uri: None,
                parent_id: Some("Pkg::System".to_string()),
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::Engine".to_string(),
                element_type: "part def".to_string(),
                name: "Engine".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::Engine::pump".to_string(),
                element_type: "part".to_string(),
                name: "pump".to_string(),
                uri: None,
                parent_id: Some("Pkg::Engine".to_string()),
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::Pump".to_string(),
                element_type: "part def".to_string(),
                name: "Pump".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
        ],
        edges: vec![
            crate::semantic::dto::GraphEdgeDto {
                source: "Pkg::System::engine".to_string(),
                target: "Pkg::Engine".to_string(),
                rel_type: "typing".to_string(),
                name: None,
            },
            crate::semantic::dto::GraphEdgeDto {
                source: "Pkg::Engine::pump".to_string(),
                target: "Pkg::Pump".to_string(),
                rel_type: "typing".to_string(),
                name: None,
            },
        ],
    };
    let evaluated = EvaluatedView {
        id: "Pkg::view".to_string(),
        name: "view".to_string(),
        effective_view_type: Some("InterconnectionView".to_string()),
        exposed_ids: HashSet::from(["Pkg::System".to_string()]),
        conforms_to: Vec::new(),
        filters: Vec::new(),
        visible_ids: HashSet::new(),
        issues: Vec::new(),
    };

    let projected = project_ids_for_renderer(&evaluated, &graph, "interconnection-view");
    assert!(projected.contains("Pkg::System::engine"));
    assert!(projected.contains("Pkg::Engine"));
    assert!(projected.contains("Pkg::Engine::pump"));
    assert!(projected.contains("Pkg::Pump"));
}

#[test]
fn browser_view_projection_applies_expose_kind_filters_after_expansion() {
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

    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::Robot".to_string(),
                element_type: "part def".to_string(),
                name: "Robot".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::robot".to_string(),
                element_type: "part".to_string(),
                name: "robot".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::Robot::chassis".to_string(),
                element_type: "part".to_string(),
                name: "chassis".to_string(),
                uri: None,
                parent_id: Some("Pkg::Robot".to_string()),
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::Robot::powerPort".to_string(),
                element_type: "port".to_string(),
                name: "powerPort".to_string(),
                uri: None,
                parent_id: Some("Pkg::Robot".to_string()),
                range: zero_range(),
                attributes: HashMap::new(),
            },
        ],
        edges: vec![crate::semantic::dto::GraphEdgeDto {
            source: "Pkg::robot".to_string(),
            target: "Pkg::Robot".to_string(),
            rel_type: "typing".to_string(),
            name: None,
        }],
    };
    let evaluated = EvaluatedView {
        id: "Pkg::structure".to_string(),
        name: "structure".to_string(),
        effective_view_type: Some("BrowserView".to_string()),
        exposed_ids: HashSet::from(["Pkg::robot".to_string()]),
        conforms_to: Vec::new(),
        filters: vec![FilterExpr::Matches("@SysML::PartUsage".to_string())],
        visible_ids: HashSet::new(),
        issues: Vec::new(),
    };

    let projected = project_ids_for_renderer(&evaluated, &graph, "browser-view");
    assert!(projected.contains("Pkg::robot"));
    assert!(projected.contains("Pkg::Robot::chassis"));
    assert!(
        !projected.contains("Pkg::Robot::powerPort"),
        "PartUsage filter should exclude ports after expansion"
    );
}

#[test]
fn requirement_view_projection_follows_traceability_links_without_structural_expansion() {
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

    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::need".to_string(),
                element_type: "requirement".to_string(),
                name: "need".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::req".to_string(),
                element_type: "requirement".to_string(),
                name: "req".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::design".to_string(),
                element_type: "action".to_string(),
                name: "design".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::unrelatedPart".to_string(),
                element_type: "part".to_string(),
                name: "unrelatedPart".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
        ],
        edges: vec![
            GraphEdgeDto {
                source: "Pkg::need".to_string(),
                target: "Pkg::req".to_string(),
                rel_type: "derivation".to_string(),
                name: None,
            },
            GraphEdgeDto {
                source: "Pkg::design".to_string(),
                target: "Pkg::req".to_string(),
                rel_type: "satisfy".to_string(),
                name: None,
            },
        ],
    };
    let evaluated = EvaluatedView {
        id: "Pkg::trace".to_string(),
        name: "trace".to_string(),
        effective_view_type: Some("GeneralView".to_string()),
        exposed_ids: HashSet::from(["Pkg::need".to_string(), "Pkg::design".to_string()]),
        conforms_to: Vec::new(),
        filters: vec![FilterExpr::Matches("@SysML::RequirementUsage".to_string())],
        visible_ids: HashSet::new(),
        issues: Vec::new(),
    };

    let projected = project_ids_for_renderer(&evaluated, &graph, "general-view");
    assert!(projected.contains("Pkg::need"));
    assert!(projected.contains("Pkg::req"));
    assert!(projected.contains("Pkg::design"));
    assert!(
        !projected.contains("Pkg::unrelatedPart"),
        "traceability projection should not structurally expand unrelated elements"
    );
}

#[test]
fn extracts_rendering_from_view_usage() {
    let uri = Url::parse("file:///C:/demo/model.sysml").expect("uri");
    let content = r#"
            package Demo {
                part def System { part child; }
                part system : System;
                view connections {
                    expose Demo::system;
                    render asInterconnectionDiagram;
                }
            }
        "#;
    let parsed = parse(content).expect("parse");
    let doc = WorkspaceParsedDocument {
        uri: uri.clone(),
        content: content.to_string(),
        parsed,
        parse_time_ms: 1,
        parse_cached: false,
    };

    let catalog = build_view_catalog(std::slice::from_ref(&uri), std::slice::from_ref(&doc));
    assert_eq!(catalog.usages.len(), 1);
    assert_eq!(
        catalog.usages[0].rendering_ref.as_deref(),
        Some("asInterconnectionDiagram")
    );
}

#[test]
fn stdlib_rendering_maps_to_view_type_and_renderer() {
    assert_eq!(
        super::view_type_for_stdlib_rendering(Some("asInterconnectionDiagram"), None),
        Some("InterconnectionView")
    );
    assert_eq!(
        super::renderer_view_for_view_type(Some("InterconnectionView")),
        Some("interconnection-view")
    );
    assert_eq!(
        super::view_type_for_stdlib_rendering(Some("asTreeDiagram"), None),
        Some("BrowserView")
    );
    assert_eq!(
        super::view_type_for_stdlib_rendering(Some("asElementTable"), None),
        Some("GridView")
    );
}

#[test]
fn rendering_only_view_is_supported_candidate() {
    let evaluated_views = vec![EvaluatedView {
        id: "Demo::connections".to_string(),
        name: "connections".to_string(),
        effective_view_type: Some("InterconnectionView".to_string()),
        exposed_ids: HashSet::new(),
        conforms_to: Vec::new(),
        filters: Vec::new(),
        visible_ids: HashSet::new(),
        issues: Vec::new(),
    }];

    let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
    assert!(candidates[0].supported);
    assert_eq!(
        candidates[0].renderer_view.as_deref(),
        Some("interconnection-view")
    );
}

#[test]
fn untyped_view_without_render_falls_back_to_general_view_candidate() {
    let evaluated_views = vec![EvaluatedView {
        id: "Demo::overview".to_string(),
        name: "overview".to_string(),
        effective_view_type: Some("GeneralView".to_string()),
        exposed_ids: HashSet::new(),
        conforms_to: Vec::new(),
        filters: Vec::new(),
        visible_ids: HashSet::new(),
        issues: Vec::new(),
    }];

    let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
    assert!(candidates[0].supported);
    assert_eq!(candidates[0].renderer_view.as_deref(), Some("general-view"));
}

#[test]
fn state_transition_projection_expands_exposed_machine_descendants() {
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

    let graph = SysmlGraphDto {
        nodes: vec![
            GraphNodeDto {
                id: "Pkg::OrderLifecycle".to_string(),
                element_type: "state def".to_string(),
                name: "OrderLifecycle".to_string(),
                uri: None,
                parent_id: None,
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::OrderLifecycle::created".to_string(),
                element_type: "state".to_string(),
                name: "created".to_string(),
                uri: None,
                parent_id: Some("Pkg::OrderLifecycle".to_string()),
                range: zero_range(),
                attributes: HashMap::new(),
            },
            GraphNodeDto {
                id: "Pkg::OrderLifecycle::paid".to_string(),
                element_type: "state".to_string(),
                name: "paid".to_string(),
                uri: None,
                parent_id: Some("Pkg::OrderLifecycle".to_string()),
                range: zero_range(),
                attributes: HashMap::new(),
            },
        ],
        edges: vec![crate::semantic::dto::GraphEdgeDto {
            source: "Pkg::OrderLifecycle::created".to_string(),
            target: "Pkg::OrderLifecycle::paid".to_string(),
            rel_type: "transition".to_string(),
            name: Some("to_paid".to_string()),
        }],
    };
    let evaluated = EvaluatedView {
        id: "Pkg::orderLifecycle".to_string(),
        name: "orderLifecycle".to_string(),
        effective_view_type: Some("StateTransitionView".to_string()),
        exposed_ids: HashSet::from(["Pkg::OrderLifecycle".to_string()]),
        conforms_to: Vec::new(),
        filters: Vec::new(),
        visible_ids: HashSet::new(),
        issues: Vec::new(),
    };

    let projected = project_ids_for_renderer(&evaluated, &graph, "state-transition-view");
    assert!(projected.contains("Pkg::OrderLifecycle"));
    assert!(projected.contains("Pkg::OrderLifecycle::created"));
    assert!(projected.contains("Pkg::OrderLifecycle::paid"));
}
