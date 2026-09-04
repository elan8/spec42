//! Phase 6: the resolved structure of every authored constraint / calc / value expression.
//!
//! [`crate::index::expressions::ExpressionIndex`] settles what an expression *evaluates to*; this
//! index settles what it *is* -- the same tree resolution already walks to fold constants, kept
//! this time instead of discarded.
//!
//! One flat node arena per declaration. The walk is gated by the canonical classifier
//! ([`classify_constraint_node`] / [`classify_calc_node`]): an expression those reject is
//! [`ExpressionOutcome::Unsupported`] and no tree is offered, so this module never widens the
//! supported-shape boundary the evaluation engine owns. Within an accepted expression the walk
//! threads the operand ordinal in lockstep with lowering, so `FeatureReference` node *n* names the
//! `n`-th [`ReferenceKind::ExpressionOperand`] reference the declaration authored -- the same
//! pairing `EvalNode::Operand` relies on.

use sysml_v2_parser::ast::{BinaryOperator, Expression, Node, Span, UnaryOperator};
use sysml_v2_parser::ParsedDocument;

use crate::evaluate::classify::{
    classify_calc_node, classify_constraint_node, is_arithmetic_operator, is_comparison_operator,
    is_logical_operator, is_unary_operator,
};
use crate::evaluate::fold::literal_expression_value;
use crate::expression::{
    ExpressionNode, ExpressionNodeKind, ExpressionOperator, ExpressionOutcome, PublishedExpression,
};
use crate::lower::facts::{AuthoredExpression, ExpressionGrammar, PendingEvaluationFact};
use crate::lower::storage::{ParsedSources, SemanticModelStorage};
use crate::model::evaluation::evaluated_scalar;
use crate::model::resolver::SemanticModel;
use crate::model::span::document_range;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DocumentIdx;
use crate::model::ReferenceKind;
use crate::model::SymbolPathId;
use crate::resolve::results::{ResolutionError, ResolutionResults, ResolutionStatus};
use crate::EvaluatedScalar;
use crate::OccurrenceRole;
use crate::QueryAnswer;
use crate::QueryOutcome;
use crate::SourceLocation;
use crate::SymbolId;
use crate::TextPosition;
use crate::TextRange;

/// One node of a settled tree, before its span and target are projected onto the published
/// contract. `span`/`document` become a `SourceLocation` and `target` a `SymbolId` at query time,
/// exactly as `SettledUnit` is projected onto `AuthoredUnit`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RawExpressionNode {
    pub(crate) kind: RawExpressionNodeKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RawExpressionNodeKind {
    Literal(EvaluatedScalar),
    FeatureReference {
        target: Option<DeclarationId>,
        authored: Box<str>,
    },
    Operator {
        operator: ExpressionOperator,
        operands: Box<[u32]>,
    },
    Unsupported {
        children: Box<[u32]>,
    },
}

/// The settled resolved-expression tree of one declaration.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResolvedExpressionRow {
    pub(crate) outcome: ExpressionOutcome,
    pub(crate) document: DocumentIdx,
    pub(crate) nodes: Box<[RawExpressionNode]>,
    pub(crate) root: Option<u32>,
}

impl ResolvedExpressionRow {
    fn unsupported(document: DocumentIdx) -> Self {
        Self {
            outcome: ExpressionOutcome::Unsupported,
            document,
            nodes: Box::default(),
            root: None,
        }
    }
}

/// Every declaration's resolved expression tree, indexed by declaration ordinal.
#[derive(Debug)]
pub(crate) struct ResolvedExpressionIndex {
    rows: Box<[Option<ResolvedExpressionRow>]>,
}

/// The settled facts this index reads, borrowed from the phase products that own them.
pub(crate) struct ResolvedExpressionInputs<'a> {
    pub(crate) storage: &'a SemanticModelStorage,
    pub(crate) sources: &'a ParsedSources,
    pub(crate) resolution: &'a ResolutionResults,
}

impl ResolvedExpressionIndex {
    /// Classifies every authored constraint / calc / value expression into its resolved tree.
    pub(crate) fn build(inputs: &ResolvedExpressionInputs<'_>) -> Result<Self, ResolutionError> {
        let count = inputs.storage.declarations.len();
        let mut rows: Vec<Option<ResolvedExpressionRow>> =
            std::iter::repeat_with(|| None).take(count).collect();

        // The `ExpressionOperand` references a declaration authored, in ordinal order. Slot `n`
        // pairs with the `n`-th `FeatureReference` the walk produces for that declaration.
        let mut operands: std::collections::BTreeMap<
            DeclarationId,
            Vec<(u32, AuthoredReferenceId)>,
        > = std::collections::BTreeMap::new();
        for (index, reference) in inputs.storage.references.iter().enumerate() {
            if reference.kind == ReferenceKind::ExpressionOperand {
                let id = AuthoredReferenceId::from_index(index)
                    .map_err(|_| ResolutionError::Capacity)?;
                operands
                    .entry(reference.source)
                    .or_default()
                    .push((reference.ordinal, id));
            }
        }
        let mut ordered: std::collections::BTreeMap<DeclarationId, Vec<AuthoredReferenceId>> =
            std::collections::BTreeMap::new();
        for (declaration, mut slots) in operands {
            slots.sort_by_key(|(ordinal, _)| *ordinal);
            ordered.insert(declaration, slots.into_iter().map(|(_, id)| id).collect());
        }
        let empty: Vec<AuthoredReferenceId> = Vec::new();

        // A declaration can author several boolean expressions in one constraint body; they are
        // conjoined into one tree, in authored (operand-ordinal) order.
        let mut by_declaration: std::collections::BTreeMap<
            DeclarationId,
            Vec<&PendingEvaluationFact>,
        > = std::collections::BTreeMap::new();
        for pending in inputs.storage.evaluation_facts.iter() {
            by_declaration
                .entry(pending.declaration)
                .or_default()
                .push(pending);
        }
        for (declaration, mut pendings) in by_declaration {
            pendings.sort_by_key(|pending| pending.expression.operand_start);
            let row = classify_declaration(
                inputs,
                &pendings,
                ordered.get(&declaration).unwrap_or(&empty),
            );
            if let Some(slot) = rows.get_mut(declaration.index()) {
                *slot = Some(row);
            }
        }

        Ok(Self {
            rows: rows.into_boxed_slice(),
        })
    }

    pub(crate) fn row(&self, declaration: DeclarationId) -> Option<&ResolvedExpressionRow> {
        self.rows
            .get(declaration.index())
            .and_then(|row| row.as_ref())
    }
}

fn classify_declaration(
    inputs: &ResolvedExpressionInputs<'_>,
    pendings: &[&PendingEvaluationFact],
    operands: &[AuthoredReferenceId],
) -> ResolvedExpressionRow {
    let document = pendings
        .first()
        .map_or(DocumentIdx(0), |pending| pending.expression.document);

    // The canonical accept boundary. If any authored expression on this declaration is a shape
    // the evaluation classifier rejects, the whole declaration reports `unsupported` -- a
    // conjunction with an unmodelled clause is not a tree this slice can hand back honestly.
    for pending in pendings {
        let Some(parsed) = inputs.sources.parsed(pending.expression.document) else {
            return ResolvedExpressionRow::unsupported(document);
        };
        if canonical_classification(parsed, &pending.expression).is_none() {
            return ResolvedExpressionRow::unsupported(document);
        }
    }

    let mut builder = TreeBuilder {
        storage: inputs.storage,
        resolution: inputs.resolution,
        parsed: None,
        operands,
        next_ordinal: 0,
        nodes: Vec::new(),
    };
    let mut roots = Vec::with_capacity(pendings.len());
    for pending in pendings {
        let Some(parsed) = inputs.sources.parsed(pending.expression.document) else {
            return ResolvedExpressionRow::unsupported(document);
        };
        builder.parsed = Some(parsed);
        match builder.walk(&pending.expression.node, Span::dummy()) {
            Some(root) => roots.push(root),
            None => return ResolvedExpressionRow::unsupported(document),
        }
    }

    // The walk mirrors the canonical classifier's operand traversal, so it must consume exactly
    // the operand references lowering pushed for this declaration. A mismatch would mispair every
    // `FeatureReference` after the drift point, so it degrades to "no tree" rather than a wrong
    // one -- and fails a debug build so a test catches it.
    debug_assert_eq!(
        builder.next_ordinal as usize,
        operands.len(),
        "resolved-expression walk consumed a different operand count than lowering pushed"
    );
    if builder.next_ordinal as usize != operands.len() {
        return ResolvedExpressionRow::unsupported(document);
    }

    let root = match roots.as_slice() {
        [] => return ResolvedExpressionRow::unsupported(document),
        [single] => *single,
        many => builder.push(
            RawExpressionNodeKind::Operator {
                operator: ExpressionOperator::And,
                operands: many.to_vec().into_boxed_slice(),
            },
            Span::dummy(),
        ),
    };
    ResolvedExpressionRow {
        outcome: ExpressionOutcome::Resolved,
        document,
        nodes: builder.nodes.into_boxed_slice(),
        root: Some(root),
    }
}

fn canonical_classification(
    parsed: &ParsedDocument,
    expression: &AuthoredExpression,
) -> Option<()> {
    let mut ordinal = expression.operand_start;
    match expression.grammar {
        ExpressionGrammar::Constraint => {
            classify_constraint_node(parsed, &expression.node, &mut ordinal).map(|_| ())
        }
        ExpressionGrammar::Calc => {
            classify_calc_node(parsed, &expression.node, &mut ordinal).map(|_| ())
        }
    }
}

struct TreeBuilder<'a> {
    storage: &'a SemanticModelStorage,
    resolution: &'a ResolutionResults,
    parsed: Option<&'a ParsedDocument>,
    operands: &'a [AuthoredReferenceId],
    /// The global operand ordinal the next `FeatureReference` leaf consumes, dense from zero
    /// across every authored expression on the declaration.
    next_ordinal: u32,
    nodes: Vec<RawExpressionNode>,
}

impl TreeBuilder<'_> {
    fn push(&mut self, kind: RawExpressionNodeKind, span: Span) -> u32 {
        let index = self.nodes.len() as u32;
        self.nodes.push(RawExpressionNode { kind, span });
        index
    }

    /// Consumes the next authored operand reference and produces a `FeatureReference` node.
    fn feature_reference(&mut self, fallback_span: Span) -> u32 {
        let index = self.next_ordinal as usize;
        self.next_ordinal = self.next_ordinal.saturating_add(1);
        let (target, authored, span) = match self.operands.get(index).copied() {
            Some(reference_id) => {
                let reference = &self.storage.references[reference_id.index()];
                let target = match self.resolution.outcome(reference_id) {
                    Some(ResolutionStatus::Resolved(declaration)) => Some(declaration),
                    _ => None,
                };
                (
                    target,
                    authored_path(self.storage, reference.path).into_boxed_str(),
                    reference.span,
                )
            }
            None => (None, Box::from(""), fallback_span),
        };
        self.push(
            RawExpressionNodeKind::FeatureReference { target, authored },
            span,
        )
    }

    /// Walks one AST expression node, mirroring `classify_constraint_node` / `classify_calc_node`'s
    /// traversal order and operand-ordinal threading. Returns `None` for a shape outside the slice.
    /// `outer_span` is the enclosing node's span, used where the node itself carries none (the
    /// root of an `AuthoredExpression` is a bare `Expression`, not a `Node<Expression>`).
    fn walk(&mut self, node: &Expression, outer_span: Span) -> Option<u32> {
        match node {
            Expression::LiteralInteger(_)
            | Expression::LiteralReal(_)
            | Expression::LiteralBoolean(_)
            | Expression::LiteralString(_)
            | Expression::Bracket { .. } => {
                let value = literal_expression_value(self.parsed?, node)
                    .as_ref()
                    .and_then(evaluated_scalar)?;
                Some(self.push(RawExpressionNodeKind::Literal(value), outer_span))
            }
            Expression::FeatureRef(_) | Expression::FeatureChainRef(_) => {
                Some(self.feature_reference(outer_span))
            }
            Expression::Sequence { operands, .. } => {
                let elements = &operands.value.elements;
                if let [only] = elements.as_slice() {
                    return self.walk(&only.expression.value, only.expression.span);
                }
                let children =
                    self.walk_children(elements.iter().map(|element| &element.expression))?;
                Some(self.push(RawExpressionNodeKind::Unsupported { children }, outer_span))
            }
            Expression::BodyExpr(body) if body.value.parameters.is_empty() => {
                let result = body.value.result.as_ref()?;
                self.walk(&result.value, result.span)
            }
            Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
                let child = self.walk(&operand.value, operand.span)?;
                Some(self.push(
                    RawExpressionNodeKind::Operator {
                        operator: unary_operator(op),
                        operands: Box::from([child]),
                    },
                    operand.span.covering(&outer_span),
                ))
            }
            Expression::BinaryOp { op, left, right }
                if is_comparison_operator(op)
                    || is_arithmetic_operator(op)
                    || is_logical_operator(op) =>
            {
                let left_node = self.walk(&left.value, left.span)?;
                let right_node = self.walk(&right.value, right.span)?;
                Some(self.push(
                    RawExpressionNodeKind::Operator {
                        operator: binary_operator(op)?,
                        operands: Box::from([left_node, right_node]),
                    },
                    left.span.covering(&right.span),
                ))
            }
            // `select`'s selector is an operand leaf; its base is walked first, matching the
            // classifier. The pair is an unsupported shape here (a `select` is not yet a node
            // kind), but nothing resolved under it is lost.
            Expression::Select { base, .. } => {
                let base_node = self.walk(&base.value, base.span)?;
                let selector = self.feature_reference(base.span);
                Some(self.push(
                    RawExpressionNodeKind::Unsupported {
                        children: Box::from([base_node, selector]),
                    },
                    base.span,
                ))
            }
            Expression::Index { base, operands, .. } => {
                let mut children = vec![self.walk(&base.value, base.span)?];
                children.extend(
                    self.walk_children(operands.value.elements.iter().map(|e| &e.expression))?,
                );
                Some(self.push(
                    RawExpressionNodeKind::Unsupported {
                        children: children.into_boxed_slice(),
                    },
                    base.span,
                ))
            }
            Expression::Invocation { args, .. } | Expression::Constructor { args, .. } => {
                let children = self.walk_children(args.iter().map(|arg| &arg.value))?;
                Some(self.push(RawExpressionNodeKind::Unsupported { children }, outer_span))
            }
            Expression::CollectionOp { base, args, .. } => {
                let mut children = vec![self.walk(&base.value, base.span)?];
                children.extend(self.walk_children(args.iter().map(|arg| &arg.value))?);
                Some(self.push(
                    RawExpressionNodeKind::Unsupported {
                        children: children.into_boxed_slice(),
                    },
                    base.span,
                ))
            }
            Expression::TypeCheck { operand, .. } => {
                let children: Box<[u32]> = match operand {
                    Some(operand) => Box::from([self.walk(&operand.value, operand.span)?]),
                    None => Box::default(),
                };
                Some(self.push(RawExpressionNodeKind::Unsupported { children }, outer_span))
            }
            Expression::MetaCast { base, .. } => {
                let child = self.walk(&base.value, base.span)?;
                Some(self.push(
                    RawExpressionNodeKind::Unsupported {
                        children: Box::from([child]),
                    },
                    base.span,
                ))
            }
            _ => None,
        }
    }

    fn walk_children<'n>(
        &mut self,
        children: impl Iterator<Item = &'n Node<Expression>>,
    ) -> Option<Box<[u32]>> {
        let mut indexes = Vec::new();
        for child in children {
            indexes.push(self.walk(&child.value, child.span)?);
        }
        Some(indexes.into_boxed_slice())
    }
}

fn binary_operator(op: &BinaryOperator) -> Option<ExpressionOperator> {
    Some(match op {
        BinaryOperator::Eq => ExpressionOperator::Equal,
        BinaryOperator::Ne => ExpressionOperator::NotEqual,
        BinaryOperator::StrictEq => ExpressionOperator::IdenticalTo,
        BinaryOperator::StrictNe => ExpressionOperator::NotIdenticalTo,
        BinaryOperator::Lt => ExpressionOperator::Less,
        BinaryOperator::Le => ExpressionOperator::LessOrEqual,
        BinaryOperator::Gt => ExpressionOperator::Greater,
        BinaryOperator::Ge => ExpressionOperator::GreaterOrEqual,
        BinaryOperator::Add => ExpressionOperator::Add,
        BinaryOperator::Sub => ExpressionOperator::Subtract,
        BinaryOperator::Mul => ExpressionOperator::Multiply,
        BinaryOperator::Div => ExpressionOperator::Divide,
        BinaryOperator::Mod => ExpressionOperator::Modulo,
        BinaryOperator::Pow | BinaryOperator::Exp => ExpressionOperator::Power,
        BinaryOperator::And | BinaryOperator::BitAnd => ExpressionOperator::And,
        BinaryOperator::Or | BinaryOperator::BitOr => ExpressionOperator::Or,
        BinaryOperator::Xor => ExpressionOperator::Xor,
        BinaryOperator::Implies => ExpressionOperator::Implies,
        BinaryOperator::Range | BinaryOperator::NullCoalesce => return None,
    })
}

fn unary_operator(op: &UnaryOperator) -> ExpressionOperator {
    match op {
        UnaryOperator::Not => ExpressionOperator::Not,
        _ => ExpressionOperator::Negate,
    }
}

/// The authored path text of a reference, as written -- the index-time twin of
/// `SemanticModel::authored_path`.
fn authored_path(storage: &SemanticModelStorage, path: SymbolPathId) -> String {
    let Some((segments, rooted)) = storage.paths.get(path) else {
        return String::new();
    };
    let mut text = String::new();
    if rooted {
        text.push_str("$::");
    }
    for (index, segment) in segments.iter().enumerate() {
        if index != 0 {
            text.push_str("::");
        }
        text.push_str(storage.symbol(*segment).unwrap_or_default());
    }
    text
}

impl<D> SemanticModel<D> {
    /// The resolved structure of one element's authored constraint / calc / value expression.
    ///
    /// One indexed lookup and a projection of its rows -- no traversal, no re-resolution, no
    /// evaluation. `ExpressionOutcome::NotApplicable` when the element authored no such expression;
    /// `ExpressionOutcome::Unsupported` when it authored one whose shape is outside the published
    /// slice.
    pub(crate) fn resolved_expression(
        &self,
        symbol: SymbolId,
    ) -> QueryOutcome<PublishedExpression> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(element) = self.symbol_id(declaration) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        let Some(row) = self.resolved_expressions.row(declaration) else {
            return self.resolved_outcome(PublishedExpression {
                element,
                outcome: ExpressionOutcome::NotApplicable,
                nodes: Box::default(),
                root: None,
            });
        };
        let projected: Option<Vec<ExpressionNode>> = row
            .nodes
            .iter()
            .map(|raw| self.project_expression_node(raw, row.document))
            .collect();
        let Some(nodes) = projected else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        self.resolved_outcome(PublishedExpression {
            element,
            outcome: row.outcome,
            nodes: nodes.into_boxed_slice(),
            root: row.root,
        })
    }

    fn project_expression_node(
        &self,
        raw: &RawExpressionNode,
        document: DocumentIdx,
    ) -> Option<ExpressionNode> {
        let range = document_range(&self.storage, document, &raw.span).unwrap_or(TextRange {
            start: TextPosition {
                line: raw.span.line.saturating_sub(1),
                character: 0,
            },
            end: TextPosition {
                line: raw.span.line.saturating_sub(1),
                character: 0,
            },
        });
        let location = SourceLocation {
            document: self.document_handle(document)?,
            range,
            role: OccurrenceRole::Reference,
        };
        let kind = match &raw.kind {
            RawExpressionNodeKind::Literal(value) => ExpressionNodeKind::Literal(value.clone()),
            RawExpressionNodeKind::FeatureReference { target, authored } => {
                ExpressionNodeKind::FeatureReference {
                    symbol: target.and_then(|declaration| self.symbol_id(declaration)),
                    authored: authored.clone(),
                }
            }
            RawExpressionNodeKind::Operator { operator, operands } => {
                ExpressionNodeKind::Operator {
                    operator: *operator,
                    operands: operands.clone(),
                }
            }
            RawExpressionNodeKind::Unsupported { children } => ExpressionNodeKind::Unsupported {
                children: children.clone(),
            },
        };
        Some(ExpressionNode { kind, location })
    }
}
