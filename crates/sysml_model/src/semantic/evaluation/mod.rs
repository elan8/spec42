use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::semantic::analysis_typing::{
    typed_case_definition_scope_prefixes, typed_requirement_definition_scope_prefixes,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, NodeId, SemanticNode};
use crate::semantic::reference_resolution::{resolve_member_via_type, ResolveResult};

mod units;

use units::UnitError;
pub use units::UnitRegistry;

mod engine;
mod outcome;
use engine::*;
use outcome::*;

#[derive(Debug, Clone)]
pub(crate) struct BoundValue(pub(crate) Quantity);

fn json_value_to_f64(value: &Value) -> Option<f64> {
    value.as_f64().filter(|value| value.is_finite())
}

fn number_to_json(value: f64) -> Value {
    if value.fract() == 0.0 {
        Value::Number(serde_json::Number::from(value as i64))
    } else {
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .unwrap_or(Value::Null)
    }
}

fn unit_error(error: UnitError) -> EvalStatus {
    match error {
        UnitError::UnknownUnit => EvalStatus::Unresolved,
        UnitError::IncompatibleDimension => EvalStatus::TypeError,
        UnitError::UnsupportedConversion | UnitError::AmbiguousMetadata => EvalStatus::Unsupported,
    }
}

fn add_quantities(
    units: &UnitRegistry,
    left: Quantity,
    right: Quantity,
) -> Result<Quantity, EvalStatus> {
    match (&left.unit, &right.unit) {
        (None, None) => Ok(Quantity::scalar(left.value + right.value)),
        (Some(unit), None) | (None, Some(unit)) if units.has_symbol(unit) => {
            Err(EvalStatus::TypeError)
        }
        (Some(_), None) | (None, Some(_)) => Err(EvalStatus::Unresolved),
        (Some(left_unit), Some(right_unit)) => units
            .convert_value(right.value, right_unit, left_unit)
            .map(|value| Quantity {
                value: left.value + value,
                unit: Some(left_unit.clone()),
            })
            .map_err(unit_error),
    }
}

fn is_definition_only_analysis_node(node: &SemanticNode) -> bool {
    matches!(
        node.element_kind,
        ElementKind::ConstraintDef | ElementKind::CalcDef
    )
}

pub fn evaluate_expressions(graph: &mut SemanticGraph) {
    evaluate_expressions_with_unit_catalogs(graph);
}

pub fn evaluate_expressions_with_unit_catalogs(graph: &mut SemanticGraph) {
    let units = UnitRegistry::from_graph(graph);
    let outcomes = {
        let mut engine = EvalEngine::new(graph, units.clone());
        graph
            .node_index_by_id
            .keys()
            .filter_map(|node_id| {
                if engine.node_expression(node_id).is_some() {
                    Some((node_id.clone(), engine.evaluate_node(node_id)))
                } else {
                    None
                }
            })
            .collect::<Vec<(NodeId, EvalOutcome)>>()
    };
    for (node_id, outcome) in outcomes {
        let Some(node) = graph.get_node_mut(&node_id) else {
            continue;
        };
        node.attributes.remove(EVALUATED_VALUE_KEY);
        node.attributes.remove(EVALUATED_UNIT_KEY);
        node.attributes.remove(EVALUATION_STATUS_KEY);
        node.attributes.remove(EVALUATION_ERROR_KEY);
        node.attributes.insert(
            EVALUATION_STATUS_KEY.to_string(),
            Value::String(outcome.status.as_str().to_string()),
        );
        if let Some(value) = outcome.value {
            node.attributes
                .insert(EVALUATED_VALUE_KEY.to_string(), value);
        }
        if let Some(unit) = outcome.unit {
            node.attributes
                .insert(EVALUATED_UNIT_KEY.to_string(), Value::String(unit));
        }
        if let Some(error) = outcome.error {
            node.attributes
                .insert(EVALUATION_ERROR_KEY.to_string(), Value::String(error));
        }
    }
    evaluate_analysis_results(graph, units);
}

/// Project analysis results from the same declared expression evaluator used for feature values.
///
/// A Boolean result is a constraint verdict. A numeric result is an evaluated analysis value, not
/// a verdict: assigning pass/fail from its sign would invent a semantic rule that was never
/// authored.
fn evaluate_analysis_results(graph: &mut SemanticGraph, units: UnitRegistry) {
    let outcomes = {
        let mut engine = EvalEngine::new(graph, units);
        let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
        let mut outcomes = Vec::new();
        for node_id in node_ids {
            let Some(node) = graph.get_node(&node_id) else {
                continue;
            };
            let has_declared_constraint = graph.children_of(node).into_iter().any(|child| {
                matches!(
                    child.element_kind,
                    ElementKind::AssertConstraint | ElementKind::RequireConstraint
                )
            });
            if is_definition_only_analysis_node(node)
                || !matches!(
                    node.element_kind,
                    ElementKind::Analysis
                        | ElementKind::Verification
                        | ElementKind::Requirement
                        | ElementKind::RequirementDef
                ) && !has_declared_constraint
            {
                continue;
            }
            let Some(expression) = engine.node_expression(&node_id) else {
                continue;
            };
            let outcome = engine.evaluate_declared_expression(&node_id, &expression);
            let (status, value, error, passed, computed) = if outcome.status == EvalStatus::Ok {
                let passed = outcome.value.as_ref().and_then(Value::as_bool);
                let status = match passed {
                    Some(false) => "failed_constraint".to_string(),
                    Some(true) | None => STATUS_OK.to_string(),
                };
                let computed = outcome
                    .value
                    .as_ref()
                    .and_then(json_value_to_f64)
                    .map(|value| Quantity {
                        value,
                        unit: outcome.unit.clone(),
                    });
                (status, outcome.value, None, passed, computed)
            } else {
                (
                    outcome.status.as_str().to_string(),
                    None,
                    outcome.error,
                    None,
                    None,
                )
            };
            outcomes.push((node_id, status, value, error, passed, computed));
        }
        outcomes
    };

    for (node_id, status, value, error, passed, computed) in outcomes {
        let Some(node_mut) = graph.get_node_mut(&node_id) else {
            continue;
        };
        for key in [
            ANALYSIS_EVAL_STATUS_KEY,
            ANALYSIS_EVAL_VALUE_KEY,
            ANALYSIS_EVAL_ERROR_KEY,
            ANALYSIS_CONSTRAINT_PASSED_KEY,
            ANALYSIS_COMPUTED_VALUE_KEY,
            ANALYSIS_COMPUTED_UNIT_KEY,
        ] {
            node_mut.attributes.remove(key);
        }
        node_mut.attributes.insert(
            ANALYSIS_EVAL_STATUS_KEY.to_string(),
            Value::String(status.clone()),
        );
        if let Some(v) = value {
            node_mut
                .attributes
                .insert(ANALYSIS_EVAL_VALUE_KEY.to_string(), v);
        }
        if let Some(err) = error {
            node_mut
                .attributes
                .insert(ANALYSIS_EVAL_ERROR_KEY.to_string(), Value::String(err));
        }
        if let Some(p) = passed {
            node_mut
                .attributes
                .insert(ANALYSIS_CONSTRAINT_PASSED_KEY.to_string(), Value::Bool(p));
        }
        if let Some(quantity) = computed {
            node_mut.attributes.insert(
                ANALYSIS_COMPUTED_VALUE_KEY.to_string(),
                number_to_json(quantity.value),
            );
            if let Some(unit) = quantity.unit {
                node_mut
                    .attributes
                    .insert(ANALYSIS_COMPUTED_UNIT_KEY.to_string(), Value::String(unit));
            }
        }
    }
}

#[cfg(test)]
mod tests;
