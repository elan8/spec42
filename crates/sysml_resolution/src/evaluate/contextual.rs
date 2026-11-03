//! Evaluation after canonical contextual expression binding has settled.
//!
//! This supplies an occurrence-specific environment to the same classifier/folding
//! algebra as authored evaluation. It never changes an authored declaration's value.

use std::collections::{BTreeMap, BTreeSet};

use super::classify::{classify_authored, ExpressionEvalShape};
use super::fold::{fold_eval_node_pending, CallTarget, EvalOperand};
use crate::index::resolved_expressions::{ContextualExpressionResult, RawExpressionNodeKind};
use crate::lower::storage::ParsedSources;
use crate::model::resolver::SemanticModel;
use crate::model::{DeclarationId, EvaluatedValue, ReferenceKind};
use crate::resolve::results::ResolutionError;
use crate::{EvaluatedScalar, EvaluationFailure, EvaluationState};

fn scalar(value: &EvaluatedScalar) -> EvaluatedValue {
    match value {
        EvaluatedScalar::Boolean(value) => EvaluatedValue::Boolean(*value),
        EvaluatedScalar::Integer(value) => EvaluatedValue::Integer(*value),
        EvaluatedScalar::Real(value) => EvaluatedValue::Real(*value),
        EvaluatedScalar::String(value) => EvaluatedValue::String(value.to_string()),
        EvaluatedScalar::Quantity { magnitude, unit } => {
            EvaluatedValue::Quantity(Box::new(scalar(magnitude)), unit.to_string())
        }
    }
}

fn value(state: &EvaluationState) -> EvaluatedValue {
    match state {
        EvaluationState::Literal(value) | EvaluationState::Evaluated(value) => scalar(value),
        EvaluationState::NotRun => EvaluatedValue::NotEvaluated,
        EvaluationState::Cyclic => EvaluatedValue::NonConverged,
        EvaluationState::Failed(failure) => match failure {
            EvaluationFailure::UnresolvedOperand => EvaluatedValue::UnresolvedOperand,
            EvaluationFailure::DivisionByZero => EvaluatedValue::DivisionByZero,
            EvaluationFailure::TypeMismatch => EvaluatedValue::TypeMismatch,
            EvaluationFailure::UnsupportedUnitOperation => EvaluatedValue::UnsupportedUnitOperation,
        },
        EvaluationState::NotApplicable
        | EvaluationState::Unsupported
        | EvaluationState::NonConstant => EvaluatedValue::NonConstant,
    }
}

struct Program {
    expression: DeclarationId,
    shape: ExpressionEvalShape,
    operands: BTreeMap<EvalOperand, Option<DeclarationId>>,
    skipped: bool,
}

pub(crate) fn settle<D>(
    model: &mut SemanticModel<D>,
    sources: &ParsedSources,
) -> Result<(), ResolutionError> {
    let pending = model
        .storage
        .evaluation_facts
        .iter()
        .map(|fact| (fact.declaration, &fact.expression))
        .collect::<BTreeMap<_, _>>();
    let global = model
        .evaluation
        .iter()
        .map(|fact| (fact.declaration, value(&fact.state)))
        .collect::<BTreeMap<_, _>>();
    let contexts = model
        .resolved_expressions
        .contextual
        .keys()
        .map(|(context, _)| *context)
        .collect::<BTreeSet<_>>();
    let mut shapes = BTreeMap::new();
    let mut updates = Vec::new();
    for context in contexts {
        let rows = model
            .resolved_expressions
            .contextual
            .range((context, DeclarationId(0))..=(context, DeclarationId(u32::MAX)));
        let mut programs = BTreeMap::<DeclarationId, Program>::new();
        let mut keys = Vec::new();
        for (key, contextual) in rows {
            let ContextualExpressionResult::Resolved(row) = &contextual.result else {
                continue;
            };
            let expression = if model
                .resolved_expressions
                .row(contextual.authored)
                .is_some()
            {
                contextual.authored
            } else {
                model
                    .value_expression(contextual.authored)
                    .ok_or(ResolutionError::InvalidStorage)?
            };
            let authored = pending
                .get(&expression)
                .ok_or(ResolutionError::InvalidStorage)?;
            let mut operands = BTreeMap::new();
            let mut callees = super::CalleeTargets::new();
            for node in &row.nodes {
                let RawExpressionNodeKind::FeatureReference {
                    target,
                    reference: Some(reference),
                    ..
                } = &node.kind
                else {
                    continue;
                };
                let reference = model
                    .storage
                    .references
                    .get(reference.index())
                    .ok_or(ResolutionError::InvalidStorage)?;
                if let Some(operand) = super::operand_key(reference) {
                    operands
                        .entry(operand)
                        .and_modify(|existing| {
                            if *existing != *target {
                                *existing = None;
                            }
                        })
                        .or_insert(*target);
                }
                if reference.kind == ReferenceKind::InvocationCallee {
                    let end = reference
                        .span
                        .offset
                        .checked_add(reference.span.len)
                        .ok_or(ResolutionError::InvalidStorage)?;
                    let callee = super::resolved_callee(&model.storage, *target);
                    callees
                        .entry((row.document, end))
                        .and_modify(|existing| {
                            if *existing != callee {
                                *existing = CallTarget::Unresolved;
                            }
                        })
                        .or_insert(callee);
                }
            }
            let mut shape = shapes
                .entry(expression)
                .or_insert_with(|| classify_authored(sources, authored))
                .clone();
            super::bind_callees(&mut shape, row.document, &callees);
            let program = Program {
                expression,
                shape,
                operands,
                skipped: !model.metadata.has_evaluation
                    || matches!(model.evaluation_for(expression), EvaluationState::NotRun),
            };
            if let Some(existing) = programs.get(&contextual.effective) {
                if existing.expression != program.expression
                    || existing.operands != program.operands
                    || existing.shape != program.shape
                {
                    return Err(ResolutionError::InvalidStorage);
                }
            } else {
                programs.insert(contextual.effective, program);
            }
            keys.push((*key, contextual.effective));
        }
        let mut outcomes = BTreeMap::new();
        for (element, program) in &programs {
            let initial = if program.skipped {
                Some(EvaluatedValue::NotEvaluated)
            } else {
                match &program.shape {
                    ExpressionEvalShape::Literal(value)
                    | ExpressionEvalShape::ConstantFolded(value) => Some(value.clone()),
                    ExpressionEvalShape::Unsupported => Some(EvaluatedValue::NonConstant),
                    ExpressionEvalShape::HasOperand(_) => None,
                }
            };
            if let Some(initial) = initial {
                outcomes.insert(*element, initial);
            }
        }
        // N programs cover every acyclic dependency chain. Values still pending at the
        // fixed point depend on a cycle; unavailable external inputs settle nonconstant.
        for _ in 0..=programs.len() {
            let mut changed = false;
            for (element, program) in &programs {
                if outcomes.contains_key(element) {
                    continue;
                }
                let ExpressionEvalShape::HasOperand(tree) = &program.shape else {
                    continue;
                };
                let result = fold_eval_node_pending(tree, &mut |operand| {
                    let Some(target) = program.operands.get(&operand).copied().flatten() else {
                        return Some(EvaluatedValue::UnresolvedOperand);
                    };
                    if programs.contains_key(&target) {
                        return outcomes.get(&target).cloned();
                    }
                    let expression = model.value_expression(target).unwrap_or(target);
                    Some(
                        global
                            .get(&expression)
                            .cloned()
                            .unwrap_or(EvaluatedValue::NonConstant),
                    )
                });
                if let Some(result) = result {
                    outcomes.insert(*element, result);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        for (key, element) in keys {
            let program = &programs[&element];
            let state = if program.skipped {
                EvaluationState::NotRun
            } else if matches!(program.shape, ExpressionEvalShape::Unsupported) {
                EvaluationState::Unsupported
            } else {
                crate::model::evaluation::evaluation_state(
                    outcomes
                        .get(&element)
                        .unwrap_or(&EvaluatedValue::NonConverged),
                    &program.shape,
                )
            };
            updates.push((key, state));
        }
    }
    for (key, state) in updates {
        model
            .resolved_expressions
            .contextual
            .get_mut(&key)
            .ok_or(ResolutionError::InvalidStorage)?
            .evaluation = state;
    }
    Ok(())
}
