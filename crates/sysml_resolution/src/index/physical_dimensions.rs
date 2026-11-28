//! Physical quantity dimensions from resolved library power factors and expression operands.

use std::collections::{BTreeMap, BTreeSet};

use sysml_v2_parser::ast::Span;

use crate::expression::{ExpressionOperator, RawExpressionNodeKind, ResolvedExpressionRow};
use crate::index::expressions::{conforms, find_anchor, RequiredMeasurement, UnitOutcome};
use crate::index::types::ScopeBits;
use crate::model::resolver::SemanticModel;
use crate::model::{DeclarationId, DocumentIdx};
use crate::EvaluatedScalar;

/// Integer powers of resolved base-quantity declarations. Missing or fractional factors
/// leave the entire dimension unknown.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Dimension(BTreeMap<DeclarationId, i64>);

impl Dimension {
    fn add_factor(&mut self, base: DeclarationId, exponent: i64) -> Option<()> {
        let total = self
            .0
            .get(&base)
            .copied()
            .unwrap_or(0)
            .checked_add(exponent)?;
        if total == 0 {
            self.0.remove(&base);
        } else {
            self.0.insert(base, total);
        }
        Some(())
    }

    fn product(&self, right: &Self, power: i64) -> Option<Self> {
        let mut result = self.clone();
        for (&base, &exponent) in &right.0 {
            result.add_factor(base, exponent.checked_mul(power)?)?;
        }
        Some(result)
    }

    pub(crate) fn display(&self, name: impl Fn(DeclarationId) -> String) -> String {
        if self.0.is_empty() {
            return "dimensionless".into();
        }
        self.0
            .iter()
            .map(|(&base, &exponent)| match exponent {
                1 => name(base),
                _ => format!("{}^{exponent}", name(base)),
            })
            .collect::<Vec<_>>()
            .join("·")
    }
}

#[derive(Debug)]
pub(crate) struct DimensionMismatch {
    pub(crate) declaration: DeclarationId,
    pub(crate) span: Span,
    pub(crate) operator: ExpressionOperator,
    pub(crate) left: Dimension,
    pub(crate) right: Dimension,
}

#[derive(Debug, Default)]
pub(crate) struct PhysicalDimensions {
    measurement_types: BTreeMap<DeclarationId, Dimension>,
    pub(crate) mismatches: BTreeMap<DocumentIdx, Vec<DimensionMismatch>>,
}

impl PhysicalDimensions {
    /// A dimension is known only when every possible measurement type states the same powers.
    pub(crate) fn for_measurement_types(&self, types: &[DeclarationId]) -> Option<&Dimension> {
        let (first, remaining) = types.split_first()?;
        let dimension = self.measurement_types.get(first)?;
        remaining
            .iter()
            .all(|other| self.measurement_types.get(other) == Some(dimension))
            .then_some(dimension)
    }
}

struct Anchors {
    dimension: DeclarationId,
    factors: DeclarationId,
    base: DeclarationId,
    exponent: DeclarationId,
}

impl Anchors {
    fn find<D>(model: &SemanticModel<D>) -> Option<Self> {
        let anchor = |path: &[&str]| find_anchor(&model.storage, path);
        Some(Self {
            dimension: anchor(&[
                "MeasurementReferences",
                "ScalarMeasurementReference",
                "quantityDimension",
            ])?,
            factors: anchor(&["Quantities", "QuantityDimension", "quantityPowerFactors"])?,
            base: anchor(&["Quantities", "QuantityPowerFactor", "quantity"])?,
            exponent: anchor(&["Quantities", "QuantityPowerFactor", "exponent"])?,
        })
    }

    fn measurement_dimension<D>(
        &self,
        model: &SemanticModel<D>,
        measurement: DeclarationId,
    ) -> Option<Dimension> {
        let dimension = specific_member(model, measurement, self.dimension)?;
        let factors = specific_member(model, dimension, self.factors)?;
        let row = model.resolved_expressions.body(factors)?;
        let root = row.root?;
        let roots = match &row.nodes.get(root as usize)?.kind {
            RawExpressionNodeKind::Sequence { elements } => elements.as_ref(),
            _ => std::slice::from_ref(&root),
        };
        let mut vector = Dimension::default();
        for &node in roots {
            let factor = reference_target(row, node)?;
            let base = specific_member(model, factor, self.base)?;
            let base = model.resolved_expressions.body(base)?;
            let base = reference_target(base, base.root?)?;
            let exponent = specific_member(model, factor, self.exponent)?;
            let EvaluatedScalar::Integer(exponent) = model.evaluated_value(exponent)? else {
                return None;
            };
            vector.add_factor(base, exponent)?;
        }
        Some(vector)
    }
}

/// Select the uniquely most-specific authored specialization of a library member.
fn specific_member<D>(
    model: &SemanticModel<D>,
    owner: DeclarationId,
    anchor: DeclarationId,
) -> Option<DeclarationId> {
    let specialization = model.types.specialization();
    let candidates = std::iter::once(owner)
        .chain(
            specialization
                .scoped_ancestors(owner)
                .map(|(ancestor, _)| ancestor),
        )
        .flat_map(|owner| {
            model
                .child_declarations(owner)
                .iter()
                .copied()
                .filter_map(move |member| {
                    specialization
                        .reaches(member, anchor, ScopeBits::FeatureSpecialization)
                        .then_some((owner, member))
                })
        })
        .collect::<BTreeSet<_>>();
    let mut specific = candidates.iter().filter(|(owner, _)| {
        !candidates
            .iter()
            .any(|(other, _)| other != owner && conforms(&model.types, *other, *owner))
    });
    let &(_, member) = specific.next()?;
    specific.next().is_none().then_some(member)
}

fn reference_target(row: &ResolvedExpressionRow, node: u32) -> Option<DeclarationId> {
    match &row.nodes.get(node as usize)?.kind {
        RawExpressionNodeKind::FeatureReference { target, .. } => *target,
        _ => None,
    }
}

pub(crate) fn settle<D>(model: &SemanticModel<D>) -> PhysicalDimensions {
    let Some(anchors) = Anchors::find(model) else {
        return PhysicalDimensions::default();
    };
    let unit_types = model
        .expressions
        .all_units()
        .iter()
        .filter_map(|unit| match &unit.outcome {
            UnitOutcome::Resolved { dimensions, .. } => Some(dimensions.as_ref()),
            _ => None,
        });
    let required_types = model
        .expressions
        .required
        .iter()
        .filter_map(|required| match required {
            RequiredMeasurement::Required(types) => Some(types.as_ref()),
            _ => None,
        });
    let types: BTreeSet<_> = unit_types
        .chain(required_types)
        .flatten()
        .copied()
        .collect();
    let mut dimensions = PhysicalDimensions {
        measurement_types: types
            .into_iter()
            .filter_map(|id| Some((id, anchors.measurement_dimension(model, id)?)))
            .collect(),
        mismatches: BTreeMap::new(),
    };
    for (index, row) in model.resolved_expressions.rows.iter().enumerate() {
        let (Some(row), Ok(declaration)) = (row, DeclarationId::from_index(index)) else {
            continue;
        };
        if model
            .storage
            .document(row.document)
            .is_none_or(|document| document.role != source_identity::SourceRole::Workspace)
        {
            continue;
        }
        let mut mismatches = Vec::new();
        expression_dimensions(model, &dimensions, declaration, row, &mut mismatches);
        if !mismatches.is_empty() {
            dimensions
                .mismatches
                .entry(row.document)
                .or_default()
                .extend(mismatches);
        }
    }
    dimensions
}

#[derive(Clone)]
struct OperandDimension {
    dimension: Dimension,
    measured: bool,
    integer: Option<i64>,
}

impl OperandDimension {
    fn scalar(integer: Option<i64>) -> Self {
        Self {
            dimension: Dimension::default(),
            measured: false,
            integer,
        }
    }

    fn quantity(dimension: &Dimension) -> Self {
        Self {
            dimension: dimension.clone(),
            measured: true,
            integer: None,
        }
    }
}

fn expression_dimensions<D>(
    model: &SemanticModel<D>,
    dimensions: &PhysicalDimensions,
    declaration: DeclarationId,
    row: &ResolvedExpressionRow,
    mismatches: &mut Vec<DimensionMismatch>,
) {
    use ExpressionOperator::{
        Add, Divide, Equal, Greater, GreaterOrEqual, IdenticalTo, Less, LessOrEqual, Multiply,
        Negate, NotEqual, NotIdenticalTo, Power, Subtract,
    };
    let mut values: Vec<Option<OperandDimension>> = Vec::with_capacity(row.nodes.len());
    for node in row.nodes.iter() {
        let child = |index: u32| values.get(index as usize).and_then(Option::as_ref);
        let value = match &node.kind {
            RawExpressionNodeKind::Literal(EvaluatedScalar::Integer(value)) => {
                Some(OperandDimension::scalar(Some(*value)))
            }
            RawExpressionNodeKind::Literal(EvaluatedScalar::Real(_)) => {
                Some(OperandDimension::scalar(None))
            }
            RawExpressionNodeKind::Literal(EvaluatedScalar::Quantity { .. }) => {
                let matching = model.expressions.units(declaration).iter().filter(|unit| {
                    unit.span.offset >= node.span.offset
                        && unit.span.offset.saturating_add(unit.span.len)
                            <= node.span.offset.saturating_add(node.span.len)
                });
                let mut matching = matching.peekable();
                match (matching.next(), matching.peek()) {
                    (Some(unit), None) => match &unit.outcome {
                        UnitOutcome::Resolved {
                            dimensions: types, ..
                        } => dimensions
                            .for_measurement_types(types)
                            .map(OperandDimension::quantity),
                        _ => None,
                    },
                    _ => None,
                }
            }
            RawExpressionNodeKind::FeatureReference {
                target: Some(target),
                ..
            } => match model.expressions.required_measurement(*target) {
                RequiredMeasurement::Required(types) => dimensions
                    .for_measurement_types(types)
                    .map(OperandDimension::quantity),
                RequiredMeasurement::NotApplicable => {
                    let anchors = &model.expressions.anchors;
                    model
                        .types
                        .effective_types(*target)
                        .iter()
                        .any(|(ty, _)| {
                            [anchors.integer, anchors.real]
                                .into_iter()
                                .flatten()
                                .any(|scalar| conforms(&model.types, *ty, scalar))
                        })
                        .then(|| OperandDimension::scalar(None))
                }
                RequiredMeasurement::Indeterminate => None,
            },
            RawExpressionNodeKind::Operator { operator, operands } => match operands.as_ref() {
                [operand] if *operator == Negate => child(*operand).cloned().map(|mut value| {
                    value.integer = value.integer.and_then(i64::checked_neg);
                    value
                }),
                [left, right] => child(*left).zip(child(*right)).and_then(|(left, right)| {
                    let comparison = matches!(
                        operator,
                        Equal
                            | NotEqual
                            | IdenticalTo
                            | NotIdenticalTo
                            | Less
                            | LessOrEqual
                            | Greater
                            | GreaterOrEqual
                    );
                    if matches!(operator, Add | Subtract) || comparison {
                        if left.dimension != right.dimension {
                            if left.measured && right.measured {
                                mismatches.push(DimensionMismatch {
                                    declaration,
                                    span: node.operator_span.unwrap_or(node.span),
                                    operator: *operator,
                                    left: left.dimension.clone(),
                                    right: right.dimension.clone(),
                                });
                            }
                            return None;
                        }
                        return (!comparison).then(|| OperandDimension {
                            dimension: left.dimension.clone(),
                            measured: left.measured || right.measured,
                            integer: None,
                        });
                    }
                    let dimension = match operator {
                        Multiply => left.dimension.product(&right.dimension, 1)?,
                        Divide => left.dimension.product(&right.dimension, -1)?,
                        Power if !right.measured => {
                            Dimension::default().product(&left.dimension, right.integer?)?
                        }
                        _ => return None,
                    };
                    Some(OperandDimension {
                        dimension,
                        measured: left.measured || right.measured,
                        integer: None,
                    })
                }),
                _ => None,
            },
            _ => None,
        };
        values.push(value);
    }
}
