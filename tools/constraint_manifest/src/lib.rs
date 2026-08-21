//! Canonical schema and reader for the generated normative constraint manifest.
//!
//! The refresh binary owns extraction. Consumers, including snapshot reporting, load this typed
//! representation rather than re-parsing TOML or reproducing rule-family classification.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// The compatible schema understood by this crate and emitted by the refresh tool.
// Schema 16 adds the closed Systems::Actions and Systems::Requirements derived-property contracts.
// The committed manifest is refreshed only at the coordinated publication barrier.
pub const SCHEMA_VERSION: u32 = 16;

/// Closed identity of every specification the manifest admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecificationId {
    KerML10,
    SysML20,
}

/// One official artifact set accepted by refresh and by normal manifest readers.
///
/// This is the single authority for specification identity, formal-document identity, and
/// artifact digests. Refresh consumes it to build a manifest; consumers use it to reject a
/// hand-edited manifest with divergent provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinnedSpecification {
    pub specification_id: SpecificationId,
    pub rule_id_prefix: &'static str,
    pub name: &'static str,
    pub slug: &'static str,
    pub version: &'static str,
    pub formal_document_id: &'static str,
    pub xmi_file_id: &'static str,
    pub expected_sha256: &'static str,
    pub expected_pdf_sha256: &'static str,
}

pub const KERML10_SPECIFICATION: PinnedSpecification = PinnedSpecification {
    specification_id: SpecificationId::KerML10,
    rule_id_prefix: "kerml-1.0",
    name: "KerML",
    slug: "kerml",
    version: "1.0",
    formal_document_id: "formal/26-03-01",
    xmi_file_id: "ptc/25-04-04",
    expected_sha256: "45b18775afe2b2fcdc70e24f37c6d2f344defcc3f38a02075a193354e2d7b466",
    expected_pdf_sha256: "3bcc96f989bfa9d05cd28e026df3351b795fe8d494187b87bff3db7d96373697",
};

pub const SYSML20_SPECIFICATION: PinnedSpecification = PinnedSpecification {
    specification_id: SpecificationId::SysML20,
    rule_id_prefix: "sysml-2.0",
    name: "SysML",
    slug: "sysml",
    version: "2.0",
    formal_document_id: "formal/26-03-02",
    xmi_file_id: "ptc/25-02-15",
    expected_sha256: "caa65d54f56798bf7582d173f7567e1eea37a49c45984f8bd7df145011cf8c6f",
    expected_pdf_sha256: "46e6c0476a6f1f34f367d57e039d56659bff75e41d2e4b3d37ca4cadea84a83a",
};

/// The official artifacts the committed manifest is permitted to describe.
pub const PINNED_SPECIFICATIONS: &[PinnedSpecification] =
    &[KERML10_SPECIFICATION, SYSML20_SPECIFICATION];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintManifest {
    pub schema_version: u32,
    pub specifications: Vec<SpecificationManifest>,
}

impl ConstraintManifest {
    pub fn load_toml(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        let manifest: Self = toml::from_str(&text)
            .map_err(|error| format!("{}: invalid manifest TOML: {error}", path.display()))?;
        if manifest.schema_version != SCHEMA_VERSION {
            return Err(format!(
                "{}: unsupported constraint manifest schema {}; expected {}",
                path.display(),
                manifest.schema_version,
                SCHEMA_VERSION
            ));
        }
        manifest.validate_pinned_inputs()?;
        manifest.validate_rule_identities()?;
        Ok(manifest)
    }

    /// Verifies the provenance fields that the refresh tool extracted from pinned official inputs.
    pub fn validate_pinned_inputs(&self) -> Result<(), String> {
        if self.specifications.len() != PINNED_SPECIFICATIONS.len() {
            return Err(format!(
                "constraint manifest has {} specifications; expected {} pinned specifications",
                self.specifications.len(),
                PINNED_SPECIFICATIONS.len()
            ));
        }
        for pinned in PINNED_SPECIFICATIONS {
            let Some(specification) = self.specifications.iter().find(|specification| {
                specification.name == pinned.name && specification.version == pinned.version
            }) else {
                return Err(format!(
                    "constraint manifest is missing pinned {} {}",
                    pinned.name, pinned.version
                ));
            };
            if specification.formal_document_id != pinned.formal_document_id
                || specification.xmi_file_id != pinned.xmi_file_id
                || specification.xmi_sha256 != pinned.expected_sha256
                || specification.pdf_sha256 != pinned.expected_pdf_sha256
            {
                return Err(format!(
                    "constraint manifest provenance for {} {} does not match the pinned official artifacts",
                    pinned.name, pinned.version
                ));
            }
        }
        Ok(())
    }

    pub fn find_rule(&self, rule_id: &str) -> Option<&ConstraintManifestEntry> {
        self.specifications
            .iter()
            .flat_map(|specification| &specification.constraints)
            .find(|entry| entry.rule_id == rule_id)
    }

    /// Returns a rule together with the pinned specification that owns its normative identity.
    /// Consumers use this instead of separately scanning specifications after a `find_rule`, so
    /// provenance and the rule cannot drift into two independent lookups.
    pub fn find_rule_with_specification(&self, rule_id: &str) -> Option<ManifestRule<'_>> {
        self.specifications.iter().find_map(|specification| {
            specification
                .constraints
                .iter()
                .find(|entry| entry.rule_id == rule_id)
                .map(|entry| ManifestRule {
                    specification,
                    entry,
                })
        })
    }

    /// Rejects ambiguous, duplicate, or display-name-derived rule identities. Consumers must
    /// match the manifest's complete stable identity, never fall back to a constraint name.
    pub fn validate_rule_identities(&self) -> Result<(), String> {
        let mut rule_ids = std::collections::BTreeSet::new();
        for specification in &self.specifications {
            let specification_prefix = specification
                .specification_id()
                .ok_or_else(|| {
                    format!(
                        "constraint manifest contains an unpinned specification {} {}",
                        specification.name, specification.version
                    )
                })?
                .as_str();
            for entry in &specification.constraints {
                let expected_rule_id = format!(
                    "{specification_prefix}:{}:{}",
                    entry.clause, entry.constraint
                );
                if entry.rule_id != expected_rule_id {
                    return Err(format!(
                        "constraint manifest rule_id {:?} must be the canonical identity {:?}",
                        entry.rule_id, expected_rule_id
                    ));
                }
                if ConstraintFamily::from_constraint_name(&entry.constraint) != Some(entry.family) {
                    return Err(format!(
                        "constraint manifest rule {:?} has family {:?} inconsistent with constraint name {:?}",
                        entry.rule_id, entry.family, entry.constraint
                    ));
                }
                if entry
                    .specializes_from_library
                    .as_ref()
                    .is_some_and(|contract| !valid_library_anchor(&contract.anchor))
                    || entry
                        .redefines_from_library
                        .as_ref()
                        .is_some_and(|contract| !valid_library_anchor(&contract.anchor))
                    || entry
                        .conditional_specializes_from_library
                        .as_ref()
                        .is_some_and(|contract| {
                            !valid_library_anchor(&contract.anchor)
                                || match contract.predicate {
                                    LibrarySpecializationPredicate::CompositeOwnedBy
                                    | LibrarySpecializationPredicate::OwnedBy => {
                                        contract.owner_metaclasses.len() != 2
                                            || contract
                                                .owner_metaclasses
                                                .iter()
                                                .any(|metaclass| !valid_library_anchor(metaclass))
                                    }
                                    LibrarySpecializationPredicate::PolarityBranch
                                    | LibrarySpecializationPredicate::HasElseActionBranch
                                    | LibrarySpecializationPredicate::RequirementConstraintMembershipKind
                                    | LibrarySpecializationPredicate::ActorMembershipOwningRequirement => {
                                        !contract.owner_metaclasses.is_empty()
                                            || contract
                                                .true_anchor
                                                .as_ref()
                                                .is_none_or(|anchor| !valid_library_anchor(anchor))
                                    }
                                    _ => {
                                        !contract.owner_metaclasses.is_empty()
                                            || contract.true_anchor.is_some()
                                    }
                                }
                        })
                {
                    return Err(format!(
                        "constraint manifest rule {:?} has an invalid exact library anchor",
                        entry.rule_id
                    ));
                }
                if let Some(contract) = &entry.feature_derived_relationship {
                    if entry.family != ConstraintFamily::Derive || entry.metaclass != "Feature" {
                        return Err(format!(
                            "constraint manifest rule {:?} has a Feature relationship derivation outside the exact Feature derive family",
                            entry.rule_id
                        ));
                    }
                    if !matches!(
                        (entry.constraint.as_str(), contract.kind),
                        (
                            "deriveFeatureOwnedFeatureChaining",
                            FeatureDerivedRelationshipKind::OwnedFeatureChaining
                        ) | (
                            "deriveFeatureOwnedRedefinition",
                            FeatureDerivedRelationshipKind::OwnedRedefinition
                        ) | (
                            "deriveFeatureOwnedSubsetting",
                            FeatureDerivedRelationshipKind::OwnedSubsetting
                        ) | (
                            "deriveFeatureOwnedTyping",
                            FeatureDerivedRelationshipKind::OwnedTyping
                        ) | (
                            "deriveFeatureOwnedTypeFeaturing",
                            FeatureDerivedRelationshipKind::OwnedTypeFeaturing
                        )
                    ) {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Feature relationship derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.type_derived_relationship {
                    if entry.family != ConstraintFamily::Derive || entry.metaclass != "Type" {
                        return Err(format!(
                            "constraint manifest rule {:?} has a Type relationship derivation outside the exact Type derive family",
                            entry.rule_id
                        ));
                    }
                    if !matches!(
                        (entry.constraint.as_str(), contract.kind),
                        (
                            "deriveTypeOwnedSpecialization",
                            TypeDerivedRelationshipKind::OwnedSpecialization
                        ) | (
                            "deriveTypeOwnedUnioning",
                            TypeDerivedRelationshipKind::OwnedUnioning
                        ) | (
                            "deriveTypeOwnedIntersecting",
                            TypeDerivedRelationshipKind::OwnedIntersecting
                        ) | (
                            "deriveTypeOwnedDifferencing",
                            TypeDerivedRelationshipKind::OwnedDifferencing
                        ) | (
                            "deriveTypeOwnedDisjoining",
                            TypeDerivedRelationshipKind::OwnedDisjoining
                        ) | (
                            "deriveTypeUnioningType",
                            TypeDerivedRelationshipKind::UnioningType
                        ) | (
                            "deriveTypeIntersectingType",
                            TypeDerivedRelationshipKind::IntersectingType
                        ) | (
                            "deriveTypeDifferencingType",
                            TypeDerivedRelationshipKind::DifferencingType
                        )
                    ) {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Type relationship derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.type_derived_element {
                    if entry.family != ConstraintFamily::Derive
                        || entry.metaclass != "Type"
                        || !matches!(
                            (entry.constraint.as_str(), contract.kind),
                            (
                                "deriveTypeOwnedFeature",
                                TypeDerivedElementKind::OwnedFeature
                            ) | (
                                "deriveTypeOwnedEndFeature",
                                TypeDerivedElementKind::OwnedEndFeature
                            )
                        )
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Type element derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.type_derived_fact {
                    let compatible = matches!(
                        (entry.constraint.as_str(), contract.kind),
                        (
                            "deriveTypeOwnedFeatureMembership",
                            TypeDerivedFactKind::OwnedFeatureMembership
                        ) | (
                            "deriveTypeFeatureMembership",
                            TypeDerivedFactKind::FeatureMembership
                        ) | ("deriveTypeFeature", TypeDerivedFactKind::Feature)
                            | ("deriveTypeEndFeature", TypeDerivedFactKind::EndFeature)
                            | (
                                "deriveTypeDirectedFeature",
                                TypeDerivedFactKind::DirectedFeature
                            )
                            | (
                                "deriveTypeInheritedMembership",
                                TypeDerivedFactKind::InheritedMembership
                            )
                            | (
                                "deriveTypeInheritedFeature",
                                TypeDerivedFactKind::InheritedFeature
                            )
                            | ("deriveTypeInput", TypeDerivedFactKind::Input)
                            | ("deriveTypeOutput", TypeDerivedFactKind::Output)
                            | ("deriveTypeMultiplicity", TypeDerivedFactKind::Multiplicity)
                            | (
                                "deriveTypeOwnedConjugator",
                                TypeDerivedFactKind::OwnedConjugator
                            )
                    );
                    if entry.family != ConstraintFamily::Derive
                        || entry.metaclass != "Type"
                        || !compatible
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Type fact derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.action_derived_fact {
                    let compatible = matches!(
                        (
                            entry.constraint.as_str(),
                            entry.metaclass.as_str(),
                            contract.kind
                        ),
                        (
                            "deriveActionDefinitionAction",
                            "ActionDefinition",
                            ActionDerivedFactKind::ActionDefinitionAction
                        ) | (
                            "deriveAssignmentActionUsageValueExpression",
                            "AssignmentActionUsage",
                            ActionDerivedFactKind::AssignmentValueExpression
                        ) | (
                            "deriveAssignmentUsageTargetArgument",
                            "AssignmentActionUsage",
                            ActionDerivedFactKind::AssignmentTargetArgument
                        ) | (
                            "deriveAssignmentActionUsageReferent",
                            "AssignmentActionUsage",
                            ActionDerivedFactKind::AssignmentReferent
                        ) | (
                            "deriveForLoopActionUsageLoopVariable",
                            "ForLoopActionUsage",
                            ActionDerivedFactKind::ForLoopVariable
                        ) | (
                            "deriveForLoopActionUsageSeqArgument",
                            "ForLoopActionUsage",
                            ActionDerivedFactKind::ForLoopSeqArgument
                        ) | (
                            "deriveLoopActionUsageBodyAction",
                            "LoopActionUsage",
                            ActionDerivedFactKind::LoopBodyAction
                        ) | (
                            "deriveTerminateActionUsageTerminatedOccurrenceArgument",
                            "TerminateActionUsage",
                            ActionDerivedFactKind::TerminateOccurrenceArgument
                        ) | (
                            "deriveAcceptActionUsagePayloadArgument",
                            "AcceptActionUsage",
                            ActionDerivedFactKind::AcceptPayloadArgument
                        ) | (
                            "deriveAcceptActionUsagePayloadParameter",
                            "AcceptActionUsage",
                            ActionDerivedFactKind::AcceptPayloadParameter
                        ) | (
                            "deriveAcceptActionUsageReceiverArgument",
                            "AcceptActionUsage",
                            ActionDerivedFactKind::AcceptReceiverArgument
                        ) | (
                            "deriveWhileLoopActionUsageWhileArgument",
                            "WhileLoopActionUsage",
                            ActionDerivedFactKind::WhileArgument
                        ) | (
                            "deriveWhileLoopActionUsageUntilArgument",
                            "WhileLoopActionUsage",
                            ActionDerivedFactKind::UntilArgument
                        ) | (
                            "deriveSendActionUsageSenderArgument",
                            "SendActionUsage",
                            ActionDerivedFactKind::SendSenderArgument
                        ) | (
                            "deriveSendActionUsageReceiverArgument",
                            "SendActionUsage",
                            ActionDerivedFactKind::SendReceiverArgument
                        ) | (
                            "deriveSendActionUsagePayloadArgument",
                            "SendActionUsage",
                            ActionDerivedFactKind::SendPayloadArgument
                        ) | (
                            "deriveIfActionUsageThenAction",
                            "IfActionUsage",
                            ActionDerivedFactKind::IfThenAction
                        ) | (
                            "deriveIfActionUsageElseAction",
                            "IfActionUsage",
                            ActionDerivedFactKind::IfElseAction
                        ) | (
                            "deriveIfActionUsageIfArgument",
                            "IfActionUsage",
                            ActionDerivedFactKind::IfArgument
                        )
                    );
                    if entry.family != ConstraintFamily::Derive
                        || !compatible
                        || !is_sha256_digest(&contract.body_sha256)
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Actions derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.requirement_derived_fact {
                    let compatible = matches!(
                        (
                            entry.constraint.as_str(),
                            entry.metaclass.as_str(),
                            contract.kind
                        ),
                        (
                            "deriveRequirementDefinitionActorParameter",
                            "RequirementDefinition",
                            RequirementDerivedFactKind::DefinitionActorParameter
                        ) | (
                            "deriveRequirementDefinitionSubjectParameter",
                            "RequirementDefinition",
                            RequirementDerivedFactKind::DefinitionSubjectParameter
                        ) | (
                            "deriveRequirementDefinitionText",
                            "RequirementDefinition",
                            RequirementDerivedFactKind::DefinitionText
                        ) | (
                            "deriveRequirementDefinitionRequiredConstraint",
                            "RequirementDefinition",
                            RequirementDerivedFactKind::DefinitionRequiredConstraint
                        ) | (
                            "deriveRequirementDefinitionAssumedConstraint",
                            "RequirementDefinition",
                            RequirementDerivedFactKind::DefinitionAssumedConstraint
                        ) | (
                            "deriveRequirementDefinitionFramedConcern",
                            "RequirementDefinition",
                            RequirementDerivedFactKind::DefinitionFramedConcern
                        ) | (
                            "deriveRequirementUsageActorParameter",
                            "RequirementUsage",
                            RequirementDerivedFactKind::UsageActorParameter
                        ) | (
                            "deriveRequirementUsageSubjectParameter",
                            "RequirementUsage",
                            RequirementDerivedFactKind::UsageSubjectParameter
                        ) | (
                            "deriveRequirementUsageText",
                            "RequirementUsage",
                            RequirementDerivedFactKind::UsageText
                        ) | (
                            "deriveRequirementUsageRequiredConstraint",
                            "RequirementUsage",
                            RequirementDerivedFactKind::UsageRequiredConstraint
                        ) | (
                            "deriveRequirementUsageAssumedConstraint",
                            "RequirementUsage",
                            RequirementDerivedFactKind::UsageAssumedConstraint
                        ) | (
                            "deriveRequirementUsageFramedConcern",
                            "RequirementUsage",
                            RequirementDerivedFactKind::UsageFramedConcern
                        )
                    );
                    if entry.family != ConstraintFamily::Derive
                        || !compatible
                        || !is_sha256_digest(&contract.body_sha256)
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Requirements fact derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.type_featuring_check {
                    if entry.family != ConstraintFamily::Check
                        || entry.metaclass != "Feature"
                        || !matches!(
                            (entry.constraint.as_str(), contract.kind),
                            (
                                "checkFeatureFeatureMembershipTypeFeaturing",
                                TypeFeaturingCheckKind::FeatureFeatureMembership
                            )
                        )
                        || !is_sha256_digest(&contract.body_sha256)
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact TypeFeaturing check",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.redefinition_check {
                    if entry.family != ConstraintFamily::Check
                        || !matches!(
                            (
                                entry.constraint.as_str(),
                                entry.metaclass.as_str(),
                                contract.kind
                            ),
                            (
                                "checkFeatureEndRedefinition",
                                "Feature",
                                RedefinitionCheckKind::FeatureEnd
                            ) | (
                                "checkFeatureFlowFeatureRedefinition",
                                "Feature",
                                RedefinitionCheckKind::FeatureFlowFeature
                            ) | (
                                "checkFeatureOwnedCrossFeatureRedefinitionSpecialization",
                                "Feature",
                                RedefinitionCheckKind::FeatureOwnedCrossFeatureSpecialization
                            ) | (
                                "checkFeatureParameterRedefinition",
                                "Feature",
                                RedefinitionCheckKind::FeatureParameter
                            ) | (
                                "checkFeatureResultRedefinition",
                                "Feature",
                                RedefinitionCheckKind::FeatureResult
                            ) | (
                                "checkConstructorExpressionResultFeatureRedefinition",
                                "ConstructorExpression",
                                RedefinitionCheckKind::ConstructorExpressionResultFeature
                            ) | (
                                "checkFeatureChainExpressionSourceTargetRedefinition",
                                "FeatureChainExpression",
                                RedefinitionCheckKind::FeatureChainExpressionSourceTarget
                            ) | (
                                "checkFeatureChainExpressionTargetRedefinition",
                                "FeatureChainExpression",
                                RedefinitionCheckKind::FeatureChainExpressionTarget
                            ) | (
                                "checkActionUsageStateActionRedefinition",
                                "ActionUsage",
                                RedefinitionCheckKind::ActionUsageStateAction
                            ) | (
                                "checkAssignmentActionUsageAccessedFeatureRedefinition",
                                "AssignmentActionUsage",
                                RedefinitionCheckKind::AssignmentActionUsageAccessedFeature
                            ) | (
                                "checkAssignmentActionUsageReferentRedefinition",
                                "AssignmentActionUsage",
                                RedefinitionCheckKind::AssignmentActionUsageReferent
                            ) | (
                                "checkAssignmentActionUsageStartingAtRedefinition",
                                "AssignmentActionUsage",
                                RedefinitionCheckKind::AssignmentActionUsageStartingAt
                            ) | (
                                "checkForLoopActionUsageVarRedefinition",
                                "ForLoopActionUsage",
                                RedefinitionCheckKind::ForLoopActionUsageVar
                            ) | (
                                "checkRequirementUsageObjectiveRedefinition",
                                "RequirementUsage",
                                RedefinitionCheckKind::RequirementUsageObjective
                            ) | (
                                "checkRenderingUsageRedefinition",
                                "RenderingUsage",
                                RedefinitionCheckKind::RenderingUsage
                            )
                        )
                        || !is_sha256_digest(&contract.body_sha256)
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact redefinition check",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.specialization_check {
                    let compatible = matches!(
                        (
                            entry.constraint.as_str(),
                            entry.metaclass.as_str(),
                            contract.kind
                        ),
                        (
                            "checkFeatureCrossingSpecialization",
                            "Feature",
                            SpecializationCheckKind::FeatureCrossing
                        ) | (
                            "checkFeatureObjectSpecialization",
                            "Feature",
                            SpecializationCheckKind::FeatureObject
                        ) | (
                            "checkFeatureOccurrenceSpecialization",
                            "Feature",
                            SpecializationCheckKind::FeatureOccurrence
                        ) | (
                            "checkFeatureOwnedCrossFeatureSpecialization",
                            "Feature",
                            SpecializationCheckKind::FeatureOwnedCrossFeature
                        ) | (
                            "checkFeaturePortionSpecialization",
                            "Feature",
                            SpecializationCheckKind::FeaturePortion
                        ) | (
                            "checkFeatureSubobjectSpecialization",
                            "Feature",
                            SpecializationCheckKind::FeatureSubobject
                        ) | (
                            "checkFeatureSuboccurrenceSpecialization",
                            "Feature",
                            SpecializationCheckKind::FeatureSuboccurrence
                        ) | (
                            "checkFeatureValuationSpecialization",
                            "Feature",
                            SpecializationCheckKind::FeatureValuation
                        ) | (
                            "checkMetadataFeatureSemanticSpecialization",
                            "MetadataFeature",
                            SpecializationCheckKind::MetadataFeatureSemantic
                        ) | (
                            "checkConnectorBinaryObjectSpecialization",
                            "Connector",
                            SpecializationCheckKind::ConnectorBinaryObject
                        ) | (
                            "checkConnectorObjectSpecialization",
                            "Connector",
                            SpecializationCheckKind::ConnectorObject
                        ) | (
                            "checkStepOwnedPerformanceSpecialization",
                            "Step",
                            SpecializationCheckKind::StepOwnedPerformance
                        ) | (
                            "checkStepSubperformanceSpecialization",
                            "Step",
                            SpecializationCheckKind::StepSubperformance
                        ) | (
                            "checkSelectExpressionResultSpecialization",
                            "SelectExpression",
                            SpecializationCheckKind::SelectExpressionResult
                        ) | (
                            "checkConstructorExpressionResultSpecialization",
                            "ConstructorExpression",
                            SpecializationCheckKind::ConstructorExpressionResult
                        ) | (
                            "checkConstructorExpressionSpecialization",
                            "ConstructorExpression",
                            SpecializationCheckKind::ConstructorExpression
                        ) | (
                            "checkFeatureChainExpressionResultSpecialization",
                            "FeatureChainExpression",
                            SpecializationCheckKind::FeatureChainExpressionResult
                        ) | (
                            "checkFeatureReferenceExpressionResultSpecialization",
                            "FeatureReferenceExpression",
                            SpecializationCheckKind::FeatureReferenceExpressionResult
                        ) | (
                            "checkIndexExpressionResultSpecialization",
                            "IndexExpression",
                            SpecializationCheckKind::IndexExpressionResult
                        ) | (
                            "checkInvocationExpressionBehaviorResultSpecialization",
                            "InvocationExpression",
                            SpecializationCheckKind::InvocationExpressionBehaviorResult
                        ) | (
                            "checkInvocationExpressionSpecialization",
                            "InvocationExpression",
                            SpecializationCheckKind::InvocationExpression
                        ) | (
                            "checkMergeNodeIncomingSuccessionSpecialization",
                            "MergeNode",
                            SpecializationCheckKind::MergeNodeIncomingSuccession
                        ) | (
                            "checkDecisionNodeOutgoingSuccessionSpecialization",
                            "DecisionNode",
                            SpecializationCheckKind::DecisionNodeOutgoingSuccession
                        ) | (
                            "checkStateUsageExclusiveStateSpecialization",
                            "StateUsage",
                            SpecializationCheckKind::StateUsageExclusiveState
                        ) | (
                            "checkStateUsageSubstateSpecialization",
                            "StateUsage",
                            SpecializationCheckKind::StateUsageSubstate
                        ) | (
                            "checkTransitionUsageActionSpecialization",
                            "TransitionUsage",
                            SpecializationCheckKind::TransitionUsageAction
                        ) | (
                            "checkTransitionUsagePayloadSpecialization",
                            "TransitionUsage",
                            SpecializationCheckKind::TransitionUsagePayload
                        ) | (
                            "checkTransitionUsageStateSpecialization",
                            "TransitionUsage",
                            SpecializationCheckKind::TransitionUsageState
                        ) | (
                            "checkTransitionUsageSuccessionSourceSpecialization",
                            "TransitionUsage",
                            SpecializationCheckKind::TransitionUsageSuccessionSource
                        ) | (
                            "checkTransitionUsageTransitionFeatureSpecialization",
                            "TransitionUsage",
                            SpecializationCheckKind::TransitionUsageTransitionFeature
                        ) | (
                            "checkIncludeUseCaseSpecialization",
                            "IncludeUseCaseUsage",
                            SpecializationCheckKind::IncludeUseCase
                        ) | (
                            "checkUsageVariationDefinitionSpecialization",
                            "Usage",
                            SpecializationCheckKind::UsageVariationDefinition
                        ) | (
                            "checkUsageVariationUsageSpecialization",
                            "Usage",
                            SpecializationCheckKind::UsageVariationUsage
                        ) | (
                            "checkOccurrenceDefinitionMultiplicitySpecialization",
                            "OccurrenceDefinition",
                            SpecializationCheckKind::OccurrenceDefinitionMultiplicity
                        ) | (
                            "checkOccurrenceUsageSuboccurrenceSpecialization",
                            "OccurrenceUsage",
                            SpecializationCheckKind::OccurrenceUsageSuboccurrence
                        )
                    );
                    if entry.family != ConstraintFamily::Check
                        || !compatible
                        || !is_sha256_digest(&contract.body_sha256)
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact specialization check",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.element_derived_owner {
                    if entry.family != ConstraintFamily::Derive
                        || entry.metaclass != "Element"
                        || !matches!(
                            (entry.constraint.as_str(), contract.kind),
                            ("deriveElementOwner", ElementDerivedOwnerKind::Owner)
                        )
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Element owner derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.element_derived_documentation {
                    if entry.family != ConstraintFamily::Derive
                        || entry.metaclass != "Element"
                        || !matches!(
                            (entry.constraint.as_str(), contract.kind),
                            (
                                "deriveElementDocumentation",
                                ElementDerivedDocumentationKind::Documentation
                            ) | (
                                "deriveElementTextualRepresentation",
                                ElementDerivedDocumentationKind::TextualRepresentation
                            )
                        )
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Element documentation derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.namespace_derived_element {
                    if entry.family != ConstraintFamily::Derive
                        || entry.metaclass != "Namespace"
                        || !matches!(
                            (entry.constraint.as_str(), contract.kind),
                            (
                                "deriveNamespaceOwnedMember",
                                NamespaceDerivedElementKind::OwnedMember
                            ) | (
                                "deriveNamespaceOwnedImport",
                                NamespaceDerivedElementKind::OwnedImport
                            )
                        )
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact Namespace element derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.namespace_import_derived_element {
                    if entry.family != ConstraintFamily::Derive
                        || entry.metaclass != "NamespaceImport"
                        || !matches!(
                            (entry.constraint.as_str(), contract.kind),
                            (
                                "deriveNamespaceImportImportedElement",
                                NamespaceImportDerivedElementKind::ImportedElement
                            )
                        )
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact NamespaceImport element derivation",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.binding_connector_check {
                    let valid_kind = matches!(
                        (
                            entry.constraint.as_str(),
                            entry.metaclass.as_str(),
                            contract.kind
                        ),
                        (
                            "checkFeatureValueBindingConnector",
                            "FeatureValue",
                            BindingConnectorCheckKind::FeatureValue
                        ) | (
                            "checkExpressionResultBindingConnector",
                            "Expression",
                            BindingConnectorCheckKind::ExpressionResult
                        ) | (
                            "checkFunctionResultBindingConnector",
                            "Function",
                            BindingConnectorCheckKind::FunctionResult
                        ) | (
                            "checkConstructorExpressionResultDefaultValueBindingConnector",
                            "ConstructorExpression",
                            BindingConnectorCheckKind::ConstructorExpressionResultDefaultValueTbd
                        ) | (
                            "checkFeatureReferenceExpressionBindingConnector",
                            "FeatureReferenceExpression",
                            BindingConnectorCheckKind::FeatureReferenceExpression
                        ) | (
                            "checkInvocationExpressionBehaviorBindingConnector",
                            "InvocationExpression",
                            BindingConnectorCheckKind::InvocationExpressionBehavior
                        ) | (
                            "checkInvocationExpressionDefaultValueBindingConnector",
                            "InvocationExpression",
                            BindingConnectorCheckKind::InvocationExpressionDefaultValueTbd
                        ) | (
                            "checkAcceptActionUsageReceiverBindingConnector",
                            "AcceptActionUsage",
                            BindingConnectorCheckKind::AcceptActionUsageReceiver
                        ) | (
                            "checkTransitionUsageSourceBindingConnector",
                            "TransitionUsage",
                            BindingConnectorCheckKind::TransitionUsageSource
                        ) | (
                            "checkTransitionUsageSuccessionBindingConnector",
                            "TransitionUsage",
                            BindingConnectorCheckKind::TransitionUsageSuccession
                        ) | (
                            "checkSatisfyRequirementUsageBindingConnector",
                            "SatisfyRequirementUsage",
                            BindingConnectorCheckKind::SatisfyRequirementUsage
                        )
                    );
                    if entry.family != ConstraintFamily::Check
                        || !valid_kind
                        || !is_sha256_digest(&contract.body_sha256)
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible exact BindingConnector check contract",
                            entry.rule_id
                        ));
                    }
                }
                if let Some(contract) = &entry.definition_usage_derived {
                    let definition_kind = matches!(
                        contract.kind,
                        DefinitionUsageDerivedKind::DefinitionDirectedUsage
                            | DefinitionUsageDerivedKind::DefinitionOwnedAction
                            | DefinitionUsageDerivedKind::DefinitionOwnedAllocation
                            | DefinitionUsageDerivedKind::DefinitionOwnedAnalysisCase
                            | DefinitionUsageDerivedKind::DefinitionOwnedAttribute
                            | DefinitionUsageDerivedKind::DefinitionOwnedCalculation
                            | DefinitionUsageDerivedKind::DefinitionOwnedCase
                            | DefinitionUsageDerivedKind::DefinitionOwnedConcern
                            | DefinitionUsageDerivedKind::DefinitionOwnedConnection
                            | DefinitionUsageDerivedKind::DefinitionOwnedConstraint
                            | DefinitionUsageDerivedKind::DefinitionOwnedEnumeration
                            | DefinitionUsageDerivedKind::DefinitionOwnedFlow
                            | DefinitionUsageDerivedKind::DefinitionOwnedInterface
                            | DefinitionUsageDerivedKind::DefinitionOwnedItem
                            | DefinitionUsageDerivedKind::DefinitionOwnedMetadata
                            | DefinitionUsageDerivedKind::DefinitionOwnedOccurrence
                            | DefinitionUsageDerivedKind::DefinitionOwnedPart
                            | DefinitionUsageDerivedKind::DefinitionOwnedPort
                            | DefinitionUsageDerivedKind::DefinitionOwnedReference
                            | DefinitionUsageDerivedKind::DefinitionOwnedRendering
                            | DefinitionUsageDerivedKind::DefinitionOwnedRequirement
                            | DefinitionUsageDerivedKind::DefinitionOwnedState
                            | DefinitionUsageDerivedKind::DefinitionOwnedTransition
                            | DefinitionUsageDerivedKind::DefinitionOwnedUsage
                            | DefinitionUsageDerivedKind::DefinitionOwnedUseCase
                            | DefinitionUsageDerivedKind::DefinitionOwnedVerificationCase
                            | DefinitionUsageDerivedKind::DefinitionOwnedView
                            | DefinitionUsageDerivedKind::DefinitionOwnedViewpoint
                            | DefinitionUsageDerivedKind::DefinitionUsage
                            | DefinitionUsageDerivedKind::DefinitionVariant
                            | DefinitionUsageDerivedKind::DefinitionVariantMembership
                    );
                    let expected_metaclass = if definition_kind {
                        "Definition"
                    } else {
                        "Usage"
                    };
                    let expected_prefix = if definition_kind {
                        "deriveDefinition"
                    } else {
                        "deriveUsage"
                    };
                    if entry.family != ConstraintFamily::Derive
                        || entry.package != "Systems::DefinitionAndUsage"
                        || entry.metaclass != expected_metaclass
                        || !entry.constraint.starts_with(expected_prefix)
                        || !contract.is_exact_pinned_contract(&entry.constraint)
                    {
                        return Err(format!(
                            "constraint manifest rule {:?} has an incompatible Definition/Usage derivation contract",
                            entry.rule_id
                        ));
                    }
                }
                if !rule_ids.insert(entry.rule_id.clone()) {
                    return Err(format!(
                        "constraint manifest contains duplicate rule_id {:?}",
                        entry.rule_id
                    ));
                }
            }
        }
        Ok(())
    }
}

/// One manifest rule and the pinned specification that owns it.
#[derive(Debug, Clone, Copy)]
pub struct ManifestRule<'a> {
    pub specification: &'a SpecificationManifest,
    pub entry: &'a ConstraintManifestEntry,
}

impl ManifestRule<'_> {
    /// The closed specification identity that owns this rule and its formal document pin.
    ///
    /// Consumers use this projection instead of reparsing rule-ID prefixes or maintaining their
    /// own specification-to-document table.
    pub fn specification_id(&self) -> Option<SpecificationId> {
        self.specification.specification_id()
    }
}

impl SpecificationId {
    pub fn parse(value: &str) -> Option<Self> {
        PINNED_SPECIFICATIONS
            .iter()
            .find(|pinned| pinned.rule_id_prefix == value)
            .map(|pinned| pinned.specification_id)
    }

    pub fn from_rule_id(rule_id: &str) -> Option<Self> {
        rule_id
            .split_once(':')
            .and_then(|(prefix, _)| Self::parse(prefix))
    }

    pub const fn as_str(self) -> &'static str {
        self.pinned().rule_id_prefix
    }

    pub const fn name_version(self) -> (&'static str, &'static str) {
        (self.pinned().name, self.pinned().version)
    }

    pub const fn formal_document_id(self) -> &'static str {
        self.pinned().formal_document_id
    }

    /// The manifest-owned provenance record for this closed specification identity.
    pub const fn pinned(self) -> &'static PinnedSpecification {
        match self {
            Self::KerML10 => &KERML10_SPECIFICATION,
            Self::SysML20 => &SYSML20_SPECIFICATION,
        }
    }
}

impl SpecificationManifest {
    pub fn specification_id(&self) -> Option<SpecificationId> {
        PINNED_SPECIFICATIONS
            .iter()
            .find(|pinned| pinned.name == self.name && pinned.version == self.version)
            .map(|pinned| pinned.specification_id)
    }
}

fn valid_library_anchor(anchor: &str) -> bool {
    !anchor.is_empty()
        && anchor.split("::").all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '_')
        })
}

fn is_sha256_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecificationManifest {
    pub name: String,
    pub version: String,
    pub formal_document_id: String,
    pub xmi_file_id: String,
    pub xmi_sha256: String,
    pub pdf_sha256: String,
    pub constraints: Vec<ConstraintManifestEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintManifestEntry {
    pub package: String,
    pub metaclass: String,
    pub constraint: String,
    pub family: ConstraintFamily,
    /// Exact normative abstract-syntax clause heading from the pinned official PDF.
    pub clause: String,
    /// Present only when the complete pinned XMI body is exactly
    /// `specializesFromLibrary('qualified::anchor')`.
    pub specializes_from_library: Option<LibrarySpecializationContract>,
    /// Present only when the pinned XMI body has one of the closed, lossless conditional
    /// `specializesFromLibrary` shapes supported by this manifest contract.
    pub conditional_specializes_from_library: Option<ConditionalLibrarySpecializationContract>,
    /// Present only when the complete pinned XMI body is exactly
    /// `redefinesFromLibrary('qualified::anchor')`.
    pub redefines_from_library: Option<LibraryRedefinitionContract>,
    /// Present only when the complete pinned XMI body is one closed relationship-collection
    /// derivation over `Feature`'s already-owned canonical relationship facts.
    pub feature_derived_relationship: Option<FeatureDerivedRelationshipContract>,
    /// Present only when the complete pinned XMI body is one closed relationship-collection or
    /// operand projection over `Type`'s already-owned canonical relationship facts.
    pub type_derived_relationship: Option<TypeDerivedRelationshipContract>,
    /// Present only for a complete, exact `Type` element-valued projection whose output can be
    /// read from canonical declaration ownership and membership facts without materializing a
    /// membership relationship identity.
    pub type_derived_element: Option<TypeDerivedElementContract>,
    /// Present only for a complete exact Type derivation whose result shape is known but whose
    /// first canonical fact prerequisite is not yet published.
    pub type_derived_fact: Option<TypeDerivedFactContract>,
    /// Present only for a complete named TypeFeaturing check whose full predicate is answered
    /// by the canonical FeatureMembership and effective TypeFeaturing fact families.
    pub type_featuring_check: Option<TypeFeaturingCheckContract>,
    /// Present only for one of the complete named redefinition checks whose pinned XMI body is
    /// represented by the closed, rule-scoped contract below. The contract does not interpret
    /// OCL: it selects the resolver-owned prerequisite boundary for that exact rule only.
    pub redefinition_check: Option<RedefinitionCheckContract>,
    /// Present only for one complete named specialization check body whose exact predicate is
    /// selected by a closed manifest-owned kind.  The kind identifies the full predicate; it is
    /// never a claim that an arbitrary specialization edge alone satisfies it.
    pub specialization_check: Option<SpecializationCheckContract>,
    /// Present only when the complete pinned XMI body is the closed `Element::owner`
    /// derivation over the canonical declaration-owner fact.
    pub element_derived_owner: Option<ElementDerivedOwnerContract>,
    /// Present only when the complete pinned XMI body selects one of the forms represented by
    /// the canonical `Documentation` fact collection.
    pub element_derived_documentation: Option<ElementDerivedDocumentationContract>,
    /// Present only for a closed Namespace collection derivation whose every input is an owned
    /// canonical declaration/membership fact.
    pub namespace_derived_element: Option<NamespaceDerivedElementContract>,
    /// Present only for a closed NamespaceImport scalar projection whose authored import
    /// reference and its resolution outcome are canonical facts.
    pub namespace_import_derived_element: Option<NamespaceImportDerivedElementContract>,
    /// Present only for a complete, named BindingConnector check body in pinned XMI.
    pub binding_connector_check: Option<BindingConnectorCheckContract>,
    /// Present only for one of the complete Systems::DefinitionAndUsage derivations whose
    /// complete pinned body selects a canonical direct-owner/member/modifier projection.
    ///
    /// The closed kind names the full normative property, rather than merely the selected
    /// declaration kind.  In particular, no query consumer gets to infer Definition-versus-Usage
    /// applicability or turn a direct ownership scan into the broader inherited `feature` fact.
    pub definition_usage_derived: Option<DefinitionUsageDerivedContract>,
    /// Present only for a complete Systems::Actions derivation whose exact pinned body is mapped
    /// to one closed canonical action-role result or an explicit first missing prerequisite.
    /// This is deliberately separate from Definition/Usage: ActionDefinition::action reads the
    /// broader inherited `usage` property, while the other rules depend on ordered action
    /// arguments, input parameters, or anonymous action identities.
    pub action_derived_fact: Option<ActionDerivedFactContract>,
    /// Present only for complete Systems::Requirements derivations whose exact pinned body is
    /// satisfied by one canonical membership-role or documentation projection.
    pub requirement_derived_fact: Option<RequirementDerivedFactContract>,
    pub rule_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibrarySpecializationContract {
    pub anchor: String,
}

/// A conditional library specialization contract extracted without interpreting general OCL.
///
/// Each predicate corresponds to one complete XMI body shape. Extending this enum requires an
/// exact extractor case and an owning semantic fact; callers may not reconstruct a predicate
/// from a constraint name or a rendered comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibrarySpecializationPredicate {
    IsIndividual,
    PortionKindSnapshot,
    PortionKindTimeslice,
    /// `isComposite and owningType <> null and (owningType.oclIsKindOf(A) or
    /// owningType.oclIsKindOf(B))` for the exact pair published in
    /// [`ConditionalLibrarySpecializationContract::owner_metaclasses`].
    CompositeOwnedBy,
    /// `ownedEndFeature->size() = 2`.
    OwnedEndFeatureCountIsTwo,
    /// `connectorEnd->size() = 2`.
    ConnectorEndCountIsTwo,
    /// `associationEnd->size() = 2`.
    AssociationEndCountIsTwo,
    /// `endFeature->size() = 2`.
    EndFeatureCountIsTwo,
    /// `flowEnd->size() = 2`.
    FlowEndCountIsTwo,
    /// `ownedEndFeatures->notEmpty()`.
    OwnedEndFeaturesNotEmpty,
    /// `ownedTyping.type->exists(selectByKind(DataType))`.
    OwnedTypingDataType,
    /// `isEnd and owningType` is either an `Association` or `Connector`.
    EndOwnedByAssociationOrConnector,
    /// `association->exists(oclIsKindOf(AssociationStructure))` on a `Connector`.
    ConnectorAssociationStructure,
    /// `owningType <> null and (owningType.oclIsKindOf(A) or owningType.oclIsKindOf(B))`.
    OwnedBy,
    /// `isSubactionUsage()`.
    IsSubactionUsage,
    /// `not isTriggerAction()` for an `AcceptActionUsage`.
    IsNotTriggerAction,
    /// `isSubactionUsage() and not isTriggerAction()` for an `AcceptActionUsage`.
    IsSubactionUsageAndNotTriggerAction,
    /// `isTriggerAction()` for an `AcceptActionUsage`.
    IsTriggerAction,
    /// `if elseAction = null then specializesFromLibrary(A) else
    /// specializesFromLibrary(B) endif`, with the predicate-true anchor `B`.
    HasElseActionBranch,
    /// `if isNegated then specializesFromLibrary(A) else specializesFromLibrary(B) endif`.
    PolarityBranch,
    /// A `ConcernUsage` owned through `FramedConcernMembership`.
    FramedConcernMembership,
    /// A `ConstraintUsage` owned through `RequirementConstraintMembership`, branching on its
    /// exact `assumption` kind.
    RequirementConstraintMembershipKind,
    /// A `PartUsage` owned through `ActorMembership`, branching on whether its owning type is a
    /// requirement definition or usage.
    ActorMembershipOwningRequirement,
    /// A `PartUsage` owned through `StakeholderMembership`.
    StakeholderMembership,
    /// A `RequirementUsage` owned through `RequirementVerificationMembership`.
    RequirementVerificationMembership,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionalLibrarySpecializationContract {
    pub predicate: LibrarySpecializationPredicate,
    /// The two exact owning-type metaclasses for
    /// [`LibrarySpecializationPredicate::CompositeOwnedBy`] or
    /// [`LibrarySpecializationPredicate::OwnedBy`].
    /// Empty for predicates that do not inspect the owner.
    #[serde(default)]
    pub owner_metaclasses: Vec<String>,
    /// Anchor selected when the exact predicate's `then` branch is true. Present only for a
    /// closed branching predicate.
    #[serde(default)]
    pub true_anchor: Option<String>,
    pub anchor: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryRedefinitionContract {
    pub anchor: String,
}

/// The exact relationship collection a Feature derivation selects.
///
/// This enum is intentionally closed. It is not an OCL interpreter: each value corresponds to a
/// complete body matched byte-for-byte by the extractor and to one typed relationship family the
/// semantic publication already owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureDerivedRelationshipKind {
    OwnedFeatureChaining,
    OwnedRedefinition,
    OwnedSubsetting,
    OwnedTyping,
    OwnedTypeFeaturing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureDerivedRelationshipContract {
    pub kind: FeatureDerivedRelationshipKind,
}

/// Exact relationship collections and operand projections owned by KerML `Type`.
///
/// Operand variants retain the canonical relationship carrying the operand, so provenance and
/// unresolved target state are not collapsed into a target-name projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeDerivedRelationshipKind {
    OwnedSpecialization,
    OwnedUnioning,
    OwnedIntersecting,
    OwnedDifferencing,
    OwnedDisjoining,
    UnioningType,
    IntersectingType,
    DifferencingType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDerivedRelationshipContract {
    pub kind: TypeDerivedRelationshipKind,
}

/// The exact `Element` derived scalar this manifest currently admits.
///
/// This remains a closed one-variant contract rather than an open derived-property table: the
/// extractor may add another value only after the semantic publication owns that property's full
/// fact and outcome behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementDerivedOwnerKind {
    Owner,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementDerivedOwnerContract {
    pub kind: ElementDerivedOwnerKind,
}

/// The closed `Element` documentation-form projections the semantic publication owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementDerivedDocumentationKind {
    Documentation,
    TextualRepresentation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementDerivedDocumentationContract {
    pub kind: ElementDerivedDocumentationKind,
}

/// The closed Namespace projections the canonical declaration and membership facts can answer.
///
/// `ownedMembership` itself remains unrepresented as a first-class relationship identity, so
/// this deliberately admits only the final element-valued projections, not a lossy stand-in for
/// the relationship collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamespaceDerivedElementKind {
    OwnedMember,
    OwnedImport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamespaceDerivedElementContract {
    pub kind: NamespaceDerivedElementKind,
}

/// The closed Type element-valued derivations the canonical ownership and membership facts can
/// answer without synthesizing a Membership relationship object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeDerivedElementKind {
    OwnedFeature,
    OwnedEndFeature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDerivedElementContract {
    pub kind: TypeDerivedElementKind,
}

/// Closed exact Type derivations whose canonical fact owner is not yet available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeDerivedFactKind {
    OwnedFeatureMembership,
    FeatureMembership,
    Feature,
    EndFeature,
    DirectedFeature,
    InheritedMembership,
    InheritedFeature,
    Input,
    Output,
    Multiplicity,
    OwnedConjugator,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDerivedFactContract {
    pub kind: TypeDerivedFactKind,
}

/// Closed TypeFeaturing checks with all applicability facts already owned by semantic
/// publication. This is deliberately not a classifier for the other TypeFeaturing predicates:
/// those require endpoint or owner facts that are not yet canonical query inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeFeaturingCheckKind {
    FeatureFeatureMembership,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeFeaturingCheckContract {
    pub kind: TypeFeaturingCheckKind,
    /// SHA-256 of the unmodified pinned XMI OCL body before matcher normalization.
    pub body_sha256: String,
}

/// Complete named redefinition check bodies selected from pinned KerML/SysML XMI.
///
/// Each variant has one exact normative body digest. The resolver may decide a predicate only
/// after its complete applicability and endpoint facts are published; otherwise it publishes the
/// first missing prerequisite explicitly instead of deriving relationships from names or output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedefinitionCheckKind {
    FeatureEnd,
    FeatureFlowFeature,
    FeatureOwnedCrossFeatureSpecialization,
    FeatureParameter,
    FeatureResult,
    ConstructorExpressionResultFeature,
    FeatureChainExpressionSourceTarget,
    FeatureChainExpressionTarget,
    ActionUsageStateAction,
    AssignmentActionUsageAccessedFeature,
    AssignmentActionUsageReferent,
    AssignmentActionUsageStartingAt,
    ForLoopActionUsageVar,
    RequirementUsageObjective,
    RenderingUsage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedefinitionCheckContract {
    pub kind: RedefinitionCheckKind,
    /// SHA-256 of the unmodified pinned XMI OCL body before matcher normalization.
    pub body_sha256: String,
}

/// Complete named specialization checks that are not already represented by the closed
/// `specializesFromLibrary` contracts.  Every variant corresponds to one exact pinned XMI body;
/// the resolver must retain an explicit prerequisite outcome until all of that body's role and
/// endpoint facts are published.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecializationCheckKind {
    FeatureCrossing,
    FeatureObject,
    FeatureOccurrence,
    FeatureOwnedCrossFeature,
    FeaturePortion,
    FeatureSubobject,
    FeatureSuboccurrence,
    FeatureValuation,
    MetadataFeatureSemantic,
    ConnectorBinaryObject,
    ConnectorObject,
    StepOwnedPerformance,
    StepSubperformance,
    SelectExpressionResult,
    ConstructorExpressionResult,
    ConstructorExpression,
    FeatureChainExpressionResult,
    FeatureReferenceExpressionResult,
    IndexExpressionResult,
    InvocationExpressionBehaviorResult,
    InvocationExpression,
    MergeNodeIncomingSuccession,
    DecisionNodeOutgoingSuccession,
    StateUsageExclusiveState,
    StateUsageSubstate,
    TransitionUsageAction,
    TransitionUsagePayload,
    TransitionUsageState,
    TransitionUsageSuccessionSource,
    TransitionUsageTransitionFeature,
    IncludeUseCase,
    UsageVariationDefinition,
    UsageVariationUsage,
    OccurrenceDefinitionMultiplicity,
    OccurrenceUsageSuboccurrence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecializationCheckContract {
    pub kind: SpecializationCheckKind,
    /// SHA-256 of the unmodified pinned XMI OCL body before matcher normalization.
    pub body_sha256: String,
}

/// The closed scalar `NamespaceImport` projection the semantic publication can expose without
/// re-parsing import syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamespaceImportDerivedElementKind {
    ImportedElement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamespaceImportDerivedElementContract {
    pub kind: NamespaceImportDerivedElementKind,
}

/// Complete named BindingConnector check bodies selected from pinned KerML/SysML XMI.
///
/// This is a rule-body classification, not a general OCL interpreter. The resolver may answer a
/// variant only from canonical paired binding facts and explicitly published endpoint facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingConnectorCheckKind {
    FeatureValue,
    ExpressionResult,
    FunctionResult,
    ConstructorExpressionResultDefaultValueTbd,
    FeatureReferenceExpression,
    InvocationExpressionBehavior,
    InvocationExpressionDefaultValueTbd,
    AcceptActionUsageReceiver,
    TransitionUsageSource,
    TransitionUsageSuccession,
    SatisfyRequirementUsage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingConnectorCheckContract {
    pub kind: BindingConnectorCheckKind,
    /// SHA-256 of the unmodified pinned XMI OCL body before matcher normalization.
    pub body_sha256: String,
}

/// Complete Systems::DefinitionAndUsage derivations selected by exact pinned-body matching.
///
/// This is deliberately a property-level vocabulary: `Definition::ownedAction` and
/// `Usage::nestedAction` have a shared element-kind filter but distinct normative source
/// collections, so collapsing them into a generic "action usage" operation would lose the
/// contract boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefinitionUsageDerivedKind {
    DefinitionDirectedUsage,
    DefinitionOwnedAction,
    DefinitionOwnedAllocation,
    DefinitionOwnedAnalysisCase,
    DefinitionOwnedAttribute,
    DefinitionOwnedCalculation,
    DefinitionOwnedCase,
    DefinitionOwnedConcern,
    DefinitionOwnedConnection,
    DefinitionOwnedConstraint,
    DefinitionOwnedEnumeration,
    DefinitionOwnedFlow,
    DefinitionOwnedInterface,
    DefinitionOwnedItem,
    DefinitionOwnedMetadata,
    DefinitionOwnedOccurrence,
    DefinitionOwnedPart,
    DefinitionOwnedPort,
    DefinitionOwnedReference,
    DefinitionOwnedRendering,
    DefinitionOwnedRequirement,
    DefinitionOwnedState,
    DefinitionOwnedTransition,
    DefinitionOwnedUsage,
    DefinitionOwnedUseCase,
    DefinitionOwnedVerificationCase,
    DefinitionOwnedView,
    DefinitionOwnedViewpoint,
    DefinitionUsage,
    DefinitionVariant,
    DefinitionVariantMembership,
    UsageDirectedUsage,
    UsageIsReference,
    UsageMayTimeVary,
    UsageNestedAction,
    UsageNestedAllocation,
    UsageNestedAnalysisCase,
    UsageNestedAttribute,
    UsageNestedCalculation,
    UsageNestedCase,
    UsageNestedConcern,
    UsageNestedConnection,
    UsageNestedConstraint,
    UsageNestedEnumeration,
    UsageNestedFlow,
    UsageNestedInterface,
    UsageNestedItem,
    UsageNestedMetadata,
    UsageNestedOccurrence,
    UsageNestedPart,
    UsageNestedPort,
    UsageNestedReference,
    UsageNestedRendering,
    UsageNestedRequirement,
    UsageNestedState,
    UsageNestedTransition,
    UsageNestedUsage,
    UsageNestedUseCase,
    UsageNestedVerificationCase,
    UsageNestedView,
    UsageNestedViewpoint,
    UsageUsage,
    UsageVariant,
    UsageVariantMembership,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefinitionUsageDerivedContract {
    pub kind: DefinitionUsageDerivedKind,
    /// SHA-256 of the unmodified pinned XMI OCL body before matcher normalization.
    pub body_sha256: String,
}

impl DefinitionUsageDerivedContract {
    /// The closed exact Definition/Usage contract vocabulary.  The refresh extractor and the
    /// normal manifest reader share this table, so a manually edited kind or evidence digest
    /// cannot redirect a rule to another canonical projection.
    pub fn from_exact_pinned_body(constraint: &str, body_sha256: &str) -> Option<Self> {
        let kind = match (constraint, body_sha256) {
            (
                "deriveDefinitionDirectedUsage",
                "2ed3a08e894113fc85abef4fc1bc5e65e3e87a9fbeacb8f6acdb51384d27b889",
            ) => DefinitionUsageDerivedKind::DefinitionDirectedUsage,
            (
                "deriveDefinitionOwnedAction",
                "d3bb0e3677bd947b287e3e8d41b2be5240780d5f878dc7bfd16f21553a2d0ad4",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedAction,
            (
                "deriveDefinitionOwnedAllocation",
                "3f71e08bbd7ee20f1c9d7d0345f30747a759fc4ce225f3090724a73bdabac02c",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedAllocation,
            (
                "deriveDefinitionOwnedAnalysisCase",
                "6e0c9a5a0fd516f00485460581ccfdee9a488d5a4dc76a7f908cb7361dc24761",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedAnalysisCase,
            (
                "deriveDefinitionOwnedAttribute",
                "0a7d92d89181e80c40313dcf5200714caf8504dfe0684339e036d227750d12e4",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedAttribute,
            (
                "deriveDefinitionOwnedCalculation",
                "c958803deabdff4f4f9a133f5acc01489ae45cfe9e88952a67e97e8eafc6cec5",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedCalculation,
            (
                "deriveDefinitionOwnedCase",
                "19134d6d80b90ae5ccabfd3d2c8e4460a26b0a87c6c5be2106e7ed69b6a2d256",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedCase,
            (
                "deriveDefinitionOwnedConcern",
                "e925cb6829e5e09cced8b72de3e2342747714fb763b21724b2af3115bca531b8",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedConcern,
            (
                "deriveDefinitionOwnedConnection",
                "36079b19326c7b7a111ab9f258d699e8d1fbb1815bdecacda507c5da764eb3b1",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedConnection,
            (
                "deriveDefinitionOwnedConstraint",
                "1a86afbddc7aa9b13b5816d1e6adb2f65a4f5bb72ee5bd9953c4561003200fbd",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedConstraint,
            (
                "deriveDefinitionOwnedEnumeration",
                "74a15ef161e0d7a29f7bfdce3ba0374f52f25a951764208ef11573cf3160e3a3",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedEnumeration,
            (
                "deriveDefinitionOwnedFlow",
                "a244a92dde25fc704568115e0b7fe9ee26476207754fc64b0179b7fd8e9e2faf",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedFlow,
            (
                "deriveDefinitionOwnedInterface",
                "87f6f229ff3ae8141b4607caf4cc3f1c98617caf9e3d07b01f6e49ab3e5c7d92",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedInterface,
            (
                "deriveDefinitionOwnedItem",
                "800e08c72436beb37221398b409825f5b596e9aa185e1d39bba9b01f9364fbef",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedItem,
            (
                "deriveDefinitionOwnedMetadata",
                "6a7add99b1451bc0eab1a9f23f4de8e1b536462e958d416fce38a86e38040d5d",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedMetadata,
            (
                "deriveDefinitionOwnedOccurrence",
                "a52c528104f793960bb05ab3f5fc43f95c6515f071a423718d4fa7fed95451b7",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedOccurrence,
            (
                "deriveDefinitionOwnedPart",
                "a1147ce07accd96a4f0f5c4d46d69bea3cff69abd6cd93a8bceb49116b4920f7",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedPart,
            (
                "deriveDefinitionOwnedPort",
                "db35d379286943f031df47222f58308cff4a16c1c028b701660a11e8b0339d32",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedPort,
            (
                "deriveDefinitionOwnedReference",
                "12426e92c448d07e2e0030a5d0ece9bd8dc3e959ba54f3b9ad1f2ac82f85e93a",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedReference,
            (
                "deriveDefinitionOwnedRendering",
                "621c39e181148e22becafe32e431739eee5349bfa2dfdc31ed140805c6f94675",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedRendering,
            (
                "deriveDefinitionOwnedRequirement",
                "c91182168e22f2a53f84e731f01d635ccbf35360e1da4f7622e823e44b40ad80",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedRequirement,
            (
                "deriveDefinitionOwnedState",
                "970179c2f30c127a20373eb9d8391c13db818b6ce7a4dea116b6873ef610a8ea",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedState,
            (
                "deriveDefinitionOwnedTransition",
                "6760a6a2c06841d610dced096e15de73526175cb60b62b2770db0f25df889773",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedTransition,
            (
                "deriveDefinitionOwnedUsage",
                "b07bf0d165ddfc594239cd381cf6c5c3d060acfe9dea199770a031f60b55dddf",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedUsage,
            (
                "deriveDefinitionOwnedUseCase",
                "e81a0eeb1352a4bdfc8d04bddd51f88fa35e487e23bc9381c641767fa52fc36a",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedUseCase,
            (
                "deriveDefinitionOwnedVerificationCase",
                "a4fc89e91fce9e6975e83696e0f54d231729c69a0211fff1ce2f886e8529357b",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedVerificationCase,
            (
                "deriveDefinitionOwnedView",
                "26fa7c4f5051008cee87a4347e6cee25c002aced23c60286b70bf4cc873ef795",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedView,
            (
                "deriveDefinitionOwnedViewpoint",
                "cd12fe09f57b50609c3183786bd87e27045273a3cd9d35df7b1b06ee7cb6efcd",
            ) => DefinitionUsageDerivedKind::DefinitionOwnedViewpoint,
            (
                "deriveDefinitionUsage",
                "a80a029f2f5ba896fe1c1437aaf88bc602c0b89a693b2b75d33969a390a8174a",
            ) => DefinitionUsageDerivedKind::DefinitionUsage,
            (
                "deriveDefinitionVariant",
                "ad05bc70b9b19e4c8c604124790786bed59336e66e768186628f625627aaa31d",
            ) => DefinitionUsageDerivedKind::DefinitionVariant,
            (
                "deriveDefinitionVariantMembership",
                "a0c93af47e34c982184a25a6596eefe8eb8d76a58bb01ce83ec2a86d1970f77e",
            ) => DefinitionUsageDerivedKind::DefinitionVariantMembership,
            (
                "deriveUsageDirectedUsage",
                "2ed3a08e894113fc85abef4fc1bc5e65e3e87a9fbeacb8f6acdb51384d27b889",
            ) => DefinitionUsageDerivedKind::UsageDirectedUsage,
            (
                "deriveUsageIsReference",
                "5a3d16260110b1e24862025367df8399600a3fa7ac8ca9dd5766718771125ac2",
            ) => DefinitionUsageDerivedKind::UsageIsReference,
            (
                "deriveUsageMayTimeVary",
                "e61909a051f3d934242bdee9f5c1d8122457c6bd82a779be7f34a9eff0c47112",
            ) => DefinitionUsageDerivedKind::UsageMayTimeVary,
            (
                "deriveUsageNestedAction",
                "13cbe48afdac828cbb3bd7290a9b1e2e87e72247ba4540dfdb43c4d864ed501a",
            ) => DefinitionUsageDerivedKind::UsageNestedAction,
            (
                "deriveUsageNestedAllocation",
                "670e43ee0bb04f7d8e293b9afc9c48f24b97fb05180058ac025936d7456f5636",
            ) => DefinitionUsageDerivedKind::UsageNestedAllocation,
            (
                "deriveUsageNestedAnalysisCase",
                "3ee9eebffed958331e0ff1347c080ee31d9d249da1dace524a5815094dcc507f",
            ) => DefinitionUsageDerivedKind::UsageNestedAnalysisCase,
            (
                "deriveUsageNestedAttribute",
                "b2310dd95cb0f78bb967db33d0f4e9110fddbbeddebade6cae79b734fa802f1b",
            ) => DefinitionUsageDerivedKind::UsageNestedAttribute,
            (
                "deriveUsageNestedCalculation",
                "0caff03e3a07a642aa32414f18c8472ce7d8864f65ce01049065da46ec451bc2",
            ) => DefinitionUsageDerivedKind::UsageNestedCalculation,
            (
                "deriveUsageNestedCase",
                "77293a14d0d61d1ebf6418e1551680ee9d6bd6dbecb2d25284339aaeb45ec0e3",
            ) => DefinitionUsageDerivedKind::UsageNestedCase,
            (
                "deriveUsageNestedConcern",
                "803efbe47a225a4338867aadf5a346f10e67640f6d5bb395d06487c50dd9940c",
            ) => DefinitionUsageDerivedKind::UsageNestedConcern,
            (
                "deriveUsageNestedConnection",
                "ee9df8d067c1bf5aa1c7a7a026270cd20f5335a91ad31efa8dabf8b301ebd45e",
            ) => DefinitionUsageDerivedKind::UsageNestedConnection,
            (
                "deriveUsageNestedConstraint",
                "119dafe064335dd2eaa653797970dbc134a62dff51891267bfe98304466e1da7",
            ) => DefinitionUsageDerivedKind::UsageNestedConstraint,
            (
                "deriveUsageNestedEnumeration",
                "ea8bdfd5b57322345533cfe16ed20edf12bd8812bfbf2648158805ae7fed830a",
            ) => DefinitionUsageDerivedKind::UsageNestedEnumeration,
            (
                "deriveUsageNestedFlow",
                "4de3ed01b82c1d26335ffe447525da006e7c688866718709829c6e4d80452a69",
            ) => DefinitionUsageDerivedKind::UsageNestedFlow,
            (
                "deriveUsageNestedInterface",
                "ac99a8c2902500b4ead6d16b6fb9ca94d89057e3215dfb31f699182ab4eb8ada",
            ) => DefinitionUsageDerivedKind::UsageNestedInterface,
            (
                "deriveUsageNestedItem",
                "9fa78d383f639f508564b2538c4f479d871fe412e81d1239cda00a0923c61a89",
            ) => DefinitionUsageDerivedKind::UsageNestedItem,
            (
                "deriveUsageNestedMetadata",
                "847bf30fa5563fc0445e09b5cb4761935076f83b06e2c342a6c0ccb166ae249e",
            ) => DefinitionUsageDerivedKind::UsageNestedMetadata,
            (
                "deriveUsageNestedOccurrence",
                "ead45a1576cc5caf6a4d1e49a940a294d480f074c5918214c313d0372429cea6",
            ) => DefinitionUsageDerivedKind::UsageNestedOccurrence,
            (
                "deriveUsageNestedPart",
                "a745face9158afe840656110e067d31ce546fcbe9adbd1ccfff968a3b0a5ed9d",
            ) => DefinitionUsageDerivedKind::UsageNestedPart,
            (
                "deriveUsageNestedPort",
                "aabbc431cd667bee20d2f21fb6f8387d8fc3c13a352715ac4441935f1210d7b1",
            ) => DefinitionUsageDerivedKind::UsageNestedPort,
            (
                "deriveUsageNestedReference",
                "e830cc0375db07dc06d041ce4bee2de18329d77e2cea34698f10b312a2ed2541",
            ) => DefinitionUsageDerivedKind::UsageNestedReference,
            (
                "deriveUsageNestedRendering",
                "6c4f8f2695269de4c9abbaec486881a052ef94602c1e129a42f5e5a68f122401",
            ) => DefinitionUsageDerivedKind::UsageNestedRendering,
            (
                "deriveUsageNestedRequirement",
                "9057999ef15b7b593ffef5fab8aca00074f9bf5a2195480331028df1635db012",
            ) => DefinitionUsageDerivedKind::UsageNestedRequirement,
            (
                "deriveUsageNestedState",
                "9cd607670d92177c4b42957ed0a451d2fd6a5101a8bdc954ec0ac471ed3af991",
            ) => DefinitionUsageDerivedKind::UsageNestedState,
            (
                "deriveUsageNestedTransition",
                "f106f84c0515c6b474628d00dd6ed25c1e0ae4a0e0047f1545e91434279592d6",
            ) => DefinitionUsageDerivedKind::UsageNestedTransition,
            (
                "deriveUsageNestedUsage",
                "003555435358c56ec417d4eb1c4fdb830f578d6dbf9f71ef019dcc0af9ac2d16",
            ) => DefinitionUsageDerivedKind::UsageNestedUsage,
            (
                "deriveUsageNestedUseCase",
                "5a3110d48311bcd9457245b2ff02700443029acfa0e96f5b3edfcee101b2d6c8",
            ) => DefinitionUsageDerivedKind::UsageNestedUseCase,
            (
                "deriveUsageNestedVerificationCase",
                "ccd1a8b7bb5c98c6cb9089d1ab24d4037bc829f0e519f037d9661b027d6e4738",
            ) => DefinitionUsageDerivedKind::UsageNestedVerificationCase,
            (
                "deriveUsageNestedView",
                "fa82d9e2744bf876ad8e3fa77b8e8defc50952844ed90c4d3091597c3c4c61a6",
            ) => DefinitionUsageDerivedKind::UsageNestedView,
            (
                "deriveUsageNestedViewpoint",
                "1aa5854659622c700574bf3a719a30efae2a51f93b69c5f0722092b5116ee565",
            ) => DefinitionUsageDerivedKind::UsageNestedViewpoint,
            (
                "deriveUsageUsage",
                "a80a029f2f5ba896fe1c1437aaf88bc602c0b89a693b2b75d33969a390a8174a",
            ) => DefinitionUsageDerivedKind::UsageUsage,
            (
                "deriveUsageVariant",
                "ad05bc70b9b19e4c8c604124790786bed59336e66e768186628f625627aaa31d",
            ) => DefinitionUsageDerivedKind::UsageVariant,
            (
                "deriveUsageVariantMembership",
                "a0c93af47e34c982184a25a6596eefe8eb8d76a58bb01ce83ec2a86d1970f77e",
            ) => DefinitionUsageDerivedKind::UsageVariantMembership,
            _ => return None,
        };
        Some(Self {
            kind,
            body_sha256: body_sha256.to_string(),
        })
    }

    pub fn is_exact_pinned_contract(&self, constraint: &str) -> bool {
        Self::from_exact_pinned_body(constraint, &self.body_sha256)
            .is_some_and(|expected| expected.kind == self.kind)
    }
}

/// Complete, exact Systems::Actions derived properties.  Each kind is the normative property,
/// not merely an argument ordinal, so no consumer can treat another action form as equivalent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionDerivedFactKind {
    ActionDefinitionAction,
    AssignmentValueExpression,
    AssignmentTargetArgument,
    AssignmentReferent,
    ForLoopVariable,
    ForLoopSeqArgument,
    LoopBodyAction,
    TerminateOccurrenceArgument,
    AcceptPayloadArgument,
    AcceptPayloadParameter,
    AcceptReceiverArgument,
    WhileArgument,
    UntilArgument,
    SendSenderArgument,
    SendReceiverArgument,
    SendPayloadArgument,
    IfThenAction,
    IfElseAction,
    IfArgument,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionDerivedFactContract {
    pub kind: ActionDerivedFactKind,
    /// SHA-256 of the unmodified pinned XMI OCL body before matcher normalization.
    pub body_sha256: String,
}

/// Complete exact Systems::Requirements projections. The kind names both its source metaclass
/// and normative property so a consumer cannot widen it to an ordinary feature-member scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementDerivedFactKind {
    DefinitionActorParameter,
    DefinitionSubjectParameter,
    DefinitionText,
    DefinitionRequiredConstraint,
    DefinitionAssumedConstraint,
    DefinitionFramedConcern,
    UsageActorParameter,
    UsageSubjectParameter,
    UsageText,
    UsageRequiredConstraint,
    UsageAssumedConstraint,
    UsageFramedConcern,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementDerivedFactContract {
    pub kind: RequirementDerivedFactKind,
    /// SHA-256 of the unmodified pinned XMI OCL body before matcher normalization.
    pub body_sha256: String,
}

/// The closed semantic-query family a rule is entitled to select.
///
/// This is a manifest-owned projection, not an OCL interpreter. It carries no model facts; it
/// only tells a consumer which canonical query family is applicable to an exact pinned rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintQueryFamily {
    FeatureDerivedRelationship(FeatureDerivedRelationshipKind),
    TypeDerivedRelationship(TypeDerivedRelationshipKind),
    TypeDerivedElement(TypeDerivedElementKind),
    TypeDerivedFact(TypeDerivedFactKind),
    TypeFeaturingCheck(TypeFeaturingCheckKind),
    RedefinitionCheck(RedefinitionCheckKind),
    SpecializationCheck(SpecializationCheckKind),
    ElementDerivedOwner(ElementDerivedOwnerKind),
    ElementDerivedDocumentation(ElementDerivedDocumentationKind),
    NamespaceDerivedElement(NamespaceDerivedElementKind),
    NamespaceImportDerivedElement(NamespaceImportDerivedElementKind),
    BindingConnectorCheck(BindingConnectorCheckKind),
    DefinitionUsageDerived(DefinitionUsageDerivedKind),
    ActionDerivedFact(ActionDerivedFactKind),
    RequirementDerivedFact(RequirementDerivedFactKind),
}

impl ConstraintManifestEntry {
    pub fn query_family(&self) -> Option<ConstraintQueryFamily> {
        if let Some(contract) = &self.feature_derived_relationship {
            Some(ConstraintQueryFamily::FeatureDerivedRelationship(
                contract.kind,
            ))
        } else if let Some(contract) = &self.type_derived_relationship {
            Some(ConstraintQueryFamily::TypeDerivedRelationship(
                contract.kind,
            ))
        } else if let Some(contract) = &self.type_derived_element {
            Some(ConstraintQueryFamily::TypeDerivedElement(contract.kind))
        } else if let Some(contract) = &self.type_derived_fact {
            Some(ConstraintQueryFamily::TypeDerivedFact(contract.kind))
        } else if let Some(contract) = &self.type_featuring_check {
            Some(ConstraintQueryFamily::TypeFeaturingCheck(contract.kind))
        } else if let Some(contract) = &self.redefinition_check {
            Some(ConstraintQueryFamily::RedefinitionCheck(contract.kind))
        } else if let Some(contract) = &self.specialization_check {
            Some(ConstraintQueryFamily::SpecializationCheck(contract.kind))
        } else {
            self.element_derived_owner
                .as_ref()
                .map(|contract| ConstraintQueryFamily::ElementDerivedOwner(contract.kind))
                .or_else(|| {
                    self.element_derived_documentation.as_ref().map(|contract| {
                        ConstraintQueryFamily::ElementDerivedDocumentation(contract.kind)
                    })
                })
                .or_else(|| {
                    self.definition_usage_derived.as_ref().map(|contract| {
                        ConstraintQueryFamily::DefinitionUsageDerived(contract.kind)
                    })
                })
                .or_else(|| {
                    self.action_derived_fact
                        .as_ref()
                        .map(|contract| ConstraintQueryFamily::ActionDerivedFact(contract.kind))
                })
                .or_else(|| {
                    self.requirement_derived_fact.as_ref().map(|contract| {
                        ConstraintQueryFamily::RequirementDerivedFact(contract.kind)
                    })
                })
                .or_else(|| {
                    self.namespace_derived_element.as_ref().map(|contract| {
                        ConstraintQueryFamily::NamespaceDerivedElement(contract.kind)
                    })
                })
                .or_else(|| {
                    self.namespace_import_derived_element
                        .as_ref()
                        .map(|contract| {
                            ConstraintQueryFamily::NamespaceImportDerivedElement(contract.kind)
                        })
                })
                .or_else(|| {
                    self.binding_connector_check
                        .as_ref()
                        .map(|contract| ConstraintQueryFamily::BindingConnectorCheck(contract.kind))
                })
        }
    }
}

/// The only normative constraint prefixes recorded by this inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintFamily {
    Derive,
    Check,
    Validate,
}

impl ConstraintFamily {
    pub fn from_constraint_name(name: &str) -> Option<Self> {
        let suffix = |prefix: &str| {
            name.strip_prefix(prefix)
                .is_some_and(|rest| rest.chars().next().is_some_and(char::is_uppercase))
        };
        if suffix("derive") {
            Some(Self::Derive)
        } else if suffix("check") {
            Some(Self::Check)
        } else if suffix("validate") {
            Some(Self::Validate)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_families_are_rejected_by_the_manifest_contract() {
        let manifest = r#"
schema_version = 1

[[specifications]]
name = "TestML"
version = "1.0"
formal_document_id = "formal/test"
xmi_file_id = "ptc/test"
xmi_sha256 = "xmi"
pdf_sha256 = "pdf"

[[specifications.constraints]]
package = "Core"
metaclass = "Element"
constraint = "deriveOwner"
family = "other"
clause = "8.3.1"
rule_id = "testml-1.0:Core::Element:deriveOwner"
"#;
        assert!(toml::from_str::<ConstraintManifest>(manifest).is_err());
    }

    #[test]
    fn loader_rejects_an_unsupported_schema_version() {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), "schema_version = 1\nspecifications = []\n").unwrap();
        let error = ConstraintManifest::load_toml(file.path()).unwrap_err();
        assert!(error.contains("unsupported constraint manifest schema 1"));
    }

    #[test]
    fn manifest_rule_owns_specification_identity_and_closed_query_family() {
        let entry = ConstraintManifestEntry {
            package: "Kernel".to_string(),
            metaclass: "Element".to_string(),
            constraint: "deriveElementDocumentation".to_string(),
            family: ConstraintFamily::Derive,
            clause: "8.3.2.1.2".to_string(),
            specializes_from_library: None,
            conditional_specializes_from_library: None,
            redefines_from_library: None,
            feature_derived_relationship: None,
            type_derived_relationship: None,
            type_derived_element: None,
            type_derived_fact: None,
            type_featuring_check: None,
            redefinition_check: None,
            specialization_check: None,
            element_derived_owner: None,
            element_derived_documentation: Some(ElementDerivedDocumentationContract {
                kind: ElementDerivedDocumentationKind::Documentation,
            }),
            namespace_derived_element: None,
            namespace_import_derived_element: None,
            binding_connector_check: None,
            definition_usage_derived: None,
            action_derived_fact: None,
            requirement_derived_fact: None,
            rule_id: "kerml-1.0:8.3.2.1.2:deriveElementDocumentation".to_string(),
        };
        let manifest = ConstraintManifest {
            schema_version: SCHEMA_VERSION,
            specifications: vec![SpecificationManifest {
                name: "KerML".to_string(),
                version: "1.0".to_string(),
                formal_document_id: SpecificationId::KerML10.formal_document_id().to_string(),
                xmi_file_id: "ptc/test".to_string(),
                xmi_sha256: "xmi".to_string(),
                pdf_sha256: "pdf".to_string(),
                constraints: vec![entry],
            }],
        };

        let rule = manifest
            .find_rule_with_specification("kerml-1.0:8.3.2.1.2:deriveElementDocumentation")
            .unwrap();
        assert_eq!(rule.specification_id(), Some(SpecificationId::KerML10));
        assert_eq!(
            rule.entry.query_family(),
            Some(ConstraintQueryFamily::ElementDerivedDocumentation(
                ElementDerivedDocumentationKind::Documentation
            ))
        );
        assert_eq!(
            SpecificationId::from_rule_id(&rule.entry.rule_id),
            rule.specification_id()
        );
    }

    #[test]
    fn pinned_specification_identity_is_shared_by_reader_and_fixture_contracts() {
        assert_eq!(PINNED_SPECIFICATIONS.len(), 2);
        for pinned in PINNED_SPECIFICATIONS {
            assert_eq!(pinned.specification_id.pinned(), pinned);
            assert_eq!(pinned.specification_id.as_str(), pinned.rule_id_prefix);
            assert_eq!(
                pinned.specification_id.name_version(),
                (pinned.name, pinned.version)
            );
            assert_eq!(
                pinned.specification_id.formal_document_id(),
                pinned.formal_document_id
            );
            assert_eq!(
                SpecificationId::parse(pinned.rule_id_prefix),
                Some(pinned.specification_id)
            );
        }
    }

    #[test]
    fn definition_usage_contract_rejects_wrong_kind_and_pinned_body_digest() {
        let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("specifications/constraint_manifest.toml");
        let mut manifest = ConstraintManifest::load_toml(&manifest_path).unwrap();
        let (specification_index, constraint_index) = manifest
            .specifications
            .iter()
            .enumerate()
            .find_map(|(specification_index, specification)| {
                specification
                    .constraints
                    .iter()
                    .position(|entry| entry.constraint == "deriveDefinitionOwnedAction")
                    .map(|constraint_index| (specification_index, constraint_index))
            })
            .unwrap();
        let original = manifest.specifications[specification_index].constraints[constraint_index]
            .definition_usage_derived
            .clone()
            .unwrap();

        manifest.specifications[specification_index].constraints[constraint_index]
            .definition_usage_derived
            .as_mut()
            .unwrap()
            .kind = DefinitionUsageDerivedKind::DefinitionOwnedPart;
        assert!(manifest
            .validate_rule_identities()
            .unwrap_err()
            .contains("incompatible Definition/Usage derivation contract"));

        manifest.specifications[specification_index].constraints[constraint_index]
            .definition_usage_derived = Some(original.clone());
        manifest.specifications[specification_index].constraints[constraint_index]
            .definition_usage_derived
            .as_mut()
            .unwrap()
            .body_sha256 = "0".repeat(64);
        assert!(manifest
            .validate_rule_identities()
            .unwrap_err()
            .contains("incompatible Definition/Usage derivation contract"));
        assert!(original.is_exact_pinned_contract("deriveDefinitionOwnedAction"));
    }

    #[test]
    fn identity_validation_rejects_name_fallbacks_and_duplicate_ids() {
        let specifications = PINNED_SPECIFICATIONS
            .iter()
            .map(|pinned| SpecificationManifest {
                name: pinned.name.to_string(),
                version: pinned.version.to_string(),
                formal_document_id: pinned.formal_document_id.to_string(),
                xmi_file_id: pinned.xmi_file_id.to_string(),
                xmi_sha256: pinned.expected_sha256.to_string(),
                pdf_sha256: pinned.expected_pdf_sha256.to_string(),
                constraints: Vec::new(),
            })
            .collect::<Vec<_>>();
        let mut manifest = ConstraintManifest {
            schema_version: SCHEMA_VERSION,
            specifications,
        };
        manifest.validate_pinned_inputs().unwrap();
        manifest.specifications[0]
            .constraints
            .push(ConstraintManifestEntry {
                package: "Core".to_string(),
                metaclass: "Element".to_string(),
                constraint: "deriveOwner".to_string(),
                family: ConstraintFamily::Derive,
                clause: "8.3.2.1.2".to_string(),
                specializes_from_library: None,
                conditional_specializes_from_library: None,
                redefines_from_library: None,
                feature_derived_relationship: None,
                type_derived_relationship: None,
                type_derived_element: None,
                type_derived_fact: None,
                type_featuring_check: None,
                redefinition_check: None,
                specialization_check: None,
                element_derived_owner: None,
                element_derived_documentation: None,
                namespace_derived_element: None,
                namespace_import_derived_element: None,
                binding_connector_check: None,
                definition_usage_derived: None,
                action_derived_fact: None,
                requirement_derived_fact: None,
                rule_id: "deriveOwner".to_string(),
            });
        assert!(manifest
            .validate_rule_identities()
            .unwrap_err()
            .contains("canonical identity"));
        manifest.specifications[0].constraints[0].rule_id =
            "kerml-1.0:8.3.2.1.2:deriveOwner".to_string();
        let duplicate = manifest.specifications[0].constraints[0].clone();
        manifest.specifications[0].constraints.push(duplicate);
        assert!(manifest
            .validate_rule_identities()
            .unwrap_err()
            .contains("duplicate rule_id"));
    }

    #[test]
    fn owned_by_contract_requires_exactly_two_valid_owner_metaclasses() {
        let specifications = PINNED_SPECIFICATIONS
            .iter()
            .map(|pinned| SpecificationManifest {
                name: pinned.name.to_string(),
                version: pinned.version.to_string(),
                formal_document_id: pinned.formal_document_id.to_string(),
                xmi_file_id: pinned.xmi_file_id.to_string(),
                xmi_sha256: pinned.expected_sha256.to_string(),
                pdf_sha256: pinned.expected_pdf_sha256.to_string(),
                constraints: Vec::new(),
            })
            .collect::<Vec<_>>();
        let mut manifest = ConstraintManifest {
            schema_version: SCHEMA_VERSION,
            specifications,
        };
        let entry = ConstraintManifestEntry {
            package: "Systems::Ports".to_string(),
            metaclass: "PortUsage".to_string(),
            constraint: "checkPortUsageOwnedPortSpecialization".to_string(),
            family: ConstraintFamily::Check,
            clause: "8.3.12.6".to_string(),
            specializes_from_library: None,
            conditional_specializes_from_library: Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::OwnedBy,
                owner_metaclasses: vec!["PartDefinition".to_string(), "PartUsage".to_string()],
                anchor: "Parts::Part::ownedPorts".to_string(),
                true_anchor: None,
            }),
            redefines_from_library: None,
            feature_derived_relationship: None,
            type_derived_relationship: None,
            type_derived_element: None,
            type_derived_fact: None,
            type_featuring_check: None,
            redefinition_check: None,
            specialization_check: None,
            element_derived_owner: None,
            element_derived_documentation: None,
            namespace_derived_element: None,
            namespace_import_derived_element: None,
            binding_connector_check: None,
            definition_usage_derived: None,
            action_derived_fact: None,
            requirement_derived_fact: None,
            rule_id: "sysml-2.0:8.3.12.6:checkPortUsageOwnedPortSpecialization".to_string(),
        };
        manifest.specifications[1].constraints.push(entry.clone());
        manifest.validate_rule_identities().unwrap();

        manifest.specifications[1].constraints[0]
            .conditional_specializes_from_library
            .as_mut()
            .unwrap()
            .owner_metaclasses = vec!["PartDefinition".to_string()];
        assert!(manifest
            .validate_rule_identities()
            .unwrap_err()
            .contains("invalid exact library anchor"));

        manifest.specifications[1].constraints[0]
            .conditional_specializes_from_library
            .as_mut()
            .unwrap()
            .owner_metaclasses = vec!["Part Definition".to_string(), "PartUsage".to_string()];
        assert!(manifest
            .validate_rule_identities()
            .unwrap_err()
            .contains("invalid exact library anchor"));
    }
}
