//! Contract tests for the navigation phase, driven through the crate's public
//! `build()` / `PublishedResolution` surface. Relocated verbatim from the inline
//! `#[cfg(test)]` modules of `src/lib.rs` and `src/model.rs`.

#![allow(clippy::too_many_lines)]

mod common;

#[allow(unused_imports)]
use common::*;
#[allow(unused_imports)]
use sysml_resolution::*;

#[test]
fn connection_usage_connector_end_references_resolve_to_their_targets() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconnection def C;\n\
         \tpart d1;\n\
         \tpart d2;\n\
         \tconnection bus : C connect d1 to d2;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::bus\"))) (kind connection)"),
        "expected a connection usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::bus\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d1\")))"
        ),
        "expected bus's connector-end reference to d1 to resolve, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::bus\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d2\")))"
        ),
        "expected bus's connector-end reference to d2 to resolve, got:\n{output}"
    );
}

#[test]
fn interface_def_connect_stmt_connector_end_references_resolve_to_their_targets() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart d1;\n\
         \tpart d2;\n\
         \tinterface def I {\n\
         \t\tconnect d1 to d2;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::I\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d1\")))"
        ),
        "expected I's connector-end reference to d1 to resolve, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::I\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d2\")))"
        ),
        "expected I's connector-end reference to d2 to resolve, got:\n{output}"
    );
}

#[test]
fn constraint_literal_only_comparison_is_supported_with_no_operand_references() {
    let request = sysml_resolution::BuildRequest::new(
        vec![sysml_resolution::SourceInput::new(
            "memory://test/enum.sysml",
            "package Demo {\n\
             \tconstraint def C { 1 < 2 }\n\
             }\n"
            .to_string(),
            sysml_resolution::SourceKind::Workspace,
        )],
        sysml_resolution::ConstructionSchedule::Sequential,
        "test-contract-v1",
    )
    .unwrap();
    let published = sysml_resolution::build(request).unwrap();
    let mut output = String::new();
    published
        .debug()
        .write_diagnostics_sexpr(&mut output)
        .unwrap();
    assert!(
        !output.contains("unsupported_constraint_definition_member"),
        "did not expect an unsupported constraint-definition-member diagnostic for a \
         literal-only comparison, got:\n{output}"
    );
}

#[test]
fn constraint_arithmetic_operand_resolves_all_leaf_references() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute a : ScalarValues::Integer;\n\
         \tattribute b : ScalarValues::Integer;\n\
         \tattribute c : ScalarValues::Integer;\n\
         \tconstraint def C { (a + b) < c }\n\
         }\n",
    );
    for name in ["a", "b", "c"] {
        assert!(
            output.contains(&format!(
                "(authored-target \"{name}\")\n      (outcome (status resolved) (target \
                 (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::{name}\")))))"
            )),
            "expected operand `{name}` in `(a + b) < c` to resolve to its sibling attribute \
             declaration, got:\n{output}"
        );
    }
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state non-constant))"
        ),
        "expected `(a + b) < c` with no constant-valued operands to publish NonConstant \
         rather than a fabricated boolean, got:\n{output}"
    );
}

#[test]
fn purpose_member_resolves_its_concern_target() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconcern modularity;\n\
         \tviewpoint def SystemView {\n\
         \t\tpurpose modularity;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind purposeTarget)"),
        "expected a purposeTarget reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::modularity\")))"
        ),
        "expected the purpose reference to resolve to modularity, got:\n{output}"
    );
}

#[test]
fn verify_requirement_shorthand_resolves_its_target() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trequirement speedRequirement;\n\
         \trequirement def CheckSpeed {\n\
         \t\tverify speedRequirement;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind verify-requirement)"),
        "expected a verify-requirement declaration, got:\n{output}"
    );
    assert!(
        output.contains("(kind verifyRequirementTarget)"),
        "expected a verifyRequirementTarget reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::speedRequirement\")))"
        ),
        "expected the verify target to resolve to speedRequirement, got:\n{output}"
    );
}

#[test]
fn kerml_feature_member_redefines_resolves_its_target() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tderived feature base : Integer;\n\
         \tderived feature x redefines base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::base\")))"
        ),
        "expected x's redefinition of base to resolve, got:\n{output}"
    );
}

#[test]
fn filter_and_expression_resolves_both_metadata_test_and_operand_references() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tmetadata def Safety {\n\
         \t\tattribute isMandatory : Boolean;\n\
         \t}\n\
         \tpackage 'Mandatory Safety Features' {\n\
         \t\tpublic import Demo::**;\n\
         \t\tfilter @Safety and Safety::isMandatory;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind filterMetadataTest) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Mandatory Safety Features\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Safety\")))"
        ),
        "expected the filter's @Safety metadata-test reference to resolve, got:\n{output}"
    );
    // `Safety::isMandatory`'s second segment is a `metadata def`-owned attribute with default
    // (non-package-owner) visibility, so it is not effective-public and the qualified lexical
    // lookup's second segment finds no exported candidate -- exactly the same shape
    // `40_filtering_example_1.md`'s real-corpus fixture exercises (see `Safety::isMandatory`'s
    // own `featureTyping` reference staying unresolved there too). The reference is still
    // authored, sourced, and explicitly unresolved rather than silently dropped or unsupported.
    assert!(
        output.contains("(kind expressionOperand)")
            && output.contains("(authored-target \"Safety::isMandatory\")")
            && output.contains("(status unresolved)"),
        "expected the filter's Safety::isMandatory operand reference to be resolved-attempted \
         and stay explicitly unresolved (not unsupported), got:\n{output}"
    );
}

/// A `transition t first s1 then s2;` body element's `source`/`target` operands now resolve
/// to their sibling state declarations (this task picks up the full `transition` construct
/// explicitly deferred by `4762b875`), so it no longer surfaces as an explicit unsupported
/// state-definition-member diagnostic. See `TransitionEffect`/`TransitionAccept`-specific
/// unsupported sub-piece coverage in `lib.rs`'s `transition_*` tests for what remains
/// deliberately out of scope (typed `accept` payload declarations, time triggers, and the
/// richer `Accept`/`Send`/`Assign` effect shapes).
#[test]
fn transition_inside_a_state_def_resolves_source_and_target() {
    let request = sysml_resolution::BuildRequest::new(
        vec![sysml_resolution::SourceInput::new(
            "memory://test/enum.sysml",
            "package Demo {\n\
             \tstate def SD {\n\
             \t\tstate s1;\n\
             \t\tstate s2;\n\
             \t\ttransition t first s1 then s2;\n\
             \t}\n\
             }\n"
            .to_string(),
            sysml_resolution::SourceKind::Workspace,
        )],
        sysml_resolution::ConstructionSchedule::Sequential,
        "test-contract-v1",
    )
    .unwrap();
    let published = sysml_resolution::build(request).unwrap();
    let mut output = String::new();
    published
        .debug()
        .write_diagnostics_sexpr(&mut output)
        .unwrap();
    assert!(
        !output.contains("unsupported_state_definition_member"),
        "did not expect an unsupported state-definition-member diagnostic for a fully \
         resolvable transition, got:\n{output}"
    );
    let mut semantic = String::new();
    published
        .debug()
        .write_semantic_sexpr(&mut semantic)
        .unwrap();
    assert!(
        semantic.contains("(kind transitionSource)")
            && semantic.contains("(kind transitionTarget)"),
        "expected transitionSource/transitionTarget relationship kinds, got:\n{semantic}"
    );
}

#[test]
fn satisfy_inside_a_part_usage_resolves_source_and_target() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trequirement def R;\n\
         \tpart def P;\n\
         \trequirement r : R;\n\
         \tpart p : P {\n\
         \t\tsatisfy r by p;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind satisfySource)") && output.contains("(kind satisfyTarget)"),
        "expected satisfySource/satisfyTarget relationship kinds, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind satisfy) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::p::satisfy\""
        ) || output.contains("(kind satisfy)"),
        "expected an owned satisfy declaration, got:\n{output}"
    );
    assert!(
        !output.contains("(status unresolved)"),
        "expected both satisfy operands to resolve, got:\n{output}"
    );
}

#[test]
fn allocate_statement_inside_a_part_usage_resolves_source_and_target() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def A;\n\
         \tpart def B;\n\
         \tpart a : A;\n\
         \tpart b : B {\n\
         \t\tallocate a to b;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind allocateSource)") && output.contains("(kind allocateTarget)"),
        "expected allocateSource/allocateTarget relationship kinds, got:\n{output}"
    );
    assert!(
        output.contains("(kind allocate)"),
        "expected an owned allocate declaration, got:\n{output}"
    );
    assert!(
        !output.contains("(status unresolved)"),
        "expected both allocate operands to resolve, got:\n{output}"
    );
}

#[test]
fn bind_statement_inside_a_part_usage_resolves_source_and_target() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def A;\n\
         \tpart def B;\n\
         \tpart a : A;\n\
         \tpart b : B {\n\
         \t\tbind a = b;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind bindSource)") && output.contains("(kind bindTarget)"),
        "expected bindSource/bindTarget relationship kinds, got:\n{output}"
    );
    assert!(
        output.contains("(kind bind)"),
        "expected an owned bind declaration, got:\n{output}"
    );
    assert!(
        !output.contains("(status unresolved)"),
        "expected both bind operands to resolve, got:\n{output}"
    );
}

#[test]
fn use_case_include_resolves_its_target_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tuse case def UsedUseCase;\n\
         \tuse case def MainUseCase {\n\
         \t\tinclude UsedUseCase;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind includeUseCase)"),
        "expected an includeUseCase relationship kind, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind includeUseCase) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MainUseCase\""
        ),
        "expected the includeUseCase reference to be sourced at the enclosing use case \
         declaration (no anonymous nested-declaration scope shift), got:\n{output}"
    );
    assert!(
        output.contains("(authored-target \"UsedUseCase\")"),
        "expected the include target to be authored, got:\n{output}"
    );
    assert!(
        !output.contains("(status unresolved)"),
        "expected the include target to resolve, got:\n{output}"
    );
}

#[test]
fn ref_decl_resolves_combined_redefines_and_subsets_references_independently() {
    // GH-51: a single `ref` can carry both an explicit `:>>` redefines clause and a `:>`
    // subsets clause at once, e.g. `ref requirement originalRequirement[1] :>>
    // originalRequirements :> participant { ... }` (Systems Library `Domain Libraries/
    // Requirement Derivation/DerivationConnections.sysml`). `lower_ref_decl` already checks
    // `node.value.redefines` and `node.value.subsets` as two independent `if let`s (not an
    // `if`/`else if`), so both references are expected to resolve independently -- this test
    // locks that in.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trequirement def Req {\n\
         \t\trequirement participant;\n\
         \t\trequirement original;\n\
         \t}\n\
         \tconnection def C :> Req {\n\
         \t\tref requirement r :>> original :> participant;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind redefinition)"),
        "expected a redefinition relationship kind for the ref's `:>>` clause, got:\n{output}"
    );
    assert!(
        output.contains("(kind subsetting)"),
        "expected a subsetting relationship kind for the ref's `:>` clause, got:\n{output}"
    );
    assert!(
        output.contains("(authored-target \"original\")"),
        "expected the ref's redefines target to be authored, got:\n{output}"
    );
    assert!(
        output.contains("(authored-target \"participant\")"),
        "expected the ref's subsets target to be authored, got:\n{output}"
    );
}

#[test]
fn value_assignment_istype_resolves_operand_and_type_target() {
    // `Expression::TypeCheck` (`x istype T`) resolves the operand through the ordinary
    // ExpressionOperand recursion and the `T` target through the new TypeCheckTarget
    // reference, mirroring `AcceptPayloadType`/`FilterMetadataTest`'s Type-domain lookup.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute a : ScalarValues::Integer;\n\
         \tclass T;\n\
         \tattribute check = a istype T;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"a\")\n      \
             (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::a\")))))"
        ),
        "expected `a` to resolve as an expressionOperand reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typeCheckTarget) (ordinal 0))\n      (authored-target \"T\")\n      \
             (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::T\")))))"
        ),
        "expected `T` to resolve as a typeCheckTarget reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::check\"))) (state non-constant))"
        ),
        "expected `a istype T` to publish NonConstant (no runtime type info available), \
         got:\n{output}"
    );
}

#[test]
fn value_assignment_hastype_resolves_operand_and_type_target() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute a : ScalarValues::Integer;\n\
         \tclass T;\n\
         \tattribute check = a hastype T;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"a\")\n      \
             (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::a\")))))"
        ),
        "expected `a` to resolve as an expressionOperand reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typeCheckTarget) (ordinal 0))\n      (authored-target \"T\")\n      \
             (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::T\")))))"
        ),
        "expected `T` to resolve as a typeCheckTarget reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::check\"))) (state non-constant))"
        ),
        "expected `a hastype T` to publish NonConstant (no runtime type info available), \
         got:\n{output}"
    );
}

#[test]
fn value_assignment_meta_cast_resolves_base_and_metaclass_target() {
    // `Expression::MetaCast` (`Base meta Ns::Metaclass`) resolves the base operand through
    // the ordinary ExpressionOperand recursion and the qualified `Ns::Metaclass` target
    // through the new MetaCastTarget reference, mirroring `TypeCheckTarget`'s Type-domain
    // lookup and supporting a multi-segment qualified reference exactly like other
    // Type-domain targets (e.g. `KerML::Classifier`).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpackage Meta {\n\
         \t\tclass Classifier;\n\
         \t}\n\
         \tattribute a : ScalarValues::Integer;\n\
         \tattribute check = a meta Meta::Classifier;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"a\")\n      \
             (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::a\")))))"
        ),
        "expected `a` to resolve as an expressionOperand reference, got:\n{output}"
    );
    assert!(
        output.contains("(kind metaCastTarget)")
            && output.contains(
                "(outcome (status resolved) (target (node (document \
                 \"memory://test/enum.sysml\") (qualified-name \"Demo::Meta::Classifier\")))))"
            ),
        "expected `Meta::Classifier` to resolve as a metaCastTarget reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::check\"))) (state non-constant))"
        ),
        "expected `a meta Meta::Classifier` to publish NonConstant (denotes a metaclass \
         relationship, not a computable scalar value), got:\n{output}"
    );
}

#[test]
fn construction_schedule_does_not_change_semantic_publication_identity() {
    let sequential =
        BuildRequest::new(Vec::new(), ConstructionSchedule::Sequential, "contract-v1").unwrap();
    let parallel =
        BuildRequest::new(Vec::new(), ConstructionSchedule::Parallel, "contract-v1").unwrap();

    assert_eq!(sequential.identity(), parallel.identity());
}

#[test]
fn state_transition_scene_owns_vertices_and_composed_transitions() {
    let request = BuildRequest::new(
        vec![
            SourceInput::new(
                "memory://standard-views.sysml",
                "standard library package StandardViewDefinitions { view def StateTransitionView; }".to_owned(),
                SourceKind::StandardLibrary,
            ),
            SourceInput::new(
                "memory://timer.sysml",
                concat!(
                    "package Timer { import StandardViewDefinitions::*; item def StartPressed; ",
                    "state def Machine { entry; then idle; state idle; state running; ",
                    "transition start first idle accept StartPressed then running; } ",
                    "view stateView : StateTransitionView { expose Machine; } }",
                ).to_owned(),
                SourceKind::Workspace,
            ),
        ],
        ConstructionSchedule::Sequential,
        "contract-v1",
    ).unwrap();
    let published = build(request).unwrap();
    let catalog = match published.diagram_view_catalog() {
        QueryOutcome::Resolved(catalog) => catalog,
        other => panic!("expected diagram catalog, got {other:?}"),
    };
    let view = catalog
        .iter()
        .find(|view| view.kind == DiagramViewKind::StateTransition)
        .unwrap();
    let projection = match published.diagram_view(view.semantic_id) {
        QueryOutcome::Resolved(projection) => projection,
        other => panic!("expected state scene, got {other:?}"),
    };
    let DiagramScene::StateTransition(scene) = projection.scene else {
        panic!("expected typed State Transition scene");
    };
    assert_eq!(
        scene
            .vertices
            .iter()
            .filter(|vertex| vertex.kind == DiagramStateVertexKind::Initial)
            .count(),
        1
    );
    assert_eq!(
        scene
            .vertices
            .iter()
            .filter(|vertex| vertex.kind == DiagramStateVertexKind::State)
            .count(),
        2
    );
    assert_eq!(scene.transitions.len(), 2);
    assert!(scene.transitions.iter().any(|transition| matches!(
        &transition.trigger,
        DiagramTransitionFeature::Resolved { label, .. } if label.as_ref() == "StartPressed"
    )));
    assert!(!scene
        .vertices
        .iter()
        .any(|vertex| vertex.label.as_ref() == "start"));
}

#[test]
fn diagram_projection_keeps_inherited_features_distinct_in_each_usage_context() {
    let request = BuildRequest::new(
        vec![
            SourceInput::new(
                "memory://standard-views.sysml",
                "standard library package StandardViewDefinitions { view def GeneralView; }"
                    .to_owned(),
                SourceKind::StandardLibrary,
            ),
            SourceInput::new(
                "memory://model.sysml",
                concat!(
                    "package Model { import StandardViewDefinitions::*; ",
                    "part def Board; part def Module { part pcb : Board; part spare : Board; connection wire connect pcb to spare; } ",
                    "part def Assembly { part left : Module; part right : Module; } ",
                    "part root : Assembly; view structure : GeneralView { expose root; } }",
                )
                .to_owned(),
                SourceKind::Workspace,
            ),
        ],
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .unwrap();
    let published = build(request).unwrap();
    let catalog = match published.diagram_view_catalog() {
        QueryOutcome::Resolved(catalog) | QueryOutcome::UnsupportedWith(catalog) => catalog,
        other => panic!("expected diagram catalog, got {other:?}"),
    };
    let view = catalog
        .iter()
        .find(|view| view.kind == DiagramViewKind::General)
        .unwrap();
    let projection = match published.diagram_view(view.semantic_id) {
        QueryOutcome::Resolved(projection) => projection,
        other => panic!("expected General View projection, got {other:?}"),
    };

    let pcbs = projection
        .elements
        .iter()
        .filter(|element| element.name.as_deref() == Some("pcb"))
        .collect::<Vec<_>>();
    assert_eq!(
        pcbs.len(),
        2,
        "one declaration must occur under both module usages"
    );
    assert_eq!(pcbs[0].semantic_id, pcbs[1].semantic_id);
    assert_ne!(pcbs[0].occurrence_id, pcbs[1].occurrence_id);
    assert_ne!(pcbs[0].owner, pcbs[1].owner);
    assert!(pcbs
        .iter()
        .all(|pcb| pcb.occurrence_id.semantic_path.len() == 3));

    let connectors = projection
        .edges
        .iter()
        .filter(|edge| edge.kind == DiagramEdgeKind::Connector)
        .collect::<Vec<_>>();
    assert_eq!(
        connectors.len(),
        2,
        "the inherited connector occurs in both modules"
    );
    assert_eq!(
        connectors[0].source_semantic_id,
        connectors[1].source_semantic_id
    );
    assert_eq!(
        connectors[0].target_semantic_id,
        connectors[1].target_semantic_id
    );
    assert_ne!(connectors[0].source, connectors[1].source);
    assert_ne!(connectors[0].target, connectors[1].target);
}

/// Reporting a document changes the answer, so it changes the publication's identity.
#[test]
fn the_reported_document_set_is_part_of_the_publication_identity() {
    let request = || {
        BuildRequest::new(
            vec![SourceInput::new(
                "memory://workspace.sysml",
                "package W { part w; }".to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap()
    };
    let plain = request();
    let reporting = request().reporting([Box::from("memory://lib.sysml")]);
    assert_ne!(plain.identity(), reporting.identity());
    assert_eq!(
        reporting.identity().reported_documents(),
        [Box::<str>::from("memory://lib.sysml")]
    );
}

#[test]
fn a_document_query_answers_from_the_publication_index_and_repeats_identically() {
    let published = published_for("package P { part def A; part def A; part b; }");
    let first = published.document_diagnostics("memory://test.sysml");
    let second = published.document_diagnostics("memory://test.sysml");
    assert_eq!(first, second, "a repeated query returns identical values");
    assert_eq!(
        first.diagnostics.as_ref(),
        published.diagnostics().diagnostics.as_ref(),
        "the document slice is the publication's own sequence"
    );
    let absent = published.document_diagnostics("memory://absent.sysml");
    assert!(absent.diagnostics.is_empty());
    assert_eq!(
        absent.completeness, first.completeness,
        "completeness travels with the answer even when there is nothing to report"
    );
}

/// A `transition ... first X then Y;` body element's `source`/`target` operands must each
/// resolve to their sibling state declarations, not fall through to
/// `unsupported_state_definition_member`.
#[test]
fn transition_source_and_target_resolve() {
    let sexpr = semantic_sexpr_for(
        "package P { state def S { state off; state on; transition first off then on; } }",
    );
    assert!(
        sexpr.contains("(kind transitionSource)"),
        "expected a transitionSource relationship kind, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind transitionTarget)"),
        "expected a transitionTarget relationship kind, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected both transition ends to resolve to their sibling state declarations, got: {sexpr}"
    );
}

/// A bare `then <target>;` continuation (`ThenTarget::Feature`) referencing an
/// already-declared sibling action must resolve as a `thenTarget` reference sourced at the
/// enclosing action, not fall through to `unsupported_action_definition_member`.
#[test]
fn then_target_feature_resolves() {
    let sexpr = semantic_sexpr_for("package P { action def A { action x; then x; } }");
    assert!(
        sexpr.contains("(kind thenTarget)"),
        "expected a thenTarget relationship kind, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected the `then` target to resolve to its sibling action, got: {sexpr}"
    );
}

/// A standalone `action <name> send via <source> to <target>;` action-usage shorthand (an
/// `ActionUsage` with `send`/`via`/`to` all set on the usage itself, distinct from the
/// `then send ...;` continuation form blocked by planning/UPSTREAM_PARSER_GAPS.md Gap 30) must resolve
/// its `via`/`to` operands, mirroring satisfy/allocate/bind's two-operand pattern via
/// `lower_satisfy_operand`.
#[test]
fn send_action_usage_via_and_to_targets_resolve() {
    let sexpr = semantic_sexpr_for(
        "package P { action def A { action aa; action b; action snd2 send via aa to b; } }",
    );
    assert!(
        sexpr.contains("(kind sendTarget)"),
        "expected a sendTarget reference for the `to b` clause, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind acceptVia)"),
        "expected an acceptVia reference for the `via aa` clause, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected both the send usage's `via` and `to` targets to resolve, got: {sexpr}"
    );
}

/// The `then send new S() to b;` continuation shorthand (formerly parser Gap 30) now parses as
/// `ThenTarget::Send`, carrying the same `ActionUsage` shape a standalone `send ...;` statement
/// produces, so it lowers through `lower_action_usage` exactly like `then action ...;` does:
/// the payload constructor resolves as an `invocationCallee` and the `to` clause as a
/// `sendTarget`, and nothing is left to parser recovery.
#[test]
fn then_send_continuation_resolves_payload_and_target() {
    let sexpr = semantic_sexpr_for(
        "package P { attribute def S; action def A { action b; then send new S() to b; } }",
    );
    assert!(
        !sexpr.contains("(completeness parse-recovery)"),
        "expected `then send ...;` to parse, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind invocationCallee)") && sexpr.contains("(kind sendTarget)"),
        "expected the send payload and target to resolve as references, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// A bare `flow <source> to <target>;` statement (distinct from a named/typed flow usage or
/// def) must lower as its own `DeclarationKind::Flow` feature. Its ends are typed
/// `KermlConnectorEnd`s upstream -- the same connector-end shape the KerML connector, binding
/// and succession members carry -- so each resolves directly as a `flowSource`/`flowTarget`
/// reference to the feature it names, including the dotted feature-chain spelling.
#[test]
fn bare_flow_stmt_resolves_source_and_target() {
    let sexpr = semantic_sexpr_for(
        "package P { action def A { action aa { out part target; } action snd { in receiver; } flow aa.target to snd.receiver; } }",
    );
    assert!(
        sexpr.contains("(kind flow)"),
        "expected a flow declaration, got: {sexpr}"
    );
    assert!(
        sexpr.contains(
            "(kind flowSource) (ordinal 0))\n      (authored-target \"aa::target\")\n      (outcome (status resolved)"
        ) && sexpr.contains(
            "(kind flowTarget) (ordinal 0))\n      (authored-target \"snd::receiver\")\n      (outcome (status resolved)"
        ),
        "expected both flow ends to resolve to the features they name, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// `terminate <name>;` nested inside a `then action <name> { ... }` self-named action usage
/// (the representative fixture shape, e.g. `then action c1 { terminate c1; }`) must resolve
/// its target through the shared `DeclarationDomain::Any` lexical lookup, sourced directly at
/// the enclosing action usage's own declaration (not an anonymous nested one): the terminate
/// statement's own enclosing scope is the action usage's *parent*'s children, where its own
/// self-name is declared -- a genuine self-termination idiom.
#[test]
fn terminate_stmt_with_target_resolves() {
    let sexpr =
        semantic_sexpr_for("package P { action def A { then action c1 { terminate c1; } } }");
    assert!(
        sexpr.contains("(kind terminateTarget)"),
        "expected a terminateTarget reference, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected the terminate target to resolve to its enclosing self-named action, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// An `assign <target> := <value>;` reassignment statement must lower as an anonymous
/// `assign` declaration whose `lhs` resolves as an `assignTarget` reference to its sibling
/// action and whose `rhs` value expression resolves/evaluates, not fall through to
/// `unsupported_action_definition_member`.
#[test]
fn assign_stmt_target_and_value_resolve() {
    let sexpr = semantic_sexpr_for("package P { action def A { action x; assign x := 5; } }");
    assert!(
        sexpr.contains("(kind assign)"),
        "expected an assign declaration, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind assignTarget)"),
        "expected an assignTarget relationship kind, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected the assign target to resolve to its sibling action, got: {sexpr}"
    );
}

/// `PerformBodyElement::InOut` (BNF `PerformInOutBinding`, the `in`/`out <target> = <value>;`
/// parameter-argument-binding shorthand used when invoking a nested `perform action`, e.g.
/// `perform action dynamics : StraightLineDynamics { in power = vehiclePower; }`) was
/// unconditionally unsupported -- wires it via `lower_perform_inout_binding`.
#[test]
fn perform_inout_binding_resolves_target_and_value() {
    let sexpr = semantic_sexpr_for(
        "package P { action def A { in power; perform action dynamics : A { in power = vehiclePower; } } action def Outer { attribute vehiclePower; } }",
    );
    assert!(
        sexpr.contains("(kind perform-parameter-binding)"),
        "expected an anonymous perform-parameter-binding declaration, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind performParameterTarget)"),
        "expected the `in power` target to resolve as performParameterTarget, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_usage_member"),
        "did not expect unsupported_action_usage_member, got: {sexpr}"
    );
}

/// `lower_succession_end` (used for `AssignTarget` among others) handled `Expression::
/// MemberAccess` but not the sibling `Expression::FeatureChainRef` shape the parser actually
/// produces for a dotted assign target (e.g. `assign a.b := 1;`), mirroring the fix already
/// applied to `lower_satisfy_operand`.
#[test]
fn assign_target_dotted_feature_chain_resolves() {
    let sexpr = semantic_sexpr_for(
        "package P { part def A { part def B { attribute count; } part b : B; } action def Act { part a : A; assign a.b.count := 1; } }",
    );
    assert!(
        sexpr.contains("(kind memberAccessOperand)"),
        "expected the dotted assign target to resolve as memberAccessOperand, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// Anonymous ordinals are allocated per `(document, owner, kind)`, so an identity that named
/// only the kind and ordinal could not tell two same-kind anonymous declarations under
/// different owners apart. The identity spells out the owner chain for exactly this reason.
#[test]
fn anonymous_declarations_under_different_owners_get_distinct_identities() {
    let sexpr = semantic_sexpr_for(
        "package P { action def A { action x; if x { action y; } else { action z; } } action def B { action x; if x { action y; } else { action z; } } }",
    );
    assert!(
        sexpr.contains(r#"(path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind if) (ordinal 0)))"#),
        "expected the if-scope under A to carry its owner in its identity, got: {sexpr}"
    );
    assert!(
        sexpr.contains(r#"(path (named (kind package) (name "P")) (named (kind action-def) (name "B")) (anonymous (kind if) (ordinal 0)))"#),
        "expected the if-scope under B to carry its owner in its identity, got: {sexpr}"
    );
}

/// The identity is structural, so editing an unrelated document cannot change it. A dense
/// storage ordinal would shift as soon as any earlier document gained a declaration.
///
/// The structural identity is the *token*: a `SymbolId` is a rank within one publication and is
/// deliberately not comparable across builds, which is why anything that has to survive an edit
/// takes a token first.
#[test]
fn element_identity_survives_an_edit_to_an_unrelated_document() {
    let before = publication_for(&[
        ("memory://a.sysml", "package A { part def Wheel; }"),
        ("memory://b.sysml", "package B { part def Engine; }"),
    ]);
    let after = publication_for(&[
        (
            "memory://a.sysml",
            "package A { part def Wheel; part def Axle; part def Frame; }",
        ),
        ("memory://b.sysml", "package B { part def Engine; }"),
    ]);

    let engine_before = before
        .symbol_token(target_symbol(&before, "memory://b.sysml", 0, 21))
        .expect("expected a token for a symbol this publication just answered with");
    let engine_after = after
        .symbol_token(target_symbol(&after, "memory://b.sysml", 0, 21))
        .expect("expected a token for a symbol this publication just answered with");
    assert_eq!(
        engine_before.as_str(),
        engine_after.as_str(),
        "expected an unrelated document's edit to leave this element's identity unchanged"
    );
}

/// Two identically named siblings of the same kind are distinguished by an occurrence ordinal,
/// so each remains addressable. The Pilot does the same: its `qualifiedName` derivation yields
/// null for every same-named member after the first, and `path()` then falls through to a
/// positional form.
///
/// The first occurrence keeps the plain name, so authoring a duplicate later never disturbs
/// the identity already published for the declaration that was there first.
#[test]
fn duplicate_sibling_names_stay_separately_addressable() {
    let published = publication_for(&[(
        "memory://dup.sysml",
        "package P { part def Failure; part def Failure; }",
    )]);

    let first = target_symbol(&published, "memory://dup.sysml", 0, 21);
    let second = target_symbol(&published, "memory://dup.sysml", 0, 39);
    assert_ne!(
        first, second,
        "expected identically named siblings to carry distinct identities"
    );

    for symbol in [first, second] {
        match published.references(symbol, true) {
            QueryOutcome::Resolved(locations) => assert_eq!(
                locations.len(),
                1,
                "expected each sibling to resolve to its own declaration site"
            ),
            other => panic!("expected a resolved references outcome, got: {other:?}"),
        }
    }
}

/// The facts an inspector needs arrive as one typed answer, each from its own producer --
/// no attribute map, and nothing recovered by re-reading source text.
#[test]
fn inspection_publishes_every_authored_fact_of_an_element() {
    let published = publication_for(&[(
        "memory://i.sysml",
        "package P {\n  part def Wheel;\n  /* doc */\n  part def Car {\n    doc /* the car */\n    part wheels : Wheel[0..4] ordered;\n  }\n}",
    )]);
    let wheels = inspect_named(&published, "memory://i.sysml", 5, 9);

    assert_eq!(wheels.kind, ElementKind::PartUsage);
    assert_eq!(wheels.name.as_deref(), Some("wheels"));
    assert_eq!(&*wheels.qualified_name, "P::Car::wheels");
    assert_eq!(wheels.membership.kind, MembershipKind::Feature);
    assert_eq!(
        wheels.membership.provenance,
        VisibilityProvenance::Default,
        "no visibility keyword was written, so the default applies and says so"
    );
    assert_eq!(
        wheels.multiplicity,
        MultiplicityFacts::Declared {
            lower: MultiplicityBound::Literal(0),
            upper: MultiplicityBound::Literal(4),
            ordered: true,
            nonunique: false,
        }
    );
    assert_eq!(&*wheels.modifiers, &[ElementModifier::Ordered]);
    assert_eq!(wheels.evaluation, EvaluationState::NotApplicable);

    let typing = wheels
        .relationships
        .iter()
        .find(|relationship| relationship.kind == "featureTyping")
        .expect("expected a featureTyping relationship");
    assert_eq!(typing.authored.as_deref(), Some("Wheel"));
    assert_eq!(typing.provenance, RelationshipProvenance::Authored);
    assert!(matches!(typing.target, RelationshipTarget::Resolved(_)));

    let car = inspect_named(&published, "memory://i.sysml", 3, 11);
    assert_eq!(car.documentation.len(), 1, "expected the doc comment");
    assert_eq!(&*car.documentation[0].text, " the car ");
    assert_eq!(car.documentation[0].form, AnnotationForm::Documentation);
}

/// A position identifies two different elements, and an inspector needs both: the declaration
/// the cursor sits in, and what the reference under it points at.
#[test]
fn inspect_at_reports_the_containing_and_the_referenced_element() {
    let published = publication_for(&[(
        "memory://i.sysml",
        "package P {\n  part def Wheel;\n  part w : Wheel;\n}",
    )]);
    let at = match published.inspect_at(
        "memory://i.sysml",
        TextPosition {
            line: 2,
            character: 12,
        },
    ) {
        QueryOutcome::Resolved(at) => at,
        other => panic!("expected a resolved inspection, got: {other:?}"),
    };

    assert_eq!(
        at.containing.as_ref().and_then(|c| c.name.as_deref()),
        Some("w"),
        "the cursor sits inside `w`'s declaration"
    );
    match &at.referenced {
        ReferenceAt::Resolved(referenced) => assert_eq!(
            referenced.name.as_deref(),
            Some("Wheel"),
            "and points at a reference resolving to `Wheel`"
        ),
        other => panic!("expected a resolved reference at the position, got: {other:?}"),
    }
}

/// A qualifier is resolved through lexical scope indexes. Two same-name candidates therefore
/// remain explicitly ambiguous instead of having their members silently merged by display
/// name.
#[test]
fn visible_members_keeps_ambiguous_qualifier_scopes_separate() {
    let source = "package P { part def A; } package P { part def B; } package Use { part x; }";
    let published = publication_for(&[("memory://i.sysml", source)]);
    let outcome =
        published.visible_members("memory://i.sysml", position_of(source, "part x"), Some("P"));
    let QueryOutcome::Ambiguous(candidates) = outcome else {
        panic!("expected ambiguous qualifier scopes, got: {outcome:?}");
    };
    assert_eq!(candidates.len(), 2);
    let mut names = candidates
        .iter()
        .flat_map(|members| members.iter().map(|member| member.name.as_ref()))
        .collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(names, ["A", "B"]);
}

/// The outline lists every declaration in the document, including anonymous ones, each with
/// the identity that addresses it.
#[test]
fn document_symbols_lists_every_declaration_with_its_identity() {
    let published = publication_for(&[(
        "memory://i.sysml",
        "package P {\n  part def Wheel;\n  part w : Wheel;\n}",
    )]);
    let symbols = match published.document_symbols("memory://i.sysml") {
        QueryOutcome::Resolved(symbols) => symbols,
        other => panic!("expected resolved symbols, got: {other:?}"),
    };
    let names = symbols
        .iter()
        .filter_map(|entry| entry.name.as_deref())
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["P", "Wheel", "w"]);

    let wheel = symbols
        .iter()
        .find(|entry| entry.name.as_deref() == Some("Wheel"))
        .expect("Wheel");
    assert_eq!(wheel.kind, ElementKind::PartDefinition);
    assert!(
        matches!(
            published.inspect(wheel.identity),
            QueryOutcome::Resolved(_)
        ),
        "an outline entry's identity must address the same element"
    );
}

#[test]
fn typed_element_search_filters_by_kind_and_authored_source_in_canonical_order() {
    let request = BuildRequest::new(
        vec![
            SourceInput::new(
                "memory://z.sysml",
                "package Z { requirement def Later; part def NotARequirement; }".into(),
                SourceKind::Workspace,
            ),
            SourceInput::new(
                "memory://standard.sysml",
                "package Standard { requirement def LibraryRequirement; }".into(),
                SourceKind::StandardLibrary,
            ),
            SourceInput::new(
                "memory://a.sysml",
                "package A { requirement def First; requirement def Second; }".into(),
                SourceKind::Workspace,
            ),
        ],
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .expect("request");
    let published = build(request).expect("publication");

    let requirements = match published.search_elements(ElementSearch {
        kind: ElementKind::RequirementDefinition,
        source: ElementSource::Workspace,
    }) {
        QueryOutcome::Resolved(entries) => entries,
        other => panic!("expected resolved search, got: {other:?}"),
    };
    assert_eq!(
        requirements
            .iter()
            .map(|entry| (
                entry.location.document.as_ref(),
                entry.qualified_name.as_ref()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("memory://a.sysml", "A::First"),
            ("memory://a.sysml", "A::Second"),
            ("memory://z.sysml", "Z::Later"),
        ]
    );
    assert!(requirements
        .iter()
        .all(|entry| entry.kind == ElementKind::RequirementDefinition));

    let library = match published.search_elements(ElementSearch {
        kind: ElementKind::RequirementDefinition,
        source: ElementSource::StandardLibrary,
    }) {
        QueryOutcome::Resolved(entries) => entries,
        other => panic!("expected resolved search, got: {other:?}"),
    };
    assert_eq!(library.len(), 1);
    assert_eq!(
        library[0].qualified_name.as_ref(),
        "Standard::LibraryRequirement"
    );
}

#[test]
fn namespace_import_derived_elements_preserve_canonical_target_outcomes() {
    let published = detail_publication(
        &[
            (
                "memory://library.sysml",
                "package Library { part def Imported; }",
            ),
            (
                "memory://model.sysml",
                "package Model { import Library::*; part def Owned; }",
            ),
        ],
        ConstructionSchedule::Sequential,
    );
    let model = identity_of(&published, "memory://model.sysml", "Model");
    let library = identity_of(&published, "memory://library.sysml", "Library");
    let values = settled(published.namespace_import_derived_elements(model));
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].relationship.kind, "namespaceImport");
    assert_eq!(
        values[0].relationship.provenance,
        RelationshipProvenance::Authored
    );
    assert_eq!(
        values[0].relationship.target,
        RelationshipTarget::Resolved(library),
        "the scalar derivation must retain the import reference's canonical target outcome"
    );
}

/// Never select the first candidate of an ambiguous reference: every candidate is retained and
/// the outcome says none was chosen.
#[test]
fn an_ambiguous_relationship_target_keeps_every_candidate_and_chooses_none() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            concat!(
                "package P {\n",
                "  package A { part def Shared; }\n",
                "  package B { part def Shared; }\n",
                "  package C { import A::*; import B::*; part unit : Shared; }\n",
                "}\n",
            ),
        )],
        ConstructionSchedule::Sequential,
    );
    let usage = details_of(&published, "memory://model.sysml", "P::C::unit");
    assert_eq!(usage.typing.outcome, RelationshipOutcome::Ambiguous);
    assert!(
        usage.typing.targets.is_empty(),
        "an ambiguous family must publish no chosen target"
    );
    assert_eq!(usage.typing.candidates.len(), 2, "{:?}", usage.typing);
    assert_eq!(
        usage
            .effective_typing
            .candidates
            .iter()
            .map(|candidate| candidate.element.qualified_name.as_ref())
            .collect::<Vec<_>>(),
        vec!["P::A::Shared", "P::B::Shared"]
    );
    assert_eq!(
        usage.effective_typing.outcome,
        RelationshipOutcome::Ambiguous
    );
}

const VEHICLE_MODEL: &str = concat!(
    "package P {\n",
    "  metadata def Safety;\n",
    "  part def Wheel;\n",
    "  part def Vehicle {\n",
    "    @Safety;\n",
    "    part wheel[4] : Wheel;\n",
    "    part spare[0..*] : Wheel;\n",
    "  }\n",
    "  part def Rover :> Vehicle {\n",
    "    part :>> wheel[4];\n",
    "  }\n",
    "  part rover : Rover;\n",
    "  part broken : Missing;\n",
    "  part selected subsets rover;\n",
    "}\n",
);

/// The cohesive answer and the individual services read the same settled facts, so a consumer
/// choosing one cannot see a different model from a consumer choosing the other.
#[test]
fn element_details_agrees_with_the_services_it_is_assembled_from() {
    let published = detail_publication(
        &[("memory://model.sysml", VEHICLE_MODEL)],
        ConstructionSchedule::Sequential,
    );
    let symbol = identity_of(&published, "memory://model.sysml", "P::rover");
    let details = settled(published.element_details(symbol));
    assert_eq!(details.inspection, settled(published.inspect(symbol)));
    assert_eq!(details.evaluation, settled(published.evaluate(symbol)));
    let effective = settled(published.effective_types(symbol));
    assert_eq!(
        details
            .effective_typing
            .types
            .iter()
            .map(|entry| entry.element.identity.clone())
            .collect::<Vec<_>>(),
        effective
            .iter()
            .map(|entry| entry.symbol.clone())
            .collect::<Vec<_>>()
    );
}

/// Sequential and parallel construction publish the same details, and so do the same sources
/// admitted in a different order.
#[test]
fn construction_strategy_and_source_order_publish_equivalent_details() {
    let sources = [
        ("memory://a.sysml", "package P { part def Wheel; }"),
        (
            "memory://b.sysml",
            "package P { part def Vehicle { part wheel : Wheel; } part car : Vehicle; }",
        ),
    ];
    let permuted = [sources[1], sources[0]];

    let render = |published: &PublishedResolution| {
        ["P::car", "P::Vehicle", "P::Wheel"]
            .iter()
            .map(|name| {
                let document = if *name == "P::Wheel" {
                    "memory://a.sysml"
                } else {
                    "memory://b.sysml"
                };
                render_details(&details_of(published, document, name))
            })
            .collect::<Vec<_>>()
    };

    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let reordered = detail_publication(&permuted, ConstructionSchedule::Sequential);
    assert_eq!(render(&sequential), render(&parallel));
    assert_eq!(render(&sequential), render(&reordered));
}

/// A publication that reuses a solved library stratum answers exactly as a full solve does.
#[test]
fn library_stratum_reuse_publishes_the_same_details_as_a_full_solve() {
    let library = SourceInput::new(
        "memory://lib.sysml",
        "standard library package Lib { part def Wheel; }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package W { part w : Lib::Wheel; }".to_string(),
        SourceKind::Workspace,
    );

    let full = build(
        BuildRequest::new(
            vec![library.clone(), workspace.clone()],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    let stratum = std::sync::Arc::new(build_library_stratum(vec![library]).unwrap());
    let warm = build(
        BuildRequest::with_library(
            vec![workspace],
            ConstructionSchedule::Sequential,
            "contract-v1",
            stratum,
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        render_details(&details_of(&full, "memory://model.sysml", "W::w")),
        render_details(&details_of(&warm, "memory://model.sysml", "W::w")),
    );
}

/// A cross-document reference is a resolved relationship with the target's own document, not a
/// name the consumer has to look up.
#[test]
fn element_details_resolve_a_cross_document_relationship_to_its_declaring_document() {
    let published = detail_publication(
        &[
            (
                "memory://defs.sysml",
                "package Defs { requirement def Endurance; }",
            ),
            (
                "memory://usage.sysml",
                "package Usage { import Defs::*; requirement check : Endurance; }",
            ),
        ],
        ConstructionSchedule::Sequential,
    );
    let check = details_of(&published, "memory://usage.sysml", "Usage::check");
    assert_eq!(check.typing.outcome, RelationshipOutcome::Resolved);
    assert_eq!(
        check.typing.targets[0].location.document.as_ref(),
        "memory://defs.sysml"
    );
}

/// A position identifies two different elements, and both are answered in full.
#[test]
fn element_details_at_a_position_answer_the_container_and_the_reference_separately() {
    let published = detail_publication(
        &[(
            "memory://at.sysml",
            "package P {\n  part def Engine;\n  part motor : Engine;\n}\n",
        )],
        ConstructionSchedule::Sequential,
    );
    let at = settled(published.element_details_at(
        "memory://at.sysml",
        TextPosition {
            line: 2,
            character: 15,
        },
    ));
    assert_eq!(
        at.containing
            .as_ref()
            .and_then(|details| details.inspection.name.as_deref()),
        Some("motor")
    );
    match &at.referenced {
        ReferencedDetails::Resolved(details) => {
            assert_eq!(details.inspection.name.as_deref(), Some("Engine"))
        }
        other => panic!("expected the reference under the cursor, got: {other:?}"),
    }

    // A position with no reference under it says so rather than reporting an unresolved one.
    let at = settled(published.element_details_at(
        "memory://at.sysml",
        TextPosition {
            line: 2,
            character: 8,
        },
    ));
    assert_eq!(at.referenced, ReferencedDetails::None);
}

#[test]
fn affected_documents_are_transitive_across_public_imports_and_aliases() {
    let sources = vec![
        SourceInput::new(
            "memory://a.sysml",
            "package A { part def T; }".into(),
            SourceKind::Workspace,
        ),
        SourceInput::new(
            "memory://b.sysml",
            "package B { public import A::*; alias AliasT for T; }".into(),
            SourceKind::Workspace,
        ),
        SourceInput::new(
            "memory://c.sysml",
            "package C { import B::*; part p : AliasT; }".into(),
            SourceKind::Workspace,
        ),
    ];
    let published =
        build(BuildRequest::new(sources, ConstructionSchedule::Sequential, "contract-v1").unwrap())
            .unwrap();
    let QueryOutcome::Resolved(affected) = published.affected_documents("memory://a.sysml") else {
        panic!("complete imports must publish a settled dependency outcome")
    };
    assert_eq!(
        affected
            .iter()
            .map(|document| document.identity.as_ref())
            .collect::<Vec<_>>(),
        vec!["memory://b.sysml", "memory://c.sysml"]
    );
}
