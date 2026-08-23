//! Contract tests for the diagnostics phase, driven through the crate's public
//! `build()` / `PublishedResolution` surface. Relocated verbatim from the inline
//! `#[cfg(test)]` modules of `src/lib.rs` and `src/model.rs`.

#![allow(clippy::too_many_lines)]

mod common;

#[allow(unused_imports)]
use common::*;
#[allow(unused_imports)]
use sysml_resolution::*;

#[test]
fn attribute_default_value_dotted_member_access_with_unresolvable_base_stays_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute g = nope.a;\n\
         }\n",
    );
    assert!(
        output.contains("(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"nope::a\")\n      (outcome (status unresolved))"),
        "expected an unresolvable base to leave the whole chain explicitly unresolved (never fabricated), got:\n{output}"
    );
}

#[test]
fn attribute_default_value_dotted_member_access_with_missing_member_stays_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def F {\n\
         \t\tattribute a;\n\
         \t}\n\
         \tpart f : F;\n\
         \tattribute g = f.missing;\n\
         }\n",
    );
    assert!(
        output.contains("(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"f::missing\")\n      (outcome (status unresolved))"),
        "expected a member absent from f's type F to leave the chain explicitly unresolved (never fabricated), got:\n{output}"
    );
}

#[test]
fn constraint_comparison_expression_leaves_undeclared_operand_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute x : ScalarValues::Integer;\n\
         \tconstraint def C { x > y }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"x\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::x\")))))"
        ),
        "expected x to resolve, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 1))\n      (authored-target \"y\")\n      (outcome (status unresolved))"
        ),
        "expected undeclared y to stay unresolved (not fabricated), got:\n{output}"
    );
}

#[test]
fn constraint_unsupported_expression_shape_still_falls_through_to_diagnostic() {
    // `Expression::Invocation` (e.g. `compute(x, y)`) is a supported shape as of this slice
    // (see `lower_invocation_callee`/`ReferenceKind::InvocationCallee`); `-`/`not` unary ops
    // are now supported too (`is_unary_operator`), so `~x` (`UnaryOperator::BitNot`, out of
    // scope, see `is_unary_operator`'s doc comment) exercises the still-unsupported path.
    let request = sysml_resolution::BuildRequest::new(
        vec![sysml_resolution::SourceInput::new(
            "memory://test/enum.sysml",
            "package Demo {\n\
             \tconstraint def C { ~x }\n\
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
        output.contains("unsupported_constraint_definition_member"),
        "expected a still-unsupported unary-op expression to surface as an unsupported \
         constraint-definition-member diagnostic, got:\n{output}"
    );
}

#[test]
fn occurrence_definition_member_body_construct_stays_explicitly_unsupported() {
    let request = sysml_resolution::BuildRequest::new(
        vec![sysml_resolution::SourceInput::new(
            "memory://test/enum.sysml",
            "package Demo {\n\
             \toccurrence def Occ {\n\
             \t\tsuccession first x then y;\n\
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
        output.contains("unsupported_occurrence_definition_member"),
        "expected the succession usage to surface as an explicit unsupported diagnostic, got:\n{output}"
    );
}

#[test]
fn calc_def_body_kinded_parameter_is_recovered_by_the_pinned_parser() {
    // Regression pin, not desired behavior. Through `49bdf3f` a directed KerML-kinded
    // parameter in a calc-shaped body reached the AST as a `KermlFeature` and lowered under
    // the kind its keyword names (`expr` -> `kerml-expression`) with its direction as a
    // declaration fact. At the pinned `f52100fd` the new `in`/`out`/`inout` branch of
    // `parser/constraint.rs` commits to the `InOutDecl` parameter parser and no longer falls
    // back to the KerML feature route, so the member is dropped to parse recovery and nothing
    // is published for it. See planning/UPSTREAM_PARSER_GAPS.md gap 81. The regression is
    // scoped to the SysML `calc`/`constraint`-shaped bodies that route through that branch:
    // the same spelling in a KerML `function`/`behavior` body still parses and lowers, which
    // is why `tests/snapshots/sysml.library/control_functions.md` is unaffected.
    //
    // This pins the loss so it stays visible: the publication must say `parse-recovery`
    // rather than silently publishing a partial model that looks complete.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def C {\n\
         \t\tin a : Boolean;\n\
         \t\tin expr p : Boolean = a;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(completeness parse-recovery)"),
        "expected the kinded parameter to be reported as parse recovery, got:\n{output}"
    );
    assert!(
        !output.contains("(qualified-name \"Demo::C::p\")"),
        "expected no declaration for the recovered parameter p, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::C::a\")"),
        "expected the plain directed parameter a to still lower, got:\n{output}"
    );
}

#[test]
fn calc_def_body_kinded_parameter_redefinition_is_recovered_by_the_pinned_parser() {
    // The redefinition-only spelling of the same production (`in bool redefines onOccurrence
    // { ... }`, the shape Kernel Semantic Library `Observation.kerml` authors in a KerML
    // function body) is lost the same way in a `calc def` body at the pinned revision,
    // including its nested body. See the sibling test above and
    // planning/UPSTREAM_PARSER_GAPS.md gap 81.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def C {\n\
         \t\tin a : Boolean;\n\
         \t\tin bool redefines a {\n\
         \t\t\treturn : Boolean;\n\
         \t\t}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(completeness parse-recovery)"),
        "expected the kinded redefinition to be reported as parse recovery, got:\n{output}"
    );
    assert!(
        !output.contains("kerml-boolean-expression"),
        "expected no kerml-boolean-expression declaration for the recovered member, got:\n\
         {output}"
    );
}

#[test]
fn metadata_annotation_with_unresolvable_target_stays_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Vehicle {\n\
         \t\tpart seatBelt[2] {@NoSuchMetadata;}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind metadataAnnotation)") && output.contains("(status unresolved)"),
        "expected seatBelt's @NoSuchMetadata metadata annotation reference to stay explicitly unresolved, got:\n{output}"
    );
}

#[test]
fn filter_with_unresolvable_metadata_target_stays_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpackage 'Safety Features' {\n\
         \t\tpublic import Demo::**;\n\
         \t\tfilter @NoSuchMetadata;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind filterMetadataTest)") && output.contains("(status unresolved)"),
        "expected the filter's @NoSuchMetadata metadata-test reference to stay explicitly unresolved, got:\n{output}"
    );
}

/// `first X then Y;` inside an action def body now lowers as a resolved `succession`
/// relationship (see `sysml_resolution::tests::first_then_succession_inside_action_def_body_resolves_both_ends`
/// in `lib.rs` for the full assertion); it no longer falls through to the generic
/// unsupported-member diagnostic this test originally locked in per commit `f4ae83f7`.
#[test]
fn first_then_succession_inside_an_action_def_no_longer_surfaces_as_unsupported() {
    let request = sysml_resolution::BuildRequest::new(
        vec![sysml_resolution::SourceInput::new(
            "memory://test/enum.sysml",
            "package Demo {\n\
             \taction def ExecuteMission {\n\
             \t\taction validateRoute;\n\
             \t\taction startMission;\n\
             \t\tfirst validateRoute then startMission;\n\
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
        !output.contains("unsupported_action_definition_member"),
        "did not expect an unsupported action-definition-member diagnostic, got:\n{output}"
    );
}

#[test]
fn satisfy_with_an_unresolvable_requirement_stays_explicitly_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P;\n\
         \tpart p : P {\n\
         \t\tsatisfy missingReq by p;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind satisfySource)")
            && output.contains("(authored-target \"missingReq\")")
            && output.contains("(status unresolved)"),
        "expected the unresolvable satisfy source to stay explicitly unresolved (not \
         fabricated), got:\n{output}"
    );
    assert!(
        output.contains("(kind satisfyTarget)")
            && output.contains("(authored-target \"p\")\n      (outcome (status resolved)"),
        "expected the satisfy target to still resolve independently, got:\n{output}"
    );
}

#[test]
fn satisfy_with_an_unresolvable_satisfying_element_stays_explicitly_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trequirement def R;\n\
         \trequirement r : R;\n\
         \tpart def P;\n\
         \tpart p : P {\n\
         \t\tsatisfy r by missingElement;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind satisfyTarget)")
            && output.contains("(authored-target \"missingElement\")")
            && output.contains("(status unresolved)"),
        "expected the unresolvable satisfying element to stay explicitly unresolved (not \
         fabricated), got:\n{output}"
    );
    assert!(
        output.contains("(kind satisfySource)")
            && output.contains("(authored-target \"r\")\n      (outcome (status resolved)"),
        "expected the satisfy source to still resolve independently, got:\n{output}"
    );
}

#[test]
fn allocate_statement_with_an_unresolvable_target_stays_explicitly_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def A;\n\
         \tpart a : A;\n\
         \tpart b : A {\n\
         \t\tallocate a to missingTarget;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind allocateTarget)")
            && output.contains("(authored-target \"missingTarget\")")
            && output.contains("(status unresolved)"),
        "expected the unresolvable allocate target to stay explicitly unresolved (not \
         fabricated), got:\n{output}"
    );
    assert!(
        output.contains("(kind allocateSource)")
            && output.contains("(authored-target \"a\")\n      (outcome (status resolved)"),
        "expected the allocate source to still resolve independently, got:\n{output}"
    );
}

#[test]
fn bind_statement_with_an_unresolvable_target_stays_explicitly_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def A;\n\
         \tpart a : A;\n\
         \tpart b : A {\n\
         \t\tbind a = missingTarget;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind bindTarget)")
            && output.contains("(authored-target \"missingTarget\")")
            && output.contains("(status unresolved)"),
        "expected the unresolvable bind target to stay explicitly unresolved (not \
         fabricated), got:\n{output}"
    );
    assert!(
        output.contains("(kind bindSource)")
            && output.contains("(authored-target \"a\")\n      (outcome (status resolved)"),
        "expected the bind source to still resolve independently, got:\n{output}"
    );
}

#[test]
fn variant_with_an_unresolvable_target_stays_explicitly_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Transmission;\n\
         \tpart manualTransmission;\n\
         \tpart vehicle {\n\
         \t\tvariation part transmission : Transmission {\n\
         \t\t\tvariant manualTransmission;\n\
         \t\t\tvariant missingVariant;\n\
         \t\t}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind variant)")
            && output.contains("(authored-target \"missingVariant\")")
            && output.contains("(status unresolved)"),
        "expected the unresolvable variant target to stay explicitly unresolved (not \
         fabricated), got:\n{output}"
    );
    assert!(
        output
            .contains("(authored-target \"manualTransmission\")\n      (outcome (status resolved)"),
        "expected the resolvable variant to still resolve independently, got:\n{output}"
    );
}

#[test]
fn use_case_include_with_an_unresolvable_target_stays_explicitly_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tuse case def MainUseCase {\n\
         \t\tinclude missingUseCase;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind includeUseCase)")
            && output.contains("(authored-target \"missingUseCase\")")
            && output.contains("(status unresolved)"),
        "expected the unresolvable include target to stay explicitly unresolved (not \
         fabricated), got:\n{output}"
    );
}

#[test]
fn ref_decl_with_an_unresolvable_typing_target_stays_explicitly_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Holder {\n\
         \t\tref self: MissingType;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind featureTyping)")
            && output.contains("(authored-target \"MissingType\")")
            && output.contains("(status unresolved)"),
        "expected the unresolvable ref typing target to stay explicitly unresolved (not \
         fabricated), got:\n{output}"
    );
}

#[test]
fn value_assignment_tuple_with_unresolvable_element_leaves_only_that_element_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute a : ScalarValues::Integer;\n\
         \tattribute tuple = (a, missing);\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"a\")\n      \
             (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::a\")))))"
        ),
        "expected resolvable tuple element `a` to resolve, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 1))\n      (authored-target \"missing\")\n      \
             (outcome (status unresolved))"
        ),
        "expected undeclared tuple element `missing` to stay explicitly unresolved (not \
         fabricated), got:\n{output}"
    );
}

#[test]
fn value_assignment_istype_with_unresolvable_operand_and_type_stays_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute check = missingOperand istype MissingType;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \
             \"missingOperand\")\n      (outcome (status unresolved))"
        ),
        "expected undeclared operand `missingOperand` to stay explicitly unresolved, \
         got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typeCheckTarget) (ordinal 0))\n      (authored-target \"MissingType\")\n      \
             (outcome (status unresolved))"
        ),
        "expected undeclared type target `MissingType` to stay explicitly unresolved, \
         got:\n{output}"
    );
}

#[test]
fn value_assignment_meta_cast_with_unresolvable_base_and_metaclass_stays_unresolved() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute check = missingOperand meta Missing::Metaclass;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \
             \"missingOperand\")\n      (outcome (status unresolved))"
        ),
        "expected undeclared base `missingOperand` to stay explicitly unresolved, \
         got:\n{output}"
    );
    assert!(
        output.contains("(kind metaCastTarget)") && output.contains("(status unresolved)"),
        "expected undeclared metaclass target `Missing::Metaclass` to stay explicitly \
         unresolved, got:\n{output}"
    );
}

#[test]
fn diagram_projection_preserves_resolved_facts_from_unsupported_inspections() {
    let request = BuildRequest::new(
        vec![
            SourceInput::new(
                "memory://standard-views.sysml",
                concat!(
                    "standard library package StandardViewDefinitions { view def GeneralView; ",
                    "view def StateTransitionView; } standard library package SysML { ",
                    "metaclass PartUsage; }",
                ).to_owned(),
                SourceKind::StandardLibrary,
            ),
            SourceInput::new(
                "memory://model.sysml",
                concat!(
                    "package Model { import StandardViewDefinitions::*; ",
                    "part def Board; part def Assembly { part pcb : Board; } part root : Assembly; ",
                    "state def Machine { state idle; state running; transition start first idle then running; } ",
                    "view structure : GeneralView { expose root; filter @SysML::PartUsage; } ",
                    "view behavior : StateTransitionView { expose Machine; } }",
                ).to_owned(),
                SourceKind::Workspace,
            ),
        ],
        ConstructionSchedule::Sequential,
        "contract-v1",
    ).unwrap();
    let published = build(request).unwrap();
    let catalog = match published.diagram_view_catalog() {
        QueryOutcome::Resolved(catalog) | QueryOutcome::UnsupportedWith(catalog) => catalog,
        other => panic!("expected diagram catalog, got {other:?}"),
    };
    let structure = catalog
        .iter()
        .find(|view| view.kind == DiagramViewKind::General)
        .unwrap();
    let projection = match published.diagram_view(structure.semantic_id) {
        QueryOutcome::Resolved(projection) => projection,
        other => panic!("expected General View projection, got {other:?}"),
    };
    let root = projection
        .elements
        .iter()
        .find(|element| element.name.as_deref() == Some("root"))
        .unwrap();
    assert!(matches!(root.typing, DiagramElementTyping::Resolved(_)));
    assert!(projection.relationships.iter().any(|relationship| {
        relationship.source == root.occurrence_id
            && relationship.source_semantic_id == root.semantic_id
            && relationship.kind == DiagramRelationshipKind::FeatureTyping
    }));

    let behavior = catalog
        .iter()
        .find(|view| view.kind == DiagramViewKind::StateTransition)
        .unwrap();
    let projection = match published.diagram_view(behavior.semantic_id) {
        QueryOutcome::Resolved(projection) => projection,
        other => panic!("expected State Transition projection, got {other:?}"),
    };
    let DiagramScene::StateTransition(scene) = projection.scene else {
        panic!("expected State Transition scene");
    };
    assert_eq!(scene.transitions.len(), 1);
}

#[test]
fn every_diagnostic_carries_an_owner_produced_message() {
    let diagnostics = diagnostics_for(
        "package P { part def A; part def A; part b; port def PD; \
         part def D { port p : PD; } }",
    );
    assert!(!diagnostics.is_empty());
    for diagnostic in &diagnostics {
        assert!(
            !diagnostic.message().trim().is_empty(),
            "empty message: {diagnostic:#?}"
        );
        assert!(
            !diagnostic.category().as_str().is_empty(),
            "diagnostic has no typed category: {diagnostic:#?}"
        );
    }
}

/// A `first X then Y;` succession whose `then` target is not declared anywhere in the model
/// must stay an explicit unresolved reference fact, not a fabricated or guessed target.
#[test]
fn first_then_succession_unresolvable_target_stays_unresolved() {
    let sexpr = semantic_sexpr_for(
        "package P { action def ExecuteMission { action validateRoute; first validateRoute then missingAction; } }",
    );
    assert!(
        sexpr.contains("(kind succession)"),
        "expected a succession reference to be authored, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(status unresolved)"),
        "expected the unresolvable `then` target to remain explicitly unresolved, got: {sexpr}"
    );
}

/// An `entry action X;` binding whose target is not declared anywhere in the model must stay
/// an explicit unresolved reference fact, not a fabricated or guessed target.
#[test]
fn entry_action_binding_unresolvable_target_stays_unresolved() {
    let sexpr = semantic_sexpr_for("package P { state def S { entry action missingAction; } }");
    assert!(
        sexpr.contains("(kind entryActionBinding)"),
        "expected an entryActionBinding reference to be authored, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(status unresolved)"),
        "expected the unresolvable entry action target to remain explicitly unresolved, got: {sexpr}"
    );
}

/// A `then <target>;` initial-state marker whose target is not declared anywhere in the model
/// must stay an explicit unresolved reference fact, not a fabricated or guessed target.
#[test]
fn then_initial_state_unresolvable_target_stays_unresolved() {
    let sexpr = semantic_sexpr_for("package P { state def S { then missingState; } }");
    assert!(
        sexpr.contains("(kind initialState)"),
        "expected an initialState reference to be authored, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(status unresolved)"),
        "expected the unresolvable `then` target to remain explicitly unresolved, got: {sexpr}"
    );
}

/// A transition whose `source`/`target` are not declared anywhere in the model must stay an
/// explicit unresolved reference fact, not a fabricated or guessed target.
#[test]
fn transition_source_and_target_unresolvable_stay_unresolved() {
    let sexpr = semantic_sexpr_for(
        "package P { state def S { transition first missingOff then missingOn; } }",
    );
    assert!(
        sexpr.contains("(kind transitionSource)") && sexpr.contains("(kind transitionTarget)"),
        "expected transitionSource/transitionTarget references to be authored, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(status unresolved)"),
        "expected the unresolvable transition ends to remain explicitly unresolved, got: {sexpr}"
    );
}

/// A bare `require;`-less-constraint shorthand (`has_constraint_keyword == false`, e.g.
/// `require someExistingConstraint;`) references an existing constraint rather than declaring
/// one. Upstream now carries that role on `RequireConstraint::target`, but nothing lowers it
/// yet (planning/UPSTREAM_PARSER_GAPS.md, "Typed upstream, not yet lowered here"), so it must
/// stay an explicit unsupported diagnostic rather than being silently dropped or guessed at.
#[test]
fn require_shorthand_reference_without_constraint_keyword_stays_unsupported() {
    let sexpr =
        diagnostics_sexpr_for("package P { constraint c; requirement def R { require c; } }");
    assert!(
        sexpr.contains("unsupported_requirement_definition_member"),
        "expected the constraint-keyword-less `require c;` shorthand to remain unsupported, got: {sexpr}"
    );
}

/// A state def/usage body's bare `entry;`/`do;`/`exit;` (no `action` reference, no body
/// content) is a legal no-op marker -- pervasive in the training/validation corpus (e.g.
/// `entry; then off;`) -- and must not be reported as `unsupported_state_definition_member`
/// merely because it has no bound action reference to lower.
#[test]
fn bare_entry_do_exit_with_no_reference_or_body_is_not_unsupported() {
    let sexpr =
        semantic_sexpr_for("package P { state def S { state off; entry; do; exit; then off; } }");
    assert!(
        !sexpr.contains("unsupported_state_definition_member"),
        "did not expect unsupported_state_definition_member for bare entry/do/exit, got: {sexpr}"
    );
}

/// An inline `entry { <members> }` anonymous action body (non-empty brace, no `action`
/// reference) genuinely has no representation in the `EntryAction` typed AST and must stay an
/// explicit unsupported diagnostic, distinguishing it from the empty/semicolon no-op case
/// above.
#[test]
fn entry_with_inline_body_content_and_no_reference_stays_unsupported() {
    let sexpr = diagnostics_sexpr_for("package P { state def S { entry { state inner; } } }");
    assert!(
        sexpr.contains("unsupported_state_definition_member"),
        "expected an inline non-empty entry body with no reference to remain unsupported, got: {sexpr}"
    );
}

/// A bare `terminate;` (no target) has nothing to resolve and must not be flagged as
/// unsupported -- it is a legitimate no-op self-termination form, not a parser gap.
#[test]
fn bare_terminate_stmt_is_not_unsupported() {
    let sexpr = semantic_sexpr_for("package P { action def A { terminate; } }");
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// An `assign` statement whose value expression references an unresolvable operand must
/// still publish the target/value references, staying explicitly unresolved rather than
/// silently dropped.
#[test]
fn assign_stmt_unresolvable_target_stays_unresolved() {
    let sexpr = semantic_sexpr_for("package P { action def A { assign missing := 5; } }");
    assert!(
        sexpr.contains("(kind assignTarget)"),
        "expected an assignTarget reference to be authored, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(status unresolved)"),
        "expected the unresolvable assign target to remain explicitly unresolved, got: {sexpr}"
    );
}

/// An unresolved reference keeps its own outcome instead of being dropped, so an inspector can
/// show what was written alongside the fact that it did not resolve.
#[test]
fn inspection_keeps_an_unresolved_reference_and_its_authored_text() {
    let published = publication_for(&[(
        "memory://i.sysml",
        "package P {\n  part broken : NoSuchType;\n}",
    )]);
    let broken = inspect_named(&published, "memory://i.sysml", 1, 8);
    let typing = broken
        .relationships
        .iter()
        .find(|relationship| relationship.kind == "featureTyping")
        .expect("expected the authored typing reference to survive");
    assert_eq!(typing.authored.as_deref(), Some("NoSuchType"));
    assert_eq!(typing.target, RelationshipTarget::Unresolved);
}

#[test]
fn inspecting_an_unknown_document_is_unresolved() {
    let published = publication_for(&[("memory://i.sysml", "package P { }")]);
    assert!(matches!(
        published.document_symbols("memory://absent.sysml"),
        QueryOutcome::Unresolved
    ));
}

#[test]
fn satisfy_query_pairs_directional_ends_preserves_identity_polarity_and_unresolved() {
    let published = publication_for(&[(
        "memory://trace.sysml",
        r#"
package Trace {
requirement def Safety;
requirement def Performance;
part def Vehicle;
part vehicle : Vehicle;
satisfy Performance by vehicle;
not satisfy Safety by vehicle;
satisfy Missing by vehicle;
}
"#,
    )]);
    let values = match published.satisfy_relationships() {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected resolved satisfy query, got {other:?}"),
    };
    assert_eq!(values.len(), 3);
    let requirements = match published.search_elements(ElementSearch {
        kind: ElementKind::RequirementDefinition,
        source: ElementSource::Workspace,
    }) {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected requirements, got {other:?}"),
    };
    let performance = requirements
        .iter()
        .find(|value| value.qualified_name.as_ref() == "Trace::Performance")
        .expect("Performance");
    let parts = match published.search_elements(ElementSearch {
        kind: ElementKind::PartUsage,
        source: ElementSource::Workspace,
    }) {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected parts, got {other:?}"),
    };
    let vehicle = parts
        .iter()
        .find(|value| value.qualified_name.as_ref() == "Trace::vehicle")
        .expect("vehicle");
    assert!(
        matches!(&values[0].requirement, SatisfyEndpoint::Resolved(value) if value == &performance.identity)
    );
    assert!(
        matches!(&values[0].satisfying_element, SatisfyEndpoint::Resolved(value) if value == &vehicle.identity)
    );
    assert_eq!(values[0].polarity, SatisfyPolarity::Satisfied);
    assert_eq!(values[1].polarity, SatisfyPolarity::NotSatisfied);
    assert!(matches!(values[2].requirement, SatisfyEndpoint::Unresolved));
    assert_eq!(values[0].provenance, RelationshipProvenance::Authored);
    assert_ne!(values[0].identity, values[1].identity);
}

#[test]
fn binding_connector_query_pairs_ends_preserves_duplicates_and_unresolved_outcomes() {
    let published = publication_for(&[(
        "memory://binding.sysml",
        r#"
package Binding {
action def Act {
    action start;
    action done;
    bind start = done;
    bind Missing = done;
    bind start = done;
}
}
"#,
    )]);
    let values = match published.binding_connectors() {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected resolved binding connectors, got {other:?}"),
    };
    assert_eq!(
        values.len(),
        3,
        "each authored binding must remain a separate fact"
    );
    let actions = match published.search_elements(ElementSearch {
        kind: ElementKind::ActionUsage,
        source: ElementSource::Workspace,
    }) {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected actions, got {other:?}"),
    };
    let start = actions
        .iter()
        .find(|value| value.qualified_name.as_ref() == "Binding::Act::start")
        .expect("start action");
    let done = actions
        .iter()
        .find(|value| value.qualified_name.as_ref() == "Binding::Act::done")
        .expect("done action");
    assert!(
        matches!(&values[0].source, BindingEndpoint::Resolved(value) if value == &start.identity)
    );
    assert!(
        matches!(&values[0].target, BindingEndpoint::Resolved(value) if value == &done.identity)
    );
    assert!(matches!(values[1].source, BindingEndpoint::Unresolved));
    assert!(
        matches!(&values[2].source, BindingEndpoint::Resolved(value) if value == &start.identity)
    );
    assert_eq!(values[0].provenance, RelationshipProvenance::Authored);
    assert_ne!(values[0].identity, values[2].identity);
}

#[test]
fn feature_reference_expression_binding_check_is_explicitly_unsupported_without_owned_facts() {
    let published = publication_for(&[(
        "memory://binding-rule.sysml",
        "package Binding { action def Act { action start; action done; bind start = done; } }",
    )]);
    assert!(matches!(
        published
            .binding_connector_validation(BindingConnectorCheckKind::FeatureReferenceExpression),
        QueryOutcome::Resolved(BindingConnectorValidationOutcome::Unsupported {
            prerequisite:
                BindingConnectorValidationPrerequisite::FeatureReferenceExpressionTargetAndResult,
        })
    ));
    assert!(matches!(
        published.binding_connectors(),
        QueryOutcome::Resolved(values) if values.len() == 1
    ));
}

#[test]
fn verification_query_owns_case_direction_endpoint_status_and_unsupported_outcome() {
    let published = publication_for(&[(
        "memory://verification.sysml",
        r#"
package V {
requirement required;
verification def Check {
    objective { verify required; }
    objective second { verify Missing; }
}
}
"#,
    )]);
    let values = match published.requirement_verifications() {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected resolved verification query, got {other:?}"),
    };
    assert_eq!(values.len(), 2);
    let cases = match published.search_elements(ElementSearch {
        kind: ElementKind::VerificationCaseDefinition,
        source: ElementSource::Workspace,
    }) {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected verification cases, got {other:?}"),
    };
    let check = cases
        .iter()
        .find(|value| value.qualified_name.as_ref() == "V::Check")
        .expect("Check");
    assert!(values
        .iter()
        .all(|value| value.verification_case == check.identity));
    assert!(matches!(
        values[0].requirement,
        VerificationRequirement::Resolved(_)
    ));
    assert!(matches!(
        values[1].requirement,
        VerificationRequirement::Unresolved
    ));
    assert!(values
        .iter()
        .all(|value| value.provenance == RelationshipProvenance::Authored));
    assert!(values
        .iter()
        .all(|value| value.outcome == VerificationOutcome::Unsupported));
    assert_ne!(values[0].identity, values[1].identity);
}

#[test]
fn a_cyclic_hierarchy_yields_no_conformance_answer() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def A :> B; part def B :> A; part def C; }",
    )]);
    let a = symbol_named(&published, "memory://types.sysml", "P::A");
    let c = symbol_named(&published, "memory://types.sysml", "P::C");

    assert_eq!(
        conformance(published.conforms_to(a, c, SpecializationScope::AnySpecialization)),
        Conformance::Indeterminate(ConformanceObstacle::CyclicSpecialization),
        "a malformed hierarchy must not produce a published conformance fact"
    );
    assert_eq!(
        conformance(published.conforms_to(a, a, SpecializationScope::AnySpecialization)),
        Conformance::Conforms,
        "reflexivity holds even inside a cycle"
    );
}

#[test]
fn feature_typing_conformance_rejects_an_unrelated_type() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def T; part def U; part def A { part x : T; } part def B :> A { part y : U :>> x; } }",
    )]);
    let general = symbol_named(&published, "memory://types.sysml", "P::A::x");
    let specific = symbol_named(&published, "memory://types.sysml", "P::B::y");

    assert_eq!(
        conformance(published.feature_typing_conforms(specific, general)),
        Conformance::DoesNotConform,
        "U neither is nor specializes T"
    );
}

#[test]
fn view_selection_keeps_unresolved_and_unsupported_predicates_explicit() {
    let document = "memory://views.sysml";
    let published = detail_publication(
        &[(
            document,
            concat!(
                "package P {\n",
                "  part candidate;\n",
                "  view unresolved { filter @Missing; }\n",
                "  view unsupported { filter 1; }\n",
                "}\n",
            ),
        )],
        ConstructionSchedule::Sequential,
    );
    let candidate = identity_of(&published, document, "P::candidate");
    let unresolved = identity_of(&published, document, "P::unresolved");
    assert_eq!(
        settled(published.view_selection(unresolved, candidate)).outcome,
        ViewSelectionOutcome::Indeterminate(Box::new([ViewSelectionObstacle::UnresolvedPredicate]))
    );
    let unsupported = identity_of(&published, document, "P::unsupported");
    assert_eq!(
        settled(published.view_selection(unsupported, candidate)).outcome,
        ViewSelectionOutcome::Indeterminate(Box::new([
            ViewSelectionObstacle::UnsupportedPredicate
        ]))
    );
}

#[test]
fn feature_relationship_collection_keeps_an_unresolved_canonical_edge_visible() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { classifier Vehicle { feature derived chains Missing; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let derived = identity_of(
        &published,
        "memory://model.sysml",
        "Model::Vehicle::derived",
    );
    assert!(matches!(
        settled(published.feature_derived_relationships(
            derived,
            FeatureDerivedRelationshipCollection::OwnedFeatureChaining,
        ))
        .as_ref(),
        [ElementRelationship {
            kind: "featureChaining",
            target: RelationshipTarget::Unresolved,
            provenance: RelationshipProvenance::Authored,
            ..
        }]
    ));
}

#[test]
fn exact_type_relationship_collections_project_canonical_authored_and_unresolved_facts() {
    let published = detail_publication(
        &[ (
            "memory://model.sysml",
            "package Model { classifier Base; classifier Derived specializes Base unions Base intersects Base differences Base disjoint from Base; classifier Partial unions Missing; }",
        ) ],
        ConstructionSchedule::Sequential,
    );
    let base = identity_of(&published, "memory://model.sysml", "Model::Base");
    let derived = identity_of(&published, "memory://model.sysml", "Model::Derived");
    let partial = identity_of(&published, "memory://model.sysml", "Model::Partial");
    let values =
        |collection| settled(published.type_derived_relationships(derived, collection)).into_vec();
    for (collection, kind) in [
        (
            TypeDerivedRelationshipCollection::OwnedSpecialization,
            "specialization",
        ),
        (TypeDerivedRelationshipCollection::OwnedUnioning, "unioning"),
        (
            TypeDerivedRelationshipCollection::OwnedIntersecting,
            "intersecting",
        ),
        (
            TypeDerivedRelationshipCollection::OwnedDifferencing,
            "differencing",
        ),
        (
            TypeDerivedRelationshipCollection::OwnedDisjoining,
            "disjoining",
        ),
        (TypeDerivedRelationshipCollection::UnioningType, "unioning"),
        (
            TypeDerivedRelationshipCollection::IntersectingType,
            "intersecting",
        ),
        (
            TypeDerivedRelationshipCollection::DifferencingType,
            "differencing",
        ),
    ] {
        assert!(matches!(
            values(collection).as_slice(),
            [ElementRelationship {
                kind: actual_kind,
                provenance: RelationshipProvenance::Authored,
                target: RelationshipTarget::Resolved(target),
                ..
            }] if *actual_kind == kind && target == &base
        ));
    }
    assert!(matches!(
        settled(
            published.type_derived_relationships(
                partial,
                TypeDerivedRelationshipCollection::UnioningType,
            )
        )
        .as_ref(),
        [ElementRelationship {
            kind: "unioning",
            provenance: RelationshipProvenance::Authored,
            target: RelationshipTarget::Unresolved,
            ..
        }]
    ));
}

#[test]
fn variable_feature_membership_is_explicitly_unsupported_without_snapshots() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { classifier Vehicle { var feature mass; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let mass = identity_of(&published, "memory://model.sysml", "Model::Vehicle::mass");
    assert!(matches!(
        published.featuring_types(mass),
        QueryOutcome::Unsupported
    ));
    assert!(matches!(
        published.featuring_type(mass),
        QueryOutcome::Unsupported
    ));
    assert!(type_featuring_relationships(
        &published,
        "memory://model.sysml",
        "Model::Vehicle::mass"
    )
    .is_empty());
}

#[test]
fn polarity_branch_anchor_failures_are_explicit_and_deduplicated() {
    const RULE: &str = "sysml-2.0:8.3.21.10:checkSatisfyRequirementUsageSpecialization";
    let workspace = || {
        SourceInput::new(
            "memory://model.sysml",
            "package Model { requirement def Safety; part def Vehicle; not satisfy Safety by Vehicle; not satisfy Safety by Vehicle; }".to_string(),
            SourceKind::Workspace,
        )
    };
    let missing = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://requirements.sysml",
                    "standard library package Requirements { constraint def satisfiedRequirementChecks; }".to_string(),
                    SourceKind::StandardLibrary,
                ),
                workspace(),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    assert!(matches!(
        missing.library_specialization_anchor_branch(
            RULE,
            LibrarySpecializationAnchorBranch::PredicateTrue,
        ),
        QueryOutcome::Unresolved
    ));
    let missing_published_diagnostics = missing.diagnostics();
    let missing_diagnostics = missing_published_diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code().as_str() == "missing_library_anchor"
                && diagnostic
                    .message()
                    .contains("Requirements::notSatisfiedRequirementChecks")
        })
        .collect::<Vec<_>>();
    assert_eq!(missing_diagnostics.len(), 1);

    let ambiguous = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://requirements-a.sysml",
                    "standard library package Requirements { constraint def notSatisfiedRequirementChecks; }".to_string(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://requirements-b.sysml",
                    "standard library package Requirements { constraint def notSatisfiedRequirementChecks; }".to_string(),
                    SourceKind::StandardLibrary,
                ),
                workspace(),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    assert!(matches!(
        ambiguous.library_specialization_anchor_branch(
            RULE,
            LibrarySpecializationAnchorBranch::PredicateTrue,
        ),
        QueryOutcome::Ambiguous(candidates) if candidates.len() == 2
    ));
    let ambiguous_published_diagnostics = ambiguous.diagnostics();
    let ambiguous_diagnostics = ambiguous_published_diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.code().as_str() == "ambiguous_library_anchor"
                && diagnostic
                    .message()
                    .contains("Requirements::notSatisfiedRequirementChecks")
        })
        .collect::<Vec<_>>();
    assert_eq!(ambiguous_diagnostics.len(), 1);
    assert_eq!(ambiguous_diagnostics[0].related_len(), 2);
}

/// Anchor failures remain typed published states and report one actionable cause, rather than
/// one warning per `part def` or a guessed workspace substitute.
#[test]
fn part_definition_anchor_failures_are_explicit_and_report_one_root_cause() {
    let missing_library = SourceInput::new(
        "memory://incomplete-standard.sysml",
        "standard library package NotParts {}".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { part def Component; part def Other; }".to_string(),
        SourceKind::Workspace,
    );
    let missing = build(
        BuildRequest::new(
            vec![missing_library, workspace.clone()],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    assert!(matches!(
        missing.part_definition_specialization_anchor(),
        QueryOutcome::Unresolved
    ));
    assert_eq!(
        missing
            .diagnostics()
            .iter()
            .map(|diagnostic| diagnostic.code().as_str())
            .collect::<Vec<_>>(),
        vec!["missing_library_anchor"]
    );
    assert!(
        specialization_relationships(&missing, "memory://model.sysml", "Model::Component")
            .is_empty()
    );
    assert_eq!(
        missing
            .diagnostics()
            .get(0)
            .expect("one diagnostic")
            .category(),
        DiagnosticCategory::MissingContext
    );

    let ambiguous = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://parts-a.sysml",
                    "standard library package Parts { part def Part; }".to_string(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://parts-b.sysml",
                    "standard library package Parts { part def Part; }".to_string(),
                    SourceKind::StandardLibrary,
                ),
                workspace,
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    assert!(matches!(
        ambiguous.part_definition_specialization_anchor(),
        QueryOutcome::Ambiguous(candidates) if candidates.len() == 2
    ));
    let published_diagnostics = ambiguous.diagnostics();
    let diagnostics = published_diagnostics.iter().collect::<Vec<_>>();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code().as_str(), "ambiguous_library_anchor");
    assert_eq!(diagnostics[0].category(), DiagnosticCategory::Ambiguous);
    assert_eq!(diagnostics[0].related_len(), 2);
}

/// A missing generated anchor is reported once for every affected document and anchor, not
/// once per matching declaration. The stored rule outcome remains the query result.
#[test]
fn generated_library_anchor_diagnostics_deduplicate_by_anchor_and_document() {
    const ITEM_RULE: &str = "sysml-2.0:8.3.10.2:checkItemDefinitionSpecialization";
    let published = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://incomplete.sysml",
                    "standard library package Incomplete {}".to_string(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://model.sysml",
                    "package Model { item def First; item def Second; }".to_string(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();

    assert!(matches!(
        published.library_specialization_anchor(ITEM_RULE),
        QueryOutcome::Unresolved
    ));
    let published_diagnostics = published.diagnostics();
    let diagnostics = published_diagnostics.iter().collect::<Vec<_>>();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code().as_str(), "missing_library_anchor");
    assert!(diagnostics[0].message().contains("Items::Item"));
}

/// Recovery-produced input is still answered, and the outcome says the publication recovered
/// rather than presenting the answer as complete.
#[test]
fn element_details_over_recovery_produced_input_keep_their_recovery_outcome() {
    let published = detail_publication(
        &[(
            "memory://recovery.sysml",
            "package P { part def Wheel; part broken : ; }",
        )],
        ConstructionSchedule::Sequential,
    );
    let symbol = identity_of(&published, "memory://recovery.sysml", "P::Wheel");
    assert!(
        matches!(
            published.element_details(symbol),
            QueryOutcome::Recovered(_) | QueryOutcome::UnsupportedWith(_)
        ),
        "expected a degraded publication to say so, got: {:?}",
        published.completeness()
    );
}

#[test]
fn an_unresolved_import_makes_dependency_selection_explicitly_recovered() {
    let published = build(
        BuildRequest::new(
            vec![SourceInput::new(
                "memory://a.sysml",
                "package A { import Missing::*; }".into(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    assert!(matches!(
        published.affected_documents("memory://a.sysml"),
        QueryOutcome::Recovered(_)
    ));
}
