use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
};

const ELECTRICAL_QUANTITIES_SYSML: &str = r#"
package ISQElectromagnetism {
    attribute def ElectricPotentialDifferenceValue;
}

package ElectricalQuantities {
    private import ISQElectromagnetism::*;
    attribute def Voltage :> ElectricPotentialDifferenceValue;
}
"#;

#[test]
fn attribute_def_quantity_specialization_resolves_in_workspace() {
    let doc = SysmlDocument::from_memory_path(
        "electrical-quantities",
        "electrical_quantities.sysml",
        ELECTRICAL_QUANTITIES_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[doc]).expect("semantic graph should build");

    let voltage = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "attribute def" && node.name == "Voltage")
        .expect("Voltage attribute def");

    let typing_targets = graph.outgoing_targets_by_kind(voltage, RelationshipKind::Typing);
    assert_eq!(typing_targets.len(), 1);
    assert_eq!(
        typing_targets[0].name, "ElectricPotentialDifferenceValue",
        "Voltage should specialize the ISQ quantity attribute def"
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let unresolved: Vec<_> = diagnostics
        .iter()
        .filter(|diag| {
            diag.source == "semantic"
                && matches!(
                    diag.code.as_str(),
                    "unresolved_type_reference" | "unresolved_specializes_reference"
                )
        })
        .map(|diag| (&diag.code, &diag.message))
        .collect();
    assert!(
        unresolved.is_empty(),
        "expected quantity attribute specialization to resolve without type diagnostics: {unresolved:?}"
    );
}
