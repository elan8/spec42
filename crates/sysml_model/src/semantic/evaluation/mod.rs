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


mod outcome;
mod analysis_constraints;
mod engine;
mod invocation_parsing;
mod quantity_parser;
use outcome::*;
use analysis_constraints::*;
use engine::*;
use invocation_parsing::*;
use quantity_parser::*;

pub fn evaluate_expressions(graph: &mut SemanticGraph) {
    evaluate_expressions_with_unit_catalogs(graph);
}

pub fn evaluate_expressions_with_unit_catalogs(graph: &mut SemanticGraph) {
    crate::semantic::analysis_typing::prepare_analysis_evaluation_context(graph);
    let units = UnitRegistry::from_graph(graph);
    let outcomes = {
        let mut engine = EvalEngine::new(graph, units.clone());
        graph
            .node_index_by_id
            .keys()
            .filter_map(|node_id| {
                if engine.node_source_value(node_id).is_some() {
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
    evaluate_analysis_constraints(graph, units);
}

fn evaluate_analysis_constraints(graph: &mut SemanticGraph, units: UnitRegistry) {
    let outcomes = {
        let mut engine = EvalEngine::new(graph, units);
        let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
        let mut outcomes = Vec::new();
        for node_id in node_ids {
            let Some(node) = graph.get_node(&node_id) else {
                continue;
            };
            if is_definition_only_analysis_node(node) {
                continue;
            }
            let inline_constraints = node
                .attributes
                .get(ANALYSIS_CONSTRAINTS_KEY)
                .and_then(|value| value.as_array())
                .cloned()
                .unwrap_or_default();
            let single_expression = node
                .attributes
                .get(ANALYSIS_EXPRESSION_KEY)
                .and_then(Value::as_str)
                .map(str::to_string);

            let mut evaluated = false;
            let mut status = STATUS_UNKNOWN.to_string();
            let mut value: Option<Value> = None;
            let mut error: Option<String> = None;
            let mut passed: Option<bool> = None;
            let mut computed: Option<Quantity> = None;
            let mut limit: Option<Quantity> = None;
            if !inline_constraints.is_empty() {
                evaluated = true;
                for constraint in inline_constraints {
                    let Some(expr) = constraint
                        .as_object()
                        .and_then(|obj| obj.get("expression"))
                        .and_then(Value::as_str)
                    else {
                        continue;
                    };
                    match evaluate_analysis_expression(&mut engine, &node_id, expr) {
                        Ok(bool_value) => {
                            let all_pass = passed.unwrap_or(true) && bool_value;
                            passed = Some(all_pass);
                            status = if all_pass {
                                STATUS_OK.to_string()
                            } else {
                                "failed_constraint".to_string()
                            };
                            value = Some(Value::Bool(all_pass));
                            if computed.is_none() {
                                computed =
                                    evaluate_analysis_display_quantity(&mut engine, &node_id, expr);
                            }
                            if limit.is_none() {
                                limit =
                                    evaluate_analysis_limit_quantity(&mut engine, &node_id, expr);
                            }
                        }
                        Err(err) => {
                            status = err.status.as_str().to_string();
                            error = Some(err.message);
                        }
                    }
                }
            } else if let Some(expr) = single_expression.as_deref() {
                evaluated = true;
                if let Some(verdict_token) = resolve_verdict_kind_token(graph, &node_id, expr) {
                    let is_pass = verdict_token == "pass";
                    status = STATUS_OK.to_string();
                    value = Some(Value::Bool(is_pass));
                    passed = Some(is_pass);
                } else {
                    limit = evaluate_analysis_limit_quantity(&mut engine, &node_id, expr);
                    match evaluate_analysis_expression(&mut engine, &node_id, expr) {
                        Ok(bool_value) => {
                            status = if bool_value {
                                STATUS_OK.to_string()
                            } else {
                                "failed_constraint".to_string()
                            };
                            value = Some(Value::Bool(bool_value));
                            passed = Some(bool_value);
                            computed =
                                evaluate_analysis_display_quantity(&mut engine, &node_id, expr);
                        }
                        Err(err) => {
                            status = err.status.as_str().to_string();
                            error = Some(err.message);
                        }
                    }
                }
            }

            if evaluated {
                outcomes.push((node_id, status, value, error, passed, computed, limit));
            }
        }
        outcomes
    };

    for (node_id, status, value, error, passed, computed, limit) in outcomes {
        let Some(node_mut) = graph.get_node_mut(&node_id) else {
            continue;
        };
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
        if let Some(quantity) = limit {
            let display = format_quantity_display(&quantity);
            node_mut.attributes.insert(
                ANALYSIS_LIMIT_VALUE_KEY.to_string(),
                number_to_json(quantity.value),
            );
            if let Some(unit) = quantity.unit {
                node_mut
                    .attributes
                    .insert(ANALYSIS_LIMIT_UNIT_KEY.to_string(), Value::String(unit));
            }
            node_mut.attributes.insert(
                ANALYSIS_LIMIT_DISPLAY_KEY.to_string(),
                Value::String(display),
            );
        }
    }
}

#[cfg(test)]
mod tests;
