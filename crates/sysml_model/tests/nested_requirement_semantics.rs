use sysml_model::{
    build_semantic_graph_from_documents, ElementKind, NodeId, RelationshipKind, SysmlDocument,
    SysmlDocumentSourceKind,
};

#[test]
fn composite_subrequirements_are_materialized_with_inherited_subjects() {
    let source = r#"
package Requirements {
    part def Vehicle;
    part robot : Vehicle;
    requirement brakingNeed;

    requirement systemSpecification {
        subject vehicle : Vehicle;

        requirement braking {
            requirement emergencyStop;
        }
    }

    dependency brakingRefinement
        from systemSpecification::braking to brakingNeed;
    satisfy systemSpecification.braking by robot;
}
"#;
    let document = SysmlDocument::from_memory_path(
        "workspace",
        "nested-requirements.sysml",
        source.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("fixture document");
    let uri = document.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("semantic graph");

    let specification_id = NodeId::new(&uri, "Requirements::systemSpecification");
    let braking_id = NodeId::new(&uri, "Requirements::systemSpecification::braking");
    let emergency_stop_id = NodeId::new(
        &uri,
        "Requirements::systemSpecification::braking::emergencyStop",
    );
    let subject_id = NodeId::new(&uri, "Requirements::systemSpecification::vehicle");

    let specification = graph
        .get_node(&specification_id)
        .expect("system specification requirement");
    let braking = graph.get_node(&braking_id).expect("braking subrequirement");
    let emergency_stop = graph
        .get_node(&emergency_stop_id)
        .expect("recursive emergency-stop subrequirement");

    assert_eq!(specification.element_kind, ElementKind::Requirement);
    assert_eq!(braking.element_kind, ElementKind::Requirement);
    assert_eq!(emergency_stop.element_kind, ElementKind::Requirement);

    for requirement in [braking, emergency_stop] {
        let subjects = graph.outgoing_targets_by_kind(requirement, RelationshipKind::Subject);
        assert_eq!(subjects.len(), 1);
        assert_eq!(subjects[0].id, subject_id);
    }

    let dependency_targets = graph.outgoing_targets_by_kind(braking, RelationshipKind::Dependency);
    assert_eq!(dependency_targets.len(), 1);
    assert_eq!(
        dependency_targets[0].id.qualified_name,
        "Requirements::brakingNeed"
    );

    let robot_id = NodeId::new(&uri, "Requirements::robot");
    let satisfied_by = graph.outgoing_targets_by_kind(braking, RelationshipKind::Satisfy);
    assert_eq!(satisfied_by.len(), 1);
    assert_eq!(satisfied_by[0].id, robot_id);
}
