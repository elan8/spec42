use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

fn workspace_doc(content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "workspace",
        "multiplicity.sysml",
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

fn invalid_multiplicity_count(graph: &sysml_model::SemanticGraph, uri: &url::Url) -> usize {
    collect_diagnostics_from_graph(graph, uri, DiagnosticsOptions::default())
        .into_iter()
        .filter(|diagnostic| diagnostic.code == "invalid_multiplicity")
        .count()
}

#[test]
fn multiplicity_validation_uses_declared_bounds_and_handles_unbounded_forms() {
    let doc = workspace_doc(
        r#"package P {
  part valid : Thing[1..*];
  part reversed : Thing[3..1];
  part unboundedLower : Thing[*..5];
  part negative : Thing[-1..2];
  part fractional : Thing[1.5..2];
}"#,
    );
    let uri = doc.uri.clone();
    let (mut graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let valid = graph
        .nodes_named("valid")
        .into_iter()
        .next()
        .expect("valid part");
    assert!(
        valid.declared_facts.multiplicity.is_some(),
        "the parser-backed multiplicity must be retained on the semantic node"
    );
    assert_eq!(invalid_multiplicity_count(&graph, &uri), 4);

    let valid_id = valid.id.clone();
    graph
        .get_node_mut(&valid_id)
        .expect("valid part")
        .attributes
        .insert("multiplicity".to_string(), serde_json::json!("[3..1]"));
    assert_eq!(
        invalid_multiplicity_count(&graph, &uri),
        4,
        "display attributes must not replace parser-backed multiplicity facts"
    );
}

#[test]
fn redefinition_multiplicity_conformance_uses_declared_bounds() {
    let doc = workspace_doc(
        r#"package P {
  part def Engine {
    part cylinders[8];
  }
  part def SixCylinderEngine :> Engine {
    part redefines cylinders[6];
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (mut graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let redefining = graph
        .nodes_named("cylinders")
        .into_iter()
        .find(|node| node.id.qualified_name == "P::SixCylinderEngine::cylinders")
        .expect("redefining feature");
    let redefining_id = redefining.id.clone();
    graph
        .get_node_mut(&redefining_id)
        .expect("redefining feature")
        .attributes
        .insert("multiplicity".to_string(), serde_json::json!("[8]"));
    assert!(
        collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default())
            .iter()
            .any(|diagnostic| diagnostic.code == "redefinition_multiplicity_widened"),
        "redefinition conformance must use its declared multiplicity rather than display text"
    );
}
