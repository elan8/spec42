//! Comparison of addressable derived facts in the host semantic projection.
//!
//! These facts are already owned and materialized by `HostSemanticProjection`.
//! Comparison deliberately consumes that canonical projection instead of
//! reconstructing multiplicities, expressions, feature values, or connector
//! ends from syntax or renderer output.

use std::collections::BTreeMap;

use crate::error::{WorkspaceError, WorkspaceResult};
use crate::snapshot::{
    HostConnectorEnd, HostExpression, HostFeatureValue, HostMultiplicity, HostSemanticProjection,
};

/// A changed addressable semantic fact, paired by its projection-owned ID.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostSemanticFactChange<T> {
    pub semantic_id: String,
    pub previous: T,
    pub next: T,
}

/// Deterministically ordered added, removed, and changed facts of one category.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "T: serde::Serialize",
    deserialize = "T: serde::Deserialize<'de>"
))]
pub struct HostSemanticFactComparison<T> {
    pub added: Vec<T>,
    pub removed: Vec<T>,
    #[serde(default)]
    pub changed: Vec<HostSemanticFactChange<T>>,
}

impl<T> Default for HostSemanticFactComparison<T> {
    fn default() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            changed: Vec::new(),
        }
    }
}

/// Complete comparison of the addressable derived fact categories currently
/// exposed by a host semantic projection.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostDerivedFactComparison {
    pub multiplicities: HostSemanticFactComparison<HostMultiplicity>,
    pub expressions: HostSemanticFactComparison<HostExpression>,
    pub feature_values: HostSemanticFactComparison<HostFeatureValue>,
    pub connector_ends: HostSemanticFactComparison<HostConnectorEnd>,
}

pub(crate) fn compare_derived_facts(
    previous: &HostSemanticProjection,
    next: &HostSemanticProjection,
) -> WorkspaceResult<HostDerivedFactComparison> {
    Ok(HostDerivedFactComparison {
        multiplicities: compare_fact_category(
            "multiplicity",
            &previous.multiplicities,
            &next.multiplicities,
            |fact| &fact.semantic_id,
        )?,
        expressions: compare_fact_category(
            "expression",
            &previous.expressions,
            &next.expressions,
            |fact| &fact.semantic_id,
        )?,
        feature_values: compare_fact_category(
            "feature_value",
            &previous.feature_values,
            &next.feature_values,
            |fact| &fact.semantic_id,
        )?,
        connector_ends: compare_fact_category(
            "connector_end",
            &previous.connector_ends,
            &next.connector_ends,
            |fact| &fact.semantic_id,
        )?,
    })
}

fn compare_fact_category<T: Clone + PartialEq>(
    category: &str,
    previous: &[T],
    next: &[T],
    semantic_id: impl Fn(&T) -> &String,
) -> WorkspaceResult<HostSemanticFactComparison<T>> {
    let previous_by_id = facts_by_id(category, previous, &semantic_id)?;
    let next_by_id = facts_by_id(category, next, &semantic_id)?;

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();
    for (id, next_fact) in &next_by_id {
        match previous_by_id.get(id) {
            None => added.push((*next_fact).clone()),
            Some(previous_fact) if *previous_fact != *next_fact => {
                changed.push(HostSemanticFactChange {
                    semantic_id: id.clone(),
                    previous: (*previous_fact).clone(),
                    next: (*next_fact).clone(),
                });
            }
            Some(_) => {}
        }
    }
    for (id, previous_fact) in &previous_by_id {
        if !next_by_id.contains_key(id) {
            removed.push((*previous_fact).clone());
        }
    }

    Ok(HostSemanticFactComparison {
        added,
        removed,
        changed,
    })
}

fn facts_by_id<'a, T>(
    category: &str,
    facts: &'a [T],
    semantic_id: &impl Fn(&T) -> &String,
) -> WorkspaceResult<BTreeMap<String, &'a T>> {
    let mut result = BTreeMap::new();
    for fact in facts {
        let id = semantic_id(fact).clone();
        if id.is_empty() {
            return Err(WorkspaceError::missing_comparison_identity(category));
        }
        if result.insert(id.clone(), fact).is_some() {
            return Err(WorkspaceError::duplicate_comparison_identity(category, id));
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::{
        HostConnectorEnd, HostExpressionArgument, HostFeatureValue, HostMultiplicity,
        HostSemanticProjection,
    };
    use sysml_model::{TextPosition, TextRange};

    fn expression(id: &str, literal: i64) -> HostExpression {
        HostExpression {
            semantic_id: id.to_string(),
            kind: "literal".to_string(),
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1)),
            literal: Some(serde_json::json!(literal)),
            reference: None,
            operator: None,
            operand_ids: Vec::new(),
            arguments: Vec::<HostExpressionArgument>::new(),
        }
    }

    fn multiplicity(id: &str) -> HostMultiplicity {
        HostMultiplicity {
            semantic_id: id.to_string(),
            owner_id: "s42e:owner".to_string(),
            lower_bound_id: None,
            upper_bound_id: None,
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1)),
            is_implied: false,
            is_ordered: false,
            is_unique: None,
        }
    }

    fn feature_value(id: &str) -> HostFeatureValue {
        HostFeatureValue {
            semantic_id: id.to_string(),
            owner_id: "s42e:owner".to_string(),
            expression_id: "s42f:expression".to_string(),
            kind: "default".to_string(),
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1)),
            is_implied: false,
        }
    }

    fn connector_end(id: &str) -> HostConnectorEnd {
        HostConnectorEnd {
            semantic_id: id.to_string(),
            owner_id: "s42r:connection".to_string(),
            end_index: 0,
            target_feature_id: Some("s42e:feature".to_string()),
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1)),
            is_implied: false,
        }
    }

    #[test]
    fn compares_addressable_expression_facts_and_orders_by_semantic_id() {
        let previous = HostSemanticProjection {
            expressions: vec![expression("s42f:z", 1), expression("s42f:a", 1)],
            ..HostSemanticProjection::default()
        };
        let next = HostSemanticProjection {
            expressions: vec![expression("s42f:a", 2), expression("s42f:b", 3)],
            ..HostSemanticProjection::default()
        };

        let comparison = compare_derived_facts(&previous, &next).expect("compare");
        assert_eq!(
            comparison
                .expressions
                .added
                .iter()
                .map(|fact| fact.semantic_id.as_str())
                .collect::<Vec<_>>(),
            vec!["s42f:b"]
        );
        assert_eq!(
            comparison
                .expressions
                .removed
                .iter()
                .map(|fact| fact.semantic_id.as_str())
                .collect::<Vec<_>>(),
            vec!["s42f:z"]
        );
        assert_eq!(
            comparison
                .expressions
                .changed
                .iter()
                .map(|change| change.semantic_id.as_str())
                .collect::<Vec<_>>(),
            vec!["s42f:a"]
        );
    }

    #[test]
    fn rejects_duplicate_addressable_fact_identity() {
        let duplicate = expression("s42f:duplicate", 1);
        let error = compare_derived_facts(
            &HostSemanticProjection {
                expressions: vec![duplicate.clone(), duplicate],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection::default(),
        )
        .expect_err("duplicate fact identity must not be overwritten");
        assert_eq!(error.code(), "duplicate_comparison_identity");
    }

    #[test]
    fn rejects_missing_addressable_fact_identity() {
        let error = compare_derived_facts(
            &HostSemanticProjection {
                expressions: vec![expression("", 1)],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection::default(),
        )
        .expect_err("missing fact identity must fail");
        assert_eq!(error.code(), "missing_comparison_identity");
    }

    #[test]
    fn rejects_duplicates_in_every_addressable_fact_category() {
        let cases = [
            (
                "multiplicity",
                HostSemanticProjection {
                    multiplicities: vec![
                        multiplicity("s42f:duplicate"),
                        multiplicity("s42f:duplicate"),
                    ],
                    ..HostSemanticProjection::default()
                },
            ),
            (
                "expression",
                HostSemanticProjection {
                    expressions: vec![
                        expression("s42f:duplicate", 1),
                        expression("s42f:duplicate", 2),
                    ],
                    ..HostSemanticProjection::default()
                },
            ),
            (
                "feature_value",
                HostSemanticProjection {
                    feature_values: vec![
                        feature_value("s42f:duplicate"),
                        feature_value("s42f:duplicate"),
                    ],
                    ..HostSemanticProjection::default()
                },
            ),
            (
                "connector_end",
                HostSemanticProjection {
                    connector_ends: vec![
                        connector_end("s42f:duplicate"),
                        connector_end("s42f:duplicate"),
                    ],
                    ..HostSemanticProjection::default()
                },
            ),
        ];

        for (category, projection) in cases {
            let error = compare_derived_facts(&projection, &HostSemanticProjection::default())
                .expect_err("duplicate fact identity must not be overwritten");
            assert!(
                matches!(
                    error,
                    WorkspaceError::DuplicateComparisonIdentity {
                        category: ref error_category,
                        ..
                    } if error_category == category
                ),
                "unexpected error for {category}: {error}"
            );
        }
    }
}
