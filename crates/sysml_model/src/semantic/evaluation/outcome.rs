use super::*;

pub(crate) const EVALUATED_VALUE_KEY: &str = "evaluatedValue";
pub(crate) const EVALUATED_UNIT_KEY: &str = "evaluatedUnit";
pub(crate) const EVALUATION_STATUS_KEY: &str = "evaluationStatus";
pub(crate) const EVALUATION_ERROR_KEY: &str = "evaluationError";

pub(crate) const STATUS_OK: &str = "ok";
pub(crate) const STATUS_UNRESOLVED: &str = "unresolved";
pub(crate) const STATUS_AMBIGUOUS: &str = "ambiguous";
pub(crate) const STATUS_MALFORMED: &str = "malformed";
pub(crate) const STATUS_INCOMPLETE: &str = "incomplete";
pub(crate) const STATUS_TYPE_ERROR: &str = "type_error";
pub(crate) const STATUS_DIV_BY_ZERO: &str = "div_by_zero";
pub(crate) const STATUS_UNSUPPORTED: &str = "unsupported";
pub(crate) const STATUS_CYCLE: &str = "cycle";

pub(crate) const ANALYSIS_EVAL_STATUS_KEY: &str = "analysisEvaluationStatus";
pub(crate) const ANALYSIS_EVAL_VALUE_KEY: &str = "analysisEvaluationValue";
pub(crate) const ANALYSIS_EVAL_ERROR_KEY: &str = "analysisEvaluationError";
pub(crate) const ANALYSIS_CONSTRAINT_PASSED_KEY: &str = "analysisConstraintPassed";
pub(crate) const ANALYSIS_COMPUTED_VALUE_KEY: &str = "analysisComputedValue";
pub(crate) const ANALYSIS_COMPUTED_UNIT_KEY: &str = "analysisComputedUnit";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EvalStatus {
    Ok,
    Unresolved,
    Ambiguous,
    Malformed,
    Incomplete,
    TypeError,
    DivByZero,
    Unsupported,
    Cycle,
}

impl EvalStatus {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            EvalStatus::Ok => STATUS_OK,
            EvalStatus::Unresolved => STATUS_UNRESOLVED,
            EvalStatus::Ambiguous => STATUS_AMBIGUOUS,
            EvalStatus::Malformed => STATUS_MALFORMED,
            EvalStatus::Incomplete => STATUS_INCOMPLETE,
            EvalStatus::TypeError => STATUS_TYPE_ERROR,
            EvalStatus::DivByZero => STATUS_DIV_BY_ZERO,
            EvalStatus::Unsupported => STATUS_UNSUPPORTED,
            EvalStatus::Cycle => STATUS_CYCLE,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EvalOutcome {
    pub(crate) status: EvalStatus,
    pub(crate) value: Option<Value>,
    pub(crate) unit: Option<String>,
    pub(crate) error: Option<String>,
}

impl EvalOutcome {
    pub(crate) fn ok(value: Value, unit: Option<String>) -> Self {
        Self {
            status: EvalStatus::Ok,
            value: Some(value),
            unit,
            error: None,
        }
    }

    pub(crate) fn from_quantity(quantity: Quantity) -> Self {
        Self::ok(number_to_json(quantity.value), quantity.unit)
    }

    pub(crate) fn error(status: EvalStatus, message: impl Into<String>) -> Self {
        Self {
            status,
            value: None,
            unit: None,
            error: Some(message.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Quantity {
    pub(crate) value: f64,
    pub(crate) unit: Option<String>,
}

impl Quantity {
    pub(crate) fn scalar(value: f64) -> Self {
        Self { value, unit: None }
    }
}
