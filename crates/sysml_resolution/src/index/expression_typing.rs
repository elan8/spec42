//! Static result typing over settled expression trees and declared function signatures.
//!
//! Evaluation does not narrow a computed Integer result to Natural. Operator overloads
//! are identified by specialization of admitted DataFunctions declarations, and their
//! parameter/result types are read from the canonical feature-membership/type queries.

use std::collections::BTreeMap;

use crate::expression::ExpressionOperator;
use crate::index::expressions::{conforms, find_anchor};
use crate::index::resolved_expressions::{
    ContextualExpressionResult, RawExpressionNodeKind, ResolvedExpressionRow,
};
use crate::lower::facts::ParameterDirection;
use crate::model::resolver::SemanticModel;
use crate::model::{DeclarationId, DeclarationKind, ReferenceKind};
use crate::{EvaluatedScalar, MembershipRole, MultiplicityBound, MultiplicityFacts};

pub(crate) use crate::expression::RawResultTyping;

#[derive(Clone, Copy)]
pub(crate) enum ReferenceContext {
    /// An authored expression whose featuring types are already settled.
    Authored(DeclarationId),
    /// The explicit type of the occurrence in which a contextual expression is evaluated.
    Effective(DeclarationId),
}

fn is_self_reference<D>(model: &SemanticModel<D>, target: DeclarationId) -> bool {
    model
        .expressions
        .anchors
        .anything_self
        .is_some_and(|anchor| {
            target == anchor
                || model.types.specialization().reaches(
                    target,
                    anchor,
                    crate::index::types::ScopeBits::FeatureSpecialization,
                )
        })
}

fn is_that_reference<D>(model: &SemanticModel<D>, target: DeclarationId) -> bool {
    model.expressions.anchors.things_that.is_some_and(|anchor| {
        target == anchor
            || model.types.specialization().reaches(
                target,
                anchor,
                crate::index::types::ScopeBits::FeatureSpecialization,
            )
    })
}

fn context_dependent_reference<D>(
    model: &SemanticModel<D>,
    target: DeclarationId,
    reference: Option<crate::model::AuthoredReferenceId>,
) -> bool {
    is_self_reference(model, target)
        || is_that_reference(model, target)
        || reference
            .and_then(|reference| model.resolution.member_access_paths.get(&reference))
            .is_some_and(|path| {
                path.iter().any(|hop| {
                    matches!(hop,
                crate::resolve::results::ResolutionStatus::Resolved(target)
                    if is_self_reference(model, *target) || is_that_reference(model, *target))
                })
            })
}

/// Self denotes the current featuring occurrence; Base::things::that navigates to its featuring
/// instance. Their broad library typings must not replace the canonical occurrence path.
pub(crate) fn reference_result_types<D>(
    model: &SemanticModel<D>,
    context: ReferenceContext,
    target: DeclarationId,
    reference: Option<crate::model::AuthoredReferenceId>,
) -> Box<[DeclarationId]> {
    let values = if context_dependent_reference(model, target, reference) {
        let mut contexts = match context {
            ReferenceContext::Authored(expression) => model
                .types
                .featuring_types(expression)
                .iter()
                .map(|(target, _)| *target)
                .collect::<Vec<_>>(),
            ReferenceContext::Effective(context) => vec![context],
        };
        // Base::things::that is the inverse featuring instance. Walk settled hop identities,
        // keeping feature occurrences (not just their classifier types) until navigation ends.
        let path =
            reference.and_then(|reference| model.resolution.member_access_paths.get(&reference));
        let mut contextual_end = false;
        for hop in path
            .into_iter()
            .flat_map(|path| path.iter())
            .cloned()
            .chain(
                path.is_none()
                    .then_some(crate::resolve::results::ResolutionStatus::Resolved(target)),
            )
        {
            let crate::resolve::results::ResolutionStatus::Resolved(hop) = hop else {
                return Box::default();
            };
            if is_self_reference(model, hop) {
                contextual_end = true;
            } else if is_that_reference(model, hop) {
                contexts = contexts
                    .iter()
                    .flat_map(|context| {
                        model
                            .types
                            .featuring_types(*context)
                            .iter()
                            .map(|(target, _)| *target)
                    })
                    .collect();
                contextual_end = true;
            } else {
                contexts = vec![hop];
                contextual_end = false;
            }
        }
        if contextual_end {
            contexts
        } else {
            // A contextual binding may replace the terminal member while retaining authored
            // path provenance; its frozen replacement is the result, not the old final hop.
            model
                .types
                .effective_types(target)
                .iter()
                .map(|(target, _)| *target)
                .collect()
        }
    } else {
        model
            .types
            .effective_types(target)
            .iter()
            .map(|(target, _)| *target)
            .collect()
    };
    minimal_types(model, values)
}

fn minimal_types<D>(
    model: &SemanticModel<D>,
    mut values: Vec<DeclarationId>,
) -> Box<[DeclarationId]> {
    values.sort_unstable();
    values.dedup();
    let all = values.clone();
    values.retain(|general| {
        !all.iter()
            .any(|specific| specific != general && conforms(&model.types, *specific, *general))
    });
    values.into_boxed_slice()
}

struct Input {
    types: Box<[DeclarationId]>,
    optional: bool,
}

struct Signature {
    inputs: Vec<Input>,
    result: Box<[DeclarationId]>,
}

struct Registry<'a, D> {
    model: &'a SemanticModel<D>,
    natural: Option<DeclarationId>,
    positive: Option<DeclarationId>,
    sequence_size: Option<DeclarationId>,
    operators: Vec<(ExpressionOperator, Vec<DeclarationId>)>,
    signatures: BTreeMap<DeclarationId, Option<std::sync::Arc<Signature>>>,
}

impl<'a, D> Registry<'a, D> {
    fn new(model: &'a SemanticModel<D>) -> Self {
        let functions = model
            .storage
            .declarations
            .iter()
            .enumerate()
            .filter(|(_, declaration)| {
                matches!(
                    declaration.kind,
                    DeclarationKind::KermlFunction | DeclarationKind::CalcDefinition
                )
            })
            .filter_map(|(index, _)| DeclarationId::from_index(index).ok())
            .collect::<Vec<_>>();
        let operators = [
            ExpressionOperator::Add,
            ExpressionOperator::Subtract,
            ExpressionOperator::Multiply,
            ExpressionOperator::Divide,
            ExpressionOperator::Modulo,
            ExpressionOperator::Power,
            ExpressionOperator::Negate,
            ExpressionOperator::Equal,
            ExpressionOperator::NotEqual,
            ExpressionOperator::IdenticalTo,
            ExpressionOperator::NotIdenticalTo,
            ExpressionOperator::Less,
            ExpressionOperator::LessOrEqual,
            ExpressionOperator::Greater,
            ExpressionOperator::GreaterOrEqual,
        ]
        .into_iter()
        .map(|operator| {
            let root = find_anchor(&model.storage, &["DataFunctions", operator.as_str()]);
            let candidates = functions
                .iter()
                .copied()
                .filter(|function| root.is_some_and(|root| conforms(&model.types, *function, root)))
                .collect();
            (operator, candidates)
        })
        .collect();
        Self {
            model,
            natural: find_anchor(&model.storage, &["ScalarValues", "Natural"]),
            positive: find_anchor(&model.storage, &["ScalarValues", "Positive"]),
            sequence_size: find_anchor(&model.storage, &["SequenceFunctions", "size"]),
            operators,
            signatures: BTreeMap::new(),
        }
    }

    fn types(&self, feature: DeclarationId) -> Box<[DeclarationId]> {
        let values = self
            .model
            .types
            .effective_types(feature)
            .iter()
            .map(|(target, _)| *target)
            .collect::<Vec<_>>();
        minimal_types(self.model, values)
    }

    fn signature(&mut self, function: DeclarationId) -> Option<std::sync::Arc<Signature>> {
        if let Some(signature) = self.signatures.get(&function) {
            return signature.clone();
        }
        let members = self.model.feature_members(function);
        let result = members
            .iter()
            .find(|member| {
                self.model.effective_membership_role(**member)
                    == Some(MembershipRole::ReturnParameter)
            })
            .map(|member| self.types(*member));
        let signature = result.filter(|result| !result.is_empty()).map(|result| {
            std::sync::Arc::new(Signature {
                inputs: members
                    .into_iter()
                    .filter(|member| {
                        self.model
                            .storage
                            .declaration_facts(*member)
                            .is_some_and(|facts| facts.direction == Some(ParameterDirection::In))
                    })
                    .map(|input| Input {
                        types: self.types(input),
                        optional: matches!(
                            self.model.multiplicity(input),
                            MultiplicityFacts::Declared {
                                lower: MultiplicityBound::Literal(0),
                                ..
                            }
                        ),
                    })
                    .collect(),
                result,
            })
        });
        self.signatures.insert(function, signature.clone());
        signature
    }

    fn node(
        &mut self,
        row: &ResolvedExpressionRow,
        index: u32,
        context: ReferenceContext,
    ) -> RawResultTyping {
        let Some(node) = row.nodes.get(index as usize) else {
            return RawResultTyping::Unresolved;
        };
        match &node.kind {
            RawExpressionNodeKind::Literal(value) => self
                .model
                .expressions
                .scalar_type(value)
                .map(|id| RawResultTyping::Resolved(Box::from([id])))
                .unwrap_or(RawResultTyping::Unsupported),
            RawExpressionNodeKind::FeatureReference {
                target: Some(target),
                reference,
                ..
            } => {
                let types = reference_result_types(self.model, context, *target, *reference);
                if types.is_empty() {
                    RawResultTyping::Unresolved
                } else {
                    RawResultTyping::Resolved(types)
                }
            }
            RawExpressionNodeKind::FeatureReference { target: None, .. } => {
                RawResultTyping::Unresolved
            }
            RawExpressionNodeKind::Invocation { callee, arguments } => {
                match row.nodes.get(*callee as usize).map(|node| &node.kind) {
                    Some(RawExpressionNodeKind::FeatureReference {
                        target: Some(target),
                        ..
                    }) => {
                        // Sequence size counts occurrences. A canonical positive lower
                        // multiplicity proves Positive independently of evaluated values.
                        if Some(*target) == self.sequence_size && arguments.len() == 1 {
                            if let (
                                Some(positive),
                                Some(RawExpressionNodeKind::FeatureReference {
                                    target: Some(argument),
                                    reference: Some(reference),
                                    ..
                                }),
                            ) = (
                                self.positive,
                                row.nodes.get(arguments[0] as usize).map(|node| &node.kind),
                            ) {
                                if self.model.storage.references[reference.index()].kind
                                    == ReferenceKind::ExpressionOperand
                                    && matches!(self.model.multiplicity(*argument), MultiplicityFacts::Declared { lower: MultiplicityBound::Literal(lower), .. } if lower > 0)
                                {
                                    return RawResultTyping::Resolved(Box::from([positive]));
                                }
                            }
                        }
                        self.signature(*target)
                            .map(|signature| RawResultTyping::Resolved(signature.result.clone()))
                            .unwrap_or(RawResultTyping::Unsupported)
                    }
                    _ => RawResultTyping::Unresolved,
                }
            }
            RawExpressionNodeKind::Operator { operator, operands } => {
                let Some((_, candidates)) = self
                    .operators
                    .iter()
                    .find(|(candidate, _)| candidate == operator)
                else {
                    return RawResultTyping::Unsupported;
                };
                let candidates = candidates.clone();
                let arguments = operands
                    .iter()
                    .map(|operand| self.node(row, *operand, context))
                    .collect::<Vec<_>>();
                if arguments
                    .iter()
                    .any(|argument| !matches!(argument, RawResultTyping::Resolved(_)))
                {
                    return RawResultTyping::Unresolved;
                }
                let mut applicable = Vec::new();
                for function in candidates {
                    let Some(signature) = self.signature(function) else {
                        continue;
                    };
                    if signature.inputs.len() < operands.len() {
                        continue;
                    }
                    if signature.inputs[operands.len()..]
                        .iter()
                        .any(|input| !input.optional)
                    {
                        continue;
                    }
                    let matches = signature.inputs.iter().zip(&arguments).zip(operands).all(|((input, argument), operand)| {
                        let required = &input.types;
                        let RawResultTyping::Resolved(actual) = argument else { return false };
                        !required.is_empty() && required.iter().all(|required| {
                            actual.iter().any(|actual| conforms(&self.model.types, *actual, *required))
                                // A nonnegative integer literal denotes a Natural singleton.
                                // A computed Integer or a feature merely holding one does not.
                                || (Some(*required) == self.natural && matches!(row.nodes.get(*operand as usize).map(|node| &node.kind), Some(RawExpressionNodeKind::Literal(EvaluatedScalar::Integer(value))) if *value >= 0))
                        })
                    });
                    if matches {
                        applicable.push((function, signature.result.clone()));
                    }
                }
                let all = applicable
                    .iter()
                    .map(|(function, _)| *function)
                    .collect::<Vec<_>>();
                applicable.retain(|(general, _)| {
                    !all.iter().any(|specific| {
                        specific != general && conforms(&self.model.types, *specific, *general)
                    })
                });
                let result = match applicable.as_slice() {
                    [] => RawResultTyping::Unresolved,
                    [(_, result)] => RawResultTyping::Resolved(result.clone()),
                    many if many.iter().all(|(_, result)| result == &many[0].1) => {
                        RawResultTyping::Resolved(many[0].1.clone())
                    }
                    _ => RawResultTyping::Ambiguous(
                        applicable
                            .into_iter()
                            .map(|(function, _)| function)
                            .collect(),
                    ),
                };
                // Positive + Natural is Positive; a product of Positive operands is
                // Positive. These are static operand-domain guarantees, not narrowing
                // from a folded value (Positive - Positive still returns Integer).
                if let (Some(natural), Some(positive), RawResultTyping::Resolved(types)) =
                    (self.natural, self.positive, &result)
                {
                    if types.as_ref() == [natural] {
                        let is_positive = |(argument, operand): (&RawResultTyping, &u32)| {
                            matches!(argument, RawResultTyping::Resolved(types) if types.iter().any(|actual| conforms(&self.model.types, *actual, positive)))
                                || matches!(row.nodes.get(*operand as usize).map(|node| &node.kind), Some(RawExpressionNodeKind::Literal(EvaluatedScalar::Integer(value))) if *value > 0)
                        };
                        let positive_result = match operator {
                            ExpressionOperator::Add => {
                                arguments.iter().zip(operands).any(is_positive)
                            }
                            ExpressionOperator::Multiply => {
                                !arguments.is_empty()
                                    && arguments.iter().zip(operands).all(is_positive)
                            }
                            _ => false,
                        };
                        if positive_result {
                            return RawResultTyping::Resolved(Box::from([positive]));
                        }
                    }
                }
                result
            }
            RawExpressionNodeKind::Sequence { .. }
            | RawExpressionNodeKind::MetaCast { .. }
            | RawExpressionNodeKind::Unsupported { .. } => RawResultTyping::Unsupported,
        }
    }

    fn row(&mut self, row: &ResolvedExpressionRow, context: ReferenceContext) -> RawResultTyping {
        row.root
            .map(|root| self.node(row, root, context))
            .unwrap_or(RawResultTyping::Unsupported)
    }
}

/// Settle once before diagnostics/publication. Queries only project these stored answers.
pub(crate) fn settle<D>(model: &mut SemanticModel<D>) {
    let mut registry = Registry::new(model);
    let rows = model
        .resolved_expressions
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            row.as_ref().map(|row| {
                registry.row(row, ReferenceContext::Authored(DeclarationId(index as u32)))
            })
        })
        .collect::<Vec<_>>();
    let contextual = model
        .resolved_expressions
        .contextual
        .iter()
        .filter_map(|(key, value)| {
            let ContextualExpressionResult::Resolved(row) = &value.result else {
                return None;
            };
            // Copy-on-write binding retains the authored arena exactly when no
            // target changed. Static typing is a function of that tree and the
            // same frozen type/signature indexes, so reuse its settled answer.
            let authored = if model.resolved_expressions.row(value.authored).is_some() {
                Some(value.authored)
            } else {
                model.value_expression(value.authored)
            };
            if let Some((authored, base)) =
                authored.and_then(|id| Some((id, model.resolved_expressions.row(id)?)))
            {
                let context_free = row.nodes.iter().all(|node| {
                    !matches!(&node.kind, RawExpressionNodeKind::FeatureReference { target: Some(target), reference, .. }
                        if context_dependent_reference(model, *target, *reference))
                });
                if context_free && row.root == base.root && std::sync::Arc::ptr_eq(&row.nodes, &base.nodes) {
                    if let Some(typing) = &rows[authored.index()] {
                        return Some((*key, typing.clone()));
                    }
                }
            }
            Some((*key, registry.row(row, ReferenceContext::Effective(key.0))))
        })
        .collect::<Vec<_>>();
    drop(registry);
    for (row, typing) in model.resolved_expressions.rows.iter_mut().zip(rows) {
        if let (Some(row), Some(typing)) = (row, typing) {
            row.result_typing = typing;
        }
    }
    for (key, typing) in contextual {
        if let Some(value) = model.resolved_expressions.contextual.get_mut(&key) {
            if let ContextualExpressionResult::Resolved(row) = &mut value.result {
                row.result_typing = typing;
            }
        }
    }
}
