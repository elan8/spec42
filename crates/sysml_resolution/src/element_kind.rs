//! The published element-kind vocabulary.
//!
//! Two channels, deliberately separate:
//!
//! - [`ElementKind`] answers *what kind of element is this* in OMG metaclass terms.
//! - [`MembershipRole`] answers *what role does it play in its owner*, which the OMG models on the
//!   owning membership rather than on the element.
//!
//! Keeping them apart follows the OMG Pilot, whose element kind is simply the element's metaclass
//! while roles live in their own enumerations (`StateSubactionKind`, `TransitionFeatureKind`,
//! `FeatureDirectionKind`, ...) carried by the `*Membership` metaclasses. Folding a role into the
//! kind would publish something the specification does not call a kind: an `entry action` and a
//! `do action` are both an `ActionUsage`, and only their membership says which slot they fill.
//!
//! Neither of these is an identity. The identity encoding uses its own frozen kebab-case names
//! (`writer::declaration_kind`); [`ElementKind::as_str`] must never feed it, because the
//! many-to-one collapses below would alias distinct elements onto one identity.

use std::fmt;

macro_rules! element_kinds {
    ($( $(#[$meta:meta])* $variant:ident, )*) => {
        /// The kind of a published element, named for the OMG metaclass it denotes.
        ///
        /// Closed, and deliberately not `#[non_exhaustive]`: when a new declaration kind is
        /// lowered and gains a variant here, every exhaustive `match` downstream *should* fail to
        /// compile so a human decides what it means. That compile error is the whole point --
        /// a silent fall-through is exactly how completion items came to be uniformly mislabelled
        /// before this type existed.
        ///
        /// There is no `Unrecognized` escape hatch either. The projection from the private
        /// declaration domain is total and compiler-checked, so no value outside these variants
        /// is constructible, and an unreachable arm in every consumer would be pure noise. (The
        /// generator ABI's `Metaclass` does need one, because it crosses a process boundary where
        /// a guest may be older than the host. This type crosses only a crate boundary.)
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum ElementKind {
            $( $(#[$meta])* $variant, )*
        }

        impl ElementKind {
            /// Every kind, in declaration order.
            pub const ALL: &'static [ElementKind] = &[ $( ElementKind::$variant, )* ];

            /// The OMG metaclass name.
            pub fn as_str(self) -> &'static str {
                match self {
                    $( ElementKind::$variant => stringify!($variant), )*
                }
            }
        }
    };
}

element_kinds! {
    // --- Namespaces ---------------------------------------------------------------------
    Namespace,
    Package,
    LibraryPackage,

    // --- SysML definitions and usages ---------------------------------------------------
    PartDefinition,
    PartUsage,
    AttributeDefinition,
    AttributeUsage,
    EnumerationDefinition,
    /// Also an enumeration literal, which the OMG models as an `EnumerationUsage` owned by a
    /// `VariantMembership`; the literal case carries [`MembershipRole::Variant`].
    EnumerationUsage,
    ItemDefinition,
    ItemUsage,
    PortDefinition,
    PortUsage,
    OccurrenceDefinition,
    OccurrenceUsage,
    /// The standalone `individual def` form.
    ///
    /// The OMG has no such metaclass -- it is an `OccurrenceDefinition` with `isIndividual` set --
    /// but that flag is not published for this form, so collapsing would erase the keyword.
    IndividualDefinition,
    ConnectionDefinition,
    ConnectionUsage,
    InterfaceDefinition,
    InterfaceUsage,
    AllocationDefinition,
    AllocationUsage,
    FlowConnectionDefinition,
    FlowConnectionUsage,
    ActionDefinition,
    ActionUsage,
    StateDefinition,
    StateUsage,
    CalculationDefinition,
    CalculationUsage,
    ConstraintDefinition,
    ConstraintUsage,
    /// `assert constraint ...`.
    ///
    /// A concrete metaclass of its own: `AssertConstraintUsage <: Invariant, ConstraintUsage`.
    AssertConstraintUsage,
    RequirementDefinition,
    RequirementUsage,
    ConcernDefinition,
    ConcernUsage,
    CaseDefinition,
    CaseUsage,
    AnalysisCaseDefinition,
    AnalysisCaseUsage,
    VerificationCaseDefinition,
    VerificationCaseUsage,
    UseCaseDefinition,
    UseCaseUsage,
    ViewDefinition,
    ViewUsage,
    ViewpointDefinition,
    ViewpointUsage,
    RenderingDefinition,
    RenderingUsage,
    MetadataDefinition,
    MetadataUsage,
    /// A usage with no more specific keyword, including the `#keyword def` extended form.
    Definition,
    /// A referential feature: `ref x : T;`, a keyword-less binding, a parameter, a subject.
    ///
    /// The role-bearing cases carry a [`MembershipRole`].
    ReferenceUsage,

    // --- Behaviour and control flow -----------------------------------------------------
    PerformActionUsage,
    TransitionUsage,
    AssignmentActionUsage,
    IfActionUsage,
    /// Both `while <cond> { ... }` and bare `loop { ... }`, which are one grammar production --
    /// `loop` is `while` with an empty condition parameter.
    WhileLoopActionUsage,
    ForLoopActionUsage,
    /// The loop variable of a `for` loop.
    ///
    /// The OMG models it as a `ReferenceUsage` under the loop's variable `FeatureMembership`, but
    /// the variable and the loop body's own members share an owner *and* a membership kind here,
    /// so nothing else would tell them apart.
    ForLoopVariable,
    DecisionNode,
    MergeNode,
    ForkNode,
    JoinNode,
    /// A succession: `first X then Y;`, a bare `then <target>;` continuation, and a state body's
    /// initial-state `then`. All three are the same OMG metaclass; whether a source end was
    /// written is recoverable from the published references, and the initial-state case from the
    /// owner being a state.
    SuccessionAsUsage,
    /// A `final` pseudo-state.
    ///
    /// Parser-local: `'final'` does not appear in the SysML grammar at all, so there is no OMG
    /// metaclass to name. Nearest element shape is `StateUsage`.
    FinalState,

    // --- Relationship-shaped usages -----------------------------------------------------
    SatisfyRequirementUsage,
    BindingConnectorAsUsage,
    Import,
    /// `alias X for Y;`.
    ///
    /// The OMG models an alias as a `Membership` carrying an alternative name rather than as an
    /// element in its own right.
    Alias,
    Dependency,

    // --- KerML types --------------------------------------------------------------------
    Classifier,
    Class,
    Structure,
    Association,
    AssociationStructure,
    DataType,
    Metaclass,
    Behavior,
    Function,
    Predicate,
    Interaction,
    Multiplicity,

    // --- KerML features -----------------------------------------------------------------
    Feature,
    Step,
    Expression,
    BooleanExpression,
    Connector,
    BindingConnector,
    Invariant,
}

impl ElementKind {
    /// The kind whose [`ElementKind::as_str`] is `text`.
    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|kind| kind.as_str() == text)
    }
}

impl fmt::Display for ElementKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Which subaction slot of a state an action fills.
///
/// Mirrors the Pilot's `StateSubactionKind`. The three are the same element kind
/// ([`ElementKind::ActionUsage`]); only the membership distinguishes them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StateSubactionKind {
    Entry,
    Do,
    Exit,
}

/// Whether a constraint framed by a requirement is an assumption or a required constraint.
///
/// Mirrors the OMG `RequirementConstraintKind`. The specification also uses it on
/// `FramedConcernMembership` and `RequirementVerificationMembership`, whose authored forms this
/// crate does not yet distinguish, so it is published only for the `assume`/`require` pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequirementConstraintKind {
    /// `assume constraint ...`.
    Assumption,
    /// `require constraint ...`.
    Requirement,
}

/// The role an element plays in its owner, where the OMG carries that role on the owning
/// membership rather than on the element itself.
///
/// Absent for the ordinary case of a member whose owning membership adds no role. Only roles this
/// crate can actually derive are published; `RequirementConstraintKind` (assumption vs
/// requirement), which the Pilot also has, is not yet recoverable because `assume constraint` and
/// `require constraint` currently lower to the same declaration kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MembershipRole {
    /// `StateSubactionMembership` -- an `entry`/`do`/`exit` action of a state.
    StateSubaction(StateSubactionKind),
    /// `RequirementConstraintMembership` -- an `assume`/`require` constraint of a requirement.
    RequirementConstraint(RequirementConstraintKind),
    /// `SubjectMembership`.
    Subject,
    /// `StakeholderMembership`.
    Stakeholder,
    /// `ActorMembership`.
    Actor,
    /// `FramedConcernMembership` -- a concern framed by a requirement.
    FramedConcern,
    /// `RequirementVerificationMembership` -- a requirement verified by a case.
    RequirementVerification,
    /// `VariantMembership` -- an enumeration literal, or a variant of a variation.
    Variant,
    /// `ParameterMembership` -- a directed parameter, a return, or a bound argument.
    Parameter,
    /// `EndFeatureMembership` -- an association or connector end.
    EndFeature,
}

impl MembershipRole {
    /// A stable lower-case name, for debug rendering and snapshot output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StateSubaction(StateSubactionKind::Entry) => "entry",
            Self::StateSubaction(StateSubactionKind::Do) => "do",
            Self::StateSubaction(StateSubactionKind::Exit) => "exit",
            Self::RequirementConstraint(RequirementConstraintKind::Assumption) => "assumption",
            Self::RequirementConstraint(RequirementConstraintKind::Requirement) => "requirement",
            Self::Subject => "subject",
            Self::Stakeholder => "stakeholder",
            Self::Actor => "actor",
            Self::FramedConcern => "framed-concern",
            Self::RequirementVerification => "requirement-verification",
            Self::Variant => "variant",
            Self::Parameter => "parameter",
            Self::EndFeature => "end-feature",
        }
    }
}

impl fmt::Display for MembershipRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn every_kind_round_trips_through_its_name() {
        for kind in ElementKind::ALL.iter().copied() {
            assert_eq!(
                ElementKind::parse(kind.as_str()),
                Some(kind),
                "{kind:?} did not round-trip through {:?}",
                kind.as_str()
            );
        }
    }

    /// Two kinds sharing a name would make `parse` lossy and would make the published vocabulary
    /// ambiguous to anyone matching on the text.
    #[test]
    fn kind_names_are_unique() {
        let names = ElementKind::ALL
            .iter()
            .map(|kind| kind.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            names.len(),
            ElementKind::ALL.len(),
            "two element kinds share a name"
        );
    }

    #[test]
    fn parse_rejects_a_name_outside_the_vocabulary() {
        assert_eq!(ElementKind::parse("NotAMetaclass"), None);
        // The identity channel's kebab spellings are deliberately a different vocabulary.
        assert_eq!(ElementKind::parse("part-def"), None);
    }

    #[test]
    fn membership_role_names_are_unique() {
        let roles = [
            MembershipRole::StateSubaction(StateSubactionKind::Entry),
            MembershipRole::StateSubaction(StateSubactionKind::Do),
            MembershipRole::StateSubaction(StateSubactionKind::Exit),
            MembershipRole::RequirementConstraint(RequirementConstraintKind::Assumption),
            MembershipRole::RequirementConstraint(RequirementConstraintKind::Requirement),
            MembershipRole::Subject,
            MembershipRole::Stakeholder,
            MembershipRole::Actor,
            MembershipRole::FramedConcern,
            MembershipRole::RequirementVerification,
            MembershipRole::Variant,
            MembershipRole::Parameter,
            MembershipRole::EndFeature,
        ];
        let names = roles
            .iter()
            .map(|role| role.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            names.len(),
            roles.len(),
            "two membership roles share a name"
        );
    }
}
