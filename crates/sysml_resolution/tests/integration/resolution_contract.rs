//! Contract tests for the resolution phase, driven through the crate's public
//! `build()` / `PublishedResolution` surface. Relocated verbatim from the inline
//! `#[cfg(test)]` modules of `src/lib.rs` and `src/model.rs`.

#![allow(clippy::too_many_lines)]

#[allow(unused_imports)]
use crate::common::*;
#[allow(unused_imports)]
use sysml_resolution::*;

/// An enumeration literal owns the members and documentation authored in its body.
///
/// `EnumeratedValue.body` is a full `PartUsageBody`, the same shape `lower_part_usage` walks,
/// so its members go through the same `lower_part_usage_body_element`. Before it was walked, a
/// literal's redefinitions and its own doc comment were both unreachable -- the per-literal
/// half of the old Gap 56.
#[test]
fn enumeration_literal_bodies_publish_their_members_and_documentation() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute def Level {\n\
         \t\tattribute code : String;\n\
         \t}\n\
         \tenum def Kind specializes Level {\n\
         \t\tsecret {\n\
         \t\t\tdoc /* The secret level. */\n\
         \t\t\t:>> code = \"secr\";\n\
         \t\t}\n\
         \t}\n\
         }\n",
    );
    let line = output
        .lines()
        .find(|line| {
            line.contains("(qualified-name \"Demo::Kind::secret\")")
                && line.contains("(declaration ")
        })
        .unwrap_or_else(|| panic!("no enum literal declaration, got:\n{output}"));
    assert!(
        line.contains("(documentation (doc (text \" The secret level. \")))"),
        "expected the literal to publish its own doc comment, got:\n{line}"
    );
    assert!(
        output.contains("(named (kind enum-literal) (name \"secret\"))"),
        "expected the literal to own the members authored in its body, got:\n{output}"
    );
    assert!(
        output.contains("(redefinition (reference \"code\"))"),
        "expected the literal body's `:>>` redefinition to reach the model, got:\n{output}"
    );
}

/// The authored value spelling on a requirement subject and an enumeration literal.
///
/// `SubjectDecl.value` became a `FeatureValue` and `EnumeratedValue` gained one, so both can
/// record `=`/`:=`/`default` through the same `record_feature_value` every sibling usage
/// already calls. Only the spelling is recorded here; the value expression is not lowered,
/// matching `lower_item_usage`'s scope boundary.
#[test]
fn subjects_and_enumeration_literals_record_their_authored_value() {
    let subject = build_semantic_sexpr(
        "package Demo {\n\tpart def Vehicle;\n\tpart v : Vehicle;\n\trequirement def R {\n\t\tsubject s = v;\n\t}\n}\n",
    );
    let line = subject
        .lines()
        .find(|line| {
            line.contains("(qualified-name \"Demo::R::s\")") && line.contains("(declaration ")
        })
        .unwrap_or_else(|| panic!("no subject declaration, got:\n{subject}"));
    assert!(
        line.contains("(feature-value (kind bind)"),
        "expected the subject to record its `=` spelling, got:\n{line}"
    );

    let literal =
        build_semantic_sexpr("package Demo {\n\tenum def E {\n\t\tenum red = 1;\n\t}\n}\n");
    let line = literal
        .lines()
        .find(|line| {
            line.contains("(qualified-name \"Demo::E::red\")") && line.contains("(declaration ")
        })
        .unwrap_or_else(|| panic!("no enum literal declaration, got:\n{literal}"));
    assert!(
        line.contains("(feature-value (kind bind)"),
        "expected the enumeration literal to record its `=` spelling, got:\n{line}"
    );
}

/// `abstract` on a connection-like definition is published, and exempts its end count.
///
/// The four connection-like definitions gained a `definition_prefix` upstream. Until they
/// did, `structural.rs`'s "an abstract declaration is deliberately incomplete" guard could
/// never fire for them, so an abstract declaration authoring one end was reported as an
/// incomplete end pair. Both halves are asserted here: the modifier reaches the model, and
/// the diagnostic it suppresses is gone.
#[test]
fn abstract_connection_like_definitions_publish_the_modifier_and_skip_the_end_guard() {
    for (label, source, qualified_name) in [
        (
            "connection def",
            "package Demo {\n\tabstract connection def C {\n\t\tend a;\n\t}\n}\n",
            "Demo::C",
        ),
        (
            "flow def",
            "package Demo {\n\tabstract flow def F {\n\t\tend a;\n\t}\n}\n",
            "Demo::F",
        ),
        (
            "allocation def",
            "package Demo {\n\tabstract allocation def A {\n\t\tend a;\n\t}\n}\n",
            "Demo::A",
        ),
        (
            "interface def",
            "package Demo {\n\tabstract interface def I {\n\t\tend a;\n\t}\n}\n",
            "Demo::I",
        ),
    ] {
        let output = build_semantic_sexpr(source);
        let expected = format!("(qualified-name \"{qualified_name}\")");
        let line = output
            .lines()
            .find(|line| line.contains(&expected) && line.contains("(declaration "))
            .unwrap_or_else(|| panic!("no declaration for {label}, got:\n{output}"));
        assert!(
            line.contains("(modifiers abstract)"),
            "expected {label} to publish (modifiers abstract), got:\n{line}"
        );

        let diagnostics = build_diagnostics_sexpr(source);
        assert!(
            !diagnostics.contains("incomplete_connection_like_end_pair"),
            "expected abstract {label} to be exempt from the end-pair guard, got:\n{diagnostics}"
        );
    }

    // The guard still fires when the declaration is not abstract -- both sides of the rule.
    let concrete =
        build_diagnostics_sexpr("package Demo {\n\tconnection def C {\n\t\tend a;\n\t}\n}\n");
    assert!(
        concrete.contains("incomplete_connection_like_end_pair"),
        "expected a concrete one-ended connection def to still be reported, got:\n{concrete}"
    );
}

/// Every declaration kind whose parser node gained a `multiplicity` field publishes it.
///
/// Five lowerings passed no `multiplicity` because their nodes genuinely had no such field.
/// Upstream brought all five to sibling parity, and each carried a comment asserting the
/// absence that had become false.
#[test]
fn every_multiplicity_carrying_declaration_publishes_it() {
    for (label, source, qualified_name, bounds) in [
        (
            "attribute def",
            "package Demo {\n\tattribute def A[2];\n}\n",
            "Demo::A",
            "(multiplicity (lower 2) (upper 2))",
        ),
        (
            "constraint usage",
            "package Demo {\n\tconstraint c[3];\n}\n",
            "Demo::c",
            "(multiplicity (lower 3) (upper 3))",
        ),
        (
            "requirement usage",
            "package Demo {\n\trequirement r[4];\n}\n",
            "Demo::r",
            "(multiplicity (lower 4) (upper 4))",
        ),
        (
            "calc usage",
            "package Demo {\n\tcalc c1[5];\n}\n",
            "Demo::c1",
            "(multiplicity (lower 5) (upper 5))",
        ),
        (
            "requirement actor",
            "package Demo {\n\trequirement def R {\n\t\tactor a : Person[6];\n\t}\n}\n",
            "Demo::R::a",
            "(multiplicity (lower 6) (upper 6))",
        ),
    ] {
        let output = build_semantic_sexpr(source);
        let expected = format!("(qualified-name \"{qualified_name}\")");
        let line = output
            .lines()
            .find(|line| line.contains(&expected) && line.contains("(declaration "))
            .unwrap_or_else(|| panic!("no declaration for {label}, got:\n{output}"));
        assert!(
            line.contains(bounds),
            "expected {label} to publish {bounds}, got:\n{line}"
        );
    }
}

/// The header-level specialization clauses four lowerings used to drop.
///
/// `ItemUsage.subsets`, `KermlFeature.references`/`crosses`, `ViewpointUsage.subsets`/
/// `redefines` and `SubjectDecl.redefines` are all ordinary `SubsettingRelationship`s that the
/// shared `lower_subsetting_relationship` already maps; only the call was missing. `references`
/// and `crosses` publish as `unsupported` outcomes, which is the pre-existing treatment of
/// those two reference kinds -- the point here is that the authored clause reaches the model
/// at all instead of being silently discarded.
#[test]
fn header_specialization_clauses_reach_the_model() {
    let item = build_semantic_sexpr(
        "package Demo {\n\titem def Item;\n\titem objects : Item;\n\titem things : Item :> objects;\n}\n",
    );
    assert!(
        item.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::things\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::objects\")))"
        ),
        "expected item usage `:>` to resolve to objects, got:\n{item}"
    );

    let feature = build_semantic_sexpr(
        "package Demo {\n\tclassifier C {\n\t\tfeature base;\n\t\tfeature alias references base;\n\t}\n}\n",
    );
    assert!(
        feature.contains("(referenceSubsetting (reference \"base\"))"),
        "expected the KerML feature `references` clause to publish a reference, got:\n{feature}"
    );

    let viewpoint = build_semantic_sexpr(
        "package Demo {\n\tviewpoint base;\n\tviewpoint derived :> base;\n}\n",
    );
    assert!(
        viewpoint.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::base\")))"
        ),
        "expected viewpoint usage `:>` to resolve to base, got:\n{viewpoint}"
    );

    let subject = build_semantic_sexpr(
        "package Demo {\n\tpart def Vehicle;\n\trequirement def R {\n\t\tsubject vehicle : Vehicle;\n\t}\n\trequirement def S :> R {\n\t\tsubject subVehicle :>> vehicle;\n\t}\n}\n",
    );
    assert!(
        subject.contains("(redefinition (reference \"vehicle\"))"),
        "expected the subject `:>>` clause to publish a redefinition, got:\n{subject}"
    );
}

/// Every declaration kind whose parser node carries a `short_name` publishes it.
///
/// Nine lowerings dropped the `<short>` spelling even though their nodes had the field. The
/// corpus never exercises these seven keywords with a short name, so `spec42-snapshot` cannot
/// pin them and this table is the only coverage.
#[test]
fn every_short_name_carrying_declaration_publishes_it() {
    for (label, source, qualified_name, short_name) in [
        (
            "action usage",
            "package Demo {\n\taction <a> act;\n}\n",
            "Demo::act",
            "a",
        ),
        (
            "occurrence usage",
            "package Demo {\n\toccurrence <o> occ;\n}\n",
            "Demo::occ",
            "o",
        ),
        (
            "constraint usage",
            "package Demo {\n\tconstraint <c> con;\n}\n",
            "Demo::con",
            "c",
        ),
        (
            "ref declaration",
            "package Demo {\n\tref <r> refUsage;\n}\n",
            "Demo::refUsage",
            "r",
        ),
        (
            "return declaration",
            "package Demo {\n\tcalc def C {\n\t\treturn <r> res : Boolean;\n\t}\n}\n",
            "Demo::C::res",
            "r",
        ),
        (
            "view usage",
            "package Demo {\n\tview <v> viewUsage;\n}\n",
            "Demo::viewUsage",
            "v",
        ),
        (
            "subject declaration",
            "package Demo {\n\trequirement def R {\n\t\tsubject <s> subj;\n\t}\n}\n",
            "Demo::R::subj",
            "s",
        ),
        (
            "end declaration",
            "package Demo {\n\tconnection def C {\n\t\tend <e> source;\n\t\tend <t> target;\n\t}\n}\n",
            "Demo::C::source",
            "e",
        ),
        (
            "enumerated value",
            "package Demo {\n\tenum def E {\n\t\tenum <r> red;\n\t}\n}\n",
            "Demo::E::red",
            "r",
        ),
    ] {
        let output = build_semantic_sexpr(source);
        let expected = format!("(qualified-name \"{qualified_name}\")");
        let fact = format!("(short-name \"{short_name}\")");
        let line = output
            .lines()
            .find(|line| line.contains(&expected) && line.contains("(declaration "))
            .unwrap_or_else(|| panic!("no declaration for {label}, got:\n{output}"));
        assert!(
            line.contains(&fact),
            "expected {label} to publish {fact}, got:\n{line}"
        );
    }
}

#[test]
fn attribute_typed_by_an_enum_def_resolves_its_feature_typing_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tenum def StatusKind {\n\
         \t\tenum approved;\n\
         \t}\n\
         \tattribute def Holder {\n\
         \t\tattribute status : StatusKind;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind featureTyping) (ordinal 0))\n      (authored-target \"StatusKind\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::StatusKind\"))))"
        ),
        "expected the attribute's featureTyping reference to StatusKind to resolve, got:\n{output}"
    );
}

#[test]
fn enum_def_specializing_another_enum_def_resolves_its_subclassification_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tenum def Base {\n\
         \t\tenum on;\n\
         \t}\n\
         \tenum def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn requirement_def_specializing_another_requirement_def_resolves_its_subclassification_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trequirement def Base;\n\
         \trequirement def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn requirement_usage_typed_by_a_requirement_def_resolves_its_feature_typing_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trequirement def MassRequirement;\n\
         \tpart def Vehicle {\n\
         \t\trequirement massReq : MassRequirement;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind featureTyping) (ordinal 0))\n      (authored-target \"MassRequirement\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassRequirement\"))))"
        ),
        "expected the requirement usage's featureTyping reference to MassRequirement to resolve, got:\n{output}"
    );
}

#[test]
fn port_def_specializing_another_port_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tport def Base;\n\
         \tport def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn connection_def_specializing_another_connection_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconnection def Base;\n\
         \tconnection def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn connector_end_dotted_member_access_resolves_through_its_bases_type() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def T {\n\
         \t\tpart bead;\n\
         \t}\n\
         \tconnection def C;\n\
         \tpart t : T;\n\
         \tpart d2;\n\
         \tconnection bus : C connect t.bead to d2;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"t::bead\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::T::bead\")))))"
        ),
        "expected t.bead to resolve to T's owned `bead` member, got:\n{output}"
    );
}

#[test]
fn attribute_default_value_dotted_member_access_resolves_through_its_bases_type() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def F {\n\
         \t\tattribute a;\n\
         \t}\n\
         \tpart f : F;\n\
         \tattribute g = f.a;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"f::a\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::F::a\")))))"
        ),
        "expected f.a to resolve to F's owned `a` member, got:\n{output}"
    );
}

#[test]
fn attribute_default_value_dotted_member_access_chain_resolves_through_multiple_hops() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def C3 {\n\
         \t\tattribute z;\n\
         \t}\n\
         \tpart def B3 {\n\
         \t\tpart c : C3;\n\
         \t}\n\
         \tpart def A3 {\n\
         \t\tpart b : B3;\n\
         \t}\n\
         \tpart a : A3;\n\
         \tattribute g = a.b.c.z;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"a::b::c::z\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C3::z\")))))"
        ),
        "expected the a.b.c.z chain to resolve through three hops to C3's owned `z` member, got:\n{output}"
    );
}

#[test]
fn attribute_default_value_member_access_through_type_check_cast_resolves_through_the_operand() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def F {\n\
         \t\tattribute a;\n\
         \t}\n\
         \tpart f : F;\n\
         \tattribute g = (f as F).a;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"f::a\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::F::a\")))))"
        ),
        "expected the TypeCheck cast wrapping f to be transparent, resolving (f as F).a exactly \
         like the uncast f.a case, got:\n{output}"
    );
}

#[test]
fn attribute_default_value_member_access_through_parenthesized_base_resolves_through_the_operand() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def F {\n\
         \t\tattribute a;\n\
         \t}\n\
         \tpart f : F;\n\
         \tattribute g = (f).a;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"f::a\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::F::a\")))))"
        ),
        "expected the redundant parentheses around f to be transparent, resolving (f).a exactly \
         like the unparenthesized f.a case, got:\n{output}"
    );
}

#[test]
fn interface_def_specializing_another_interface_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tinterface def Base;\n\
         \tinterface def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn view_def_specializing_another_view_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tview def Base;\n\
         \tview def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn constraint_def_specializing_another_constraint_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def Base;\n\
         \tconstraint def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn constraint_usage_typed_by_a_constraint_def_resolves() {
    // planning/UPSTREAM_PARSER_GAPS.md #4 was resolved upstream in `0757de13`: `ConstraintUsage` now
    // carries `subsets`/`redefines` fields.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C;\n\
         \tconstraint c : C;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::c\"))) (kind constraint)"),
        "expected constraint c to lower to a declaration with kind constraint, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::c\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C\")))"
        ),
        "expected c's featureTyping of C to resolve, got:\n{output}"
    );
}

#[test]
fn constraint_usage_subsetting_another_constraint_usage_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint baseConstraint;\n\
         \tconstraint derivedConstraint :> baseConstraint;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::derivedConstraint\"))) (kind constraint)"),
        "expected a constraint usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derivedConstraint\")))"
        ),
        "expected derivedConstraint's subsetting of baseConstraint to resolve, got:\n{output}"
    );
}

#[test]
fn constraint_comparison_expression_resolves_both_operands() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute x : ScalarValues::Integer;\n\
         \tattribute y : ScalarValues::Integer;\n\
         \tconstraint def C { x > y }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"x\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::x\")))))"
        ),
        "expected x to resolve as an expressionOperand reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 1))\n      (authored-target \"y\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::y\")))))"
        ),
        "expected y to resolve as an expressionOperand reference, got:\n{output}"
    );
}

#[test]
fn assert_constraint_operand_resolves_to_sibling_declaration() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P {\n\
         \t\tattribute x : ScalarValues::Integer;\n\
         \t\tattribute y : ScalarValues::Integer;\n\
         \t\tassert constraint { x > y }\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"x\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::x\")))))"
        ),
        "expected x to resolve to the sibling attribute declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 1))\n      (authored-target \"y\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::y\")))))"
        ),
        "expected y to resolve to the sibling attribute declaration, got:\n{output}"
    );
}

#[test]
fn assert_constraint_typed_reference_form_resolves_its_type() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def MassConstraint;\n\
         \tpart def P {\n\
         \t\tassert constraint massConstraint : MassConstraint;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind featureTyping) (ordinal 0))\n      (authored-target \"MassConstraint\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassConstraint\")))))"
        ),
        "expected `assert constraint massConstraint : MassConstraint;` to resolve its type \
         reference through the shared FeatureTyping fixed point, got:\n{output}"
    );
}

#[test]
fn constraint_collection_op_arrow_invocation_resolves_base_and_argument_operands() {
    // `x->excludes(y)` (KerML `->` collection-operator invocation, e.g.
    // `derivedRequirements->excludes(originalRequirement)` in the Systems Library). The base
    // (`x`) and the argument (`y`) are both plain feature references and resolve exactly like
    // `Expression::Invocation`'s operands; the operator name (`excludes`) itself is a fixed
    // `CollectionOperator` enum value with no `QualifiedReferenceId` in the parser AST, so it
    // is never pushed as a reference.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P {\n\
         \t\tattribute x : ScalarValues::Integer;\n\
         \t\tattribute y : ScalarValues::Integer;\n\
         \t\tassert constraint { x->excludes(y) }\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"x\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::x\")))))"
        ),
        "expected `x` (the collection-op base) to resolve to the sibling attribute \
         declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 1))\n      (authored-target \"y\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::y\")))))"
        ),
        "expected `y` (the collection-op argument) to resolve to the sibling attribute \
         declaration, got:\n{output}"
    );
}

#[test]
fn calc_exponent_operator_negative_integer_exponent_promotes_to_real() {
    // A negative integer exponent (`2 ^ -1`) cannot stay `Integer` (fractional result), so it
    // promotes to `Real` via `powf`, exactly like a `Real`-involving pairing.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { 2 ^ -1 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind real) (real 0.5)))"
        ),
        "expected `2 ^ -1` to fold to Real(0.5) via the Real-promotion path, got:\n{output}"
    );
}

#[test]
fn constraint_logical_and_combines_two_comparisons_to_boolean() {
    // `and`/`or` combining multiple comparisons in a general constraint body (not just a
    // `filter <expr>;` condition, which already supported `and`/`or` for reference resolution
    // per `25c8bf52`) is the same "widen the recursive classifier" pattern applied to
    // evaluation: `EvalNode::Logical` folds two already-folded Boolean comparison operands.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass1 = 2;\n\
         \tattribute mass2 = 3;\n\
         \tattribute massLimit = 10;\n\
         \tattribute isActive = true;\n\
         \tconstraint def C { (mass1 + mass2) < massLimit and isActive }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `(mass1 + mass2) < massLimit and isActive` to fold to Boolean(true) \
         (2 + 3 = 5 < 10, and isActive is true), got:\n{output}"
    );
}

#[test]
fn calc_unary_minus_resolves_feature_operand() {
    // `-x` with a resolvable feature operand: the operand reference resolves and, since it
    // has a known constant value, the whole expression folds through `fold_unary`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass = 5;\n\
         \tcalc def Calc { -mass }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer -5)))"
        ),
        "expected `-mass` (mass = 5) to resolve the operand reference and fold to \
         Integer(-5), got:\n{output}"
    );
}

#[test]
fn constraint_logical_xor_combines_two_comparisons_to_boolean() {
    // `xor` shares `and`/`or`'s exact Boolean/Boolean truth-table shape (`is_logical_operator`
    // widened, `fold_logical`'s new `Xor` arm): true xor false = true.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass1 = 2;\n\
         \tattribute massLimit = 10;\n\
         \tattribute isActive = false;\n\
         \tconstraint def C { mass1 < massLimit xor isActive }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `mass1 < massLimit xor isActive` (true xor false) to fold to \
         Boolean(true), got:\n{output}"
    );
}

#[test]
fn constraint_logical_implies_combines_two_comparisons_to_boolean() {
    // `implies`: false implies anything is true (`!left || right`).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass1 = 20;\n\
         \tattribute massLimit = 10;\n\
         \tattribute isActive = false;\n\
         \tconstraint def C { mass1 < massLimit implies isActive }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `mass1 < massLimit implies isActive` (false implies false) to fold to \
         Boolean(true), got:\n{output}"
    );
}

#[test]
fn constraint_simple_comparison_only_regression_unaffected() {
    // Regression guard: a plain comparison-only constraint body (slices 1-3, no arithmetic or
    // logical widening involved) must fold exactly as before.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C { 1 < 2 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected plain comparison-only `1 < 2` to still fold to Boolean(true), got:\n{output}"
    );
}

#[test]
fn calc_arithmetic_only_regression_unaffected() {
    // Regression guard: calc-body arithmetic (slice 4) must stay comparison-free and fold
    // exactly as before -- unaffected by the constraint-side widening.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { 2 + 3 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer 5)))"
        ),
        "expected plain arithmetic-only `2 + 3` calc body to still fold to Integer(5), \
         got:\n{output}"
    );
}

#[test]
fn redefinition_value_with_a_qualified_reference_is_pushed_and_classified() {
    // The exact `enum_status_redefinition.md` shape (`attribute :>> status =
    // RequirementStatusKind::approved;`): the `= RequirementStatusKind::approved` value
    // portion publishes an `ExpressionOperand` reference (the shared lookup every
    // constraint/calc operand reference already uses) sourced at the redefining attribute's
    // own anonymous declaration, and -- since the multi-segment qualified-path lookup bug
    // fixed alongside this test -- now resolves to the enum literal, exactly as the same
    // qualified name would resolve if used as e.g. a `FeatureTyping` target.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tenum def RequirementStatusKind {\n\
         \t\tenum approved;\n\
         \t}\n\
         \trequirement def Base {\n\
         \t\tattribute status : RequirementStatusKind;\n\
         \t}\n\
         \trequirement def Derived :> Base {\n\
         \t\tattribute :>> status = RequirementStatusKind::approved;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(authored-target \"RequirementStatusKind::approved\")\n      (outcome (status \
             resolved) (target (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::RequirementStatusKind::approved\")))))"
        ),
        "expected the redefinition value `RequirementStatusKind::approved` to resolve its \
         ExpressionOperand reference to the enum literal, got:\n{output}"
    );
}

#[test]
fn multi_segment_qualified_expression_operand_resolves_through_nested_namespaces() {
    // Regression for the qualified-path `ExpressionOperand` lookup bug: `resolve_reference`'s
    // multi-segment segment loop was reading `exported_names` (the cross-file import
    // propagation index, which treats a member owned by a non-Package/LibraryPackage
    // namespace as private by KerML's default-visibility rule) instead of `direct_names` (the
    // unfiltered index every other same-scope qualified traversal -- e.g. usage-typing
    // redefinition targets -- reads from). A three-segment qualified name reaching through two
    // nested non-Package namespaces (`Outer::Inner::member`, `Inner` owned by `Outer`, not by
    // a package) now resolves, matching how `Outer::Inner` alone already resolved as e.g. a
    // `FeatureTyping` target.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Outer {\n\
         \t\tpart def Inner {\n\
         \t\t\tattribute member = 5;\n\
         \t\t}\n\
         \t}\n\
         \tattribute x = Outer::Inner::member;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(authored-target \"Outer::Inner::member\")\n      (outcome (status resolved) \
             (target (node (document \"memory://test/enum.sysml\") (qualified-name \
             \"Demo::Outer::Inner::member\")))))"
        ),
        "expected the three-segment qualified name `Outer::Inner::member` to resolve its \
         ExpressionOperand reference to the nested attribute, got:\n{output}"
    );
}

#[test]
fn metadata_annotation_body_override_value_resolves() {
    // The metadata annotation body override deferred by `2680ca20` pending exactly this
    // value-assignment machinery: `isMandatory = true;` inside `@Safety{...}` now lowers
    // through the same shared pipeline as an attribute default value. Upstream types a
    // `MetadataBody` member as a `MetadataBodyUsage` -- a reference redefinition of a feature
    // of the annotated type, not a declaration named `isMandatory` -- so the override owns an
    // anonymous attribute whose `redefinition` names the overridden feature.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tmetadata def Safety {\n\
         \t\tattribute isMandatory : Boolean;\n\
         \t}\n\
         \tpart def Vehicle {\n\
         \t\tpart seatBelt[2] {@Safety{isMandatory = true;}}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (path (named (kind package) (name \"Demo\")) (named (kind part-def) (name \"Vehicle\")) (named (kind part) (name \"seatBelt\")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind \
             boolean) (boolean true)))"
        ),
        "expected `isMandatory = true;` inside `@Safety{{...}}` to publish its own \
         Boolean(true) evaluation fact, got:\n{output}"
    );
}

#[test]
fn parameter_default_value_with_member_access_resolves() {
    // The `out v_out : SpeedValue = vel.v;` shape deferred by `494b0ba6`: the parameter
    // default value now resolves its `vel.v` member-access operand through the exact same
    // pipeline `ReturnDecl::value` already used (bd50fccd precedent).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \taction def Calc {\n\
         \t\tattribute vel;\n\
         \t\tout v_out = vel.v;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(memberAccessOperand (reference \"vel::v\"))"),
        "expected `out v_out = vel.v;`'s parameter default value to resolve `vel.v` as a \
         memberAccessOperand reference, got:\n{output}"
    );
}

#[test]
fn concern_usage_typed_by_a_concern_def_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconcern def C;\n\
         \tconcern c : C;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::c\"))) (kind concern)"),
        "expected concern c to lower to a declaration with kind concern, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::c\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C\")))"
        ),
        "expected c's featureTyping of C to resolve, got:\n{output}"
    );
}

#[test]
fn concern_usage_subsetting_another_concern_usage_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconcern baseConcern;\n\
         \tconcern derivedConcern :> baseConcern;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::derivedConcern\"))) (kind concern)"),
        "expected a concern usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derivedConcern\")))"
        ),
        "expected derivedConcern's subsetting of baseConcern to resolve, got:\n{output}"
    );
}

#[test]
fn calc_def_specializing_another_calc_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P {\n\
         \t\tcalc def Base;\n\
         \t\tcalc def Derived :> Base;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn calc_usage_typed_by_a_calc_def_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P {\n\
         \t\tcalc def Calc;\n\
         \t\tcalc c : Calc;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::P::c\"))) (kind calc)"),
        "expected calc c to lower to a declaration with kind calc, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::c\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::Calc\")))"
        ),
        "expected c's featureTyping of Calc to resolve, got:\n{output}"
    );
}

#[test]
fn calc_usage_redefining_another_calc_usage_resolves() {
    // `CalcUsage::redefines` is a bare `Vec<QualifiedReferenceId>`, not a
    // `Node<SubsettingRelationship>` -- lowered as direct `Redefinition` references rather
    // than through `lower_subsetting_relationship`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P {\n\
         \t\tcalc def Calc;\n\
         \t\tcalc calcA : Calc;\n\
         \t\tcalc calcB : Calc :>> calcA;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::P::calcB\"))) (kind calc)"),
        "expected a calc usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind redefinition) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::calcB\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::calcA\")))"
        ),
        "expected calcB's redefinition of calcA to resolve, got:\n{output}"
    );
}

#[test]
fn view_usage_typed_by_a_view_def_resolves() {
    // planning/UPSTREAM_PARSER_GAPS.md #8 was resolved upstream in `0757de13`: `ViewUsage` now carries
    // a `subsets` field, so `view` usage lowering is no longer deferred.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tview def V;\n\
         \tview v : V;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::V\"))) (kind view-def)"),
        "expected view def V to still lower to a declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::v\"))) (kind view)"),
        "expected view v to lower to a declaration with kind view, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::v\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::V\")))"
        ),
        "expected v's featureTyping of V to resolve, got:\n{output}"
    );
}

#[test]
fn view_usage_subsetting_another_view_usage_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tview baseView;\n\
         \tview derivedView :> baseView;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::derivedView\"))) (kind view)"),
        "expected a view usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derivedView\")))"
        ),
        "expected derivedView's subsetting of baseView to resolve, got:\n{output}"
    );
}

#[test]
fn rendering_usage_typed_and_subsetting_resolve() {
    // planning/UPSTREAM_PARSER_GAPS.md #26 was resolved upstream in `cb026cd`: `RenderingUsage` now
    // carries `subsets`/`redefines` fields (full parity with `ViewUsage`), so package-level
    // `rendering` usage lowering (previously unconditionally `unsupported_package_member`) is
    // no longer deferred.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trendering def R;\n\
         \trendering renderings : R;\n\
         \trendering asTree : R :> renderings;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::asTree\"))) (kind rendering)"),
        "expected a rendering usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::asTree\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::R\")))"
        ),
        "expected asTree's featureTyping of R to resolve, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::asTree\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::renderings\")))"
        ),
        "expected asTree's subsetting of renderings to resolve, got:\n{output}"
    );
}

#[test]
fn use_case_usage_and_verification_case_usage_at_package_scope_resolve() {
    // `UseCaseUsage`/`VerificationCaseUsage` were previously unconditionally
    // `unsupported_package_member` at package scope even for the plain `use case <name> :
    // <Type> { ... }` header shape, which needs no multiplicity field (still missing
    // upstream, planning/UPSTREAM_PARSER_GAPS.md Gap 53).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tuse case def UC;\n\
         \tuse case uc : UC;\n\
         \tverification def V;\n\
         \tverification v : V;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::uc\"))) (kind use-case)"),
        "expected a use case usage declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::v\"))) (kind verification)"),
        "expected a verification case usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::uc\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::UC\")))"
        ),
        "expected uc's featureTyping of UC to resolve, got:\n{output}"
    );
}

#[test]
fn viewpoint_usage_at_package_scope_resolves() {
    // `ViewpointUsage` was previously unconditionally `unsupported_package_member`. Only the
    // plain `viewpoint <name>[: <Type>]` header shape lowers: its `subsets`/`redefines`
    // clauses now parse but are not lowered yet (see `lower_viewpoint_usage`).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tviewpoint def VP;\n\
         \tviewpoint vp : VP;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::vp\"))) (kind viewpoint)"),
        "expected a viewpoint usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::vp\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::VP\")))"
        ),
        "expected vp's featureTyping of VP to resolve, got:\n{output}"
    );
}

#[test]
fn interface_usage_declaration_typed_by_an_interface_def_resolves() {
    // planning/UPSTREAM_PARSER_GAPS.md #6 was resolved upstream in `0757de13`: all three
    // `InterfaceUsage` variants now carry `subsets`/`redefines` fields. Nested in a `part def`
    // body: `part/body.rs` tries `interface_usage` before `interface_def_required`, so a bare
    // `interface i : I;` (no `connect`) unambiguously parses as `InterfaceUsage::Declaration`
    // there, unlike at package level where `interface_def` (optional `def`) is tried first.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tinterface def I;\n\
         \tpart def P {\n\
         \t\tinterface i : I;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::I\"))) (kind interface-def)"),
        "expected interface def I to lower to a declaration, got:\n{output}"
    );
    assert!(
        output.contains("(qualified-name \"Demo::P::i\"))) (kind interface)"),
        "expected interface i to lower to a declaration with kind interface, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::i\")))"
        ) && output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::I\")))"
        ),
        "expected i's featureTyping of I to resolve, got:\n{output}"
    );
}

#[test]
fn interface_usage_subsetting_another_interface_usage_resolves() {
    // `interface_usage`'s `named_interface` capture requires a `:` typed form to consume the
    // name at all (a bare `name :> target` with no `: Type` never captures `name` -- see
    // `part::usage::interface_usage`'s doc comments), so both usages carry an explicit `: I`
    // typing target alongside the `:>` subsetting clause.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tinterface def I;\n\
         \tpart def P {\n\
         \t\tinterface baseInterface : I;\n\
         \t\tinterface derivedInterface : I :> baseInterface;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::P::derivedInterface\"))) (kind interface)"),
        "expected an interface usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::derivedInterface\")))"
        ),
        "expected derivedInterface's subsetting of baseInterface to resolve, got:\n{output}"
    );
}

#[test]
fn occurrence_def_specializing_another_occurrence_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \toccurrence def Base;\n\
         \toccurrence def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn occurrence_usage_typed_by_an_occurrence_def_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \toccurrence def Occ;\n\
         \toccurrence o : Occ;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::o\"))) (kind occurrence)"),
        "expected an occurrence usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::o\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Occ\")))"
        ),
        "expected o's typing reference to Occ to resolve, got:\n{output}"
    );
}

#[test]
fn analysis_case_def_specializing_another_analysis_case_def_resolves_its_specialization_reference()
{
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tanalysis def Base;\n\
         \tanalysis def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn case_def_specializing_another_case_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcase def Base;\n\
         \tcase def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn verification_case_def_specializing_another_verification_case_def_resolves_its_specialization_reference(
) {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tverification def Base;\n\
         \tverification def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn use_case_def_specializing_another_use_case_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tuse case def Base;\n\
         \tuse case def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn conjugated_port_usage_typing_reference_resolves_and_carries_the_conjugated_flag() {
    // `port p : ~Base;` nested inside a `part def` body dispatches through the real
    // `PortUsage` grammar production (package-level bare `port name : Type;` instead folds
    // into `PortDef`, see `lower_port_def`'s doc comment) -- the `~` conjugation polarity
    // must survive as an explicit fact distinct from the (unconjugated) target declaration.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tport def Base;\n\
         \tpart def Holder {\n\
         \t\tport p : ~Base;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Holder::p\"))) (kind port)"),
        "expected a port usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (conjugated true) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::p\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected p's conjugated typing reference to Base to resolve with the conjugated flag, got:\n{output}"
    );
}

#[test]
fn non_conjugated_port_usage_typing_reference_does_not_carry_the_conjugated_flag() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tport def Base;\n\
         \tpart def Holder {\n\
         \t\tport p : Base;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::p\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected p's unconjugated typing reference to Base to resolve without a conjugated flag, got:\n{output}"
    );
    assert!(
        !output.contains("(kind typing) (conjugated true)"),
        "did not expect the conjugated flag on an unconjugated port typing reference, got:\n{output}"
    );
}

#[test]
fn item_def_specializing_another_item_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \titem def Base;\n\
         \titem def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn item_usage_typed_by_an_item_def_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \titem def Base;\n\
         \tpart def Holder {\n\
         \t\titem w : Base;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Holder::w\"))) (kind item)"),
        "expected an item usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::w\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected w's typing reference to Base to resolve, got:\n{output}"
    );
}

#[test]
fn calc_def_parameter_subsets_clause_resolves_as_a_subsetting_relationship() {
    // `in value :> seq;` on a *named* `InOutDecl` is an authored subsetting clause, carried on
    // `ast::InOutDecl::subsets`. The parser previously folded the `:>` spelling into
    // `type_name`, which reported a subsetting as a typing; the two clauses are now separate
    // fields, so this lowers through `lower_subsetting_relationship` like every other
    // `:>` clause and no typing reference is invented for it.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def ExcludingOnce {\n\
         \t\tin seq;\n\
         \t\tin value :> seq;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::ExcludingOnce::value\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::ExcludingOnce::seq\")))"
        ),
        "expected value's `:>` clause to resolve as a subsetting relationship to seq, got:\n{output}"
    );
    assert!(
        !output.contains("(kind typing)"),
        "expected no FeatureTyping reference for the subsets clause, got:\n{output}"
    );
}

#[test]
fn stakeholder_concern_reference_resolves_through_the_any_domain_lookup() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconcern modularity;\n\
         \tviewpoint def SystemView {\n\
         \t\tstakeholder modularity;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind stakeholderTarget)"),
        "expected a stakeholderTarget reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::modularity\")))"
        ),
        "expected the stakeholder concern reference to resolve to modularity, got:\n{output}"
    );
}

#[test]
fn stakeholder_redefinition_resolves_through_the_redefinition_reference_kind() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconcern modularity;\n\
         \tviewpoint def SystemView {\n\
         \t\tstakeholder :>> modularity;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind redefinition) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Demo\")) (named (kind viewpoint-def) (name \"SystemView\")) (anonymous (kind stakeholder) (ordinal 0))))"
        ),
        "expected a redefinition reference sourced at the anonymous stakeholder declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::modularity\")))"
        ),
        "expected the stakeholder redefinition to resolve to modularity, got:\n{output}"
    );
}

#[test]
fn frame_member_recurses_into_its_nested_body_content() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Driver;\n\
         \trequirement def SafetyRequirement {\n\
         \t\tframe concernFraming {\n\
         \t\t\tstakeholder driver : Driver;\n\
         \t\t}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(qualified-name \"Demo::SafetyRequirement::concernFraming\"))) (kind frame)"
        ),
        "expected a frame declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(qualified-name \"Demo::SafetyRequirement::concernFraming::driver\"))) (kind stakeholder)"
        ),
        "expected the nested stakeholder to lower under the frame, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::SafetyRequirement::concernFraming::driver\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Driver\")))"
        ),
        "expected the nested stakeholder's typing reference to resolve, got:\n{output}"
    );
}

#[test]
fn subject_ref_shorthand_is_recognized_and_ignored() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tviewpoint def SystemView {\n\
         \t\tsubject;\n\
         \t}\n\
         }\n",
    );
    assert!(
        !output.contains("unsupported_requirement_definition_member"),
        "expected the bare `subject;` shorthand not to be reported as unsupported, got:\n{output}"
    );
}

#[test]
fn class_def_specializing_another_class_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tclass Base;\n\
         \tclass Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn kerml_classifier_decl_specializing_another_classifier_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tstruct Base;\n\
         \tstruct Derived specializes Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn kerml_feature_member_nested_inside_classifier_decl_resolves_its_typing() {
    // `classifier { ... }` bodies share the `CalcDefBody` grammar (b7d6ac36), so a bare
    // `feature` member inside a `classifier` body dispatches through the same
    // `CalcDefBodyElement::KermlFeature` -> `lower_kerml_feature_member` path already used
    // for package-level and calc-def-nested feature members; it should resolve identically.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tclassifier Wheel {}\n\
         \tclassifier Bicycle {\n\
         \t\tfeature rollsOn : Wheel [2];\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Bicycle::rollsOn\"))) (kind kerml-feature)"),
        "expected a nested kerml-feature declaration for rollsOn, got:\n{output}"
    );
    assert!(
        output.contains("(relationships (featureTyping (reference \"Wheel\")))"),
        "expected rollsOn's FeatureTyping reference, got:\n{output}"
    );
    assert!(
        output.contains(
            "(outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Wheel\"))))"
        ),
        "expected rollsOn's featureTyping reference to Wheel to resolve, got:\n{output}"
    );
}

#[test]
fn bare_connect_at_package_scope_resolves_ends() {
    // `PackageBodyElement::Connect` (the keyword-less `Connect` struct, distinct from
    // `ConnectStmt`) dispatches into the new `lower_bare_connect`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart a;\n\
         \tpart b;\n\
         \tconnect a to b;\n\
         }\n",
    );
    assert!(
        output.contains("(kind connectorEnd)")
            && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::a\")))"
            )
            && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::b\")))"
            ),
        "expected both connector ends to resolve, got:\n{output}"
    );
}

#[test]
fn metadata_def_specializing_another_metadata_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tmetadata def Base;\n\
         \tmetadata def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn metadata_usage_typed_by_a_metadata_def_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tmetadata def Base;\n\
         \tpart def Holder {\n\
         \t\tmetadata m : Base;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::Holder::m\"))) (kind metadata)"),
        "expected a metadata usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::m\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected m's typing reference to Base to resolve, got:\n{output}"
    );
}

#[test]
fn metadata_annotation_on_part_usage_resolves_the_annotation_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tmetadata def Safety {\n\
         \t\tattribute isMandatory : Boolean;\n\
         \t}\n\
         \tpart def Vehicle {\n\
         \t\tpart seatBelt[2] {@Safety{isMandatory = true;}}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind metadataAnnotation) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Vehicle::seatBelt\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Safety\")))"
        ),
        "expected seatBelt's @Safety metadata annotation reference to resolve, got:\n{output}"
    );
}

#[test]
fn filter_metadata_test_resolves_the_metadata_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tmetadata def Safety;\n\
         \tpackage 'Safety Features' {\n\
         \t\tpublic import Demo::**;\n\
         \t\tfilter @Safety;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind filterMetadataTest) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Safety Features\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Safety\")))"
        ),
        "expected the filter's @Safety metadata-test reference to resolve, got:\n{output}"
    );
}

#[test]
fn filter_with_not_unary_operator_resolves_its_operand() {
    // `lower_filter_expression` previously had no `Expression::UnaryOp` arm at all (unlike
    // `lower_calc_expression`/`lower_constraint_expression`, which both already recurse
    // through `not`), so `not <operand>` inside a `filter` statement always fell to the
    // blanket `unsupported_package_member` diagnostic even though the operand itself is an
    // ordinary resolvable reference (`kerml/filtering.md`'s `filter (... and not
    // Type::isAbstract) or ...`).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tmetadata def Safety {\n\
         \t\tattribute isMandatory : Boolean;\n\
         \t}\n\
         \tpackage 'Not Mandatory' {\n\
         \t\tpublic import Demo::**;\n\
         \t\tfilter not Safety::isMandatory;\n\
         \t}\n\
         }\n",
    );
    assert!(
        !output.contains("unsupported_package_member"),
        "expected `not Safety::isMandatory` to no longer trip the blanket unsupported \
         diagnostic, got:\n{output}"
    );
    assert!(
        output.contains("(kind expressionOperand)")
            && output.contains("(authored-target \"Safety::isMandatory\")"),
        "expected the filter's `not`-wrapped operand reference to still be lowered and \
         attempted, got:\n{output}"
    );
}

#[test]
fn action_def_specializing_another_action_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \taction def Base;\n\
         \taction def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn action_usage_typed_by_an_action_def_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \taction def Base;\n\
         \taction a : Base;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::a\"))) (kind action)"),
        "expected an action usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::a\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected a's typing reference to Base to resolve, got:\n{output}"
    );
}

#[test]
fn state_def_specializing_another_state_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tstate def Base;\n\
         \tstate def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn state_usage_typed_by_a_state_def_resolves() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tstate def Base;\n\
         \tstate s : Base;\n\
         }\n",
    );
    assert!(
        output.contains("(qualified-name \"Demo::s\"))) (kind state)"),
        "expected a state usage declaration, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::s\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected s's typing reference to Base to resolve, got:\n{output}"
    );
}

#[test]
fn bind_statement_with_dotted_feature_chain_operands_resolves_both_ends() {
    // Regression test: `lower_satisfy_operand` (shared by `lower_bind`, `lower_satisfy`,
    // `lower_allocate`, etc.) only matched `Expression::MemberAccess` in its dotted-chain arm,
    // not `Expression::FeatureChainRef` -- the shape the parser actually produces for a
    // dotted path like `f.a`/`a.g` (see `flatten_member_access_chain`, which has always
    // handled both). `lower_connector_end` (used by `connect`) already matched both variants,
    // so `connect f.a to a.g;` resolved while the very next line, `bind f.a = a.g;`, fell
    // through to an unsupported diagnostic on both operands -- exactly the shape from
    // `tests/snapshots/sysml/examples/feature_path_test.md`. Fixed by adding
    // `Expression::FeatureChainRef(_)` to `lower_satisfy_operand`'s dotted-chain match arm,
    // mirroring `lower_connector_end`.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def F { part a : A; }\n\
         \tpart def A { part g : F; }\n\
         \tpart def B {\n\
         \t\tpart f : F;\n\
         \t\tpart a : A;\n\
         \t}\n\
         \tpart b : B {\n\
         \t\tbind f.a = a.g;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind bind)"),
        "expected an owned bind declaration, got:\n{output}"
    );
    assert!(
        output.matches("(kind memberAccessOperand)").count() >= 2,
        "expected both dotted bind operands to lower as memberAccessOperand references, \
         got:\n{output}"
    );
    assert!(
        !output.contains("(status unresolved)") && !output.contains("unsupported"),
        "expected both dotted bind operands (f.a, a.g) to resolve, got:\n{output}"
    );
}

#[test]
fn connect_statement_with_dotted_feature_chain_operands_resolves_both_ends() {
    // Companion regression to the `bind` test above: `connect a.b to c.d;` with dotted
    // endpoints already resolved correctly (via `lower_connector_end`, which has always
    // matched both `Expression::MemberAccess` and `Expression::FeatureChainRef`) -- this
    // pins that behavior down explicitly so a future refactor of the shared
    // `flatten_member_access_chain`/`push_member_access_reference` path can't silently
    // regress the `connect` side while fixing/touching the `bind` side.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def A { part d : A; }\n\
         \tpart def B {\n\
         \t\tpart a : A;\n\
         \t\tpart c : A;\n\
         \t}\n\
         \tpart b : B {\n\
         \t\tconnect a.d to c.d;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.matches("(kind memberAccessOperand)").count() >= 2,
        "expected both dotted connect endpoints to lower as memberAccessOperand \
         references, got:\n{output}"
    );
    assert!(
        !output.contains("(status unresolved)") && !output.contains("unsupported"),
        "expected both dotted connect endpoints (a.d, c.d) to resolve, got:\n{output}"
    );
}

#[test]
fn variation_part_resolves_both_variant_members_to_sibling_declarations() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Transmission;\n\
         \tpart manualTransmission;\n\
         \tpart automaticTransmission;\n\
         \tpart vehicle {\n\
         \t\tvariation part transmission : Transmission {\n\
         \t\t\tvariant manualTransmission;\n\
         \t\t\tvariant automaticTransmission;\n\
         \t\t}\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.matches("(kind variant)").count() >= 2,
        "expected two variant relationship kinds, got:\n{output}"
    );
    assert!(
        output.contains(
            "(kind variant) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::vehicle::transmission\""
        ),
        "expected both variant references to be sourced at the variation declaration \
         itself (no anonymous nested-declaration shift), got:\n{output}"
    );
    assert!(
        output.contains("(authored-target \"manualTransmission\")")
            && output.contains("(authored-target \"automaticTransmission\")"),
        "expected both variant targets to be authored, got:\n{output}"
    );
    assert!(
        !output.contains("(status unresolved)"),
        "expected both variant members to resolve to their sibling declarations, \
         got:\n{output}"
    );
    assert!(
        output.contains("(variation true)"),
        "expected the variation part's own typing reference to carry the variation flag, \
         got:\n{output}"
    );
}

#[test]
fn ref_decl_resolves_its_typing_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Part;\n\
         \tpart def Holder {\n\
         \t\tref self: Part;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind ref)"),
        "expected a `ref` declaration, got:\n{output}"
    );
    assert!(
        output.contains("(kind featureTyping)"),
        "expected a featureTyping relationship kind for the ref's `:` clause, got:\n{output}"
    );
    assert!(
        output.contains("(authored-target \"Part\")"),
        "expected the ref's typing target to be authored, got:\n{output}"
    );
    assert!(
        !output.contains("(status unresolved)"),
        "expected the ref's typing target to resolve, got:\n{output}"
    );
}

#[test]
fn ref_decl_resolves_its_redefines_reference() {
    // `part def`/`part` usage bodies parse `ref` through the narrower `part_ref_usage`
    // production (`ast::part::usage::part_ref_usage`), which does not capture a trailing
    // `:>>` redefines target at all. `connection def`/`interface def` bodies instead parse
    // `ref` through `connector::ref_decl`, which captures the full `:`/`:>>`/`:>` clause set
    // -- use a `connection def` body here so the redefines clause actually round-trips.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def Item {\n\
         \t\tref self: Item;\n\
         \t}\n\
         \tconnection def C {\n\
         \t\tref self: Item :>> Item::self;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind redefinition)"),
        "expected a redefinition relationship kind for the ref's `:>>` clause, got:\n{output}"
    );
    assert!(
        output.contains("(authored-target \"Item::self\")"),
        "expected the ref's redefines target to be authored, got:\n{output}"
    );
}

#[test]
fn viewpoint_def_specializing_another_viewpoint_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tviewpoint def Base;\n\
         \tviewpoint def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn rendering_def_specializing_another_rendering_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \trendering def Base;\n\
         \trendering def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn allocation_def_specializing_another_allocation_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tallocation def Base;\n\
         \tallocation def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn flow_def_specializing_another_flow_def_resolves_its_specialization_reference() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tflow def Base;\n\
         \tflow def Derived :> Base;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
        ),
        "expected Derived's specialization of Base to resolve, got:\n{output}"
    );
}

#[test]
fn value_assignment_tuple_resolves_every_element_reference() {
    // `Expression::Tuple` (`(a, b, c)`) reuses the Invocation-shaped reference-resolution
    // slice: no callee, but every element recurses back into `lower_constraint_expression`
    // exactly like an invocation argument, so all three feature references resolve.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute a : ScalarValues::Integer;\n\
         \tattribute b : ScalarValues::Integer;\n\
         \tattribute c : ScalarValues::Integer;\n\
         \tattribute tuple = (a, b, c);\n\
         }\n",
    );
    for (name, ordinal) in [("a", 0), ("b", 1), ("c", 2)] {
        assert!(
            output.contains(&format!(
                "(kind expressionOperand) (ordinal {ordinal}))\n      (authored-target \
                 \"{name}\")\n      (outcome (status resolved) (target (node (document \
                 \"memory://test/enum.sysml\") (qualified-name \"Demo::{name}\")))))"
            )),
            "expected tuple element `{name}` to resolve as an expressionOperand reference, \
             got:\n{output}"
        );
    }
}

#[test]
fn default_reference_usage_typed_binding_resolves_its_typing() {
    // A typed keyword-less binding, `<name> : <Type> = <expr>;`, still routes through
    // `DefaultReferenceUsage` (no leading keyword) and should resolve its `FeatureTyping`
    // reference in addition to the declaration and value.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute def MassValue;\n\
         \tpart def Vehicle {\n\
         \t\tmass : MassValue = 10;\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(kind default-reference)")
            && output.contains("(qualified-name \"Demo::Vehicle::mass\")"),
        "expected a DefaultReferenceUsage declaration for mass, got:\n{output}"
    );
    assert!(
        output.contains("(kind featureTyping)")
            && output.contains(
                "(outcome (status resolved) (target (node (document \
                 \"memory://test/enum.sysml\") (qualified-name \"Demo::MassValue\")))))"
            ),
        "expected `mass`'s typing to resolve to Demo::MassValue, got:\n{output}"
    );
}

/// The standard-view rule needs a library-admitted definition, and the source role that makes
/// a document a library is not expressible in the snapshot corpus, so it is pinned here.
#[test]
fn a_view_typed_by_a_non_standard_library_definition_is_reported() {
    let publish = |library: &str| {
        let request = BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://views.sysml",
                    format!("library package Views {{ view def {library}; }}"),
                    SourceKind::Library,
                ),
                SourceInput::new(
                    "memory://test.sysml",
                    format!("package P {{ import Views::*; view v : {library}; }}"),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        build(request)
            .unwrap()
            .document_diagnostics("memory://test.sysml")
            .iter()
            .map(|diagnostic| diagnostic.code().as_str().to_string())
            .collect::<Vec<_>>()
    };
    assert!(
        publish("RequirementView").contains(&"view_type_non_standard".to_string()),
        "{:?}",
        publish("RequirementView")
    );
    assert!(
        !publish("GeneralView").contains(&"view_type_non_standard".to_string()),
        "a standard view definition is not reported: {:?}",
        publish("GeneralView")
    );
}

/// A workspace's own `view def` is the author's to define, whatever it is called.
#[test]
fn a_view_typed_by_a_workspace_definition_is_never_reported_as_non_standard() {
    let published =
        published_for("package P { view def RequirementView; view v : RequirementView; }");
    assert!(!published
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.code() == &DiagnosticCode::ViewTypeNonStandard));
}

/// The library-context hint is a fact about the publication, not about a host's configuration.
///
/// Both admission paths answer the same way, which is what makes the hint safe for a host that
/// reuses a solved library stratum: a workspace that admitted a library never reports it,
/// whether the library came in as a source or as a stratum.
#[test]
fn the_library_context_hint_reads_what_the_publication_admitted() {
    let workspace = "package P { import Lib::*; part def D :> Missing; }";
    let codes = |published: PublishedResolution| {
        published
            .document_diagnostics("memory://workspace.sysml")
            .iter()
            .map(|diagnostic| diagnostic.code().as_str().to_string())
            .collect::<Vec<_>>()
    };

    let alone = build(
        BuildRequest::new(
            vec![SourceInput::new(
                "memory://workspace.sysml",
                workspace.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    assert!(
        codes(alone).contains(&"missing_library_context".to_string()),
        "a workspace with unresolved imports and no library admitted reports the hint"
    );

    let with_stratum = build(
        BuildRequest::with_library(
            vec![SourceInput::new(
                "memory://workspace.sysml",
                workspace.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
            library_stratum(),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(
        !codes(with_stratum).contains(&"missing_library_context".to_string()),
        "a publication that admitted a library stratum has library context"
    );

    let with_source = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://lib.sysml",
                    LIBRARY_SOURCE.to_string(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://workspace.sysml",
                    workspace.to_string(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    assert!(
        !codes(with_source).contains(&"missing_library_context".to_string()),
        "admitting the library as a source is the same fact as admitting it as a stratum"
    );
}

/// A library document is reported only when the host names it as an authoring surface.
///
/// Provenance and authoring surface are different questions. The default answers the first --
/// a workspace does not inherit its library's diagnostics -- and naming the document answers
/// the second, which is the only thing an editor with that file open needs.
#[test]
fn an_admitted_library_document_is_reported_only_when_the_host_names_it() {
    let sources = || {
        vec![
            SourceInput::new(
                "memory://lib.sysml",
                "library package Lib { part def A; part def A; }".to_string(),
                SourceKind::Library,
            ),
            SourceInput::new(
                "memory://workspace.sysml",
                "package W { part w; }".to_string(),
                SourceKind::Workspace,
            ),
        ]
    };
    let codes = |published: &PublishedResolution, document: &str| {
        published
            .document_diagnostics(document)
            .iter()
            .map(|diagnostic| diagnostic.code().as_str().to_string())
            .collect::<Vec<_>>()
    };

    let default = build(
        BuildRequest::new(sources(), ConstructionSchedule::Sequential, "contract-v1").unwrap(),
    )
    .unwrap();
    assert!(
        codes(&default, "memory://lib.sysml").is_empty(),
        "a library is admitted for resolution, not reported: {:?}",
        codes(&default, "memory://lib.sysml")
    );
    assert!(
        !codes(&default, "memory://workspace.sysml").is_empty(),
        "the workspace is always reported"
    );

    let named = build(
        BuildRequest::new(sources(), ConstructionSchedule::Sequential, "contract-v1")
            .unwrap()
            .reporting([Box::from("memory://lib.sysml")]),
    )
    .unwrap();
    assert!(
        codes(&named, "memory://lib.sysml").contains(&"duplicate_namespace_member".to_string()),
        "a named library document reports its own diagnostics: {:?}",
        codes(&named, "memory://lib.sysml")
    );
    assert_eq!(
        codes(&named, "memory://workspace.sysml"),
        codes(&default, "memory://workspace.sysml"),
        "naming a library document does not change what the workspace reports"
    );
    assert!(
        named
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.location().document() == "memory://lib.sysml"),
        "the aggregate carries the named document too, so the two cannot disagree"
    );
}

/// A `first X then Y;` control-flow succession statement inside an `action def` body (BNF
/// `ActionDefBodyElement::FirstStmt`) must resolve both ends as `succession` relationships
/// against the two sibling owned action declarations, not fall through to
/// `unsupported_action_definition_member`.
#[test]
fn first_then_succession_inside_action_def_body_resolves_both_ends() {
    let sexpr = semantic_sexpr_for(
        "package P { action def ExecuteMission { action validateRoute; action startMission; first validateRoute then startMission; } }",
    );
    assert!(
        sexpr.contains("(kind succession)"),
        "expected a succession relationship kind, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
    // Both ends resolve to their sibling declarations, not unresolved/unsupported.
    assert!(
        sexpr.matches("(kind succession)").count() >= 2,
        "expected a succession reference for both the `first` and `then` ends, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)") || !sexpr.contains("succession"),
        "did not expect an unresolved succession outcome for two declared siblings, got: {sexpr}"
    );
}

/// A `state def` body's `entry action X;` / `do action Y;` / `exit action Z;` bindings (BNF
/// `EntryAction`/`DoAction`/`ExitAction.action_reference`) must each resolve to the enclosing
/// package's action declarations (there is no `StateDefBodyElement::ActionUsage` shape --
/// bound actions are ordinarily declared alongside the state def, not nested inside it,
/// mirroring the real corpus fixture `24_state_actions.md`), not fall through to
/// `unsupported_state_definition_member`.
#[test]
fn entry_do_exit_action_bindings_inside_state_def_body_resolve() {
    let sexpr = semantic_sexpr_for(
        "package P { action enter1; action running1; action leave1; state def S { entry action enter1; do action running1; exit action leave1; } }",
    );
    assert!(
        sexpr.contains("(kind entryActionBinding)"),
        "expected an entryActionBinding relationship kind, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind doActionBinding)"),
        "expected a doActionBinding relationship kind, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind exitActionBinding)"),
        "expected an exitActionBinding relationship kind, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_state_definition_member"),
        "did not expect unsupported_state_definition_member, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected all three action bindings to resolve to their sibling declarations, got: {sexpr}"
    );
}

/// A `state def` body's `then <target>;` initial-state marker (BNF `ThenStmt.state_reference`)
/// must resolve to the sibling owned state declaration, not fall through to
/// `unsupported_state_definition_member`.
#[test]
fn then_initial_state_inside_state_def_body_resolves() {
    let sexpr = semantic_sexpr_for("package P { state def S { state off; then off; } }");
    assert!(
        sexpr.contains("(kind initialState)"),
        "expected an initialState relationship kind, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_state_definition_member"),
        "did not expect unsupported_state_definition_member, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected the `then` target to resolve to its sibling state declaration, got: {sexpr}"
    );
}

/// A transition's shorthand `accept <trigger>;` and `do action <effect>;` clauses must each
/// resolve to their sibling declarations, not fall through to
/// `unsupported_state_definition_member`.
#[test]
fn transition_trigger_and_effect_resolve() {
    let sexpr = semantic_sexpr_for(
        "package P { action doStuff; state def S { state off; state on; transition first off accept trigger1 do doStuff then on; } action trigger1; }",
    );
    assert!(
        sexpr.contains("(kind transitionTrigger)"),
        "expected a transitionTrigger relationship kind, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind transitionEffect)"),
        "expected a transitionEffect relationship kind, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected both the trigger and effect to resolve to their sibling declarations, got: {sexpr}"
    );
}

/// A control-node declaration does not fabricate an unresolved input reference from its name.
#[test]
fn decide_stmt_name_is_not_an_input_reference() {
    let sexpr = semantic_sexpr_for("package P { action def A { decide missing; } }");
    assert!(
        sexpr.contains("(qualified-name \"P::A::missing\"))) (kind decide)"),
        "expected the authored control-node name, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(kind decisionInput)") && !sexpr.contains("(status unresolved)"),
        "did not expect a declaration name to become a reference, got: {sexpr}"
    );
}

/// Standalone `merge`/`fork`/`join` declarations retain their authored names and kinds.
#[test]
fn merge_fork_join_stmts_resolve() {
    let sexpr = semantic_sexpr_for("package P { action def A { merge m; fork f; join j; } }");
    for (kind, name) in [("merge", "m"), ("fork", "f"), ("join", "j")] {
        assert!(
            sexpr.contains(&format!(
                "(qualified-name \"P::A::{name}\"))) (kind {kind})"
            )),
            "expected a named {kind} declaration, got: {sexpr}"
        );
    }
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
    assert!(!sexpr.contains("(kind mergeInput)"));
    assert!(!sexpr.contains("(kind forkInput)"));
    assert!(!sexpr.contains("(kind joinInput)"));
}

/// A `then accept <sig>;` shorthand trigger (`ThenTarget::Accept`, `TransitionAccept::
/// Shorthand`) must resolve its expression operand through the same constraint-expression
/// machinery as an ordinary `accept`, not fall through to `unsupported_action_definition_member`.
#[test]
fn then_accept_shorthand_resolves_its_payload() {
    let sexpr =
        semantic_sexpr_for("package P { attribute def Sig; action def A { then accept Sig; } }");
    assert!(
        sexpr.contains("(kind expressionOperand)"),
        "expected an expressionOperand reference for the accept payload, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// A `then accept at <expr>;` time trigger whose expression is a `new Type(...)` invocation
/// must resolve the invocation callee through the existing `Expression::Invocation`/
/// `InvocationCallee` machinery (session `1c035232`), reused unchanged here.
#[test]
fn then_accept_at_time_trigger_resolves_invocation_callee() {
    let sexpr = semantic_sexpr_for(
        "package P { attribute def Time; action def A { then accept at new Time(); } }",
    );
    assert!(
        sexpr.contains("(kind invocationCallee)"),
        "expected an invocationCallee reference for the `new Time()` constructor, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// A `then accept when <boolExpr>;` change trigger must resolve its dotted feature-chain
/// operand as a `memberAccessOperand` reference, reusing the general `MemberAccess` machinery
/// (session `64318c70`) directly rather than duplicating it.
#[test]
fn then_accept_when_resolves_member_access_operand() {
    let sexpr = semantic_sexpr_for(
        "package P { action def A { action b { attribute f; } then accept when b.f; } }",
    );
    assert!(
        sexpr.contains("(kind memberAccessOperand)"),
        "expected a memberAccessOperand reference for `b.f`, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// A `Transition`'s own `accept at <expr>`/`accept when <expr>`/`accept after <expr>` time
/// trigger (`TransitionAccept::TimeTrigger`) previously fell through to
/// `unsupported_state_definition_member` unconditionally. It now mirrors
/// `lower_then_accept`'s `TimeTrigger` arm, lowering the trigger expression through the
/// general constraint-expression dispatch (picking up `MemberAccess` chains like
/// `vehicle.maintenanceTime`, not just bare `FeatureRef` names).
#[test]
fn transition_time_trigger_resolves_member_access_operand() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Vehicle { attribute maintenanceTime; } state def S { in vehicle : Vehicle; state a; state b; accept at vehicle.maintenanceTime then b; } }",
    );
    assert!(
        sexpr.contains("(kind memberAccessOperand)"),
        "expected a memberAccessOperand reference for `vehicle.maintenanceTime`, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_state_definition_member"),
        "did not expect unsupported_state_definition_member, got: {sexpr}"
    );
}

/// `RequirementDefBodyElement::VariantUsage` (a bare `variant <name>;` member inside a
/// `requirement def`/usage body, e.g. inside a `variation`-flavored requirement choice) was
/// unconditionally unsupported even though `lower_variant_usage` is already shared by
/// `part def`/`part usage` bodies for the identical AST node. Wires the existing lowering
/// into the requirement-shaped body walker.
#[test]
fn requirement_def_variant_usage_resolves() {
    let sexpr = semantic_sexpr_for(
        "package P { requirement def R1; requirement def R2; requirement def choice { variant R1; variant R2; } }",
    );
    assert!(
        sexpr.contains("(kind variant)"),
        "expected a variant reference, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_requirement_definition_member"),
        "did not expect unsupported_requirement_definition_member, got: {sexpr}"
    );
}

/// `RequirementDefBodyElement::RequireConstraint` (`require constraint { ... }`/`assume
/// constraint <name> { ... }`) was unconditionally unsupported even though its body is the
/// exact same `ConstraintDefBody`-shaped `elements` list `lower_constraint_def_body` already
/// walks for `Constraint`/`AssertConstraintMember`. Wires the anonymous and named forms into
/// the requirement-shaped body walker (`lower_require_constraint_member`), covering both
/// `require`/`assume` spellings.
#[test]
fn require_and_assume_constraint_members_resolve() {
    let sexpr = semantic_sexpr_for(
        "package P { attribute massActual; attribute massReqd; requirement def R { require constraint { massActual <= massReqd } assume constraint fuelOk { massActual >= 0 } } }",
    );
    // The two spellings get their own declaration kinds, because the `assume`/`require`
    // keyword is the only thing that carries `RequirementConstraintMembership.kind`.
    assert!(
        sexpr.contains("(kind require-constraint)"),
        "expected a require-constraint declaration for the anonymous `require`, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(qualified-name \"P::R::fuelOk\"))) (kind assume-constraint)"),
        "expected an assume-constraint declaration for `assume constraint fuelOk`, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_requirement_definition_member"),
        "did not expect unsupported_requirement_definition_member, got: {sexpr}"
    );
}

/// A state def/usage body's `final <name>;` body element (BNF `FinalState`) declares a new
/// named final pseudo-state, distinct from `then <target>;`'s reference-to-an-existing-state
/// shape. Must lower as its own `DeclarationKind::FinalState` feature, not fall through to
/// `unsupported_state_definition_member`.
#[test]
fn final_state_declares_named_pseudo_state() {
    let sexpr = semantic_sexpr_for("package P { state def S { final done; } }");
    assert!(
        sexpr.contains("(kind final-state)"),
        "expected a final-state declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_state_definition_member"),
        "did not expect unsupported_state_definition_member, got: {sexpr}"
    );
}

/// A `while <condition> { ... }` loop must lower as an anonymous `while` declaration whose
/// condition resolves its operand and whose nested body recurses back into the same action-
/// body-element dispatch (a nested `action x;` usage must be reachable), not fall through to
/// `unsupported_action_definition_member`.
#[test]
fn while_stmt_condition_and_body_resolve() {
    let sexpr =
        semantic_sexpr_for("package P { action def A { action x; while x { action y; } } }");
    assert!(
        sexpr.contains("(kind while)"),
        "expected a while declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(path (named (kind package) (name \"P\")) (named (kind action-def) (name \"A\")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name \"y\")))"),
        "expected the nested `action y;` body member to be lowered, got: {sexpr}"
    );
}

/// A bare `loop { ... }` (no condition) must lower as an anonymous `loop` declaration whose
/// body recurses, not fall through to `unsupported_action_definition_member`.
#[test]
fn loop_stmt_body_resolves() {
    let sexpr = semantic_sexpr_for("package P { action def A { loop { action y; } } }");
    assert!(
        sexpr.contains("(kind loop)"),
        "expected a loop declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// An `if <condition> { ... } else { ... }` control node must lower as an anonymous `if`
/// declaration whose condition resolves and whose then/else bodies both recurse, not fall
/// through to `unsupported_action_definition_member`.
#[test]
fn if_stmt_condition_and_both_branches_resolve() {
    let sexpr = semantic_sexpr_for(
        "package P { action def A { action x; if x { action y; } else { action z; } } }",
    );
    assert!(
        sexpr.contains("(kind if)"),
        "expected an if declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
    assert!(
        sexpr.contains(
            "(path (named (kind package) (name \"P\")) (named (kind action-def) (name \"A\")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name \"y\")))"
        ) && sexpr.contains(
            "(path (named (kind package) (name \"P\")) (named (kind action-def) (name \"A\")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name \"z\")))"
        ),
        "expected both the then and else branch body members to be lowered, got: {sexpr}"
    );
}

/// A `for <var> in <range> { ... }` loop must lower as an anonymous `forLoop` declaration
/// whose range expression resolves, whose loop variable is declared as a named
/// `forLoopVariable` sibling, and whose body recurses, not fall through to
/// `unsupported_action_definition_member`.
#[test]
fn for_loop_range_variable_and_body_resolve() {
    let sexpr = semantic_sexpr_for(
        "package P { action def A { action items; for i in items { action y; } } }",
    );
    assert!(
        sexpr.contains("(kind for-loop)"),
        "expected a for-loop declaration, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind for-loop-variable)"),
        "expected a for-loop-variable declaration for `i`, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected the for-loop range `items` to resolve to its sibling action, got: {sexpr}"
    );
}

/// `UseCaseDefBodyElement::ActorUsage` (`actor <name> : <Type>;`) was unconditionally
/// unsupported despite being a fully typed node (name, mandatory `type_name`, membership).
/// Wires it into the shared case-family body walker (`lower_actor_usage`), mirroring
/// `lower_requirement_actor_decl`'s shape.
#[test]
fn case_family_actor_usage_resolves() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Person; use case def U { actor driver : Person; } }",
    );
    assert!(
        sexpr.contains("(kind case-actor)"),
        "expected a case-actor declaration for `driver`, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_use_case_definition_member"),
        "did not expect unsupported_use_case_definition_member, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected `driver`'s `Person` type to resolve, got: {sexpr}"
    );
}

/// `UseCaseDefBodyElement::CaseReturnDecl` (`return [part|attribute]? [:>>]? <name>?
/// [:|:>] <Type> [= expr];`) is a fully typed node (declared name, redefinition target, typed
/// or subsetting type reference, bound value) but was unconditionally unsupported. Wires it
/// (`lower_case_return_decl`), mirroring `lower_parameter_declaration`'s shape: a `:>>`
/// redefinition target lowers as an authored `Redefinition` reference, and a `:`-typed name
/// lowers as a `FeatureTyping` reference.
#[test]
fn case_return_decl_resolves_redefinition_and_type() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Engine; part def selectedAlternative; analysis def A { return part :>> selectedAlternative : Engine; } }",
    );
    assert!(
        sexpr.contains("(kind redefinition)"),
        "expected a redefinition relationship for the `:>>` target, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_analysis_case_definition_member"),
        "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected both the redefinition target and the `Engine` type to resolve, got: {sexpr}"
    );
}

/// A bare `return <name> = <expr>;` (no type, no `part`/`attribute` keyword, no `:>>`) is the
/// anonymous-declared-name shape of `CaseReturnDecl`; its value expression should be lowered
/// through the same `classify_expression`/`lower_calc_expression` pipeline `lower_return_
/// decl` (a calc's own `return`) uses.
#[test]
fn case_return_decl_value_expression_resolves() {
    let sexpr = semantic_sexpr_for(
        "package P { analysis def A { attribute source; return computed = source; } }",
    );
    assert!(
        sexpr.contains("(kind parameter)"),
        "expected an anonymous parameter declaration for the bare return, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_analysis_case_definition_member"),
        "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(status unresolved)"),
        "expected `source` to resolve as the return value's operand, got: {sexpr}"
    );
}

/// `UseCaseDefBodyElement::Expression` (a bare result expression directly in an analysis/case
/// body, e.g. `vehicle.mass`) mirrors `CalcDefBodyElement::Expression`'s identical shape: it is
/// the enclosing declaration's own evaluated result, not a new nested declaration.
#[test]
fn case_family_bare_expression_resolves_as_own_result() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Vehicle { attribute mass; } analysis def A { in vehicle : Vehicle; vehicle.mass } }",
    );
    assert!(
        sexpr.contains("(kind memberAccessOperand)"),
        "expected a memberAccessOperand reference for `vehicle.mass`, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_analysis_case_definition_member"),
        "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
    );
}

/// `UseCaseDefBodyElement::Assign`/`ForLoop`/`ThenAction`/`FlowUsage` all already had working
/// `lower_*` functions shared with `ActionDefBodyElement`/`ActionUsageBodyElement`, but were
/// never dispatched from the case-family body walker. Wires all four through the same shared
/// functions.
#[test]
fn case_family_shares_action_body_statement_wiring() {
    let sexpr = semantic_sexpr_for(
        "package P { analysis def A { attribute x; for i in 1 { assign x := i; } } }",
    );
    assert!(
        sexpr.contains("(kind for-loop)"),
        "expected a for-loop declaration inside the analysis def body, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind assign)"),
        "expected an assign declaration nested inside the for-loop body, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_analysis_case_definition_member"),
        "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
    );
}

/// `PerformBodyElement::Action` (an anonymous `perform action { ... }`'s own body, e.g. the
/// OMG spec Annex A vehicle model's `perform action startVehicle { action turnVehicleOn send
/// ... via ...; }`) was unconditionally unsupported despite wrapping the exact same
/// `ActionUsageBodyElement` shape `lower_action_usage_body` already dispatches -- wires it
/// through the shared `lower_action_usage_body_element` dispatcher.
#[test]
fn perform_action_body_element_dispatches_nested_action_usage() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Driver { port p1; } part part0 { perform action startVehicle { action turnVehicleOn send ignitionCmd via driver.p1 { in ignitionCmd:IgnitionCmd; } } } }",
    );
    assert!(
        sexpr.contains("(kind action)"),
        "expected a nested action-usage declaration inside the perform body, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind memberAccessOperand)"),
        "expected the send-usage's dotted `via driver.p1` clause to resolve as memberAccessOperand, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_usage_member"),
        "did not expect unsupported_action_usage_member, got: {sexpr}"
    );
}

/// `Expression::Index` (`base#(index)`, e.g. `assign x := seq#(i);`) had no arm in
/// `lower_constraint_expression`, so both the base and index sub-expressions fell through to
/// unsupported. Recurses into both, mirroring `Tuple`/`CollectionOp`.
#[test]
fn assign_value_index_expression_resolves() {
    let sexpr = semantic_sexpr_for(
        "package P { action def Act { attribute seq; attribute i; assign x := seq#(i); } }",
    );
    assert!(
        sexpr.matches("(kind expressionOperand)").count() >= 2,
        "expected both the index base and index expression to resolve as expressionOperand references, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// `Expression::Null` (KerML `null`) had no arm in `lower_constraint_expression`, so `assign x
/// := null;` fell through to unsupported even though it needs no reference resolution at all,
/// mirroring the existing literal arms.
#[test]
fn assign_value_null_literal_is_supported() {
    let sexpr = semantic_sexpr_for("package P { action def Act { assign x := null; } }");
    assert!(
        sexpr.contains("(kind assign)"),
        "expected an assign declaration, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_definition_member"),
        "did not expect unsupported_action_definition_member, got: {sexpr}"
    );
}

/// `flow of <payload> from <a> to <b>;` (the payload-first anonymous flow shorthand, BNF §6
/// G12) was unconditionally unsupported purely because `payload.is_some()` was treated the
/// same as a genuinely out-of-scope named/typed flow -- widens `lower_flow_usage` to resolve
/// the payload's type as a new `FlowPayloadType` reference alongside `FlowSource`/
/// `FlowTarget`.
#[test]
fn flow_usage_with_payload_only_resolves() {
    let sexpr = semantic_sexpr_for(
        "package P { item def Exposure; action def Focus { out xrsl: Exposure; } action def Shoot { in xsf: Exposure; } action takePicture { action focus: Focus; action shoot: Shoot; flow of Exposure from focus.xrsl to shoot.xsf; } }",
    );
    assert!(
        sexpr.contains("(kind flow)"),
        "expected a flow declaration, got: {sexpr}"
    );
    assert!(
        sexpr.contains("(kind flowPayloadType)"),
        "expected the payload type to resolve as flowPayloadType, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("unsupported_action_usage_member"),
        "did not expect unsupported_action_usage_member, got: {sexpr}"
    );
}

#[test]
fn a_bare_bound_sets_both_multiplicity_bounds() {
    let sexpr = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel[3]; }");
    assert!(
        sexpr.contains("(multiplicity (lower 3) (upper 3))"),
        "expected `[3]` to set both bounds to 3, got: {sexpr}"
    );
}

/// `[*]` writes neither bound and `[1..*]` writes only the lower one, so both render their
/// missing side as `unbounded` -- but a declaration with no `[...]` at all publishes no
/// multiplicity fact whatsoever, which is a different answer from `[*]`.
#[test]
fn unwritten_and_absent_multiplicity_bounds_stay_distinct() {
    let unbounded = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel[*]; }");
    assert!(
        unbounded.contains("(multiplicity (lower unbounded) (upper unbounded))"),
        "expected `[*]` to publish an unbounded multiplicity fact, got: {unbounded}"
    );

    let lower_only = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel[1..*]; }");
    assert!(
        lower_only.contains("(multiplicity (lower 1) (upper unbounded))"),
        "expected `[1..*]` to keep its authored lower bound, got: {lower_only}"
    );

    let absent = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel; }");
    assert!(
        !absent.contains("(multiplicity"),
        "expected no multiplicity fact when none is authored, got: {absent}"
    );
}

/// A bound the parser records as a non-literal `Expression` is published as an explicit
/// non-literal fact rather than folded, dropped, or re-read from source text.
#[test]
fn a_non_literal_multiplicity_bound_is_published_as_an_expression() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Wheel; attribute n : Integer; part wheels : Wheel[1..n]; }",
    );
    assert!(
        sexpr.contains("(multiplicity (lower 1) (upper expression))"),
        "expected the non-literal upper bound published as `expression`, got: {sexpr}"
    );
}

/// A `doc` body element annotates the declaration owning that body, and the recorded text is
/// the raw content between the comment delimiters -- the parser performs no leading-`*`
/// stripping or dedent, so neither does this fact.
#[test]
fn doc_comments_bind_to_the_declaration_owning_their_body() {
    let sexpr = semantic_sexpr_for("package P { part def Wheel { doc /* a wheel */ } }");
    assert!(
        sexpr.contains(r#"(documentation (doc (text " a wheel ")))"#),
        "expected the doc comment bound to the part def, got: {sexpr}"
    );
}

/// All five authored value spellings stay distinguishable: `=`, `:=`, `default =`,
/// `default :=`, and the operator-less bare `default`.
#[test]
fn authored_feature_value_spellings_stay_distinct() {
    let bind = semantic_sexpr_for("package P { attribute mass : Integer = 10; }");
    assert!(
        bind.contains("(feature-value (kind bind))"),
        "expected a plain `=` bind, got: {bind}"
    );

    let assign = semantic_sexpr_for("package P { attribute mass : Integer := 10; }");
    assert!(
        assign.contains("(feature-value (kind assign))"),
        "expected a `:=` assign, got: {assign}"
    );

    let default_bind = semantic_sexpr_for("package P { attribute mass : Integer default = 10; }");
    assert!(
        default_bind.contains("(feature-value (kind bind) (default true))"),
        "expected a `default =` bind, got: {default_bind}"
    );

    let bare_default = semantic_sexpr_for("package P { attribute mass : Integer default 10; }");
    assert!(
        bare_default.contains("(feature-value (kind bind) (default true) (operator false))"),
        "expected the operator-less bare `default` spelling, got: {bare_default}"
    );
}

/// A named declaration whose owner chain passes through an anonymous scope cannot be
/// identified by a qualified name alone -- the anonymous owner contributes no name segment --
/// so it renders the explicit path form instead.
#[test]
fn a_named_declaration_under_an_anonymous_owner_renders_an_explicit_path() {
    let sexpr = semantic_sexpr_for(
        "package P { action def A { action x; if x { action y; } else { action z; } } }",
    );
    assert!(
        sexpr.contains(
            r#"(path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "y")))"#
        ),
        "expected the branch member to render an explicit path, got: {sexpr}"
    );
    assert!(
        !sexpr.contains(r#"(qualified-name "P::A::::y")"#),
        "expected no ambiguous empty-segment qualified name, got: {sexpr}"
    );
}

#[test]
fn binding_connector_query_uses_the_same_paired_fact_for_kerml_binding_members() {
    let published = publication_for(&[(
        "memory://kerml-binding.sysml",
        r#"
package Binding {
classifier C {
    feature left : Integer;
    feature right : Integer;
    binding left = right;
}
}
"#,
    )]);
    let values = match published.binding_connectors() {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected resolved binding connector, got {other:?}"),
    };
    assert_eq!(values.len(), 1);
    assert!(matches!(values[0].source, BindingEndpoint::Resolved(_)));
    assert!(matches!(values[0].target, BindingEndpoint::Resolved(_)));
    assert_eq!(values[0].provenance, RelationshipProvenance::Authored);
}

#[test]
fn binding_connector_checks_are_manifest_scoped_and_preserve_first_missing_prerequisite() {
    let published = publication_for(&[(
        "memory://binding-rule-family.sysml",
        "package Binding { action def Act { action start; action done; bind start = done; } }",
    )]);
    let expected = [
        (
            BindingConnectorCheckKind::FeatureValue,
            BindingConnectorValidationPrerequisite::FeatureValueEndpointFacts,
        ),
        (
            BindingConnectorCheckKind::ExpressionResult,
            BindingConnectorValidationPrerequisite::ExpressionResultEndpointFacts,
        ),
        (
            BindingConnectorCheckKind::FunctionResult,
            BindingConnectorValidationPrerequisite::FunctionResultEndpointFacts,
        ),
        (
            BindingConnectorCheckKind::ConstructorExpressionResultDefaultValueTbd,
            BindingConnectorValidationPrerequisite::NormativeSpecificationTbd,
        ),
        (
            BindingConnectorCheckKind::FeatureReferenceExpression,
            BindingConnectorValidationPrerequisite::FeatureReferenceExpressionTargetAndResult,
        ),
        (
            BindingConnectorCheckKind::InvocationExpressionBehavior,
            BindingConnectorValidationPrerequisite::InvocationExpressionBehaviorEndpointFacts,
        ),
        (
            BindingConnectorCheckKind::InvocationExpressionDefaultValueTbd,
            BindingConnectorValidationPrerequisite::NormativeSpecificationTbd,
        ),
        (
            BindingConnectorCheckKind::AcceptActionUsageReceiver,
            BindingConnectorValidationPrerequisite::AcceptActionUsageReceiverEndpointFacts,
        ),
        (
            BindingConnectorCheckKind::TransitionUsageSource,
            BindingConnectorValidationPrerequisite::TransitionUsageSourceEndpointFacts,
        ),
        (
            BindingConnectorCheckKind::TransitionUsageSuccession,
            BindingConnectorValidationPrerequisite::TransitionUsageSuccessionEndpointFacts,
        ),
        (
            BindingConnectorCheckKind::SatisfyRequirementUsage,
            BindingConnectorValidationPrerequisite::SatisfyRequirementUsageEndpointFacts,
        ),
    ];

    for (rule, prerequisite) in expected {
        let outcome = match published.binding_connector_validation(rule) {
            QueryOutcome::Resolved(outcome) => outcome,
            other => panic!("expected resolved validation outcome for {rule:?}, got {other:?}"),
        };
        assert_eq!(
            outcome,
            BindingConnectorValidationOutcome::Unsupported { prerequisite },
            "{rule:?} must not be mistaken for a satisfied predicate before its canonical endpoint facts exist"
        );
    }
}

#[test]
fn redefinition_checks_are_manifest_scoped_and_preserve_first_missing_prerequisite() {
    let published = publication_for(&[(
        "memory://redefinition-rule-family.sysml",
        "package Model { classifier Parent { feature shared; } classifier Child :> Parent { feature shared; } }",
    )]);
    let expected = [
        (
            RedefinitionCheckKind::FeatureEnd,
            RedefinitionCheckPrerequisite::EndFeaturePositionAndInheritedEnds,
        ),
        (
            RedefinitionCheckKind::FeatureFlowFeature,
            RedefinitionCheckPrerequisite::FlowEndOrdinalAndLibraryAnchors,
        ),
        (
            RedefinitionCheckKind::FeatureOwnedCrossFeatureSpecialization,
            RedefinitionCheckPrerequisite::CrossFeatureAndSubsettingEndpoints,
        ),
        (
            RedefinitionCheckKind::FeatureParameter,
            RedefinitionCheckPrerequisite::ParameterDirectionAndInheritedPosition,
        ),
        (
            RedefinitionCheckKind::FeatureResult,
            RedefinitionCheckPrerequisite::FunctionOrExpressionResult,
        ),
        (
            RedefinitionCheckKind::ConstructorExpressionResultFeature,
            RedefinitionCheckPrerequisite::ConstructorResultAndInstantiatedTypeFeatures,
        ),
        (
            RedefinitionCheckKind::FeatureChainExpressionSourceTarget,
            RedefinitionCheckPrerequisite::FeatureChainSourceTarget,
        ),
        (
            RedefinitionCheckKind::FeatureChainExpressionTarget,
            RedefinitionCheckPrerequisite::FeatureChainSourceTargetAndLibraryAnchor,
        ),
        (
            RedefinitionCheckKind::ActionUsageStateAction,
            RedefinitionCheckPrerequisite::StateSubactionMembershipAndKind,
        ),
        (
            RedefinitionCheckKind::AssignmentActionUsageAccessedFeature,
            RedefinitionCheckPrerequisite::AssignmentActionInputParameterEndpoints,
        ),
        (
            RedefinitionCheckKind::AssignmentActionUsageReferent,
            RedefinitionCheckPrerequisite::AssignmentActionInputParameterEndpoints,
        ),
        (
            RedefinitionCheckKind::AssignmentActionUsageStartingAt,
            RedefinitionCheckPrerequisite::AssignmentActionInputParameterEndpoints,
        ),
        (
            RedefinitionCheckKind::ForLoopActionUsageVar,
            RedefinitionCheckPrerequisite::ForLoopVariableProjection,
        ),
        (
            RedefinitionCheckKind::RequirementUsageObjective,
            RedefinitionCheckPrerequisite::ObjectiveMembershipAndCaseObjective,
        ),
        (
            RedefinitionCheckKind::RenderingUsage,
            RedefinitionCheckPrerequisite::ViewRenderingMembership,
        ),
    ];

    for (rule, prerequisite) in expected {
        assert_eq!(
            published.redefinition_check(rule),
            QueryOutcome::Resolved(RedefinitionCheckOutcome::Unsupported { prerequisite }),
            "{rule:?} must expose its first missing canonical prerequisite rather than infer a relationship"
        );
    }
}

#[test]
fn redefinition_check_outcomes_have_cold_warm_and_schedule_parity() {
    let sources = [(
        "memory://redefinition-parity.sysml",
        "package Model { classifier Parent { feature shared; } classifier Child :> Parent { feature shared; } }",
    )];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let query = |published: &PublishedResolution| {
        [
            RedefinitionCheckKind::FeatureEnd,
            RedefinitionCheckKind::FeatureFlowFeature,
            RedefinitionCheckKind::FeatureOwnedCrossFeatureSpecialization,
            RedefinitionCheckKind::FeatureParameter,
            RedefinitionCheckKind::FeatureResult,
            RedefinitionCheckKind::ConstructorExpressionResultFeature,
            RedefinitionCheckKind::FeatureChainExpressionSourceTarget,
            RedefinitionCheckKind::FeatureChainExpressionTarget,
            RedefinitionCheckKind::ActionUsageStateAction,
            RedefinitionCheckKind::AssignmentActionUsageAccessedFeature,
            RedefinitionCheckKind::AssignmentActionUsageReferent,
            RedefinitionCheckKind::AssignmentActionUsageStartingAt,
            RedefinitionCheckKind::ForLoopActionUsageVar,
            RedefinitionCheckKind::RequirementUsageObjective,
            RedefinitionCheckKind::RenderingUsage,
        ]
        .map(|rule| settled(published.redefinition_check(rule)))
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

#[test]
fn specialization_checks_do_not_launder_authored_or_implied_edges_into_success() {
    let sources = [(
        "memory://specialization-check-rule-family.sysml",
        "package Model { classifier Parent { feature shared; } classifier Child :> Parent { feature shared; } }",
    )];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let expected = [
        (
            SpecializationCheckKind::FeatureOwnedCrossFeature,
            SpecializationCheckPrerequisite::OwnedCrossFeatureOwnerTypes,
        ),
        (
            SpecializationCheckKind::ConstructorExpressionResult,
            SpecializationCheckPrerequisite::ExpressionResultAndInstantiatedType,
        ),
        (
            SpecializationCheckKind::UsageVariationUsage,
            SpecializationCheckPrerequisite::UsageVariationOwner,
        ),
        (
            SpecializationCheckKind::TransitionUsageSuccessionSource,
            SpecializationCheckPrerequisite::TransitionSuccessionSource,
        ),
    ];
    let query = |published: &PublishedResolution| {
        expected.map(|(rule, prerequisite)| {
            assert_eq!(
                published.specialization_check(rule),
                QueryOutcome::Resolved(SpecializationCheckOutcome::Unsupported { prerequisite }),
                "{rule:?} must not treat the model's authored/implied specialization facts as proof of its richer predicate"
            );
            settled(published.specialization_check(rule))
        })
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

#[test]
fn binding_connector_facts_have_sequential_parallel_and_source_order_parity() {
    let sources = [
        (
            "memory://z.sysml",
            "package Z { action def Act { action start; action done; bind start = done; } }",
        ),
        ("memory://a.sysml", "package A { action def Other; }"),
    ];
    let build_with = |sources: &[(&str, &str)], schedule| {
        build(
            BuildRequest::new(
                sources
                    .iter()
                    .map(|(identity, source)| {
                        SourceInput::new(*identity, (*source).to_string(), SourceKind::Workspace)
                    })
                    .collect(),
                schedule,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap()
    };
    let permuted = [sources[1], sources[0]];
    let render = |published: &PublishedResolution| match published.binding_connectors() {
        QueryOutcome::Resolved(values) => values,
        other => panic!("expected binding facts, got {other:?}"),
    };
    let sequential = build_with(&sources, ConstructionSchedule::Sequential);
    let parallel = build_with(&sources, ConstructionSchedule::Parallel);
    let reordered = build_with(&permuted, ConstructionSchedule::Sequential);
    assert_eq!(render(&sequential), render(&parallel));
    assert_eq!(render(&sequential), render(&reordered));
}

#[test]
fn effective_features_follow_usage_typing_and_nearest_inheritance_with_shadowing() {
    let published = publication_for(&[(
        "memory://features.sysml",
        r#"
package P {
part def Base {
    attribute inherited;
    attribute shadowed;
}
part def Vehicle :> Base {
    attribute direct;
    attribute shadowed;
}
part vehicle : Vehicle;
}
"#,
    )]);
    let entries = match published.search_elements(ElementSearch {
        kind: ElementKind::PartUsage,
        source: ElementSource::Workspace,
    }) {
        QueryOutcome::Resolved(entries) => entries,
        other => panic!("expected part usage, got {other:?}"),
    };
    let vehicle = entries
        .iter()
        .find(|entry| published.qualified_name(entry.identity) == Some("P::vehicle"))
        .expect("vehicle usage");
    let features = match published.effective_features(vehicle.identity) {
        QueryOutcome::Resolved(features) => features,
        other => panic!("expected effective features, got {other:?}"),
    };
    assert_eq!(
        features
            .iter()
            .map(|entry| published.qualified_name(entry.identity).unwrap_or_default())
            .collect::<Vec<_>>(),
        vec![
            "P::Vehicle::direct",
            "P::Vehicle::shadowed",
            "P::Base::inherited"
        ]
    );
}

/// A name shared by siblings of *different* kinds needs no occurrence ordinal -- the kind on
/// every path segment already separates them. This is the sibling `sysml-compiler`'s tag byte:
/// `metadata def X` and the `metadata X about ...` annotating it are distinct elements.
#[test]
fn same_name_different_kind_siblings_are_separated_by_kind() {
    let sexpr = semantic_sexpr_for(
        "package P { part def Vehicle; metadata def Safety; metadata Safety about Vehicle; }",
    );
    assert!(
        sexpr.contains(r#"(named (kind metadata-def) (name "Safety"))"#),
        "expected the metadata definition's kind in its identity, got: {sexpr}"
    );
    assert!(
        sexpr.contains(r#"(named (kind metadata) (name "Safety"))"#),
        "expected the metadata usage's kind in its identity, got: {sexpr}"
    );
}

#[test]
fn conformance_is_reflexive_and_transitive() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def A; part def B :> A; part def C :> B; }",
    )]);
    let a = symbol_named(&published, "memory://types.sysml", "P::A");
    let c = symbol_named(&published, "memory://types.sysml", "P::C");

    assert_eq!(
        conformance(published.conforms_to(a, a, SpecializationScope::AnySpecialization)),
        Conformance::Conforms,
        "a type conforms to itself"
    );
    assert_eq!(
        conformance(published.conforms_to(c, a, SpecializationScope::AnySpecialization)),
        Conformance::Conforms,
        "C :> B :> A conforms through the chain"
    );
    assert_eq!(
        conformance(published.conforms_to(a, c, SpecializationScope::AnySpecialization)),
        Conformance::DoesNotConform,
        "conformance is directional"
    );
}

#[test]
fn all_supertypes_includes_the_type_itself() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def A; part def B :> A; }",
    )]);
    let a = symbol_named(&published, "memory://types.sysml", "P::A");
    let b = symbol_named(&published, "memory://types.sysml", "P::B");

    let supertypes = symbols(published.all_supertypes(b, SpecializationScope::AnySpecialization));
    assert!(
        supertypes.contains(&b) && supertypes.contains(&a),
        "the Pilot's allSupertypes is reflexive, got: {supertypes:?}"
    );
}

/// A feature reaches its type through `FeatureTyping`, which is a `Specialization` but not a
/// `Subclassification`. Asking in the narrower scope must not return it.
#[test]
fn specialization_scope_selects_which_paths_count() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def A; part def B :> A; part b : B; }",
    )]);
    let a = symbol_named(&published, "memory://types.sysml", "P::A");
    let b = symbol_named(&published, "memory://types.sysml", "P::B");
    let usage = symbol_named(&published, "memory://types.sysml", "P::b");

    assert_eq!(
        conformance(published.conforms_to(usage, a, SpecializationScope::AnySpecialization)),
        Conformance::Conforms,
        "the usage reaches A through its typing"
    );
    assert_eq!(
        conformance(published.conforms_to(usage, a, SpecializationScope::Subclassification)),
        Conformance::DoesNotConform,
        "a typing edge is not classifier generalization"
    );
    assert_eq!(
        conformance(published.conforms_to(b, a, SpecializationScope::Subclassification)),
        Conformance::Conforms,
        "`:>` between part defs is classifier generalization"
    );
}

#[test]
fn direct_subtypes_reports_the_reverse_edge() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def A; part def B :> A; part def C :> A; }",
    )]);
    let a = symbol_named(&published, "memory://types.sysml", "P::A");
    let b = symbol_named(&published, "memory://types.sysml", "P::B");
    let c = symbol_named(&published, "memory://types.sysml", "P::C");

    let subtypes = symbols(published.direct_subtypes(a, SpecializationScope::Subclassification));
    assert!(
        subtypes.contains(&b) && subtypes.contains(&c) && subtypes.len() == 2,
        "expected both direct specializers, got: {subtypes:?}"
    );
}

#[test]
fn an_untyped_feature_conforms_because_it_inherits_the_typing() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def T; part def A { part x : T; } part def B :> A { part y :>> x; } }",
    )]);
    let general = symbol_named(&published, "memory://types.sysml", "P::A::x");
    let specific = symbol_named(&published, "memory://types.sysml", "P::B::y");

    assert_eq!(
        conformance(published.feature_typing_conforms(specific, general)),
        Conformance::Conforms,
        "a redefinition that declares no typing takes the redefined feature's"
    );
    let effective = match published.effective_types(specific) {
        QueryOutcome::Resolved(types)
        | QueryOutcome::Recovered(types)
        | QueryOutcome::UnsupportedWith(types) => types,
        other => panic!("expected settled effective types, got: {other:?}"),
    };
    assert!(
        effective
            .iter()
            .any(|entry| matches!(entry.origin, EffectiveTypeOrigin::Inherited(_))),
        "the inherited typing must keep the feature it came from, got: {effective:?}"
    );
}

/// KerML §8.4.3.4 has two halves, and a consumer reporting a violation has to say which one
/// failed. Here the types conform and the featuring types do not.
#[test]
fn subsetting_conformance_reports_its_halves_separately() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def T; part def A { part x : T; } part def U { part y : T subsets x; } }",
    )]);
    let subsetting = symbol_named(&published, "memory://types.sysml", "P::U::y");
    let subsetted = symbol_named(&published, "memory://types.sysml", "P::A::x");

    let outcome = match published.subsetting_conforms(subsetting, subsetted) {
        QueryOutcome::Resolved(value)
        | QueryOutcome::Recovered(value)
        | QueryOutcome::UnsupportedWith(value) => value,
        other => panic!("expected a settled subsetting answer, got: {other:?}"),
    };
    assert_eq!(
        outcome.types,
        Conformance::Conforms,
        "both features are typed by T"
    );
    assert_eq!(
        outcome.featuring,
        Conformance::DoesNotConform,
        "U does not specialize A, so it cannot subset A's feature"
    );
}

/// A type query reads one declaration's row, so its answer is a property of that declaration's
/// type structure and nothing else. Growing the model around it must change neither the answer
/// nor its size -- if either moved, some part of the query would be reading the whole model.
#[test]
fn type_query_answers_do_not_grow_with_the_model() {
    let core = "package P { part def A; part def B :> A; part def C :> B; }";
    let mut padded = String::from("package P { part def A; part def B :> A; part def C :> B; }");
    for index in 0..200 {
        padded.push_str(&format!(" package Q{index} {{ part def U{index}; part def V{index} :> U{index}; part v{index} : V{index}; }}"));
    }

    let small = publication_for(&[("memory://types.sysml", core)]);
    let large = publication_for(&[("memory://types.sysml", &padded)]);

    let small_c = symbol_named(&small, "memory://types.sysml", "P::C");
    let large_c = symbol_named(&large, "memory://types.sysml", "P::C");
    let small_a = symbol_named(&small, "memory://types.sysml", "P::A");
    let large_a = symbol_named(&large, "memory://types.sysml", "P::A");

    assert_eq!(
        symbols(small.all_supertypes(small_c, SpecializationScope::AnySpecialization)).len(),
        symbols(large.all_supertypes(large_c, SpecializationScope::AnySpecialization)).len(),
        "C's supertypes are C, B and A whatever else the model contains"
    );
    assert_eq!(
        symbols(small.direct_subtypes(small_a, SpecializationScope::AnySpecialization)).len(),
        symbols(large.direct_subtypes(large_a, SpecializationScope::AnySpecialization)).len(),
        "only B specializes A directly, whatever else the model contains"
    );
    assert_eq!(
        conformance(small.conforms_to(small_c, small_a, SpecializationScope::AnySpecialization)),
        conformance(large.conforms_to(large_c, large_a, SpecializationScope::AnySpecialization)),
        "conformance is a property of the pair, not of the publication's size"
    );
}

/// The set entailments are query-time recursion rather than a closure lookup, so they need the
/// same complexity property the closure has: an answer is a function of the operand structure,
/// not of how much else the publication contains.
#[test]
fn set_entailment_answers_do_not_grow_with_the_model() {
    let core = "package P { classifier Base; classifier L :> Base; classifier R :> Base; \
                classifier U unions L, R; }";
    let mut padded = String::from(core);
    for index in 0..200 {
        padded.push_str(&format!(
            " package Q{index} {{ classifier A{index}; classifier B{index}; \
             classifier U{index} unions A{index}, B{index}; }}"
        ));
    }
    let small = publication_for(&[("memory://sets.kerml", core)]);
    let large = publication_for(&[("memory://sets.kerml", &padded)]);
    for (published, label) in [(&small, "small"), (&large, "large")] {
        let union = symbol_named(published, "memory://sets.kerml", "P::U");
        let base = symbol_named(published, "memory://sets.kerml", "P::Base");
        let left = symbol_named(published, "memory://sets.kerml", "P::L");
        assert_eq!(
            conformance(published.conforms_to(union, base, SpecializationScope::AnySpecialization)),
            Conformance::Conforms,
            "{label}: every operand of U is a Base, so U is one"
        );
        assert_eq!(
            conformance(published.conforms_to(left, union, SpecializationScope::AnySpecialization)),
            Conformance::Conforms,
            "{label}: each operand is included in the union it belongs to"
        );
    }
}

/// A union whose operands reach back to it is malformed, and the entailment has to answer rather
/// than recurse forever. The visiting set is what bounds it.
#[test]
fn mutually_recursive_unions_terminate() {
    let published = publication_for(&[(
        "memory://sets.kerml",
        "package P { classifier Other; classifier A unions B, Other; classifier B unions A, Other; }",
    )]);
    let a = symbol_named(&published, "memory://sets.kerml", "P::A");
    let other = symbol_named(&published, "memory://sets.kerml", "P::Other");
    // `A` is `B` union `Other`, and `B` is `A` union `Other`; nothing establishes that either
    // is an `Other`, and the answer is reached rather than looped over.
    assert_eq!(
        conformance(published.conforms_to(a, other, SpecializationScope::AnySpecialization)),
        Conformance::DoesNotConform
    );
}

/// The published closure carries only what the model actually specializes. A model with no
/// specialization at all publishes no ancestors, so the storage cannot quietly become
/// quadratic in declaration count.
#[test]
fn a_model_without_specialization_publishes_no_supertypes() {
    let published = publication_for(&[(
        "memory://types.sysml",
        "package P { part def A; part def B; part def C; }",
    )]);
    for name in ["P::A", "P::B", "P::C"] {
        let symbol = symbol_named(&published, "memory://types.sysml", name);
        let supertypes =
            symbols(published.all_supertypes(symbol, SpecializationScope::AnySpecialization));
        assert_eq!(supertypes, vec![symbol], "{name} should report only itself");
    }
}

/// Reusing settled outcomes is an optimisation, so it has to be invisible. Everything the
/// publication owns -- facts, type answers and diagnostics -- must come out identical.
#[test]
fn a_seeded_publication_matches_an_unseeded_one() {
    let (seeded, unseeded) =
        seeded_and_unseeded("package W { part def Car :> Lib::Wheel; attribute m : Lib::Mass; }");
    assert_eq!(
        seeded, unseeded,
        "a workspace built against a settled library must publish what a full build does"
    );
    assert!(
        seeded.contains("Lib::Base"),
        "the workspace should reach the library's own supertypes, got: {seeded}"
    );
}

/// A settled library contributes its resolved import references to the effective import
/// indexes rebuilt for a workspace publication. Omitting the settled prefix makes a public
/// import disappear only on the warm path, so a qualified metadata filter becomes unresolved
/// even though the same source resolves in a cold/full build.
#[test]
fn a_seeded_publication_preserves_publicly_reexported_filter_metadata() {
    let library = concat!(
        "standard library package Lib { ",
        "public import Systems::*; ",
        "package Systems { metadata def PartUsage; } ",
        "}"
    );
    let workspace = concat!(
        "package W { ",
        "part candidate; ",
        "view selected { expose candidate; filter @Lib::PartUsage; } ",
        "}"
    );
    let (seeded, unseeded) = seeded_and_unseeded_with_library(library, workspace);
    assert_eq!(
        seeded, unseeded,
        "publicly re-exported metadata must resolve identically with a library stratum"
    );
    assert!(
        seeded.contains("(filterMetadataTest (reference \"Lib::PartUsage\"))"),
        "the parity fixture must exercise the metadata filter reference: {seeded}"
    );
    assert!(
        !seeded.contains("(unresolved (reference \"Lib::PartUsage\"))"),
        "the publicly re-exported metadata reference must settle: {seeded}"
    );
}

/// The parity above only proves what the workspace it builds exercises, and a clean workspace
/// exercises no conformance rule. This one authors a violation of each family that reads a
/// library declaration, so a seeded build that answered any of them differently would show up.
#[test]
fn a_seeded_publication_matches_an_unseeded_one_for_feature_conformance() {
    let (seeded, unseeded) = seeded_and_unseeded(
        "package W { \
         part def Holder { part wrong : Lib::Mass; attribute right : Lib::Mass; } \
         part def Widened :> Lib::Wheel { part slot[0..*] : Lib::Wheel; } \
         part def Narrowed :> Lib::Wheel { part slot[1..2] : Lib::Wheel; } \
         part def Cycle :> Cycle; }",
    );
    assert_eq!(
        seeded, unseeded,
        "feature-conformance decisions must not depend on library-stratum reuse"
    );
    for code in ["incompatible_type_kind", "specialization_cycle"] {
        assert!(
            seeded.contains(code),
            "the parity workspace must actually exercise {code}, got: {seeded}"
        );
    }
}

/// A workspace root sharing a library root's name is the one way a workspace declaration can
/// change what a library reference resolves to. The guard has to notice and fall back, and the
/// fallback has to be invisible too.
#[test]
fn a_workspace_root_colliding_with_the_library_falls_back_to_a_full_solve() {
    let (seeded, unseeded) = seeded_and_unseeded(
        "package Lib { part def Intruder; } package W { part def Car :> Lib::Wheel; }",
    );
    assert_eq!(
        seeded, unseeded,
        "a colliding workspace root must not be answered from stale library outcomes"
    );
}

/// The library leaves `Missing` unresolved. A workspace that then declares a root by that name
/// would newly satisfy the library's own reference, so its outcomes cannot be reused.
#[test]
fn a_workspace_root_answering_an_unsettled_library_reference_falls_back() {
    let library = std::sync::Arc::new(
        build_library_stratum(vec![SourceInput::new(
            "memory://lib.sysml",
            "standard library package Lib { part def Wheel :> Missing; }".to_string(),
            SourceKind::StandardLibrary,
        )])
        .expect("library stratum"),
    );
    let workspace = "part def Missing;";
    let seeded = build(
        BuildRequest::with_library(
            vec![SourceInput::new(
                "memory://workspace.sysml",
                workspace.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
            library,
        )
        .expect("seeded request"),
    )
    .expect("seeded build");
    // The projection reports workspace documents, so the library's own reference is not in it.
    // Its outcome is observable through the reverse type edge instead: if the stratum's
    // unresolved outcome had been reused, nothing would specialize `Missing`.
    let missing = symbol_named(&seeded, "memory://workspace.sysml", "Missing");
    let subtypes = symbols(seeded.direct_subtypes(missing, SpecializationScope::Subclassification));
    assert_eq!(
        subtypes.len(),
        1,
        "the library's reference must resolve against the workspace root rather than staying \
         unresolved from the stratum, got: {subtypes:?}"
    );
}

#[test]
fn a_library_document_cannot_be_admitted_twice() {
    let error = BuildRequest::with_library(
        vec![SourceInput::new(
            "memory://lib.sysml",
            LIBRARY_SOURCE.to_string(),
            SourceKind::Workspace,
        )],
        ConstructionSchedule::Sequential,
        "contract-v1",
        library_stratum(),
    )
    .expect_err("a source already in the stratum must be rejected");
    assert_eq!(error, BuildFailure::DuplicateSourceIdentity);
}

#[test]
fn derived_element_owner_projects_the_canonical_ownership_fact() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { part def Vehicle { attribute mass; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let package = identity_of(&published, "memory://model.sysml", "Model");
    let vehicle = identity_of(&published, "memory://model.sysml", "Model::Vehicle");
    let mass = identity_of(&published, "memory://model.sysml", "Model::Vehicle::mass");

    assert_eq!(
        settled(published.derived_element_owner(mass)),
        DerivedElementOwner::Owner(vehicle)
    );
    assert_eq!(
        settled(published.derived_element_owner(package)),
        DerivedElementOwner::NoOwner
    );
    assert_eq!(
        settled(published.inspect(mass)).owner,
        Some(vehicle),
        "the exact derivation must read the same canonical owner inspection publishes"
    );
}

#[test]
fn derived_element_owner_has_cold_warm_and_schedule_parity() {
    let sources = [(
        "memory://model.sysml",
        "package Model { part def Vehicle { attribute mass; } }",
    )];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let query = |published: &PublishedResolution| {
        let mass = identity_of(published, "memory://model.sysml", "Model::Vehicle::mass");
        settled(published.derived_element_owner(mass))
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

#[test]
fn derived_element_documentation_filters_canonical_typed_forms() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { action def Vehicle { doc /* vehicle documentation */ language \"Alf\" /* vehicle implementation */ } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let vehicle = identity_of(&published, "memory://model.sysml", "Model::Vehicle");
    let documentation = settled(published.element_derived_documentation(
        vehicle,
        ElementDerivedDocumentationCollection::Documentation,
    ));
    assert_eq!(documentation.len(), 1);
    assert_eq!(documentation[0].form, AnnotationForm::Documentation);
    assert_eq!(
        published.text(documentation[0].text).unwrap_or_default(),
        " vehicle documentation "
    );
    assert!(documentation[0].language.is_none());

    let representations = settled(published.element_derived_documentation(
        vehicle,
        ElementDerivedDocumentationCollection::TextualRepresentation,
    ));
    assert_eq!(representations.len(), 1);
    assert_eq!(
        representations[0].form,
        AnnotationForm::TextualRepresentation
    );
    assert_eq!(representations[0].language.as_deref(), Some("Alf"));
    assert_eq!(
        published.text(representations[0].text).unwrap_or_default(),
        " vehicle implementation "
    );
}

#[test]
fn derived_element_documentation_has_cold_warm_and_schedule_parity() {
    let sources = [(
        "memory://model.sysml",
        "package Model { action def Vehicle { doc /* vehicle documentation */ language \"Alf\" /* vehicle implementation */ } }",
    )];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let query = |published: &PublishedResolution| {
        let vehicle = identity_of(published, "memory://model.sysml", "Model::Vehicle");
        [
            ElementDerivedDocumentationCollection::Documentation,
            ElementDerivedDocumentationCollection::TextualRepresentation,
        ]
        .into_iter()
        .map(|collection| settled(published.element_derived_documentation(vehicle, collection)))
        .collect::<Vec<_>>()
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

#[test]
fn namespace_derived_elements_project_canonical_membership_and_import_facts() {
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
    let owned = identity_of(&published, "memory://model.sysml", "Model::Owned");

    let members = settled(
        published.namespace_derived_elements(model, NamespaceDerivedElementCollection::OwnedMember),
    );
    assert_eq!(members.as_ref(), std::slice::from_ref(&owned));
    let imports = settled(
        published.namespace_derived_elements(model, NamespaceDerivedElementCollection::OwnedImport),
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(
        settled(published.inspect(imports[0])).kind,
        ElementKind::Import,
        "the owned-import derivation returns the canonical lowered import declaration"
    );
    assert!(matches!(
        published
            .namespace_derived_elements(owned, NamespaceDerivedElementCollection::OwnedMember,),
        QueryOutcome::Unsupported
    ));
}

#[test]
fn namespace_derived_elements_have_cold_warm_and_schedule_parity() {
    let sources = [
        (
            "memory://library.sysml",
            "package Library { part def Imported; }",
        ),
        (
            "memory://model.sysml",
            "package Model { import Library::*; part def Owned; }",
        ),
    ];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let query = |published: &PublishedResolution| {
        let model = identity_of(published, "memory://model.sysml", "Model");
        [
            NamespaceDerivedElementCollection::OwnedMember,
            NamespaceDerivedElementCollection::OwnedImport,
        ]
        .into_iter()
        .map(|collection| settled(published.namespace_derived_elements(model, collection)))
        .collect::<Vec<_>>()
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

#[test]
fn namespace_import_derived_elements_have_cold_warm_and_schedule_parity() {
    let sources = [
        (
            "memory://library.sysml",
            "package Library { part def Imported; }",
        ),
        (
            "memory://model.sysml",
            "package Model { import Library::*; part def Owned; }",
        ),
    ];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let query = |published: &PublishedResolution| {
        let model = identity_of(published, "memory://model.sysml", "Model");
        settled(published.namespace_import_derived_elements(model))
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

/// The three states an inspector must keep apart. An empty target list alone cannot tell
/// "nothing was written" from "what was written did not resolve".
#[test]
fn a_relationship_family_separates_no_declaration_from_a_failed_one() {
    let published = detail_publication(
        &[("memory://model.sysml", VEHICLE_MODEL)],
        ConstructionSchedule::Sequential,
    );

    let wheel = details_of(&published, "memory://model.sysml", "P::Wheel");
    assert_eq!(wheel.typing.outcome, RelationshipOutcome::NotApplicable);
    assert!(wheel.typing.targets.is_empty());

    let broken = details_of(&published, "memory://model.sysml", "P::broken");
    assert_eq!(broken.typing.outcome, RelationshipOutcome::Unresolved);
    assert!(
        broken.typing.targets.is_empty(),
        "an unresolved typing must not present a guessed target"
    );

    let rover = details_of(&published, "memory://model.sysml", "P::rover");
    assert_eq!(rover.typing.outcome, RelationshipOutcome::Resolved);
    assert_eq!(names(&rover.typing.targets), vec!["Rover"]);
}

/// A family where one reference settled and another did not is not a resolved family.
#[test]
fn a_partly_settled_relationship_family_is_not_reported_as_resolved() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package P { part def Wheel; part def Frame; part axle : Wheel, Missing; }",
        )],
        ConstructionSchedule::Sequential,
    );
    let axle = details_of(&published, "memory://model.sysml", "P::axle");
    assert_eq!(axle.typing.outcome, RelationshipOutcome::Partial);
    assert_eq!(names(&axle.typing.targets), vec!["Wheel"]);
}

#[test]
fn effective_typing_preserves_partial_and_inherited_ambiguous_candidates() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            concat!(
                "package P {\n",
                "  part def A; part def B;\n",
                "  package Left { part shared : A; }\n",
                "  package Right { part shared : B; }\n",
                "  package Use { import Left::*; import Right::*;\n",
                "    part partial : A, Missing;\n",
                "    part inherited subsets shared;\n",
                "  }\n",
                "}\n",
            ),
        )],
        ConstructionSchedule::Sequential,
    );

    let partial = details_of(&published, "memory://model.sysml", "P::Use::partial");
    assert_eq!(
        partial.effective_typing.outcome,
        RelationshipOutcome::Partial,
        "a settled type must not hide another typing that failed"
    );
    assert_eq!(
        names(
            &partial
                .effective_typing
                .types
                .iter()
                .map(|entry| entry.element.clone())
                .collect::<Vec<_>>()
        ),
        vec!["A"]
    );

    let inherited = details_of(&published, "memory://model.sysml", "P::Use::inherited");
    assert_eq!(
        inherited.effective_typing.outcome,
        RelationshipOutcome::Ambiguous
    );
    assert_eq!(
        inherited
            .effective_typing
            .candidates
            .iter()
            .map(|entry| entry.element.name.as_deref().unwrap_or_default())
            .collect::<Vec<_>>(),
        vec!["A", "B"]
    );
    assert!(inherited
        .effective_typing
        .candidates
        .iter()
        .all(|entry| { matches!(entry.origin, EffectiveTypeOrigin::Inherited(_)) }));
}

/// Effective typing is a fact about the declaration, so its outcome distinguishes a feature
/// that inherits nothing from one whose declaration did not resolve.
#[test]
fn effective_typing_reports_its_own_outcome_rather_than_an_empty_list() {
    let published = detail_publication(
        &[("memory://model.sysml", VEHICLE_MODEL)],
        ConstructionSchedule::Sequential,
    );

    let wheel = details_of(&published, "memory://model.sysml", "P::Wheel");
    assert_eq!(
        wheel.effective_typing.outcome,
        RelationshipOutcome::NotApplicable
    );

    let broken = details_of(&published, "memory://model.sysml", "P::broken");
    assert_eq!(
        broken.effective_typing.outcome,
        RelationshipOutcome::Unresolved
    );

    // A feature that declares no typing of its own still has one along its subsetting chain.
    let selected = details_of(&published, "memory://model.sysml", "P::selected");
    assert_eq!(selected.typing.outcome, RelationshipOutcome::NotApplicable);
    assert_eq!(
        selected.effective_typing.outcome,
        RelationshipOutcome::Resolved
    );
    assert_eq!(
        selected
            .effective_typing
            .types
            .iter()
            .map(|entry| entry.element.name.as_deref().unwrap_or_default())
            .collect::<Vec<_>>(),
        vec!["Rover"]
    );
    assert!(
        selected
            .effective_typing
            .types
            .iter()
            .all(|entry| matches!(entry.origin, EffectiveTypeOrigin::Inherited(_))),
        "a type reached through subsetting is inherited, not direct: {:?}",
        selected.effective_typing
    );
}

#[test]
fn view_selection_applies_inherited_metadata_disjunctions_and_conjoins_conditions() {
    let document = "memory://views.sysml";
    let published = detail_publication(
        &[(
            (document),
            concat!(
                "package P {\n",
                "  metadata def Safety; metadata def Security;\n",
                "  part safe { @Safety; }\n",
                "  part secure { @Security; }\n",
                "  part both { @Safety; @Security; }\n",
                "  part plain;\n",
                "  view def Classified { filter @Safety | @Security; filter true; }\n",
                "  view selected : Classified;\n",
                "  view requiresBoth { filter @Safety; filter @Security; }\n",
                "}\n",
            ),
        )],
        ConstructionSchedule::Sequential,
    );
    let view = identity_of(&published, document, "P::selected");
    for name in ["P::safe", "P::secure"] {
        let candidate = identity_of(&published, document, name);
        assert_eq!(
            settled(published.view_selection(view, candidate)).outcome,
            ViewSelectionOutcome::Included
        );
    }
    let plain = identity_of(&published, document, "P::plain");
    assert_eq!(
        settled(published.view_selection(view, plain)).outcome,
        ViewSelectionOutcome::Excluded
    );
    let requires_both = identity_of(&published, document, "P::requiresBoth");
    let safe = identity_of(&published, document, "P::safe");
    assert_eq!(
        settled(published.view_selection(requires_both, safe)).outcome,
        ViewSelectionOutcome::Excluded
    );
    let both = identity_of(&published, document, "P::both");
    assert_eq!(
        settled(published.view_selection(requires_both, both)).outcome,
        ViewSelectionOutcome::Included
    );
}

/// Inherited features carry the type that declares them, and a redefinition replaces the
/// feature it redefines even when the redefining feature is anonymous.
#[test]
fn inherited_features_carry_provenance_and_a_redefinition_shadows_the_redefined_feature() {
    let published = detail_publication(
        &[("memory://model.sysml", VEHICLE_MODEL)],
        ConstructionSchedule::Sequential,
    );
    let rover = details_of(&published, "memory://model.sysml", "P::Rover");
    assert_eq!(
        rover
            .inherited_features
            .iter()
            .map(|entry| (
                entry.feature.name.as_deref().unwrap_or_default(),
                entry.declared_in.name.as_deref().unwrap_or_default()
            ))
            .collect::<Vec<_>>(),
        vec![("spare", "Vehicle")],
        "the anonymous `part :>> wheel[4]` must replace the inherited wheel"
    );

    // A usage reaches the same features through the definition it is typed by.
    let usage = details_of(&published, "memory://model.sysml", "P::rover");
    assert!(
        usage
            .inherited_features
            .iter()
            .any(|entry| entry.feature.name.as_deref() == Some("spare")),
        "{:?}",
        usage.inherited_features
    );
}

/// A metadata annotation is a settled binding to the definition it names, not a string.
#[test]
fn metadata_annotations_publish_the_definition_they_bind_to() {
    let published = detail_publication(
        &[("memory://model.sysml", VEHICLE_MODEL)],
        ConstructionSchedule::Sequential,
    );
    let vehicle = details_of(&published, "memory://model.sysml", "P::Vehicle");
    assert_eq!(names(&vehicle.metadata), vec!["Safety"]);

    // An annotation naming nothing publishes no binding at all rather than an empty-named one.
    let unresolved = detail_publication(
        &[(
            "memory://unresolved.sysml",
            "package P { part def Vehicle { @Missing; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let vehicle = details_of(&unresolved, "memory://unresolved.sysml", "P::Vehicle");
    assert!(vehicle.metadata.is_empty(), "{:?}", vehicle.metadata);
}

/// Both directions are published, so an inspector never has to scan the model to find what
/// points at an element.
#[test]
fn relationships_are_published_in_both_directions() {
    let published = detail_publication(
        &[("memory://model.sysml", VEHICLE_MODEL)],
        ConstructionSchedule::Sequential,
    );
    let rover_def = details_of(&published, "memory://model.sysml", "P::Rover");
    assert!(
        rover_def
            .outgoing
            .iter()
            .any(|entry| entry.kind == "specialization"
                && entry.peer.name.as_deref() == Some("Vehicle")),
        "{:?}",
        rover_def.outgoing
    );
    assert!(
        rover_def
            .incoming
            .iter()
            .any(|entry| entry.kind == "typing" && entry.peer.name.as_deref() == Some("rover")),
        "{:?}",
        rover_def.incoming
    );
}

/// Repetition and query order cannot change a published answer.
#[test]
fn repeated_and_reordered_element_detail_queries_return_identical_answers() {
    let published = detail_publication(
        &[("memory://model.sysml", VEHICLE_MODEL)],
        ConstructionSchedule::Sequential,
    );
    let names = ["P::rover", "P::Rover", "P::Vehicle", "P::selected"];
    let forward = names
        .iter()
        .map(|name| {
            render_details(
                &published,
                &details_of(&published, "memory://model.sysml", name),
            )
        })
        .collect::<Vec<_>>();
    let mut reverse = names
        .iter()
        .rev()
        .map(|name| {
            render_details(
                &published,
                &details_of(&published, "memory://model.sysml", name),
            )
        })
        .collect::<Vec<_>>();
    reverse.reverse();
    assert_eq!(forward, reverse);
    // Asking a second time, after every other element has been asked for, must not differ.
    for (index, name) in names.iter().enumerate() {
        assert_eq!(
            forward[index],
            render_details(
                &published,
                &details_of(&published, "memory://model.sysml", name)
            )
        );
    }
}

#[test]
fn feature_membership_publishes_an_implied_type_featuring_fact() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { part def Vehicle { attribute mass; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let vehicle = identity_of(&published, "memory://model.sysml", "Model::Vehicle");
    let mass = identity_of(&published, "memory://model.sysml", "Model::Vehicle::mass");
    assert_eq!(
        type_featuring_relationships(&published, "memory://model.sysml", "Model::Vehicle::mass"),
        vec![ElementRelationship {
            kind: "typeFeaturing",
            provenance: RelationshipProvenance::Implied,
            authored: None,
            target: RelationshipTarget::Resolved(vehicle),
            location: None,
        }]
    );
    assert_eq!(
        settled(published.featuring_types(mass))
            .into_vec()
            .into_iter()
            .map(|value| (value.symbol, value.provenance))
            .collect::<Vec<_>>(),
        vec![(vehicle, RelationshipProvenance::Implied)]
    );
    assert_eq!(settled(published.featuring_type(mass)), Some(vehicle));
}

#[test]
fn authored_type_featuring_suppresses_the_membership_implication() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { classifier Vehicle { feature mass featured by Vehicle; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let vehicle = identity_of(&published, "memory://model.sysml", "Model::Vehicle");
    let mass = identity_of(&published, "memory://model.sysml", "Model::Vehicle::mass");
    let relationships =
        type_featuring_relationships(&published, "memory://model.sysml", "Model::Vehicle::mass");
    assert_eq!(relationships.len(), 1);
    assert_eq!(
        relationships[0].provenance,
        RelationshipProvenance::Authored
    );
    assert_eq!(relationships[0].authored.as_deref(), Some("Vehicle"));
    assert_eq!(
        relationships[0].target,
        RelationshipTarget::Resolved(vehicle)
    );
    assert_eq!(
        settled(published.featuring_types(mass))[0].provenance,
        RelationshipProvenance::Authored
    );
}

#[test]
fn feature_membership_type_featuring_is_canonical_across_cold_warm_and_parallel_publications() {
    let library = SourceInput::new(
        "memory://library.sysml",
        "standard library package Library { classifier Marker; }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { part def Vehicle { attribute mass; } }".to_string(),
        SourceKind::Workspace,
    );
    let publish = |schedule| {
        build(
            BuildRequest::new(
                vec![library.clone(), workspace.clone()],
                schedule,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap()
    };
    let cold = publish(ConstructionSchedule::Sequential);
    let parallel = publish(ConstructionSchedule::Parallel);
    let warm = build(
        BuildRequest::with_library(
            vec![workspace],
            ConstructionSchedule::Parallel,
            "contract-v1",
            std::sync::Arc::new(build_library_stratum(vec![library]).unwrap()),
        )
        .unwrap(),
    )
    .unwrap();
    let render = |published: &PublishedResolution| {
        let mass = identity_of(published, "memory://model.sysml", "Model::Vehicle::mass");
        settled(published.featuring_types(mass))
            .into_vec()
            .into_iter()
            .map(|reference| (reference.symbol, reference.provenance))
            .collect::<Vec<_>>()
    };
    assert_eq!(render(&cold), render(&parallel));
    assert_eq!(render(&cold), render(&warm));
    assert!(matches!(
        render(&cold).as_slice(),
        [(target, RelationshipProvenance::Implied)]
            if target == &identity_of(&cold, "memory://model.sysml", "Model::Vehicle")
    ));
}

#[test]
fn feature_membership_type_featuring_check_uses_the_manifest_scoped_canonical_outcome() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { classifier Vehicle { feature mass; var feature snapshot; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let vehicle = identity_of(&published, "memory://model.sysml", "Model::Vehicle");
    let mass = identity_of(&published, "memory://model.sysml", "Model::Vehicle::mass");
    let snapshot = identity_of(
        &published,
        "memory://model.sysml",
        "Model::Vehicle::snapshot",
    );
    assert_eq!(
        settled(
            published.type_featuring_check(mass, TypeFeaturingCheckKind::FeatureFeatureMembership,)
        ),
        TypeFeaturingCheckOutcome::Satisfied,
    );
    assert_eq!(
        settled(
            published
                .type_featuring_check(snapshot, TypeFeaturingCheckKind::FeatureFeatureMembership,)
        ),
        TypeFeaturingCheckOutcome::Unsupported {
            prerequisite: TypeFeaturingCheckPrerequisite::VariableFeatureSnapshots,
        },
    );
    assert_eq!(
        settled(
            published
                .type_featuring_check(vehicle, TypeFeaturingCheckKind::FeatureFeatureMembership,)
        ),
        TypeFeaturingCheckOutcome::Unsupported {
            prerequisite: TypeFeaturingCheckPrerequisite::FeatureMembershipFacts,
        },
    );
}

#[test]
fn type_featuring_derives_through_authored_feature_chaining() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { classifier Vehicle { feature base featured by Vehicle; feature derived chains base; } }",
        )],
        ConstructionSchedule::Sequential,
    );
    let vehicle = identity_of(&published, "memory://model.sysml", "Model::Vehicle");
    let derived = identity_of(
        &published,
        "memory://model.sysml",
        "Model::Vehicle::derived",
    );
    assert_eq!(
        settled(published.featuring_types(derived))
            .into_vec()
            .into_iter()
            .map(|value| (value.symbol, value.provenance))
            .collect::<Vec<_>>(),
        vec![(vehicle, RelationshipProvenance::Implied)]
    );
    assert!(settled(published.inspect(derived))
        .relationships
        .iter()
        .any(|relationship| relationship.kind == "featureChaining"));
}

#[test]
fn exact_feature_relationship_collections_project_canonical_authored_and_implied_facts() {
    let published = detail_publication(
        &[ (
            "memory://model.sysml",
            "package Model { classifier Vehicle { feature base; feature derived : Vehicle redefines base chains base; } }",
        ) ],
        ConstructionSchedule::Sequential,
    );
    let vehicle = identity_of(&published, "memory://model.sysml", "Model::Vehicle");
    let base = identity_of(&published, "memory://model.sysml", "Model::Vehicle::base");
    let derived = identity_of(
        &published,
        "memory://model.sysml",
        "Model::Vehicle::derived",
    );
    let values = |collection| {
        settled(published.feature_derived_relationships(derived, collection)).into_vec()
    };

    assert!(matches!(
        &values(FeatureDerivedRelationshipCollection::OwnedFeatureChaining)[0],
        ElementRelationship {
            kind: "featureChaining",
            provenance: RelationshipProvenance::Authored,
            target: RelationshipTarget::Resolved(target),
            ..
        } if target == &base
    ));
    assert!(matches!(
        &values(FeatureDerivedRelationshipCollection::OwnedRedefinition)[0],
        ElementRelationship {
            kind: "redefinition",
            provenance: RelationshipProvenance::Authored,
            target: RelationshipTarget::Resolved(target),
            ..
        } if target == &base
    ));
    assert!(matches!(
        &values(FeatureDerivedRelationshipCollection::OwnedSubsetting)[0],
        ElementRelationship {
            kind: "redefinition",
            provenance: RelationshipProvenance::Authored,
            target: RelationshipTarget::Resolved(target),
            ..
        } if target == &base
    ));
    assert!(matches!(
        &values(FeatureDerivedRelationshipCollection::OwnedTyping)[0],
        ElementRelationship {
            kind: "featureTyping",
            provenance: RelationshipProvenance::Authored,
            target: RelationshipTarget::Resolved(target),
            ..
        } if target == &vehicle
    ));
    assert!(matches!(
        &values(FeatureDerivedRelationshipCollection::OwnedTypeFeaturing)[0],
        ElementRelationship {
            kind: "typeFeaturing",
            provenance: RelationshipProvenance::Implied,
            authored: None,
            target: RelationshipTarget::Resolved(target),
            location: None,
        } if target == &vehicle
    ));
    assert!(matches!(
        published.feature_derived_relationships(
            vehicle,
            FeatureDerivedRelationshipCollection::OwnedTyping,
        ),
        QueryOutcome::Unsupported
    ));
}

#[test]
fn feature_relationship_collections_have_sequential_parallel_and_warm_library_parity() {
    let library = SourceInput::new(
        "memory://library.sysml",
        "standard library package Lib { classifier Type; }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { import Lib::*; classifier Vehicle { feature base; feature derived : Type redefines base chains base; } }".to_string(),
        SourceKind::Workspace,
    );
    let full = |schedule| {
        build(
            BuildRequest::new(
                vec![library.clone(), workspace.clone()],
                schedule,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap()
    };
    let sequential = full(ConstructionSchedule::Sequential);
    let parallel = full(ConstructionSchedule::Parallel);
    let stratum = std::sync::Arc::new(build_library_stratum(vec![library]).unwrap());
    let warm = build(
        BuildRequest::with_library(
            vec![workspace],
            ConstructionSchedule::Parallel,
            "contract-v1",
            stratum,
        )
        .unwrap(),
    )
    .unwrap();
    let render = |published: &PublishedResolution| {
        let derived = identity_of(published, "memory://model.sysml", "Model::Vehicle::derived");
        [
            FeatureDerivedRelationshipCollection::OwnedFeatureChaining,
            FeatureDerivedRelationshipCollection::OwnedRedefinition,
            FeatureDerivedRelationshipCollection::OwnedSubsetting,
            FeatureDerivedRelationshipCollection::OwnedTyping,
            FeatureDerivedRelationshipCollection::OwnedTypeFeaturing,
        ]
        .into_iter()
        .map(|collection| settled(published.feature_derived_relationships(derived, collection)))
        .collect::<Vec<_>>()
    };
    assert_eq!(render(&sequential), render(&parallel));
    assert_eq!(render(&sequential), render(&warm));
}

#[test]
fn type_relationship_collections_have_sequential_parallel_and_warm_library_parity() {
    let library = SourceInput::new(
        "memory://library.sysml",
        "standard library package Lib { classifier Base; }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { import Lib::*; classifier Derived specializes Base unions Base intersects Base differences Base disjoint from Base; }".to_string(),
        SourceKind::Workspace,
    );
    let full = |schedule| {
        build(
            BuildRequest::new(
                vec![library.clone(), workspace.clone()],
                schedule,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap()
    };
    let sequential = full(ConstructionSchedule::Sequential);
    let parallel = full(ConstructionSchedule::Parallel);
    let stratum = std::sync::Arc::new(build_library_stratum(vec![library]).unwrap());
    let warm = build(
        BuildRequest::with_library(
            vec![workspace],
            ConstructionSchedule::Parallel,
            "contract-v1",
            stratum,
        )
        .unwrap(),
    )
    .unwrap();
    let render = |published: &PublishedResolution| {
        let derived = identity_of(published, "memory://model.sysml", "Model::Derived");
        [
            TypeDerivedRelationshipCollection::OwnedSpecialization,
            TypeDerivedRelationshipCollection::OwnedUnioning,
            TypeDerivedRelationshipCollection::OwnedIntersecting,
            TypeDerivedRelationshipCollection::OwnedDifferencing,
            TypeDerivedRelationshipCollection::OwnedDisjoining,
            TypeDerivedRelationshipCollection::UnioningType,
            TypeDerivedRelationshipCollection::IntersectingType,
            TypeDerivedRelationshipCollection::DifferencingType,
        ]
        .into_iter()
        .map(|collection| settled(published.type_derived_relationships(derived, collection)))
        .collect::<Vec<_>>()
    };
    assert_eq!(render(&sequential), render(&parallel));
    assert_eq!(render(&sequential), render(&warm));
}

#[test]
fn type_owned_feature_projects_canonical_direct_feature_members() {
    let published = detail_publication(
        &[(
            "memory://model.sysml",
            "package Model { type Container { feature owned; } type Empty; }",
        )],
        ConstructionSchedule::Sequential,
    );
    let container = identity_of(&published, "memory://model.sysml", "Model::Container");
    let owned = identity_of(
        &published,
        "memory://model.sysml",
        "Model::Container::owned",
    );
    let empty = identity_of(&published, "memory://model.sysml", "Model::Empty");
    assert_eq!(
        settled(
            published.type_derived_elements(container, TypeDerivedElementCollection::OwnedFeature,)
        )
        .as_ref(),
        std::slice::from_ref(&owned)
    );
    assert!(settled(
        published.type_derived_elements(empty, TypeDerivedElementCollection::OwnedFeature,)
    )
    .is_empty());
    assert!(matches!(
        published.type_derived_elements(owned, TypeDerivedElementCollection::OwnedFeature),
        QueryOutcome::Unsupported
    ));
}

#[test]
fn type_owned_feature_has_sequential_parallel_and_warm_parity() {
    let sources = [(
        "memory://model.sysml",
        "package Model { type Container { feature alpha; feature beta; } }",
    )];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let query = |published: &PublishedResolution| {
        let container = identity_of(published, "memory://model.sysml", "Model::Container");
        settled(
            published.type_derived_elements(container, TypeDerivedElementCollection::OwnedFeature),
        )
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

#[test]
fn type_owned_end_feature_projects_only_canonical_end_feature_members() {
    let published = detail_publication(
        &[ (
            "memory://model.sysml",
            "package Model { type Container { feature ordinary; end feature endpoint; } type Empty; }",
        ) ],
        ConstructionSchedule::Sequential,
    );
    let container = identity_of(&published, "memory://model.sysml", "Model::Container");
    let endpoint = identity_of(
        &published,
        "memory://model.sysml",
        "Model::Container::endpoint",
    );
    let empty = identity_of(&published, "memory://model.sysml", "Model::Empty");
    assert_eq!(
        settled(
            published
                .type_derived_elements(container, TypeDerivedElementCollection::OwnedEndFeature,)
        )
        .as_ref(),
        [endpoint]
    );
    assert!(settled(
        published.type_derived_elements(empty, TypeDerivedElementCollection::OwnedEndFeature,)
    )
    .is_empty());
}

#[test]
fn type_owned_end_feature_has_sequential_parallel_and_warm_parity() {
    let sources = [(
        "memory://model.sysml",
        "package Model { type Container { end feature alpha; feature beta; end feature gamma; } }",
    )];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let query = |published: &PublishedResolution| {
        let container = identity_of(published, "memory://model.sysml", "Model::Container");
        settled(
            published
                .type_derived_elements(container, TypeDerivedElementCollection::OwnedEndFeature),
        )
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

#[test]
fn definition_usage_derivations_use_canonical_direct_members_and_preserve_missing_fact_boundaries()
{
    let sources = [(
        "memory://definition-usage.sysml",
        "package Model { part def Vehicle { part wheel; action service; } part vehicle; }",
    )];
    let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
    let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
    let warm = detail_publication(&sources, ConstructionSchedule::Sequential);
    let vehicle = identity_of(
        &sequential,
        "memory://definition-usage.sysml",
        "Model::Vehicle",
    );
    let wheel = identity_of(
        &sequential,
        "memory://definition-usage.sysml",
        "Model::Vehicle::wheel",
    );
    let service = identity_of(
        &sequential,
        "memory://definition-usage.sysml",
        "Model::Vehicle::service",
    );
    assert!(matches!(
        sequential.definition_usage_derived(
            vehicle,
            DefinitionUsageDerivedKind::DefinitionOwnedPart,
        ),
        QueryOutcome::Resolved(DefinitionUsageDerivedOutcome::Elements(values))
            if values.as_ref() == [wheel]
    ));
    assert!(matches!(
        sequential.definition_usage_derived(
            vehicle,
            DefinitionUsageDerivedKind::DefinitionOwnedAction,
        ),
        QueryOutcome::Resolved(DefinitionUsageDerivedOutcome::Elements(values))
            if values.as_ref() == [service]
    ));
    // `usage` selects every usage in the effective feature membership, so both direct members
    // appear; `directedUsage` selects none of them, because neither is directed.
    assert!(matches!(
        sequential
            .definition_usage_derived(vehicle, DefinitionUsageDerivedKind::DefinitionUsage,),
        QueryOutcome::Resolved(DefinitionUsageDerivedOutcome::Elements(values))
            if values.contains(&wheel) && values.contains(&service)
    ));
    assert!(matches!(
        sequential.definition_usage_derived(
            vehicle,
            DefinitionUsageDerivedKind::DefinitionDirectedUsage,
        ),
        QueryOutcome::Resolved(DefinitionUsageDerivedOutcome::Elements(values))
            if values.is_empty()
    ));
    assert!(matches!(
        sequential.definition_usage_derived(
            vehicle,
            DefinitionUsageDerivedKind::DefinitionVariantMembership,
        ),
        QueryOutcome::Resolved(DefinitionUsageDerivedOutcome::Unsupported {
            prerequisite: DefinitionUsageDerivedPrerequisite::VariantMembershipIdentity,
        })
    ));
    let query = |published: &PublishedResolution| {
        let vehicle = identity_of(
            published,
            "memory://definition-usage.sysml",
            "Model::Vehicle",
        );
        published.definition_usage_derived(vehicle, DefinitionUsageDerivedKind::DefinitionOwnedPart)
    };
    assert_eq!(query(&sequential), query(&parallel));
    assert_eq!(query(&sequential), query(&warm));
}

#[test]
fn exact_type_derived_facts_publish_closure_values_or_the_first_missing_prerequisite() {
    let published = detail_publication(
        &[ (
            "memory://model.sysml",
            "package Model { classifier Parent { feature inherited; } classifier Child specializes Parent { in feature input; out feature output; end feature endpoint; } classifier Sized[1]; }",
        ) ],
        ConstructionSchedule::Sequential,
    );
    let child = identity_of(&published, "memory://model.sysml", "Model::Child");
    let sized = identity_of(&published, "memory://model.sysml", "Model::Sized");
    let unsupported = |symbol: SymbolId, collection, prerequisite| {
        assert!(matches!(
            published.type_derived_fact(symbol, collection),
            QueryOutcome::Resolved(TypeDerivedFactOutcome::Unsupported { prerequisite: actual })
                if actual == prerequisite
        ));
    };
    let values =
        |symbol: SymbolId, collection| match published.type_derived_fact(symbol, collection) {
            QueryOutcome::Resolved(TypeDerivedFactOutcome::Values(values)) => values,
            other => panic!("expected published values, got {other:?}"),
        };
    let member = |symbol: SymbolId| TypeDerivedFactValue::FeatureMembership { member: symbol };
    let inherited = identity_of(
        &published,
        "memory://model.sysml",
        "Model::Parent::inherited",
    );
    let input = identity_of(&published, "memory://model.sysml", "Model::Child::input");
    let output = identity_of(&published, "memory://model.sysml", "Model::Child::output");
    let endpoint = identity_of(&published, "memory://model.sysml", "Model::Child::endpoint");

    // The Membership relationship identity itself is still unpublished, so only the
    // owned-membership derivation -- whose normative result *is* that relationship -- stays
    // explicitly unsupported.
    unsupported(
        child,
        TypeDerivedFactCollection::OwnedFeatureMembership,
        TypeDerivedFactPrerequisite::FeatureMembershipIdentity,
    );
    unsupported(
        sized,
        TypeDerivedFactCollection::Multiplicity,
        TypeDerivedFactPrerequisite::MultiplicityIdentity,
    );
    // `ownedConjugator` is answered from the authored `conjugation` reference; a type that
    // declares none has an empty value set, not a missing prerequisite.
    assert!(
        values(child, TypeDerivedFactCollection::OwnedConjugator).is_empty(),
        "expected no owned conjugator for an unconjugated type"
    );

    assert_eq!(
        values(child, TypeDerivedFactCollection::InheritedMembership).into_vec(),
        vec![member(inherited)]
    );
    assert_eq!(
        values(child, TypeDerivedFactCollection::InheritedFeature).into_vec(),
        vec![TypeDerivedFactValue::Feature(inherited)]
    );
    for collection in [
        TypeDerivedFactCollection::FeatureMembership,
        TypeDerivedFactCollection::Feature,
    ] {
        let published_values = values(child, collection);
        assert!(published_values.iter().any(|value| match value {
            TypeDerivedFactValue::FeatureMembership { member } => member == &inherited,
            TypeDerivedFactValue::Feature(feature) => feature == &inherited,
            _ => false,
        }));
        assert!(published_values.iter().any(|value| match value {
            TypeDerivedFactValue::FeatureMembership { member } => member == &input,
            TypeDerivedFactValue::Feature(feature) => feature == &input,
            _ => false,
        }));
    }
    assert_eq!(
        values(child, TypeDerivedFactCollection::EndFeature).into_vec(),
        vec![TypeDerivedFactValue::Feature(endpoint)]
    );
    assert_eq!(
        values(child, TypeDerivedFactCollection::Input).into_vec(),
        vec![TypeDerivedFactValue::Feature(input)]
    );
    assert_eq!(
        values(child, TypeDerivedFactCollection::Output).into_vec(),
        vec![TypeDerivedFactValue::Feature(output)]
    );
    let directed = values(child, TypeDerivedFactCollection::DirectedFeature).into_vec();
    assert!(directed.contains(&TypeDerivedFactValue::Feature(input)));
    assert!(directed.contains(&TypeDerivedFactValue::Feature(output)));
}

/// `checkPartDefinitionSpecialization` is published as an implied relationship, then consumed
/// by the ordinary specialization query. The check never appears as an author warning.
#[test]
fn part_definition_specialization_is_implied_from_the_canonical_standard_library_anchor() {
    let library = part_definition_library();
    let workspace = part_definition_workspace();
    let published = build(
        BuildRequest::new(
            vec![library, workspace],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();

    let part = identity_of(&published, "memory://parts.sysml", "Parts::Part");
    assert_eq!(
        settled(published.part_definition_specialization_anchor()),
        part
    );
    let component = identity_of(&published, "memory://model.sysml", "Model::Component");
    let relationships =
        specialization_relationships(&published, "memory://model.sysml", "Model::Component");
    assert_eq!(relationships.len(), 1);
    assert_eq!(relationships[0].provenance, RelationshipProvenance::Implied);
    assert_eq!(relationships[0].authored, None);
    assert_eq!(relationships[0].location, None);
    assert_eq!(relationships[0].target, RelationshipTarget::Resolved(part));
    assert_eq!(
        settled(published.direct_supertypes(component, SpecializationScope::Subclassification,))
            .as_ref(),
        &[part],
    );
    assert!(published.diagnostics().is_empty());
}

/// The generated rule table, rather than the compatibility `PartDefinition` query, owns
/// synthesis and publication. An unrelated generated rule therefore has the same semantic
/// contract: its rule-keyed anchor fact drives the implied edge and the public generic query.
#[test]
fn generated_library_specialization_rules_publish_generic_anchor_outcomes() {
    const ITEM_RULE: &str = "sysml-2.0:8.3.10.2:checkItemDefinitionSpecialization";
    let library = SourceInput::new(
        "memory://items.sysml",
        "standard library package Items { item def Item; }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { item def Component; }".to_string(),
        SourceKind::Workspace,
    );
    let published = build(
        BuildRequest::new(
            vec![library, workspace],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();

    let item = identity_of(&published, "memory://items.sysml", "Items::Item");
    assert_eq!(
        settled(published.library_specialization_anchor(ITEM_RULE)),
        item.clone()
    );
    assert_eq!(
        specialization_relationships(&published, "memory://model.sysml", "Model::Component"),
        vec![ElementRelationship {
            kind: "specialization",
            provenance: RelationshipProvenance::Implied,
            authored: None,
            target: RelationshipTarget::Resolved(item),
            location: None,
        }]
    );
    assert!(published.diagnostics().is_empty());
}

/// The exact flow predicates are evaluated from the canonical endpoint facts: positional
/// `end` declarations for a flow definition, and the typed `from`/`to` endpoint pair for an
/// anonymous flow usage. No consumer reconstructs either collection from source text.
#[test]
fn flow_specializations_publish_implied_edges_from_canonical_end_facts() {
    const BINARY_RULE: &str = "sysml-2.0:8.3.16.2:checkFlowDefinitionBinarySpecialization";
    const FLOW_USAGE_RULE: &str = "sysml-2.0:8.3.16.3:checkFlowUsageFlowSpecialization";
    const FLOW_WITH_ENDS_RULE: &str = "kerml-1.0:8.3.4.9.2:checkFlowWithEndsSpecialization";
    let library = SourceInput::new(
        "memory://flows.sysml",
        "standard library package Flows { flow def Message; flow def flows; } standard library package Transfers { flow def flowTransfers; }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { part def Component; flow def Binary { end source : Component; end target : Component; } action def Owner { action source; action target; flow from source to target; } }".to_string(),
        SourceKind::Workspace,
    );
    let published = build(
        BuildRequest::new(
            vec![library, workspace],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    let message = identity_of(&published, "memory://flows.sysml", "Flows::Message");
    let flows = identity_of(&published, "memory://flows.sysml", "Flows::flows");
    let transfers = identity_of(
        &published,
        "memory://flows.sysml",
        "Transfers::flowTransfers",
    );
    assert_eq!(
        settled(published.library_specialization_anchor(BINARY_RULE)),
        message.clone()
    );
    assert_eq!(
        settled(published.library_specialization_anchor(FLOW_USAGE_RULE)),
        flows.clone()
    );
    assert_eq!(
        settled(published.library_specialization_anchor(FLOW_WITH_ENDS_RULE)),
        transfers.clone()
    );
    assert!(
        specialization_relationships(&published, "memory://model.sysml", "Model::Binary")
            .iter()
            .any(|relationship| {
                relationship.provenance == RelationshipProvenance::Implied
                    && relationship.target == RelationshipTarget::Resolved(message)
            })
    );
    let symbols = settled(published.document_symbols("memory://model.sysml"));
    let flow = symbols
        .iter()
        .find(|entry| entry.kind == ElementKind::FlowConnectionUsage)
        .expect("lowered anonymous flow usage");
    let relationships = settled(published.inspect(flow.identity)).relationships;
    assert!(relationships.iter().any(|relationship| {
        relationship.kind == "specialization"
            && relationship.provenance == RelationshipProvenance::Implied
            && relationship.target == RelationshipTarget::Resolved(flows)
    }));
    assert!(relationships.iter().any(|relationship| {
        relationship.kind == "specialization"
            && relationship.provenance == RelationshipProvenance::Implied
            && relationship.target == RelationshipTarget::Resolved(transfers)
    }));
}

/// Feature category specialization consumes direct authored FeatureTyping outcomes and the
/// owning end/association facts. A class-typed sibling and a feature outside an association
/// demonstrate that neither category rule is inferred from a feature's label or effective
/// display type.
#[test]
fn feature_data_value_and_end_specializations_use_canonical_typing_and_owner_facts() {
    const DATA_VALUE_RULE: &str = "kerml-1.0:8.3.3.3.4:checkFeatureDataValueSpecialization";
    const END_RULE: &str = "kerml-1.0:8.3.3.3.4:checkFeatureEndSpecialization";
    let library = SourceInput::new(
        "memory://feature-anchors.sysml",
        "standard library package Base { feature dataValues; } standard library package Links { class Link { feature participant; } }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.kerml",
        "package Model { datatype Value; class ClassValue; class Owner { feature data : Value; feature ordinary : ClassValue; } assoc Association { end feature endFeature : Value; } class NotAssociation { end feature nonEnd : Value; } }".to_string(),
        SourceKind::Workspace,
    );
    let published = build(
        BuildRequest::new(
            vec![library, workspace],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    let data_values = identity_of(
        &published,
        "memory://feature-anchors.sysml",
        "Base::dataValues",
    );
    let participant = identity_of(
        &published,
        "memory://feature-anchors.sysml",
        "Links::Link::participant",
    );
    let implied = |target| ElementRelationship {
        kind: "specialization",
        provenance: RelationshipProvenance::Implied,
        authored: None,
        target: RelationshipTarget::Resolved(target),
        location: None,
    };
    assert_eq!(
        settled(published.library_specialization_anchor(DATA_VALUE_RULE)),
        data_values.clone()
    );
    assert_eq!(
        settled(published.library_specialization_anchor(END_RULE)),
        participant.clone()
    );
    assert!(
        specialization_relationships(&published, "memory://model.kerml", "Model::Owner::data")
            .contains(&implied(data_values))
    );
    assert!(specialization_relationships(
        &published,
        "memory://model.kerml",
        "Model::Association::endFeature",
    )
    .contains(&implied(participant)));
    assert!(specialization_relationships(
        &published,
        "memory://model.kerml",
        "Model::Owner::ordinary",
    )
    .is_empty());
    assert!(!specialization_relationships(
        &published,
        "memory://model.kerml",
        "Model::NotAssociation::nonEnd",
    )
    .contains(&implied(participant)));
}

/// `Connector::association` is the direct, settled typing target restricted to Association.
/// This rule therefore has a complete fact path without relying on a connector-end body or
/// display-name inference. The binary companion deliberately has separate coverage because
/// its positional endpoint collection is not yet published for KerML connector bodies.
#[test]
fn connector_object_specialization_uses_direct_association_structure_typing() {
    const RULE: &str = "kerml-1.0:8.3.4.5.3:checkConnectorObjectSpecialization";
    let library = SourceInput::new(
        "memory://objects.kerml",
        "standard library package Objects { assoc struct linkObjects; }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.kerml",
        "package Model { assoc struct LinkObject; classifier Holder { connector pair : LinkObject; connector ordinary; } }".to_string(),
        SourceKind::Workspace,
    );
    let publish = |schedule| {
        build(
            BuildRequest::new(
                vec![library.clone(), workspace.clone()],
                schedule,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap()
    };
    let sequential = publish(ConstructionSchedule::Sequential);
    let parallel = publish(ConstructionSchedule::Parallel);
    let anchor = identity_of(
        &sequential,
        "memory://objects.kerml",
        "Objects::linkObjects",
    );
    let implied = ElementRelationship {
        kind: "specialization",
        provenance: RelationshipProvenance::Implied,
        authored: None,
        target: RelationshipTarget::Resolved(anchor),
        location: None,
    };
    assert_eq!(
        settled(sequential.library_specialization_anchor(RULE)),
        anchor.clone()
    );
    assert!(specialization_relationships(
        &sequential,
        "memory://model.kerml",
        "Model::Holder::pair"
    )
    .contains(&implied));
    assert!(
        specialization_relationships(&parallel, "memory://model.kerml", "Model::Holder::pair")
            .contains(&implied)
    );
    assert!(!specialization_relationships(
        &sequential,
        "memory://model.kerml",
        "Model::Holder::ordinary",
    )
    .contains(&implied));
}

/// The pinned Step predicate places `self.isComposite` after its owner disjunction. The
/// manifest extractor preserves that complete spelling while the resolver consumes the same
/// canonical composite and owner facts as every other `CompositeOwnedBy` rule.
#[test]
fn step_subperformance_specialization_uses_composite_behavior_ownership() {
    const RULE: &str = "kerml-1.0:8.3.4.6.3:checkStepSubperformanceSpecialization";
    let library = SourceInput::new(
        "memory://performances.kerml",
        "standard library package Performances { behavior Performance { step subperformance; } }"
            .to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.kerml",
        "package Model { behavior Parent { composite step child; step ordinary; } }".to_string(),
        SourceKind::Workspace,
    );
    let published = build(
        BuildRequest::new(
            vec![library, workspace],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    let anchor = identity_of(
        &published,
        "memory://performances.kerml",
        "Performances::Performance::subperformance",
    );
    let implied = ElementRelationship {
        kind: "specialization",
        provenance: RelationshipProvenance::Implied,
        authored: None,
        target: RelationshipTarget::Resolved(anchor),
        location: None,
    };
    assert_eq!(
        settled(published.library_specialization_anchor(RULE)),
        anchor.clone()
    );
    assert!(specialization_relationships(
        &published,
        "memory://model.kerml",
        "Model::Parent::child"
    )
    .contains(&implied));
    assert!(!specialization_relationships(
        &published,
        "memory://model.kerml",
        "Model::Parent::ordinary",
    )
    .contains(&implied));
}

/// Exact conditional contracts publish both branch anchors at the same barrier. The legacy
/// query remains the `else`/default projection, while the typed branch query exposes the
/// predicate-true anchor without recreating anchor names in a consumer.
#[test]
fn polarity_library_specialization_anchors_are_branch_keyed_and_schedule_stable() {
    const RULE: &str = "sysml-2.0:8.3.21.10:checkSatisfyRequirementUsageSpecialization";
    let sources = || {
        vec![
            SourceInput::new(
                "memory://requirements.sysml",
                "standard library package Requirements { constraint def satisfiedRequirementChecks; constraint def notSatisfiedRequirementChecks; }".to_string(),
                SourceKind::StandardLibrary,
            ),
            SourceInput::new(
                "memory://model.sysml",
                "package Model {}".to_string(),
                SourceKind::Workspace,
            ),
        ]
    };
    let publish = |schedule| {
        build(BuildRequest::new(sources(), schedule, "contract-v1").expect("polarity request"))
            .expect("polarity publication")
    };
    let sequential = publish(ConstructionSchedule::Sequential);
    let parallel = publish(ConstructionSchedule::Parallel);
    let default = identity_of(
        &sequential,
        "memory://requirements.sysml",
        "Requirements::satisfiedRequirementChecks",
    );
    let negated = identity_of(
        &sequential,
        "memory://requirements.sysml",
        "Requirements::notSatisfiedRequirementChecks",
    );
    assert_eq!(
        settled(sequential.library_specialization_anchor(RULE)),
        default
    );
    assert_eq!(
        settled(sequential.library_specialization_anchor_branch(
            RULE,
            LibrarySpecializationAnchorBranch::PredicateTrue,
        )),
        negated
    );
    assert_eq!(
        sequential.library_specialization_anchor_branch(
            RULE,
            LibrarySpecializationAnchorBranch::Default,
        ),
        sequential.library_specialization_anchor(RULE),
    );
    assert_eq!(
        parallel.library_specialization_anchor_branch(
            RULE,
            LibrarySpecializationAnchorBranch::PredicateTrue,
        ),
        sequential.library_specialization_anchor_branch(
            RULE,
            LibrarySpecializationAnchorBranch::PredicateTrue,
        ),
    );
}

#[test]
fn membership_role_specializations_select_published_anchors_and_suppress_ordinary_members() {
    let library = SourceInput::new(
        "memory://roles.sysml",
        "standard library package Requirements { package RequirementCheck { constraint def concerns; constraint def assumptions; constraint def constraints; part actors; part stakeholders; } } standard library package Cases { package Case { part actors; } } standard library package VerificationCases { package VerificationCase { package obj { requirement requirementVerifications; } } }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { part def Component; concern def Safety; requirement def R { subject item : Component; frame concern framed : Safety; actor requirementActor : Component; stakeholder stakeholder : Component; } case def C { actor caseActor : Component; } part ordinary : Component; }".to_string(),
        SourceKind::Workspace,
    );
    let publish = |schedule| {
        build(
            BuildRequest::new(
                vec![library.clone(), workspace.clone()],
                schedule,
                "contract-v1",
            )
            .expect("membership-role request"),
        )
        .expect("membership-role publication")
    };
    let sequential = publish(ConstructionSchedule::Sequential);
    let parallel = publish(ConstructionSchedule::Parallel);
    let target = |name| identity_of(&sequential, "memory://roles.sysml", name);
    let relationships = |published: &PublishedResolution, source| {
        specialization_relationships(published, "memory://model.sysml", source)
    };
    let implied = |target| ElementRelationship {
        kind: "specialization",
        provenance: RelationshipProvenance::Implied,
        authored: None,
        target: RelationshipTarget::Resolved(target),
        location: None,
    };

    assert_eq!(
        relationships(&sequential, "Model::R::framed"),
        vec![implied(target("Requirements::RequirementCheck::concerns"))]
    );
    assert_eq!(
        relationships(&sequential, "Model::R::requirementActor"),
        vec![implied(target("Requirements::RequirementCheck::actors"))]
    );
    assert_eq!(
        relationships(&sequential, "Model::R::stakeholder"),
        vec![implied(target(
            "Requirements::RequirementCheck::stakeholders"
        ))]
    );
    assert_eq!(
        relationships(&sequential, "Model::C::caseActor"),
        vec![implied(target("Cases::Case::actors"))]
    );
    assert!(relationships(&sequential, "Model::ordinary").is_empty());
    assert_eq!(
        relationships(&parallel, "Model::R::requirementActor"),
        relationships(&sequential, "Model::R::requirementActor"),
    );
}

#[test]
fn requirement_derived_facts_use_canonical_membership_roles() {
    let workspace = SourceInput::new(
        "memory://requirements-derived.sysml",
        "package Model { part def Component; concern def Safety; requirement def R { subject item : Component; actor operator : Component; frame concern framed : Safety; } }".to_string(),
        SourceKind::Workspace,
    );
    let publish = |schedule| {
        build(
            BuildRequest::new(vec![workspace.clone()], schedule, "contract-v1")
                .expect("requirements derived request"),
        )
        .expect("requirements derived publication")
    };
    let sequential = publish(ConstructionSchedule::Sequential);
    let parallel = publish(ConstructionSchedule::Parallel);
    let source = identity_of(
        &sequential,
        "memory://requirements-derived.sysml",
        "Model::R",
    );
    let actor = identity_of(
        &sequential,
        "memory://requirements-derived.sysml",
        "Model::R::operator",
    );
    let framed = identity_of(
        &sequential,
        "memory://requirements-derived.sysml",
        "Model::R::framed",
    );
    assert_eq!(
        sequential.requirement_derived_fact(
            source,
            RequirementDerivedFactCollection::DefinitionActorParameter,
        ),
        QueryOutcome::Resolved(RequirementDerivedFactOutcome::Elements(
            vec![actor].into_boxed_slice()
        ))
    );
    assert_eq!(
        sequential.requirement_derived_fact(
            source,
            RequirementDerivedFactCollection::DefinitionFramedConcern,
        ),
        QueryOutcome::Resolved(RequirementDerivedFactOutcome::Elements(
            vec![framed].into_boxed_slice()
        ))
    );
    assert_eq!(
        parallel.requirement_derived_fact(
            identity_of(&parallel, "memory://requirements-derived.sysml", "Model::R"),
            RequirementDerivedFactCollection::DefinitionActorParameter,
        ),
        QueryOutcome::Resolved(RequirementDerivedFactOutcome::Elements(
            vec![identity_of(
                &parallel,
                "memory://requirements-derived.sysml",
                "Model::R::operator",
            )]
            .into_boxed_slice()
        ))
    );
}

#[test]
fn accept_action_specializations_use_canonical_trigger_and_subaction_facts() {
    let library = SourceInput::new(
        "memory://actions.sysml",
        "standard library package Actions { action acceptActions; action def Action { action acceptSubactions; } action def TransitionAction { action accepter; } }".to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { item def Message; action standalone accept payload : Message; action def Parent { action child accept payload : Message; } state def Machine { state source; state target; transition first source accept when true then target; } }".to_string(),
        SourceKind::Workspace,
    );
    let published = build(
        BuildRequest::new(
            vec![library, workspace],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .expect("accept-action request"),
    )
    .expect("accept-action publication");
    let accept_actions = identity_of(
        &published,
        "memory://actions.sysml",
        "Actions::acceptActions",
    );
    let accept_subactions = identity_of(
        &published,
        "memory://actions.sysml",
        "Actions::Action::acceptSubactions",
    );
    let accepter = identity_of(
        &published,
        "memory://actions.sysml",
        "Actions::TransitionAction::accepter",
    );
    let accepts = settled(published.search_elements(ElementSearch {
        kind: ElementKind::AcceptActionUsage,
        source: ElementSource::Workspace,
    }));
    assert_eq!(accepts.len(), 3);
    let targets = accepts
        .iter()
        .map(|accept| {
            settled(published.inspect(accept.identity))
                .relationships
                .iter()
                .filter(|relationship| relationship.kind == "specialization")
                .map(|relationship| relationship.target.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let roles = accepts
        .iter()
        .map(|accept| settled(published.inspect(accept.identity)).role)
        .collect::<Vec<_>>();
    assert_eq!(
        targets,
        vec![
            vec![RelationshipTarget::Resolved(accept_actions)],
            vec![
                RelationshipTarget::Resolved(accept_actions),
                RelationshipTarget::Resolved(accept_subactions),
            ],
            vec![RelationshipTarget::Resolved(accepter)],
        ]
    );
    assert_eq!(
        roles,
        vec![None, None, Some(MembershipRole::TransitionTriggerAction),]
    );
}

#[test]
fn if_action_specialization_uses_the_typed_else_action_fact() {
    let library = SourceInput::new(
        "memory://actions.sysml",
        "standard library package Actions { action ifThenActions; action ifThenElseActions; }"
            .to_string(),
        SourceKind::StandardLibrary,
    );
    let workspace = SourceInput::new(
        "memory://model.sysml",
        "package Model { action def Decision { action condition; if condition { action thenOnly; } if condition { action thenElse; } else { action otherwise; } } }".to_string(),
        SourceKind::Workspace,
    );
    let published = build(
        BuildRequest::new(
            vec![library, workspace],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .expect("if-action request"),
    )
    .expect("if-action publication");
    let if_then = identity_of(
        &published,
        "memory://actions.sysml",
        "Actions::ifThenActions",
    );
    let if_then_else = identity_of(
        &published,
        "memory://actions.sysml",
        "Actions::ifThenElseActions",
    );
    let if_actions = settled(published.search_elements(ElementSearch {
        kind: ElementKind::IfActionUsage,
        source: ElementSource::Workspace,
    }));
    assert_eq!(if_actions.len(), 2);
    let targets = if_actions
        .iter()
        .map(|if_action| {
            settled(published.inspect(if_action.identity))
                .relationships
                .iter()
                .filter(|relationship| relationship.kind == "specialization")
                .map(|relationship| relationship.target.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        targets,
        vec![
            vec![RelationshipTarget::Resolved(if_then)],
            vec![RelationshipTarget::Resolved(if_then_else)],
        ]
    );
}

#[test]
fn satisfy_specialization_selects_the_published_negation_branch() {
    let published = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://requirements.sysml",
                    "standard library package Requirements { constraint def satisfiedRequirementChecks; constraint def notSatisfiedRequirementChecks; }".to_string(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://model.sysml",
                    "package Model { requirement def Safety; part def Vehicle; satisfy Safety by Vehicle; not satisfy Safety by Vehicle; }".to_string(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    let satisfied = identity_of(
        &published,
        "memory://requirements.sysml",
        "Requirements::satisfiedRequirementChecks",
    );
    let not_satisfied = identity_of(
        &published,
        "memory://requirements.sysml",
        "Requirements::notSatisfiedRequirementChecks",
    );
    let uses = settled(published.search_elements(ElementSearch {
        kind: ElementKind::SatisfyRequirementUsage,
        source: ElementSource::Workspace,
    }));
    assert_eq!(uses.len(), 2);
    let targets = uses
        .iter()
        .map(|use_| {
            settled(published.inspect(use_.identity))
                .relationships
                .iter()
                .filter(|relationship| relationship.kind == "specialization")
                .map(|relationship| relationship.target.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        targets,
        vec![
            vec![RelationshipTarget::Resolved(satisfied)],
            vec![RelationshipTarget::Resolved(not_satisfied)],
        ]
    );
}

#[test]
fn assert_specialization_selects_the_published_negation_branch() {
    let published = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://constraints.sysml",
                    "standard library package Constraints { constraint def assertedConstraintChecks; constraint def negatedConstraintChecks; }".to_string(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://model.sysml",
                    "package Model { part def Container { assert constraint Positive { true; } assert not constraint Negative { true; } } }".to_string(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    let asserted = identity_of(
        &published,
        "memory://constraints.sysml",
        "Constraints::assertedConstraintChecks",
    );
    let negated = identity_of(
        &published,
        "memory://constraints.sysml",
        "Constraints::negatedConstraintChecks",
    );
    let assertions = settled(published.search_elements(ElementSearch {
        kind: ElementKind::AssertConstraintUsage,
        source: ElementSource::Workspace,
    }));
    assert_eq!(assertions.len(), 2);
    let targets = assertions
        .iter()
        .map(|assertion| {
            settled(published.inspect(assertion.identity))
                .relationships
                .iter()
                .filter(|relationship| relationship.kind == "specialization")
                .map(|relationship| relationship.target.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        targets,
        vec![
            vec![RelationshipTarget::Resolved(asserted)],
            vec![RelationshipTarget::Resolved(negated)],
        ]
    );
}

#[test]
fn invariant_specialization_selects_the_published_negation_branch() {
    let published = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://performances.sysml",
                    "standard library package Performances { constraint def trueEvaluations; constraint def falseEvaluations; }".to_string(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://model.sysml",
                    "package Model { inv Positive { true } inv not Negative { true } }".to_string(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    let true_evaluation = identity_of(
        &published,
        "memory://performances.sysml",
        "Performances::trueEvaluations",
    );
    let false_evaluation = identity_of(
        &published,
        "memory://performances.sysml",
        "Performances::falseEvaluations",
    );
    let invariants = settled(published.search_elements(ElementSearch {
        kind: ElementKind::Invariant,
        source: ElementSource::Workspace,
    }));
    assert_eq!(invariants.len(), 2);
    let targets = invariants
        .iter()
        .map(|invariant| {
            settled(published.inspect(invariant.identity))
                .relationships
                .iter()
                .filter(|relationship| relationship.kind == "specialization")
                .map(|relationship| relationship.target.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        targets,
        vec![
            vec![RelationshipTarget::Resolved(true_evaluation)],
            vec![RelationshipTarget::Resolved(false_evaluation)],
        ]
    );
}

/// An authored specialization that reaches the canonical anchor is already the effective
/// semantic fact. The resolver must not add a second, redundant direct edge.
#[test]
fn part_definition_authored_equivalent_and_more_specific_specializations_suppress_implication() {
    let published = build(
        BuildRequest::new(
            vec![part_definition_library(), part_definition_workspace()],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    for qualified_name in ["Model::Equivalent", "Model::Specific"] {
        let relationships =
            specialization_relationships(&published, "memory://model.sysml", qualified_name);
        assert_eq!(
            relationships.len(),
            1,
            "{qualified_name}: {relationships:?}"
        );
        assert_eq!(
            relationships[0].provenance,
            RelationshipProvenance::Authored
        );
    }
}

/// A user-facing validation rule observes the settled specialization closure, including the
/// implied PartDefinition edge. This deliberately forms a cycle only after implication: if
/// validation re-walked authored syntax instead of consuming the canonical type facts, the
/// `specialization_cycle` diagnostic would be absent.
#[test]
fn specialization_cycle_validation_consumes_the_implied_part_definition_fact() {
    let published = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://parts.sysml",
                    "standard library package Parts { part def Part specializes Model::Component; }"
                        .to_string(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://model.sysml",
                    "package Model { part def Component; }".to_string(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap(),
    )
    .unwrap();
    let part = identity_of(&published, "memory://parts.sysml", "Parts::Part");
    assert_eq!(
        specialization_relationships(&published, "memory://model.sysml", "Model::Component")
            .as_slice(),
        &[ElementRelationship {
            kind: "specialization",
            provenance: RelationshipProvenance::Implied,
            authored: None,
            target: RelationshipTarget::Resolved(part),
            location: None,
        }]
    );
    assert_eq!(
        published
            .diagnostics()
            .iter()
            .map(|diagnostic| diagnostic.code().as_str())
            .collect::<Vec<_>>(),
        vec!["specialization_cycle"]
    );
}

/// The implied fact is independent of construction schedule and whether the standard library
/// was freshly solved or supplied as a reusable stratum.
#[test]
fn part_definition_specialization_has_sequential_parallel_and_warm_library_parity() {
    let library = part_definition_library();
    let workspace = part_definition_workspace();
    let full = |schedule| {
        build(
            BuildRequest::new(
                vec![library.clone(), workspace.clone()],
                schedule,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap()
    };
    let sequential = full(ConstructionSchedule::Sequential);
    let parallel = full(ConstructionSchedule::Parallel);
    let stratum = std::sync::Arc::new(build_library_stratum(vec![library]).unwrap());
    let warm = build(
        BuildRequest::with_library(
            vec![workspace],
            ConstructionSchedule::Parallel,
            "contract-v1",
            stratum,
        )
        .unwrap(),
    )
    .unwrap();
    let render = |published: &PublishedResolution| {
        let component = identity_of(published, "memory://model.sysml", "Model::Component");
        (
            specialization_relationships(published, "memory://model.sysml", "Model::Component"),
            settled(published.direct_supertypes(component, SpecializationScope::Subclassification)),
            published.diagnostics().iter().cloned().collect::<Vec<_>>(),
        )
    };
    assert_eq!(render(&sequential), render(&parallel));
    assert_eq!(render(&sequential), render(&warm));
}

/// Two documents that both declare `package P` declare two packages, not one.
///
/// The mutable graph merged them, so an unqualified name in one reached the other's members.
/// This layer keeps them apart -- which is what makes each declaration separately addressable
/// (see `duplicate_qualified_names`) -- so an unqualified cross-document name is unresolved
/// rather than silently bound to a sibling package that happens to share a spelling. Pinned
/// here because the element-details service is the surface where the difference is visible.
#[test]
fn same_named_packages_in_two_documents_do_not_share_an_unqualified_scope() {
    let published = detail_publication(
        &[
            ("memory://defs.sysml", "package R { part def Engine; }"),
            ("memory://usage.sysml", "package R { part motor : Engine; }"),
        ],
        ConstructionSchedule::Sequential,
    );
    let motor = details_of(&published, "memory://usage.sysml", "R::motor");
    assert_eq!(motor.typing.outcome, RelationshipOutcome::Unresolved);

    // The qualified form crosses the document boundary, because it names the package.
    let qualified = detail_publication(
        &[
            ("memory://defs.sysml", "package R { part def Engine; }"),
            (
                "memory://usage.sysml",
                "package S { part motor : R::Engine; }",
            ),
        ],
        ConstructionSchedule::Sequential,
    );
    let motor = details_of(&qualified, "memory://usage.sysml", "S::motor");
    assert_eq!(motor.typing.outcome, RelationshipOutcome::Resolved);
}
