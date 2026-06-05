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
