//! Canonical binding-connector facts.
//!
//! A lowered `bind` has two directional authored references, but those references are not the
//! public relationship. This index pairs them once at the publication barrier so every consumer
//! reads the same connector fact and cannot accidentally match a left end from one statement to a
//! right end from another.

use crate::index::types;
use crate::lower::facts::FeatureValueKind;
use crate::lower::storage::SemanticModelStorage;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::resolve::implied::resolve_library_specialization_anchor;
use crate::resolve::implied::LibrarySpecializationAnchor;
use crate::resolve::results::FeatureReferenceExpressionSpecializationStatus;
use crate::resolve::results::InvocationExpressionProjectionStatus;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::results::TransitionSuccessionSourceStatus;

use crate::{
    BindingConnectorCheckKind, BindingConnectorValidationOutcome,
    BindingConnectorValidationPrerequisite,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BindingEndpointFact {
    Resolved(DeclarationId),
    Ambiguous(Box<[DeclarationId]>),
    Unresolved,
    Unsupported,
}

/// Which normative rule implies one binding connector the source never spells.
///
/// Implied binding connectors have no authored declaration; their identity is the owning element
/// together with the implying rule, which is unique because each rule implies at most one
/// connector per owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ImpliedBindingRule {
    /// `checkFeatureValueBindingConnector` (KerML 8.3.4.10.2): a non-default FeatureValue binds
    /// its `value.result` to `featureWithValue`.
    FeatureValue(FeatureValueBindingFeaturing),
    /// `checkFeatureReferenceExpressionBindingConnector` (KerML 8.3.4.8.5): binds `targetFeature`
    /// to the expression `result`.
    FeatureReferenceExpression,
    /// `checkInvocationExpressionBehaviorBindingConnector` (KerML 8.3.4.8.8): an invocation of a
    /// non-Function binds the expression itself to its `result`.
    InvocationExpressionBehavior,
    /// `checkExpressionResultBindingConnector` / `checkFunctionResultBindingConnector` (KerML
    /// 8.3.4.7.3/4): a Function or Expression binds the `result` of its result expression to its
    /// own `result` (the Pilot's `TypeAdapter.addResultBinding`).
    ResultExpression,
    /// `checkTransitionUsageSourceBindingConnector` (SysML 8.3.18.9): a TransitionUsage binds its
    /// `source` to its first input parameter.
    TransitionUsageSource,
    /// `checkTransitionUsageSuccessionBindingConnector` (SysML 8.3.18.9): a TransitionUsage binds
    /// its succession to its `transitionLink` feature.
    TransitionUsageSuccession,
}

/// The canonical `featuringType` of a FeatureValue binding connector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FeatureValueBindingFeaturing {
    /// Not initial: exactly the featuring types of `featureWithValue`.
    FeaturingTypesOf(DeclarationId),
    /// Initial (`:=`): the chain `Base::things::that.Occurrences::Occurrence::startShot`.
    InitialStartShot {
        that: DeclarationId,
        start_shot: DeclarationId,
    },
    /// Initial, but a chain library anchor is missing or ambiguous; the featuring type stays
    /// explicitly unresolved instead of falling back to `featureWithValue`'s featuring types.
    InitialStartShotUnresolved,
}

/// The identity and provenance of one binding connector fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BindingConnectorOrigin {
    /// An authored `binding`/`bind` declaration.
    Authored(DeclarationId),
    /// A binding connector implied by `rule` and owned by `owner`.
    Implied {
        owner: DeclarationId,
        rule: ImpliedBindingRule,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BindingConnectorFact {
    pub(crate) origin: BindingConnectorOrigin,
    pub(crate) source: BindingEndpointFact,
    pub(crate) target: BindingEndpointFact,
    pub(crate) provenance: types::FactProvenance,
}

impl BindingConnectorFact {
    /// Whether the connector's related features are exactly `{left, right}`, in either order.
    fn relates(&self, left: DeclarationId, right: DeclarationId) -> bool {
        match (&self.source, &self.target) {
            (BindingEndpointFact::Resolved(source), BindingEndpointFact::Resolved(target)) => {
                (*source == left && *target == right) || (*source == right && *target == left)
            }
            _ => false,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct BindingConnectorIndex {
    pub(crate) facts: Box<[BindingConnectorFact]>,
    /// Implying rules with at least one applicable owner whose endpoint facts are unresolved, so
    /// the rule's implied connector could not be published.
    pub(crate) undecided: Box<[BindingConnectorCheckKind]>,
    /// Every Function or Expression that owns a result expression, with its settled result.
    pub(crate) result_expression_owners: Box<[ResultExpressionOwner]>,
}

/// One Function or Expression that owns at least one result expression (KerML
/// `ResultExpressionMembership`), with the endpoints its result binding connectors relate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResultExpressionOwner {
    /// The rule that applies to the owner: `FunctionResult` for a Function, `ExpressionResult`
    /// for an Expression.
    pub(crate) rule: BindingConnectorCheckKind,
    pub(crate) owner: DeclarationId,
    /// The owner's `result` parameter, owned or inherited (`TypeUtil.getResultParameterOf`);
    /// `None` when it is not settled (absent, ambiguous, or over an unsettled specialization).
    pub(crate) result: Option<DeclarationId>,
    /// The `result` of each result expression the owner owns, in authored order.
    pub(crate) expression_results: Box<[DeclarationId]>,
}

#[derive(Debug, Default)]
pub(crate) struct EndpointReferences {
    pub(crate) reference: Option<AuthoredReferenceId>,
    pub(crate) duplicate: bool,
}

impl EndpointReferences {
    pub(crate) fn record(&mut self, reference: AuthoredReferenceId) {
        if self.reference.replace(reference).is_some() {
            self.duplicate = true;
        }
    }

    pub(crate) fn settled(&self, resolution: &ResolutionResults) -> BindingEndpointFact {
        if self.duplicate {
            return BindingEndpointFact::Unsupported;
        }
        let Some(reference) = self.reference else {
            return BindingEndpointFact::Unsupported;
        };
        match resolution.outcome(reference) {
            Some(ResolutionStatus::Resolved(target)) => BindingEndpointFact::Resolved(target),
            Some(ResolutionStatus::Ambiguous(candidates)) => {
                BindingEndpointFact::Ambiguous(resolution.ambiguous_candidates(candidates).into())
            }
            Some(ResolutionStatus::Unresolved | ResolutionStatus::NonConverged) => {
                BindingEndpointFact::Unresolved
            }
            Some(ResolutionStatus::Unsupported) | None => BindingEndpointFact::Unsupported,
        }
    }
}

impl BindingConnectorIndex {
    pub(crate) fn build(
        storage: &SemanticModelStorage,
        resolution: &ResolutionResults,
        types: &types::TypeIndex,
    ) -> Result<Self, ResolutionError> {
        let mut ends = std::collections::BTreeMap::<
            DeclarationId,
            (EndpointReferences, EndpointReferences),
        >::new();
        for (index, declaration) in storage.declarations.iter().enumerate() {
            if !matches!(
                declaration.kind,
                DeclarationKind::Bind | DeclarationKind::KermlBinding
            ) {
                continue;
            }
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            ends.insert(
                id,
                (EndpointReferences::default(), EndpointReferences::default()),
            );
        }
        for (index, reference) in storage.references.iter().enumerate() {
            let Some((source, target)) = ends.get_mut(&reference.source) else {
                continue;
            };
            let reference =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            match storage.references[index].kind {
                ReferenceKind::BindSource => source.record(reference),
                ReferenceKind::BindTarget => target.record(reference),
                _ => {}
            }
        }

        let mut facts = ends
            .into_iter()
            .map(|(connector, (source, target))| BindingConnectorFact {
                origin: BindingConnectorOrigin::Authored(connector),
                source: source.settled(resolution),
                target: target.settled(resolution),
                provenance: types::FactProvenance::Authored,
            })
            .collect::<Vec<_>>();
        let mut undecided = Vec::new();
        implied_binding_connectors(storage, resolution, &mut facts, &mut undecided);
        let result_expression_owners = result_expression_owners(storage, resolution, types)?;
        for owner in result_expression_owners.iter() {
            // The Pilot binds the first result expression only
            // (`TypeAdapter.createResultConnector`); the rule then checks every membership.
            match (owner.result, owner.expression_results.first()) {
                (Some(result), Some(expression_result)) => facts.push(BindingConnectorFact {
                    origin: BindingConnectorOrigin::Implied {
                        owner: owner.owner,
                        rule: ImpliedBindingRule::ResultExpression,
                    },
                    source: BindingEndpointFact::Resolved(*expression_result),
                    target: BindingEndpointFact::Resolved(result),
                    provenance: types::FactProvenance::Implied,
                }),
                (None, _) => {
                    if !undecided.contains(&owner.rule) {
                        undecided.push(owner.rule);
                    }
                }
                (Some(_), None) => {}
            }
        }
        facts.sort();
        Ok(Self {
            facts: facts.into_boxed_slice(),
            undecided: undecided.into_boxed_slice(),
            result_expression_owners: result_expression_owners.into_boxed_slice(),
        })
    }

    pub(crate) fn facts(&self) -> &[BindingConnectorFact] {
        &self.facts
    }

    /// Evaluates one exact named binding-connector rule over the canonical connector facts.
    ///
    /// The requirement side is the same derivation that published the implied facts
    /// (`required_implied_bindings`), so a rule is satisfied exactly when every applicable owner
    /// holds its connector. Rules whose endpoint facts are not yet owned by the semantic layer stay
    /// explicitly unsupported; a diagnostic or facade never re-inspects source syntax for a
    /// lookalike.
    pub(crate) fn validation(
        &self,
        storage: &SemanticModelStorage,
        resolution: &ResolutionResults,
        rule: BindingConnectorCheckKind,
    ) -> BindingConnectorValidationOutcome {
        match rule {
            BindingConnectorCheckKind::FeatureValue
            | BindingConnectorCheckKind::FeatureReferenceExpression
            | BindingConnectorCheckKind::InvocationExpressionBehavior
            | BindingConnectorCheckKind::TransitionUsageSource
            | BindingConnectorCheckKind::TransitionUsageSuccession => {
                if self.undecided.contains(&rule) {
                    return BindingConnectorValidationOutcome::Unresolved;
                }
                let mut required = Vec::new();
                required_implied_bindings(storage, resolution, rule, &mut required);
                if required.iter().any(|requirement| {
                    matches!(
                        requirement.origin,
                        BindingConnectorOrigin::Implied {
                            rule: ImpliedBindingRule::FeatureValue(
                                FeatureValueBindingFeaturing::InitialStartShotUnresolved
                            ),
                            ..
                        }
                    )
                }) {
                    return BindingConnectorValidationOutcome::Unresolved;
                }
                let satisfied = required.iter().all(|requirement| {
                    self.facts.iter().any(|fact| {
                        fact.origin == requirement.origin
                            && fact.relates(requirement.left, requirement.right)
                    })
                });
                if satisfied {
                    BindingConnectorValidationOutcome::Satisfied
                } else {
                    BindingConnectorValidationOutcome::Violated
                }
            }
            BindingConnectorCheckKind::ExpressionResult
            | BindingConnectorCheckKind::FunctionResult => {
                self.result_expression_binding_outcome(storage, rule)
            }
            BindingConnectorCheckKind::ConstructorExpressionResultDefaultValueTbd => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite: BindingConnectorValidationPrerequisite::NormativeSpecificationTbd,
                }
            }
            BindingConnectorCheckKind::InvocationExpressionDefaultValueTbd => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite: BindingConnectorValidationPrerequisite::NormativeSpecificationTbd,
                }
            }
            BindingConnectorCheckKind::AcceptActionUsageReceiver => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite:
                        BindingConnectorValidationPrerequisite::AcceptActionUsageReceiverEndpointFacts,
                }
            }
            BindingConnectorCheckKind::SatisfyRequirementUsage => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite:
                        BindingConnectorValidationPrerequisite::SatisfyRequirementUsageEndpointFacts,
                }
            }
        }
    }
}

impl BindingConnectorIndex {
    /// `checkExpressionResultBindingConnector` (KerML 8.3.4.7.3) and
    /// `checkFunctionResultBindingConnector` (8.3.4.7.4): every ResultExpressionMembership of an
    /// instance of the rule's metaclass has an owned binding connector between the owner's
    /// `result` and the result expression's `result`.
    ///
    /// The connectors are the implied ones published from `result_expression_owners` and any
    /// the owner authors. An owner whose result is not settled, or whose result-expression set is
    /// not exactly known (`result_expressions_incomplete`), leaves the rule unresolved.
    fn result_expression_binding_outcome(
        &self,
        storage: &SemanticModelStorage,
        rule: BindingConnectorCheckKind,
    ) -> BindingConnectorValidationOutcome {
        let Some(metaclass) = crate::resolve::implied::binding_connector_check_rule(rule)
            .and_then(|rule| sysml_contract::ElementKind::parse(rule.metaclass))
        else {
            return BindingConnectorValidationOutcome::Unsupported {
                prerequisite: BindingConnectorValidationPrerequisite::RuleNotPublished,
            };
        };
        if self.undecided.contains(&rule) {
            return BindingConnectorValidationOutcome::Unresolved;
        }
        let incomplete = storage
            .declarations
            .iter()
            .zip(storage.declaration_facts.iter())
            .any(|(declaration, facts)| {
                facts.result_expressions_incomplete
                    && crate::model::element_kind::element_kind(declaration.kind)
                        .conforms_to(metaclass)
            });
        if incomplete {
            return BindingConnectorValidationOutcome::Unresolved;
        }
        let owned_by = |fact: &BindingConnectorFact, owner: DeclarationId| match fact.origin {
            BindingConnectorOrigin::Implied {
                owner: implied_owner,
                rule: ImpliedBindingRule::ResultExpression,
            } => implied_owner == owner,
            BindingConnectorOrigin::Authored(connector) => storage
                .declaration(connector)
                .is_some_and(|connector| connector.owner == Some(owner)),
            BindingConnectorOrigin::Implied { .. } => false,
        };
        let satisfied = self
            .result_expression_owners
            .iter()
            .filter(|owner| owner.rule == rule)
            .all(|owner| {
                let Some(result) = owner.result else {
                    return false;
                };
                owner.expression_results.iter().all(|expression_result| {
                    self.facts.iter().any(|fact| {
                        owned_by(fact, owner.owner) && fact.relates(result, *expression_result)
                    })
                })
            });
        if satisfied {
            BindingConnectorValidationOutcome::Satisfied
        } else {
            BindingConnectorValidationOutcome::Violated
        }
    }
}

/// Every Function or Expression that owns a result expression, in owner order, with its result
/// (`TypeUtil.getResultParameterOf`: the owned result parameter, else the one inherited over the
/// settled direct supertypes, through the same derivation `checkFeatureResultRedefinition`
/// pairs results with) and its result expressions' results.
fn result_expression_owners(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    types: &types::TypeIndex,
) -> Result<Vec<ResultExpressionOwner>, ResolutionError> {
    use crate::resolve::inherited_members::derive_inherited_members_from;
    use crate::resolve::inherited_members::InheritedMember;
    use crate::resolve::result_parameters::owned_result_parameters;
    let mut owners = std::collections::BTreeMap::<DeclarationId, Vec<DeclarationId>>::new();
    for (owner, expression) in storage.result_expressions() {
        let result = storage
            .declaration_facts(expression)
            .and_then(|facts| facts.expression_result)
            .ok_or(ResolutionError::InvalidStorage)?;
        owners.entry(owner).or_default().push(result);
    }
    let rule_of = |owner: DeclarationId| {
        let kind = crate::model::element_kind::element_kind(storage.declaration(owner)?.kind);
        if kind.conforms_to(sysml_contract::ElementKind::Function) {
            Some(BindingConnectorCheckKind::FunctionResult)
        } else if kind.conforms_to(sysml_contract::ElementKind::Expression) {
            Some(BindingConnectorCheckKind::ExpressionResult)
        } else {
            None
        }
    };
    owners.retain(|owner, _| rule_of(*owner).is_some());
    if owners.is_empty() {
        return Ok(Vec::new());
    }
    let owned = owned_result_parameters(storage)?;
    let inheriting = owners
        .keys()
        .copied()
        .filter(|owner| owned.owned(*owner).is_none())
        .collect::<Vec<_>>();
    let mut inherited = Vec::new();
    let mut unsettled_specialization = Vec::new();
    if !inheriting.is_empty() {
        // The derivation reads only the authored redefinitions of owned result parameters.
        let mut is_result = vec![false; storage.declarations.len()];
        for (_, result) in owned.iter() {
            if let Some(slot) = is_result.get_mut(result.index()) {
                *slot = true;
            }
        }
        let mut authored = std::collections::BTreeSet::new();
        unsettled_specialization = vec![false; storage.declarations.len()];
        for (reference, outcome) in storage.references.iter().zip(resolution.outcomes.iter()) {
            match outcome {
                ResolutionStatus::Resolved(target) => {
                    if reference.kind == ReferenceKind::Redefinition
                        && is_result
                            .get(reference.source.index())
                            .copied()
                            .unwrap_or(false)
                    {
                        authored.insert((reference.source, *target));
                    }
                }
                _ if types::edge_scopes(reference.kind).is_some() => {
                    if let Some(slot) = unsettled_specialization.get_mut(reference.source.index()) {
                        *slot = true;
                    }
                }
                _ => {}
            }
        }
        let generals = |declaration: DeclarationId| {
            let mut generals = types
                .supertypes(declaration)
                .iter()
                .map(|(general, _)| *general)
                .collect::<Vec<_>>();
            generals.sort();
            generals.dedup();
            generals
        };
        inherited = derive_inherited_members_from(
            &owned,
            generals,
            &authored,
            inheriting.iter().map(|owner| owner.index()),
        )?;
    }
    let unsettled = |owner: DeclarationId| {
        std::iter::once(owner)
            .chain(
                types
                    .specialization()
                    .scoped_ancestors(owner)
                    .map(|(ancestor, _)| ancestor),
            )
            .any(|declaration| {
                unsettled_specialization
                    .get(declaration.index())
                    .copied()
                    .unwrap_or(false)
            })
    };
    Ok(owners
        .into_iter()
        .filter_map(|(owner, expression_results)| {
            let result = match owned.owned(owner) {
                Some(result) => Some(result),
                None => match inherited.get(owner.index()) {
                    Some(InheritedMember::Resolved(result)) if !unsettled(owner) => Some(*result),
                    _ => None,
                },
            };
            Some(ResultExpressionOwner {
                rule: rule_of(owner)?,
                owner,
                result,
                expression_results: expression_results.into_boxed_slice(),
            })
        })
        .collect())
}

/// One binding connector a normative rule requires: its canonical origin and related features.
struct RequiredBinding {
    origin: BindingConnectorOrigin,
    left: DeclarationId,
    right: DeclarationId,
}

/// The single derivation of which implied binding connectors the model requires.
///
/// Publication (`implied_binding_connectors`) and the exact rule query both consume it, so the
/// published facts and the evaluated requirement cannot drift apart. Each endpoint comes from an
/// owned semantic fact: the FeatureValue record, or the settled FeatureReferenceExpression and
/// InvocationExpression projections.
fn required_implied_bindings(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    rule: BindingConnectorCheckKind,
    required: &mut Vec<RequiredBinding>,
) {
    match rule {
        BindingConnectorCheckKind::FeatureValue => {
            let anchor = |path| match resolve_library_specialization_anchor(storage, path) {
                LibrarySpecializationAnchor::Resolved(anchor) => Some(anchor),
                LibrarySpecializationAnchor::Missing
                | LibrarySpecializationAnchor::Ambiguous(_) => None,
            };
            let mut start_shot_chain = None;
            for value in storage.feature_values.iter() {
                if value.is_default {
                    continue;
                }
                let featuring = match value.kind {
                    FeatureValueKind::Bind => {
                        FeatureValueBindingFeaturing::FeaturingTypesOf(value.declaration)
                    }
                    // `:=` is the initial FeatureValue form (`isInitial`).
                    FeatureValueKind::Assign => *start_shot_chain.get_or_insert_with(|| {
                        match (
                            anchor("Base::things::that"),
                            anchor("Occurrences::Occurrence::startShot"),
                        ) {
                            (Some(that), Some(start_shot)) => {
                                FeatureValueBindingFeaturing::InitialStartShot { that, start_shot }
                            }
                            _ => FeatureValueBindingFeaturing::InitialStartShotUnresolved,
                        }
                    }),
                };
                required.push(RequiredBinding {
                    origin: BindingConnectorOrigin::Implied {
                        owner: value.declaration,
                        rule: ImpliedBindingRule::FeatureValue(featuring),
                    },
                    left: value.result,
                    right: value.declaration,
                });
            }
        }
        BindingConnectorCheckKind::FeatureReferenceExpression => {
            for projection in resolution.feature_reference_expression_projections.iter() {
                required.push(RequiredBinding {
                    origin: BindingConnectorOrigin::Implied {
                        owner: projection.expression,
                        rule: ImpliedBindingRule::FeatureReferenceExpression,
                    },
                    left: projection.referent,
                    right: projection.result,
                });
            }
        }
        BindingConnectorCheckKind::InvocationExpressionBehavior => {
            for projection in resolution.invocation_expression_projections.iter() {
                if projection.instantiated_type_kind.is_function() {
                    continue;
                }
                required.push(RequiredBinding {
                    origin: BindingConnectorOrigin::Implied {
                        owner: projection.expression,
                        rule: ImpliedBindingRule::InvocationExpressionBehavior,
                    },
                    left: projection.expression,
                    right: projection.result,
                });
            }
        }
        BindingConnectorCheckKind::TransitionUsageSource => {
            let parameters = transition_source_parameters(storage);
            for projection in resolution.transition_succession_source_projections.iter() {
                let (Some(source), Some(parameter)) = (
                    projection.transition_source,
                    parameters.get(&projection.transition),
                ) else {
                    continue;
                };
                required.push(RequiredBinding {
                    origin: BindingConnectorOrigin::Implied {
                        owner: projection.transition,
                        rule: ImpliedBindingRule::TransitionUsageSource,
                    },
                    left: source,
                    right: *parameter,
                });
            }
        }
        BindingConnectorCheckKind::TransitionUsageSuccession => {
            let links = transition_members(storage, |facts| facts.is_transition_link);
            for (transition, succession) in
                transition_members(storage, |facts| facts.is_transition_succession)
            {
                let Some(link) = links.get(&transition) else {
                    continue;
                };
                required.push(RequiredBinding {
                    origin: BindingConnectorOrigin::Implied {
                        owner: transition,
                        rule: ImpliedBindingRule::TransitionUsageSuccession,
                    },
                    left: succession,
                    right: *link,
                });
            }
        }
        _ => {}
    }
}

/// The member each TransitionUsage owns with the lowering role `role` selects, by transition.
fn transition_members(
    storage: &SemanticModelStorage,
    role: impl Fn(&crate::lower::facts::DeclarationFacts) -> bool,
) -> std::collections::BTreeMap<DeclarationId, DeclarationId> {
    storage
        .declarations
        .iter()
        .zip(storage.declaration_facts.iter())
        .enumerate()
        .filter(|(_, (_, facts))| role(facts))
        .filter_map(|(index, (declaration, _))| {
            Some((declaration.owner?, DeclarationId::from_index(index).ok()?))
        })
        .collect()
}

/// Each TransitionUsage's first input parameter, by transition.
fn transition_source_parameters(
    storage: &SemanticModelStorage,
) -> std::collections::BTreeMap<DeclarationId, DeclarationId> {
    storage
        .declarations
        .iter()
        .zip(storage.declaration_facts.iter())
        .enumerate()
        .filter(|(_, (_, facts))| facts.is_transition_source_parameter)
        .filter_map(|(index, (declaration, _))| {
            Some((declaration.owner?, DeclarationId::from_index(index).ok()?))
        })
        .collect()
}

/// Publishes every implied binding connector as a canonical fact with implied provenance.
///
/// A rule with an applicable owner whose endpoint facts are unsettled (an unresolved
/// FeatureReferenceExpression target or invocation callee) is recorded as undecided rather than
/// publishing a guessed connector.
fn implied_binding_connectors(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    facts: &mut Vec<BindingConnectorFact>,
    undecided: &mut Vec<BindingConnectorCheckKind>,
) {
    let mut required = Vec::new();
    for rule in [
        BindingConnectorCheckKind::FeatureValue,
        BindingConnectorCheckKind::FeatureReferenceExpression,
        BindingConnectorCheckKind::InvocationExpressionBehavior,
        BindingConnectorCheckKind::TransitionUsageSource,
        BindingConnectorCheckKind::TransitionUsageSuccession,
    ] {
        required_implied_bindings(storage, resolution, rule, &mut required);
    }
    facts.extend(
        required
            .into_iter()
            .map(|requirement| BindingConnectorFact {
                origin: requirement.origin,
                source: BindingEndpointFact::Resolved(requirement.left),
                target: BindingEndpointFact::Resolved(requirement.right),
                provenance: types::FactProvenance::Implied,
            }),
    );
    if !storage.feature_reference_expressions.is_empty()
        && resolution.feature_reference_expression_status
            == FeatureReferenceExpressionSpecializationStatus::Unresolved
    {
        undecided.push(BindingConnectorCheckKind::FeatureReferenceExpression);
    }
    if !storage.invocations.is_empty()
        && resolution.invocation_expression_projection_status
            == InvocationExpressionProjectionStatus::Unresolved
    {
        undecided.push(BindingConnectorCheckKind::InvocationExpressionBehavior);
    }
    // A transition whose `source` is not a settled authored member (an unresolved source, or the
    // implicit previous-feature source the semantic layer does not derive yet) has an applicable
    // binding whose endpoint is unknown.
    let transitions = storage
        .declarations
        .iter()
        .filter(|declaration| declaration.kind == DeclarationKind::Transition)
        .count();
    let decided = resolution
        .transition_succession_source_projections
        .iter()
        .filter(|projection| projection.transition_source.is_some())
        .count();
    if resolution.transition_succession_source_status
        == TransitionSuccessionSourceStatus::Unresolved
        || decided != transitions
    {
        undecided.push(BindingConnectorCheckKind::TransitionUsageSource);
    }
}
