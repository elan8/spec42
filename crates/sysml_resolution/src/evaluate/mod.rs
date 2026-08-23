//! Phase 5: evaluation. The single writer for evaluated values.

pub(crate) mod classify;
pub(crate) mod fold;

use crate::evaluate::classify::ExpressionEvalShape;
use crate::evaluate::fold::fold_eval_node;
use crate::evaluate::fold::fold_eval_node_pending;
use crate::index::expressions as expression;
use crate::lower::facts::AuthoredFilterCondition;
use crate::lower::storage::SemanticModelStorage;
use crate::model::evaluation;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::EvaluatedValue;
use crate::model::ReferenceKind;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::results::SolverStatus;
use crate::EvaluationPolicy;
use crate::EvaluationState;

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
}

/// Slice 2 of the constraint/calc expression fact family (see `4ca42166` for slice 1). Runs
/// strictly after `resolve_dense`'s resolution fixed point settles, mirroring where slice 1's own
/// `ExpressionOperand` references are resolved: literal-only expressions never depend on
/// resolution and could in principle fold earlier, but folding here keeps evaluation a single
/// pass over one immutable resolved state, with no risk of an evaluation fact observing a
/// mid-fixed-point resolution outcome.
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
    resolution: &ResolutionResults,
    policy: EvaluationPolicy,
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
                    state: skipped(&pending.shape),
                })
                .collect(),
            filters: settled_filters(storage, |condition| skipped(&condition.shape)),
        };
    }
    if resolution.solver_status != SolverStatus::Converged {
        return SettledEvaluation::Vacuous;
    }

    // operand_targets[declaration][ordinal] = the ExpressionOperand reference's resolved target,
    // or None when that reference did not resolve to exactly one declaration.
    let mut operand_targets: std::collections::BTreeMap<DeclarationId, Vec<Option<DeclarationId>>> =
        Default::default();
    for (index, reference) in storage.references.iter().enumerate() {
        if reference.kind != ReferenceKind::ExpressionOperand {
            continue;
        }
        let Ok(id) = AuthoredReferenceId::from_index(index) else {
            continue;
        };
        let target = match resolution.outcome(id) {
            Some(ResolutionStatus::Resolved(target)) => Some(target),
            _ => None,
        };
        let ordinal = reference.ordinal as usize;
        let slot = operand_targets.entry(reference.source).or_default();
        if slot.len() <= ordinal {
            slot.resize(ordinal + 1, None);
        }
        slot[ordinal] = target;
    }

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
        .filter(|pending| !matches!(pending.shape, ExpressionEvalShape::Unsupported))
        .map(|pending| pending.declaration)
        .collect();

    let mut outcomes: std::collections::BTreeMap<DeclarationId, EvaluatedValue> =
        Default::default();
    for pending in storage.evaluation_facts.iter() {
        match &pending.shape {
            ExpressionEvalShape::Literal(value) | ExpressionEvalShape::ConstantFolded(value) => {
                outcomes.insert(pending.declaration, value.clone());
            }
            ExpressionEvalShape::HasOperand(_) | ExpressionEvalShape::Unsupported => {}
        }
    }

    let pass_limit = storage.evaluation_facts.len().saturating_add(1);
    for _ in 0..pass_limit {
        let mut changed = false;
        for pending in storage.evaluation_facts.iter() {
            let ExpressionEvalShape::HasOperand(tree) = &pending.shape else {
                continue;
            };
            if outcomes.contains_key(&pending.declaration) {
                continue;
            }
            let targets = operand_targets.get(&pending.declaration);
            let mut resolve_operand = |ordinal: u32| -> Option<EvaluatedValue> {
                match targets.and_then(|targets| targets.get(ordinal as usize).copied().flatten()) {
                    None => Some(EvaluatedValue::UnresolvedOperand),
                    Some(target) => match outcomes.get(&target) {
                        Some(value) => Some(value.clone()),
                        None if has_fact.contains(&target) => None,
                        None => Some(EvaluatedValue::NonConstant),
                    },
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
    for pending in storage.evaluation_facts.iter() {
        if matches!(pending.shape, ExpressionEvalShape::Unsupported) {
            continue;
        }
        outcomes
            .entry(pending.declaration)
            .or_insert(EvaluatedValue::NonConverged);
    }

    let mut facts = Vec::with_capacity(storage.evaluation_facts.len());
    for pending in storage.evaluation_facts.iter() {
        if matches!(pending.shape, ExpressionEvalShape::Unsupported) {
            facts.push(EvaluationFact {
                declaration: pending.declaration,
                state: EvaluationState::Unsupported,
            });
            continue;
        }
        let Some(outcome) = outcomes.get(&pending.declaration) else {
            continue;
        };
        facts.push(EvaluationFact {
            declaration: pending.declaration,
            state: evaluation::evaluation_state(outcome, &pending.shape),
        });
    }

    // Filter conditions settle after the declaration fixed point, against its result. They are
    // read-only consumers of it: a condition defines no declaration's value, so nothing can depend
    // on one, and folding them here cannot change what any declaration evaluated to.
    let filters = settled_filters(storage, |condition| {
        fold_settled_expression(
            &condition.shape,
            condition.owner,
            &operand_targets,
            &outcomes,
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
    mut state_of: impl FnMut(&AuthoredFilterCondition) -> EvaluationState,
) -> Box<[expression::SettledFilter]> {
    storage
        .filter_conditions
        .iter()
        .map(|condition| expression::SettledFilter {
            owner: condition.owner,
            document: condition.document,
            form: condition.form,
            span: condition.span.clone(),
            state: state_of(condition),
            predicate: condition.predicate.clone(),
        })
        .collect()
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
    operand_targets: &std::collections::BTreeMap<DeclarationId, Vec<Option<DeclarationId>>>,
    outcomes: &std::collections::BTreeMap<DeclarationId, EvaluatedValue>,
) -> EvaluationState {
    match shape {
        ExpressionEvalShape::Unsupported => EvaluationState::Unsupported,
        ExpressionEvalShape::Literal(value) | ExpressionEvalShape::ConstantFolded(value) => {
            evaluation::evaluation_state(value, shape)
        }
        ExpressionEvalShape::HasOperand(tree) => {
            let targets = operand_targets.get(&owner);
            let value = fold_eval_node(tree, &mut |ordinal: u32| {
                match targets.and_then(|targets| targets.get(ordinal as usize).copied().flatten()) {
                    None => EvaluatedValue::UnresolvedOperand,
                    // A target with no settled constant of its own is correctly not a constant:
                    // the declaration fixed point has already run to completion, so an absent
                    // entry is an answer, not a pending one.
                    Some(target) => outcomes
                        .get(&target)
                        .cloned()
                        .unwrap_or(EvaluatedValue::NonConstant),
                }
            });
            evaluation::evaluation_state(&value, shape)
        }
    }
}
