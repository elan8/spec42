//! Phase 5: constant folding over the authored expression shapes.

use sysml_v2_parser::{
    ast::{BinaryOperator, Expression, SequenceExpressionList, UnaryOperator},
    ParsedDocument,
};

use crate::model::EvaluatedValue;

/// Authored occurrence keys join the classifier to canonical resolved reference facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EvalOperand {
    Ordinal(u32),
    /// The end of the terminal member token within this expression's source document.
    MemberAccessEnd(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NumericalFunction {
    Sum,
    Max,
    Min,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallTarget {
    AuthoredEnd(usize),
    Numerical(NumericalFunction),
    Unresolved,
    Unsupported,
}

/// Binary operators share the same absorbing-state priority, independent of operand order.
fn absorbing_outcome(left: &EvaluatedValue, right: &EvaluatedValue) -> Option<EvaluatedValue> {
    use EvaluatedValue::*;
    match (left, right) {
        (NonConverged, _) | (_, NonConverged) => Some(NonConverged),
        (UnresolvedOperand, _) | (_, UnresolvedOperand) => Some(UnresolvedOperand),
        (NonConstant, _) | (_, NonConstant) => Some(NonConstant),
        (DivisionByZero, _) | (_, DivisionByZero) => Some(DivisionByZero),
        (TypeMismatch, _) | (_, TypeMismatch) => Some(TypeMismatch),
        (UnsupportedUnitOperation, _) | (_, UnsupportedUnitOperation) => {
            Some(UnsupportedUnitOperation)
        }
        _ => None,
    }
}

/// A construction-time-classified mirror of a supported constraint/calc expression tree, built by
/// `classify_expression` in lockstep with
/// `lower_constraint_expression`/`lower_calc_expression`'s own left-to-right traversal: each
/// `Operand` leaf's ordinal exactly matches the `ordinal` `push_reference` assigns the
/// `ReferenceKind::ExpressionOperand` reference pushed for the same leaf (both walk literal /
/// feature-ref / parenthesized / comparison shapes identically), so `compute_evaluation` (slice 3)
/// can re-walk this tree at resolution time and pair each `Operand(n)` with the n-th
/// `ExpressionOperand` reference sourced at the same declaration.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EvalNode {
    Literal(EvaluatedValue),
    /// The n-th (0-based) `ExpressionOperand` reference sourced at the owning declaration, in
    /// left-to-right expression order.
    Operand(EvalOperand),
    Comparison(BinaryOperator, Box<EvalNode>, Box<EvalNode>),
    /// Slice 4 (`classify_calc_node`): an arithmetic `BinaryOp` (`Add`/`Sub`/`Mul`/`Div`/`Mod`)
    /// found in a calc-body expression. Widened by the arithmetic/logical-combinator slice:
    /// `classify_constraint_node` now also builds this variant for an arithmetic sub-expression
    /// nested inside (or alongside) a comparison/logical combinator in a constraint body (e.g.
    /// `chassisMass + engine.mass` as a comparison operand), so a constraint body's `EvalNode` tree
    /// may now contain both `Comparison`/`Logical` and `Arithmetic` nodes, unlike slices 1-3.
    Arithmetic(BinaryOperator, Box<EvalNode>, Box<EvalNode>),
    /// A logical `BinaryOp` (`and`/`or`/`xor`/`implies`, `is_logical_operator`) combining two
    /// constraint-body sub-expressions, each of which folds to a `Boolean` (typically a
    /// `Comparison`). Only `classify_constraint_node` builds this variant -- calc bodies stay
    /// comparison/logical-free, unchanged.
    Logical(BinaryOperator, Box<EvalNode>, Box<EvalNode>),
    /// An `Expression::Invocation`/`Expression::Constructor` node (reference-resolution slice; see
    /// `ReferenceKind::InvocationCallee`). Carries each argument's own classified `EvalNode` purely
    /// so `ordinal` keeps advancing past every nested `Operand` leaf in exact lockstep with `lower_
    /// constraint_expression`/`lower_calc_expression`'s traversal (an operand lexically following
    /// the invocation in the same expression must still get the right ordinal) -- the argument
    /// values themselves are never read by `fold_eval_node_pending`, which always folds this node
    /// straight to `EvaluatedValue::NonConstant` regardless of its children's folded values,
    /// exactly as evaluating an invocation's function/constructor semantics is out of scope.
    ///
    /// Unsupported constructor, collection, indexing, and type-check shapes retain their
    /// children for authored operand alignment and stay explicitly nonconstant.
    Invocation(Vec<EvalNode>),
    /// A sequence can be consumed by `sum`, while scalar evaluation remains nonconstant.
    Sequence(Vec<EvalNode>),
    /// An ordinary function call; phase 5 binds its occurrence to a settled callee.
    Call(CallTarget, Vec<EvalNode>),
    /// A unary prefix `UnaryOp` (`-x` negation or `not x` logical negation, see
    /// `UnaryOperator::Minus`/`UnaryOperator::Not`) wrapping a single classified operand, built by
    /// both `classify_constraint_node` and `classify_calc_node` using the exact same recursive
    /// classification call already used for every other single-child shape (`Parenthesized`).
    /// `Plus`/`BitNot`/`Other` unary operators are deliberately out of scope (unsupported), mirroring
    /// `is_arithmetic_operator`'s narrow-slice precedent: only `-`/`not` are folded (`fold_unary`).
    Unary(UnaryOperator, Box<EvalNode>),
}

/// Folds a comparison of two already-literal operands to a `Boolean` outcome. Integer/Real
/// operands compare numerically (mixed Integer/Real is widened to `f64`); Boolean operands
/// support only `Eq`/`Ne`, mirroring `is_comparison_operator`'s scope. Any other literal-type
/// pairing (e.g. comparing a Boolean to an Integer) is conservatively `NonConstant`: SysML typing
/// would reject it, but this slice does not perform type checking, so it never fabricates a
/// truth value for a shape it cannot type.
pub(crate) fn fold_literal_comparison(
    op: BinaryOperator,
    left: EvaluatedValue,
    right: EvaluatedValue,
) -> EvaluatedValue {
    if let Some(outcome) = absorbing_outcome(&left, &right) {
        return outcome;
    }
    fn as_f64(value: EvaluatedValue) -> Option<f64> {
        match value {
            EvaluatedValue::Integer(value) => Some(value as f64),
            EvaluatedValue::Real(value) => Some(value),
            _ => None,
        }
    }
    let result = match (left, right) {
        (EvaluatedValue::Integer(left), EvaluatedValue::Integer(right)) => Some(match op {
            BinaryOperator::Eq | BinaryOperator::StrictEq => left == right,
            BinaryOperator::Ne | BinaryOperator::StrictNe => left != right,
            BinaryOperator::Lt => left < right,
            BinaryOperator::Le => left <= right,
            BinaryOperator::Gt => left > right,
            BinaryOperator::Ge => left >= right,
            _ => return EvaluatedValue::NonConstant,
        }),
        // KerML's strict-identity `===`/`!==` (`StrictEq`/`StrictNe`) fold identically to
        // `==`/`!=` for the already-literal-scalar operands this fold ever sees -- there is no
        // separate "same object identity, different value" case to distinguish once both sides
        // are already-folded constants, so treating them as ordinary equality/inequality is exact,
        // not an approximation.
        (EvaluatedValue::Boolean(left), EvaluatedValue::Boolean(right)) => match op {
            BinaryOperator::Eq | BinaryOperator::StrictEq => Some(left == right),
            BinaryOperator::Ne | BinaryOperator::StrictNe => Some(left != right),
            _ => None,
        },
        (EvaluatedValue::String(left), EvaluatedValue::String(right)) => match op {
            BinaryOperator::Eq | BinaryOperator::StrictEq => Some(left == right),
            BinaryOperator::Ne | BinaryOperator::StrictNe => Some(left != right),
            _ => None,
        },
        (left, right) => match (as_f64(left), as_f64(right)) {
            (Some(left), Some(right)) => Some(match op {
                BinaryOperator::Eq | BinaryOperator::StrictEq => left == right,
                BinaryOperator::Ne | BinaryOperator::StrictNe => left != right,
                BinaryOperator::Lt => left < right,
                BinaryOperator::Le => left <= right,
                BinaryOperator::Gt => left > right,
                BinaryOperator::Ge => left >= right,
                _ => return EvaluatedValue::NonConstant,
            }),
            _ => None,
        },
    };
    result.map_or(EvaluatedValue::NonConstant, EvaluatedValue::Boolean)
}

/// Folds an arithmetic operation of two already-constant operands (slice 4). Numeric promotion
/// rule: `Integer op Integer` stays `Integer` via checked arithmetic (overflow is reported as
/// `NonConstant` rather than panicking or silently wrapping -- the same conservative "cannot fold
/// this" posture the house convention already uses for a mistyped comparison pairing); any pairing
/// involving a `Real` operand (`Real op Real` or mixed `Integer op Real`/`Real op Integer`)
/// promotes the `Integer` side to `f64` and produces a `Real`, matching Rust's/IEEE754's usual
/// widen-to-float promotion and the same `as f64` widening `fold_literal_comparison` already uses
/// for mixed-numeric comparisons -- i.e. no separate reference implementation was consulted, since
/// this repository already committed to that promotion rule for comparisons and arithmetic should
/// not diverge from it. `Div`/`Mod` by a constant zero divisor (integer or real) is intercepted
/// explicitly *before* the operation runs and reports `DivisionByZero`, never a panic (`i64 / 0`
/// panics) or a silently "valid" `f64` infinity/NaN. A `Boolean` operand in an arithmetic position
/// reports `TypeMismatch` (defensive; unreachable via today's supported shapes). `NonConverged`/
/// `UnresolvedOperand`/`NonConstant` operands propagate through unchanged, in the same priority
/// order `fold_literal_comparison`/`fold_eval_node_pending`'s comparison arm already uses.
pub(crate) fn fold_arithmetic(
    op: BinaryOperator,
    left: EvaluatedValue,
    right: EvaluatedValue,
) -> EvaluatedValue {
    if let Some(outcome) = absorbing_outcome(&left, &right) {
        return outcome;
    }
    match (left, right) {
        (EvaluatedValue::Boolean(_), _) | (_, EvaluatedValue::Boolean(_)) => {
            EvaluatedValue::TypeMismatch
        }
        (
            EvaluatedValue::Quantity(left, left_unit),
            EvaluatedValue::Quantity(right, right_unit),
        ) => {
            if left_unit != right_unit || !matches!(op, BinaryOperator::Add | BinaryOperator::Sub) {
                return EvaluatedValue::UnsupportedUnitOperation;
            }
            quantity_result(fold_arithmetic(op, *left, *right), left_unit)
        }
        (
            EvaluatedValue::Quantity(magnitude, unit),
            scalar @ (EvaluatedValue::Integer(_) | EvaluatedValue::Real(_)),
        ) => {
            if !matches!(op, BinaryOperator::Mul | BinaryOperator::Div) {
                return EvaluatedValue::UnsupportedUnitOperation;
            }
            quantity_result(fold_arithmetic(op, *magnitude, scalar), unit)
        }
        (
            scalar @ (EvaluatedValue::Integer(_) | EvaluatedValue::Real(_)),
            EvaluatedValue::Quantity(magnitude, unit),
        ) => {
            if !matches!(op, BinaryOperator::Mul) {
                return EvaluatedValue::UnsupportedUnitOperation;
            }
            quantity_result(fold_arithmetic(op, scalar, *magnitude), unit)
        }
        (EvaluatedValue::Integer(left), EvaluatedValue::Integer(right))
            if matches!(op, BinaryOperator::Pow | BinaryOperator::Exp) =>
        {
            // A negative integer exponent produces a fractional result, so promote to `Real`
            // via `powf` (e.g. `2 ^ -1` folds to `Real(0.5)`) rather than folding to `NonConstant`.
            // A non-negative exponent that does not fit `u32` (checked_pow's exponent type) cannot
            // be computed as an exact integer either way, so it conservatively falls to
            // `NonConstant` rather than silently promoting to a lossy `Real`.
            if right < 0 {
                EvaluatedValue::Real((left as f64).powf(right as f64))
            } else {
                match u32::try_from(right) {
                    Ok(exponent) => left
                        .checked_pow(exponent)
                        .map_or(EvaluatedValue::NonConstant, EvaluatedValue::Integer),
                    Err(_) => EvaluatedValue::NonConstant,
                }
            }
        }
        (EvaluatedValue::Integer(left), EvaluatedValue::Integer(right)) => {
            let result = match op {
                BinaryOperator::Add => left.checked_add(right),
                BinaryOperator::Sub => left.checked_sub(right),
                BinaryOperator::Mul => left.checked_mul(right),
                BinaryOperator::Div => {
                    if right == 0 {
                        return EvaluatedValue::DivisionByZero;
                    }
                    left.checked_div(right)
                }
                BinaryOperator::Mod => {
                    if right == 0 {
                        return EvaluatedValue::DivisionByZero;
                    }
                    left.checked_rem(right)
                }
                _ => return EvaluatedValue::NonConstant,
            };
            result.map_or(EvaluatedValue::NonConstant, EvaluatedValue::Integer)
        }
        (left, right) => {
            fn as_f64(value: EvaluatedValue) -> Option<f64> {
                match value {
                    EvaluatedValue::Integer(value) => Some(value as f64),
                    EvaluatedValue::Real(value) => Some(value),
                    _ => None,
                }
            }
            let (Some(left), Some(right)) = (as_f64(left), as_f64(right)) else {
                return EvaluatedValue::NonConstant;
            };
            match op {
                BinaryOperator::Add => EvaluatedValue::Real(left + right),
                BinaryOperator::Sub => EvaluatedValue::Real(left - right),
                BinaryOperator::Mul => EvaluatedValue::Real(left * right),
                BinaryOperator::Div => {
                    if right == 0.0 {
                        EvaluatedValue::DivisionByZero
                    } else {
                        EvaluatedValue::Real(left / right)
                    }
                }
                BinaryOperator::Mod => {
                    if right == 0.0 {
                        EvaluatedValue::DivisionByZero
                    } else {
                        EvaluatedValue::Real(left % right)
                    }
                }
                BinaryOperator::Pow | BinaryOperator::Exp => EvaluatedValue::Real(left.powf(right)),
                _ => EvaluatedValue::NonConstant,
            }
        }
    }
}

/// Folds an `and`/`or` combination of two already-folded operands, mirroring
/// `fold_literal_comparison`'s priority order (`NonConverged` > `UnresolvedOperand` >
/// `NonConstant` > a genuine value): only a `Boolean`/`Boolean` pairing produces a result;
/// anything else (e.g. an `Integer` operand, which the grammar should never actually produce here
/// since a logical combinator's operands are themselves boolean comparisons) is conservatively
/// `NonConstant`, the same defensive fallback `fold_literal_comparison` uses for a mistyped
/// pairing.
pub(crate) fn fold_logical(
    op: BinaryOperator,
    left: EvaluatedValue,
    right: EvaluatedValue,
) -> EvaluatedValue {
    if let Some(outcome) = absorbing_outcome(&left, &right) {
        return outcome;
    }
    match (left, right) {
        (EvaluatedValue::Boolean(left), EvaluatedValue::Boolean(right)) => {
            EvaluatedValue::Boolean(match op {
                // `BitAnd`/`BitOr` are KerML's single-`&`/single-`|` spellings of boolean
                // conjunction/disjunction in a constraint/invariant boolean expression (see
                // `is_logical_operator`'s doc comment) -- not bitwise operators here, so they fold
                // identically to `And`/`Or`.
                BinaryOperator::And | BinaryOperator::BitAnd => left && right,
                BinaryOperator::Or | BinaryOperator::BitOr => left || right,
                // `Xor`/`Implies` share `And`/`Or`'s Boolean/Boolean truth-table shape exactly --
                // no new failure state, just a different two-operand boolean combination.
                BinaryOperator::Xor => left != right,
                BinaryOperator::Implies => !left || right,
                _ => return EvaluatedValue::NonConstant,
            })
        }
        _ => EvaluatedValue::NonConstant,
    }
}

/// Folds a unary `-`/`not` operation on an already-folded operand. `Minus` negates a constant
/// `Integer`/`Real` (`Integer` negation uses `checked_neg`, mirroring `fold_arithmetic`'s
/// conservative "cannot fold" `NonConstant` posture for `i64::MIN`'s unrepresentable negation rather
/// than panicking or silently wrapping); `Not` negates a constant `Boolean`. Any other operator/
/// operand-type pairing (unreachable via `classify_constraint_node`/`classify_calc_node`, which only
/// ever build this node for `Minus`/`Not`) conservatively falls to `NonConstant`, mirroring
/// `fold_literal_comparison`/`fold_arithmetic`/`fold_logical`'s own defensive fallback for a
/// mistyped pairing. `NonConverged`/`UnresolvedOperand`/`NonConstant` operands propagate through
/// unchanged, in the same priority order the binary folds already use.
pub(crate) fn fold_unary(op: &UnaryOperator, value: EvaluatedValue) -> EvaluatedValue {
    match value {
        EvaluatedValue::Quantity(magnitude, unit) if matches!(op, UnaryOperator::Minus) => {
            quantity_result(fold_unary(op, *magnitude), unit)
        }
        EvaluatedValue::DivisionByZero => EvaluatedValue::DivisionByZero,
        EvaluatedValue::TypeMismatch => EvaluatedValue::TypeMismatch,
        EvaluatedValue::UnsupportedUnitOperation => EvaluatedValue::UnsupportedUnitOperation,
        EvaluatedValue::NonConverged => EvaluatedValue::NonConverged,
        EvaluatedValue::UnresolvedOperand => EvaluatedValue::UnresolvedOperand,
        EvaluatedValue::NonConstant => EvaluatedValue::NonConstant,
        EvaluatedValue::Integer(value) => match op {
            UnaryOperator::Minus => value
                .checked_neg()
                .map_or(EvaluatedValue::NonConstant, EvaluatedValue::Integer),
            _ => EvaluatedValue::NonConstant,
        },
        EvaluatedValue::Real(value) => match op {
            UnaryOperator::Minus => EvaluatedValue::Real(-value),
            _ => EvaluatedValue::NonConstant,
        },
        EvaluatedValue::Boolean(value) => match op {
            UnaryOperator::Not => EvaluatedValue::Boolean(!value),
            _ => EvaluatedValue::NonConstant,
        },
        _ => EvaluatedValue::NonConstant,
    }
}

/// Keep failures outside a quantity value; only a numeric magnitude can carry a unit.
fn quantity_result(magnitude: EvaluatedValue, unit: String) -> EvaluatedValue {
    match magnitude {
        EvaluatedValue::Integer(_) | EvaluatedValue::Real(_) => {
            EvaluatedValue::Quantity(Box::new(magnitude), unit)
        }
        other => other,
    }
}

pub(crate) fn literal_expression_value(
    parsed: &ParsedDocument,
    node: &Expression,
) -> Option<EvaluatedValue> {
    match node {
        Expression::LiteralBoolean(value) => Some(EvaluatedValue::Boolean(*value)),
        Expression::LiteralInteger(value) => Some(EvaluatedValue::Integer(*value)),
        Expression::LiteralReal(literal) => parsed
            .real_literal(*literal)?
            .parse::<f64>()
            .ok()
            .map(EvaluatedValue::Real),
        Expression::LiteralString(literal) => Some(EvaluatedValue::String(
            parsed.decoded_string_literal(*literal)?.into_owned(),
        )),
        Expression::Bracket { base, operands, .. } => {
            let magnitude = literal_expression_value(parsed, &base.value)?;
            let unit_text = quantity_unit_text(parsed, &operands.value)?;
            Some(EvaluatedValue::Quantity(Box::new(magnitude), unit_text))
        }
        _ => None,
    }
}

/// The unit identity authored inside `[...]` for a `value [unit]` quantity literal.
///
/// The pinned parser models `27316[K]` as `Expression::Bracket`, whose operands are ordinary
/// typed expressions rather than a copied unit string: a unit-looking `SI::mm` stays a
/// source-backed qualified reference. This reads that reference's decoded segments, so the unit
/// spelling comes from the arena rather than from re-serializing source text. A compound unit
/// retains its authored expression text, without claiming a resolved unit identity. Multiple
/// bracket operands are outside the quantity-literal shape and return `None`.
pub(crate) fn quantity_unit_text(
    parsed: &ParsedDocument,
    operands: &SequenceExpressionList,
) -> Option<String> {
    let [element] = operands.elements.as_slice() else {
        return None;
    };
    let (Expression::FeatureRef(target) | Expression::FeatureChainRef(target)) =
        &element.expression.value
    else {
        fn unit_expression(expression: &Expression) -> bool {
            match expression {
                Expression::FeatureRef(_) | Expression::FeatureChainRef(_) => true,
                Expression::BinaryOp {
                    op: BinaryOperator::Mul | BinaryOperator::Div,
                    left,
                    right,
                } => unit_expression(&left.value) && unit_expression(&right.value),
                Expression::BinaryOp {
                    op: BinaryOperator::Pow | BinaryOperator::Exp,
                    left,
                    right,
                } => {
                    unit_expression(&left.value)
                        && matches!(right.value, Expression::LiteralInteger(_))
                }
                Expression::Sequence { operands, .. } => {
                    matches!(operands.value.elements.as_slice(), [only] if unit_expression(&only.expression.value))
                }
                _ => false,
            }
        }
        if !unit_expression(&element.expression.value) {
            return None;
        }
        // This is authored unit syntax, not a resolved unit identity. The unit catalog keeps
        // compound expressions explicitly unsupported while the literal retains its magnitude.
        return parsed
            .source
            .slice(&element.expression.span)
            .map(str::to_owned);
    };
    let reference = parsed.qualified_reference(*target)?;
    let mut text = String::new();
    for index in 0..reference.segments.len() {
        if index > 0 {
            text.push_str("::");
        }
        text.push_str(reference.segment_decoded_text(index)?.as_ref());
    }
    Some(text)
}

/// Whether an `EvalNode` tree contains no `Operand` leaf at all (i.e. is a pure literal, needing
/// no resolved state whatsoever to fold).
pub(crate) fn eval_node_is_pure_literal(node: &EvalNode) -> bool {
    match node {
        EvalNode::Literal(_) => true,
        EvalNode::Operand(_) => false,
        EvalNode::Comparison(_, left, right)
        | EvalNode::Arithmetic(_, left, right)
        | EvalNode::Logical(_, left, right) => {
            eval_node_is_pure_literal(left) && eval_node_is_pure_literal(right)
        }
        EvalNode::Unary(_, operand) => eval_node_is_pure_literal(operand),
        // Always `NonConstant` once folded (see `EvalNode::Invocation`'s doc comment), never a
        // "genuinely known" literal, regardless of how many/which literal arguments it carries.
        EvalNode::Invocation(_) | EvalNode::Sequence(_) | EvalNode::Call(..) => false,
    }
}

/// Folds an `EvalNode` tree to a concrete `EvaluatedValue`, resolving each `Operand(n)` leaf via
/// `resolve_operand`. Used both for construction-time pure-literal folding (an empty/unreachable
/// resolver) and for `compute_evaluation`'s resolution-time constant-propagation fold (slice 3).
pub(crate) fn fold_eval_node(
    node: &EvalNode,
    resolve_operand: &mut impl FnMut(EvalOperand) -> EvaluatedValue,
) -> EvaluatedValue {
    fold_eval_node_pending(node, &mut |ordinal| Some(resolve_operand(ordinal)))
        .expect("resolve_operand never returns None")
}

/// Same fold as `fold_eval_node`, but `resolve_operand` may report an operand as not-yet-settled
/// (`None`) -- used by `compute_evaluation`'s bounded constant-propagation fixed point (slice 3):
/// a `None` anywhere in the tree means the whole expression cannot be folded *this pass*, without
/// asserting anything about its eventual outcome (unlike a settled `EvaluatedValue`, which is
/// final once produced).
pub(crate) fn fold_eval_node_pending(
    node: &EvalNode,
    resolve_operand: &mut impl FnMut(EvalOperand) -> Option<EvaluatedValue>,
) -> Option<EvaluatedValue> {
    match node {
        EvalNode::Literal(value) => Some(value.clone()),
        EvalNode::Operand(ordinal) => resolve_operand(*ordinal),
        EvalNode::Comparison(op, left, right) => {
            let left = fold_eval_node_pending(left, resolve_operand)?;
            let right = fold_eval_node_pending(right, resolve_operand)?;
            Some(match (left, right) {
                (EvaluatedValue::NonConverged, _) | (_, EvaluatedValue::NonConverged) => {
                    EvaluatedValue::NonConverged
                }
                (EvaluatedValue::UnresolvedOperand, _) | (_, EvaluatedValue::UnresolvedOperand) => {
                    EvaluatedValue::UnresolvedOperand
                }
                (EvaluatedValue::NonConstant, _) | (_, EvaluatedValue::NonConstant) => {
                    EvaluatedValue::NonConstant
                }
                (left, right) => fold_literal_comparison(op.clone(), left, right),
            })
        }
        EvalNode::Arithmetic(op, left, right) => {
            let left = fold_eval_node_pending(left, resolve_operand)?;
            let right = fold_eval_node_pending(right, resolve_operand)?;
            Some(fold_arithmetic(op.clone(), left, right))
        }
        EvalNode::Logical(op, left, right) => {
            let left = fold_eval_node_pending(left, resolve_operand)?;
            let right = fold_eval_node_pending(right, resolve_operand)?;
            Some(fold_logical(op.clone(), left, right))
        }
        EvalNode::Invocation(_) | EvalNode::Sequence(_) => Some(EvaluatedValue::NonConstant),
        EvalNode::Call(target, arguments) => {
            let function = match target {
                CallTarget::Numerical(function) => *function,
                CallTarget::Unresolved => return Some(EvaluatedValue::UnresolvedOperand),
                CallTarget::Unsupported | CallTarget::AuthoredEnd(_) => {
                    return Some(EvaluatedValue::NonConstant)
                }
            };
            fn collect(
                argument: &EvalNode,
                flatten: bool,
                resolver: &mut impl FnMut(EvalOperand) -> Option<EvaluatedValue>,
                values: &mut Vec<EvaluatedValue>,
            ) -> Option<()> {
                if let EvalNode::Sequence(elements) = argument {
                    if flatten {
                        for element in elements {
                            collect(element, true, resolver, values)?;
                        }
                        return Some(());
                    }
                }
                values.push(fold_eval_node_pending(argument, resolver)?);
                Some(())
            }
            let mut values = Vec::new();
            for argument in arguments {
                collect(
                    argument,
                    matches!(function, NumericalFunction::Sum),
                    resolve_operand,
                    &mut values,
                )?;
            }
            Some(fold_numerical(function, values))
        }
        EvalNode::Unary(op, operand) => {
            let operand = fold_eval_node_pending(operand, resolve_operand)?;
            Some(fold_unary(op, operand))
        }
    }
}

fn fold_numerical(function: NumericalFunction, values: Vec<EvaluatedValue>) -> EvaluatedValue {
    fn numeric(value: &EvaluatedValue) -> bool {
        match value {
            EvaluatedValue::Integer(_) | EvaluatedValue::Real(_) => true,
            EvaluatedValue::Quantity(magnitude, _) => matches!(
                magnitude.as_ref(),
                EvaluatedValue::Integer(_) | EvaluatedValue::Real(_)
            ),
            _ => false,
        }
    }
    let mut failure = None;
    for value in &values {
        if let Some(outcome) = absorbing_outcome(
            value,
            failure.as_ref().unwrap_or(&EvaluatedValue::Integer(0)),
        ) {
            failure = Some(outcome);
        }
    }
    if let Some(failure) = failure {
        return failure;
    }
    if values.iter().any(|value| !numeric(value)) {
        return EvaluatedValue::TypeMismatch;
    }
    if !matches!(function, NumericalFunction::Sum) && values.len() != 2 {
        return EvaluatedValue::TypeMismatch;
    }
    let mut values = values.into_iter();
    let Some(mut result) = values.next() else {
        return EvaluatedValue::NonConstant;
    };
    for value in values {
        result = match function {
            NumericalFunction::Sum => fold_arithmetic(BinaryOperator::Add, result, value),
            NumericalFunction::Max | NumericalFunction::Min => {
                if let Some(outcome) = absorbing_outcome(&result, &value) {
                    outcome
                } else {
                    let comparison = match (&result, &value) {
                        (
                            EvaluatedValue::Quantity(left, left_unit),
                            EvaluatedValue::Quantity(right, right_unit),
                        ) if left_unit == right_unit => fold_literal_comparison(
                            BinaryOperator::Ge,
                            *left.clone(),
                            *right.clone(),
                        ),
                        (EvaluatedValue::Quantity(..), _) | (_, EvaluatedValue::Quantity(..)) => {
                            EvaluatedValue::UnsupportedUnitOperation
                        }
                        _ => fold_literal_comparison(
                            BinaryOperator::Ge,
                            result.clone(),
                            value.clone(),
                        ),
                    };
                    match comparison {
                        EvaluatedValue::Boolean(left_is_greater) => {
                            if left_is_greater == matches!(function, NumericalFunction::Max) {
                                result
                            } else {
                                value
                            }
                        }
                        other => other,
                    }
                }
            }
        };
    }
    result
}
