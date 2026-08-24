//! Batch validation diagnostics: what `workspace::validate_paths` reports for a workspace.
//!
//! These exercise the batch path end to end — a temp workspace on disk, one publication, and the
//! neutral diagnostics the reporting policy settles. They lived in the editor host's integration
//! suite while the validation pipeline did; both belong to the batch host.

use std::fs;
use std::path::PathBuf;

use sysml_diagnostics::SemanticDiagnostic;
use workspace::{validate_paths, EngineBuilder, Spec42Engine, ValidationRequest};

fn test_engine(cache: &tempfile::TempDir, library_paths: Vec<PathBuf>) -> Spec42Engine {
    EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .library_paths(library_paths)
        .build()
        .expect("engine")
}

fn validate_inline_sysml(filename: &str, content: &str) -> Vec<SemanticDiagnostic> {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let cache = tempfile::tempdir().expect("cache dir");
    let file_path = temp_dir.path().join(filename);
    fs::write(&file_path, content).expect("write sysml fixture");
    let engine = test_engine(&cache, Vec::new());
    let report = validate_paths(
        &engine,
        &[],
        ValidationRequest {
            targets: vec![file_path.clone()],
            workspace_root: Some(temp_dir.path().to_path_buf()),
            library_paths: Vec::new(),
            parallel_enabled: false,
            strict_diagnostics: false,
        },
    )
    .expect("validate paths");
    report
        .documents
        .iter()
        .find(|document| document.uri.ends_with(&filename.replace('\\', "/")))
        .map(|document| document.diagnostics.clone())
        .or_else(|| {
            report
                .documents
                .first()
                .map(|document| document.diagnostics.clone())
        })
        .expect("validated document diagnostics")
}

fn has_diag_code(diagnostics: &[SemanticDiagnostic], source: &str, code: &str) -> bool {
    diagnostics
        .iter()
        .any(|diagnostic| diagnostic.source == source && diagnostic.code == code)
}

fn diagnostic_range_text(content: &str, diagnostic: &SemanticDiagnostic) -> String {
    let line = content
        .lines()
        .nth(diagnostic.range.start.line as usize)
        .expect("diagnostic line");
    line.chars()
        .skip(diagnostic.range.start.character as usize)
        .take((diagnostic.range.end.character - diagnostic.range.start.character) as usize)
        .collect()
}

fn diagnostic_by_code<'a>(
    diagnostics: &'a [SemanticDiagnostic],
    source: &str,
    code: &str,
) -> Option<&'a SemanticDiagnostic> {
    diagnostics
        .iter()
        .find(|diagnostic| diagnostic.source == source && diagnostic.code == code)
}

#[test]
fn lsp_diagnostics_on_invalid_sysml() {
    // Use invalid input that parse_with_diagnostics reports (extra closing brace).
    let content = "package P { } }";
    let diagnostics = validate_inline_sysml("bad.sysml", content);
    let got_diagnostics = !diagnostics.is_empty();
    assert!(
        got_diagnostics,
        "invalid SysML should produce at least one diagnostic"
    );
}

#[test]
fn surveillance_drone_semantic_diagnostics_have_meaningful_ranges() {
    let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("fixtures")
        .join("surveillance_drone_full.sysml");
    let content = fs::read_to_string(&fixture_path).expect("read drone fixture");
    let diagnostics = validate_inline_sysml("surveillance_drone_diag_test.sysml", &content);
    let semantic_diags: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.source == "semantic")
        .collect();

    // With workspace-wide linking and typing-only materialization, this fixture may
    // now fully resolve and produce zero semantic diagnostics.
    // Keep validating range quality and unresolved-reference invariants on whatever
    // semantic diagnostics are emitted.
    let at_1_1 = semantic_diags
        .iter()
        .filter(|diagnostic| {
            diagnostic.range.start.line == 0
                && diagnostic.range.start.character == 0
                && diagnostic.range.end.line == 0
                && diagnostic.range.end.character == 0
        })
        .count();
    assert_eq!(
        at_1_1, 0,
        "expected semantic diagnostics to avoid line1/col1 sentinel ranges"
    );

    let unconnected_count = semantic_diags
        .iter()
        .filter(|diagnostic| diagnostic.code == "unconnected_port")
        .count();
    assert!(
        unconnected_count <= 25,
        "expected reduced unconnected_port noise, got {unconnected_count}"
    );

    let duplicate_connection_count = semantic_diags
        .iter()
        .filter(|diagnostic| diagnostic.code == "duplicate_connection")
        .count();
    assert_eq!(
        duplicate_connection_count, 0,
        "fan-out to distinct usage ports must not be reported as duplicate_connection: {semantic_diags:#?}"
    );

    let unresolved: Vec<_> = semantic_diags
        .iter()
        .filter(|diagnostic| diagnostic.code == "unresolved_type_reference")
        .collect();

    let unresolved_string = unresolved
        .iter()
        .filter(|d| d.message.contains("Type reference 'String'"))
        .count();
    assert_eq!(
        unresolved_string, 0,
        "expected String to be treated as built-in; got unresolved String diagnostics: {unresolved:#?}"
    );

    let unresolved_conjugated = unresolved
        .iter()
        .filter(|d| d.message.contains("Type reference '~"))
        .count();
    assert_eq!(
        unresolved_conjugated, 0,
        "expected no unresolved diagnostics for conjugated type refs; got: {unresolved:#?}"
    );

    let unresolved_behavior_actions = unresolved
        .iter()
        .filter(|d| {
            let msg = d.message.as_str();
            msg.contains("Type reference 'ExecutePatrol'")
                || msg.contains("Type reference 'ExecuteOrbit'")
                || msg.contains("Type reference 'ControlGimbal'")
                || msg.contains("Type reference 'CaptureVideo'")
        })
        .count();
    assert_eq!(
        unresolved_behavior_actions, 0,
        "expected action usages to resolve to local action defs; got: {unresolved:#?}"
    );

    let mut unresolved_ranges_to_type_refs: std::collections::HashMap<
        String,
        std::collections::HashSet<String>,
    > = std::collections::HashMap::new();
    for diag in &unresolved {
        let msg = diag.message.as_str();
        let type_ref = msg
            .split("Type reference '")
            .nth(1)
            .and_then(|rest| rest.split('\'').next())
            .unwrap_or_default()
            .to_string();
        let range_key = format!(
            "{}:{}:{}:{}",
            diag.range.start.line,
            diag.range.start.character,
            diag.range.end.line,
            diag.range.end.character
        );
        unresolved_ranges_to_type_refs
            .entry(range_key)
            .or_default()
            .insert(type_ref);
    }
    let conflicting_anchor_count = unresolved_ranges_to_type_refs
        .values()
        .filter(|type_refs| type_refs.len() > 1)
        .count();
    assert_eq!(
        conflicting_anchor_count, 0,
        "expected unresolved diagnostics to have stable anchors (no unrelated type refs sharing one range): {:?}",
        unresolved_ranges_to_type_refs
    );
}

#[test]
fn unresolved_type_reference_emits_semantic_diagnostic() {
    let content = r#"
        package P {
            part def Vehicle {
                part engine : MissingEngineType;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("missing_type.sysml", content);
    let diagnostic = diagnostic_by_code(&diagnostics, "semantic", "unresolved_type_reference")
        .expect("expected unresolved_type_reference semantic diagnostic");
    assert_eq!(
        diagnostic_range_text(content, diagnostic),
        "MissingEngineType"
    );
}

#[test]
fn unresolved_ref_type_reference_emits_semantic_diagnostic() {
    let content = r#"
        package P {
            part def OrbitContext {
                ref centralBody : MissingCelestialBody;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("missing_ref_type.sysml", content);
    // A `ref` usage's type reference is settled like any other typing, so it reports the same
    // code. The publication does not carry a separate outcome for the `ref` spelling.
    let diagnostic = diagnostic_by_code(&diagnostics, "semantic", "unresolved_type_reference")
        .expect("expected unresolved_type_reference semantic diagnostic");
    assert_eq!(
        diagnostic_range_text(content, diagnostic),
        "MissingCelestialBody"
    );
}

#[test]
fn unresolved_viewpoint_conformance_target_emits_semantic_diagnostic() {
    let content = r#"
        package P {
            view def StructuralView;
            view structure : StructuralView;
            satisfy structure by MissingViewpoint;
        }
    "#;
    let diagnostics = validate_inline_sysml("missing_viewpoint_conformance_target.sysml", content);
    // A satisfy endpoint that names nothing is an unresolved authored reference, reported at the
    // endpoint. The publication has no viewpoint-specific unresolved outcome.
    let diagnostic = diagnostic_by_code(&diagnostics, "semantic", "unresolved_reference")
        .expect("expected an unresolved reference for the missing viewpoint");
    assert_eq!(
        diagnostic_range_text(content, diagnostic),
        "MissingViewpoint"
    );
}

#[test]
fn non_viewpoint_target_for_view_conformance_emits_semantic_diagnostic() {
    let content = r#"
        package P {
            requirement def RequirementTarget;
            view def StructuralView;
            view structure : StructuralView;
            satisfy structure by RequirementTarget;
        }
    "#;
    let diagnostics = validate_inline_sysml("invalid_viewpoint_conformance_target.sysml", content);
    let found = has_diag_code(
        &diagnostics,
        "semantic",
        "viewpoint_conformance_invalid_target_kind",
    );
    assert!(
        found,
        "expected viewpoint_conformance_invalid_target_kind semantic diagnostic"
    );
}

#[test]
fn missing_library_context_info_is_emitted_for_imported_unresolved_types_without_library_paths() {
    let content = r#"
        package P {
            import ScalarValues::Real;

            part def Vehicle {
                attribute mass : Real;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("missing_library_context.sysml", content);
    let found_missing_library_context =
        has_diag_code(&diagnostics, "semantic", "missing_library_context");
    let found_unresolved = has_diag_code(&diagnostics, "semantic", "unresolved_type_reference");

    assert!(
        found_unresolved,
        "expected unresolved_type_reference semantic diagnostic"
    );
    assert!(
        found_missing_library_context,
        "expected missing_library_context informational diagnostic"
    );
}

#[test]
fn missing_library_context_info_is_emitted_for_unresolved_import_targets_without_library_paths() {
    let content = r#"
        package P {
            import MissingLibrary::*;
        }
    "#;
    let diagnostics = validate_inline_sysml("missing_import_target_context.sysml", content);
    let found_missing_library_context =
        has_diag_code(&diagnostics, "semantic", "missing_library_context");
    let found_unresolved_import =
        has_diag_code(&diagnostics, "semantic", "unresolved_import_target");

    assert!(
        found_unresolved_import,
        "expected unresolved_import_target semantic diagnostic"
    );
    assert!(
        found_missing_library_context,
        "expected missing_library_context informational diagnostic"
    );
}

#[test]
fn unresolved_specializes_reference_is_emitted_for_imported_missing_bases() {
    let content = r#"
        package P {
            import RoboticsCore::*;
            part def InspectionRover :> RobotPlatform {
                attribute robotName = "inspection-rover";
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("missing_specializes_base.sysml", content);
    let found_unresolved_specializes = diagnostics.iter().any(|diagnostic| {
        diagnostic.source == "semantic" && diagnostic.code == "unresolved_specializes_reference"
    });

    assert!(
        found_unresolved_specializes,
        "expected unresolved_specializes_reference semantic diagnostic"
    );
}

#[test]
fn unresolved_specializes_reference_is_not_emitted_when_base_resolves() {
    let content = r#"
        package P {
            part def RobotPlatform {}
            part def InspectionRover :> RobotPlatform {
                attribute robotName = "inspection-rover";
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("resolved_specializes_base.sysml", content);
    let found_unresolved_specializes = diagnostics.iter().any(|diagnostic| {
        diagnostic.source == "semantic" && diagnostic.code == "unresolved_specializes_reference"
    });

    assert!(
        !found_unresolved_specializes,
        "did not expect unresolved_specializes_reference when base resolves"
    );
}

#[test]
fn analysis_usage_typed_by_imported_analysis_def_does_not_emit_unresolved_type_reference() {
    let content = r#"
        package GridAnalysis {
            analysis def LoadFlowAnalysis {
                return ref loadFlowComplete {
                    return true;
                }
            }
        }
        package AnalysisCases {
            private import GridAnalysis::*;
            analysis loadFlowRun : LoadFlowAnalysis;
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_usage_typing.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "unresolved_type_reference"),
        "expected imported analysis def typing to resolve, got: {diagnostics:#?}"
    );
}

#[test]
fn unresolved_specializes_reference_is_not_emitted_for_sibling_analysis_def_specialization() {
    let content = r#"
        package PowerAnalysis {
            part def PowerSystem;

            analysis def LoadFlowAnalysis {
                subject powerSystem : PowerSystem;
                return ref loadFlowComplete {
                    return true;
                }
            }

            analysis def VoltageDropAnalysis :> LoadFlowAnalysis {
                subject powerSystem : PowerSystem;
                return ref voltageDropComplete {
                    return true;
                }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("resolved_analysis_specializes_base.sysml", content);
    let found_unresolved_specializes = diagnostics.iter().any(|diagnostic| {
        diagnostic.source == "semantic" && diagnostic.code == "unresolved_specializes_reference"
    });

    assert!(
        !found_unresolved_specializes,
        "did not expect unresolved_specializes_reference when sibling analysis def base resolves"
    );
}

#[test]
fn implicit_redefinition_without_operator_emits_error_for_inherited_features() {
    let content = r#"
        package P {
            part def Engine {}
            port def PowerPort {}
            part def Base {
                attribute mass : Real;
                part engine : Engine;
                port outlet : PowerPort;
            }
            part def Child :> Base {
                attribute mass = 1200;
                attribute engine = replacementEngine;
                attribute outlet = replacementOutlet;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("implicit_redefine_inherited.sysml", content);
    let implicit_redefine: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.source == "semantic"
                && diagnostic.code == "implicit_redefinition_without_operator"
        })
        .collect();
    assert!(
        !implicit_redefine.is_empty(),
        "expected implicit_redefinition_without_operator diagnostics for inherited assignments"
    );
    assert!(
        implicit_redefine
            .iter()
            .all(|diagnostic| diagnostic.severity == sysml_diagnostics::DiagnosticSeverity::Error),
        "expected implicit redefinition diagnostics to be errors: {implicit_redefine:#?}"
    );
}

#[test]
fn explicit_redefinition_operator_avoids_implicit_redefinition_diagnostic() {
    let content = r#"
        package P {
            part def Engine {}
            port def PowerPort {}
            part def Base {
                attribute mass : Real;
                part engine : Engine;
                port outlet : PowerPort;
            }
            part def Child :> Base {
                attribute :>> mass = 1200;
                attribute :>> engine = replacementEngine;
                attribute :>> outlet = replacementOutlet;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("explicit_redefine_inherited.sysml", content);
    let has_implicit_redefine = diagnostics.iter().any(|diagnostic| {
        diagnostic.source == "semantic"
            && diagnostic.code == "implicit_redefinition_without_operator"
    });
    assert!(
        !has_implicit_redefine,
        "did not expect implicit_redefinition_without_operator with explicit :>>"
    );
}

#[test]
fn unresolved_satisfy_reference_emits_semantic_diagnostic() {
    let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("requirements_unresolved_satisfy.sysml");
    let content = fs::read_to_string(&fixture_path).expect("read unresolved satisfy fixture");
    let diagnostics = validate_inline_sysml("unresolved_satisfy.sysml", &content);
    // The publication settles every authored reference the same way, so a satisfy endpoint that
    // names nothing is `unresolved_reference` rather than a satisfy-specific code.
    assert!(
        has_diag_code(&diagnostics, "semantic", "unresolved_reference"),
        "expected an unresolved reference for the missing satisfy endpoint: {diagnostics:#?}"
    );
}

#[test]
fn unresolved_allocate_reference_emits_semantic_diagnostic() {
    let content = r#"
        package P {
            part def Host {
                allocate missingAction to missingPart;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("unresolved_allocate.sysml", content);
    let found_unresolved_allocate = has_diag_code(&diagnostics, "semantic", "unresolved_reference");

    assert!(
        found_unresolved_allocate,
        "expected unresolved_allocate_* semantic diagnostic for missing allocate reference"
    );
}

#[test]
fn unbound_constraint_def_expression_does_not_emit_analysis_evaluation_unresolved_diagnostic() {
    let content = r#"
        package P {
            constraint def EnduranceMargin {
                in measured : Real;
                in limit : Real;
                measured <= limit
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_constraint_unbound.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "analysis_evaluation_unresolved"),
        "did not expect analysis_evaluation_unresolved semantic diagnostic for definition-only constraint"
    );
}

#[test]
fn requirement_local_attributes_resolve_in_arithmetic_constraint() {
    let content = r#"
        package P {
            requirement def SarEvaluation {
                attribute allowedSar = 2.0;
                attribute estimatedSar = 1.7;
                attribute uncertaintyAllowance = 0.1;
                require constraint {
                    allowedSar - estimatedSar - uncertaintyAllowance >= 0
                }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_requirement_locals.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "analysis_evaluation_unresolved"),
        "did not expect analysis_evaluation_unresolved diagnostic when requirement-local attributes are declared"
    );
}

#[test]
fn typed_requirement_local_attributes_resolve_in_arithmetic_constraint() {
    let content = r#"
        package P {
            requirement def SarEvaluation {
                attribute allowedSar: Real = 2.0;
                attribute estimatedSar: Real = 1.7;
                attribute uncertaintyAllowance: Real = 0.1;
                require constraint {
                    allowedSar - estimatedSar - uncertaintyAllowance >= 0
                }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_requirement_locals_typed.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "analysis_evaluation_unresolved"),
        "did not expect analysis_evaluation_unresolved diagnostic when typed requirement-local attributes are declared: {diagnostics:#?}"
    );
}

#[test]
fn requirement_def_placeholder_attribute_does_not_emit_incomplete_analysis_info() {
    // `requirement def` is a reusable, parametric template intentionally left without a
    // concrete binding (S42-LIM-012). `analysis_evaluation_incomplete` was removed entirely
    // (it was more distracting than useful — an unassigned declared value isn't itself a
    // problem worth surfacing), so this and the usage case below both assert its absence now.
    let content = r#"
        package P {
            requirement def PlaceholderEvaluation {
                attribute actual;
                attribute limit = 1.0;
                require constraint { actual <= limit }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_requirement_placeholder.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "analysis_evaluation_incomplete"),
        "requirement def templates should not emit analysis_evaluation_incomplete: {diagnostics:#?}"
    );
    assert!(
        !has_diag_code(&diagnostics, "semantic", "analysis_evaluation_unresolved"),
        "placeholder should not be reported as unresolved: {diagnostics:#?}"
    );
}

#[test]
fn requirement_usage_placeholder_attribute_does_not_emit_incomplete_analysis_info() {
    let content = r#"
        package P {
            requirement placeholderEvaluation {
                attribute actual;
                attribute limit = 1.0;
                require constraint { actual <= limit }
            }
        }
    "#;
    let diagnostics =
        validate_inline_sysml("analysis_requirement_usage_placeholder.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "analysis_evaluation_incomplete"),
        "analysis_evaluation_incomplete was removed entirely: {diagnostics:#?}"
    );
    assert!(
        !has_diag_code(&diagnostics, "semantic", "analysis_evaluation_unresolved"),
        "placeholder should not be reported as unresolved: {diagnostics:#?}"
    );
}

#[test]
fn missing_analysis_identifier_still_emits_unresolved_warning() {
    let content = r#"
        package P {
            requirement def MissingReferenceEvaluation {
                attribute limit = 1.0;
                require constraint { missingActual <= limit }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_requirement_missing_ref.sysml", content);
    let unresolved: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.source == "semantic" && diagnostic.code == "analysis_evaluation_unresolved"
        })
        .collect();
    assert_eq!(
        unresolved.len(),
        1,
        "expected one analysis_evaluation_unresolved diagnostic: {diagnostics:#?}"
    );
    assert_eq!(
        unresolved[0].severity,
        sysml_diagnostics::DiagnosticSeverity::Warning
    );
}

#[test]
fn false_analysis_constraint_still_emits_failed_warning() {
    let content = r#"
        package P {
            requirement def FailedEvaluation {
                attribute actual = 2.0;
                attribute limit = 1.0;
                require constraint { actual <= limit }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_requirement_failed.sysml", content);
    let failed: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.source == "semantic" && diagnostic.code == "analysis_constraint_failed"
        })
        .collect();
    assert_eq!(
        failed.len(),
        1,
        "expected one analysis_constraint_failed diagnostic: {diagnostics:#?}"
    );
    assert_eq!(
        failed[0].severity,
        sysml_diagnostics::DiagnosticSeverity::Warning
    );
}

#[test]
fn valid_analysis_constraint_emits_no_analysis_diagnostic() {
    let content = r#"
        package P {
            requirement def PassingEvaluation {
                attribute actual: Real = 0.5;
                attribute limit: Real = 1.0;
                require constraint { actual <= limit }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_requirement_passing.sysml", content);
    assert!(
        !diagnostics.iter().any(|diagnostic| {
            diagnostic.source == "semantic" && diagnostic.code.starts_with("analysis_")
        }),
        "expected no analysis diagnostic for passing constraint: {diagnostics:#?}"
    );
}

#[test]
fn multi_line_and_requirement_constraint_uses_full_expression_span() {
    let content = r#"
        package P {
            requirement def LandingEvaluation {
                attribute actualVerticalVelocity: Real = 0.9;
                attribute maxVerticalVelocity: Real = 2.0;
                attribute actualHorizontalVelocity: Real = 0.2;
                attribute maxHorizontalVelocity: Real = 0.5;
                attribute actualLandingZoneDeviation: Real = 600.0;
                attribute maxLandingZoneDeviation: Real = 1000.0;
                require constraint {
                    (actualVerticalVelocity <= maxVerticalVelocity) and
                    (actualHorizontalVelocity <= maxHorizontalVelocity) and
                    (actualLandingZoneDeviation <= maxLandingZoneDeviation)
                }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_requirement_multiline_and.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "analysis_evaluation_unresolved"),
        "did not expect analysis_evaluation_unresolved diagnostic for multi-line boolean constraint: {diagnostics:#?}"
    );
}

#[test]
fn analysis_objective_inherits_parent_return_ref_without_local_result() {
    let content = r#"
        package PowerAnalysis {
            part def PowerSystem;

            analysis def LoadFlowAnalysis {
                subject powerSystem : PowerSystem;
                return ref loadFlowComplete {
                    return true;
                }
            }

            analysis def VoltageDropAnalysis :> LoadFlowAnalysis {
                objective voltageDropObjective {
                    doc /* Evaluate voltage deviations across medium-voltage nodes. */
                }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("analysis_inherited_return_ref.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "objective_binding_unresolved"),
        "specialized analysis def should inherit parent return ref for objective binding"
    );
}

#[test]
fn compatible_different_port_def_connection_has_no_port_type_mismatch_diagnostic() {
    let content = r#"
        package P {
            item def Water;

            port def DeviceWaterInletPort {
                in item water : Water;
            }

            port def WaterSpigotPort {
                out item water : Water;
            }

            part def Dishwasher {
                port waterInlet : DeviceWaterInletPort;
            }

            part def Kitchen {
                port waterSpigot : WaterSpigotPort;
            }

            part def Home {
                part dishwasher : Dishwasher;
                part kitchen : Kitchen;
                connect dishwasher.waterInlet to kitchen.waterSpigot;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("compatible_ports_lsp.sysml", content);
    let found_port_type_mismatch = has_diag_code(&diagnostics, "semantic", "port_type_mismatch");

    assert!(
        !found_port_type_mismatch,
        "feature-compatible port definitions should not emit port_type_mismatch diagnostics"
    );
}

#[test]
fn typed_interface_does_not_compare_distinct_declared_ends_pairwise() {
    let content = r#"
        package P {
            item def ElectricalSignal;

            port def I2cPort {
                inout item clock : ElectricalSignal;
                inout item data : ElectricalSignal;
            }
            port def I2cControllerPort :> I2cPort;
            port def I2cTargetPort :> I2cPort;

            interface def I2cLink {
                end controllerPort : I2cControllerPort;
                end targetPort : I2cTargetPort;
            }

            part def Controller {
                port bus : I2cControllerPort;
            }
            part def Target {
                port bus : I2cTargetPort;
            }
            part def System {
                part controller : Controller;
                part target : Target;
                interface link : I2cLink
                    connect controllerPort ::> controller.bus
                    to targetPort ::> target.bus;
            }
            part def UntypedSystem {
                part controller : Controller;
                part target : Target;
                interface controller.bus to target.bus;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("typed_interface_distinct_ends.sysml", content);

    assert!(
        !has_diag_code(&diagnostics, "semantic", "port_type_mismatch"),
        "typed interface endpoints conform to their declared ends and must not be compared pairwise: {diagnostics:?}"
    );
    assert!(
        !has_diag_code(&diagnostics, "semantic", "conjugated_port_inconsistent"),
        "typed interface endpoints need not share one conjugated port definition: {diagnostics:?}"
    );
}

#[test]
fn part_to_part_connect_has_no_connection_endpoint_not_port_diagnostic() {
    let content = r#"
        package P {
            part def System;
            part def Environment;
            part def Context {
                part system : System;
                part environment : Environment;
                connect environment to system;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("part_to_part_connect.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "semantic", "connection_endpoint_not_port"),
        "logical part-to-part connect should not warn about non-port endpoints: {:?}",
        diagnostics
    );
}

#[test]
fn homonymous_port_defs_emit_port_type_mismatch_with_qualified_names() {
    // The feature types are declared here rather than taken from the standard library: without a
    // library the publication cannot say whether `Real` and `Integer` are related, and a rule that
    // reported them as unrelated would be answering from a missing input.
    let content = r#"
        package P {
            attribute def Level;
            attribute def Count;
            package PkgA {
                port def FillState { in level : P::Level; }
            }
            package PkgB {
                port def FillState { in level : P::Count; }
            }
            part def TankA { port fill : PkgA::FillState; }
            part def TankB { port fill : PkgB::FillState; }
            part context {
                part tankA : TankA;
                part tankB : TankB;
                connect tankA.fill to tankB.fill;
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("homonymous_ports.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "sysml", "expected_keyword"),
        "fixture should parse cleanly: {:?}",
        diagnostics
    );
    assert!(
        has_diag_code(&diagnostics, "semantic", "port_type_mismatch"),
        "homonymous incompatible port defs should emit port_type_mismatch; got: {:?}",
        diagnostics
    );
    // The two ends are named by typed related locations rather than by the message: a
    // diagnostic's text is presentation, and reading a symbol back out of it is exactly the
    // inference this migration removed.
    let related = diagnostics
        .iter()
        .find(|d| d.source == "semantic" && d.code == "port_type_mismatch")
        .map(|d| d.related_information.clone())
        .unwrap_or_default();
    assert_eq!(
        related.len(),
        2,
        "both connected ports are related locations: {related:?}"
    );
}

/// `part def` (and other package-body elements) directly at the root, with no enclosing
/// `package { ... }`, is valid SysML v2 -- sysml-v2-parser 0.50.0 removed the non-spec
/// `illegal_top_level_definition` diagnostic and parses this as `RootElement::Member` instead
/// (see sysml-v2-parser's `PARSER_BACKLOG_ROADMAP.md`). Only the genuine semantic issue in the
/// body (`part motherboard;` has no type) should still be reported.
#[test]
fn top_level_part_def_is_valid_and_only_reports_the_untyped_member_inside_it() {
    let content = r#"
part def Laptop {
    part motherboard;
}
"#;
    let diagnostics = validate_inline_sysml("top_level_part_def.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "sysml", "illegal_top_level_definition"),
        "illegal_top_level_definition was removed as non-spec; a top-level part def is valid: {:?}",
        diagnostics
    );
    assert!(
        has_diag_code(&diagnostics, "semantic", "untyped_part_usage"),
        "expected the untyped `part motherboard;` to still be flagged: {:?}",
        diagnostics
    );
}

#[test]
fn qualified_package_declaration_has_no_diagnostics() {
    let content = r#"
        package AstronomyReference::Domain {
            part def Thing;
        }
    "#;
    let diagnostics = validate_inline_sysml("qualified_package.sysml", content);
    assert!(
        diagnostics.is_empty(),
        "expected qualified package declaration to be diagnostic-clean, got: {diagnostics:#?}"
    );
}

#[test]
fn nested_ref_part_assignments_have_no_parse_diagnostics() {
    let content = r#"
        package RefPartAssignmentProbe {
            part def Body;
            part def Orbit {
                ref part centralBody : Body;
                ref part orbitingBody : Body;
            }
            part system {
                part sun : Body;
                part earth : Body;
                part earthOrbit : Orbit {
                    ref part centralBody = sun;
                    ref part orbitingBody : Body = earth;
                }
            }
        }
    "#;
    let diagnostics = validate_inline_sysml("ref_part_assignment.sysml", content);
    assert!(
        !has_diag_code(&diagnostics, "parser", "recovered_part_usage_body_element"),
        "valid ref part assignments must not recover as part usage body elements: {diagnostics:#?}"
    );
    // `part system { ... }` declares no type, which the publication reports as information. The
    // fixture is about parsing, so only the parser's own diagnostics must be absent.
    let parse_diagnostics = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.source == "sysml")
        .collect::<Vec<_>>();
    assert!(
        parse_diagnostics.is_empty(),
        "expected ref part assignment fixture to parse cleanly, got: {parse_diagnostics:#?}"
    );
}
