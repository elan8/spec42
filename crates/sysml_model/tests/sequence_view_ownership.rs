use sysml_model::{
    semantic::sequence_views::build_workspace_sequence_diagrams, SysmlDocument,
    SysmlDocumentSourceKind,
};

/// The legacy sequence extractor still assigns roles from names because the semantic
/// graph has no explicit canonical sequence-library/profile identity provider yet.
#[test]
#[ignore = "SKIP: sequence role classification requires an explicit canonical library/profile identity; an ordinary resolved part def named InteractionScenario must not establish that role"]
fn unprofiled_interaction_scenario_name_does_not_establish_a_sequence_role() {
    let document = SysmlDocument::from_memory_path(
        "workspace",
        "unprofiled-sequence-role.sysml",
        r#"
            package Demo {
                part def InteractionScenario;
                part presentation : InteractionScenario;
            }
        "#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document");
    let uri = document.uri.clone();
    let (graph, _) =
        sysml_model::build_semantic_graph_from_documents(&[document]).expect("semantic graph");

    assert!(
        build_workspace_sequence_diagrams(&graph, &[uri])
            .iter()
            .all(|diagram| diagram.name != "presentation"),
        "a resolved target with only the spelling InteractionScenario must remain unclassified until the canonical sequence profile identity is available"
    );
}
