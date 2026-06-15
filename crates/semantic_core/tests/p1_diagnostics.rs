//! P1 diagnostic regression tests.

use semantic_core::{build_graph_from_doc, collect_diagnostics_from_graph, DiagnosticsOptions};
use url::Url;

fn diags_for(input: &str) -> Vec<semantic_core::SemanticDiagnostic> {
    let parsed = sysml_v2_parser::parse(input).expect("parse");
    let uri = Url::parse("file:///p1.sysml").expect("uri");
    let graph = build_graph_from_doc(&parsed, &uri);
    collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default())
}

fn has_code(diags: &[semantic_core::SemanticDiagnostic], code: &str) -> bool {
    diags.iter().any(|d| d.code == code)
}

#[test]
fn emits_incompatible_type_kind_for_part_typed_as_port_def() {
    let input = r#"
        package P {
            port def CommandPort;
            part engine : CommandPort;
        }
    "#;
    let diags = diags_for(input);
    assert!(
        has_code(&diags, "incompatible_type_kind"),
        "expected incompatible_type_kind, got {:?}",
        diags.iter().map(|d| &d.code).collect::<Vec<_>>()
    );
}

#[test]
fn actor_typed_by_item_def_does_not_emit_incompatible_type_kind() {
    let input = r#"
        package P {
            item def Operator;
            use case def Patrol {
                subject drone;
                actor pilot : Operator;
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "incompatible_type_kind"),
        "actor typed by item def is valid per SysML §7.11.2 / §7.22.2, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "incompatible_type_kind")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn part_typed_by_item_def_does_not_emit_incompatible_type_kind() {
    let input = r#"
        package P {
            item def Person;
            part def Vehicle {
                part driver : Person;
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "incompatible_type_kind"),
        "part typed by item def is valid per SysML §7.11.2, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "incompatible_type_kind")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn subject_typed_by_resolved_analysis_def_does_not_emit_incompatible_type_kind() {
    let input = r#"
        package P {
            analysis def TradeStudy;
            verification def VerifyTrade {
                subject study : TradeStudy;
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "incompatible_type_kind"),
        "subject ReferenceUsage may reference any resolved Classifier per §8.3.6.3, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "incompatible_type_kind")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn private_wildcard_import_does_not_emit_visibility_violation() {
    let input = r#"
        package P {
            private import ScalarValues::*;
            part def Device {
                attribute mass : Real;
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "visibility_violation"),
        "private wildcard import for internal use must not warn, got {:?}",
        diags
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn action_defs_in_part_body_do_not_emit_duplicate_namespace_member() {
    let input = r#"
        package P {
            part def Navigation {
                action def DoNavigate;
                action def FindHome;
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "duplicate_namespace_member"),
        "action def siblings must not collide as name 'def', got {:?}",
        diags
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn requirement_def_id_dialect_does_not_emit_duplicate_namespace_member() {
    let input = r#"
        package P {
            package Requirements {
                requirement def id 'Req001' MaximaleMasse { doc /* x */ }
                requirement def id 'Req002' Lenken { }
                requirement def id 'Req003' Beschleunigen { }
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "duplicate_namespace_member"),
        "legacy requirement def id dialect must not collide as name 'def', got {:?}",
        diags
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn use_case_subject_and_then_action_same_name_do_not_emit_duplicate() {
    let input = r#"
        package P {
            part def Robot;
            action def DoClean;
            use case def Vacuming {
                subject roboticVacuumCleaner : Robot;
                first start;
                then action roboticVacuumCleaner : DoClean;
                then done;
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "duplicate_namespace_member"),
        "subject and then action with the same name are different feature kinds, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "duplicate_namespace_member")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn emits_duplicate_namespace_member() {
    let input = r#"
        package P {
            part def A;
            part def A;
        }
    "#;
    let diags = diags_for(input);
    assert!(has_code(&diags, "duplicate_namespace_member"));
}

#[test]
fn emits_attribute_value_type_mismatch_for_boolean_on_real() {
    let input = r#"
        package P {
            part def Device {
                part subsystem {
                    attribute ratedVoltage : Real = true;
                }
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(has_code(&diags, "attribute_value_type_mismatch"));
}

#[test]
fn part_feature_redefinition_does_not_emit_subset_kind_warnings() {
    let input = r#"
        package DutchGridProfile {
            part def DutchOperatorProfile {
                attribute operator : Integer;
                attribute defaultMvVoltageClass : Integer;
                attribute dominantMvVoltages : String;
            }
            part def DutchGridExpansionProject {
                part operatorProfile : DutchOperatorProfile;
            }
            part regionalExpansionProject : DutchGridExpansionProject {
                part :>> operatorProfile {
                    attribute :>> operator = 1;
                    attribute :>> defaultMvVoltageClass = 2;
                    attribute :>> dominantMvVoltages = "10 kV";
                }
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "incompatible_subset_redefine_kind"),
        "part :>> operatorProfile should be valid, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "incompatible_subset_redefine_kind")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
    assert!(
        !has_code(&diags, "unresolved_redefines_target"),
        "nested attribute redefines should resolve via DutchOperatorProfile, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "unresolved_redefines_target")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn emits_unresolved_redefines_target() {
    let input = r#"
        package P {
            part def Base { attribute mass : Real; }
            part def Child :> Base {
                attribute :>> notMass = 1;
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(has_code(&diags, "unresolved_redefines_target"));
}

#[test]
fn specialized_part_local_typed_attributes_do_not_emit_unresolved_redefines_target() {
    let input = r#"
        package Architecture {
            attribute def PowerValue :> Real;
            part def BaseSubsystem {
                attribute drivePowerW : PowerValue;
            }
            part def DriveSubsystem :> BaseSubsystem {
                attribute drivePowerW : PowerValue = 28;
            }
        }
    "#;
    let diags = diags_for(input);
    assert!(
        !has_code(&diags, "unresolved_redefines_target"),
        "local typed attributes without :>> must not be treated as redefines, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "unresolved_redefines_target")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
}
