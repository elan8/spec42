use std::collections::{HashMap, HashSet};

use crate::semantic::analysis_typing::{
    typed_case_definition_scope_prefixes, typed_requirement_definition_scope_prefixes,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{
    AnalysisEvaluation, ElementKind, EvaluatedValue, EvaluationPublicationState,
    NodeEvaluationFacts, NodeId, SemanticNode,
};
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
    // Unit metadata has not yet been published as typed semantic facts. Do not let evaluation
    // reinterpret display attributes through the graph-derived unit registry; unit-bearing expressions
    // stay explicitly unsupported until that owner exists.
    let units = UnitRegistry::default();
    let outcomes = {
        let mut engine = EvalEngine::new(graph, units.clone());
        let mut node_ids = graph.node_index_by_id.keys().cloned().collect::<Vec<_>>();
        node_ids.sort_by(|left, right| {
            left.uri
                .as_str()
                .cmp(right.uri.as_str())
                .then_with(|| left.qualified_name.cmp(&right.qualified_name))
        });
        node_ids
            .iter()
            .filter_map(|node_id| {
                if engine.node_expression(node_id).is_some() {
                    Some((node_id.clone(), engine.evaluate_node(node_id)))
                } else {
                    None
                }
            })
            .collect::<Vec<(NodeId, EvalOutcome)>>()
    };
    let mut published = outcomes
        .into_iter()
        .map(|(node_id, outcome)| {
            (
                node_id,
                NodeEvaluationFacts {
                    expression: Some(outcome.into_expression_evaluation()),
                    analysis: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    for (node_id, analysis) in evaluate_analysis_results(graph, units) {
        published.entry(node_id).or_default().analysis = Some(analysis);
    }
    // One atomic owner-scoped publication replaces all prior results. No result is inferred
    // from, or written into, a presentation attribute map.
    graph.evaluation_facts_by_node_id = published;
    graph.evaluation_publication = EvaluationPublicationState::Complete;
}

/// Project analysis results from the same declared expression evaluator used for feature values.
///
/// A Boolean result is a constraint verdict. A numeric result is an evaluated analysis value, not
/// a verdict: assigning pass/fail from its sign would invent a semantic rule that was never
/// authored.
fn evaluate_analysis_results(
    graph: &SemanticGraph,
    units: UnitRegistry,
) -> Vec<(NodeId, AnalysisEvaluation)> {
    {
        let mut engine = EvalEngine::new(graph, units);
        let mut node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
        node_ids.sort_by(|left, right| {
            left.uri
                .as_str()
                .cmp(right.uri.as_str())
                .then_with(|| left.qualified_name.cmp(&right.qualified_name))
        });
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
                        | ElementKind::Constraint
                ) && !has_declared_constraint
            {
                continue;
            }
            let Some(expression) = engine.node_expression(&node_id) else {
                continue;
            };
            let outcome = engine.evaluate_declared_expression(&node_id, &expression);
            let (passed, computed) = if outcome.status == EvalStatus::Ok {
                let passed = outcome.value.as_ref().and_then(EvaluatedValue::as_boolean);
                let computed =
                    outcome
                        .value
                        .as_ref()
                        .and_then(EvaluatedValue::as_f64)
                        .map(|value| Quantity {
                            value,
                            unit: outcome.unit.clone(),
                        });
                (passed, computed)
            } else {
                (None, None)
            };
            outcomes.push((
                node_id,
                AnalysisEvaluation {
                    expression: outcome.into_expression_evaluation(),
                    passed,
                    computed_value: computed.as_ref().map(|quantity| {
                        if quantity.value.fract() == 0.0
                            && quantity.value >= i64::MIN as f64
                            && quantity.value <= i64::MAX as f64
                        {
                            EvaluatedValue::Integer(quantity.value as i64)
                        } else {
                            EvaluatedValue::Real(quantity.value)
                        }
                    }),
                    computed_unit: computed.and_then(|quantity| quantity.unit),
                },
            ));
        }
        outcomes
    }
}

#[cfg(test)]
mod tests;
