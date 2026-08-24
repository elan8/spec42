//! Contract tests for the evaluation phase, driven through the crate's public
//! `build()` / `PublishedResolution` surface. Relocated verbatim from the inline
//! `#[cfg(test)]` modules of `src/lib.rs` and `src/model.rs`.

#![allow(clippy::too_many_lines)]

mod common;

#[allow(unused_imports)]
use common::*;
#[allow(unused_imports)]
use sysml_resolution::*;

#[test]
fn constraint_literal_comparison_evaluates_to_boolean_true() {
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
        "expected `1 < 2` to fold to a published Boolean(true) evaluation fact, got:\n{output}"
    );
    assert!(
        output.contains("(has-evaluation true)"),
        "expected has-evaluation to flip true once a fact publishes, got:\n{output}"
    );
}

#[test]
fn constraint_literal_comparison_evaluates_to_boolean_false() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C { 2 < 1 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean false)))"
        ),
        "expected `2 < 1` to fold to a published Boolean(false) evaluation fact, got:\n{output}"
    );
}

#[test]
fn attribute_quantity_literal_default_value_evaluates_to_quantity_with_folded_magnitude() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass = 0[kg];\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::mass\"))) (state literal) (value (kind quantity) (magnitude (value \
             (kind integer) (integer 0))) (unit \"kg\")))"
        ),
        "expected `attribute mass = 0[kg];` to fold its magnitude to Integer(0) while carrying \
         the authored unit token as a riding-along string fact, got:\n{output}"
    );
}

#[test]
fn constraint_comparison_of_property_against_quantity_literal_resolves_both_operands() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass : ScalarValues::Integer;\n\
         \tconstraint def C { mass > 0[kg] }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(kind expressionOperand) (ordinal 0))\n      (authored-target \"mass\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::mass\")))))"
        ),
        "expected `mass` to resolve as an expressionOperand reference, got:\n{output}"
    );
    assert!(
        !output.contains("unsupported_constraint_definition_member"),
        "expected `mass > 0[kg]` to be a supported shape (quantity-literal leaf), got:\n{output}"
    );
}

#[test]
fn attribute_string_literal_default_value_evaluates_to_string() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute value = \"approved\";\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::value\"))) (state literal) (value (kind string) (value \"approved\")))"
        ),
        "expected `attribute value = \"approved\";` to fold to a published \
         EvaluatedValue::String(\"approved\") evaluation fact, got:\n{output}"
    );
}

#[test]
fn constraint_string_equality_comparison_evaluates_to_boolean_true() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C { \"a\" == \"a\" }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `\"a\" == \"a\"` to fold to a published Boolean(true) evaluation fact, \
         got:\n{output}"
    );
}

#[test]
fn constraint_string_equality_comparison_evaluates_to_boolean_false() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C { \"a\" == \"b\" }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean false)))"
        ),
        "expected `\"a\" == \"b\"` to fold to a published Boolean(false) evaluation fact, \
         got:\n{output}"
    );
}

#[test]
fn assert_constraint_literal_comparison_evaluates_to_boolean_true() {
    // `assert constraint { <boolExpr> }` is semantically an anonymous constraint usage --
    // reuses the exact same `lower_constraint_expression`/`classify_expression`
    // evaluation machinery as `constraint def`/`constraint` (Slice 1, `4ca42166`).
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tpart def P {\n\
         \t\tassert constraint { 1 < 2 }\n\
         \t}\n\
         }\n",
    );
    assert!(
        output.contains("(state evaluated) (value (kind boolean) (boolean true)))"),
        "expected `assert constraint {{ 1 < 2 }}` to fold to a published Boolean(true) \
         evaluation fact, got:\n{output}"
    );
    assert!(
        output.contains("(has-evaluation true)"),
        "expected has-evaluation to flip true once a fact publishes, got:\n{output}"
    );
}

#[test]
fn constraint_resolved_feature_ref_operand_evaluates_to_non_constant() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute x : ScalarValues::Integer;\n\
         \tconstraint def C { x < 2 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state non-constant))"
        ),
        "expected a resolved but non-literal operand `x` to publish NonConstant rather than \
         a fabricated boolean, got:\n{output}"
    );
}

#[test]
fn constraint_collection_op_arrow_invocation_evaluates_to_non_constant() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute x : ScalarValues::Integer;\n\
         \tattribute y : ScalarValues::Integer;\n\
         \tconstraint def C { x->excludes(y) }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state non-constant))"
        ),
        "expected `x->excludes(y)` to publish NonConstant, matching `Invocation`'s own \
         evaluation shape, got:\n{output}"
    );
}

#[test]
fn constraint_undeclared_feature_ref_operand_evaluates_to_unresolved_operand() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C { x < 2 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state unresolved-operand))"
        ),
        "expected an undeclared operand `x` to publish UnresolvedOperand rather than a \
         fabricated boolean, got:\n{output}"
    );
}

/// An expression whose shape this engine does not evaluate says so.
///
/// It previously published nothing, which made the declaration indistinguishable from one that
/// authored no expression at all -- and a consumer asking "does this element have a value" got
/// the same answer for "there is nothing here" and "there is something here I cannot fold".
///
/// See `constraint_unsupported_expression_shape_still_falls_through_to_diagnostic`: an
/// invocation and `-`/`not` unary ops are supported (reference-resolvable) shapes, so this uses
/// `~x` (`UnaryOperator::BitNot`), still genuinely unsupported.
#[test]
fn constraint_unsupported_expression_shape_publishes_an_unsupported_evaluation_state() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C { ~x }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state unsupported))"
        ),
        "expected an unsupported expression shape to publish the explicit unsupported state, \
         got:\n{output}"
    );
    assert!(
        !output.contains("(value "),
        "an unsupported expression must carry no value, got:\n{output}"
    );
}

#[test]
fn calc_literal_addition_evaluates_to_integer() {
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
        "expected `2 + 3` to fold to a published Integer(5) evaluation fact, got:\n{output}"
    );
}

#[test]
fn calc_mixed_multiplication_evaluates_to_promoted_real() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { 2.0 * 3 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind real) (real 6"
        ),
        "expected `2.0 * 3` to fold to a promoted Real(6.0) evaluation fact, got:\n{output}"
    );
}

#[test]
fn calc_integer_division_by_zero_publishes_typed_division_by_zero_not_a_panic() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { 10 / 0 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state division-by-zero))"
        ),
        "expected `10 / 0` to publish a typed DivisionByZero outcome rather than panicking, \
         got:\n{output}"
    );
}

#[test]
fn calc_real_division_by_zero_publishes_typed_division_by_zero() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { 10.0 / 0.0 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state division-by-zero))"
        ),
        "expected `10.0 / 0.0` to publish a typed DivisionByZero outcome rather than a \
         fabricated infinity, got:\n{output}"
    );
}

#[test]
fn calc_propagates_constant_operands_through_referenced_attribute_default_values() {
    // `length` and `width` are both literal-default-valued attributes (slice 3); `Calc`
    // arithmetic-propagates through both, mirroring the constraint-body propagation tests.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute length = 4;\n\
         \tattribute width = 5;\n\
         \tcalc def Calc { length * width }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer 20)))"
        ),
        "expected `length * width` to propagate both attributes' literal defaults and fold to \
         Integer(20), got:\n{output}"
    );
}

#[test]
fn calc_exponent_operator_integer_base_folds_to_integer() {
    // `**` (BinaryOperator::Exp) with a non-negative integer exponent stays `Integer` via
    // `checked_pow`, mirroring `fold_arithmetic`'s other checked-integer arms.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { 2 ** 3 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer 8)))"
        ),
        "expected `2 ** 3` to fold to Integer(8), got:\n{output}"
    );
}

#[test]
fn calc_exponent_operator_real_base_folds_to_real() {
    // `^` (BinaryOperator::Pow) with a `Real` base promotes to `Real` via `f64::powf`, the
    // same `Real`-involving promotion rule `fold_arithmetic` already uses for +/-/*//  /%.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { 2.0 ^ 3 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind real) (real 8"
        ),
        "expected `2.0 ^ 3` to fold to a promoted Real(8.0), got:\n{output}"
    );
}

#[test]
fn calc_exponent_operator_integer_overflow_folds_to_non_constant() {
    // A huge integer base/exponent pairing that overflows `checked_pow` conservatively folds
    // to `NonConstant`, never a panic, mirroring `fold_arithmetic`'s other checked-integer arms.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { 99999999999 ** 99999999999 }\n\
         }\n",
    );
    assert!(
        output.contains("(state non-constant)"),
        "expected an overflowing `**` to publish a NonConstant evaluation fact, \
         got:\n{output}"
    );
}

#[test]
fn constraint_arithmetic_mixed_with_comparison_folds_to_boolean() {
    // Mixing arithmetic into a constraint's comparison shape (`(a + b) > c`) is now supported:
    // `classify_constraint_node` recognizes an arithmetic `BinaryOp` operand nested inside a
    // comparison, reusing the same `EvalNode::Arithmetic` slice-4 already built for calc bodies.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C { (1 + 2) > 0 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `(1 + 2) > 0` (arithmetic mixed with comparison) to fold to a published \
         Boolean(true) evaluation fact, got:\n{output}"
    );
}

#[test]
fn constraint_arithmetic_operand_constant_propagates_to_boolean() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass1 = 2;\n\
         \tattribute mass2 = 3;\n\
         \tattribute massLimit = 4;\n\
         \tconstraint def C { (mass1 + mass2) > massLimit }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `(mass1 + mass2) > massLimit` to constant-propagate through all three \
         attribute defaults and fold to Boolean(true) (2 + 3 = 5 > 4), got:\n{output}"
    );
}

#[test]
fn constraint_ampersand_folds_as_logical_and() {
    // KerML's single-`&` conjunction spelling (`BinaryOperator::BitAnd`, see
    // `is_logical_operator`'s doc comment) combines two comparisons exactly like `and`, e.g.
    // `sysml.library/trig_functions.md`'s `-1.0 <= that & that <= 1.0` shape.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass1 = 2;\n\
         \tattribute massLimit = 10;\n\
         \tconstraint def C { (mass1 < massLimit) & (massLimit > 0) }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `(mass1 < massLimit) & (massLimit > 0)` to fold to Boolean(true) via the same \
         `fold_logical` path as `and`, got:\n{output}"
    );
}

#[test]
fn calc_unary_minus_negates_literal_integer() {
    // Unary negation (`UnaryOperator::Minus`) on a pure-literal calc body folds at
    // construction time (`eval_node_is_pure_literal`), exactly like a bare literal.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { -5 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer -5)))"
        ),
        "expected `-5` to fold to Integer(-5), got:\n{output}"
    );
}

#[test]
fn constraint_unary_not_negates_literal_boolean() {
    // Unary logical negation (`UnaryOperator::Not`) on a literal boolean constraint body.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def C { not true }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean false)))"
        ),
        "expected `not true` to fold to Boolean(false), got:\n{output}"
    );
}

#[test]
fn calc_anonymous_return_decl_arithmetic_evaluates_to_integer() {
    // Slice 5: most real-corpus calc arithmetic lives inside a `return : Type = expr;`
    // declaration, a distinct `CalcDefBodyElement::ReturnDecl` shape bd50fccd (slice 4)
    // deferred. This wires the return declaration's own expression through the exact same
    // classify_expression/lower_calc_expression pipeline slices 1-4 already built.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tcalc def Calc { return : ScalarValues::Integer = 2 + 3; }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (path (named (kind package) (name \"Demo\")) (named (kind calc-def) (name \"Calc\")) (anonymous (kind parameter) (ordinal 0))))) (state evaluated) (value (kind integer) (integer 5)))"
        ),
        "expected `return : Type = 2 + 3;` to fold to a published Integer(5) evaluation fact \
         on the anonymous return declaration, got:\n{output}"
    );
}

#[test]
fn attribute_literal_default_value_publishes_its_own_evaluation_fact() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass = 5;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::mass\"))) (state literal) (value (kind integer) (integer 5)))"
        ),
        "expected a literal attribute default value to publish its own Integer(5) evaluation \
         fact, got:\n{output}"
    );
}

#[test]
fn attribute_arithmetic_default_value_resolves_operands_and_evaluates() {
    // Widened value-assignment handling: `length * width` (arithmetic, not a bare literal)
    // now resolves both operand references and, since both are themselves constant-valued,
    // evaluates via the same classify_expression/EvalNode::Arithmetic machinery
    // slice 4/6ce84b06 built for constraint/calc bodies.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute length = 4;\n\
         \tattribute width = 5;\n\
         \tattribute area = length * width;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::area\"))) (state evaluated) (value (kind integer) (integer 20)))"
        ),
        "expected `attribute area = length * width;` to resolve both operands and fold to \
         Integer(20), got:\n{output}"
    );
    for name in ["length", "width"] {
        assert!(
            output.contains(&format!(
                "(authored-target \"{name}\")\n      (outcome (status resolved) (target \
                 (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::{name}\")))))"
            )),
            "expected `area`'s arithmetic default value operand `{name}` to resolve to its \
             sibling attribute declaration, got:\n{output}"
        );
    }
}

#[test]
fn non_constant_value_assignment_stays_non_constant_not_fabricated() {
    // A resolved-but-non-constant value assignment (`other`'s own default value is not
    // itself a known constant) must stay explicitly NonConstant, never fabricate a value.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute other : ScalarValues::Integer;\n\
         \tattribute mass = other;\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::mass\"))) (state non-constant))"
        ),
        "expected `attribute mass = other;` (operand with no evaluation fact of its own) to \
         stay explicitly NonConstant, got:\n{output}"
    );
}

#[test]
fn constraint_propagates_a_referenced_attributes_literal_default_value() {
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute mass = 5;\n\
         \tconstraint def C { mass > 3 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `mass > 3` to propagate through `attribute mass = 5;`'s own evaluated \
         constant and fold to Boolean(true), got:\n{output}"
    );
}

#[test]
fn constraint_propagates_transitively_through_another_constraints_evaluated_value() {
    // Two-hop propagation through a non-attribute declaration: `B` folds to a literal
    // comparison, and `A` references `B` as a feature operand, so `A` should propagate `B`'s
    // own evaluated Boolean(true) rather than staying NonConstant.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def B { 1 < 2 }\n\
         \tconstraint def A { B == true }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::A\"))) (state evaluated) (value (kind boolean) (boolean true)))"
        ),
        "expected `A` to propagate `B`'s evaluated Boolean(true) and fold to Boolean(true), \
         got:\n{output}"
    );
}

#[test]
fn constraints_referencing_each_others_evaluated_value_publish_non_converged() {
    // A genuine cross-declaration dependency cycle: `A`'s expression operand references `B`,
    // and `B`'s expression operand references `A`. Neither can ever settle to a concrete
    // constant; both must publish the explicit `NonConverged` outcome rather than hang,
    // panic, or fabricate a value.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tconstraint def A { B == true }\n\
         \tconstraint def B { A == true }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::A\"))) (state cyclic))"
        ),
        "expected cyclic constraint A to publish NonConverged, got:\n{output}"
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::B\"))) (state cyclic))"
        ),
        "expected cyclic constraint B to publish NonConverged, got:\n{output}"
    );
}

#[test]
fn constraint_operand_with_no_evaluated_value_at_all_still_stays_non_constant() {
    // `x` has no default value at all (no evaluation fact of its own), so `C` cannot
    // propagate any constant through it and must stay `NonConstant`, not fabricate a value.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute x : ScalarValues::Integer;\n\
         \tconstraint def C { x > 3 }\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::C\"))) (state non-constant))"
        ),
        "expected an operand with no evaluated value at all to keep the expression \
         NonConstant, got:\n{output}"
    );
}

#[test]
fn value_assignment_tuple_of_literals_evaluates_to_non_constant() {
    // A tuple never folds to a single scalar `EvaluatedValue` (see `EvalNode::Invocation`'s
    // doc comment, reused unchanged for `Expression::Tuple`): even an all-literal tuple
    // publishes `NonConstant`, matching the `Invocation`/`Constructor` precedent rather than
    // fabricating an unmodeled composite-value representation.
    let output = build_semantic_sexpr(
        "package Demo {\n\
         \tattribute tuple = (1, 2, 3);\n\
         }\n",
    );
    assert!(
        output.contains(
            "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
             (qualified-name \"Demo::tuple\"))) (state non-constant))"
        ),
        "expected an all-literal tuple to publish NonConstant rather than a fabricated \
         composite value, got:\n{output}"
    );
}

/// A transition `if <guard>;` boolean expression with literal comparison operands must
/// evaluate to a constant `Boolean` through the exact same `classify_expression`/
/// `EvalNode` machinery a `constraint`/`calc` body uses (see `9f63c5a4` and earlier
/// expression/evaluation slices), not a separate transition-specific evaluator.
#[test]
fn transition_guard_with_literal_operands_evaluates() {
    let sexpr = semantic_sexpr_for(
        "package P { state def S { state off; state on; transition first off if 1 < 2 then on; } }",
    );
    assert!(
        sexpr.contains("(value (boolean true))") || sexpr.contains("(boolean true)"),
        "expected the literal guard `1 < 2` to fold to a constant true, got: {sexpr}"
    );
}

/// A transition guard referencing an operand with no known constant value must stay
/// non-constant, not fabricate a truth value.
#[test]
fn transition_guard_with_unresolvable_operand_stays_non_constant() {
    let sexpr = semantic_sexpr_for(
        "package P { state def S { state off; state on; transition first off if missingFlag then on; } }",
    );
    assert!(
        sexpr.contains("(kind expressionOperand)"),
        "expected the guard's feature reference to be lowered as an expressionOperand, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(value (boolean true))") && !sexpr.contains("(value (boolean false))"),
        "did not expect an unresolvable guard operand to fold to a concrete boolean, got: {sexpr}"
    );
}

// --- Evaluation states ------------------------------------------------------------------

/// `EvaluationPolicy::Skip` publishes a coherent resolved model in which every element that
/// has an expression says so explicitly, rather than an empty table a consumer cannot tell
/// from "there was nothing to evaluate".
#[test]
fn skipping_evaluation_publishes_not_run_rather_than_nothing() {
    let source = "package P { attribute mass : Integer = 5; }";

    let evaluated = semantic_sexpr_for(source);
    assert!(
        evaluated.contains("(state literal) (value (kind integer) (integer 5))"),
        "expected the default policy to evaluate, got: {evaluated}"
    );

    let request = BuildRequest::new(
        vec![SourceInput::new(
            "memory://test.sysml",
            source.to_string(),
            SourceKind::Workspace,
        )],
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .unwrap()
    .with_evaluation_policy(EvaluationPolicy::Skip);
    let published = build(request).unwrap();
    let mut skipped = String::new();
    published
        .debug()
        .write_semantic_sexpr(&mut skipped)
        .unwrap();

    assert!(
        skipped.contains("(state not-run)"),
        "expected a declared not-run state, got: {skipped}"
    );
    assert!(
        !skipped.contains("(value (kind"),
        "a skipped build must publish no value, got: {skipped}"
    );
}

/// A value that was *written* and one that was *computed* are both constants, but only the
/// expression's shape tells them apart, and a consumer showing "declared" versus "computed"
/// needs the distinction.
#[test]
fn a_written_literal_and_a_computed_constant_report_different_states() {
    let written = semantic_sexpr_for("package P { attribute mass : Integer = 5; }");
    assert!(
        written.contains("(state literal) (value (kind integer) (integer 5))"),
        "expected a written literal, got: {written}"
    );

    let computed = semantic_sexpr_for("package P { attribute mass : Integer = 2 + 3; }");
    assert!(
        computed.contains("(state evaluated) (value (kind integer) (integer 5))"),
        "expected a computed constant, got: {computed}"
    );
}

/// A value that depends on itself is a property of the model, published as its own state --
/// never a fabricated value, an infinite loop, or a panic.
#[test]
fn a_self_referential_value_reports_the_cyclic_state() {
    let sexpr = semantic_sexpr_for(
        "package P { constraint def C { a } attribute a : Integer = b; attribute b : Integer = a; }",
    );
    assert!(
        sexpr.contains("(state cyclic)"),
        "expected a cyclic evaluation state for the mutually dependent values, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(state cyclic) (value"),
        "a cyclic state must carry no value, got: {sexpr}"
    );
}

/// A failure is not a value: the rendered fact names the failure and stops, so a consumer
/// cannot mistake it for a value of some fallback kind.
#[test]
fn a_division_by_zero_reports_a_failure_and_no_value() {
    let sexpr = semantic_sexpr_for("package P { calc def C { return : Integer = 1 / 0; } }");
    assert!(
        sexpr.contains("(state division-by-zero)"),
        "expected a division-by-zero failure state, got: {sexpr}"
    );
    assert!(
        !sexpr.contains("(state division-by-zero) (value"),
        "a failure must carry no value, got: {sexpr}"
    );
}

/// Whether an element is quantity-typed can only be answered against the library that declares
/// what a quantity value is. Without it the answer is unknown, and publishing "not a quantity"
/// would state as a fact about the model what is really a missing input -- silently ruling out
/// the unit rules rather than reporting that they could not be applied.
#[test]
fn a_missing_quantity_library_leaves_measurement_applicability_unavailable() {
    let workspace = "package P { attribute plain = 1; }";
    let published = publication_for(&[("memory://q.sysml", workspace)]);
    let symbol = probe_symbol(&published, workspace, "memory://q.sysml", "plain");
    let QueryOutcome::Resolved(evaluation) = published.evaluate(symbol) else {
        panic!("the probe must resolve");
    };
    assert_eq!(
        evaluation.expected_measurement,
        ExpectedMeasurement::Unavailable
    );
}

/// With the library admitted, the same shape of element gets the affirmative answer.
#[test]
fn an_admitted_quantity_library_answers_a_non_quantity_element_affirmatively() {
    let workspace = "package P { attribute plain : ScalarValues::Integer = 1; }";
    let published = against_measurement_library(workspace, ConstructionSchedule::Sequential);
    let symbol = probe_symbol(&published, workspace, "memory://workspace.sysml", "plain");
    let QueryOutcome::Resolved(evaluation) = published.evaluate(symbol) else {
        panic!("the probe must resolve");
    };
    assert_eq!(
        evaluation.expected_measurement,
        ExpectedMeasurement::NotApplicable
    );
}

/// Every migrated expression rule the parity cases below rely on actually firing.
const MEASUREMENT_CODES: [&str; 4] = [
    "incompatible_unit_dimension",
    "unknown_unit_symbol",
    "attribute_value_type_mismatch",
    "non_boolean_expression",
];

/// Evaluation, unit resolution and the decisions they feed must not depend on the schedule
/// that built the publication.
#[test]
fn parallel_and_sequential_construction_publish_the_same_evaluation_and_units() {
    let sequential = measurement_publication(ConstructionSchedule::Sequential);
    let parallel = measurement_publication(ConstructionSchedule::Parallel);
    assert_eq!(
        sequential, parallel,
        "evaluation, unit and measurement facts must not depend on construction schedule"
    );
    for code in MEASUREMENT_CODES {
        assert!(
            sequential.contains(code),
            "the parity workspace must actually exercise {code}, got: {sequential}"
        );
    }
}

/// The same facts, reached through a settled library stratum rather than a cold solve.
#[test]
fn a_seeded_publication_matches_an_unseeded_one_for_evaluation_and_units() {
    let library = std::sync::Arc::new(
        build_library_stratum(vec![SourceInput::new(
            "memory://measurement.sysml",
            MEASUREMENT_LIBRARY_SOURCE.to_string(),
            SourceKind::StandardLibrary,
        )])
        .expect("measurement stratum"),
    );
    let seeded = build(
        BuildRequest::with_library(
            vec![SourceInput::new(
                "memory://workspace.sysml",
                MEASUREMENT_WORKSPACE.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
            library,
        )
        .expect("seeded request"),
    )
    .expect("seeded build");
    let seeded = render_publication(&seeded);
    assert_eq!(
        seeded,
        measurement_publication(ConstructionSchedule::Sequential),
        "unit and evaluation decisions must not depend on library-stratum reuse"
    );
    for code in MEASUREMENT_CODES {
        assert!(
            seeded.contains(code),
            "the parity workspace must actually exercise {code}, got: {seeded}"
        );
    }
}

/// The verdict channel is a projection of the same settled value channel, gated by the
/// element's kind, so the two cannot disagree.
#[test]
fn analysis_evaluation_is_a_second_channel_over_the_settled_value() {
    let published = detail_publication(
        &[(
            "memory://analysis.sysml",
            concat!(
                "package P {\n",
                "  attribute plain = 1;\n",
                "  constraint holds { true }\n",
                "  constraint fails { false }\n",
                "  constraint broken { missing }\n",
                "}\n",
            ),
        )],
        ConstructionSchedule::Sequential,
    );

    let plain = details_of(&published, "memory://analysis.sysml", "P::plain");
    assert_eq!(
        plain.analysis,
        AnalysisEvaluation::NotApplicable,
        "an attribute's value is not a verdict"
    );
    assert_eq!(
        plain.evaluation.state,
        EvaluationState::Literal(EvaluatedScalar::Integer(1))
    );

    assert_eq!(
        details_of(&published, "memory://analysis.sysml", "P::holds").analysis,
        AnalysisEvaluation::Verdict(true)
    );
    assert_eq!(
        details_of(&published, "memory://analysis.sysml", "P::fails").analysis,
        AnalysisEvaluation::Verdict(false)
    );

    let broken = details_of(&published, "memory://analysis.sysml", "P::broken");
    assert!(
        matches!(broken.analysis, AnalysisEvaluation::Unsettled(_)),
        "an unsettled constraint must not read as a failing verdict, got {:?}",
        broken.analysis
    );
}

/// A build that does not evaluate reports the verdict channel as not run, which is neither a
/// verdict nor an inapplicable element.
#[test]
fn a_skipped_evaluation_policy_reports_the_verdict_channel_as_not_run() {
    let request = BuildRequest::new(
        vec![SourceInput::new(
            "memory://skip.sysml",
            "package P { constraint holds { true } }".to_string(),
            SourceKind::Workspace,
        )],
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .unwrap()
    .with_evaluation_policy(EvaluationPolicy::Skip);
    let published = build(request).unwrap();
    let holds = details_of(&published, "memory://skip.sysml", "P::holds");
    assert_eq!(holds.evaluation.state, EvaluationState::NotRun);
    assert_eq!(holds.analysis, AnalysisEvaluation::NotRun);
}
