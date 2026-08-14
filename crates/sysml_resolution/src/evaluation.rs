//! The published evaluation contract.
//!
//! Two channels, deliberately separate, for the same reason the element kind and its membership
//! role are separate: [`EvaluatedScalar`] is *what the value is*, [`EvaluationState`] is *what
//! happened*. Folding internally needs one lattice type whose failure variants absorb -- a
//! division by zero anywhere in a tree must propagate out of it -- but a consumer asking "does
//! this element have a value" should not have to pattern-match a failure out of the value type.
//!
//! Every state is explicit. The sibling compiler collapses non-constant, unsupported,
//! depth-exceeded, cyclic and failed-lookup into a single `.unevaluable`
//! (`src/analyse/Evaluator.zig`), and expresses "not run" as absence from a memo table. Absence is
//! not a state a consumer can reason about: it cannot tell "there was nothing to evaluate" from
//! "evaluation was not run" from "evaluation ran and found nothing".

use std::fmt;

/// A computed constant value.
///
/// `PartialEq` but not `Eq`, because [`EvaluatedScalar::Real`] carries an `f64`.
#[derive(Debug, Clone, PartialEq)]
pub enum EvaluatedScalar {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    String(Box<str>),
    /// A magnitude with an authored unit token, e.g. `10 [kg]`.
    ///
    /// The unit is the raw authored text (`kg`, `SI::s`, `m/s^2`), never a resolved declaration:
    /// units may contain operators, so the parser hands them over as opaque text rather than as a
    /// qualified reference, and this layer does not invent a resolution the parser cannot support.
    Quantity {
        magnitude: Box<EvaluatedScalar>,
        unit: Box<str>,
    },
}

/// Why an attempted evaluation produced no value.
///
/// Distinct from [`EvaluationState::NonConstant`] and [`EvaluationState::Cyclic`], which are
/// answers rather than failures: an expression over a feature with no constant value is correctly
/// not constant, and a value cycle is a property of the model. These three are cases where the
/// expression could not be folded at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvaluationFailure {
    /// A constant `/` or `%` whose divisor folded to zero.
    ///
    /// Intercepted before the operation runs: integer division would panic and float division
    /// would silently yield an infinity, and neither is an honest published value.
    DivisionByZero,
    /// An operand folded to a type the operator cannot take, such as a boolean in an arithmetic
    /// position.
    TypeMismatch,
    /// An operand reference that resolution left unresolved, ambiguous, unsupported or
    /// non-converged, so what it would evaluate to is unknown.
    UnresolvedOperand,
}

/// What evaluation produced for one element.
#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationState {
    /// The element carries no expression, so there is nothing to evaluate.
    NotApplicable,
    /// The build's [`EvaluationPolicy`] excluded evaluation.
    ///
    /// Distinct from every other state: it says nothing about the expression, only that no attempt
    /// was made.
    NotRun,
    /// The authored expression is itself a literal value.
    Literal(EvaluatedScalar),
    /// The expression folded to a constant.
    ///
    /// Includes an expression over literals alone (`2 + 3`), which needs folding even though it
    /// needs no resolution -- the distinction from [`EvaluationState::Literal`] is whether a value
    /// was written or computed.
    Evaluated(EvaluatedScalar),
    /// The expression is a supported shape, but a resolved operand has no constant value of its
    /// own, so the expression is correctly not a constant.
    NonConstant,
    /// The value depends on itself, directly or through other declarations.
    ///
    /// A property of the model rather than a failure of the evaluator, and never a fabricated
    /// value, an infinite loop, or a panic.
    Cyclic,
    /// The expression's syntactic shape is outside the supported evaluation slice.
    ///
    /// Explicit rather than absent, so a consumer can tell "this crate does not evaluate this
    /// shape yet" from "this expression has no constant value".
    Unsupported,
    /// Evaluation was attempted and could not produce a value.
    Failed(EvaluationFailure),
}

impl EvaluationState {
    /// The computed value, when there is one.
    pub fn value(&self) -> Option<&EvaluatedScalar> {
        match self {
            Self::Literal(value) | Self::Evaluated(value) => Some(value),
            _ => None,
        }
    }

    /// A stable kebab-case name for this state, for debug rendering and snapshot output.
    ///
    /// Names the state only; the value, where there is one, is rendered separately.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotApplicable => "not-applicable",
            Self::NotRun => "not-run",
            Self::Literal(_) => "literal",
            Self::Evaluated(_) => "evaluated",
            Self::NonConstant => "non-constant",
            Self::Cyclic => "cyclic",
            Self::Unsupported => "unsupported",
            Self::Failed(EvaluationFailure::DivisionByZero) => "division-by-zero",
            Self::Failed(EvaluationFailure::TypeMismatch) => "type-mismatch",
            Self::Failed(EvaluationFailure::UnresolvedOperand) => "unresolved-operand",
        }
    }
}

impl fmt::Display for EvaluationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Whether a build evaluates constant expressions.
///
/// Makes [`EvaluationState::NotRun`] a declared outcome rather than an empty table that a consumer
/// cannot distinguish from "nothing to evaluate".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EvaluationPolicy {
    /// Evaluate every supported expression.
    #[default]
    Evaluate,
    /// Publish resolution only; every element with an expression reports
    /// [`EvaluationState::NotRun`].
    Skip,
}
