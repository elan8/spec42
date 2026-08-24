//! Projection from the folding algebra onto the published evaluation contract.
//!
//! Folding needs one lattice type: a failure has to absorb, so that a division by zero anywhere in
//! a tree propagates out of it rather than being lost when the enclosing operator folds. That is
//! why `EvaluatedValue` mixes computed scalars with failure outcomes internally.
//!
//! The published contract wants them apart -- a consumer asking "does this element have a value"
//! should not pattern-match a failure out of the value type -- so the two are joined here by a
//! total match with no wildcard arm, exactly as the element-kind projection is.

use super::EvaluatedValue;
use crate::evaluate::classify::ExpressionEvalShape;
use crate::evaluation::{EvaluatedScalar, EvaluationFailure, EvaluationState};

/// The published state for one settled evaluation outcome.
///
/// `shape` supplies what the outcome value cannot: whether a folded constant was *written* as a
/// literal or *computed*.
pub(crate) fn evaluation_state(
    outcome: &EvaluatedValue,
    shape: &ExpressionEvalShape,
) -> EvaluationState {
    match outcome {
        EvaluatedValue::Boolean(_)
        | EvaluatedValue::Integer(_)
        | EvaluatedValue::Real(_)
        | EvaluatedValue::String(_)
        | EvaluatedValue::Quantity(..) => {
            let value = evaluated_scalar(outcome)
                .expect("a scalar outcome always projects to a scalar value");
            match shape {
                ExpressionEvalShape::Literal(_) => EvaluationState::Literal(value),
                ExpressionEvalShape::ConstantFolded(_)
                | ExpressionEvalShape::HasOperand(_)
                | ExpressionEvalShape::Unsupported => EvaluationState::Evaluated(value),
            }
        }
        // Reserved internally and never published: the policy gate settles a skipped build before
        // any outcome exists, so `NotRun` is produced there rather than folded to here.
        EvaluatedValue::NotEvaluated => EvaluationState::NotRun,
        EvaluatedValue::NonConstant => EvaluationState::NonConstant,
        // The bounded pass count already covers every acyclic chain -- a chain of N facts settles
        // within N passes, and the loop stops as soon as a pass changes nothing -- so an outcome
        // still unsettled at the fixed point depends on itself. The internal name predates the
        // published one; the condition it detects is a value cycle, not solver exhaustion.
        EvaluatedValue::NonConverged => EvaluationState::Cyclic,
        EvaluatedValue::UnresolvedOperand => {
            EvaluationState::Failed(EvaluationFailure::UnresolvedOperand)
        }
        EvaluatedValue::DivisionByZero => {
            EvaluationState::Failed(EvaluationFailure::DivisionByZero)
        }
        EvaluatedValue::TypeMismatch => EvaluationState::Failed(EvaluationFailure::TypeMismatch),
    }
}

/// The scalar projection of a computed outcome, or `None` for a non-value outcome.
fn evaluated_scalar(outcome: &EvaluatedValue) -> Option<EvaluatedScalar> {
    match outcome {
        EvaluatedValue::Boolean(value) => Some(EvaluatedScalar::Boolean(*value)),
        EvaluatedValue::Integer(value) => Some(EvaluatedScalar::Integer(*value)),
        EvaluatedValue::Real(value) => Some(EvaluatedScalar::Real(*value)),
        EvaluatedValue::String(value) => Some(EvaluatedScalar::String(value.as_str().into())),
        EvaluatedValue::Quantity(magnitude, unit) => Some(EvaluatedScalar::Quantity {
            magnitude: Box::new(evaluated_scalar(magnitude)?),
            unit: unit.as_str().into(),
        }),
        EvaluatedValue::NotEvaluated
        | EvaluatedValue::UnresolvedOperand
        | EvaluatedValue::NonConstant
        | EvaluatedValue::NonConverged
        | EvaluatedValue::DivisionByZero
        | EvaluatedValue::TypeMismatch => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn literal(value: EvaluatedValue) -> ExpressionEvalShape {
        ExpressionEvalShape::Literal(value)
    }

    fn folded(value: EvaluatedValue) -> ExpressionEvalShape {
        ExpressionEvalShape::ConstantFolded(value)
    }

    /// A value that was written and a value that was computed are both constants, but a consumer
    /// showing "declared value" versus "computed value" needs them apart, and only the expression
    /// shape distinguishes them.
    #[test]
    fn a_written_literal_and_a_folded_constant_are_distinct_states() {
        let five = EvaluatedValue::Integer(5);
        assert_eq!(
            evaluation_state(&five, &literal(five.clone())),
            EvaluationState::Literal(EvaluatedScalar::Integer(5))
        );
        assert_eq!(
            evaluation_state(&five, &folded(five.clone())),
            EvaluationState::Evaluated(EvaluatedScalar::Integer(5))
        );
    }

    /// Every non-value outcome reaches its own published state; none may collapse into a shared
    /// "unevaluable", which is the distinction the sibling compiler loses.
    #[test]
    fn every_non_value_outcome_has_its_own_state() {
        let shape = folded(EvaluatedValue::Integer(0));
        for (outcome, expected) in [
            (EvaluatedValue::NonConstant, EvaluationState::NonConstant),
            (EvaluatedValue::NonConverged, EvaluationState::Cyclic),
            (
                EvaluatedValue::UnresolvedOperand,
                EvaluationState::Failed(EvaluationFailure::UnresolvedOperand),
            ),
            (
                EvaluatedValue::DivisionByZero,
                EvaluationState::Failed(EvaluationFailure::DivisionByZero),
            ),
            (
                EvaluatedValue::TypeMismatch,
                EvaluationState::Failed(EvaluationFailure::TypeMismatch),
            ),
            (EvaluatedValue::NotEvaluated, EvaluationState::NotRun),
        ] {
            assert_eq!(evaluation_state(&outcome, &shape), expected);
        }
    }

    #[test]
    fn a_quantity_keeps_its_magnitude_and_authored_unit() {
        let outcome = EvaluatedValue::Quantity(Box::new(EvaluatedValue::Integer(10)), "kg".into());
        assert_eq!(
            evaluation_state(&outcome, &literal(outcome.clone())),
            EvaluationState::Literal(EvaluatedScalar::Quantity {
                magnitude: Box::new(EvaluatedScalar::Integer(10)),
                unit: "kg".into(),
            })
        );
    }

    /// Only a state carrying a value answers `value()`; a failure must not masquerade as one.
    #[test]
    fn only_a_value_bearing_state_yields_a_value() {
        assert!(EvaluationState::Literal(EvaluatedScalar::Boolean(true))
            .value()
            .is_some());
        assert!(EvaluationState::Evaluated(EvaluatedScalar::Integer(1))
            .value()
            .is_some());
        for state in [
            EvaluationState::NotApplicable,
            EvaluationState::NotRun,
            EvaluationState::NonConstant,
            EvaluationState::Cyclic,
            EvaluationState::Unsupported,
            EvaluationState::Failed(EvaluationFailure::DivisionByZero),
        ] {
            assert!(state.value().is_none(), "{state} must carry no value");
        }
    }

    #[test]
    fn state_names_are_unique() {
        let states = [
            EvaluationState::NotApplicable,
            EvaluationState::NotRun,
            EvaluationState::Literal(EvaluatedScalar::Integer(0)),
            EvaluationState::Evaluated(EvaluatedScalar::Integer(0)),
            EvaluationState::NonConstant,
            EvaluationState::Cyclic,
            EvaluationState::Unsupported,
            EvaluationState::Failed(EvaluationFailure::DivisionByZero),
            EvaluationState::Failed(EvaluationFailure::TypeMismatch),
            EvaluationState::Failed(EvaluationFailure::UnresolvedOperand),
        ];
        let names = states
            .iter()
            .map(EvaluationState::as_str)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            names.len(),
            states.len(),
            "two evaluation states share a name"
        );
    }
}
