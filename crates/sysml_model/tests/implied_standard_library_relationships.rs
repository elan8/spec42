use sysml_model::{
    build_and_link_graph, build_and_link_graph_parallel, patch_graph_for_document,
    DerivedRelationshipResolution, ImpliedRelationshipRule, RelationshipKind,
    RelationshipProvenance, StandardLibraryElement, SysmlDocument, SysmlDocumentSourceKind,
};

fn document(path: &str, content: &str, source_kind: SysmlDocumentSourceKind) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "implied-standard-library",
        path,
        content.to_owned(),
        source_kind,
        None,
        None,
    )
    .expect("memory document")
}

fn vehicle(graph: &sysml_model::SemanticGraph) -> &sysml_model::SemanticNode {
    let id = graph
        .node_ids_for_qualified_name("Demo::Vehicle")
        .and_then(|ids| ids.first())
        .expect("Vehicle node");
    graph.get_node(id).expect("Vehicle graph node")
}

#[test]
fn definition_and_usage_publish_typed_implied_edges_from_the_standard_library() {
    let workspace = document(
        "workspace.sysml",
        "package Demo { part def Vehicle; part vehicle; }",
        SysmlDocumentSourceKind::Workspace,
    );
    let standard_library = document(
        "standard.sysml",
        "package Parts { part def Part; part parts; }",
        SysmlDocumentSourceKind::StandardLibrary,
    );
    let (graph, _) = build_and_link_graph(&[workspace, standard_library]).expect("graph");
    let definition = vehicle(&graph);

    assert!(matches!(
        graph.universal_relationship_resolution_for(definition),
        DerivedRelationshipResolution::Resolved { ref target }
            if target.qualified_name == "Parts::Part"
    ));
    assert_eq!(
        graph
            .outgoing_targets_by_kind_and_provenance(
                definition,
                RelationshipKind::Specializes,
                RelationshipProvenance::Implied(
                    ImpliedRelationshipRule::UniversalStandardLibraryRelationship,
                ),
            )
            .into_iter()
            .map(|target| target.id.qualified_name.as_str())
            .collect::<Vec<_>>(),
        vec!["Parts::Part"]
    );

    let usage_id = graph
        .node_ids_for_qualified_name("Demo::vehicle")
        .and_then(|ids| ids.first())
        .expect("Vehicle usage");
    let usage = graph.get_node(usage_id).expect("usage node");
    assert!(matches!(
        graph.universal_relationship_resolution_for(usage),
        DerivedRelationshipResolution::Resolved { ref target }
            if target.qualified_name == "Parts::parts"
    ));
    assert_eq!(
        graph
            .outgoing_targets_by_kind_and_provenance(
                usage,
                RelationshipKind::Subsetting,
                RelationshipProvenance::Implied(
                    ImpliedRelationshipRule::UniversalStandardLibraryRelationship,
                ),
            )
            .len(),
        1
    );
}

#[test]
fn missing_or_nonstandard_library_targets_never_resolve() {
    let workspace = document(
        "workspace.sysml",
        "package Demo { part def Vehicle; }",
        SysmlDocumentSourceKind::Workspace,
    );
    let custom_library = document(
        "custom.sysml",
        "package Parts { part def Part; }",
        SysmlDocumentSourceKind::Library,
    );
    let (graph, _) = build_and_link_graph(&[workspace, custom_library]).expect("graph");

    assert_eq!(
        graph.universal_relationship_resolution_for(vehicle(&graph)),
        DerivedRelationshipResolution::MissingPrerequisite {
            target: StandardLibraryElement::PartsPart,
        }
    );
}

#[test]
fn duplicate_standard_library_targets_are_deterministically_ambiguous() {
    let workspace = document(
        "workspace.sysml",
        "package Demo { part def Vehicle; }",
        SysmlDocumentSourceKind::Workspace,
    );
    let first = document(
        "standard-a.sysml",
        "package Parts { part def Part; }",
        SysmlDocumentSourceKind::StandardLibrary,
    );
    let second = document(
        "standard-b.sysml",
        "package Parts { part def Part; }",
        SysmlDocumentSourceKind::StandardLibrary,
    );
    let (graph, _) = build_and_link_graph(&[workspace, first, second]).expect("graph");

    assert!(matches!(
        graph.universal_relationship_resolution_for(vehicle(&graph)),
        DerivedRelationshipResolution::Ambiguous { ref candidates }
            if candidates.len() == 2
                && candidates[0].uri.as_str() < candidates[1].uri.as_str()
    ));
}

#[test]
fn removal_republishes_missing_prerequisite_instead_of_stale_success() {
    let workspace = document(
        "workspace.sysml",
        "package Demo { part def Vehicle; }",
        SysmlDocumentSourceKind::Workspace,
    );
    let standard_library = document(
        "standard.sysml",
        "package Parts { part def Part; }",
        SysmlDocumentSourceKind::StandardLibrary,
    );
    let standard_uri = standard_library.uri.clone();
    let (mut graph, _) = build_and_link_graph(&[workspace, standard_library]).expect("graph");
    patch_graph_for_document(&mut graph, &standard_uri, None, true);

    assert_eq!(
        graph.universal_relationship_resolution_for(vehicle(&graph)),
        DerivedRelationshipResolution::MissingPrerequisite {
            target: StandardLibraryElement::PartsPart,
        }
    );
}

#[test]
fn self_target_and_authored_equivalent_are_never_replaced_or_duplicated() {
    let standard_library = document(
        "standard.sysml",
        "package Parts { part def Part; }",
        SysmlDocumentSourceKind::StandardLibrary,
    );
    let (self_graph, _) = build_and_link_graph(&[standard_library.clone()]).expect("graph");
    let self_target = self_graph
        .node_ids_for_qualified_name("Parts::Part")
        .and_then(|ids| ids.first())
        .and_then(|id| self_graph.get_node(id))
        .expect("standard part");
    assert!(matches!(
        self_graph.universal_relationship_resolution_for(self_target),
        DerivedRelationshipResolution::SelfTargetSuppressed { .. }
    ));

    let workspace = document(
        "workspace.sysml",
        "package Demo { part def Vehicle :> Parts::Part; }",
        SysmlDocumentSourceKind::Workspace,
    );
    let (graph, _) = build_and_link_graph(&[workspace, standard_library]).expect("graph");
    let definition = vehicle(&graph);
    assert!(matches!(
        graph.universal_relationship_resolution_for(definition),
        DerivedRelationshipResolution::Resolved { .. }
    ));
    assert_eq!(
        graph
            .outgoing_targets_by_kind_and_provenance(
                definition,
                RelationshipKind::Specializes,
                RelationshipProvenance::Authored,
            )
            .len(),
        1
    );
    assert!(graph
        .outgoing_targets_by_kind_and_provenance(
            definition,
            RelationshipKind::Specializes,
            RelationshipProvenance::Implied(
                ImpliedRelationshipRule::UniversalStandardLibraryRelationship,
            ),
        )
        .is_empty());
}

#[test]
fn full_and_parallel_publications_are_semantically_identical() {
    let workspace = document(
        "workspace.sysml",
        "package Demo { part def Vehicle; part vehicle; }",
        SysmlDocumentSourceKind::Workspace,
    );
    let standard_library = document(
        "standard.sysml",
        "package Parts { part def Part; part parts; }",
        SysmlDocumentSourceKind::StandardLibrary,
    );
    let (sequential, _) =
        build_and_link_graph(&[workspace.clone(), standard_library.clone()]).expect("graph");
    let (parallel, _) = build_and_link_graph_parallel(&[workspace, standard_library]);
    assert_eq!(sequential.to_semantic_sexpr(), parallel.to_semantic_sexpr());
}
