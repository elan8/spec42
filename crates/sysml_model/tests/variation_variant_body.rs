//! Regression coverage for S42-LIM-014: `variant` members inside a `variation part def` body
//! should parse without recovery errors and appear as owned nodes in the semantic graph.

use sysml_model::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind,
};

fn workspace_doc(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "workspace",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

#[test]
fn variation_part_def_variant_members_produce_no_recovery_diagnostics() {
    let doc = workspace_doc(
        "variants.sysml",
        r#"package P {
  part def SensorAssembly;

  variation part def NavigationSensorSuiteChoice :> SensorAssembly {
    variant tofImuOnly;
    variant lidarSlamSuite;
    variant aiVisionSuite;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let recovery_errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.code == "recovered_part_def_body_element")
        .collect();
    assert!(
        recovery_errors.is_empty(),
        "variation body should not produce parser recovery diagnostics: {recovery_errors:#?}"
    );

    let variation = graph
        .nodes_named("NavigationSensorSuiteChoice")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("variation part def node");

    for name in ["tofImuOnly", "lidarSlamSuite", "aiVisionSuite"] {
        let variant = graph
            .nodes_named(name)
            .into_iter()
            .find(|node| node.element_kind == "variant")
            .unwrap_or_else(|| panic!("expected variant node '{name}' in semantic graph"));
        assert_eq!(
            variant.parent_id.as_ref(),
            Some(&variation.id),
            "variant '{name}' should be owned by the variation part def"
        );
    }
}

/// Spec §7.6.7's own first example uses the typed form (`variant part name : Type;`), not the
/// bare reference form — both must be supported.
#[test]
fn variation_part_def_typed_part_variant_materializes_as_part_node() {
    let doc = workspace_doc(
        "typed_variants.sysml",
        r#"package P {
  part def Transmission;
  part def ManualTransmission :> Transmission;
  part def AutomaticTransmission :> Transmission;

  variation part def TransmissionChoices :> Transmission {
    variant part manual : ManualTransmission;
    variant part automatic : AutomaticTransmission;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let recovery_errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.code == "recovered_part_def_body_element")
        .collect();
    assert!(
        recovery_errors.is_empty(),
        "typed variant body should not produce parser recovery diagnostics: {recovery_errors:#?}"
    );

    let variation = graph
        .nodes_named("TransmissionChoices")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("variation part def node");

    for (name, type_name) in [
        ("manual", "ManualTransmission"),
        ("automatic", "AutomaticTransmission"),
    ] {
        let variant = graph
            .nodes_named(name)
            .into_iter()
            .find(|node| node.element_kind == "part")
            .unwrap_or_else(|| panic!("expected '{name}' to materialize as a part node"));
        assert_eq!(
            variant.parent_id.as_ref(),
            Some(&variation.id),
            "typed variant '{name}' should be owned by the variation part def"
        );
        assert_eq!(
            variant.attributes.get("isVariant").and_then(|v| v.as_bool()),
            Some(true),
            "typed variant '{name}' should be tagged isVariant"
        );
        assert_eq!(
            variant.attributes.get("partType").and_then(|v| v.as_str()),
            Some(type_name),
            "typed variant '{name}' should carry its declared type"
        );
    }
}
