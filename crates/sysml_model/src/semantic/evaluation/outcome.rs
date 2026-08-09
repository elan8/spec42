pub(crate) use crate::semantic::model::EvaluationStatus as EvalStatus;
use crate::semantic::model::{EvaluatedValue, ExpressionEvaluation};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EvalOutcome {
    pub(crate) status: EvalStatus,
    pub(crate) value: Option<EvaluatedValue>,
    pub(crate) unit: Option<String>,
    pub(crate) error: Option<String>,
}

impl EvalOutcome {
    pub(crate) fn ok(value: EvaluatedValue, unit: Option<String>) -> Self {
        Self {
            status: EvalStatus::Ok,
            value: Some(value),
            unit,
            error: None,
        }
    }

    pub(crate) fn from_quantity(quantity: Quantity) -> Self {
        if !quantity.value.is_finite() {
            return Self::error(
                EvalStatus::TypeError,
                "evaluator produced a non-finite numeric result",
            );
        }
        let value = if quantity.value.fract() == 0.0
            && quantity.value >= i64::MIN as f64
            && quantity.value <= i64::MAX as f64
        {
            EvaluatedValue::Integer(quantity.value as i64)
        } else {
            EvaluatedValue::Real(quantity.value)
        };
        Self::ok(value, quantity.unit)
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

impl EvalOutcome {
    pub(crate) fn into_expression_evaluation(self) -> ExpressionEvaluation {
        ExpressionEvaluation {
            status: self.status,
            value: self.value,
            unit: self.unit,
            error: self.error,
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
