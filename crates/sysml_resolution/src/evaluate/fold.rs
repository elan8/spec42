//! Phase 5: constant folding over the authored expression shapes.

use sysml_v2_parser::{
    ast::{BinaryOperator, Expression, SequenceExpressionList, UnaryOperator},
    ParsedDocument,
};

use crate::model::EvaluatedValue;

/// A construction-time-classified mirror of a supported constraint/calc expression tree, built by
/// `classify_constraint_expression`/`classify_calc_expression` in lockstep with
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
    Operand(u32),
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
    /// Also reused, unchanged, for `Expression::Tuple` (`(a, b, c)`, e.g.
    /// `quantityPowerFactors = (lengthPF, massPF, durationPF)`): a tuple has no callee to resolve,
    /// but is otherwise identical in shape -- a plain `Vec<Node<Expression>>` of elements, each
    /// recursed back into the same classifier/lowerer, with the tuple as a whole never evaluated as
    /// a single scalar (folds to `NonConstant`, like an invocation). Reusing this variant rather
    /// than adding a distinct `Tuple` one keeps the "one EvalNode shape per fold behavior" property:
    /// nothing downstream (`fold_eval_node_pending`, `eval_node_is_pure_literal`) needs to
    /// distinguish "called with args" from "grouped as a sequence" since both fold identically.
    Invocation(Vec<EvalNode>),
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
    fn as_f64(value: EvaluatedValue) -> Option<f64> {
        match value {
            EvaluatedValue::Integer(value) => Some(value as f64),
            EvaluatedValue::Real(value) => Some(value),
            _ => None,
        }
    }
    let result = match (left, right) {
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
    match (left, right) {
        (EvaluatedValue::NonConverged, _) | (_, EvaluatedValue::NonConverged) => {
            EvaluatedValue::NonConverged
        }
        (EvaluatedValue::UnresolvedOperand, _) | (_, EvaluatedValue::UnresolvedOperand) => {
            EvaluatedValue::UnresolvedOperand
        }
        (EvaluatedValue::NonConstant, _) | (_, EvaluatedValue::NonConstant) => {
            EvaluatedValue::NonConstant
        }
        (EvaluatedValue::Boolean(_), _) | (_, EvaluatedValue::Boolean(_)) => {
            EvaluatedValue::TypeMismatch
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
    match (left, right) {
        (EvaluatedValue::NonConverged, _) | (_, EvaluatedValue::NonConverged) => {
            EvaluatedValue::NonConverged
        }
        (EvaluatedValue::UnresolvedOperand, _) | (_, EvaluatedValue::UnresolvedOperand) => {
            EvaluatedValue::UnresolvedOperand
        }
        (EvaluatedValue::NonConstant, _) | (_, EvaluatedValue::NonConstant) => {
            EvaluatedValue::NonConstant
        }
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

pub(crate) fn literal_expression_value(
    parsed: &ParsedDocument,
    node: &Expression,
) -> Option<EvaluatedValue> {
    match node {
        Expression::LiteralBoolean(value) => Some(EvaluatedValue::Boolean(*value)),
        Expression::LiteralInteger(value) => Some(EvaluatedValue::Integer(*value)),
        Expression::LiteralReal(text) => text.parse::<f64>().ok().map(EvaluatedValue::Real),
        Expression::LiteralString(value) => Some(EvaluatedValue::String(value.clone())),
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
/// identity comes from the arena rather than from re-serializing source text. Any other operand
/// shape -- a computed unit such as `N * m`, or a multi-operand list -- returns `None`, so the
/// caller falls through to `NonConstant`/`Unsupported` exactly as before rather than inventing a
/// unit.
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
        return None;
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
        EvalNode::Invocation(_) => false,
    }
}

/// Folds an `EvalNode` tree to a concrete `EvaluatedValue`, resolving each `Operand(n)` leaf via
/// `resolve_operand`. Used both for construction-time pure-literal folding (an empty/unreachable
/// resolver) and for `compute_evaluation`'s resolution-time constant-propagation fold (slice 3).
pub(crate) fn fold_eval_node(
    node: &EvalNode,
    resolve_operand: &mut impl FnMut(u32) -> EvaluatedValue,
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
    resolve_operand: &mut impl FnMut(u32) -> Option<EvaluatedValue>,
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
        EvalNode::Invocation(_) => Some(EvaluatedValue::NonConstant),
        EvalNode::Unary(op, operand) => {
            let operand = fold_eval_node_pending(operand, resolve_operand)?;
            Some(fold_unary(op, operand))
        }
    }
}
