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
    /// The `ConjugatedPortDefinition` every `PortDefinition` owns, named `~` + its name, whose
    /// `PortConjugation` makes it the conjugate of that definition. Never authored.
    ConjugatedPortDefinition,
    PortUsage,
    OccurrenceDefinition,
    OccurrenceUsage,
    ConnectionDefinition,
    ConnectionUsage,
    InterfaceDefinition,
    InterfaceUsage,
    AllocationDefinition,
    AllocationUsage,
    FlowConnectionDefinition,
    FlowConnectionUsage,
    /// A `succession flow`: a flow whose transfer starts after its source ends
    /// (`SuccessionFlowUsage :> FlowUsage, SuccessionFlow`).
    SuccessionFlowUsage,
    ActionDefinition,
    ActionUsage,
    /// An `ActionUsage` carrying the `accept` action form.
    AcceptActionUsage,
    SendActionUsage,
    TerminateActionUsage,
    StateDefinition,
    StateUsage,
    /// An `exhibit` state: a StateUsage that is also a PerformActionUsage, never composite.
    ExhibitStateUsage,
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
    /// A definition with no more specific keyword, including the `#keyword def` extended form.
    Definition,
    /// A usage with no more specific keyword: the `#keyword <name>` extended usage form
    /// (`ExtendedUsage`, SysML BNF 341), whose metaclass is the abstract `Usage` itself.
    Usage,
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
    /// The abstract `ControlNode` metaclass every decision, merge, fork and join node
    /// specializes. No declaration is published with this kind; it exists so the metaclass
    /// generalization hierarchy states `ForkNode :> ControlNode :> ActionUsage`.
    ControlNode,
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
    /// A view body's `expose` member.
    Expose,
    /// `alias X for Y;`.
    ///
    /// The OMG models an alias as a `Membership` carrying an alternative name rather than as an
    /// element in its own right.
    Alias,
    Dependency,

    // --- Annotating elements ------------------------------------------------------------
    /// The abstract KerML `AnnotatingElement` every comment, documentation and textual
    /// representation specializes. No declaration is published with this kind.
    AnnotatingElement,
    Comment,
    Documentation,
    TextualRepresentation,

    // --- KerML types --------------------------------------------------------------------
    Type,
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
    /// The KerML `MultiplicityRange` an authored `[m..n]` lowers to, owned by the declaration
    /// it bounds.
    MultiplicityRange,

    // --- KerML features -----------------------------------------------------------------
    Feature,
    Step,
    Expression,
    BooleanExpression,
    /// The abstract KerML `LiteralExpression` every literal metaclass specializes. No
    /// declaration is published with this kind.
    LiteralExpression,
    LiteralBoolean,
    LiteralInteger,
    LiteralRational,
    LiteralString,
    NullExpression,
    MetadataAccessExpression,
    Connector,
    BindingConnector,
    /// The KerML `Succession` metaclass. Every published succession is a SysML
    /// `SuccessionAsUsage`, which specializes it; no declaration is published with this kind.
    Succession,
    Invariant,
    /// The KerML `FlowEnd` a flow's `from`/`to` endpoint lowers to: an end Feature owning the
    /// flow feature that redefines the endpoint's last segment.
    FlowEnd,
}

impl ElementKind {
    /// The kind whose [`ElementKind::as_str`] is `text`.
    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|kind| kind.as_str() == text)
    }

    /// The direct metaclass generalizations of this kind in the KerML/SysML abstract syntax,
    /// restricted to the metaclasses this vocabulary publishes.
    ///
    /// This is the language's own static metamodel (for instance `PartUsage :> ItemUsage`,
    /// `ConnectionDefinition :> PartDefinition, AssociationStructure`), not a fact about any
    /// admitted library. Relationship metaclasses (`Import`, `Expose`, `Dependency`) and the
    /// `Alias` membership generalize no published element metaclass.
    pub fn direct_generals(self) -> &'static [ElementKind] {
        use ElementKind as K;
        match self {
            K::Namespace
            | K::Import
            | K::Expose
            | K::Alias
            | K::Dependency
            | K::AnnotatingElement => &[],
            K::Comment | K::TextualRepresentation => &[K::AnnotatingElement],
            K::Documentation => &[K::Comment],
            K::Package => &[K::Namespace],
            K::LibraryPackage => &[K::Package],

            // KerML Core and Kernel.
            K::Type => &[K::Namespace],
            K::Classifier => &[K::Type],
            K::Class | K::DataType => &[K::Classifier],
            K::Structure => &[K::Class],
            K::Association => &[K::Classifier],
            K::AssociationStructure => &[K::Association, K::Structure],
            K::Metaclass => &[K::Structure],
            K::Behavior => &[K::Class],
            K::Function => &[K::Behavior],
            K::Predicate => &[K::Function],
            K::Interaction => &[K::Association, K::Behavior],
            K::Feature => &[K::Type],
            K::Multiplicity | K::Step | K::Connector => &[K::Feature],
            K::MultiplicityRange => &[K::Multiplicity],
            K::Expression => &[K::Step],
            K::BooleanExpression
            | K::LiteralExpression
            | K::NullExpression
            | K::MetadataAccessExpression => &[K::Expression],
            K::LiteralBoolean | K::LiteralInteger | K::LiteralRational | K::LiteralString => {
                &[K::LiteralExpression]
            }
            K::Invariant => &[K::BooleanExpression],
            K::FlowEnd => &[K::Feature],
            K::BindingConnector | K::Succession => &[K::Connector],

            // SysML definitions.
            K::Definition => &[K::Classifier],
            K::AttributeDefinition => &[K::Definition, K::DataType],
            K::EnumerationDefinition => &[K::AttributeDefinition],
            K::OccurrenceDefinition => &[K::Definition, K::Class],
            K::ItemDefinition => &[K::OccurrenceDefinition, K::Structure],
            K::PartDefinition => &[K::ItemDefinition],
            K::PortDefinition => &[K::OccurrenceDefinition, K::Structure],
            K::ConjugatedPortDefinition => &[K::PortDefinition],
            K::ConnectionDefinition => &[K::PartDefinition, K::AssociationStructure],
            K::InterfaceDefinition | K::AllocationDefinition => &[K::ConnectionDefinition],
            K::ActionDefinition => &[K::OccurrenceDefinition, K::Behavior],
            K::FlowConnectionDefinition => &[K::ActionDefinition, K::Interaction],
            K::StateDefinition => &[K::ActionDefinition],
            K::CalculationDefinition => &[K::ActionDefinition, K::Function],
            K::ConstraintDefinition => &[K::OccurrenceDefinition, K::Predicate],
            K::RequirementDefinition => &[K::ConstraintDefinition],
            K::ConcernDefinition | K::ViewpointDefinition => &[K::RequirementDefinition],
            K::CaseDefinition => &[K::CalculationDefinition],
            K::AnalysisCaseDefinition | K::VerificationCaseDefinition | K::UseCaseDefinition => {
                &[K::CaseDefinition]
            }
            K::ViewDefinition | K::RenderingDefinition => &[K::PartDefinition],
            K::MetadataDefinition => &[K::ItemDefinition, K::Metaclass],

            // SysML usages.
            K::Usage => &[K::Feature],
            K::ReferenceUsage | K::AttributeUsage | K::OccurrenceUsage => &[K::Usage],
            K::ForLoopVariable => &[K::ReferenceUsage],
            K::EnumerationUsage => &[K::AttributeUsage],
            K::ItemUsage | K::PortUsage => &[K::OccurrenceUsage],
            K::PartUsage => &[K::ItemUsage],
            K::ConnectionUsage => &[K::PartUsage, K::Connector],
            K::InterfaceUsage | K::AllocationUsage => &[K::ConnectionUsage],
            K::ActionUsage => &[K::OccurrenceUsage, K::Step],
            K::FlowConnectionUsage => &[K::ActionUsage, K::Connector],
            // KerML `SuccessionFlow :> Flow, Succession`; `Flow` is not published, so the
            // succession side is stated directly.
            K::SuccessionFlowUsage => &[K::FlowConnectionUsage, K::Succession],
            K::AcceptActionUsage
            | K::SendActionUsage
            | K::TerminateActionUsage
            | K::PerformActionUsage
            | K::TransitionUsage
            | K::AssignmentActionUsage
            | K::IfActionUsage
            | K::WhileLoopActionUsage
            | K::ForLoopActionUsage
            | K::ControlNode
            | K::StateUsage => &[K::ActionUsage],
            K::DecisionNode | K::MergeNode | K::ForkNode | K::JoinNode => &[K::ControlNode],
            K::FinalState => &[K::StateUsage],
            K::ExhibitStateUsage => &[K::StateUsage, K::PerformActionUsage],
            K::CalculationUsage => &[K::ActionUsage, K::Expression],
            K::ConstraintUsage => &[K::OccurrenceUsage, K::BooleanExpression],
            K::AssertConstraintUsage => &[K::ConstraintUsage, K::Invariant],
            K::RequirementUsage => &[K::ConstraintUsage],
            K::ConcernUsage | K::ViewpointUsage => &[K::RequirementUsage],
            K::SatisfyRequirementUsage => &[K::RequirementUsage, K::AssertConstraintUsage],
            K::CaseUsage => &[K::CalculationUsage],
            K::AnalysisCaseUsage | K::VerificationCaseUsage | K::UseCaseUsage => &[K::CaseUsage],
            K::ViewUsage | K::RenderingUsage => &[K::PartUsage],
            K::MetadataUsage => &[K::ItemUsage],
            K::SuccessionAsUsage => &[K::Usage, K::Succession],
            K::BindingConnectorAsUsage => &[K::Usage, K::BindingConnector],
        }
    }

    /// Whether this metaclass is `general` or (transitively) specializes it: OCL
    /// `oclIsKindOf(general)` over the published metaclass vocabulary.
    pub fn conforms_to(self, general: ElementKind) -> bool {
        self == general
            || self
                .direct_generals()
                .iter()
                .any(|direct| direct.conforms_to(general))
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
    /// `ObjectiveMembership` -- the objective requirement of a case.
    Objective,
    /// `ViewRenderingMembership` -- the rendering selected by a view definition or usage.
    ViewRendering,
    /// `TransitionFeatureMembership` with `kind = trigger` -- the accept action that triggers a
    /// transition.
    TransitionTriggerAction,
    /// `VariantMembership` -- an enumeration literal, or a variant of a variation.
    Variant,
    /// `ParameterMembership` -- a directed parameter or a bound argument.
    Parameter,
    /// `ReturnParameterMembership` -- the result parameter of a function or expression, authored
    /// with `return`.
    ReturnParameter,
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
            Self::Objective => "objective",
            Self::ViewRendering => "view-rendering",
            Self::TransitionTriggerAction => "transition-trigger-action",
            Self::Variant => "variant",
            Self::Parameter => "parameter",
            Self::ReturnParameter => "return-parameter",
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
    fn metaclass_generalization_is_acyclic_and_rooted() {
        for kind in ElementKind::ALL.iter().copied() {
            // Every chain terminates (recursion would overflow on a cycle) and no kind lists
            // itself or a duplicate as a direct general.
            let generals = kind.direct_generals();
            assert!(!generals.contains(&kind), "{kind} generalizes itself");
            let unique = generals.iter().collect::<BTreeSet<_>>();
            assert_eq!(unique.len(), generals.len(), "{kind} repeats a general");
            for general in generals {
                assert!(
                    !general.conforms_to(kind),
                    "{kind} and {general} form a cycle"
                );
            }
        }
    }

    #[test]
    fn metaclass_conformance_follows_the_metamodel() {
        assert!(ElementKind::PartDefinition.conforms_to(ElementKind::Structure));
        assert!(ElementKind::ConnectionDefinition.conforms_to(ElementKind::Association));
        assert!(ElementKind::AttributeDefinition.conforms_to(ElementKind::DataType));
        assert!(ElementKind::CaseUsage.conforms_to(ElementKind::Expression));
        assert!(ElementKind::DataType.conforms_to(ElementKind::Classifier));
        assert!(ElementKind::SuccessionAsUsage.conforms_to(ElementKind::Succession));
        assert!(ElementKind::Succession.conforms_to(ElementKind::Connector));
        assert!(ElementKind::ForkNode.conforms_to(ElementKind::ControlNode));
        assert!(ElementKind::ExhibitStateUsage.conforms_to(ElementKind::StateUsage));
        assert!(ElementKind::ExhibitStateUsage.conforms_to(ElementKind::PerformActionUsage));
        assert!(ElementKind::ControlNode.conforms_to(ElementKind::ActionUsage));
        assert!(!ElementKind::Classifier.conforms_to(ElementKind::DataType));
        assert!(!ElementKind::PartUsage.conforms_to(ElementKind::AttributeUsage));
        assert!(!ElementKind::ActionDefinition.conforms_to(ElementKind::Structure));
    }

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
            MembershipRole::ViewRendering,
            MembershipRole::TransitionTriggerAction,
            MembershipRole::Variant,
            MembershipRole::Parameter,
            MembershipRole::ReturnParameter,
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
