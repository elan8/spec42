//! Canonical binding-connector facts.
//!
//! A lowered `bind` has two directional authored references, but those references are not the
//! public relationship. This index pairs them once at the publication barrier so every consumer
//! reads the same connector fact and cannot accidentally match a left end from one statement to a
//! right end from another.

use super::*;
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BindingConnectorFact {
    pub(crate) connector: DeclarationId,
    pub(crate) source: BindingEndpointFact,
    pub(crate) target: BindingEndpointFact,
    pub(crate) provenance: types::FactProvenance,
}

#[derive(Debug, Default)]
pub(crate) struct BindingConnectorIndex {
    facts: Box<[BindingConnectorFact]>,
}

#[derive(Debug, Default)]
struct EndpointReferences {
    reference: Option<AuthoredReferenceId>,
    duplicate: bool,
}

impl EndpointReferences {
    fn record(&mut self, reference: AuthoredReferenceId) {
        if self.reference.replace(reference).is_some() {
            self.duplicate = true;
        }
    }

    fn settled(&self, resolution: &ResolutionResults) -> BindingEndpointFact {
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

        let facts = ends
            .into_iter()
            .map(|(connector, (source, target))| BindingConnectorFact {
                connector,
                source: source.settled(resolution),
                target: target.settled(resolution),
                provenance: types::FactProvenance::Authored,
            })
            .collect();
        Ok(Self { facts })
    }

    pub(crate) fn facts(&self) -> &[BindingConnectorFact] {
        &self.facts
    }

    /// The exact named FeatureReferenceExpression check asks about the expression's canonical
    /// target feature and result feature. Lowering currently does not publish either fact, so a
    /// connector pair alone cannot decide it. This outcome stays attached to the connector index
    /// rather than letting a diagnostic or facade re-inspect source syntax for a lookalike.
    pub(crate) fn validation(
        &self,
        rule: BindingConnectorCheckKind,
    ) -> BindingConnectorValidationOutcome {
        let _connector_facts = &self.facts;
        match rule {
            BindingConnectorCheckKind::FeatureValue => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite: BindingConnectorValidationPrerequisite::FeatureValueEndpointFacts,
                }
            }
            BindingConnectorCheckKind::ExpressionResult => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite: BindingConnectorValidationPrerequisite::ExpressionResultEndpointFacts,
                }
            }
            BindingConnectorCheckKind::FunctionResult => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite: BindingConnectorValidationPrerequisite::FunctionResultEndpointFacts,
                }
            }
            BindingConnectorCheckKind::ConstructorExpressionResultDefaultValueTbd => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite: BindingConnectorValidationPrerequisite::NormativeSpecificationTbd,
                }
            }
            BindingConnectorCheckKind::FeatureReferenceExpression => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite:
                        BindingConnectorValidationPrerequisite::FeatureReferenceExpressionTargetAndResult,
                }
            }
            BindingConnectorCheckKind::InvocationExpressionBehavior => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite:
                        BindingConnectorValidationPrerequisite::InvocationExpressionBehaviorEndpointFacts,
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
            BindingConnectorCheckKind::TransitionUsageSource => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite:
                        BindingConnectorValidationPrerequisite::TransitionUsageSourceEndpointFacts,
                }
            }
            BindingConnectorCheckKind::TransitionUsageSuccession => {
                BindingConnectorValidationOutcome::Unsupported {
                    prerequisite:
                        BindingConnectorValidationPrerequisite::TransitionUsageSuccessionEndpointFacts,
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
