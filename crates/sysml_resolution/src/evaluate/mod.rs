//! Canonical evaluation: authored values in phase 5, contextual values after the
//! effective-binding index barrier. Both consume the same classifier/folding algebra.

pub(crate) mod classify;
pub(crate) mod contextual;
pub(crate) mod fold;

use crate::evaluate::classify::classify_authored;
use crate::evaluate::classify::ExpressionEvalShape;
use crate::evaluate::fold::fold_eval_node;
use crate::evaluate::fold::fold_eval_node_pending;
use crate::evaluate::fold::EvalOperand;
use crate::evaluate::fold::{CallTarget, EvalNode, NumericalFunction};
use crate::index::expressions as expression;
use crate::lower::facts::AuthoredFilterCondition;
use crate::lower::storage::ParsedSources;
use crate::lower::storage::SemanticModelStorage;
use crate::model::evaluation;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DocumentIdx;
use crate::model::ReferenceKind;
use crate::model::{EvaluatedValue, QuantityUnitBinding};
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::results::SolverStatus;
use crate::EvaluationPolicy;
use crate::EvaluationState;

type OperandTargets = std::collections::BTreeMap<
    DeclarationId,
    std::collections::BTreeMap<EvalOperand, Option<DeclarationId>>,
>;

type CalleeTargets = std::collections::BTreeMap<(DocumentIdx, usize), CallTarget>;

fn resolved_callee(storage: &SemanticModelStorage, target: Option<DeclarationId>) -> CallTarget {
    let Some(target) = target else {
        return CallTarget::Unresolved;
    };
    let function = storage.declaration(target).and_then(|declaration| {
        if declaration.kind != crate::model::DeclarationKind::KermlFunction {
            return None;
        }
        if storage.documents.get(declaration.document.index())?.role
            != source_identity::SourceRole::StandardLibrary
        {
            return None;
        }
        let owner = storage.declaration(declaration.owner?)?;
        if owner.owner.is_some() || storage.symbol(owner.name.resolved()?)? != "NumericalFunctions"
        {
            return None;
        }
        match storage.symbol(declaration.name.resolved()?)? {
            "sum" => Some(NumericalFunction::Sum),
            "max" => Some(NumericalFunction::Max),
            "min" => Some(NumericalFunction::Min),
            _ => None,
        }
    });
    function
        .map(CallTarget::Numerical)
        .unwrap_or(CallTarget::Unsupported)
}

/// Intrinsics bind to the resolved, admitted standard-library declaration, never to the
/// author's callee spelling. A workspace function called `sum` cannot become an intrinsic.
fn numerical_callees(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> CalleeTargets {
    let mut targets = CalleeTargets::new();
    for (index, reference) in storage.references.iter().enumerate() {
        if reference.kind != ReferenceKind::InvocationCallee {
            continue;
        }
        let Some(end) = reference.span.offset.checked_add(reference.span.len) else {
            continue;
        };
        let Ok(id) = AuthoredReferenceId::from_index(index) else {
            continue;
        };
        let Some(document) = storage
            .declaration(reference.source)
            .map(|source| source.document)
        else {
            continue;
        };
        let target = match resolution.outcome(id) {
            Some(ResolutionStatus::Resolved(target)) => resolved_callee(storage, Some(target)),
            _ => CallTarget::Unresolved,
        };
        targets
            .entry((document, end))
            .and_modify(|existing| {
                if *existing != target {
                    *existing = CallTarget::Unresolved;
                }
            })
            .or_insert(target);
    }
    targets
}

fn bind_callees(shape: &mut ExpressionEvalShape, document: DocumentIdx, callees: &CalleeTargets) {
    if let ExpressionEvalShape::HasOperand(tree) = shape {
        tree.visit_mut(&mut |node| {
            if let EvalNode::Call(target, _) = node {
                if let CallTarget::AuthoredEnd(end) = target {
                    *target = callees
                        .get(&(document, *end))
                        .copied()
                        .unwrap_or(CallTarget::Unsupported);
                }
            }
        });
    }
}

/// Lookup by source occurrence, shared by authored and contextual evaluation.
#[derive(Default)]
pub(crate) struct UnitBindings {
    by_site: std::collections::BTreeMap<(DocumentIdx, usize, usize), UnitBinding>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum UnitBinding {
    Resolved(DeclarationId),
    Invalid,
    Unsupported,
}

impl UnitBindings {
    pub(crate) fn new(units: &[expression::SettledUnit]) -> Self {
        let mut bindings = Self::default();
        for unit in units {
            let binding = match unit.outcome {
                expression::UnitOutcome::Resolved { unit, .. } => UnitBinding::Resolved(unit),
                expression::UnitOutcome::UnknownSymbol
                | expression::UnitOutcome::Unresolved
                | expression::UnitOutcome::Ambiguous(_) => UnitBinding::Invalid,
                expression::UnitOutcome::UnsupportedExpression
                | expression::UnitOutcome::CatalogUnavailable => UnitBinding::Unsupported,
            };
            bindings
                .by_site
                .entry((unit.document, unit.span.offset, unit.span.len))
                .and_modify(|previous| {
                    if *previous != binding {
                        *previous = UnitBinding::Invalid;
                    }
                })
                .or_insert(binding);
        }
        bindings
    }

    fn bind(&self, tree: &mut EvalNode, document: DocumentIdx) {
        tree.visit_mut(&mut |node| {
            let EvalNode::Literal(value) = node else {
                return;
            };
            let EvaluatedValue::Quantity(_, unit) = value else {
                return;
            };
            let QuantityUnitBinding::Authored { offset, len } = unit.binding else {
                return;
            };
            match self.by_site.get(&(document, offset, len)) {
                Some(UnitBinding::Resolved(id)) => {
                    unit.binding = QuantityUnitBinding::Resolved(*id)
                }
                Some(UnitBinding::Invalid) => *value = EvaluatedValue::UnresolvedOperand,
                Some(UnitBinding::Unsupported) | None => {
                    unit.binding = QuantityUnitBinding::Unsupported
                }
            }
        });
    }
}

/// The published outcome of evaluating one `PendingEvaluationFact`: the declaration whose
/// constraint/calc expression it belongs to, and what evaluation settled for it.
///
/// The folding algebra's own `EvaluatedValue` is deliberately not kept here. It is the lattice the
/// fixed point needs, not a fact about the model, and `EvaluationState` already carries the value
/// where there is one -- keeping both would make two representations of one answer, with nothing
/// stopping them from disagreeing.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EvaluationFact {
    pub(crate) declaration: DeclarationId,
    pub(crate) state: EvaluationState,
    /// Identity of the unit carried by a quantity result, including computed values.
    pub(crate) unit: Option<DeclarationId>,
}

/// Slice 2 of the constraint/calc expression fact family (see `4ca42166` for slice 1). Runs
/// strictly after `resolve_dense`'s resolution fixed point settles. Both expression operands and
/// bracketed unit occurrences read the same immutable resolution product.
///
/// If resolution itself did not converge, no evaluation fact is published: an expression whose
/// operand's own resolution outcome is not settled cannot be conservatively classified as
/// resolved/unresolved, so evaluation stays vacuous for that publication (an empty `evaluation`
/// section, `has-evaluation` false) rather than guessing.
///
/// Slice 3 (constant propagation, see `EvalNode`/`fold_eval_node_pending`): a `HasOperand` fact's
/// tree is folded against a declaration -> constant-value map that is itself the result of a
/// bounded fixed point over every pending evaluation fact (`Literal` facts -- including slice 3's
/// literal-only attribute default values, see `lower_attribute_default_value` -- seed the map with
/// no dependency; each `HasOperand` fact settles once every operand it needs is itself settled,
/// either to a concrete constant, to `NonConstant`/`UnresolvedOperand`, or because its resolved
/// target has no evaluation fact at all). A dependency chain through resolved operand references
/// can be at most one hop longer per pass, so the fixed point is bounded by
/// `evaluation_facts.len() + 1` passes (mirrors `build_ancestor_closures`'s bound); any fact still
/// unsettled after the bound is exhausted is a genuine cross-declaration dependency cycle and
/// publishes the explicit `EvaluatedValue::NonConverged` outcome, never a fabricated value, an
/// infinite loop, or a panic.
pub(crate) fn compute_evaluation(
    storage: &SemanticModelStorage,
    sources: &ParsedSources,
    resolution: &ResolutionResults,
    policy: EvaluationPolicy,
    units: &UnitBindings,
) -> SettledEvaluation {
    if policy == EvaluationPolicy::Skip {
        // A declared outcome, not an absent one: every element that has an expression reports
        // that no attempt was made, which a consumer can tell apart from "nothing to evaluate".
        //
        // An unsupported shape still reports `Unsupported`: the policy decides whether evaluation
        // ran, and the shape is outside the evaluated slice whether it ran or not.
        let skipped = |shape: &ExpressionEvalShape| match shape {
            ExpressionEvalShape::Unsupported => EvaluationState::Unsupported,
            _ => EvaluationState::NotRun,
        };
        return SettledEvaluation::Settled {
            facts: storage
                .evaluation_facts
                .iter()
                .map(|pending| EvaluationFact {
                    declaration: pending.declaration,
                    state: skipped(&classify_authored(sources, &pending.expression, units)),
                    unit: None,
                })
                .collect(),
            filters: {
                let operand_targets = operand_targets(storage, resolution);
                settled_filters(storage, |condition| {
                    let shape = classify_authored(sources, &condition.expression, units);
                    (
                        skipped(&shape),
                        evaluability_inputs(&shape, condition.owner, &operand_targets),
                    )
                })
            },
        };
    }
    if resolution.solver_status != SolverStatus::Converged {
        return SettledEvaluation::Vacuous;
    }

    let operand_targets = operand_targets(storage, resolution);
    let callees = numerical_callees(storage, resolution);

    // Every authored expression, classified exactly once. Classification walks the expression
    // tree, so the five passes below read this vector rather than re-deriving the shape each time
    // they need it -- and, more importantly, there is now exactly one place in the engine where an
    // authored expression becomes an evaluation shape.
    let shapes: Vec<ExpressionEvalShape> = storage
        .evaluation_facts
        .iter()
        .map(|pending| {
            let mut shape = classify_authored(sources, &pending.expression, units);
            bind_callees(&mut shape, pending.expression.document, &callees);
            shape
        })
        .collect();

    // Every declaration whose expression can ever settle to a constant -- the only declarations
    // constant propagation can look up a value for. A resolved operand reference whose target is
    // *not* in this set has no known constant, settling immediately as `NonConstant`.
    //
    // An unsupported shape is excluded on purpose. It publishes a fact, but never a value, so a
    // dependent expression must settle against it now rather than wait for a value that cannot
    // arrive and then be reported as a dependency cycle.
    let has_fact: std::collections::BTreeSet<DeclarationId> = storage
        .evaluation_facts
        .iter()
        .zip(shapes.iter())
        .filter(|(_, shape)| !matches!(shape, ExpressionEvalShape::Unsupported))
        .map(|(pending, _)| pending.declaration)
        .collect();
    // Operand references resolve to the valued declaration (a Feature with a value, or a
    // Function or Expression with a result expression), while evaluation is owned by its value
    // Expression. This projection is the one canonical bridge between those identity domains;
    // the fixed point never duplicates an outcome onto the valued declaration.
    let value_expressions = storage.value_expressions();

    let mut outcomes: std::collections::BTreeMap<DeclarationId, EvaluatedValue> =
        Default::default();
    for (pending, shape) in storage.evaluation_facts.iter().zip(shapes.iter()) {
        match shape {
            ExpressionEvalShape::Literal(value) | ExpressionEvalShape::ConstantFolded(value) => {
                outcomes.insert(pending.declaration, value.clone());
            }
            ExpressionEvalShape::HasOperand(_) | ExpressionEvalShape::Unsupported => {}
        }
    }

    let pass_limit = storage.evaluation_facts.len().saturating_add(1);
    for _ in 0..pass_limit {
        let mut changed = false;
        for (pending, shape) in storage.evaluation_facts.iter().zip(shapes.iter()) {
            let ExpressionEvalShape::HasOperand(tree) = shape else {
                continue;
            };
            if outcomes.contains_key(&pending.declaration) {
                continue;
            }
            let targets = operand_targets.get(&pending.declaration);
            let mut resolve_operand = |operand: EvalOperand| -> Option<EvaluatedValue> {
                match targets.and_then(|targets| targets.get(&operand).copied().flatten()) {
                    None => Some(EvaluatedValue::UnresolvedOperand),
                    Some(target) => {
                        let target = value_expressions
                            .get(target.index())
                            .copied()
                            .flatten()
                            .unwrap_or(target);
                        match outcomes.get(&target) {
                            Some(value) => Some(value.clone()),
                            None if has_fact.contains(&target) => None,
                            None => Some(EvaluatedValue::NonConstant),
                        }
                    }
                }
            };
            if let Some(value) = fold_eval_node_pending(tree, &mut resolve_operand) {
                outcomes.insert(pending.declaration, value);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // Anything still unsettled after the bound is a genuine cross-declaration dependency cycle
    // (directly or transitively self-referential), not a longer-than-expected acyclic chain --
    // the bound already covers every acyclic chain up to the total fact count.
    for (pending, shape) in storage.evaluation_facts.iter().zip(shapes.iter()) {
        if matches!(shape, ExpressionEvalShape::Unsupported) {
            continue;
        }
        outcomes
            .entry(pending.declaration)
            .or_insert(EvaluatedValue::NonConverged);
    }

    let mut facts = Vec::with_capacity(storage.evaluation_facts.len());
    for (pending, shape) in storage.evaluation_facts.iter().zip(shapes.iter()) {
        if matches!(shape, ExpressionEvalShape::Unsupported) {
            facts.push(EvaluationFact {
                declaration: pending.declaration,
                state: EvaluationState::Unsupported,
                unit: None,
            });
            continue;
        }
        let Some(outcome) = outcomes.get(&pending.declaration) else {
            continue;
        };
        facts.push(EvaluationFact {
            declaration: pending.declaration,
            state: evaluation::evaluation_state(outcome, shape),
            unit: match outcome {
                EvaluatedValue::Quantity(_, unit) => unit.identity(),
                _ => None,
            },
        });
    }

    // Filter conditions settle after the declaration fixed point, against its result. They are
    // read-only consumers of it: a condition defines no declaration's value, so nothing can depend
    // on one, and folding them here cannot change what any declaration evaluated to.
    let filters = settled_filters(storage, |condition| {
        let mut shape = classify_authored(sources, &condition.expression, units);
        bind_callees(&mut shape, condition.expression.document, &callees);
        (
            fold_settled_expression(
                &shape,
                condition.owner,
                &operand_targets,
                &outcomes,
                &value_expressions,
            ),
            evaluability_inputs(&shape, condition.owner, &operand_targets),
        )
    });

    SettledEvaluation::Settled {
        facts: facts.into_boxed_slice(),
        filters,
    }
}

/// One settled record per authored `filter` condition, carrying its authored facts alongside the
/// state `state_of` decides for it.
///
/// The authored half is copied here rather than left to a later join: a state array parallel to
/// `SemanticModelStorage::filter_conditions` was a second invariant to keep, and the one branch
/// that could not produce a state for every condition broke it by publishing none.
pub(crate) fn settled_filters(
    storage: &SemanticModelStorage,
    mut settle: impl FnMut(
        &AuthoredFilterCondition,
    ) -> (EvaluationState, expression::EvaluabilityInputs),
) -> Box<[expression::SettledFilter]> {
    storage
        .filter_conditions
        .iter()
        .map(|condition| {
            let (state, evaluability) = settle(condition);
            expression::SettledFilter {
                owner: condition.owner,
                document: condition.document,
                form: condition.form,
                span: condition.span,
                state,
                predicate: condition.predicate.clone(),
                evaluability,
            }
        })
        .collect()
}

/// Each evaluation operand occurrence's canonical resolved target. Plain references use their
/// authored ordinal; member accesses use the terminal token end in the owning expression's
/// document. An unresolved or conflicting occurrence remains `None`.
fn operand_targets(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> OperandTargets {
    let mut operand_targets = OperandTargets::new();
    for (index, reference) in storage.references.iter().enumerate() {
        let Some(operand) = operand_key(reference) else {
            continue;
        };
        let Ok(id) = AuthoredReferenceId::from_index(index) else {
            continue;
        };
        let target = match resolution.outcome(id) {
            Some(ResolutionStatus::Resolved(target)) => Some(target),
            _ => None,
        };
        let slot = operand_targets.entry(reference.source).or_default();
        slot.entry(operand)
            .and_modify(|existing| {
                if *existing != target {
                    *existing = None;
                }
            })
            .or_insert(target);
    }
    operand_targets
}

fn operand_key(reference: &crate::lower::facts::AuthoredReference) -> Option<EvalOperand> {
    match reference.kind {
        ReferenceKind::ExpressionOperand => Some(EvalOperand::Ordinal(reference.ordinal)),
        ReferenceKind::MemberAccessOperand => Some(EvalOperand::MemberAccessEnd(
            reference.span.offset.checked_add(reference.span.len)?,
        )),
        _ => None,
    }
}

/// The model-level evaluability inputs of one classified expression: its feature-reference
/// referents when every node is a literal, an operand, or an operator the classifier admits (each
/// of which a model-level evaluable Kernel Function Library function implements), and
/// `Undecided` for anything else.
fn evaluability_inputs(
    shape: &ExpressionEvalShape,
    owner: DeclarationId,
    operand_targets: &OperandTargets,
) -> expression::EvaluabilityInputs {
    fn collect(node: &fold::EvalNode, ordinals: &mut Vec<EvalOperand>) -> bool {
        match node {
            fold::EvalNode::Literal(_) => true,
            fold::EvalNode::Operand(ordinal) => {
                ordinals.push(*ordinal);
                true
            }
            fold::EvalNode::Comparison(_, left, right)
            | fold::EvalNode::Arithmetic(_, left, right)
            | fold::EvalNode::Logical(_, left, right) => {
                collect(left, ordinals) && collect(right, ordinals)
            }
            fold::EvalNode::Unary(_, operand) => collect(operand, ordinals),
            fold::EvalNode::Invocation(_)
            | fold::EvalNode::Sequence(_)
            | fold::EvalNode::Call(..) => false,
        }
    }
    let mut ordinals = Vec::new();
    let composed = match shape {
        ExpressionEvalShape::Literal(_) | ExpressionEvalShape::ConstantFolded(_) => true,
        ExpressionEvalShape::HasOperand(tree) => collect(tree, &mut ordinals),
        ExpressionEvalShape::Unsupported => false,
    };
    if !composed {
        return expression::EvaluabilityInputs::Undecided;
    }
    let targets = operand_targets.get(&owner);
    expression::EvaluabilityInputs::Composed {
        referents: ordinals
            .into_iter()
            .map(|ordinal| targets.and_then(|targets| targets.get(&ordinal).copied().flatten()))
            .collect(),
    }
}

/// Everything the evaluation pass settled.
///
/// Two states, not one with empty collections. A publication whose resolution did not converge has
/// no stable outcomes to evaluate against, so it settles nothing at all -- which is a different
/// fact from settling that nothing has a value, and the completeness the publication carries is
/// where a consumer reads it.
#[derive(Debug)]
pub(crate) enum SettledEvaluation {
    /// One outcome per classified declaration expression, and one per authored filter condition.
    Settled {
        facts: Box<[EvaluationFact]>,
        filters: Box<[expression::SettledFilter]>,
    },
    /// Resolution did not converge, so no expression was evaluated.
    Vacuous,
}

/// Folds one already-classified expression against the constants a settled publication holds.
///
/// The declaration fixed point above settles every expression a declaration *owns*. A `filter`
/// condition is not one: it is written inside a declaration whose own value it does not define, so
/// it never contributes a constant and never participates in the fixed point. It is folded once,
/// afterwards, against the same settled operand targets and constants, which is exactly the
/// "consume settled facts, publish nothing back" shape the barrier requires.
pub(crate) fn fold_settled_expression(
    shape: &ExpressionEvalShape,
    owner: DeclarationId,
    operand_targets: &OperandTargets,
    outcomes: &std::collections::BTreeMap<DeclarationId, EvaluatedValue>,
    value_expressions: &[Option<DeclarationId>],
) -> EvaluationState {
    match shape {
        ExpressionEvalShape::Unsupported => EvaluationState::Unsupported,
        ExpressionEvalShape::Literal(value) | ExpressionEvalShape::ConstantFolded(value) => {
            evaluation::evaluation_state(value, shape)
        }
        ExpressionEvalShape::HasOperand(tree) => {
            let targets = operand_targets.get(&owner);
            let value = fold_eval_node(tree, &mut |operand: EvalOperand| {
                match targets.and_then(|targets| targets.get(&operand).copied().flatten()) {
                    None => EvaluatedValue::UnresolvedOperand,
                    // A target with no settled constant of its own is correctly not a constant:
                    // the declaration fixed point has already run to completion, so an absent
                    // entry is an answer, not a pending one.
                    Some(target) => {
                        let expression = value_expressions
                            .get(target.index())
                            .copied()
                            .flatten()
                            .unwrap_or(target);
                        outcomes
                            .get(&expression)
                            .cloned()
                            .unwrap_or(EvaluatedValue::NonConstant)
                    }
                }
            });
            evaluation::evaluation_state(&value, shape)
        }
    }
}
