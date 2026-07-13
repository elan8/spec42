    use super::extract_activity_diagrams;
    use sysml_v2_parser::parse;

    #[test]
    fn extract_activity_diagrams_exposes_in_out_as_interface_metadata() {
        let input = r#"
            package P {
                action def UpdateDisplay {
                    in currentTime : TimeValue;
                    out displayText : String;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "UpdateDisplay")
            .expect("diagram");

        assert!(
            diagram.actions.is_empty(),
            "interface declarations should not become flow steps"
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["currentTime".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["displayText".to_string()])
        );
        assert!(
            diagram.flows.is_empty(),
            "should not synthesize pseudo-flows"
        );
    }

    #[test]
    fn extract_activity_diagrams_includes_perform_steps() {
        let input = r#"
            package P {
                action def ExecuteMission {
                    in route : Route;
                    perform action captureVideo : CaptureVideo;
                    out report : MissionReport;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");
        let action_names: Vec<_> = diagram.actions.iter().map(|a| a.name.as_str()).collect();

        assert_eq!(action_names, vec!["captureVideo"]);
        assert_eq!(diagram.actions[0].kind.as_deref(), Some("perform"));
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["route".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["report".to_string()])
        );
        assert!(
            diagram.flows.is_empty(),
            "perform-only diagrams should not invent ordering edges"
        );
    }

    #[test]
    fn extract_activity_diagrams_includes_usage_bind_and_flows() {
        let input = r#"
            package P {
                action def ExecuteMission {
                    in route : Route;
                    action captureVideo : CaptureVideo;
                    bind route = captureVideo;
                    flow captureVideo to route;
                    first captureVideo then route;
                    merge route;
                    out report : MissionReport;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");

        assert!(
            diagram
                .actions
                .iter()
                .any(|a| a.name == "captureVideo" && a.kind.as_deref() == Some("action")),
            "expected action usage step to be emitted as a regular action node kind"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("bind")),
            "expected bind to be represented as a guarded flow edge"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("flow")),
            "expected flow statement edge"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("first")),
            "expected first/then edge"
        );
        assert!(
            diagram.states.iter().any(|s| s.state_type == "merge"),
            "expected merge node"
        );
    }

    #[test]
    fn extract_activity_diagrams_includes_decision_assign_and_for_loop() {
        let input = r#"
            package P {
                action def Route;
                action def Pipeline {
                    then action validate : Route;
                    action checkRoute : Decision;
                    then assign status := "ok";
                    for item in items {
                        then action validate : Route;
                    }
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "Pipeline")
            .expect("diagram");

        assert!(
            diagram
                .decisions
                .iter()
                .any(|decision| decision.name == "checkRoute"),
            "expected decision node"
        );
        assert!(
            diagram
                .states
                .iter()
                .any(|state| state.state_type == "assign" && state.name == "assign_status"),
            "expected assign state"
        );
        assert!(
            diagram
                .states
                .iter()
                .any(|state| state.state_type == "for-loop" && state.name == "for_item"),
            "expected for-loop state"
        );
    }

    #[test]
    fn extract_activity_diagrams_with_only_interface_have_no_behavior_nodes() {
        let input = r#"
            package P {
                action def ValidateRoute {
                    in route : Route;
                    out isValid : Boolean;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ValidateRoute")
            .expect("diagram");

        assert!(diagram.actions.is_empty());
        assert!(diagram.flows.is_empty());
        assert!(diagram.states.is_empty());
        assert!(diagram.interface.is_some());
    }

    #[test]
    fn extract_activity_diagrams_finds_action_defs_in_library_package() {
        let input = r#"
            standard library package P {
                action def ExecuteMission {
                    perform action captureVideo : CaptureVideo;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        assert!(
            diagrams.iter().any(|d| d.name == "ExecuteMission"),
            "expected action def inside library package to be discovered; diagrams: {:?}",
            diagrams.iter().map(|d| d.name.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn extract_activity_diagrams_synthesizes_nodes_referenced_by_first_then() {
        let input = r#"
            package P {
                action def ExecuteMission {
                    action validateRoute { out ok : Boolean; };
                    action startMission { out started : Boolean; };
                    first validateRoute then startMission;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");

        assert!(
            diagram.actions.iter().any(|a| a.name == "validateRoute"),
            "expected referenced step node validateRoute to exist"
        );
        assert!(
            diagram.actions.iter().any(|a| a.name == "startMission"),
            "expected referenced step node startMission to exist"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("first")
                    && f.from == "validateRoute"
                    && f.to == "startMission"),
            "expected first/then flow edge"
        );
    }

    #[test]
    fn extract_activity_diagrams_does_not_synthesize_interface_parameters_as_step_nodes() {
        let input = r#"
            package P {
                action def ExecutePatrol {
                    in route : String;
                    out status : String;

                    action finishMission { out missionStatus : String; };
                    bind status = finishMission::missionStatus;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecutePatrol")
            .expect("diagram");

        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["route".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["status".to_string()])
        );

        assert!(
            diagram
                .actions
                .iter()
                .all(|a| a.name != "route" && a.name != "status"),
            "interface parameters should not be synthesized into action nodes; actions={:?}",
            diagram
                .actions
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn extract_activity_diagrams_emits_performer_context_diagrams_from_part_bodies() {
        let input = r#"
            package Mission {
                part def FlightController {
                    perform action assessVehicleState : AssessVehicleState;
                    perform action manageMissionEvents : ManageMissionEvents;
                    perform action commandVehicle : CommandVehicle;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "FlightController" && d.source_kind == "performer")
            .expect("performer diagram");

        assert_eq!(diagram.package_path, "Mission");
        assert_eq!(diagram.id, "Mission::FlightController::performer");
        assert_eq!(diagram.actions.len(), 3);
        assert_eq!(diagram.flows.len(), 2);
        assert_eq!(diagram.flows[0].from, "assessVehicleState");
        assert_eq!(diagram.flows[0].to, "manageMissionEvents");
        assert_eq!(diagram.flows[1].from, "manageMissionEvents");
        assert_eq!(diagram.flows[1].to, "commandVehicle");
    }

    #[test]
    fn extract_activity_diagrams_include_package_metadata_for_action_defs() {
        let input = r#"
            package Mission {
                package Control {
                    action def ExecuteMission {
                        action assessVehicleState : AssessVehicleState;
                        action commandVehicle : CommandVehicle;
                        first assessVehicleState then commandVehicle;
                    }
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");

        assert_eq!(diagram.source_kind, "actionDef");
        assert_eq!(diagram.package_path, "Mission::Control");
        assert_eq!(diagram.id, "Mission::Control::ExecuteMission::actionDef");
    }

    #[test]
    fn extract_activity_diagrams_then_action_chain_adds_actions_and_flows() {
        let input = r#"
            package P {
                action def A;
                action def B;
                action def Pipeline {
                    then action step1 : A;
                    then action step2 : B;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|diagram| diagram.name == "Pipeline")
            .expect("Pipeline diagram");

        assert!(
            diagram.actions.iter().any(|action| action.name == "step1"),
            "expected step1 action from then action"
        );
        assert!(
            diagram.actions.iter().any(|action| action.name == "step2"),
            "expected step2 action from then action"
        );
        assert!(
            diagram.flows.iter().any(|flow| {
                flow.guard.as_deref() == Some("flow") && flow.from == "step1" && flow.to == "step2"
            }),
            "expected flow edge step1 -> step2; flows={:?}",
            diagram.flows
        );
    }
