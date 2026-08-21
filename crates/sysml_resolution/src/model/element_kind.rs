//! Projection from the private declaration domain onto the published element-kind vocabulary.
//!
//! Both functions are **total** and match exhaustively with no wildcard arm, so adding a
//! `DeclarationKind` fails to compile until someone decides what it means publicly. Unlike the
//! sibling compiler -- whose `tagToMetaclass` returns `null` for five internal placeholder tags --
//! every one of our declaration kinds is reachable only by lowering syntax a user typed, so there
//! is no case for an `Option<ElementKind>`: the `None` branch would be permanently empty and would
//! force a meaningless fallback on every consumer.
//!
//! Two kinds share one [`ElementKind`] only when they denote the same OMG metaclass **and** the
//! distinction between them is either pure surface syntax or recoverable from a fact this crate
//! already publishes. Where the OMG parks the distinction on a membership, it is published as a
//! [`MembershipRole`] instead of being folded into the kind.
//!
//! The collapse table is pinned by a test, so a *new* collapse cannot appear by accident.

use super::DeclarationKind;
use crate::element_kind::{
    ElementKind, MembershipRole, RequirementConstraintKind, StateSubactionKind,
};

/// The published kind of a declaration.
pub(crate) fn element_kind(kind: DeclarationKind) -> ElementKind {
    match kind {
        DeclarationKind::Namespace => ElementKind::Namespace,
        DeclarationKind::Package => ElementKind::Package,
        DeclarationKind::LibraryPackage => ElementKind::LibraryPackage,

        DeclarationKind::PartDefinition => ElementKind::PartDefinition,
        DeclarationKind::PartUsage => ElementKind::PartUsage,
        DeclarationKind::AttributeDefinition => ElementKind::AttributeDefinition,
        DeclarationKind::AttributeUsage => ElementKind::AttributeUsage,
        DeclarationKind::EnumerationDefinition => ElementKind::EnumerationDefinition,
        DeclarationKind::EnumerationUsage => ElementKind::EnumerationUsage,
        // An enumeration literal is an `EnumerationUsage` under a `VariantMembership`; the literal
        // case is told apart by its role, not by a separate kind.
        DeclarationKind::EnumerationLiteral => ElementKind::EnumerationUsage,
        DeclarationKind::ItemDefinition => ElementKind::ItemDefinition,
        DeclarationKind::ItemUsage => ElementKind::ItemUsage,
        DeclarationKind::PortDefinition => ElementKind::PortDefinition,
        DeclarationKind::PortUsage => ElementKind::PortUsage,
        DeclarationKind::OccurrenceDefinition => ElementKind::OccurrenceDefinition,
        DeclarationKind::OccurrenceUsage => ElementKind::OccurrenceUsage,
        DeclarationKind::IndividualDefinition => ElementKind::IndividualDefinition,
        DeclarationKind::ConnectionDefinition => ElementKind::ConnectionDefinition,
        DeclarationKind::ConnectionUsage => ElementKind::ConnectionUsage,
        // `'connection' …` and `'connect' …` are one grammar production. The bare form is
        // recognisable anyway: it is anonymous and carries connector-end references but no typing.
        DeclarationKind::BareConnect => ElementKind::ConnectionUsage,
        DeclarationKind::InterfaceDefinition => ElementKind::InterfaceDefinition,
        DeclarationKind::InterfaceUsage => ElementKind::InterfaceUsage,
        DeclarationKind::AllocationDefinition => ElementKind::AllocationDefinition,
        DeclarationKind::Allocate => ElementKind::AllocationUsage,
        DeclarationKind::FlowDefinition => ElementKind::FlowConnectionDefinition,
        DeclarationKind::Flow => ElementKind::FlowConnectionUsage,
        DeclarationKind::ActionDefinition => ElementKind::ActionDefinition,
        DeclarationKind::ActionUsage => ElementKind::ActionUsage,
        DeclarationKind::AcceptActionUsage => ElementKind::AcceptActionUsage,
        // Identical element type; the entry/do/exit slot is a membership role.
        DeclarationKind::EntryActionBinding
        | DeclarationKind::DoActionBinding
        | DeclarationKind::ExitActionBinding => ElementKind::ActionUsage,
        DeclarationKind::StateDefinition => ElementKind::StateDefinition,
        DeclarationKind::StateUsage => ElementKind::StateUsage,
        DeclarationKind::CalcDefinition => ElementKind::CalculationDefinition,
        DeclarationKind::CalcUsage => ElementKind::CalculationUsage,
        DeclarationKind::ConstraintDefinition => ElementKind::ConstraintDefinition,
        DeclarationKind::ConstraintUsage => ElementKind::ConstraintUsage,
        DeclarationKind::AssertConstraintUsage => ElementKind::AssertConstraintUsage,
        // Both are a plain `ConstraintUsage`; `assume` vs `require` is the owning
        // `RequirementConstraintMembership`'s kind, published as a role.
        DeclarationKind::AssumeConstraintUsage | DeclarationKind::RequireConstraintUsage => {
            ElementKind::ConstraintUsage
        }
        DeclarationKind::RequirementDefinition => ElementKind::RequirementDefinition,
        DeclarationKind::RequirementUsage => ElementKind::RequirementUsage,
        // A verified requirement is a `RequirementUsage` under a
        // `RequirementVerificationMembership`.
        DeclarationKind::VerifyRequirement => ElementKind::RequirementUsage,
        DeclarationKind::ConcernDefinition => ElementKind::ConcernDefinition,
        DeclarationKind::ConcernUsage => ElementKind::ConcernUsage,
        // A `frame` is a `ConcernUsage` under a `FramedConcernMembership`.
        DeclarationKind::Frame => ElementKind::ConcernUsage,
        DeclarationKind::CaseDefinition => ElementKind::CaseDefinition,
        DeclarationKind::CaseUsage => ElementKind::CaseUsage,
        DeclarationKind::AnalysisCaseDefinition => ElementKind::AnalysisCaseDefinition,
        DeclarationKind::AnalysisCaseUsage => ElementKind::AnalysisCaseUsage,
        DeclarationKind::VerificationCaseDefinition => ElementKind::VerificationCaseDefinition,
        DeclarationKind::VerificationCaseUsage => ElementKind::VerificationCaseUsage,
        DeclarationKind::UseCaseDefinition => ElementKind::UseCaseDefinition,
        DeclarationKind::UseCaseUsage => ElementKind::UseCaseUsage,
        DeclarationKind::ViewDefinition => ElementKind::ViewDefinition,
        DeclarationKind::ViewUsage => ElementKind::ViewUsage,
        DeclarationKind::ViewpointDefinition => ElementKind::ViewpointDefinition,
        DeclarationKind::ViewpointUsage => ElementKind::ViewpointUsage,
        DeclarationKind::RenderingDefinition => ElementKind::RenderingDefinition,
        DeclarationKind::RenderingUsage => ElementKind::RenderingUsage,
        DeclarationKind::MetadataDefinition => ElementKind::MetadataDefinition,
        DeclarationKind::MetadataUsage => ElementKind::MetadataUsage,
        // `#keyword def X` has no more specific type than `Definition`.
        DeclarationKind::ExtendedDefinition => ElementKind::Definition,

        DeclarationKind::ReferenceUsage => ElementKind::ReferenceUsage,
        // `DefaultReferenceUsage : ReferenceUsage`; the only difference is whether `ref` was typed.
        DeclarationKind::DefaultReferenceUsage => ElementKind::ReferenceUsage,
        // Parameters, subjects and stakeholders are referential features distinguished by their
        // owning membership, which is published as a role.
        DeclarationKind::ParameterUsage | DeclarationKind::PerformParameterBinding => {
            ElementKind::ReferenceUsage
        }
        DeclarationKind::SubjectUsage => ElementKind::ReferenceUsage,
        DeclarationKind::StakeholderUsage => ElementKind::PartUsage,
        // One `ActorUsage : PartUsage` production; the requirement/case split is a parser artefact.
        DeclarationKind::RequirementActor | DeclarationKind::CaseActor => ElementKind::PartUsage,

        DeclarationKind::PerformActionUsage => ElementKind::PerformActionUsage,
        DeclarationKind::Transition => ElementKind::TransitionUsage,
        DeclarationKind::Assign => ElementKind::AssignmentActionUsage,
        DeclarationKind::If => ElementKind::IfActionUsage,
        // One production: `loop` is `while` with an empty condition parameter.
        DeclarationKind::While | DeclarationKind::Loop => ElementKind::WhileLoopActionUsage,
        DeclarationKind::ForLoop => ElementKind::ForLoopActionUsage,
        DeclarationKind::ForLoopVariable => ElementKind::ForLoopVariable,
        DeclarationKind::Decide => ElementKind::DecisionNode,
        DeclarationKind::Merge => ElementKind::MergeNode,
        DeclarationKind::Fork => ElementKind::ForkNode,
        DeclarationKind::Join => ElementKind::JoinNode,
        // All three are `SuccessionAsUsage`; see the variant's doc comment for what tells them
        // apart without a separate kind.
        DeclarationKind::Succession
        | DeclarationKind::ThenContinuation
        | DeclarationKind::InitialState => ElementKind::SuccessionAsUsage,
        DeclarationKind::FinalState => ElementKind::FinalState,

        DeclarationKind::Satisfy => ElementKind::SatisfyRequirementUsage,
        DeclarationKind::Bind => ElementKind::BindingConnectorAsUsage,
        DeclarationKind::Import => ElementKind::Import,
        DeclarationKind::Expose => ElementKind::Expose,
        DeclarationKind::Alias => ElementKind::Alias,
        DeclarationKind::Dependency => ElementKind::Dependency,

        DeclarationKind::KermlType => ElementKind::Type,
        DeclarationKind::KermlClassifier => ElementKind::Classifier,
        // Upstream routed `class` through the shared KerML classifier declaration, so
        // `ClassDefinition` is now the single kind every `class` spelling reaches.
        DeclarationKind::ClassDefinition => ElementKind::Class,
        DeclarationKind::KermlStructure => ElementKind::Structure,
        DeclarationKind::KermlAssociation => ElementKind::Association,
        DeclarationKind::KermlAssociationStructure => ElementKind::AssociationStructure,
        DeclarationKind::KermlDataType => ElementKind::DataType,
        DeclarationKind::KermlMetaclass => ElementKind::Metaclass,
        DeclarationKind::KermlBehavior => ElementKind::Behavior,
        DeclarationKind::KermlFunction => ElementKind::Function,
        DeclarationKind::KermlPredicate => ElementKind::Predicate,
        DeclarationKind::KermlInteraction => ElementKind::Interaction,
        DeclarationKind::KermlMultiplicity => ElementKind::Multiplicity,

        DeclarationKind::KermlFeature => ElementKind::Feature,
        // An association or connector end is a `Feature` under an `EndFeatureMembership`.
        DeclarationKind::KermlEnd => ElementKind::Feature,
        DeclarationKind::KermlStep => ElementKind::Step,
        DeclarationKind::KermlExpression => ElementKind::Expression,
        DeclarationKind::KermlBooleanExpression => ElementKind::BooleanExpression,
        DeclarationKind::KermlConnector => ElementKind::Connector,
        DeclarationKind::KermlBinding => ElementKind::BindingConnector,
        DeclarationKind::KermlInvariant => ElementKind::Invariant,
    }
}

/// The role a declaration plays in its owner, where the OMG carries that role on the owning
/// membership rather than on the element.
///
/// `None` for the ordinary case of a member whose membership adds no role.
pub(crate) fn membership_role(kind: DeclarationKind) -> Option<MembershipRole> {
    match kind {
        DeclarationKind::EntryActionBinding => {
            Some(MembershipRole::StateSubaction(StateSubactionKind::Entry))
        }
        DeclarationKind::DoActionBinding => {
            Some(MembershipRole::StateSubaction(StateSubactionKind::Do))
        }
        DeclarationKind::ExitActionBinding => {
            Some(MembershipRole::StateSubaction(StateSubactionKind::Exit))
        }
        DeclarationKind::SubjectUsage => Some(MembershipRole::Subject),
        DeclarationKind::StakeholderUsage => Some(MembershipRole::Stakeholder),
        DeclarationKind::RequirementActor | DeclarationKind::CaseActor => {
            Some(MembershipRole::Actor)
        }
        DeclarationKind::Frame => Some(MembershipRole::FramedConcern),
        DeclarationKind::VerifyRequirement => Some(MembershipRole::RequirementVerification),
        DeclarationKind::EnumerationLiteral => Some(MembershipRole::Variant),
        DeclarationKind::ParameterUsage | DeclarationKind::PerformParameterBinding => {
            Some(MembershipRole::Parameter)
        }
        DeclarationKind::KermlEnd => Some(MembershipRole::EndFeature),
        DeclarationKind::AssumeConstraintUsage => Some(MembershipRole::RequirementConstraint(
            RequirementConstraintKind::Assumption,
        )),
        DeclarationKind::RequireConstraintUsage => Some(MembershipRole::RequirementConstraint(
            RequirementConstraintKind::Requirement,
        )),

        DeclarationKind::Namespace
        | DeclarationKind::Package
        | DeclarationKind::LibraryPackage
        | DeclarationKind::PartDefinition
        | DeclarationKind::PartUsage
        | DeclarationKind::AttributeDefinition
        | DeclarationKind::AttributeUsage
        | DeclarationKind::EnumerationDefinition
        | DeclarationKind::EnumerationUsage
        | DeclarationKind::ItemDefinition
        | DeclarationKind::ItemUsage
        | DeclarationKind::PortDefinition
        | DeclarationKind::PortUsage
        | DeclarationKind::OccurrenceDefinition
        | DeclarationKind::OccurrenceUsage
        | DeclarationKind::IndividualDefinition
        | DeclarationKind::ConnectionDefinition
        | DeclarationKind::ConnectionUsage
        | DeclarationKind::BareConnect
        | DeclarationKind::InterfaceDefinition
        | DeclarationKind::InterfaceUsage
        | DeclarationKind::AllocationDefinition
        | DeclarationKind::Allocate
        | DeclarationKind::FlowDefinition
        | DeclarationKind::Flow
        | DeclarationKind::ActionDefinition
        | DeclarationKind::ActionUsage
        | DeclarationKind::AcceptActionUsage
        | DeclarationKind::StateDefinition
        | DeclarationKind::StateUsage
        | DeclarationKind::CalcDefinition
        | DeclarationKind::CalcUsage
        | DeclarationKind::ConstraintDefinition
        | DeclarationKind::ConstraintUsage
        | DeclarationKind::AssertConstraintUsage
        | DeclarationKind::RequirementDefinition
        | DeclarationKind::RequirementUsage
        | DeclarationKind::ConcernDefinition
        | DeclarationKind::ConcernUsage
        | DeclarationKind::CaseDefinition
        | DeclarationKind::CaseUsage
        | DeclarationKind::AnalysisCaseDefinition
        | DeclarationKind::AnalysisCaseUsage
        | DeclarationKind::VerificationCaseDefinition
        | DeclarationKind::VerificationCaseUsage
        | DeclarationKind::UseCaseDefinition
        | DeclarationKind::UseCaseUsage
        | DeclarationKind::ViewDefinition
        | DeclarationKind::ViewUsage
        | DeclarationKind::ViewpointDefinition
        | DeclarationKind::ViewpointUsage
        | DeclarationKind::RenderingDefinition
        | DeclarationKind::RenderingUsage
        | DeclarationKind::MetadataDefinition
        | DeclarationKind::MetadataUsage
        | DeclarationKind::ExtendedDefinition
        | DeclarationKind::ReferenceUsage
        | DeclarationKind::DefaultReferenceUsage
        | DeclarationKind::PerformActionUsage
        | DeclarationKind::Transition
        | DeclarationKind::Assign
        | DeclarationKind::If
        | DeclarationKind::While
        | DeclarationKind::Loop
        | DeclarationKind::ForLoop
        | DeclarationKind::ForLoopVariable
        | DeclarationKind::Decide
        | DeclarationKind::Merge
        | DeclarationKind::Fork
        | DeclarationKind::Join
        | DeclarationKind::Succession
        | DeclarationKind::ThenContinuation
        | DeclarationKind::InitialState
        | DeclarationKind::FinalState
        | DeclarationKind::Satisfy
        | DeclarationKind::Bind
        | DeclarationKind::Import
        | DeclarationKind::Expose
        | DeclarationKind::Alias
        | DeclarationKind::Dependency
        | DeclarationKind::KermlClassifier
        | DeclarationKind::ClassDefinition
        | DeclarationKind::KermlType
        | DeclarationKind::KermlStructure
        | DeclarationKind::KermlAssociation
        | DeclarationKind::KermlAssociationStructure
        | DeclarationKind::KermlDataType
        | DeclarationKind::KermlMetaclass
        | DeclarationKind::KermlBehavior
        | DeclarationKind::KermlFunction
        | DeclarationKind::KermlPredicate
        | DeclarationKind::KermlInteraction
        | DeclarationKind::KermlMultiplicity
        | DeclarationKind::KermlFeature
        | DeclarationKind::KermlStep
        | DeclarationKind::KermlExpression
        | DeclarationKind::KermlBooleanExpression
        | DeclarationKind::KermlConnector
        | DeclarationKind::KermlBinding
        | DeclarationKind::KermlInvariant => None,
    }
}

/// The fact-sensitive role projection for the few memberships whose OMG kind is not fixed by the
/// element metaclass alone. The trigger bit is constructed at the transition lowering boundary;
/// this query projection never re-inspects parser syntax or owner names.
pub(crate) fn membership_role_with_trigger(
    kind: DeclarationKind,
    is_trigger_action: Option<bool>,
) -> Option<MembershipRole> {
    if kind == DeclarationKind::AcceptActionUsage && is_trigger_action == Some(true) {
        Some(MembershipRole::TransitionTriggerAction)
    } else {
        membership_role(kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::resolver::writer::declaration_kind;
    use std::collections::{BTreeMap, BTreeSet};

    /// Every declaration kind.
    ///
    /// Rust cannot enumerate an enum's variants, so this list is maintained by hand and pinned
    /// against the enum's own source text by a test in `sysml_query` -- a fixed-size array here
    /// would silently miss a newly added variant, which is exactly what these tests exist to
    /// catch.
    const ALL_DECLARATION_KINDS: &[DeclarationKind] = &[
        DeclarationKind::Namespace,
        DeclarationKind::Package,
        DeclarationKind::LibraryPackage,
        DeclarationKind::PartDefinition,
        DeclarationKind::PartUsage,
        DeclarationKind::AttributeDefinition,
        DeclarationKind::AttributeUsage,
        DeclarationKind::EnumerationDefinition,
        DeclarationKind::EnumerationUsage,
        DeclarationKind::EnumerationLiteral,
        DeclarationKind::RequirementDefinition,
        DeclarationKind::RequirementUsage,
        DeclarationKind::PortDefinition,
        DeclarationKind::PortUsage,
        DeclarationKind::ItemDefinition,
        DeclarationKind::ItemUsage,
        DeclarationKind::ActionDefinition,
        DeclarationKind::ActionUsage,
        DeclarationKind::AcceptActionUsage,
        DeclarationKind::Succession,
        DeclarationKind::StateDefinition,
        DeclarationKind::StateUsage,
        DeclarationKind::MetadataDefinition,
        DeclarationKind::MetadataUsage,
        DeclarationKind::ConnectionDefinition,
        DeclarationKind::ConnectionUsage,
        DeclarationKind::OccurrenceDefinition,
        DeclarationKind::OccurrenceUsage,
        DeclarationKind::AnalysisCaseDefinition,
        DeclarationKind::AnalysisCaseUsage,
        DeclarationKind::InterfaceDefinition,
        DeclarationKind::ViewDefinition,
        DeclarationKind::CaseDefinition,
        DeclarationKind::CaseUsage,
        DeclarationKind::VerificationCaseDefinition,
        DeclarationKind::UseCaseDefinition,
        DeclarationKind::ViewpointDefinition,
        DeclarationKind::RenderingDefinition,
        DeclarationKind::AllocationDefinition,
        DeclarationKind::FlowDefinition,
        DeclarationKind::ViewUsage,
        DeclarationKind::RenderingUsage,
        DeclarationKind::UseCaseUsage,
        DeclarationKind::VerificationCaseUsage,
        DeclarationKind::ViewpointUsage,
        DeclarationKind::InterfaceUsage,
        DeclarationKind::ConstraintDefinition,
        DeclarationKind::ConstraintUsage,
        DeclarationKind::AssertConstraintUsage,
        DeclarationKind::AssumeConstraintUsage,
        DeclarationKind::RequireConstraintUsage,
        DeclarationKind::ConcernDefinition,
        DeclarationKind::ConcernUsage,
        DeclarationKind::CalcDefinition,
        DeclarationKind::CalcUsage,
        DeclarationKind::ClassDefinition,
        DeclarationKind::Import,
        DeclarationKind::Expose,
        DeclarationKind::Alias,
        DeclarationKind::EntryActionBinding,
        DeclarationKind::DoActionBinding,
        DeclarationKind::ExitActionBinding,
        DeclarationKind::InitialState,
        DeclarationKind::FinalState,
        DeclarationKind::ParameterUsage,
        DeclarationKind::SubjectUsage,
        DeclarationKind::PerformActionUsage,
        DeclarationKind::Transition,
        DeclarationKind::Satisfy,
        DeclarationKind::Allocate,
        DeclarationKind::Bind,
        DeclarationKind::ReferenceUsage,
        DeclarationKind::Decide,
        DeclarationKind::Merge,
        DeclarationKind::Fork,
        DeclarationKind::Join,
        DeclarationKind::ThenContinuation,
        DeclarationKind::Flow,
        DeclarationKind::StakeholderUsage,
        DeclarationKind::RequirementActor,
        DeclarationKind::CaseActor,
        DeclarationKind::Frame,
        DeclarationKind::VerifyRequirement,
        DeclarationKind::KermlType,
        DeclarationKind::KermlClassifier,
        DeclarationKind::KermlStructure,
        DeclarationKind::KermlAssociation,
        DeclarationKind::KermlAssociationStructure,
        DeclarationKind::KermlDataType,
        DeclarationKind::KermlMetaclass,
        DeclarationKind::KermlBehavior,
        DeclarationKind::KermlFunction,
        DeclarationKind::KermlPredicate,
        DeclarationKind::KermlInteraction,
        DeclarationKind::KermlMultiplicity,
        DeclarationKind::KermlFeature,
        DeclarationKind::KermlStep,
        DeclarationKind::KermlExpression,
        DeclarationKind::KermlBooleanExpression,
        DeclarationKind::DefaultReferenceUsage,
        DeclarationKind::KermlConnector,
        DeclarationKind::KermlBinding,
        DeclarationKind::KermlInvariant,
        DeclarationKind::KermlEnd,
        DeclarationKind::Assign,
        DeclarationKind::While,
        DeclarationKind::Loop,
        DeclarationKind::If,
        DeclarationKind::ForLoop,
        DeclarationKind::ForLoopVariable,
        DeclarationKind::Dependency,
        DeclarationKind::ExtendedDefinition,
        DeclarationKind::IndividualDefinition,
        DeclarationKind::BareConnect,
        DeclarationKind::PerformParameterBinding,
    ];

    /// The complete many-to-one table.
    ///
    /// Pinned as data so a *new* collapse cannot appear by accident: exhaustiveness checking
    /// proves every declaration kind is mapped, but nothing stops two of them quietly acquiring
    /// the same public kind. This is the sibling compiler's many-to-one audit
    /// (`plans/29-many-to-one-tag-audit.md`) turned into a build-breaking artefact.
    const COLLAPSES: &[(ElementKind, &[DeclarationKind])] = &[
        (
            ElementKind::ActionUsage,
            &[
                DeclarationKind::ActionUsage,
                DeclarationKind::EntryActionBinding,
                DeclarationKind::DoActionBinding,
                DeclarationKind::ExitActionBinding,
            ],
        ),
        (
            ElementKind::ConcernUsage,
            &[DeclarationKind::ConcernUsage, DeclarationKind::Frame],
        ),
        (
            ElementKind::ConnectionUsage,
            &[
                DeclarationKind::ConnectionUsage,
                DeclarationKind::BareConnect,
            ],
        ),
        (
            ElementKind::ConstraintUsage,
            &[
                DeclarationKind::ConstraintUsage,
                DeclarationKind::AssumeConstraintUsage,
                DeclarationKind::RequireConstraintUsage,
            ],
        ),
        (
            ElementKind::EnumerationUsage,
            &[
                DeclarationKind::EnumerationUsage,
                DeclarationKind::EnumerationLiteral,
            ],
        ),
        (
            ElementKind::Feature,
            &[DeclarationKind::KermlFeature, DeclarationKind::KermlEnd],
        ),
        (
            ElementKind::PartUsage,
            &[
                DeclarationKind::PartUsage,
                DeclarationKind::StakeholderUsage,
                DeclarationKind::RequirementActor,
                DeclarationKind::CaseActor,
            ],
        ),
        (
            ElementKind::ReferenceUsage,
            &[
                DeclarationKind::ParameterUsage,
                DeclarationKind::SubjectUsage,
                DeclarationKind::ReferenceUsage,
                DeclarationKind::DefaultReferenceUsage,
                DeclarationKind::PerformParameterBinding,
            ],
        ),
        (
            ElementKind::RequirementUsage,
            &[
                DeclarationKind::RequirementUsage,
                DeclarationKind::VerifyRequirement,
            ],
        ),
        (
            ElementKind::SuccessionAsUsage,
            &[
                DeclarationKind::Succession,
                DeclarationKind::ThenContinuation,
                DeclarationKind::InitialState,
            ],
        ),
        (
            ElementKind::WhileLoopActionUsage,
            &[DeclarationKind::While, DeclarationKind::Loop],
        ),
    ];

    /// No public kind may be unreachable: a variant nothing projects onto is dead contract.
    #[test]
    fn every_public_kind_is_produced_by_some_declaration_kind() {
        let produced = ALL_DECLARATION_KINDS
            .iter()
            .copied()
            .map(element_kind)
            .collect::<BTreeSet<_>>();
        let declared = ElementKind::ALL.iter().copied().collect::<BTreeSet<_>>();
        let unreachable = declared.difference(&produced).collect::<Vec<_>>();
        assert!(
            unreachable.is_empty(),
            "these public kinds are never produced, so they are dead contract: {unreachable:?}"
        );
    }

    #[test]
    fn every_many_to_one_collapse_is_declared_in_the_audit_table() {
        let mut grouped: BTreeMap<ElementKind, Vec<DeclarationKind>> = BTreeMap::new();
        for kind in ALL_DECLARATION_KINDS.iter().copied() {
            grouped.entry(element_kind(kind)).or_default().push(kind);
        }
        // Compared as maps of sorted sets, so neither the table's reading order nor the enums'
        // declaration order is part of the assertion.
        let observed = grouped
            .into_iter()
            .filter(|(_, kinds)| kinds.len() > 1)
            .map(|(public, kinds)| (public, kinds.into_iter().collect::<BTreeSet<_>>()))
            .collect::<BTreeMap<_, _>>();
        let audited = COLLAPSES
            .iter()
            .map(|(public, kinds)| (*public, kinds.iter().copied().collect::<BTreeSet<_>>()))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(
            observed, audited,
            "the set of many-to-one collapses changed; a collapse merges two elements into one \
             published kind, so add it to COLLAPSES only with a reason recorded in \
             `element_kind`'s own comments"
        );
    }

    /// The `assume`/`require` distinction is what the OMG parks on
    /// `RequirementConstraintMembership.kind`, and the keyword is the only thing that carries it,
    /// so both spellings must reach the published role rather than collapsing on the way.
    #[test]
    fn assume_and_require_constraints_publish_their_membership_role() {
        assert_eq!(
            element_kind(DeclarationKind::AssumeConstraintUsage),
            ElementKind::ConstraintUsage
        );
        assert_eq!(
            element_kind(DeclarationKind::RequireConstraintUsage),
            ElementKind::ConstraintUsage
        );
        assert_eq!(
            membership_role(DeclarationKind::AssumeConstraintUsage),
            Some(MembershipRole::RequirementConstraint(
                RequirementConstraintKind::Assumption
            ))
        );
        assert_eq!(
            membership_role(DeclarationKind::RequireConstraintUsage),
            Some(MembershipRole::RequirementConstraint(
                RequirementConstraintKind::Requirement
            ))
        );

        // `assert constraint` is a metaclass of its own, not a role on a plain constraint.
        assert_eq!(
            element_kind(DeclarationKind::AssertConstraintUsage),
            ElementKind::AssertConstraintUsage
        );
        assert_eq!(
            membership_role(DeclarationKind::AssertConstraintUsage),
            None
        );
    }

    /// The kebab names are the *identity* channel, not the classification channel: they are
    /// length-prefixed into every published `SymbolIdentity`
    /// (`model/resolver.rs`, `encode_identity`). Changing one silently changes identities that
    /// consumers may be holding, so any edit here has to move `IDENTITY_ENCODING_VERSION` too.
    #[test]
    fn identity_kind_names_are_frozen_with_the_encoding_version() {
        assert_eq!(
            crate::model::resolver::IDENTITY_ENCODING_VERSION,
            "element/v1",
            "the identity encoding version changed; confirm every kebab kind name below is still \
             correct for it"
        );
        let names = ALL_DECLARATION_KINDS
            .iter()
            .copied()
            .map(declaration_kind)
            .collect::<BTreeSet<_>>();
        assert_eq!(
            names.len(),
            ALL_DECLARATION_KINDS.len(),
            "two declaration kinds share an identity name, which would alias their identities"
        );
    }
}
