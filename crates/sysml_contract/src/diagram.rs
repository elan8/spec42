//! What a diagram projection presents, named rather than described.
//!
//! Only the closed vocabulary lives here: which of SysML's eight view kinds a projection is, which
//! compartment a member is presented in and whether that membership is direct or inherited, and
//! what role a vertex plays in a state machine. The projected scene itself -- its elements, edges
//! and incompleteness reasons -- carries model identities and authored text, so it stays with the
//! authority that computes it.

/// One of the eight view kinds SysML's standard view definitions declare.
///
/// A closed set fixed by the specification, not an open registry: a consumer matching on it is
/// exhaustive today and stays exhaustive until OMG adds a view definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagramViewKind {
    General,
    Interconnection,
    ActionFlow,
    StateTransition,
    Sequence,
    Browser,
    Grid,
    Geometry,
}

impl DiagramViewKind {
    /// Every view kind, in specification order, so a catalog has one fixed order.
    pub const ALL: [Self; 8] = [
        Self::General,
        Self::Interconnection,
        Self::ActionFlow,
        Self::StateTransition,
        Self::Sequence,
        Self::Browser,
        Self::Grid,
        Self::Geometry,
    ];

    /// A stable kebab-case identifier, for catalog keys and snapshot output.
    pub const fn id(self) -> &'static str {
        match self {
            Self::General => "general-view",
            Self::Interconnection => "interconnection-view",
            Self::ActionFlow => "action-flow-view",
            Self::StateTransition => "state-transition-view",
            Self::Sequence => "sequence-view",
            Self::Browser => "browser-view",
            Self::Grid => "grid-view",
            Self::Geometry => "geometry-view",
        }
    }
}

/// Which compartment of a presented element a member appears in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagramCompartmentKind {
    Attributes,
    Parts,
    Ports,
    Items,
    Constraints,
    Requirements,
    Actions,
    States,
    Calculations,
    Connections,
    Interfaces,
    Occurrences,
}

/// Whether a compartment's members are declared on the element or inherited into it.
///
/// Kept beside the compartment rather than folded into it: a viewer that dims inherited members
/// needs the distinction, and a compartment that mixed the two could not supply it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagramCompartmentProvenance {
    Direct,
    Inherited,
}

/// The role one vertex plays in a projected state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramStateVertexKind {
    Initial,
    State,
    Final,
}

/// Which relationship one projected diagram relationship states.
///
/// The projection used to carry this as the canonical name's text, which made every consumer
/// that dispatched on it -- the edge composer here, the generator boundary, a renderer -- a
/// string comparison that no compiler checks and that a typo turns into a silently missing edge.
/// The variants are exactly the reference kinds the resolution authority publishes, one per
/// canonical name; [`DiagramRelationshipKind::name`] is that name, so the published text is
/// unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagramRelationshipKind {
    NamespaceImport,
    MembershipImport,
    FilterImport,
    FeatureTyping,
    TypeFeaturing,
    FeatureChaining,
    Specialization,
    Subsetting,
    Redefinition,
    ReferenceSubsetting,
    CrossSubsetting,
    Intersects,
    Unioning,
    Intersecting,
    Differencing,
    Disjoining,
    AliasBinding,
    ConnectorEnd,
    Succession,
    EntryActionBinding,
    DoActionBinding,
    ExitActionBinding,
    InitialState,
    ExpressionOperand,
    TransitionSource,
    TransitionTarget,
    TransitionTrigger,
    TransitionEffect,
    MetadataAnnotation,
    FilterMetadataTest,
    SatisfySource,
    SatisfyTarget,
    AllocateSource,
    AllocateTarget,
    BindSource,
    BindTarget,
    Variant,
    IncludeUseCase,
    ViewExpose,
    MemberAccessOperand,
    InvocationCallee,
    ThenTarget,
    AcceptVia,
    SendTarget,
    AcceptPayloadType,
    TerminateTarget,
    FlowSource,
    FlowTarget,
    TypeCheckTarget,
    MetaCastTarget,
    StakeholderTarget,
    PurposeTarget,
    VerifyRequirementTarget,
    AssignTarget,
    DependencyClient,
    DependencySupplier,
    PerformParameterTarget,
    FlowPayloadType,
}

impl DiagramRelationshipKind {
    /// Every relationship kind, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::NamespaceImport,
        Self::MembershipImport,
        Self::FilterImport,
        Self::FeatureTyping,
        Self::TypeFeaturing,
        Self::FeatureChaining,
        Self::Specialization,
        Self::Subsetting,
        Self::Redefinition,
        Self::ReferenceSubsetting,
        Self::CrossSubsetting,
        Self::Intersects,
        Self::Unioning,
        Self::Intersecting,
        Self::Differencing,
        Self::Disjoining,
        Self::AliasBinding,
        Self::ConnectorEnd,
        Self::Succession,
        Self::EntryActionBinding,
        Self::DoActionBinding,
        Self::ExitActionBinding,
        Self::InitialState,
        Self::ExpressionOperand,
        Self::TransitionSource,
        Self::TransitionTarget,
        Self::TransitionTrigger,
        Self::TransitionEffect,
        Self::MetadataAnnotation,
        Self::FilterMetadataTest,
        Self::SatisfySource,
        Self::SatisfyTarget,
        Self::AllocateSource,
        Self::AllocateTarget,
        Self::BindSource,
        Self::BindTarget,
        Self::Variant,
        Self::IncludeUseCase,
        Self::ViewExpose,
        Self::MemberAccessOperand,
        Self::InvocationCallee,
        Self::ThenTarget,
        Self::AcceptVia,
        Self::SendTarget,
        Self::AcceptPayloadType,
        Self::TerminateTarget,
        Self::FlowSource,
        Self::FlowTarget,
        Self::TypeCheckTarget,
        Self::MetaCastTarget,
        Self::StakeholderTarget,
        Self::PurposeTarget,
        Self::VerifyRequirementTarget,
        Self::AssignTarget,
        Self::DependencyClient,
        Self::DependencySupplier,
        Self::PerformParameterTarget,
        Self::FlowPayloadType,
    ];

    /// The canonical name of this relationship kind.
    pub const fn name(self) -> &'static str {
        match self {
            Self::NamespaceImport => "namespaceImport",
            Self::MembershipImport => "membershipImport",
            Self::FilterImport => "filterImport",
            Self::FeatureTyping => "featureTyping",
            Self::TypeFeaturing => "typeFeaturing",
            Self::FeatureChaining => "featureChaining",
            Self::Specialization => "specialization",
            Self::Subsetting => "subsetting",
            Self::Redefinition => "redefinition",
            Self::ReferenceSubsetting => "referenceSubsetting",
            Self::CrossSubsetting => "crossSubsetting",
            Self::Intersects => "intersects",
            Self::Unioning => "unioning",
            Self::Intersecting => "intersecting",
            Self::Differencing => "differencing",
            Self::Disjoining => "disjoining",
            Self::AliasBinding => "aliasBinding",
            Self::ConnectorEnd => "connectorEnd",
            Self::Succession => "succession",
            Self::EntryActionBinding => "entryActionBinding",
            Self::DoActionBinding => "doActionBinding",
            Self::ExitActionBinding => "exitActionBinding",
            Self::InitialState => "initialState",
            Self::ExpressionOperand => "expressionOperand",
            Self::TransitionSource => "transitionSource",
            Self::TransitionTarget => "transitionTarget",
            Self::TransitionTrigger => "transitionTrigger",
            Self::TransitionEffect => "transitionEffect",
            Self::MetadataAnnotation => "metadataAnnotation",
            Self::FilterMetadataTest => "filterMetadataTest",
            Self::SatisfySource => "satisfySource",
            Self::SatisfyTarget => "satisfyTarget",
            Self::AllocateSource => "allocateSource",
            Self::AllocateTarget => "allocateTarget",
            Self::BindSource => "bindSource",
            Self::BindTarget => "bindTarget",
            Self::Variant => "variant",
            Self::IncludeUseCase => "includeUseCase",
            Self::ViewExpose => "viewExpose",
            Self::MemberAccessOperand => "memberAccessOperand",
            Self::InvocationCallee => "invocationCallee",
            Self::ThenTarget => "thenTarget",
            Self::AcceptVia => "acceptVia",
            Self::SendTarget => "sendTarget",
            Self::AcceptPayloadType => "acceptPayloadType",
            Self::TerminateTarget => "terminateTarget",
            Self::FlowSource => "flowSource",
            Self::FlowTarget => "flowTarget",
            Self::TypeCheckTarget => "typeCheckTarget",
            Self::MetaCastTarget => "metaCastTarget",
            Self::StakeholderTarget => "stakeholderTarget",
            Self::PurposeTarget => "purposeTarget",
            Self::VerifyRequirementTarget => "verifyRequirementTarget",
            Self::AssignTarget => "assignTarget",
            Self::DependencyClient => "dependencyClient",
            Self::DependencySupplier => "dependencySupplier",
            Self::PerformParameterTarget => "performParameterTarget",
            Self::FlowPayloadType => "flowPayloadType",
        }
    }
}
