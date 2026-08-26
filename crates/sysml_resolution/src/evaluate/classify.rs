//! Phase 5: classifying an authored expression into the shape evaluation folds.

use sysml_v2_parser::{
    ast::{BinaryOperator, Expression, Node, QualifiedReferenceId, UnaryOperator},
    ParsedDocument,
};

use crate::evaluate::fold::{
    eval_node_is_pure_literal, fold_eval_node, literal_expression_value, EvalNode,
};
use crate::lower::facts::{AuthoredExpression, ExpressionGrammar, FilterPredicate};
use crate::lower::storage::ParsedSources;
use crate::model::EvaluatedValue;

/// The classification `classify_expression` assign to one
/// expression node before resolution's fixed point runs. `Literal` expressions need no resolved
/// state at all (their value is already known); `HasOperand` expressions carry an `EvalNode` tree
/// that `compute_evaluation` re-folds once operand references are resolved (and, per slice 3,
/// once each operand's own target declaration's constant value -- if any -- is known); the tree
/// settles to `UnresolvedOperand`/`NonConstant`/`NonConverged` or a genuine folded constant.
/// `Unsupported` expressions (any shape `lower_constraint_expression`/`lower_calc_expression` does
/// not recognize) publish no evaluation fact at all.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExpressionEvalShape {
    /// The authored expression is itself a literal value -- nothing was folded.
    Literal(EvaluatedValue),
    /// A tree with no operand leaf, folded at construction (`2 + 3`). Kept apart from `Literal`
    /// because the published contract distinguishes a value that was *written* from one that was
    /// *computed*, and only the root node tells them apart.
    ConstantFolded(EvaluatedValue),
    HasOperand(EvalNode),
    Unsupported,
}

/// Recursively builds the `EvalNode` mirror for a constraint-body expression, threading an
/// operand-ordinal counter that increments exactly where `lower_constraint_expression` would push
/// an `ExpressionOperand` reference, so the two traversals stay index-aligned. Returns `None` for
/// any shape `lower_constraint_expression` does not recognize (`Unsupported`).
pub(crate) fn classify_constraint_node(
    parsed: &ParsedDocument,
    node: &Expression,
    ordinal: &mut u32,
) -> Option<EvalNode> {
    match node {
        Expression::LiteralInteger(_)
        | Expression::LiteralReal(_)
        | Expression::LiteralBoolean(_)
        | Expression::LiteralString(_)
        | Expression::Bracket { .. } => {
            literal_expression_value(parsed, node).map(EvalNode::Literal)
        }
        Expression::FeatureRef(_) | Expression::FeatureChainRef(_) => {
            let leaf = EvalNode::Operand(*ordinal);
            *ordinal += 1;
            Some(leaf)
        }
        Expression::Sequence { operands, .. } => {
            // A singleton sequence is the grouping spelling the old `Parenthesized` variant
            // carried; a multi-element one is the tuple spelling. Both are one production now.
            let elements = &operands.value.elements;
            if let [only] = elements.as_slice() {
                return classify_constraint_node(parsed, &only.expression.value, ordinal);
            }
            let mut children = Vec::with_capacity(elements.len());
            for element in elements {
                children.push(classify_constraint_node(
                    parsed,
                    &element.expression.value,
                    ordinal,
                )?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Index { base, operands, .. } => {
            let mut children = Vec::with_capacity(operands.value.elements.len() + 1);
            children.push(classify_constraint_node(parsed, &base.value, ordinal)?);
            for element in &operands.value.elements {
                children.push(classify_constraint_node(
                    parsed,
                    &element.expression.value,
                    ordinal,
                )?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Select { base, .. } => {
            let base = classify_constraint_node(parsed, &base.value, ordinal)?;
            let selector = EvalNode::Operand(*ordinal);
            *ordinal += 1;
            Some(EvalNode::Invocation(vec![base, selector]))
        }
        Expression::BinaryOp { op, left, right } if is_comparison_operator(op) => {
            let left = classify_constraint_node(parsed, &left.value, ordinal)?;
            let right = classify_constraint_node(parsed, &right.value, ordinal)?;
            Some(EvalNode::Comparison(
                op.clone(),
                Box::new(left),
                Box::new(right),
            ))
        }
        Expression::BinaryOp { op, left, right } if is_arithmetic_operator(op) => {
            let left = classify_constraint_node(parsed, &left.value, ordinal)?;
            let right = classify_constraint_node(parsed, &right.value, ordinal)?;
            Some(EvalNode::Arithmetic(
                op.clone(),
                Box::new(left),
                Box::new(right),
            ))
        }
        Expression::BinaryOp { op, left, right } if is_logical_operator(op) => {
            let left = classify_constraint_node(parsed, &left.value, ordinal)?;
            let right = classify_constraint_node(parsed, &right.value, ordinal)?;
            Some(EvalNode::Logical(
                op.clone(),
                Box::new(left),
                Box::new(right),
            ))
        }
        Expression::Invocation { args, .. } => {
            let mut children = Vec::with_capacity(args.len());
            for arg in args {
                children.push(classify_constraint_node(parsed, &arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Constructor { args, .. } => {
            let mut children = Vec::with_capacity(args.len());
            for arg in args {
                children.push(classify_constraint_node(parsed, &arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::CollectionOp { base, args, .. } => {
            let mut children = Vec::with_capacity(args.len() + 1);
            children.push(classify_constraint_node(parsed, &base.value, ordinal)?);
            for arg in args {
                children.push(classify_constraint_node(parsed, &arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
            let operand = classify_constraint_node(parsed, &operand.value, ordinal)?;
            Some(EvalNode::Unary(op.clone(), Box::new(operand)))
        }
        Expression::TypeCheck { operand, .. } => {
            let mut children = Vec::with_capacity(1);
            if let Some(operand) = operand {
                children.push(classify_constraint_node(parsed, &operand.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::MetaCast { base, .. } => {
            let base = classify_constraint_node(parsed, &base.value, ordinal)?;
            Some(EvalNode::Invocation(vec![base]))
        }
        _ => None,
    }
}

pub(crate) fn classify_filter_predicate(
    node: &Expression,
    metadata_ordinal: &mut u32,
) -> FilterPredicate {
    match node {
        Expression::LiteralBoolean(value) => FilterPredicate::Boolean(*value),
        Expression::Classification { .. } => {
            let ordinal = *metadata_ordinal;
            *metadata_ordinal = metadata_ordinal.saturating_add(1);
            FilterPredicate::Metadata(ordinal)
        }
        Expression::Sequence { operands, .. }
            if matches!(operands.value.elements.as_slice(), [_]) =>
        {
            classify_filter_predicate(
                &operands.value.elements[0].expression.value,
                metadata_ordinal,
            )
        }
        Expression::BinaryOp {
            op: BinaryOperator::And | BinaryOperator::BitAnd,
            left,
            right,
        } => FilterPredicate::And(
            Box::new(classify_filter_predicate(&left.value, metadata_ordinal)),
            Box::new(classify_filter_predicate(&right.value, metadata_ordinal)),
        ),
        Expression::BinaryOp {
            op: BinaryOperator::Or | BinaryOperator::BitOr,
            left,
            right,
        } => FilterPredicate::Or(
            Box::new(classify_filter_predicate(&left.value, metadata_ordinal)),
            Box::new(classify_filter_predicate(&right.value, metadata_ordinal)),
        ),
        _ => FilterPredicate::Unsupported,
    }
}

/// Recursively builds the `EvalNode` mirror for a calc-body expression, mirroring
/// `lower_calc_expression`'s supported shapes: no comparison-operator support (calc bodies stay
/// comparison-free, per slice 1), plus slice 4's arithmetic `BinaryOp` (`Add`/`Sub`/`Mul`/`Div`/
/// `Mod`) support.
pub(crate) fn classify_calc_node(
    parsed: &ParsedDocument,
    node: &Expression,
    ordinal: &mut u32,
) -> Option<EvalNode> {
    match node {
        Expression::LiteralInteger(_)
        | Expression::LiteralReal(_)
        | Expression::LiteralBoolean(_)
        | Expression::LiteralString(_)
        | Expression::Bracket { .. } => {
            literal_expression_value(parsed, node).map(EvalNode::Literal)
        }
        Expression::FeatureRef(_) | Expression::FeatureChainRef(_) => {
            let leaf = EvalNode::Operand(*ordinal);
            *ordinal += 1;
            Some(leaf)
        }
        Expression::Sequence { operands, .. } => {
            // A singleton sequence is the grouping spelling the old `Parenthesized` variant
            // carried; a multi-element one is the tuple spelling. Both are one production now.
            let elements = &operands.value.elements;
            if let [only] = elements.as_slice() {
                return classify_calc_node(parsed, &only.expression.value, ordinal);
            }
            let mut children = Vec::with_capacity(elements.len());
            for element in elements {
                children.push(classify_calc_node(
                    parsed,
                    &element.expression.value,
                    ordinal,
                )?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Index { base, operands, .. } => {
            let mut children = Vec::with_capacity(operands.value.elements.len() + 1);
            children.push(classify_calc_node(parsed, &base.value, ordinal)?);
            for element in &operands.value.elements {
                children.push(classify_calc_node(
                    parsed,
                    &element.expression.value,
                    ordinal,
                )?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Select { base, .. } => {
            let base = classify_calc_node(parsed, &base.value, ordinal)?;
            let selector = EvalNode::Operand(*ordinal);
            *ordinal += 1;
            Some(EvalNode::Invocation(vec![base, selector]))
        }
        Expression::BinaryOp { op, left, right } if is_arithmetic_operator(op) => {
            let left = classify_calc_node(parsed, &left.value, ordinal)?;
            let right = classify_calc_node(parsed, &right.value, ordinal)?;
            Some(EvalNode::Arithmetic(
                op.clone(),
                Box::new(left),
                Box::new(right),
            ))
        }
        Expression::Invocation { args, .. } => {
            let mut children = Vec::with_capacity(args.len());
            for arg in args {
                children.push(classify_calc_node(parsed, &arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Constructor { args, .. } => {
            let mut children = Vec::with_capacity(args.len());
            for arg in args {
                children.push(classify_calc_node(parsed, &arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::CollectionOp { base, args, .. } => {
            let mut children = Vec::with_capacity(args.len() + 1);
            children.push(classify_calc_node(parsed, &base.value, ordinal)?);
            for arg in args {
                children.push(classify_calc_node(parsed, &arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
            let operand = classify_calc_node(parsed, &operand.value, ordinal)?;
            Some(EvalNode::Unary(op.clone(), Box::new(operand)))
        }
        Expression::TypeCheck { operand, .. } => {
            let mut children = Vec::with_capacity(1);
            if let Some(operand) = operand {
                children.push(classify_calc_node(parsed, &operand.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::MetaCast { base, .. } => {
            let base = classify_calc_node(parsed, &base.value, ordinal)?;
            Some(EvalNode::Invocation(vec![base]))
        }
        _ => None,
    }
}

/// Classifies one authored expression against the document it was written in.
///
/// The single entry point phase 5 uses. Lowering records the *site* -- document, grammar, operand
/// start, expression -- and nothing more; deciding what that expression means is evaluation's
/// alone, and this is where it happens.
///
/// A site whose document is not in the publication classifies as `Unsupported` rather than
/// panicking. Lowering only ever records a document it just admitted, so this is unreachable by
/// construction; an absent arena still cannot be read, and `Unsupported` is the honest answer for
/// an expression whose value this engine cannot determine.
pub(crate) fn classify_authored(
    sources: &ParsedSources,
    expression: &AuthoredExpression,
) -> ExpressionEvalShape {
    let Some(parsed) = sources.parsed(expression.document) else {
        return ExpressionEvalShape::Unsupported;
    };
    classify_expression(
        parsed,
        &expression.node,
        expression.grammar,
        expression.operand_start,
    )
}

/// Classifies an expression exactly along `lower_constraint_expression`/`lower_calc_expression`'s
/// supported-shape boundary, without pushing any reference or diagnostic (a pure, side-effect-free
/// mirror used only to decide whether/how to publish an evaluation fact). See `EvaluatedValue`.
///
/// `start` is the ordinal the expression's first operand reference was lowered under. A declaration
/// usually owns one expression, whose operand ordinals start at zero. A view owning two `filter`
/// statements is the exception: both conditions are lowered against the view, so the second one's
/// operand references are numbered after the first one's, and classifying it from zero would pair
/// every leaf with the wrong reference.
pub(crate) fn classify_expression(
    parsed: &ParsedDocument,
    node: &Expression,
    grammar: ExpressionGrammar,
    start: u32,
) -> ExpressionEvalShape {
    let mut ordinal = start;
    let classified = match grammar {
        ExpressionGrammar::Constraint => classify_constraint_node(parsed, node, &mut ordinal),
        ExpressionGrammar::Calc => classify_calc_node(parsed, node, &mut ordinal),
    };
    match classified {
        None => ExpressionEvalShape::Unsupported,
        Some(tree) if eval_node_is_pure_literal(&tree) => {
            let value = fold_eval_node(&tree, &mut |_| {
                unreachable!("eval_node_is_pure_literal guarantees no Operand leaf is folded")
            });
            if matches!(tree, EvalNode::Literal(_)) {
                ExpressionEvalShape::Literal(value)
            } else {
                ExpressionEvalShape::ConstantFolded(value)
            }
        }
        Some(tree) => ExpressionEvalShape::HasOperand(tree),
    }
}

/// Whether a `BinaryOperator` is one of the eight boolean comparison operators
/// (`lower_constraint_expression`'s supported `BinaryOp` shape): `==`, `!=`, `<`, `<=`, `>`, `>=`,
/// and KerML's strict-identity `===`/`!==` (`StrictEq`/`StrictNe`). The latter two were originally
/// deliberately excluded from this predicate, but real-corpus evidence (exhaustive
/// `unsupported_calc_definition_member` audit, e.g. Kernel Function Library `BaseFunctions.kerml`'s
/// `function '!=='{ ... return : Boolean[1] = not (x === y); }`) shows they appear alongside the
/// other six comparisons in ordinary reference-resolution contexts identically -- this pipeline only
/// ever recurses into a comparison's operands to resolve references, it does not evaluate strict vs.
/// non-strict identity semantics differently, so there is no reason to keep them unsupported.
pub(crate) fn is_comparison_operator(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Eq
            | BinaryOperator::Ne
            | BinaryOperator::StrictEq
            | BinaryOperator::StrictNe
            | BinaryOperator::Lt
            | BinaryOperator::Le
            | BinaryOperator::Gt
            | BinaryOperator::Ge
    )
}

/// Whether a `BinaryOperator` is one of the boolean-combination operators
/// `classify_constraint_node`/`lower_constraint_expression`'s logical `BinaryOp` shape supports:
/// `and`, `or`, `xor`, `implies`, KerML's single-ampersand `&` spelling of conjunction
/// (`BinaryOperator::BitAnd`, `BinaryOperator::from_token("&")`), and its single-pipe `|` spelling
/// of disjunction (`BinaryOperator::BitOr`, `BinaryOperator::from_token("|")`). Per the KerML
/// textual notation (§8.2.7 invariants), `&`/`|` are the ordinary boolean-and/-or connectives in a
/// constraint/invariant boolean expression -- e.g. `sysml.library/trig_functions.md`'s `-1.0 <=
/// that & that <= 1.0` and `sysml.library/state_performances.md`'s `accT == accableT &
/// incomingTransferSort(...)` for `&`, and `sysml.library/state_performances.md`'s `accableT ==
/// accT | incomingTransferSort(accT, accableT)` for `|` -- not a bitwise/set operator; the parser's
/// own `BinaryOperator::from_token` has no distinct "boolean and"/"boolean or" token for `&`/`|`,
/// only the shared `BitAnd`/`BitOr` classification, so this predicate (and `fold_logical`, which
/// treats them identically to `And`/`Or`) is where that context-dependent meaning is recovered.
/// `xor`/`implies` share `and`/`or`'s exact Boolean/Boolean two-operand truth-table shape
/// (`fold_logical`), so widening this predicate is the whole of their support -- no new `EvalNode`
/// variant or failure state needed.
pub(crate) fn is_logical_operator(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::And
            | BinaryOperator::Or
            | BinaryOperator::Xor
            | BinaryOperator::Implies
            | BinaryOperator::BitAnd
            | BinaryOperator::BitOr
    )
}

/// Whether a `UnaryOperator` is one of the two unary operators `classify_constraint_node`/
/// `classify_calc_node`/`lower_constraint_expression`/`lower_calc_expression` support: `-` (negation,
/// `fold_unary`) and `not` (logical negation). `+` (`Plus`, a structural no-op the grammar rarely if
/// ever needs folded) and `~` (`BitNot`, no corresponding `EvaluatedValue` bit type) are deliberately
/// out of scope, mirroring `is_arithmetic_operator`'s precedent of excluding operators that would not
/// make any additional real-corpus expression foldable.
pub(crate) fn is_unary_operator(op: &UnaryOperator) -> bool {
    matches!(op, UnaryOperator::Minus | UnaryOperator::Not)
}

/// Whether a `BinaryOperator` is one of the arithmetic operators
/// (`lower_calc_expression`'s supported `BinaryOp` shape): `+`, `-`, `*`, `/`, `%`, and the two
/// exponentiation spellings `^`/`Pow` and `**`/`Exp`. Slice 4 (`bd50fccd`) originally excluded
/// `Exp`/`Pow`, reasoning every real-corpus `**` occurrence combined it with unary negation or
/// fractional/negative exponents -- shapes outside that slice's scope regardless. Unary negation
/// landed separately (`438b8572`), and fractional/negative exponents on a `Real` base are just
/// `f64::powf`, already covered by this function's own `Real`-promotion path (see
/// `fold_arithmetic`), so neither reason still blocks folding the operator itself: e.g.
/// `10c_fuel_economy_analysis.md`'s `231.0 * 'in'^3` now folds one level further into the `Pow`.
pub(crate) fn is_arithmetic_operator(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Add
            | BinaryOperator::Sub
            | BinaryOperator::Mul
            | BinaryOperator::Div
            | BinaryOperator::Mod
            | BinaryOperator::Pow
            | BinaryOperator::Exp
    )
}

/// Whether a `BinaryOperator` is the KerML range-construction operator `..` (`Range`, e.g.
/// `(1..size(x))->forAll {...}`, Kernel Function Library `SequenceFunctions.kerml`) or the
/// null-coalescing operator `??` (`NullCoalesce`, e.g. `collection->reduce '+' ?? zero`, Kernel
/// Function Library `DataFunctions.kerml`/`NumericalFunctions.kerml`). Neither is arithmetic,
/// comparison, or logical in the sense `is_arithmetic_operator`/`is_comparison_operator`/
/// `is_logical_operator` model, but both are ordinary two-operand expression shapes for this
/// pipeline's reference-resolution-only purposes (no evaluation folding is attempted for either),
/// so they share this narrow, purpose-specific predicate rather than being folded into one of the
/// other three.
pub(crate) fn is_range_or_coalesce_operator(op: &BinaryOperator) -> bool {
    matches!(op, BinaryOperator::Range | BinaryOperator::NullCoalesce)
}

/// Flattens a dotted `Expression::MemberAccess` chain (`a.b.c`, parsed as nested
/// `MemberAccess(MemberAccess(FeatureRef(a), b), c)`) into its ordered list of qualified-reference
/// segments: the innermost `FeatureRef`/`FeatureChainRef`'s own (possibly multi-segment) path,
/// followed by each subsequent `member` segment outward. Returns `None` when the chain's root is
/// anything other than a `FeatureRef`/`FeatureChainRef` (an index expression, invocation, literal,
/// etc.), since this pipeline has no lexical-lookup starting point for those shapes -- the caller
/// falls through to its existing unsupported-member diagnostic in that case. A bare
/// `FeatureRef`/`FeatureChainRef` (no `MemberAccess` wrapper at all) flattens to a single-entry
/// list, letting callers route both shapes through the same chain-resolution path uniformly.
///
/// `Expression::Parenthesized` and `Expression::TypeCheck` (the `as`/`istype`/`hastype` cast
/// family) are transparent wrappers for this purpose: `(vehicles as VehiclePart).m` (real corpus
/// usage, `sysml/examples/calculation_test.md`) is `MemberAccess(Parenthesized(TypeCheck{operand:
/// FeatureRef(vehicles), type_name: VehiclePart}), m)`, and its member `m` resolves relative to
/// `vehicles`' own lexical lookup exactly like an uncast `vehicles.m` would -- this pipeline does
/// not model the cast's type-narrowing effect on member lookup (no call site needs it yet), it
/// just stops treating the cast as an opaque root the way it previously stopped the whole chain
/// from resolving at all. A `TypeCheck` with no operand (bare `istype`/`hastype` with an implicit
/// subject) still returns `None`, matching Gap 41's `that`-self-reference boundary: an implicit
/// `that` operand is indistinguishable from a genuinely absent one without the same upstream
/// parser fix that gap already documents.
pub(crate) fn flatten_member_access_chain(
    node: &Node<Expression>,
) -> Option<Vec<QualifiedReferenceId>> {
    match &node.value {
        Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => Some(vec![*target]),
        Expression::MemberAccess { base, member, .. } => {
            let mut chain = flatten_member_access_chain(base)?;
            chain.push(*member);
            Some(chain)
        }
        Expression::Sequence { operands, .. } => match operands.value.elements.as_slice() {
            [only] => flatten_member_access_chain(&only.expression),
            _ => None,
        },
        Expression::TypeCheck {
            operand: Some(operand),
            ..
        } => flatten_member_access_chain(operand),
        _ => None,
    }
}
