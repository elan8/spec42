//! Evaluation after canonical contextual expression binding has settled.
//!
//! This supplies an occurrence-specific environment to the same classifier/folding
//! algebra as authored evaluation. It never changes an authored declaration's value.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::classify::{classify_authored, ExpressionEvalShape};
use super::fold::{fold_eval_node_pending, CallTarget, EvalOperand};
use crate::expression::{ContextualExpressionResult, RawExpressionNodeKind, UnboundInput};
use crate::lower::storage::ParsedSources;
use crate::model::resolver::SemanticModel;
use crate::model::{DeclarationId, DeclarationKind, EvaluatedValue, NameId, ReferenceKind};
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
    authored: DeclarationId,
    expression: DeclarationId,
    shape: ExpressionEvalShape,
    operands: BTreeMap<EvalOperand, Option<(DeclarationId, NameId)>>,
    skipped: bool,
}

struct Outcome {
    value: EvaluatedValue,
    unbound_inputs: BTreeSet<UnboundInput>,
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
        let mut conflicts = BTreeMap::<DeclarationId, BTreeSet<DeclarationId>>::new();
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
            for node in row.nodes.iter() {
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
                    let input = if let Some(target) = *target {
                        let authored = reference
                            .operand_spelling
                            .ok_or(ResolutionError::InvalidStorage)?;
                        Some((target, authored))
                    } else {
                        None
                    };
                    operands
                        .entry(operand)
                        .and_modify(|existing| {
                            if *existing != input {
                                *existing = None;
                            }
                        })
                        .or_insert(input);
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
                authored: contextual.authored,
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
                    // Competing inherited programs are a model ambiguity, not corrupt storage.
                    conflicts
                        .entry(contextual.effective)
                        .or_insert_with(|| BTreeSet::from([existing.authored]))
                        .insert(program.authored);
                }
            } else {
                programs.insert(contextual.effective, program);
            }
            keys.push((*key, contextual.effective));
        }
        let mut outcomes = BTreeMap::new();
        for (element, program) in &programs {
            let initial = if conflicts.contains_key(element) {
                Some(EvaluatedValue::UnresolvedOperand)
            } else if program.skipped {
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
                outcomes.insert(
                    *element,
                    Outcome {
                        value: initial,
                        unbound_inputs: BTreeSet::new(),
                    },
                );
            }
        }
        let fold_program = |program: &Program, outcomes: &BTreeMap<DeclarationId, Outcome>| {
            let ExpressionEvalShape::HasOperand(tree) = &program.shape else {
                return None;
            };
            let mut unbound_inputs = BTreeSet::new();
            let value = fold_eval_node_pending(tree, &mut |operand| {
                let Some((target, authored)) = program.operands.get(&operand).copied().flatten()
                else {
                    return Some(EvaluatedValue::UnresolvedOperand);
                };
                if programs.contains_key(&target) {
                    let outcome = outcomes.get(&target)?;
                    unbound_inputs.extend(outcome.unbound_inputs.iter().copied());
                    return Some(outcome.value.clone());
                }
                let expression = model.value_expression(target);
                if let Some(value) = global.get(&expression.unwrap_or(target)) {
                    return Some(value.clone());
                }
                if expression.is_none()
                    && model.authored_value(target).is_none()
                    && model
                        .storage
                        .declaration(target)
                        .is_some_and(|declaration| {
                            declaration.kind == DeclarationKind::AttributeUsage
                        })
                {
                    unbound_inputs.insert(UnboundInput {
                        authored,
                        feature: target,
                    });
                }
                Some(EvaluatedValue::NonConstant)
            })?;
            if !matches!(value, EvaluatedValue::NonConstant) {
                unbound_inputs.clear();
            }
            Some(Outcome {
                value,
                unbound_inputs,
            })
        };
        // Most contexts settle in one ordered pass. Only pending programs allocate a worklist;
        // thereafter retries follow their own settled inputs, bounding folds by O(P + E) even
        // for reverse-authored chains. Individual-input retries preserve conditional folding.
        let mut pending = Vec::new();
        for (element, program) in &programs {
            if outcomes.contains_key(element) {
                continue;
            }
            if let Some(result) = fold_program(program, &outcomes) {
                outcomes.insert(*element, result);
            } else {
                pending.push(*element);
            }
        }
        let mut dependents = BTreeMap::<DeclarationId, BTreeSet<DeclarationId>>::new();
        let mut queue = VecDeque::new();
        let mut queued = BTreeSet::new();
        for element in pending {
            let mut ready = false;
            for (target, _) in programs[&element].operands.values().flatten() {
                if programs.contains_key(target) {
                    if outcomes.contains_key(target) {
                        ready = true;
                    } else {
                        dependents.entry(*target).or_default().insert(element);
                    }
                }
            }
            if ready {
                queue.push_back(element);
                queued.insert(element);
            }
        }
        while let Some(element) = queue.pop_front() {
            queued.remove(&element);
            if outcomes.contains_key(&element) {
                continue;
            }
            if let Some(result) = fold_program(&programs[&element], &outcomes) {
                outcomes.insert(element, result);
                for dependent in dependents.get(&element).into_iter().flatten() {
                    if !outcomes.contains_key(dependent) && queued.insert(*dependent) {
                        queue.push_back(*dependent);
                    }
                }
            }
        }
        for (key, element) in keys {
            let program = &programs[&element];
            let ambiguity = conflicts
                .get(&element)
                .map(|candidates| candidates.iter().copied().collect::<Box<[_]>>());
            let outcome = outcomes.get(&element);
            let state = if program.skipped {
                EvaluationState::NotRun
            } else if ambiguity.is_some() {
                EvaluationState::Failed(EvaluationFailure::UnresolvedOperand)
            } else if matches!(program.shape, ExpressionEvalShape::Unsupported) {
                EvaluationState::Unsupported
            } else {
                crate::model::evaluation::evaluation_state(
                    outcome
                        .map(|outcome| &outcome.value)
                        .unwrap_or(&EvaluatedValue::NonConverged),
                    &program.shape,
                )
            };
            let mut unbound_inputs = outcome
                .into_iter()
                .flat_map(|outcome| outcome.unbound_inputs.iter().copied())
                .collect::<Vec<_>>();
            unbound_inputs
                .sort_by_key(|input| (model.storage.symbol(input.authored), input.feature));
            updates.push((key, ambiguity, state, unbound_inputs.into_boxed_slice()));
        }
    }
    for (key, ambiguity, state, unbound_inputs) in updates {
        let row = model
            .resolved_expressions
            .contextual
            .get_mut(&key)
            .ok_or(ResolutionError::InvalidStorage)?;
        if let Some(candidates) = ambiguity {
            row.result = ContextualExpressionResult::Ambiguous(candidates);
        }
        row.evaluation = state;
        row.unbound_inputs = unbound_inputs;
    }
    Ok(())
}
