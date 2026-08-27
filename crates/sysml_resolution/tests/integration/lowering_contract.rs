//! Contract tests for the lowering phase, driven through the crate's public
//! `build()` / `PublishedResolution` surface. Relocated verbatim from the inline
//! `#[cfg(test)]` modules of `src/lib.rs` and `src/model.rs`.

#![allow(clippy::too_many_lines)]

#[allow(unused_imports)]
use crate::common::*;
#[allow(unused_imports)]
use sysml_resolution::*;

/// Every `variant` spelling delegates to the lowering its ordinary spelling already uses.
///
/// Every kind wraps exactly the node its plain spelling does, so each reuses that lowering while
/// the enclosing variant production records the canonical `VariantMembership` role. The
/// `body.is_none()` guard stays on all forms -- an outer `VariantUsage.body` is invisible to the
/// inner lowering, so lowering the inner declaration while dropping it would look complete while
/// being partial.
#[test]
fn every_variant_typed_usage_delegates_to_its_ordinary_lowering() {
    // Every kind is placed in a `variation part def` body, whose `PartDefBodyElement` is one of
    // the member sets that carries a `VariantUsage` variant at all.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Engine;\n\
         \titem def Widget;\n\
         \tport def Plug;\n\
         \trequirement def Req;\n\
         \tvariation part def V {\n\
         \t\tvariant part e : Engine;\n\
         \t\tvariant item w : Widget;\n\
         \t\tvariant port p : Plug;\n\
         \t\tvariant requirement r : Req;\n\
         \t}\n\
         }\n",
    );
    for (label, qualified_name, kind) in [
        ("variant part", "Demo::V::e", "(kind part)"),
        ("variant item", "Demo::V::w", "(kind item)"),
        ("variant port", "Demo::V::p", "(kind port)"),
        ("variant requirement", "Demo::V::r", "(kind requirement)"),
    ] {
        let expected = format!("(qualified-name \"{qualified_name}\")");
        let line = output
            .lines()
            .find(|line| line.contains(&expected) && line.contains("(declaration "));
        let line = match line {
            Some(line) => line,
            None => panic!("no declaration for {label}, got:\n{output}"),
        };
        assert!(
            line.contains(kind),
            "expected {label} to lower as {kind}, got:\n{line}"
        );
    }

    let publication = detail_publication(
        &[(
            "memory://variants.sysml",
            "package Demo { part def Engine; variation part def V { variant part e : Engine; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let variant = identity_of(&publication, "memory://variants.sysml", "Demo::V::e");
    assert!(matches!(
        publication.inspect(variant).answer,
        QueryAnswer::Resolved(ElementInspection {
            role: Some(MembershipRole::Variant),
            ..
        })
    ));

    // `variant attribute` inside a `variation attribute def` body never reaches this lowering:
    // `ast::AttributeBodyElement` has no `VariantUsage` variant at all, so the member is
    // dropped upstream. Pinned here so the silence is visible rather than mistaken for
    // coverage; see planning/UPSTREAM_PARSER_GAPS.md.
    let attribute_variant = build_semantic_sexpr(
        "package Demo {\n\tattribute def Size;\n\tvariation attribute def V :> Size {\n\t\tvariant attribute a;\n\t}\n}\n",
    );
    assert!(
        !attribute_variant.contains("(qualified-name \"Demo::V::a\")"),
        "a `variant attribute` member became representable upstream; dispatch it here and \
         retire the gap entry, got:\n{attribute_variant}"
    );

    // A brace after the typing belongs to the *inner* usage, so `VariantUsage.body` is None and
    // the member lowers in full, owned members and all. The `body.is_none()` guard is about the
    // untyped `variant x { ... }` spelling, where the body has no inner node to belong to.
    let bodied = build_semantic_sexpr(
        "package Demo {\n\tpart def Engine;\n\tvariation part def V {\n\t\tvariant part e : Engine {\n\t\t\tattribute x;\n\t\t}\n\t}\n}\n",
    );
    assert!(
        bodied.contains("(qualified-name \"Demo::V::e::x\")"),
        "expected a typed variant's brace body to lower as the inner usage's own, got:\n{bodied}"
    );
}

#[test]
fn enum_def_lowers_to_a_declaration_with_its_literal_as_an_owned_member() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tenum def StatusKind {\n\
         \t\tenum approved;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::StatusKind\"))) (kind enum-def)"),
        "expected an enum-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::StatusKind::approved\"))) (kind enum-literal)"),
        "expected an owned enum-literal declaration with its own qualified name, got:\n{output}"
    );
}

#[test]
fn requirement_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trequirement def MassRequirement {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::MassRequirement\"))) (kind requirement-def)"),
        "expected a requirement-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::MassRequirement::mass\"))) (kind attribute)"),
        "expected an owned attribute declaration under the requirement def, got:\n{output}"
    );
}

#[test]
fn port_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tport def InputPort {\n\
         \t\tattribute level : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::InputPort\"))) (kind port-def)"),
        "expected a port-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::InputPort::level\"))) (kind attribute)"),
        "expected an owned attribute declaration under the port def, got:\n{output}"
    );
}

#[test]
fn connection_def_lowers_to_a_declaration() {
    // Bare `end name;` (no `:` type, `::>`/`references` target, or nested occurrence/item
    // usage) is not valid `EndDecl` grammar at all -- confirmed against the upstream parser's
    // `end_decl` (`src/parser/connector.rs`), which requires one of those three forms after
    // the name -- so a real end declaration must carry an explicit type here.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tport def P;\n\
         \tconnection def C {\n\
         \t\tend end1 : P;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::C\"))) (kind connection-def)"),
        "expected a connection-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::C::end1\"))) (kind connection)"),
        "expected an owned end declaration under the connection def, got:\n{output}"
    );
}

#[test]
fn interface_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tport def P;\n\
         \tinterface def I {\n\
         \t\tend end1 : P;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::I\"))) (kind interface-def)"),
        "expected an interface-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::I::end1\"))) (kind connection)"),
        "expected an owned end declaration under the interface def, got:\n{output}"
    );
}

#[test]
fn view_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tview def V;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::V\"))) (kind view-def)"),
        "expected a view-def declaration, got:\n{output}"
    );
}

#[test]
fn constraint_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::C\"))) (kind constraint-def)"),
        "expected a constraint-def declaration, got:\n{output}"
    );
}

#[test]
fn concern_def_lowers_to_a_declaration() {
    // planning/UPSTREAM_PARSER_GAPS.md #9 was resolved upstream in `0757de13`: `ConcernUsage`
    // (which models both `concern def` and `concern` textual forms) now carries a
    // `type_name`/`subsets`/`redefines` field at all, previously entirely blocked.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconcern def C;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::C\"))) (kind concern-def)"),
        "expected a concern-def declaration, got:\n{output}"
    );
}

#[test]
fn calc_def_lowers_to_a_declaration() {
    // planning/UPSTREAM_PARSER_GAPS.md #3 was resolved upstream in `0757de13`: `CalcDef` now carries a
    // `specializes` field. `calc def`/`calc` usage are only reachable inside a part body in
    // the typed AST (`calc_usage` is not dispatched at package level).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P {\n\
         \t\tcalc def Calc;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::P::Calc\"))) (kind calc-def)"),
        "expected a calc-def declaration, got:\n{output}"
    );
}

#[test]
fn occurrence_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \toccurrence def Occ {\n\
         \t\titem x;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Occ\"))) (kind occurrence-def)"),
        "expected an occurrence-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::Occ::x\"))) (kind item)"),
        "expected an owned item usage under the occurrence def, got:\n{output}"
    );
}

#[test]
fn analysis_case_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tanalysis def FuelEconomyAnalysis {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::FuelEconomyAnalysis\"))) (kind analysis-def)"),
        "expected an analysis-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::FuelEconomyAnalysis::mass\"))) (kind attribute)"),
        "expected an owned attribute declaration under the analysis def, got:\n{output}"
    );
}

#[test]
fn analysis_case_usage_nested_in_an_analysis_def_body_lowers_to_a_declaration() {
    // planning/UPSTREAM_PARSER_GAPS.md #5 was resolved upstream in `0757de13`: `AnalysisCaseUsage` now
    // carries `subsets`/`redefines` fields with full parity to `RequirementUsage`, so a nested
    // `analysis` usage inside an `analysis def` body must lower as its own `analysis`
    // declaration with its `:` typing target resolved, not fall through to
    // `unsupported_analysis_case_definition_member`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tanalysis def Outer {\n\
         \t\tanalysis inner : Outer;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("Demo::Outer::inner"),
        "expected nested analysis usage declaration, got:\n{output}"
    );
    assert!(
        !output.contains("unsupported_analysis_case_definition_member"),
        "did not expect unsupported_analysis_case_definition_member, got:\n{output}"
    );
    assert!(
        output.contains("(kind analysis)"),
        "expected inner to lower with kind analysis, got:\n{output}"
    );
}

#[test]
fn case_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcase def Investigation {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Investigation\"))) (kind case-def)"),
        "expected a case-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::Investigation::mass\"))) (kind attribute)"),
        "expected an owned attribute declaration under the case def, got:\n{output}"
    );
}

#[test]
fn case_usage_lowers_to_a_declaration_with_its_subsetting_resolved() {
    // planning/UPSTREAM_PARSER_GAPS.md #5 was resolved upstream in `0757de13`: `CaseUsage` now carries
    // `subsets`/`redefines` fields with full parity to `RequirementUsage`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcase baseCase;\n\
         \tcase derivedCase :> baseCase;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::derivedCase\"))) (kind case)"),
        "expected a case usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derivedCase\")))"
        ),
        "expected derivedCase's subsetting of baseCase to resolve, got:\n{output}"
    );
}

#[test]
fn case_definition_member_nested_action_usage_lowers_to_a_declaration() {
    // A nested `action` usage inside a `case def` body dispatches through the
    // `UseCaseDefBodyElement::ActionUsage` -> `lower_action_usage` wiring shared with
    // `use case def`/`verification def` bodies (they all use `UseCaseDefBody`).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcase def Outer {\n\
         \t\taction inner;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Outer::inner\"))) (kind action)"),
        "expected an owned action usage declaration under the case def, got:\n{output}"
    );
}

#[test]
fn verification_case_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tverification def RangeVerification {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::RangeVerification\"))) (kind verification-def)"),
        "expected a verification-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::RangeVerification::mass\"))) (kind attribute)"),
        "expected an owned attribute declaration under the verification def, got:\n{output}"
    );
}

#[test]
fn verification_case_definition_member_nested_action_usage_lowers_to_a_declaration() {
    // Same `UseCaseDefBodyElement::ActionUsage` -> `lower_action_usage` wiring as
    // `case_definition_member_nested_action_usage_lowers_to_a_declaration`, exercised
    // through the `verification def` body shape.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tverification def Outer {\n\
         \t\taction inner;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Outer::inner\"))) (kind action)"),
        "expected an owned action usage declaration under the verification def, got:\n{output}"
    );
}

#[test]
fn use_case_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tuse case def PurchaseTicket {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::PurchaseTicket\"))) (kind use-case-def)"),
        "expected a use-case-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::PurchaseTicket::mass\"))) (kind attribute)"),
        "expected an owned attribute declaration under the use case def, got:\n{output}"
    );
}

#[test]
fn use_case_definition_member_nested_action_usage_lowers_to_a_declaration() {
    // Same `UseCaseDefBodyElement::ActionUsage` -> `lower_action_usage` wiring as
    // `case_definition_member_nested_action_usage_lowers_to_a_declaration`, exercised
    // through the `use case def` body shape.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tuse case def Outer {\n\
         \t\taction inner;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Outer::inner\"))) (kind action)"),
        "expected an owned action usage declaration under the use case def, got:\n{output}"
    );
}

#[test]
fn item_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \titem def Widget {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Widget\"))) (kind item-def)"),
        "expected an item-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
        "expected an owned attribute declaration under the item def, got:\n{output}"
    );
}

#[test]
fn constraint_def_in_parameter_lowers_and_resolves_with_a_direction_fact() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute def MassValue;\n\
         \tconstraint def MassConstraint {\n\
         \t\tin partMasses : MassValue;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::MassConstraint::partMasses\"))) (kind parameter)"),
        "expected a parameter declaration for partMasses, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (direction in) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassConstraint::partMasses\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassValue\")))"
        ),
        "expected partMasses's typing reference to MassValue to resolve with an `in` direction fact, got:\n{output}"
    );
}

#[test]
fn calc_def_out_parameter_lowers_and_resolves_with_a_direction_fact() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute def Real;\n\
         \tcalc def Sum {\n\
         \t\tout result : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Sum::result\"))) (kind parameter)"),
        "expected a parameter declaration for result, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (direction out) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Sum::result\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Real\")))"
        ),
        "expected result's typing reference to Real to resolve with an `out` direction fact, got:\n{output}"
    );
}

#[test]
fn action_def_inout_parameter_lowers_and_resolves_with_a_direction_fact() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \titem def Image;\n\
         \taction def Focus {\n\
         \t\tinout image : Image;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Focus::image\"))) (kind parameter)"),
        "expected a parameter declaration for image, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (direction inout) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Focus::image\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Image\")))"
        ),
        "expected image's typing reference to Image to resolve with an `inout` direction fact, got:\n{output}"
    );
}

#[test]
fn calc_def_untyped_parameter_still_lowers_a_declaration_shell() {
    // `in seq[1..*];` (BNF `InOutDecl` with no `type_name`, only a multiplicity) must still
    // lower as a declaration -- no `FeatureTyping`/direction fact is pushed for it (there is
    // no type to reference), but the declaration/membership shell is not skipped. Mirrors
    // `sysml.library/interfaces.md`'s `excludingOnce` calc's `in seq[1..*] nonunique ordered;`
    // line minus the `nonunique`/`ordered` collection modifiers, which are lowered as their
    // own modifier facts and are not what this test pins.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def ExcludingOnce {\n\
         \t\tin seq[1..*];\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::ExcludingOnce::seq\"))) (kind parameter)"),
        "expected a parameter declaration for untyped seq, got:\n{output}"
    );
    assert!(
        !output.contains("(kind typing)"),
        "expected no FeatureTyping reference for untyped seq, got:\n{output}"
    );
}

#[test]
fn calc_def_anonymous_redefinition_parameter_lowers_its_redefines_relationship() {
    // The leading `in :>> target = expr;` spelling is the one case that actually populates
    // `ast::InOutDecl::redefines` (a `Node<SubsettingRelationship>`), independent of whether a
    // type is present (`type_name` stays `None` here). `lower_parameter_declaration` now
    // lowers this via the same `lower_subsetting_relationship` helper `AttributeUsage`/
    // `ItemUsage` already call, so the redefinition target reference resolves.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Sum {\n\
         \t\tin target;\n\
         \t\tin :>> target = 1;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind redefinition) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Sum::\")))"
        ) || output.contains("(kind redefinition)"),
        "expected an anonymous parameter's redefinition reference to lower, got:\n{output}"
    );
    assert!(
        output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Sum::target\")))"
        ),
        "expected the redefinition reference to resolve to target, got:\n{output}"
    );
}

#[test]
fn requirement_subject_declaration_lowers_and_resolves_its_typing() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Vehicle;\n\
         \trequirement vehicleSpecification {\n\
         \t\tsubject vehicle : Vehicle;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output
            .contains("(qualified-name \"Demo::vehicleSpecification::vehicle\"))) (kind subject)"),
        "expected a subject declaration for vehicle, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::vehicleSpecification::vehicle\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Vehicle\")))"
        ),
        "expected vehicle's typing reference to Vehicle to resolve, got:\n{output}"
    );
}

#[test]
fn use_case_subject_declaration_lowers_and_resolves_its_typing() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Vehicle;\n\
         \tcase def Inspect {\n\
         \t\tsubject vehicle : Vehicle;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Inspect::vehicle\"))) (kind subject)"),
        "expected a subject declaration for vehicle, got:\n{output}"
    );
}

#[test]
fn requirement_actor_declaration_lowers_and_resolves_its_typing() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Operator;\n\
         \trequirement def FlightRequirement {\n\
         \t\tactor pilot : Operator;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(qualified-name \"Demo::FlightRequirement::pilot\"))) (kind requirement-actor)"
        ),
        "expected a requirement-actor declaration for pilot, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::FlightRequirement::pilot\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Operator\")))"
        ),
        "expected pilot's typing reference to Operator to resolve, got:\n{output}"
    );
}

#[test]
fn stakeholder_typed_declaration_lowers_and_resolves_its_typing() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Driver;\n\
         \trequirement def SafetyRequirement {\n\
         \t\tstakeholder driver : Driver;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output
            .contains("(qualified-name \"Demo::SafetyRequirement::driver\"))) (kind stakeholder)"),
        "expected a stakeholder declaration for driver, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::SafetyRequirement::driver\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Driver\")))"
        ),
        "expected driver's typing reference to Driver to resolve, got:\n{output}"
    );
}

#[test]
fn perform_action_usage_inside_a_part_def_lowers_and_resolves_its_typing() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \taction def GenerateTorque;\n\
         \tpart def Engine {\n\
         \t\tperform action generateTorque: GenerateTorque;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output
            .contains("(qualified-name \"Demo::Engine::generateTorque\"))) (kind perform-action)"),
        "expected a perform-action declaration for generateTorque, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Engine::generateTorque\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::GenerateTorque\")))"
        ),
        "expected generateTorque's typing reference to GenerateTorque to resolve, got:\n{output}"
    );
}

#[test]
fn perform_action_usage_inside_an_action_def_lowers_and_resolves_its_typing() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \taction def Sub;\n\
         \taction def Main {\n\
         \t\tperform action step: Sub;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Main::step\"))) (kind perform-action)"),
        "expected a perform-action declaration for step, got:\n{output}"
    );
}

#[test]
fn class_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tclass Widget {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Widget\"))) (kind class-def)"),
        "expected a class-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
        "expected an owned attribute declaration under the class def, got:\n{output}"
    );
}

#[test]
fn kerml_classifier_decl_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tstruct Widget {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Widget\"))) (kind kerml-structure)"),
        "expected `struct Widget` to lower as KerML `Structure`, not a generic classifier \
         bucket, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
        "expected an owned attribute declaration under the struct, got:\n{output}"
    );
}

/// Each KerML classifier keyword denotes its own concrete metaclass -- the spec makes them a
/// subtype lattice (`Predicate <: Function <: Behavior <: Class <: Classifier <: Type`,
/// `Structure <: Class`, `Interaction <: Association, Behavior`, `Multiplicity <: Feature`) --
/// and `ast::KermlClassifierDecl.keyword` already carries the spelling, so none of them may
/// collapse into a shared bucket. `assoc` and `association` are two spellings of one keyword
/// and so are the one exception; `subclassifier` is not a classifier keyword at all -- it
/// declares a subclassification *relationship* (`ast::KermlRelationshipDecl`), which this
/// slice reports as an unsupported package member.
#[test]
fn each_kerml_classifier_keyword_lowers_to_its_own_metaclass() {
    for (source, kind) in [
        ("type K;", "kerml-type"),
        ("classifier K;", "kerml-classifier"),
        ("struct K;", "kerml-structure"),
        ("assoc K;", "kerml-association"),
        ("association K;", "kerml-association"),
        ("assoc struct K;", "kerml-association-structure"),
        ("datatype K;", "kerml-datatype"),
        ("metaclass K;", "kerml-metaclass"),
        ("behavior K;", "kerml-behavior"),
        ("function K;", "kerml-function"),
        ("predicate K;", "kerml-predicate"),
        ("interaction K;", "kerml-interaction"),
        ("multiplicity K [0..1];", "kerml-multiplicity"),
    ] {
        // Both spellings reach `KermlClassifierDecl`: the bare forward declaration as a `;`
        // body, and the bodied form as a brace body. Each must land on the same metaclass.
        let bodied = source.replace(';', " { }");
        for spelling in [source, bodied.as_str()] {
            let output = build_semantic_sexpr(&format!("package Demo {{\n\t{spelling}\n}}\n"));
            assert!(
                output.contains(&format!("(qualified-name \"Demo::K\"))) (kind {kind})")),
                "expected `{spelling}` to lower as {kind}, got:\n{output}"
            );
        }
    }

    // A plain `class K { }` is claimed by the dedicated `class_def` production, so it lowers
    // as `ClassDefinition`; `KermlClassifierKeyword::Class` is reached only for the shapes
    // `class_def` rejects (see that variant's own doc comment).
    let class_def = build_semantic_sexpr("package Demo {\n\tclass K { }\n}\n");
    assert!(
        class_def.contains("(qualified-name \"Demo::K\"))) (kind class-def)"),
        "expected plain `class` to keep using the dedicated class-def production, got:\n\
         {class_def}"
    );
}

/// The same, for the KerML feature kind keywords: `BooleanExpression <: Expression <: Step <:
/// Feature` are four distinct metaclasses, carried by `ast::KermlFeatureMember.kind`.
#[test]
fn each_kerml_feature_keyword_lowers_to_its_own_metaclass() {
    for (source, kind) in [
        ("feature f : Real;", "kerml-feature"),
        ("step f : Real;", "kerml-step"),
        ("expr f : Real;", "kerml-expression"),
        ("bool f : Real;", "kerml-boolean-expression"),
    ] {
        let output = build_semantic_sexpr(&format!(
            "package Demo {{\n\tstruct S {{\n\t\tderived {source}\n\t}}\n}}\n"
        ));
        assert!(
            output.contains(&format!("(qualified-name \"Demo::S::f\"))) (kind {kind})")),
            "expected `{source}` to lower as {kind}, got:\n{output}"
        );
    }
}

#[test]
fn kerml_classifier_decl_nested_inside_calc_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Outer {\n\
         \t\tstruct Inner;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Outer::Inner\"))) (kind kerml-structure)"),
        "expected a nested `struct` declaration inside the calc def, got:\n{output}"
    );
}

#[test]
fn kerml_feature_member_lowers_to_a_declaration_with_typing() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tderived feature x : Integer;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::x\"))) (kind kerml-feature)"),
        "expected a kerml-feature declaration for x, got:\n{output}"
    );
    assert!(
        output.contains("(relationships (featureTyping (reference \"Integer\")))"),
        "expected x's FeatureTyping reference, got:\n{output}"
    );
}

#[test]
fn kerml_connector_member_lowers_ends_and_typing() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tclassifier Bicycle {\n\
         \t\tfeature rollsOn : Wheel;\n\
         \t\tfeature holdsWheel : BikeFork;\n\
         \t\tconnector fixWheel : BikeWheelFixed from rollsOn to holdsWheel;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Bicycle::fixWheel\"))) (kind kerml-connector)"),
        "expected a kerml-connector declaration for fixWheel, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Bicycle::fixWheel\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Bicycle::rollsOn\")))"
        ),
        "expected fixWheel's `from` end to resolve to rollsOn, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Bicycle::fixWheel\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Bicycle::holdsWheel\")))"
        ),
        "expected fixWheel's `to` end to resolve to holdsWheel, got:\n{output}"
    );
}

#[test]
fn kerml_binding_member_lowers_left_and_right_ends() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tclassifier Bicycle {\n\
         \t\tfeature startShot : Integer;\n\
         \t\tfeature endShot : Integer;\n\
         \t\tbinding startShot = endShot;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind kerml-binding)"),
        "expected a kerml-binding declaration, got:\n{output}"
    );
    assert!(
        output.contains("(kind bindSource)") && output.contains("(kind bindTarget)"),
        "expected bindSource/bindTarget references for startShot/endShot, got:\n{output}"
    );
}

#[test]
fn kerml_invariant_member_lowers_its_boolean_expression() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tclassifier Bicycle {\n\
         \t\tfeature isClosed : Boolean;\n\
         \t\tinv unitBound {\n\
         \t\t\tisClosed\n\
         \t\t}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Bicycle::unitBound\"))) (kind kerml-invariant)"),
        "expected a kerml-invariant declaration for unitBound, got:\n{output}"
    );
}

#[test]
fn calc_def_nested_inside_calc_def_lowers_to_a_declaration() {
    // `CalcDefBodyElement::CalcDef`/`CalcUsage`/`PartUsage` dispatch into a `calc def`
    // body's own already-existing `lower_calc_def`/`lower_calc_usage`/`lower_part_usage`
    // functions, mirroring the same nesting already supported inside `part def` bodies.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Outer {\n\
         \t\tcalc def Inner;\n\
         \t\tcalc rollup : Inner;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Outer::Inner\"))) (kind calc-def)"),
        "expected a nested calc-def declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Outer::rollup\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Outer::Inner\")))"
        ),
        "expected the nested calc usage's typing reference to Inner to resolve, got:\n{output}"
    );
}

#[test]
fn kerml_succession_member_lowers_first_and_then_ends() {
    // `CalcDefBodyElement::Succession` (`KermlSuccessionMember`) was previously
    // unconditionally unsupported despite `lower_kerml_connector_end` already existing to
    // lower its identical `KermlConnectorEnd`-shaped operands (see the exhaustive
    // `unsupported_calc_definition_member` audit's `a_3_6_sequences.md`/
    // `a_3_7_decisions_and_merges.md` KerML Spec Annex A fixtures).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tbehavior Manufacture {\n\
         \t\tstep paint : Paint;\n\
         \t\tstep dry : Dry;\n\
         \t\tsuccession p_before_d first paint then dry;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Manufacture::p_before_d\"))) (kind succession)"),
        "expected a succession declaration for p_before_d, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind succession) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Manufacture::p_before_d\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Manufacture::paint\")))"
        ),
        "expected p_before_d's first end to resolve to paint, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind succession) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Manufacture::p_before_d\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Manufacture::dry\")))"
        ),
        "expected p_before_d's then end to resolve to dry, got:\n{output}"
    );
    assert!(
        !output.contains("unsupported_calc_definition_member"),
        "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
    );
}

#[test]
fn calc_def_body_flow_usage_lowers_its_ends_and_payload() {
    // KerML 8.2's `Flow` in a calc-shaped body. The pinned parser types the whole declaration
    // (payload feature plus two `KermlConnectorEnd`s), so it lowers through the same
    // `lower_flow_usage` an action body uses instead of reporting an unsupported member.
    // Unblocks `tests/snapshots/validation/kerml_flow_end_is_end.md` and its two siblings.
    let output = build_semantic_sexpr(
        "package Flows {\n\
         \tclassifier Thing;\n\
         \tbehavior Moving {\n\
         \t\tfeature source : Thing;\n\
         \t\tfeature target : Thing;\n\
         \t\tflow of Thing from source to target;\n\
         \t}\n\
         }\n",
    );
    assert!(
        !output.contains("unsupported_calc_definition_member"),
        "expected the KerML flow member to lower rather than be unsupported, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind flowSource) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Flows\")) (named (kind kerml-behavior) (name \"Moving\")) (anonymous (kind flow) (ordinal 0))))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Flows::Moving::source\")))"
        ),
        "expected the flow's `from` end to resolve to source, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind flowTarget) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Flows\")) (named (kind kerml-behavior) (name \"Moving\")) (anonymous (kind flow) (ordinal 0))))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Flows::Moving::target\")))"
        ),
        "expected the flow's `to` end to resolve to target, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind flowPayloadType) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Flows\")) (named (kind kerml-behavior) (name \"Moving\")) (anonymous (kind flow) (ordinal 0))))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Flows::Thing\")))"
        ),
        "expected the `of Thing` payload type to resolve, got:\n{output}"
    );
}

#[test]
fn declared_verify_requirement_member_lowers_as_a_verify_requirement_usage() {
    // `verify requirement <name> : <Type>;` declares an inline requirement usage rather than
    // referencing an existing one. It is the same `RequirementUsage` production an ordinary
    // `requirement` member spells, so it lowers through the shared walker under
    // `DeclarationKind::VerifyRequirement` -- the kind `membership_role` reads to derive
    // `MembershipRole::RequirementVerification`, which is the prerequisite of the generated
    // `checkRequirementUsageRequirementVerificationSpecialization` library specialization.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trequirement def Limit;\n\
         \tverification def VerificationCase {\n\
         \t\tobjective {\n\
         \t\t\tverify requirement limit : Limit;\n\
         \t\t}\n\
         \t}\n\
         }\n",
    );
    assert!(
        !output.contains("unsupported_requirement_definition_member"),
        "expected the declared verify member to lower rather than be unsupported, got:\n\
         {output}"
    );
    assert!(
        output.contains(
            "(qualified-name \"Demo::VerificationCase::objective::limit\"))) (kind verify-requirement)"
        ),
        "expected a named verify-requirement declaration for limit, got:\n{output}"
    );
    assert!(
        output.contains(
            "(authored-target \"Limit\")\n      (outcome (status resolved) (target (node \
             (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Limit\")))))"
        ),
        "expected limit's typing to resolve to the Limit requirement def, got:\n{output}"
    );
}

#[test]
fn calc_def_body_assert_constraint_member_lowers_to_a_declaration() {
    // `CalcDefBodyElement::AssertConstraint` was previously unconditionally unsupported
    // despite `lower_assert_constraint_member` already existing (wired for
    // `ConstraintDefBodyElement`/case-family bodies) -- pure mechanical dispatch wiring, same
    // shape as `9_6cbf` originally added it for. Real-corpus site: Kernel Semantic Library
    // `ScalarValues.kerml`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def C {\n\
         \t\tin a : Boolean;\n\
         \t\tassert constraint check : Boolean {\n\
         \t\t\ta\n\
         \t\t}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::C::check\"))) (kind assert-constraint)"),
        "expected an assert-constraint declaration for check, got:\n{output}"
    );
    assert!(
        !output.contains("unsupported_calc_definition_member"),
        "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
    );
}

#[test]
fn calc_def_body_import_member_lowers_its_target() {
    // `CalcDefBodyElement::Import` was previously unconditionally unsupported despite
    // `lower_import` already accepting an `Option<DeclarationId>` owner -- pure mechanical
    // dispatch wiring. Real-corpus site: Kernel Function/Semantic Libraries' `private import
    // ...;`/`comment`-adjacent members inside a `calc def`/KerML classifier body.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpackage Other {\n\
         \t\tattribute def X;\n\
         \t}\n\
         \tcalc def C {\n\
         \t\tprivate import Other::*;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind import)"),
        "expected an import declaration owned by C, got:\n{output}"
    );
    assert!(
        !output.contains("unsupported_calc_definition_member"),
        "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
    );
}

#[test]
fn calc_def_body_comment_member_is_ignored() {
    // `CalcDefBodyElement::Comment` mirrors `PartDefBodyElement::Comment`/
    // `PackageBodyElement::Comment`'s existing inert no-op treatment (like `Doc`) rather than
    // being unconditionally unsupported.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def C {\n\
         \t\tcomment /* a note about C */\n\
         \t}\n\
         }\n",
    );
    assert!(
        !output.contains("unsupported_calc_definition_member"),
        "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
    );
}

#[test]
fn constraint_usage_nested_inside_requirement_def_lowers_to_a_declaration() {
    // `RequirementDefBodyElement::Constraint` dispatches into the already-existing
    // `lower_constraint_usage`, mirroring the real Systems Library
    // `RequirementCheck`/`RequirementConstraintCheck` shape (redefining
    // `assumptions`/`constraints`).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def Base;\n\
         \trequirement def Outer {\n\
         \t\tconstraint assumptions : Base;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Outer::assumptions\"))) (kind constraint)"),
        "expected a nested constraint usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Outer::assumptions\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected assumptions' typing reference to Base to resolve, got:\n{output}"
    );
}

#[test]
fn alias_def_nested_inside_part_def_lowers_to_a_declaration() {
    // `PartDefBodyElement::AliasDef`/`PartUsageBodyElement::AliasDef` dispatch into the
    // already-existing `lower_alias_def` (previously only reachable at package scope).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P {\n\
         \t\tport porig;\n\
         \t\talias po for porig;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::P::po\"))) (kind alias)"),
        "expected a nested alias declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind aliasBinding) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::po\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::porig\")))"
        ),
        "expected po's alias binding to porig to resolve, got:\n{output}"
    );
}

#[test]
fn dependency_lowers_clients_and_suppliers() {
    // `PackageBodyElement::Dependency` dispatches into the new `lower_dependency`: each
    // client/supplier is resolved as its own authored reference.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart a;\n\
         \tpart b;\n\
         \tdependency Use from a to b;\n\
         }\n",
    );
    assert!(
        output.contains("(kind dependency)"),
        "expected a dependency declaration, got:\n{output}"
    );
    assert!(
        output.contains("(kind dependencyClient)")
            && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::a\")))"
            ),
        "expected a's dependencyClient reference to resolve, got:\n{output}"
    );
    assert!(
        output.contains("(kind dependencySupplier)")
            && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::b\")))"
            ),
        "expected b's dependencySupplier reference to resolve, got:\n{output}"
    );
}

#[test]
fn extended_definition_lowers_owned_members_and_specialization() {
    // `PackageBodyElement::ExtendedDefinition` dispatches into the new
    // `lower_extended_definition`, reusing `lower_package_body` for `#<keyword> def`'s
    // owned members and `lower_typing_relationship` for its `:>` clause.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Base;\n\
         \t#scenario def Failure :> Base {\n\
         \t\tattribute cause : Boolean;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Failure\"))) (kind extended-definition)"),
        "expected an extended-definition declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::Failure::cause\"))) (kind attribute)"),
        "expected Failure's nested attribute usage, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Failure\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Failure's specialization reference to Base to resolve, got:\n{output}"
    );
}

#[test]
fn individual_def_lowers_to_a_declaration_with_specialization() {
    // `PackageBodyElement::IndividualDef` dispatches into the new `lower_individual_def`,
    // mirroring `lower_item_def`/`lower_class_def`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Base;\n\
         \tindividual def Widget :> Base {\n\
         \t\tattribute mass : Real;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Widget\"))) (kind individual-definition)"),
        "expected an individual-definition declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
        "expected Widget's nested attribute usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Widget\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Widget's specialization reference to Base to resolve, got:\n{output}"
    );
}

#[test]
fn item_usage_nested_inside_port_def_lowers_to_a_declaration() {
    // `PortDefBodyElement::ItemDef`/`ItemUsage` and `PortBodyElement::ItemUsage` dispatch
    // into the already-existing `lower_item_def`/`lower_item_usage`. A `port def` body's
    // item usage must carry an `in`/`out`/`inout` direction prefix (BNF `directed_item_usage`
    // -- unlike a plain `port` usage body's undirected `item_usage`, see
    // `PortBodyElement::ItemUsage`).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \titem def Widget;\n\
         \tport def P {\n\
         \t\tin item w : Widget;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::P::w\"))) (kind item)"),
        "expected a nested item usage declaration under the port def, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::w\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Widget\")))"
        ),
        "expected w's typing reference to Widget to resolve, got:\n{output}"
    );
}

#[test]
fn metadata_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tmetadata def Safety {\n\
         \t\tattribute isMandatory : Boolean;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Safety\"))) (kind metadata-def)"),
        "expected a metadata-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::Safety::isMandatory\"))) (kind attribute)"),
        "expected an owned attribute declaration under the metadata def, got:\n{output}"
    );
}

#[test]
fn action_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \taction def ExecuteMission {\n\
         \t\taction validateRoute;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::ExecuteMission\"))) (kind action-def)"),
        "expected an action-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::ExecuteMission::validateRoute\"))) (kind action)"),
        "expected an owned nested action usage declaration under the action def, got:\n{output}"
    );
}

#[test]
fn state_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tstate def SD {\n\
         \t\tstate s;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::SD\"))) (kind state-def)"),
        "expected a state-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::SD::s\"))) (kind state)"),
        "expected an owned nested state usage declaration under the state def, got:\n{output}"
    );
}

#[test]
fn viewpoint_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tviewpoint def V;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::V\"))) (kind viewpoint-def)"),
        "expected a viewpoint-def declaration, got:\n{output}"
    );
}

#[test]
fn rendering_def_lowers_to_a_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trendering def R;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::R\"))) (kind rendering-def)"),
        "expected a rendering-def declaration, got:\n{output}"
    );
}

#[test]
fn allocation_def_lowers_to_a_declaration_with_connector_end_references() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Logical;\n\
         \tpart def Physical;\n\
         \tallocation def A {\n\
         \t\tend logical : Logical;\n\
         \t\tend physical : Physical;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::A\"))) (kind allocation-def)"),
        "expected an allocation-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::A::logical\"))) (kind connection)"),
        "expected an owned end declaration under the allocation def, got:\n{output}"
    );
}

#[test]
fn flow_def_lowers_to_a_declaration_with_connector_end_references() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tport def SupplierPort;\n\
         \tport def ConsumerPort;\n\
         \tflow def F {\n\
         \t\tend supplierPort : SupplierPort;\n\
         \t\tend consumerPort : ConsumerPort;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::F\"))) (kind flow-def)"),
        "expected a flow-def declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::F::supplierPort\"))) (kind connection)"),
        "expected an owned end declaration under the flow def, got:\n{output}"
    );
}

#[test]
fn default_reference_usage_meta_cast_value_lowers_and_resolves() {
    // Keyword-less `<name> = <expr>;` binding (`ast::structure::DefaultReferenceUsage`),
    // e.g. `baseType = Atom meta KerML::Classifier;` inside a KerML `metaclass` body
    // (`tests/snapshots/kerml/a_2_atoms.md`). The declaration itself, and its `=` value's
    // `MetaCast` base/metaclass references, should both resolve, mirroring
    // `value_assignment_meta_cast_resolves_base_and_metaclass_target`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpackage KerML {\n\
         \t\tclass Classifier;\n\
         \t}\n\
         \tclass Atom;\n\
         \tmetaclass AtomMetadata {\n\
         \t\tbaseType = Atom meta KerML::Classifier;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind default-reference)")
            && output.contains("(qualified-name \"Demo::AtomMetadata::baseType\")"),
        "expected a DefaultReferenceUsage declaration for baseType, got:\n{output}"
    );
    assert!(
        output.contains("(kind expressionOperand) (ordinal 0))")
            && output.contains(
                "(outcome (status resolved) (target (node (document \
                 \"memory://test/enum.sysml\") (qualified-name \"Demo::Atom\")))))"
            ),
        "expected `Atom` to resolve as the meta cast's base operand, got:\n{output}"
    );
    assert!(
        output.contains("(kind metaCastTarget)")
            && output.contains(
                "(outcome (status resolved) (target (node (document \
                 \"memory://test/enum.sysml\") (qualified-name \"Demo::KerML::Classifier\")))))"
            ),
        "expected `KerML::Classifier` to resolve as the meta cast's metaclass target, got:\n{output}"
    );
}

/// A nested `part` usage inside an `attribute def` body (BNF `AttributeBodyElement::PartUsage`,
/// shared with `item def`/`item` usage bodies per the OMG `14c-Language Extensions.sysml`
/// FMEA library example) must lower as its own `part` declaration, not fall through to
/// `unsupported_attribute_member`.
#[test]
fn nested_part_usage_inside_attribute_def_body_lowers_as_part() {
    let sexpr = semantic_sexpr_for(
        "package P { attribute def Show { part frame : Frame; attribute def Frame; } }",
    );
    assert!(
        sexpr.contains("P::Show::frame"),
        "expected nested part declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_attribute_member"),
        "did not expect unsupported_attribute_member, got: {sexpr}"
    );
}

/// A nested `item` usage inside an `attribute def` body (BNF
/// `AttributeBodyElement::ItemUsage`, resolved upstream in `0757de13` --
/// planning/UPSTREAM_PARSER_GAPS.md #11) must lower as its own `item` declaration via the
/// already-existing `lower_item_usage`, not fall through to `unsupported_attribute_member`.
#[test]
fn nested_item_usage_inside_attribute_def_body_lowers_as_item() {
    let sexpr = semantic_sexpr_for(
        "package P { attribute def Show { item picture : Picture; attribute def Picture; } }",
    );
    assert!(
        sexpr.contains("P::Show::picture"),
        "expected nested item declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_attribute_member"),
        "did not expect unsupported_attribute_member, got: {sexpr}"
    );
}

/// A nested `occurrence` usage inside an `attribute def` body (BNF
/// `AttributeBodyElement::OccurrenceUsage`, e.g. the FMEA library's `#prevention occurs;`-style
/// members) must lower as its own `occurrence` declaration via the already-existing
/// `lower_occurrence_usage`, not fall through to `unsupported_attribute_member`.
#[test]
fn nested_occurrence_usage_inside_attribute_def_body_lowers_as_occurrence() {
    let sexpr = semantic_sexpr_for("package P { attribute def Show { occurrence flash; } }");
    assert!(
        sexpr.contains("P::Show::flash"),
        "expected nested occurrence declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_attribute_member"),
        "did not expect unsupported_attribute_member, got: {sexpr}"
    );
}

/// A nested `exhibit` state usage inside an `occurrence def`/usage body (BNF
/// `OccurrenceBodyElement::StateUsage`, e.g. `exhibit vehicleStates.on;` from the OMG spec
/// Annex's individuals/snapshots examples) must lower as its own `state` declaration via the
/// already-existing `lower_state_usage`, not fall through to
/// `unsupported_occurrence_definition_member`.
#[test]
fn nested_state_usage_inside_occurrence_def_body_lowers_as_state() {
    let sexpr = semantic_sexpr_for("package P { occurrence def O { exhibit vehicleStates.on; } }");
    assert!(
        sexpr.contains("(kind state)"),
        "expected nested state declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_occurrence_definition_member"),
        "did not expect unsupported_occurrence_definition_member, got: {sexpr}"
    );
}

/// A standalone `decide <name>;` decision control node (BNF `DecisionStmt`) lowers its
/// `ControlNodeDeclaration` as a named `DeclarationKind::Decide` feature. The name is not an
/// input reference to a sibling action.
#[test]
fn decide_stmt_lowers_a_named_control_node() {
    let sexpr = semantic_sexpr_for("package P { action def A { decide choice; } }");
    assert!(
        sexpr.contains("(qualified-name \"P::A::choice\"))) (kind decide)"),
        "expected a named decide declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
    assert!(!sexpr.contains("(kind decisionInput)"));
}

/// A `then decide <expr>;` continuation (`ThenTarget::Decide`) inside an action body must
/// lower through the same named-control-node dispatch as a standalone `decide` statement.
#[test]
fn then_decide_target_lowers_as_decide_declaration() {
    let sexpr = semantic_sexpr_for("package P { action def A { then decide choice; } }");
    assert!(
        sexpr.contains("(qualified-name \"P::A::choice\"))) (kind decide)"),
        "expected a named decide declaration reached via `then decide`, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// A `TextualRepresentation` (`language "..." /* ... */`) nested inside an `action def`, an
/// `action` usage, or a `requirement def` body is inert documentation content with no
/// resolvable semantic fact, mirroring the existing package-body and `ref` usage-body
/// treatment (`PackageBodyElement::TextualRep`/`RefBodyElement::TextualRep`, both silently
/// ignored alongside `Doc`). It must not be reported as an unsupported member.
#[test]
fn textual_representation_inside_action_def_body_is_ignored() {
    let sexpr =
        semantic_sexpr_for(r#"package P { action def A { language "alf" /* c.x = newX; */ } }"#);
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member for a TextualRep member, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(completeness complete)"),
        "expected TextualRep to be fully ignored (no parse-recovery/unsupported-syntax fallout), got: {sexpr}"
    );
}

/// Same as `textual_representation_inside_action_def_body_is_ignored`, but nested inside an
/// `action` usage body rather than an `action def` body.
#[test]
fn textual_representation_inside_action_usage_body_is_ignored() {
    let sexpr =
        semantic_sexpr_for(r#"package P { action a { language "alf" /* c.x = newX; */ } }"#);
    assert!(
        !sexpr.contains("unsupported_action_usage_member"),
        "did not expect unsupported_action_usage_member for a TextualRep member, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(completeness complete)"),
        "expected TextualRep to be fully ignored (no parse-recovery/unsupported-syntax fallout), got: {sexpr}"
    );
}

/// Same as `textual_representation_inside_action_def_body_is_ignored`, but nested inside a
/// `requirement def` body.
#[test]
fn textual_representation_inside_requirement_def_body_is_ignored() {
    let sexpr = semantic_sexpr_for(
        r#"package P { requirement def R { language "alf" /* c.x = newX; */ } }"#,
    );
    assert!(
        !sexpr.contains("unsupported_requirement_definition_member"),
        "did not expect unsupported_requirement_definition_member for a TextualRep member, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(completeness complete)"),
        "expected TextualRep to be fully ignored (no parse-recovery/unsupported-syntax fallout), got: {sexpr}"
    );
}

/// `UseCaseDefBodyElement::Objective` (`objective { ... }`/`objective <name> : <Type> { ... }`)
/// wraps a fully typed `RequirementUsage` (`Objective::requirement`) but was unconditionally
/// unsupported. Wires it through the existing `lower_requirement_usage` pipeline, the same as
/// every other requirement-usage site.
#[test]
fn case_family_objective_lowers_as_requirement_usage() {
    let sexpr =
        semantic_sexpr_for("package P { analysis def A { objective obj { doc /* g */ } } }");
    assert!(
        sexpr.contains("(kind requirement)"),
        "expected the objective's wrapped RequirementUsage to lower as a requirement, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_analysis_case_definition_member"),
        "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
    );
}

/// `PerformBodyElement::AttributeUsage` (an `in`/`out attribute` usage directly inside a
/// `perform` body, BNF §6 G6) was unconditionally unsupported despite being a fully typed
/// `AttributeUsage` node -- wires it through the already-existing `lower_attribute_usage`.
#[test]
fn perform_body_attribute_usage_lowers() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Vehicle { attribute mass; } part v : Vehicle; action def A { perform action doIt { in attribute mass :> v.mass; } } }",
    );
    assert!(
        sexpr.contains("(kind attribute)"),
        "expected an attribute declaration inside the perform body, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_usage_member"),
        "did not expect unsupported_action_usage_member, got: {sexpr}"
    );
}

/// `variant perform doX;` (BNF `VariantTypedUsage::Perform`, inside a `variation perform
/// action ... { ... }` body) was unconditionally unsupported both because `PerformBodyElement::
/// Variant` was never dispatched and because `lower_variant_usage` treated every typed variant
/// as out of scope; `Perform` now delegates to the already-existing `lower_perform`.
#[test]
fn variant_perform_lowers_as_perform_action_usage() {
    let sexpr = semantic_sexpr_for(
        "package P { action def Act { action doX; variation perform action doXorY { variant perform doX; } } }",
    );
    assert!(
        sexpr.contains("(kind perform-action)"),
        "expected a perform-action declaration for the variant, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_usage_member"),
        "did not expect unsupported_action_usage_member, got: {sexpr}"
    );
}

/// A `bind a = b { ... }` statement's braced body is a `PartUsageBody`, the same part-usage
/// member set a part usage body holds, but every element was unconditionally flagged
/// unsupported rather than dispatched through the shared
/// `lower_part_usage_body_element` -- confirmed against the Systems Library's `bind start =
/// done { doc /* ... */ }` shape (`Systems Library/Actions.sysml`): a `doc` comment nested in a
/// bind body must be recognized and bound to the owning `bind` declaration, not reported as an
/// unsupported member.
#[test]
fn bind_body_doc_comment_is_recorded() {
    let sexpr = semantic_sexpr_for(
        r#"package P { action def Act { first start; then done; bind start = done { doc /* note */ } } }"#,
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member for a doc comment in a bind body, got: {sexpr}"
    );
    assert!(
        sexpr.contains(r#"(documentation (doc (text " note ")))"#),
        "expected the bind body's doc comment recorded against the bind declaration, got: {sexpr}"
    );
}

/// Same as `bind_body_doc_comment_is_recorded`, but for real (non-`doc`) content: a nested
/// `part` usage inside a `bind ... { ... }` body must lower as its own `part` declaration
/// through the shared `lower_part_usage_body_element`.
#[test]
fn bind_body_nested_part_usage_lowers() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Widget; action def Act { first start; then done; bind start = done { part w : Widget; } } }",
    );
    assert!(
        sexpr.contains("(kind part)"),
        "expected a nested part declaration inside the bind body, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

// --- Canonical declaration facts -------------------------------------------------------
//
// These cover the authored presentation-adjacent facts (multiplicity, collection and
// declaration modifiers, direction, short name, documentation, and the authored feature-value
// spelling) recorded at each `lower_*` site. Every fact below has exactly one typed parser
// field behind it; none is recovered by re-reading authored text.

#[test]
fn declared_multiplicity_bounds_are_recorded_as_literals() {
    let sexpr = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel[0..4]; }");
    assert!(
        sexpr.contains("(multiplicity (lower 0) (upper 4))"),
        "expected literal multiplicity bounds, got: {sexpr}"
    );
}

#[test]
fn collection_modifiers_are_recorded() {
    let sexpr =
        semantic_sexpr_for("package P { attribute seq : Integer[1..*] ordered nonunique; }");
    assert!(
        sexpr.contains("(modifiers ordered nonunique)"),
        "expected both collection modifiers, got: {sexpr}"
    );
}

#[test]
fn definition_prefix_modifiers_are_recorded() {
    let abstract_def = semantic_sexpr_for("package P { abstract part def Vehicle; }");
    assert!(
        abstract_def.contains("(modifiers abstract)"),
        "expected the abstract prefix recorded, got: {abstract_def}"
    );

    let variation_def = semantic_sexpr_for("package P { variation part def Engine; }");
    assert!(
        variation_def.contains("(modifiers variation)"),
        "expected the variation prefix recorded, got: {variation_def}"
    );
}

#[test]
fn parameter_direction_is_recorded_as_a_declaration_fact() {
    let sexpr =
        semantic_sexpr_for("package P { calc def C { in x : Integer; return : Integer; } }");
    assert!(
        sexpr.contains("(facts (direction in))"),
        "expected the `in` direction recorded on the parameter declaration, got: {sexpr}"
    );
}

#[test]
fn authored_short_names_are_recorded() {
    let sexpr = semantic_sexpr_for("package <pkg> P { part def <w> Wheel; }");
    assert!(
        sexpr.contains(r#"(short-name "pkg")"#),
        "expected the package short name recorded, got: {sexpr}"
    );
    assert!(
        sexpr.contains(r#"(short-name "w")"#),
        "expected the part def short name recorded, got: {sexpr}"
    );
}

#[test]
fn comment_and_rep_annotations_are_recorded_as_distinct_forms() {
    let comment = semantic_sexpr_for(r#"package P { calc def C { comment /* note */ } }"#);
    assert!(
        comment.contains(r#"(comment (text " note "))"#),
        "expected the comment annotation recorded, got: {comment}"
    );

    // The corpus-proven spelling is the bare `language "..." /* ... */` form inside an action
    // def body; the `rep <name> language ...` spelling is not reachable in every scope.
    let rep = semantic_sexpr_for(r#"package P { action def A { language "Alf" /* body */ } }"#);
    assert!(
        rep.contains(r#"(rep (language "Alf") (text " body "))"#),
        "expected the textual representation recorded with its language, got: {rep}"
    );
}

/// A declaration whose parser node carries none of these facts -- and every synthesized
/// anonymous scope the lowering mints, which has no authored declaration syntax at all --
/// publishes no fact block, so absence stays absence rather than a defaulted answer.
#[test]
fn a_declaration_with_no_authored_facts_publishes_no_fact_block() {
    let sexpr = semantic_sexpr_for("package P { part def Wheel; }");
    assert!(
        !sexpr.contains("(facts "),
        "expected no fact block for a plain package and part def, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(documentation "),
        "expected no documentation block, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(feature-value "),
        "expected no feature-value block, got: {sexpr}"
    );
}
