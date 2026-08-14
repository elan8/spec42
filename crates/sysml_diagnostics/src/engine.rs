use url::Url;

use sysml_model::semantic::graph::SemanticGraph;
use sysml_model::UnitRegistry;

use super::engine_impl::{
    compute_semantic_diagnostics, compute_semantic_diagnostics_with_unit_registry,
};
use super::ordering::canonicalize_diagnostics;
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
    let mut diagnostics = compute_semantic_diagnostics(graph, uri, options);
    canonicalize_diagnostics(&mut diagnostics);
    diagnostics
}

/// Collects semantic diagnostics reusing a workspace-level [`UnitRegistry`].
pub fn collect_diagnostics_from_graph_with_unit_registry(
    graph: &SemanticGraph,
    uri: &Url,
    options: DiagnosticsOptions,
    unit_registry: &UnitRegistry,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics =
        compute_semantic_diagnostics_with_unit_registry(graph, uri, options, unit_registry);
    canonicalize_diagnostics(&mut diagnostics);
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiagnosticSeverity;
    use sysml_model::{
        build_and_link_graph, build_graph_from_doc, build_semantic_model, ConstructionStrategy,
        EvaluationPolicy, ImmutableSourceSnapshot, SemanticBuildRequest, SemanticConfiguration,
        SysmlDocument, SysmlDocumentSourceKind,
    };

    fn collect_from_model(input: &str, uri: &Url) -> Vec<SemanticDiagnostic> {
        let document = SysmlDocument::from_uri(
            uri.as_str(),
            input.to_string(),
            None,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        let model = build_semantic_model(SemanticBuildRequest {
            sources: ImmutableSourceSnapshot::new(vec![document]).expect("source snapshot"),
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: SemanticConfiguration::default(),
        })
        .expect("semantic model");
        crate::collect_document_diagnostics_from_model(
            &model,
            false,
            uri,
            input,
            false,
            DiagnosticsOptions::default(),
        )
    }

    #[test]
    fn collect_diagnostics_from_graph_accepts_resolved_port_def_specialization() {
        let input = r#"
            package P {
                port def BasePort;
                port def ChildPort :> BasePort;
            }
        "#;
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let diagnostics = collect_from_model(input, &uri);
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
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let diagnostics = collect_from_model(input, &uri);
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
    fn analysis_diagnostics_apply_to_authored_constraints_not_inherited_requirement_uses() {
        let input = r#"
            package P {
                requirement def Template {
                    require constraint { missingActual <= limit }
                }
                requirement applied : Template;
                constraint reusable { missingConstraint <= limit }
            }
        "#;
        let document = SysmlDocument::from_memory_path(
            "analysis-owner",
            "analysis-owner.sysml",
            input.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        let uri = document.uri.clone();
        let (graph, _) = build_and_link_graph(&[document]).expect("semantic graph");
        let unresolved =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default())
                .into_iter()
                .filter(|diagnostic| diagnostic.code == "analysis_evaluation_unresolved")
                .collect::<Vec<_>>();

        assert_eq!(
            unresolved.len(),
            2,
            "Template and reusable own unresolved constraints; applied inherits one: {unresolved:#?}"
        );
    }

    #[test]
    fn ambiguous_analysis_is_not_reported_as_unresolved() {
        let input = r#"
            package A { attribute value = 1; }
            package B { attribute value = 2; }
            package P {
                import A::*;
                import B::*;
                requirement def Ambiguous {
                    require constraint { value <= 2 }
                }
            }
        "#;
        let document = SysmlDocument::from_memory_path(
            "ambiguous-analysis",
            "ambiguous-analysis.sysml",
            input.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        let uri = document.uri.clone();
        let (graph, _) = build_and_link_graph(&[document]).expect("semantic graph");
        let ambiguous = graph
            .node_ids_by_qualified_name
            .get("P::Ambiguous")
            .and_then(|ids| ids.first())
            .and_then(|id| graph.get_node(id))
            .and_then(|node| graph.evaluation_facts_for(node))
            .and_then(|facts| facts.analysis.as_ref())
            .expect("analysis facts");
        assert_eq!(
            ambiguous.expression.status,
            sysml_model::EvaluationStatus::Ambiguous
        );

        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "analysis_evaluation_unresolved"),
            "ambiguous status must remain explicit instead of being relabeled: {diagnostics:#?}"
        );
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
