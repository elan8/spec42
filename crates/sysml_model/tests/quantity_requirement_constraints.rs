use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{build_semantic_graph_from_documents, evaluate_expressions, SysmlDocument, SysmlDocumentSourceKind};
use url::Url;

const SI_CATALOG_EXCERPT: &str = r#"
package SIPrefixes {
    attribute kilo: UnitPrefix { :>> symbol = "k"; :>> conversionFactor = 1E3; }
    attribute mega: UnitPrefix { :>> symbol = "M"; :>> conversionFactor = 1E6; }
}
package SI {
    attribute <W> watt : PowerUnit;
}
"#;

fn catalog_uri() -> Url {
    Url::parse("file:///sysml.library/Domain%20Libraries/Quantities%20and%20Units/SI.sysml")
        .expect("catalog uri")
}

fn catalog_document() -> SysmlDocument {
    SysmlDocument {
        uri: catalog_uri(),
        content: SI_CATALOG_EXCERPT.to_string(),
        path_hint: Some("Domain Libraries/Quantities and Units/SI.sysml".to_string()),
        source_kind: SysmlDocumentSourceKind::Library,
        sha256: None,
        byte_size: None,
    }
}

const PASSING_REQUIREMENT_SYSML: &str = r#"
package GridRequirements {
    requirement def GridCapacity {
        attribute basePeakLoad = 12.3;
        attribute winterMultiplier = 1.30;
        attribute requiredCapacity = 16.0;
        require constraint {
            basePeakLoad * winterMultiplier <= requiredCapacity
        }
    }
}
"#;

const FAILING_REQUIREMENT_SYSML: &str = r#"
package GridRequirements {
    requirement def GridCapacity {
        attribute basePeakLoad = 12.3;
        attribute winterMultiplier = 1.30;
        attribute requiredCapacity = 15.0;
        require constraint {
            basePeakLoad * winterMultiplier <= requiredCapacity
        }
    }
}
"#;

const INHERITED_DEF_CONSTRAINT_SYSML: &str = r#"
package GridRequirements {
    requirement def CapacityRequirement {
        attribute basePeakLoad;
        attribute winterMultiplier = 1.30;
        attribute requiredCapacity;
        require constraint {
            basePeakLoad * winterMultiplier <= requiredCapacity
        }
    }

    requirement gridCapacity2035 : CapacityRequirement {
        attribute basePeakLoad = 12.3;
        attribute requiredCapacity = 16.0;
    }
}
"#;

const QUANTITY_UNITS_REQUIREMENT_SYSML: &str = r#"
package GridRequirements {
    requirement def GridCapacity {
        attribute basePeakLoad = 12.3 [MW];
        attribute winterMultiplier = 1.30;
        attribute requiredCapacity = 16 [MW];
        require constraint {
            basePeakLoad * winterMultiplier <= requiredCapacity
        }
    }
}
"#;

fn build_graph(source: &str) -> sysml_model::SemanticGraph {
    build_graph_from_documents(&[document_from_source(source)])
}

fn build_graph_with_units(source: &str) -> sysml_model::SemanticGraph {
    build_graph_from_documents(&[catalog_document(), document_from_source(source)])
}

fn document_from_source(source: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "quantity-requirement-constraints",
        "GridRequirements.sysml",
        source.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri")
}

fn build_graph_from_documents(docs: &[SysmlDocument]) -> sysml_model::SemanticGraph {
    let (mut graph, _) = build_semantic_graph_from_documents(docs).expect("semantic graph");
    evaluate_expressions(&mut graph);
    graph
}

fn node_attr(graph: &sysml_model::SemanticGraph, qualified: &str, key: &str) -> Option<String> {
    graph
        .node_ids_by_qualified_name
        .get(qualified)?
        .first()
        .and_then(|node_id| graph.get_node(node_id))
        .and_then(|node| node.attributes.get(key))
        .and_then(|value| match value {
            serde_json::Value::String(text) => Some(text.clone()),
            serde_json::Value::Number(number) => Some(number.to_string()),
            serde_json::Value::Bool(flag) => Some(flag.to_string()),
            _ => None,
        })
}

fn has_code(graph: &sysml_model::SemanticGraph, code: &str) -> bool {
    let uri = graph
        .node_ids_by_qualified_name
        .get("GridRequirements::GridCapacity")
        .and_then(|ids| ids.first())
        .map(|id| id.uri.clone())
        .expect("requirement uri");
    collect_diagnostics_from_graph(graph, &uri, DiagnosticsOptions::default())
        .into_iter()
        .any(|diag| diag.code == code)
}

#[test]
fn requirement_usage_quantity_constraint_passes_when_within_capacity() {
    let graph = build_graph(PASSING_REQUIREMENT_SYSML);
    assert_eq!(
        node_attr(
            &graph,
            "GridRequirements::GridCapacity",
            "analysisEvaluationStatus"
        ),
        Some("ok".to_string())
    );
    assert!(
        !has_code(&graph, "analysis_evaluation_unresolved"),
        "quantity require constraint should evaluate without unresolved status"
    );
}

#[test]
fn requirement_usage_quantity_constraint_fails_when_over_capacity() {
    let graph = build_graph(FAILING_REQUIREMENT_SYSML);
    assert_eq!(
        node_attr(
            &graph,
            "GridRequirements::GridCapacity",
            "analysisEvaluationStatus"
        ),
        Some("failed_constraint".to_string())
    );
    assert!(has_code(&graph, "analysis_constraint_failed"));
}

#[test]
fn requirement_usage_inherits_require_constraint_from_typed_definition() {
    let graph = build_graph(INHERITED_DEF_CONSTRAINT_SYSML);
    assert_eq!(
        node_attr(
            &graph,
            "GridRequirements::gridCapacity2035",
            "analysisEvaluationStatus"
        ),
        Some("ok".to_string())
    );
}

#[test]
fn requirement_def_quantity_units_evaluate_in_constraint() {
    let graph = build_graph_with_units(QUANTITY_UNITS_REQUIREMENT_SYSML);
    assert_eq!(
        node_attr(
            &graph,
            "GridRequirements::GridCapacity",
            "analysisEvaluationStatus"
        ),
        Some("ok".to_string())
    );
}
