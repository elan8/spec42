use url::Url;

use sysml_model::semantic::graph::SemanticGraph;
use sysml_model::UnitRegistry;

use super::engine_impl::{
    compute_semantic_diagnostics, compute_semantic_diagnostics_with_unit_registry,
};
use super::types::{DiagnosticsOptions, SemanticDiagnostic};

/// Collects semantic diagnostics from an already-built semantic graph.
///
/// This API is graph-first and host-agnostic: callers provide the graph and URI,
/// and receive neutral diagnostics that can be mapped to transport-specific types.
pub fn collect_diagnostics_from_graph(
    graph: &SemanticGraph,
    uri: &Url,
    options: DiagnosticsOptions,
) -> Vec<SemanticDiagnostic> {
    compute_semantic_diagnostics(graph, uri, options)
}

/// Collects semantic diagnostics reusing a workspace-level [`UnitRegistry`].
pub fn collect_diagnostics_from_graph_with_unit_registry(
    graph: &SemanticGraph,
    uri: &Url,
    options: DiagnosticsOptions,
    unit_registry: &UnitRegistry,
) -> Vec<SemanticDiagnostic> {
    compute_semantic_diagnostics_with_unit_registry(graph, uri, options, unit_registry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiagnosticSeverity;
    use sysml_model::build_graph_from_doc;

    #[test]
    fn collect_diagnostics_from_graph_emits_implicit_redefinition_without_operator() {
        let input = r#"
            package P {
                part def Base {
                    attribute mass : Real;
                }
                part def Child :> Base {
                    attribute mass = 1200;
                }
            }
        "#;
        let parsed = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&parsed, &uri);
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "implicit_redefinition_without_operator"
                && diagnostic.severity == DiagnosticSeverity::Error
        }));
    }

    #[test]
    fn collect_diagnostics_from_graph_emits_inherited_attribute_value_type_mismatch() {
        let input = r#"
            package Demo {
                enum def RequirementStatusKind {
                    enum approved;
                }
                requirement def ManagedRequirement {
                    attribute status : RequirementStatusKind;
                }
                requirement def UserRequirement :> ManagedRequirement;
                requirement def Need :> UserRequirement;
                requirement need : Need {
                    attribute status = "approved";
                }
            }
        "#;
        let parsed = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&parsed, &uri);
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        let codes: Vec<_> = diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "implicit_redefinition_without_operator"
                    && diagnostic.severity == DiagnosticSeverity::Error
            }),
            "expected implicit_redefinition_without_operator, got {codes:?}"
        );
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "inherited_attribute_value_type_mismatch"
                && diagnostic.severity == DiagnosticSeverity::Error
        }));
    }

    #[test]
    fn collect_diagnostics_from_graph_accepts_enum_status_redefinition() {
        let input = r#"
            package Demo {
                enum def RequirementStatusKind {
                    enum approved;
                }
                requirement def ManagedRequirement {
                    attribute status : RequirementStatusKind;
                }
                requirement def UserRequirement :> ManagedRequirement;
                requirement def Need :> UserRequirement;
                requirement need : Need {
                    attribute :>> status = RequirementStatusKind::approved;
                }
            }
        "#;
        let parsed = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&parsed, &uri);
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(!diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "implicit_redefinition_without_operator"
                || diagnostic.code == "inherited_attribute_value_type_mismatch"
        }));
    }

    #[test]
    fn collect_diagnostics_from_graph_accepts_resolved_port_def_specialization() {
        let input = r#"
            package P {
                port def BasePort;
                port def ChildPort :> BasePort;
            }
        "#;
        let parsed = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&parsed, &uri);
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(!diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "unresolved_specializes_reference" }));
    }

    #[test]
    fn collect_diagnostics_from_graph_emits_inherited_part_attribute_value_type_mismatch() {
        let input = r#"
            package Demo {
                enum def StatusKind {
                    enum approved;
                }
                part def BasePart {
                    attribute status : StatusKind;
                }
                part def DerivedPart :> BasePart;
                part host : DerivedPart {
                    attribute status = "approved";
                }
            }
        "#;
        let parsed = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&parsed, &uri);
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "inherited_attribute_value_type_mismatch"
                && diagnostic.severity == DiagnosticSeverity::Error
        }));
    }

    #[test]
    fn collect_diagnostics_from_graph_emits_unresolved_import_target() {
        let input = r#"
            package P {
                import Q::*;
            }
        "#;
        let parsed = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&parsed, &uri);
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "unresolved_import_target"));
    }

    #[test]
    fn collect_diagnostics_from_graph_accepts_spec42_standard_view_types() {
        let input = r#"
            package Views {
                view structure : GeneralView;
                view connections : InterconnectionView;
                view checkoutFlow : SequenceView;
                view orderLifecycle : StateTransitionView;
                view checkoutPipeline : ActionFlowView;
            }
        "#;
        let parsed = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///Views.sysml").expect("uri");
        let graph = build_graph_from_doc(&parsed, &uri);
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        let unresolved: Vec<_> = diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "unresolved_type_reference")
            .map(|diagnostic| diagnostic.message.clone())
            .collect();
        assert!(
            unresolved.is_empty(),
            "expected no unresolved_type_reference for standard view types, got: {unresolved:?}"
        );
    }
}
