//! Core semantic graph node identity and relationship kinds.

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::semantic::text_span::TextRange;
use url::Url;

fn serialize_url<S: Serializer>(url: &Url, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(url.as_str())
}

fn deserialize_url<'de, D: Deserializer<'de>>(d: D) -> Result<Url, D::Error> {
    let s = String::deserialize(d)?;
    Url::parse(&s).map_err(serde::de::Error::custom)
}

/// Unique identifier for a node in the semantic graph.
/// Combines document URI and qualified name for workspace-wide uniqueness.
#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeId {
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    pub uri: Url,
    pub qualified_name: String,
}

impl NodeId {
    pub fn new(uri: &Url, qualified_name: impl Into<String>) -> Self {
        Self {
            uri: uri.clone(),
            qualified_name: qualified_name.into(),
        }
    }
}

/// SysML v2 relationship kinds (edges in the graph).
#[allow(dead_code)] // some relationship kinds are staged for upcoming semantic features
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipKind {
    Typing,
    Specializes,
    /// Explicit `subsets` / `:>` feature relationship on a usage.
    Subsetting,
    /// Explicit `redefines` / `:>>` feature relationship on a usage.
    Redefinition,
    /// KerML 8.3.4.4 `ReferenceSubsetting`: explicit `references` / `::>` feature relationship
    /// on a usage -- a `Subsetting` specialization, distinct from plain `Subsetting`.
    ReferenceSubsetting,
    /// KerML 8.3.4.5 `CrossSubsetting`: explicit `crosses` / `=>` feature relationship on a
    /// usage -- also a `Subsetting` specialization, distinct from plain `Subsetting`.
    CrossSubsetting,
    Connection,
    Bind,
    /// Control/data flow relationship inside behaviors (e.g. `flow`, `first ... then ...`).
    Flow,
    /// `FlowUsageKind::SuccessionFlow` — a flow that also implies a succession/ordering
    /// constraint between the connected actions, distinct from a plain data/control [`Flow`](Self::Flow).
    SuccessionFlow,
    Perform,
    Allocate,
    /// Explicit KerML/SysML `dependency ... from client to supplier` relationship.
    Dependency,
    Satisfy,
    #[serde(alias = "verification")]
    Subject,
    Reference,
    Derivation,
    Transition,
    /// `then` initial state in a state composite (`transition` without `first` uses the same resolution path with [`RelationshipKind::Transition`]).
    InitialState,
    /// Metadata usage annotates a model element (`annotatedElement` per SysML §7.27).
    Annotation,
    /// KerML 8.3.12.4 `PortConjugation`: a kind of `Conjugation` connecting a
    /// `ConjugatedPortDefinition` (source) back to its `originalPortDefinition` (target) --
    /// distinct from `Typing`/`Specializes`, not a FeatureTyping-family relationship.
    PortConjugation,
}

/// SysML v2 element kinds (node classification in the semantic graph).
///
/// Replaces the previous `element_kind: String` field on [`SemanticNode`] with a
/// type-safe enum. `as_str` returns the canonical lowercase spelling that was
/// stored before, and `parse` parses it back (falling back to
/// [`ElementKind::Unknown`] for forward-compatibility).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub enum ElementKind {
    // Definitions
    Package,
    PartDef,
    PortDef,
    Interface,
    InterfaceDef,
    ItemDef,
    AttributeDef,
    ActionDef,
    OccurrenceDef,
    FlowDef,
    AllocationDef,
    StateDef,
    RequirementDef,
    UseCaseDef,
    ConcernDef,
    AnalysisDef,
    VerificationDef,
    ViewDef,
    ViewpointDef,
    RenderingDef,
    MetadataDef,
    EnumDef,
    ConstraintDef,
    CalcDef,
    CaseDef,
    IndividualDef,
    ConnectionDef,
    Alias,
    KermlDecl,
    /// Parser-backed KerML classifier declaration (`class`, `classifier`, etc.).
    ClassifierDecl,
    // Usages
    Part,
    Port,
    Item,
    Attribute,
    Action,
    Actor,
    Stakeholder,
    State,
    Requirement,
    /// Usage of a `case def` (e.g. `case c : SomeCase;`). Kind-string `"case"`, mirroring
    /// `ElementKind::CaseDef`'s existing `"case def"` — package-level case usages already
    /// materialized with this string via `graph_builder/package_body`'s dispatch, but had no
    /// `From<&str>` arm and fell through to `Unknown("case")` until this variant existed.
    Case,
    UseCase,
    Concern,
    Analysis,
    Verification,
    View,
    Viewpoint,
    Rendering,
    ViewRendering,
    /// A `columnView` (or other) redefinition inside a `render`/`rendering` usage body -- Grid
    /// View's `asElementTable`/`columnView` column-configuration mechanism (SysML v2
    /// `Views.sysml`'s `view columnView[0..*] ordered { ... }`).
    ViewColumn,
    MetadataUsage,
    MetadataKeyword,
    Flow,
    Allocation,
    Perform,
    Subject,
    VerifiedRequirement,
    IncludeUseCase,
    Ref,
    Constraint,
    Connection,
    Individual,
    Occurrence,
    Calc,
    /// Usage of an `enum def` (e.g. `enum status : Status;`). Kind-string `"enumeration"`,
    /// matching the pre-existing (but previously unmapped) node-kind string already produced by
    /// `graph_builder/part_def.rs`'s nested-in-part-def handler.
    Enumeration,
    // Sub-elements / structural
    Transition,
    /// KerML 8.3.18.8 `TransitionFeatureMembership` child: the `accept` trigger of a
    /// `transition` statement. Baseline slice: a simplified/uniform structured
    /// representation via attributes, not full `AcceptActionUsage` typing (deferred).
    TransitionTrigger,
    /// KerML 8.3.18.8 `TransitionFeatureMembership` child: the `if` guard of a `transition`
    /// statement. Its content is the guard's real `Node<Expression>`, losslessly converted
    /// and exposed as an addressable `Expression` via `HostElementFacts::content_expression_id`.
    TransitionGuard,
    /// KerML 8.3.18.8 `TransitionFeatureMembership` child: the `do` effect of a `transition`
    /// statement. Baseline slice: a simplified/uniform structured representation via
    /// attributes, not full `ActionUsage` typing (deferred).
    TransitionEffect,
    FinalState,
    /// One named value inside an `enum def { ... }` body (e.g. `active;`). Owned by the
    /// enclosing `EnumDef`; not independently typed or specialized.
    EnumeratedValue,
    /// Standalone `first <step>;` action-body marker. This is an owned initial control node
    /// whose outgoing `Flow` identifies the behavior's first step.
    Initial,
    ForLoop,
    Assign,
    Assert,
    AssertConstraint,
    RequireConstraint,
    Binding,
    Merge,
    Decide,
    Join,
    Fork,
    Terminate,
    While,
    If,
    Else,
    Filter,
    Import,
    Dependency,
    /// Addressable `doc /* … */` documentation element.
    Documentation,
    /// Addressable `rep <name>? language "..." { ... }` textual representation.
    TextualRepresentation,
    /// KerML 8.3.12.2 `ConjugatedPortDefinition`: the implicit conjugate of a `port def`,
    /// materialized eagerly as a nested member whenever the port definition itself is
    /// materialized (SysML v2 8.2.2.12 Note 1) -- not lazily on first `~`-typed usage.
    ConjugatedPortDefinition,
    /// SysML v2 8.2.2.16 `PayloadFeature`: the `of X` clause on a named flow usage, an
    /// optionally-named `Feature` typed by (and/or given a multiplicity by) `X`.
    FlowPayload,
    DerivationConnection,
    InterfaceEnd,
    InOutParameter,
    // Analysis-specific
    AnalysisResult,
    Verdict,
    Diagnostic,
    Need,
    Objective,
    Purpose,
    Verify,
    /// Catch-all for unrecognized element kinds (forward-compatibility).
    Unknown(String),
}

impl ElementKind {
    /// Returns the canonical lowercase string spelling (matches what was stored
    /// before as `element_kind: String`).
    pub fn as_str(&self) -> &str {
        match self {
            ElementKind::Package => "package",
            ElementKind::PartDef => "part def",
            ElementKind::PortDef => "port def",
            ElementKind::Interface => "interface",
            ElementKind::InterfaceDef => "interface def",
            ElementKind::ItemDef => "item def",
            ElementKind::AttributeDef => "attribute def",
            ElementKind::ActionDef => "action def",
            ElementKind::OccurrenceDef => "occurrence def",
            ElementKind::FlowDef => "flow def",
            ElementKind::AllocationDef => "allocation def",
            ElementKind::StateDef => "state def",
            ElementKind::RequirementDef => "requirement def",
            ElementKind::UseCaseDef => "use case def",
            ElementKind::ConcernDef => "concern def",
            ElementKind::AnalysisDef => "analysis def",
            ElementKind::VerificationDef => "verification def",
            ElementKind::ViewDef => "view def",
            ElementKind::ViewpointDef => "viewpoint def",
            ElementKind::RenderingDef => "rendering def",
            ElementKind::MetadataDef => "metadata def",
            ElementKind::EnumDef => "enum def",
            ElementKind::ConstraintDef => "constraint def",
            ElementKind::CalcDef => "calc def",
            ElementKind::CaseDef => "case def",
            ElementKind::IndividualDef => "individual def",
            ElementKind::ConnectionDef => "connection def",
            ElementKind::Alias => "alias",
            ElementKind::KermlDecl => "kermlDecl",
            ElementKind::ClassifierDecl => "classifier decl",
            ElementKind::Part => "part",
            ElementKind::Port => "port",
            ElementKind::Item => "item",
            ElementKind::Attribute => "attribute",
            ElementKind::Action => "action",
            ElementKind::Actor => "actor",
            ElementKind::Stakeholder => "stakeholder",
            ElementKind::State => "state",
            ElementKind::Requirement => "requirement",
            ElementKind::Case => "case",
            ElementKind::UseCase => "use case",
            ElementKind::Concern => "concern",
            ElementKind::Analysis => "analysis",
            ElementKind::Verification => "verification",
            ElementKind::View => "view",
            ElementKind::Viewpoint => "viewpoint",
            ElementKind::Rendering => "rendering",
            ElementKind::ViewRendering => "view rendering",
            ElementKind::ViewColumn => "view column",
            ElementKind::MetadataUsage => "metadata usage",
            ElementKind::MetadataKeyword => "metadata keyword",
            ElementKind::Flow => "flow",
            ElementKind::Allocation => "allocation",
            ElementKind::Perform => "perform",
            ElementKind::Subject => "subject",
            ElementKind::VerifiedRequirement => "verified requirement",
            ElementKind::IncludeUseCase => "include use case",
            ElementKind::Ref => "ref",
            ElementKind::Constraint => "constraint",
            ElementKind::Connection => "connection",
            ElementKind::Individual => "individual",
            ElementKind::Occurrence => "occurrence",
            ElementKind::Calc => "calc",
            ElementKind::Enumeration => "enumeration",
            ElementKind::Transition => "transition",
            ElementKind::TransitionTrigger => "transition trigger",
            ElementKind::TransitionGuard => "transition guard",
            ElementKind::TransitionEffect => "transition effect",
            ElementKind::FinalState => "final state",
            ElementKind::EnumeratedValue => "enumerated value",
            ElementKind::Initial => "initial",
            ElementKind::ForLoop => "for loop",
            ElementKind::Assign => "assign",
            ElementKind::Assert => "assert",
            ElementKind::AssertConstraint => "assert constraint",
            ElementKind::RequireConstraint => "require constraint",
            ElementKind::Binding => "binding",
            ElementKind::Merge => "merge",
            ElementKind::Decide => "decide",
            ElementKind::Join => "join",
            ElementKind::Fork => "fork",
            ElementKind::Terminate => "terminate",
            ElementKind::While => "while",
            ElementKind::If => "if",
            ElementKind::Else => "else",
            ElementKind::Filter => "filter",
            ElementKind::Import => "import",
            ElementKind::Dependency => "dependency",
            ElementKind::Documentation => "documentation",
            ElementKind::TextualRepresentation => "textualRep",
            ElementKind::ConjugatedPortDefinition => "conjugated port definition",
            ElementKind::FlowPayload => "flow payload",
            ElementKind::DerivationConnection => "derivation connection",
            ElementKind::InterfaceEnd => "interface end",
            ElementKind::InOutParameter => "in out parameter",
            ElementKind::AnalysisResult => "analysis result",
            ElementKind::Verdict => "verdict",
            ElementKind::Diagnostic => "diagnostic",
            ElementKind::Need => "need",
            ElementKind::Objective => "objective",
            ElementKind::Purpose => "purpose",
            ElementKind::Verify => "verify",
            ElementKind::Unknown(s) => s.as_str(),
        }
    }

    /// Returns the complete universal relationship required by this concrete SysML kind.
    /// The relationship kind and target are intentionally inseparable: downstream layers must
    /// not recover the distinction from a generic definition/usage test.
    pub fn universal_standard_library_relationship(
        &self,
    ) -> Option<UniversalStandardLibraryRelationship> {
        use StandardLibraryElement as Target;
        use UniversalStandardLibraryRelationship as Relationship;
        match self {
            Self::AttributeDef | Self::EnumDef => {
                Some(Relationship::specializes(Target::BaseDataValue))
            }
            Self::Attribute | Self::Enumeration | Self::EnumeratedValue => {
                Some(Relationship::subsets(Target::BaseDataValues))
            }
            Self::IndividualDef => Some(Relationship::specializes(Target::OccurrencesLife)),
            Self::Occurrence | Self::Individual => {
                Some(Relationship::subsets(Target::OccurrencesOccurrences))
            }
            Self::ItemDef => Some(Relationship::specializes(Target::ItemsItem)),
            Self::Item => Some(Relationship::subsets(Target::ItemsItems)),
            Self::PartDef => Some(Relationship::specializes(Target::PartsPart)),
            Self::Part | Self::Actor | Self::Stakeholder => {
                Some(Relationship::subsets(Target::PartsParts))
            }
            Self::PortDef | Self::ConjugatedPortDefinition => {
                Some(Relationship::specializes(Target::PortsPort))
            }
            Self::Port => Some(Relationship::subsets(Target::PortsPorts)),
            Self::ConnectionDef => Some(Relationship::specializes(Target::ConnectionsConnection)),
            Self::Connection => Some(Relationship::subsets(Target::ConnectionsConnections)),
            Self::InterfaceDef => Some(Relationship::specializes(Target::InterfacesInterface)),
            Self::Interface => Some(Relationship::subsets(Target::InterfacesInterfaces)),
            Self::AllocationDef => Some(Relationship::specializes(Target::AllocationsAllocation)),
            Self::Allocation => Some(Relationship::subsets(Target::AllocationsAllocations)),
            Self::FlowDef => Some(Relationship::specializes(Target::FlowsMessageAction)),
            Self::Flow => Some(Relationship::subsets(Target::FlowsMessages)),
            Self::ActionDef => Some(Relationship::specializes(Target::ActionsAction)),
            Self::Action | Self::Perform => Some(Relationship::subsets(Target::ActionsActions)),
            Self::Assign => Some(Relationship::subsets(Target::ActionsAssignmentActions)),
            Self::ForLoop => Some(Relationship::subsets(Target::ActionsForLoopActions)),
            Self::Terminate => Some(Relationship::subsets(Target::ActionsTerminateActions)),
            Self::While => Some(Relationship::subsets(Target::ActionsWhileLoopActions)),
            Self::If | Self::Else => Some(Relationship::subsets(Target::ActionsIfThenActions)),
            Self::Transition => Some(Relationship::subsets(Target::ActionsTransitionActions)),
            Self::TransitionTrigger => Some(Relationship::subsets(Target::ActionsAcceptActions)),
            Self::TransitionEffect => Some(Relationship::subsets(Target::ActionsActions)),
            Self::StateDef => Some(Relationship::specializes(Target::StatesStateAction)),
            Self::State | Self::FinalState => {
                Some(Relationship::subsets(Target::StatesStateActions))
            }
            Self::CalcDef => Some(Relationship::specializes(Target::CalculationsCalculation)),
            Self::Calc => Some(Relationship::subsets(Target::CalculationsCalculations)),
            Self::ConstraintDef => Some(Relationship::specializes(
                Target::ConstraintsConstraintCheck,
            )),
            Self::Constraint | Self::Assert | Self::AssertConstraint | Self::RequireConstraint => {
                Some(Relationship::subsets(Target::ConstraintsConstraintChecks))
            }
            Self::RequirementDef => Some(Relationship::specializes(
                Target::RequirementsRequirementCheck,
            )),
            Self::Requirement | Self::VerifiedRequirement | Self::Objective => {
                Some(Relationship::subsets(Target::RequirementsRequirementChecks))
            }
            Self::ConcernDef => Some(Relationship::specializes(Target::RequirementsConcernCheck)),
            Self::Concern => Some(Relationship::subsets(Target::RequirementsConcernChecks)),
            Self::CaseDef => Some(Relationship::specializes(Target::CasesCase)),
            Self::Case => Some(Relationship::subsets(Target::CasesCases)),
            Self::AnalysisDef => Some(Relationship::specializes(Target::AnalysisCasesAnalysisCase)),
            Self::Analysis => Some(Relationship::subsets(Target::AnalysisCasesAnalysisCases)),
            Self::VerificationDef => Some(Relationship::specializes(
                Target::VerificationCasesVerificationCase,
            )),
            Self::Verification => Some(Relationship::subsets(
                Target::VerificationCasesVerificationCases,
            )),
            Self::UseCaseDef => Some(Relationship::specializes(Target::UseCasesUseCase)),
            Self::UseCase | Self::IncludeUseCase => {
                Some(Relationship::subsets(Target::UseCasesUseCases))
            }
            Self::RenderingDef => Some(Relationship::specializes(Target::ViewsRendering)),
            Self::Rendering | Self::ViewRendering => {
                Some(Relationship::subsets(Target::ViewsRenderings))
            }
            Self::ViewDef => Some(Relationship::specializes(Target::ViewsView)),
            Self::View => Some(Relationship::subsets(Target::ViewsViews)),
            Self::ViewpointDef => Some(Relationship::specializes(Target::ViewsViewpoint)),
            Self::Viewpoint => Some(Relationship::subsets(Target::ViewsViewpoints)),
            Self::MetadataDef => Some(Relationship::specializes(Target::MetadataMetadataItem)),
            Self::MetadataUsage | Self::MetadataKeyword => {
                Some(Relationship::subsets(Target::MetadataMetadataItems))
            }
            _ => None,
        }
    }

    /// Parses a canonical element-kind string, falling back to
    /// [`ElementKind::Unknown`] for unrecognized spellings.
    pub fn parse(s: &str) -> Self {
        match s {
            "package" => ElementKind::Package,
            "part def" => ElementKind::PartDef,
            "port def" => ElementKind::PortDef,
            "interface" => ElementKind::Interface,
            "interface def" => ElementKind::InterfaceDef,
            "item def" => ElementKind::ItemDef,
            "attribute def" => ElementKind::AttributeDef,
            "action def" => ElementKind::ActionDef,
            "occurrence def" => ElementKind::OccurrenceDef,
            "flow def" => ElementKind::FlowDef,
            "allocation def" => ElementKind::AllocationDef,
            "state def" => ElementKind::StateDef,
            "requirement def" => ElementKind::RequirementDef,
            "use case def" => ElementKind::UseCaseDef,
            "concern def" => ElementKind::ConcernDef,
            "analysis def" => ElementKind::AnalysisDef,
            "verification def" => ElementKind::VerificationDef,
            "view def" => ElementKind::ViewDef,
            "viewpoint def" => ElementKind::ViewpointDef,
            "rendering def" => ElementKind::RenderingDef,
            "metadata def" => ElementKind::MetadataDef,
            "enum def" => ElementKind::EnumDef,
            "constraint def" => ElementKind::ConstraintDef,
            "calc def" => ElementKind::CalcDef,
            "case def" => ElementKind::CaseDef,
            "individual def" => ElementKind::IndividualDef,
            "connection def" => ElementKind::ConnectionDef,
            "alias" => ElementKind::Alias,
            "kermlDecl" => ElementKind::KermlDecl,
            "classifier decl" => ElementKind::ClassifierDecl,
            "part" => ElementKind::Part,
            "port" => ElementKind::Port,
            "item" => ElementKind::Item,
            "attribute" => ElementKind::Attribute,
            "action" => ElementKind::Action,
            "actor" => ElementKind::Actor,
            "stakeholder" => ElementKind::Stakeholder,
            "state" => ElementKind::State,
            "requirement" | "requirement usage" => ElementKind::Requirement,
            "case" => ElementKind::Case,
            "use case" => ElementKind::UseCase,
            "concern" => ElementKind::Concern,
            "analysis" => ElementKind::Analysis,
            "verification" | "verification case" => ElementKind::Verification,
            "view" => ElementKind::View,
            "viewpoint" => ElementKind::Viewpoint,
            "rendering" => ElementKind::Rendering,
            "view rendering" => ElementKind::ViewRendering,
            "view column" => ElementKind::ViewColumn,
            "metadata usage" => ElementKind::MetadataUsage,
            "metadata keyword" => ElementKind::MetadataKeyword,
            "flow" => ElementKind::Flow,
            "allocation" => ElementKind::Allocation,
            "perform" => ElementKind::Perform,
            "subject" => ElementKind::Subject,
            "verified requirement" => ElementKind::VerifiedRequirement,
            "include use case" => ElementKind::IncludeUseCase,
            "ref" => ElementKind::Ref,
            "constraint" => ElementKind::Constraint,
            "connection" => ElementKind::Connection,
            "individual" => ElementKind::Individual,
            "occurrence" => ElementKind::Occurrence,
            "calc" => ElementKind::Calc,
            "enumeration" => ElementKind::Enumeration,
            "transition" => ElementKind::Transition,
            "transition trigger" => ElementKind::TransitionTrigger,
            "transition guard" => ElementKind::TransitionGuard,
            "transition effect" => ElementKind::TransitionEffect,
            "final state" => ElementKind::FinalState,
            "enumerated value" => ElementKind::EnumeratedValue,
            "initial" => ElementKind::Initial,
            "for loop" => ElementKind::ForLoop,
            "assign" => ElementKind::Assign,
            "assert" => ElementKind::Assert,
            "assert constraint" => ElementKind::AssertConstraint,
            "require constraint" => ElementKind::RequireConstraint,
            "binding" => ElementKind::Binding,
            "merge" => ElementKind::Merge,
            "decide" => ElementKind::Decide,
            "join" => ElementKind::Join,
            "fork" => ElementKind::Fork,
            "terminate" => ElementKind::Terminate,
            "while" => ElementKind::While,
            "if" => ElementKind::If,
            "else" => ElementKind::Else,
            "filter" => ElementKind::Filter,
            "import" => ElementKind::Import,
            "dependency" => ElementKind::Dependency,
            "documentation" => ElementKind::Documentation,
            "textualRep" => ElementKind::TextualRepresentation,
            "conjugated port definition" => ElementKind::ConjugatedPortDefinition,
            "flow payload" => ElementKind::FlowPayload,
            "derivation connection" => ElementKind::DerivationConnection,
            "interface end" => ElementKind::InterfaceEnd,
            "in out parameter" => ElementKind::InOutParameter,
            "analysis result" => ElementKind::AnalysisResult,
            "verdict" => ElementKind::Verdict,
            "diagnostic" => ElementKind::Diagnostic,
            "need" => ElementKind::Need,
            "objective" => ElementKind::Objective,
            "purpose" => ElementKind::Purpose,
            "verify" => ElementKind::Verify,
            other => ElementKind::Unknown(other.to_string()),
        }
    }

    /// True for SysML v2 `Definition` elements (the `xxx def` declarations) — reusable
    /// classifiers that usages are typed by. Deliberately excludes `Package` (a `Namespace`,
    /// not a `Definition` in the SysML metamodel) and structural declarations like `Alias`/
    /// `KermlDecl`. Exhaustive match — no string suffix/substring comparisons.
    pub fn is_definition(&self) -> bool {
        matches!(
            self,
            ElementKind::PartDef
                | ElementKind::PortDef
                | ElementKind::InterfaceDef
                | ElementKind::ItemDef
                | ElementKind::AttributeDef
                | ElementKind::ActionDef
                | ElementKind::OccurrenceDef
                | ElementKind::FlowDef
                | ElementKind::AllocationDef
                | ElementKind::StateDef
                | ElementKind::RequirementDef
                | ElementKind::UseCaseDef
                | ElementKind::ConcernDef
                | ElementKind::AnalysisDef
                | ElementKind::VerificationDef
                | ElementKind::ViewDef
                | ElementKind::ViewpointDef
                | ElementKind::RenderingDef
                | ElementKind::MetadataDef
                | ElementKind::EnumDef
                | ElementKind::ConstraintDef
                | ElementKind::CalcDef
                | ElementKind::CaseDef
                | ElementKind::IndividualDef
                | ElementKind::ConnectionDef
                | ElementKind::ConjugatedPortDefinition
        )
    }

    /// Whether this usage kind has the contextual composite-by-default ownership rule.
    ///
    /// This is an exhaustive typed classifier rather than a string convention. The graph still
    /// requires parser-backed feature properties before applying the default, so a kind whose
    /// builder has not yet published ownership-relevant facts remains explicitly unsupported.
    pub fn is_composite_by_default_usage(&self) -> bool {
        matches!(
            self,
            ElementKind::Part
                | ElementKind::Interface
                | ElementKind::Port
                | ElementKind::Item
                | ElementKind::Attribute
                | ElementKind::Action
                | ElementKind::State
                | ElementKind::Requirement
                | ElementKind::Case
                | ElementKind::UseCase
                | ElementKind::Concern
                | ElementKind::Analysis
                | ElementKind::Verification
                | ElementKind::View
                | ElementKind::Viewpoint
                | ElementKind::Rendering
                | ElementKind::Flow
                | ElementKind::Allocation
                | ElementKind::Connection
                | ElementKind::Occurrence
                | ElementKind::Calc
                | ElementKind::Constraint
                | ElementKind::Enumeration
                | ElementKind::Perform
                | ElementKind::ViewRendering
                | ElementKind::Objective
                | ElementKind::AssertConstraint
                | ElementKind::RequireConstraint
        )
    }

    /// Whether this element can own features in a SysML Type context.
    ///
    /// Definitions and usages are both Types for this purpose. Namespace/package ownership is
    /// deliberately excluded; callers also inspect a child's typed end/direction/ref facts.
    pub fn is_type_context(&self) -> bool {
        self.is_definition()
            || self.is_composite_by_default_usage()
            || matches!(
                self,
                ElementKind::Ref
                    | ElementKind::ViewRendering
                    | ElementKind::Actor
                    | ElementKind::Stakeholder
                    | ElementKind::Individual
                    | ElementKind::MetadataUsage
                    | ElementKind::MetadataKeyword
                    | ElementKind::Perform
                    | ElementKind::Subject
                    | ElementKind::VerifiedRequirement
                    | ElementKind::IncludeUseCase
                    | ElementKind::Interface
                    | ElementKind::Enumeration
            )
    }
}

impl std::fmt::Display for ElementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<str> for ElementKind {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for ElementKind {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<ElementKind> for str {
    fn eq(&self, other: &ElementKind) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<ElementKind> for &str {
    fn eq(&self, other: &ElementKind) -> bool {
        *self == other.as_str()
    }
}

impl From<String> for ElementKind {
    fn from(value: String) -> Self {
        ElementKind::parse(&value)
    }
}

impl From<&str> for ElementKind {
    fn from(value: &str) -> Self {
        ElementKind::parse(value)
    }
}

impl From<ElementKind> for String {
    fn from(value: ElementKind) -> Self {
        value.as_str().to_string()
    }
}

/// Optional metadata when a `Connection` edge came from a resolved `connect` statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectStatementDetail {
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    pub declaring_uri: Url,
    pub range: TextRange,
    pub source_expression: String,
    pub target_expression: String,
    pub container_prefix: Option<String>,
    /// True when the edge was declared by an `interface` usage rather than a
    /// plain `connect` statement.
    #[serde(default)]
    pub is_interface_usage: bool,
    /// The declared `InterfaceDefinition` for a typed interface usage.
    ///
    /// When present, the two connected ports specialize the corresponding
    /// interface ends. They are not required to be conjugations of one shared
    /// port definition.
    #[serde(default)]
    pub interface_type: Option<String>,
}

/// Optional metadata when a `Flow`/`SuccessionFlow` edge came from a resolved `flow` usage.
/// Text-layer only, mirroring [`ConnectStatementDetail`]: resolution to feature/type IDs happens
/// downstream in the graph builder, not in this struct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowStatementDetail {
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    pub declaring_uri: Url,
    pub range: TextRange,
    pub payload_expression: Option<String>,
    pub source_expression: Option<String>,
    pub target_expression: Option<String>,
    /// The payload's resolved type (the `of Payload` reference), when it names a real type in
    /// the workspace. Holds the target's *qualified name* at this graph-builder layer -- the
    /// `workspace` crate's projection step translates it into a semantic ID, the same
    /// two-step handoff `source_id`/`target_id` already use for edge endpoints.
    /// `payload_expression` remains the raw text for the unresolved case.
    #[serde(default)]
    pub payload_type_id: Option<String>,
}

/// Edge weight in the semantic graph: relationship kind plus optional connect/flow metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticEdge {
    pub kind: RelationshipKind,
    /// Whether this relationship was authored or was published by a named semantic rule.
    /// The rule is part of the fact's identity: consumers must not infer it from an absent
    /// source range or from relationship kind.
    #[serde(default)]
    pub provenance: RelationshipProvenance,
    /// Which construction pass created (and therefore owns and may remove) this edge. This is
    /// a distinct axis from `provenance`: `provenance` is a semantic fact about the model
    /// (authored vs. rule-implied), while `owner` is a mechanical fact about the graph builder
    /// (which pass is responsible for removing/replacing this edge on re-resolution). Never
    /// infer one from the other.
    #[serde(default)]
    pub owner: ConstructionOwner,
    /// Set when a structural expression relationship (`connect` or `bind`) was resolved, retaining
    /// its declaring context and endpoint expressions for instance projection.
    pub connect: Option<ConnectStatementDetail>,
    /// Set when this `Flow`/`SuccessionFlow` came from a resolved `flow` usage.
    #[serde(default)]
    pub flow: Option<FlowStatementDetail>,
}

impl SemanticEdge {
    pub fn plain(kind: RelationshipKind, owner: ConstructionOwner) -> Self {
        Self {
            kind,
            provenance: RelationshipProvenance::Authored,
            owner,
            connect: None,
            flow: None,
        }
    }

    /// Constructs a relationship produced by a graph-owned semantic rule. Always owned by
    /// [`ConstructionOwner::UniversalImpliedRuleConstruction`] -- the rule that publishes the
    /// fact is also the only pass allowed to remove it.
    pub fn implied(kind: RelationshipKind, rule: ImpliedRelationshipRule) -> Self {
        Self {
            kind,
            provenance: RelationshipProvenance::Implied(rule),
            owner: ConstructionOwner::UniversalImpliedRuleConstruction,
            connect: None,
            flow: None,
        }
    }

    pub fn derived(kind: RelationshipKind, rule: DerivedRelationshipRule) -> Self {
        Self {
            kind,
            provenance: RelationshipProvenance::Derived(rule),
            owner: ConstructionOwner::DocumentConstruction,
            connect: None,
            flow: None,
        }
    }

    pub fn connection_with_connect(
        connect: ConnectStatementDetail,
        owner: ConstructionOwner,
    ) -> Self {
        Self {
            kind: RelationshipKind::Connection,
            provenance: RelationshipProvenance::Authored,
            owner,
            connect: Some(connect),
            flow: None,
        }
    }

    pub fn interconnection_with_detail(
        kind: RelationshipKind,
        connect: ConnectStatementDetail,
        owner: ConstructionOwner,
    ) -> Self {
        Self {
            kind,
            provenance: RelationshipProvenance::Authored,
            owner,
            connect: Some(connect),
            flow: None,
        }
    }

    pub fn flow_with_detail(
        kind: RelationshipKind,
        flow: FlowStatementDetail,
        owner: ConstructionOwner,
    ) -> Self {
        Self {
            kind,
            provenance: RelationshipProvenance::Authored,
            owner,
            connect: None,
            flow: Some(flow),
        }
    }
}

/// Typed construction owner for a semantic edge: which pass created it, and therefore which
/// pass is responsible for removing/replacing it on re-resolution. This is a distinct axis
/// from [`RelationshipProvenance`] (semantic authored-vs-implied) -- an edge's owner is a fact
/// about the graph builder, never inferred from provenance or relationship kind.
///
/// This enum, plus source identity, is the sole authority `SemanticGraphData::rebuild_cross_document_edge_ownership_index`
/// uses to rebuild `cross_document_edges_by_source_uri`. Whole, parallel, merge-from-base,
/// incremental, and decoded builds must all tag edges with the *same* owner for equivalent
/// input, or the rebuilt index (and therefore later removal/refresh) will diverge by
/// construction path -- see `ROUNDTRIP_SEMGRAPH_PREREQS.md` B1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ConstructionOwner {
    /// Built directly from one document's own parsed content: the per-document graph builder
    /// (`graph_builder/*`) and the initial per-node materialization of a single document's
    /// declared facts. Never a Typing/Specializes/Subject edge whose endpoints can cross
    /// documents once the workspace is fully linked -- those are always retagged
    /// [`Self::WorkspaceCrossDocumentLinking`] regardless of which pass produced them (see
    /// `add_semantic_edge_once`).
    #[default]
    DocumentConstruction,
    /// Typing, Specializes, and Subject edges published by workspace relationship linking --
    /// whichever of the two equivalent engines ran: the whole-graph fixed-point pass
    /// (`link_workspace_relationships`/`link_case_subject_relationships`) or the URI-scoped
    /// resolver (`add_cross_document_edges_for_uri`/`resolve_cross_document_edges_for_uri`),
    /// including the parallel-build variant that calls the scoped resolver directly. The owning
    /// URI recorded in `cross_document_edges_by_source_uri` is always the edge's *source* node's
    /// URI, matching the scoped resolver's per-URI removal contract.
    WorkspaceCrossDocumentLinking,
    /// `Derivation` edges wired by `try_wire_derivation_connection` linking a `#derive`/
    /// `#original` connection pair.
    DerivationLinking,
    /// Edges resolved from a previously-deferred `PendingRelationship` or
    /// `PendingExpressionRelationship` once their endpoints became resolvable.
    PendingResolution,
    /// Edges published by a universal standard-library implied rule
    /// (`refresh_universal_standard_library_relationships`). Always paired with
    /// `RelationshipProvenance::Implied`; that pass self-cleans its own prior edges by
    /// provenance and does not use `cross_document_edges_by_source_uri`.
    UniversalImpliedRuleConstruction,
}

/// Provenance carried by every resolved relationship edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipProvenance {
    #[default]
    Authored,
    Implied(ImpliedRelationshipRule),
    Derived(DerivedRelationshipRule),
}

/// Exhaustive identities of rules that may publish implied relationship facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImpliedRelationshipRule {
    UniversalStandardLibraryRelationship,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DerivedRelationshipRule {
    CaseSubjectFromTypedSubject,
}

/// The complete relationship published by the universal standard-library rule for an element
/// kind. Keeping kind and target together prevents consumers from reimplementing the policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversalStandardLibraryRelationship {
    pub kind: RelationshipKind,
    pub target: StandardLibraryElement,
}

impl UniversalStandardLibraryRelationship {
    const fn specializes(target: StandardLibraryElement) -> Self {
        Self {
            kind: RelationshipKind::Specializes,
            target,
        }
    }

    const fn subsets(target: StandardLibraryElement) -> Self {
        Self {
            kind: RelationshipKind::Subsetting,
            target,
        }
    }
}

/// Closed identities of the standard-library elements used by universal SysML relationships.
/// Their canonical qualified identity belongs to the semantic model, never to a host projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StandardLibraryElement {
    BaseDataValue,
    BaseDataValues,
    OccurrencesLife,
    OccurrencesOccurrences,
    ItemsItem,
    ItemsItems,
    PartsPart,
    PartsParts,
    PortsPort,
    PortsPorts,
    ConnectionsConnection,
    ConnectionsConnections,
    InterfacesInterface,
    InterfacesInterfaces,
    AllocationsAllocation,
    AllocationsAllocations,
    FlowsMessageAction,
    FlowsMessages,
    ActionsAction,
    ActionsActions,
    ActionsAssignmentActions,
    ActionsForLoopActions,
    ActionsTerminateActions,
    ActionsWhileLoopActions,
    ActionsIfThenActions,
    ActionsTransitionActions,
    ActionsAcceptActions,
    StatesStateAction,
    StatesStateActions,
    CalculationsCalculation,
    CalculationsCalculations,
    ConstraintsConstraintCheck,
    ConstraintsConstraintChecks,
    RequirementsRequirementCheck,
    RequirementsRequirementChecks,
    RequirementsConcernCheck,
    RequirementsConcernChecks,
    CasesCase,
    CasesCases,
    AnalysisCasesAnalysisCase,
    AnalysisCasesAnalysisCases,
    VerificationCasesVerificationCase,
    VerificationCasesVerificationCases,
    UseCasesUseCase,
    UseCasesUseCases,
    ViewsRendering,
    ViewsRenderings,
    ViewsView,
    ViewsViews,
    ViewsViewpoint,
    ViewsViewpoints,
    MetadataMetadataItem,
    MetadataMetadataItems,
}

impl StandardLibraryElement {
    pub fn qualified_name(self) -> &'static str {
        match self {
            Self::BaseDataValue => "Base::DataValue",
            Self::BaseDataValues => "Base::dataValues",
            Self::OccurrencesLife => "Occurrences::Life",
            Self::OccurrencesOccurrences => "Occurrences::occurrences",
            Self::ItemsItem => "Items::Item",
            Self::ItemsItems => "Items::items",
            Self::PartsPart => "Parts::Part",
            Self::PartsParts => "Parts::parts",
            Self::PortsPort => "Ports::Port",
            Self::PortsPorts => "Ports::ports",
            Self::ConnectionsConnection => "Connections::Connection",
            Self::ConnectionsConnections => "Connections::connections",
            Self::InterfacesInterface => "Interfaces::Interface",
            Self::InterfacesInterfaces => "Interfaces::interfaces",
            Self::AllocationsAllocation => "Allocations::Allocation",
            Self::AllocationsAllocations => "Allocations::allocations",
            Self::FlowsMessageAction => "Flows::MessageAction",
            Self::FlowsMessages => "Flows::messages",
            Self::ActionsAction => "Actions::Action",
            Self::ActionsActions => "Actions::actions",
            Self::ActionsAssignmentActions => "Actions::assignmentActions",
            Self::ActionsForLoopActions => "Actions::forLoopActions",
            Self::ActionsTerminateActions => "Actions::terminateActions",
            Self::ActionsWhileLoopActions => "Actions::whileLoopActions",
            Self::ActionsIfThenActions => "Actions::ifThenActions",
            Self::ActionsTransitionActions => "Actions::transitionActions",
            Self::ActionsAcceptActions => "Actions::acceptActions",
            Self::StatesStateAction => "States::StateAction",
            Self::StatesStateActions => "States::stateActions",
            Self::CalculationsCalculation => "Calculations::Calculation",
            Self::CalculationsCalculations => "Calculations::calculations",
            Self::ConstraintsConstraintCheck => "Constraints::ConstraintCheck",
            Self::ConstraintsConstraintChecks => "Constraints::constraintChecks",
            Self::RequirementsRequirementCheck => "Requirements::RequirementCheck",
            Self::RequirementsRequirementChecks => "Requirements::requirementChecks",
            Self::RequirementsConcernCheck => "Requirements::ConcernCheck",
            Self::RequirementsConcernChecks => "Requirements::concernChecks",
            Self::CasesCase => "Cases::Case",
            Self::CasesCases => "Cases::cases",
            Self::AnalysisCasesAnalysisCase => "AnalysisCases::AnalysisCase",
            Self::AnalysisCasesAnalysisCases => "AnalysisCases::analysisCases",
            Self::VerificationCasesVerificationCase => "VerificationCases::VerificationCase",
            Self::VerificationCasesVerificationCases => "VerificationCases::verificationCases",
            Self::UseCasesUseCase => "UseCases::UseCase",
            Self::UseCasesUseCases => "UseCases::useCases",
            Self::ViewsRendering => "Views::Rendering",
            Self::ViewsRenderings => "Views::renderings",
            Self::ViewsView => "Views::View",
            Self::ViewsViews => "Views::views",
            Self::ViewsViewpoint => "Views::Viewpoint",
            Self::ViewsViewpoints => "Views::viewpoints",
            Self::MetadataMetadataItem => "Metadata::MetadataItem",
            Self::MetadataMetadataItems => "Metadata::metadataItems",
        }
    }
}

/// Published outcome of the universal standard-library relationship rule for one source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DerivedRelationshipResolution {
    /// The current graph state has not crossed the implied-relationship publication barrier.
    NotRun,
    NotApplicable,
    Resolved {
        target: NodeId,
    },
    MissingPrerequisite {
        target: StandardLibraryElement,
    },
    Ambiguous {
        candidates: Vec<NodeId>,
    },
    SelfTargetSuppressed {
        target: NodeId,
    },
}

impl RelationshipKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationshipKind::Typing => "typing",
            RelationshipKind::Specializes => "specializes",
            RelationshipKind::Subsetting => "subsetting",
            RelationshipKind::Redefinition => "redefinition",
            RelationshipKind::ReferenceSubsetting => "referenceSubsetting",
            RelationshipKind::CrossSubsetting => "crossSubsetting",
            RelationshipKind::Connection => "connection",
            RelationshipKind::Bind => "bind",
            RelationshipKind::Flow => "flow",
            RelationshipKind::SuccessionFlow => "successionFlow",
            RelationshipKind::Perform => "perform",
            RelationshipKind::Allocate => "allocate",
            RelationshipKind::Dependency => "dependency",
            RelationshipKind::Satisfy => "satisfy",
            RelationshipKind::Subject => "subject",
            RelationshipKind::Reference => "reference",
            RelationshipKind::Derivation => "derivation",
            RelationshipKind::Transition => "transition",
            RelationshipKind::InitialState => "initialState",
            RelationshipKind::Annotation => "annotation",
            RelationshipKind::PortConjugation => "portConjugation",
        }
    }

    /// Parses persisted relationship type strings (host projection storage).
    pub fn from_persisted_type(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "typing" => Some(RelationshipKind::Typing),
            "specializes" => Some(RelationshipKind::Specializes),
            "subsetting" => Some(RelationshipKind::Subsetting),
            "redefinition" => Some(RelationshipKind::Redefinition),
            "referencesubsetting" => Some(RelationshipKind::ReferenceSubsetting),
            "crosssubsetting" => Some(RelationshipKind::CrossSubsetting),
            "connection" => Some(RelationshipKind::Connection),
            "bind" => Some(RelationshipKind::Bind),
            "flow" => Some(RelationshipKind::Flow),
            "successionflow" => Some(RelationshipKind::SuccessionFlow),
            "perform" => Some(RelationshipKind::Perform),
            "allocate" => Some(RelationshipKind::Allocate),
            "dependency" => Some(RelationshipKind::Dependency),
            "satisfy" => Some(RelationshipKind::Satisfy),
            "subject" | "verification" => Some(RelationshipKind::Subject),
            "reference" => Some(RelationshipKind::Reference),
            "derivation" => Some(RelationshipKind::Derivation),
            "transition" | "initialstate" => Some(RelationshipKind::Transition),
            "annotation" => Some(RelationshipKind::Annotation),
            "portconjugation" => Some(RelationshipKind::PortConjugation),
            _ => None,
        }
    }
}

/// A node in the semantic graph representing a model element.
/// Typed source facts retained from the parser AST. These facts deliberately
/// remain separate from the legacy display-oriented `attributes` map.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeclaredSemanticFacts {
    /// Parser-authored membership and import facts. These own membership visibility and import
    /// shape; consumers must not recover either from presentation attributes.
    #[serde(default)]
    pub membership: Option<DeclaredMembershipFacts>,
    #[serde(default)]
    pub multiplicity: Option<DeclaredMultiplicity>,
    #[serde(default)]
    pub feature_value: Option<DeclaredFeatureValue>,
    /// Explicit declaration modifiers retained from the parser AST.
    /// Absent for packages and other non-feature/definition nodes.
    #[serde(default)]
    pub feature_properties: Option<DeclaredFeatureProperties>,
    /// Authored relationship targets retained from the structured parser AST.
    ///
    /// These are the sole semantic input for typing, specialization, and the
    /// subsetting family.  The display-oriented `attributes` map may project
    /// them for hosts, but relationship linking must never reconstruct them
    /// from that projection.
    #[serde(default)]
    pub relationships: DeclaredRelationshipFacts,
    /// Authored endpoint relationships whose target identities are settled by the canonical
    /// resolver after all source documents are merged.
    #[serde(default)]
    pub expression_relationships: Vec<DeclaredExpressionRelationship>,
    /// Set when this node's own declared substance *is* an expression (e.g. a
    /// `TransitionGuard` child), as opposed to `feature_value`, which represents "this
    /// feature's value is X". Projected as `HostElementFacts::content_expression_id`.
    #[serde(default)]
    pub own_expression: Option<DeclaredExpression>,
    /// Set on a `Transition` node: the authored source/target endpoint references plus the
    /// `initial`/`done` state-machine flags derived from them. Endpoint resolution to concrete
    /// node IDs happens downstream (e.g. `sysml_diagnostics::behavior_conformance`), the same
    /// two-step handoff `ConnectStatementDetail`'s `source_expression`/`target_expression` use.
    #[serde(default)]
    pub transition_endpoints: Option<TransitionEndpointFacts>,
    /// Authored short name (e.g. `part def <'CB'> ControlBoard;`), present only when a short
    /// name is declared alongside a regular name. When short_name is the *only* name,
    /// [`ast_util::identification_name`] already uses it as [`SemanticNode::name`], so there is
    /// nothing extra to capture here.
    #[serde(default)]
    pub short_name: Option<String>,
    /// Unit-catalog metadata (prefix, conversion, and value-expression facts) for
    /// `attribute def`/`attribute` nodes that participate in the unit catalog.
    #[serde(default)]
    pub unit: Option<DeclaredUnitFacts>,
    /// Analysis/verification-case level facts: rendered case expression, aggregated
    /// require/assert constraints, and objective-to-result binding. See
    /// [`DeclaredAnalysisCaseFacts`]. Replaces `attributes["analysisExpression"]`,
    /// `["analysisConstraints"]`, `["objectiveBoundTo"]` (`UNIFY_CACHE_PROGRESS.md` chunk E).
    #[serde(default)]
    pub analysis_case: Option<DeclaredAnalysisCaseFacts>,
    /// The recognized KerML metaclass role for a raw `KermlSemanticDecl`/`KermlFeatureDecl`
    /// library declaration node, when its opaque BNF-tagged text names a known metaclass.
    /// Drives genuine semantic classification (`is_kerml_metadata_supertype`, the
    /// `SemanticMetadata` parent check) rather than presentation (was
    /// `attributes["metaclassRole"]`, `UNIFY_CACHE_PROGRESS.md` chunk F).
    #[serde(default)]
    pub metaclass_role: Option<KermlMetaclassRole>,
    /// An interface/connection end's declared type name (`end name : Type;`), kept separate from
    /// `relationships.typing` so that generic unresolved-typing diagnostics do not walk it: ends
    /// resolve leniently as either a type or a feature reference, and already have their own
    /// `interface_end_invalid`-family diagnostics (was `attributes["endType"]`,
    /// `UNIFY_CACHE_PROGRESS.md` chunk F).
    #[serde(default)]
    pub interface_end_type: Option<String>,
    /// Declared keyword spelling used only for semantic-classification decisions: user-defined
    /// modeled-keyword detection on a `feature decl`/`classifier decl` node, or the matched
    /// keyword on a `MetadataKeyword` usage (see
    /// `sysml_diagnostics::checks::view_metadata_conformance`). Deliberately distinct from
    /// `SourceTextFacts::keyword`, which covers only the unrelated hover/doc-comment spelling of
    /// an opaque member / action body decl (was `attributes["keyword"]` for this use,
    /// `UNIFY_CACHE_PROGRESS.md` chunk F).
    #[serde(default)]
    pub modeled_keyword: Option<String>,
}

impl DeclaredSemanticFacts {
    /// The declared target name for an interface/connection end: either its declared
    /// `interface_end_type` or, for a `::>`/`references` end, its authored reference-subsetting
    /// target. The two are mutually exclusive per the parser's `uses_derived_syntax`
    /// discriminator, so the first present one is authoritative. Used by connection-wiring and
    /// derivation-connection fallback resolution when no `Typing` edge has been published yet.
    pub fn declared_end_reference(&self) -> Option<&str> {
        self.interface_end_type.as_deref().or_else(|| {
            self.relationships
                .reference_subsetting
                .first()
                .map(|target| target.reference.as_str())
        })
    }
}

/// A recognized KerML metaclass role carried by a raw library declaration node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KermlMetaclassRole {
    /// The declaration names (or contains) `SemanticMetadata`.
    SemanticMetadata,
}

/// Declared analysis/verification-case facts that previously round-tripped through JSON
/// attributes. `analysisKind`/`analysisParams`/`analysisReturn`/`parameters` are deliberately
/// **not** modeled here: an exhaustive repository grep found no reader for any of the four
/// (`calc_constraint_def.rs` only ever wrote them), so their producer writes were deleted
/// outright per the B9 "no reader" rule instead of being migrated to a typed fact.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredAnalysisCaseFacts {
    /// Rendered analysis/verification-case expression text (was `attributes["analysisExpression"]`).
    /// Remains a rendered string, not a `DeclaredExpression` tree, because every consumer (case
    /// inheritance propagation, `requirement_case_conformance`, hover) matches on the text itself
    /// rather than walking a parsed tree; producers only ever had a debug-rendered string here.
    #[serde(default)]
    pub expression: Option<String>,
    /// Aggregated `require constraint` / `assert constraint` facts owned by this case/requirement
    /// (was `attributes["analysisConstraints"]`).
    #[serde(default)]
    pub constraints: Vec<DeclaredAnalysisConstraint>,
    /// Qualified name of the analysis result this objective is bound to (was
    /// `attributes["objectiveBoundTo"]`). Modeled as the qualified-name target rather than a
    /// presence-only bool: `requirement_case_conformance.rs` reads the string content (and
    /// requires it to be non-empty), not merely whether the key exists; the `engine_impl.rs`
    /// gate is a strict subset (`is_some()`) of that same fact.
    #[serde(default)]
    pub objective_bound_to: Option<String>,
}

/// One aggregated constraint fact contributing to a case/requirement's
/// [`DeclaredAnalysisCaseFacts::constraints`]. Mirrors the two producers of
/// `attributes["analysisConstraints"]` entries: a requirement's authored `require constraint`
/// body, and an owned `assert constraint` child projected onto its parent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "kind")]
#[allow(clippy::enum_variant_names)]
pub enum DeclaredAnalysisConstraint {
    RequireConstraint {
        params: Vec<DeclaredAnalysisConstraintParam>,
        expression: String,
        #[serde(default)]
        doc: String,
        #[serde(default)]
        metadata: Vec<String>,
    },
    AssertConstraint {
        expression: String,
    },
}

/// One `in`/`out`/`inout` parameter of a `require constraint`'s declared parameter list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredAnalysisConstraintParam {
    pub direction: String,
    pub name: String,
    #[serde(default)]
    pub param_type: Option<String>,
}

/// Typed source/target endpoint facts for a `transition` statement's `Transition` node.
/// Mirrors [`ConnectStatementDetail`] for state-machine transitions instead of connect usages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionEndpointFacts {
    /// Authored source endpoint reference, qualified relative to the enclosing state definition.
    pub source_expression: String,
    /// Authored target endpoint reference, qualified relative to the enclosing state definition.
    pub target_expression: String,
    /// True when this transition was declared with the `initial` keyword.
    pub is_initial: bool,
    /// True when the target endpoint resolves to the implicit `done` pseudostate.
    pub target_is_done: bool,
}

/// Parser-authored unit-catalog metadata for an `attribute def`/`attribute` node: prefix
/// declarations (`UnitPrefix`), unit conversions (`ConversionByConvention`, `ConversionByPrefix`,
/// `IntervalScale`), and the rendered debug form of a declared value expression. Owned entirely
/// by the semantic system (see `graph_builder::unit_metadata`); no `serde_json::Value` involved.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DeclaredUnitFacts {
    #[serde(default)]
    pub prefix: Option<DeclaredUnitPrefix>,
    #[serde(default)]
    pub conversion: Option<DeclaredUnitConversion>,
    #[serde(default)]
    pub value_expr: Option<String>,
}

/// A declared `UnitPrefix` catalog entry (e.g. `attribute kilo: UnitPrefix { :>> symbol = "k";
/// :>> conversionFactor = 1E3; }`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeclaredUnitPrefix {
    pub symbol: Option<String>,
    pub conversion_factor: f64,
}

/// A declared unit conversion, in one of the catalog's supported shapes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeclaredUnitConversion {
    pub kind: String,
    pub reference_unit: Option<String>,
    pub conversion_factor: Option<f64>,
    pub prefix: Option<String>,
    pub interval_unit: Option<String>,
    pub zero_offset_kelvin: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredExpressionRelationship {
    pub kind: RelationshipKind,
    pub source_expression: String,
    pub target_expression: String,
    #[serde(default)]
    pub scope_owner: Option<NodeId>,
    pub source_range: TextRange,
    #[serde(default)]
    pub target_range: Option<TextRange>,
    #[serde(default)]
    pub is_interface_usage: bool,
    #[serde(default)]
    pub interface_type: Option<String>,
}

/// Visibility written on a membership declaration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum VisibilityKind {
    Public,
    Private,
    Protected,
}

impl VisibilityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Protected => "protected",
        }
    }
}

/// Parser-authored membership facts. An absent `visibility` is deliberately distinct from the
/// effective private default published by [`crate::SemanticGraph::effective_membership_visibility_for`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredMembershipFacts {
    #[serde(default)]
    pub kind: DeclaredMembershipKind,
    #[serde(default)]
    pub visibility: Option<VisibilityKind>,
    /// The parser span for the declaration carrying this membership.
    #[serde(default)]
    pub range: Option<TextRange>,
    /// Present only for import/expose memberships.
    #[serde(default)]
    pub import: Option<DeclaredImportFacts>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DeclaredMembershipKind {
    Owning,
    Feature,
    Import,
    Alias,
    Variant,
    Actor,
    /// A graph-owned synthesized membership whose parser grammar has no Membership wrapper.
    #[default]
    Synthesized,
}

/// The parser-authored form of an import membership.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredImportFacts {
    pub target: DeclaredImportTarget,
    /// Grammar production that authored this import-shaped membership. `Expose` has its own
    /// scope/expansion contract and must not inherit `Import` defaults.
    pub origin: ImportOrigin,
    pub shape: ImportShape,
    pub recursive: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ImportOrigin {
    Import,
    Expose,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredImportTarget {
    /// Authored target reference, before any lookup or normalization.
    pub reference: String,
    /// Parser-owned presence state. Missing is distinct from an unresolved non-empty target.
    pub presence: ImportTargetPresence,
    /// Exact target span when the parser exposes it; otherwise the enclosing import span.
    #[serde(default)]
    pub range: Option<TextRange>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImportTargetPresence {
    Present,
    Missing,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ImportShape {
    Membership,
    Namespace,
    /// Parser-backed filter-package form. It is retained distinctly until filter semantics are
    /// implemented; consumers must not treat it as an ordinary namespace import.
    FilteredNamespace,
}

/// Canonical effective membership visibility with explicit authored/default provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveMembershipVisibility {
    pub value: VisibilityKind,
    pub provenance: MembershipVisibilityProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipVisibilityProvenance {
    Authored,
    Implied,
}

/// Explicit relationship targets authored on a declaration.
///
/// Each list preserves source order and provenance. An empty list means that
/// the relationship was not authored; it is intentionally distinct from an
/// unresolved target, which remains represented by this fact while no edge is
/// published for it.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredRelationshipFacts {
    #[serde(default)]
    pub typing: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub specializes: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub subsetting: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub redefinition: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub reference_subsetting: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub cross_subsetting: Vec<DeclaredRelationshipTarget>,
    /// Authored verified-requirement targets whose resolved result is published
    /// as a case-level `Subject` edge. Case subject declarations themselves
    /// remain owned by their `typing` facts on `ElementKind::Subject` children.
    #[serde(default)]
    pub subject: Vec<DeclaredRelationshipTarget>,
    /// Authored but unresolved-edge reference targets: a `Stakeholder`/`Purpose` declaration's
    /// bare target name when no typed clause is authored alongside it (was
    /// `attributes["refTarget"]`, `UNIFY_CACHE_PROGRESS.md` chunk F). These never publish a
    /// graph edge; consumers resolve the name against the workspace directly. Distinct from
    /// `reference`, which holds authored `RelationshipKind::Reference` targets that publish
    /// edges on linking.
    #[serde(default)]
    pub reference_target: Vec<DeclaredRelationshipTarget>,
    /// Authored endpoint relationships whose parser adapter exposes only qualified target text.
    /// Their ranges remain explicit `None` until a richer AST adapter supplies them.
    #[serde(default)]
    pub connection: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub bind: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub satisfy: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub allocate: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub flow: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub succession_flow: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub perform: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub transition: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub initial_state: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub reference: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub dependency: Vec<DeclaredRelationshipTarget>,
    #[serde(default)]
    pub derivation: Vec<DeclaredRelationshipTarget>,
}

/// A source-level reference carried by a structured parser relationship.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredRelationshipTarget {
    /// The authored relationship target, before resolution or effective-name
    /// derivation. This is never a display or projection attribute.
    pub reference: String,
    /// Exact parser span for the authored target. `None` is explicit only for
    /// parser fields that do not expose a target node/span.
    #[serde(default)]
    pub range: Option<TextRange>,
}

impl DeclaredRelationshipFacts {
    /// Records one parser-authored target under its relationship kind.
    /// Returns `false` for relationship kinds not owned by this fact contract.
    pub fn record_target(
        &mut self,
        kind: &RelationshipKind,
        target: DeclaredRelationshipTarget,
    ) -> bool {
        let targets = match kind {
            RelationshipKind::Typing => &mut self.typing,
            RelationshipKind::Specializes => &mut self.specializes,
            RelationshipKind::Subsetting => &mut self.subsetting,
            RelationshipKind::Redefinition => &mut self.redefinition,
            RelationshipKind::ReferenceSubsetting => &mut self.reference_subsetting,
            RelationshipKind::CrossSubsetting => &mut self.cross_subsetting,
            RelationshipKind::Subject => &mut self.subject,
            RelationshipKind::Connection => &mut self.connection,
            RelationshipKind::Bind => &mut self.bind,
            RelationshipKind::Satisfy => &mut self.satisfy,
            RelationshipKind::Allocate => &mut self.allocate,
            RelationshipKind::Flow => &mut self.flow,
            RelationshipKind::SuccessionFlow => &mut self.succession_flow,
            RelationshipKind::Perform => &mut self.perform,
            RelationshipKind::Transition => &mut self.transition,
            RelationshipKind::InitialState => &mut self.initial_state,
            RelationshipKind::Reference => &mut self.reference,
            RelationshipKind::Dependency => &mut self.dependency,
            RelationshipKind::Derivation => &mut self.derivation,
            _ => return false,
        };
        // Repeated authored clauses are distinct source facts even when they
        // ultimately publish one deduplicated semantic edge.
        targets.push(target);
        true
    }

    pub fn record_reference(&mut self, kind: &RelationshipKind, reference: &str) -> bool {
        self.record_target(
            kind,
            DeclaredRelationshipTarget {
                reference: reference.trim().to_string(),
                range: None,
            },
        )
    }

    /// Records a `Stakeholder`/`Purpose` bare reference name. Kept out of `record_target`'s
    /// kind routing: `RelationshipKind::Reference` there belongs to `reference`, whose facts
    /// publish edges on linking, while these never do.
    pub fn record_reference_target(&mut self, reference: &str) {
        self.reference_target.push(DeclaredRelationshipTarget {
            reference: reference.trim().to_string(),
            range: None,
        });
    }
}

/// Derived semantic facts that are effective for a fully linked graph state.
///
/// These facts are deliberately separate from [`DeclaredSemanticFacts`]: an implied default,
/// resolved featuring type, or expression-result binding must never overwrite the source fact
/// (or make an absent authored clause appear present).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EffectiveSemanticFacts {
    /// SysML's effective default multiplicity when the feature has no authored multiplicity.
    #[serde(default)]
    pub implied_multiplicity: Option<ImpliedMultiplicity>,
    /// The nearest resolved type that features this element, after ownership and typing closure.
    #[serde(default)]
    pub featuring_type: Option<NodeId>,
    /// The implied binding from an authored `=` FeatureValue to its addressable expression result.
    #[serde(default)]
    pub implied_feature_value_binding: Option<ImpliedFeatureValueBinding>,
    /// The ownership established by SysML's contextual default for an ordinary feature usage.
    ///
    /// This is present only when no ownership modifier was authored. Explicit `ref` remains in
    /// [`DeclaredFeatureProperties`] so source provenance is never replaced by this default.
    #[serde(default)]
    pub implied_feature_ownership: Option<ImpliedFeatureOwnership>,
}

/// The explicit outcome of evaluating a declared expression.
///
/// `NotRun` is represented only by [`EvaluationPublicationState`] and
/// [`ExpressionEvaluationQuery`], never by a published per-expression result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EvaluationStatus {
    Ok,
    Unresolved,
    Ambiguous,
    Malformed,
    Incomplete,
    TypeError,
    DivByZero,
    Unsupported,
    Cycle,
}

/// Whether the current graph revision has reached the evaluation publication barrier.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EvaluationPublicationState {
    #[default]
    NotRun,
    Complete,
}

/// Exhaustive result of querying expression evaluation for one node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExpressionEvaluationQuery<'a> {
    NotRun,
    NotApplicable,
    Result(&'a ExpressionEvaluation),
}

impl EvaluationStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Unresolved => "unresolved",
            Self::Ambiguous => "ambiguous",
            Self::Malformed => "malformed",
            Self::Incomplete => "incomplete",
            Self::TypeError => "type_error",
            Self::DivByZero => "div_by_zero",
            Self::Unsupported => "unsupported",
            Self::Cycle => "cycle",
        }
    }
}

/// A value published by the evaluator.
///
/// This intentionally is not a JSON value: JSON is a transport concern, while the semantic graph
/// owns the finite set of values which its typed expression evaluator can currently produce.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
pub enum EvaluatedValue {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    String(String),
}

impl EvaluatedValue {
    pub const fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            Self::Integer(_) | Self::Real(_) | Self::String(_) => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Integer(value) => Some(*value as f64),
            Self::Real(value) if value.is_finite() => Some(*value),
            Self::Real(_) | Self::Boolean(_) | Self::String(_) => None,
        }
    }
}

/// Canonical evaluation data for one expression. `value` is present only for [`EvaluationStatus::Ok`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpressionEvaluation {
    pub status: EvaluationStatus,
    #[serde(default)]
    pub value: Option<EvaluatedValue>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

/// Canonical evaluation data for an analysis or constraint owner. A false `passed` is distinct
/// from an evaluation failure; a numeric computed value is distinct from a Boolean verdict.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisEvaluation {
    pub expression: ExpressionEvaluation,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub computed_value: Option<EvaluatedValue>,
    #[serde(default)]
    pub computed_unit: Option<String>,
}

/// Evaluation facts atomically published by the evaluation phase for one semantic node.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeEvaluationFacts {
    #[serde(default)]
    pub expression: Option<ExpressionEvaluation>,
    #[serde(default)]
    pub analysis: Option<AnalysisEvaluation>,
}

/// An effective multiplicity with no source range because it was not authored.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImpliedMultiplicity {
    pub lower: u32,
    pub upper: Option<u32>,
    pub is_ordered: bool,
    pub is_unique: Option<bool>,
}

/// SysML's contextual composite ownership default for an ordinary feature usage.
///
/// It has no source range because neither `composite` nor `non-reference` was authored. The
/// semantic graph computes it only after containment is complete; consumers must obtain the
/// combined result through [`crate::SemanticGraph::effective_feature_ownership_for`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImpliedFeatureOwnership {
    pub is_composite: bool,
    pub is_reference: bool,
}

/// The canonical effective ownership of a feature after applying its authored modifier or the
/// contextual SysML default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveFeatureOwnership {
    pub is_composite: bool,
    pub is_reference: bool,
    pub provenance: FeatureOwnershipProvenance,
}

/// Records whether effective feature ownership was written in the source or implied by context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureOwnershipProvenance {
    Authored,
    Implied,
}

/// Stable, graph-neutral identity of an expression result owned by a semantic element.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExpressionResultId {
    pub owner_id: NodeId,
    pub role: ExpressionResultRole,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ExpressionResultRole {
    FeatureValue,
}

/// An evaluated/effective binding whose target is an expression result rather than a graph node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImpliedFeatureValueBinding {
    pub expression_result: ExpressionResultId,
}

/// Explicit feature/definition modifiers from the textual declaration.
///
/// Composite/reference ownership is retained only when a parser-backed modifier
/// such as `ref` was authored. Contextual composite ownership is published
/// separately in [`EffectiveSemanticFacts`]. Conjugation is set when a type
/// reference is prefixed with `~`. Portion/time-varying semantics remain
/// omitted until the parser exposes them as typed fields.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredFeatureProperties {
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default)]
    pub is_abstract: bool,
    #[serde(default)]
    pub is_variation: bool,
    #[serde(default)]
    pub is_individual: bool,
    #[serde(default)]
    pub is_derived: bool,
    #[serde(default)]
    pub is_constant: bool,
    #[serde(default)]
    pub is_end: bool,
    #[serde(default)]
    pub is_composite: Option<bool>,
    #[serde(default)]
    pub is_reference: Option<bool>,
    #[serde(default)]
    pub is_conjugated: bool,
    #[serde(default)]
    pub is_ordered: Option<bool>,
    #[serde(default)]
    pub is_unique: Option<bool>,
    #[serde(default)]
    pub is_portion: bool,
    #[serde(default)]
    pub portion_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclaredMultiplicity {
    pub lower: Option<DeclaredExpression>,
    pub upper: Option<DeclaredExpression>,
    pub range: TextRange,
    pub is_implied: bool,
    pub is_ordered: bool,
    pub is_unique: Option<bool>,
}

/// The directly-known value of a parser-backed multiplicity bound.
///
/// Expressions that need name resolution or evaluation deliberately remain
/// [`Unevaluated`](Self::Unevaluated).  Consumers must not substitute display
/// text or treat that state as a resolved bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredMultiplicityBound {
    /// The `*` spelling used for an unbounded side of a multiplicity.
    Unbounded,
    /// A directly declared integer literal, including a signed literal.
    Integer(i64),
    /// A literal that cannot be a multiplicity integer (for example `1.5`).
    NonIntegerLiteral,
    /// A parser-backed expression whose value is not directly known here.
    Unevaluated,
}

/// The directly-known lower and upper sides of a declared multiplicity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredMultiplicityBounds {
    pub lower: DeclaredMultiplicityBound,
    pub upper: DeclaredMultiplicityBound,
}

impl DeclaredMultiplicity {
    /// Returns the literal/unbounded portion of this parser-backed fact without
    /// consulting display attributes or re-parsing source text.
    pub fn direct_bounds(&self) -> DeclaredMultiplicityBounds {
        DeclaredMultiplicityBounds {
            lower: direct_multiplicity_bound(self.lower.as_ref()),
            upper: direct_multiplicity_bound(self.upper.as_ref()),
        }
    }
}

fn direct_multiplicity_bound(expression: Option<&DeclaredExpression>) -> DeclaredMultiplicityBound {
    let Some(expression) = expression else {
        return DeclaredMultiplicityBound::Unbounded;
    };
    if expression.kind == DeclaredExpressionKind::IntegerLiteral {
        return expression
            .literal
            .as_ref()
            .and_then(DeclaredLiteral::as_integer)
            .map(DeclaredMultiplicityBound::Integer)
            .unwrap_or(DeclaredMultiplicityBound::NonIntegerLiteral);
    }
    if expression.kind == DeclaredExpressionKind::Unary && expression.children.len() == 1 {
        let child = direct_multiplicity_bound(expression.children.first());
        return match (expression.operator.as_ref(), child) {
            (
                Some(DeclaredExpressionOperator::Unary(DeclaredUnaryOperator::Plus)),
                DeclaredMultiplicityBound::Integer(value),
            ) => DeclaredMultiplicityBound::Integer(value),
            (
                Some(DeclaredExpressionOperator::Unary(DeclaredUnaryOperator::Minus)),
                DeclaredMultiplicityBound::Integer(value),
            ) => value
                .checked_neg()
                .map(DeclaredMultiplicityBound::Integer)
                .unwrap_or(DeclaredMultiplicityBound::NonIntegerLiteral),
            (_, DeclaredMultiplicityBound::NonIntegerLiteral) => {
                DeclaredMultiplicityBound::NonIntegerLiteral
            }
            _ => DeclaredMultiplicityBound::Unevaluated,
        };
    }
    if expression.literal.is_some() {
        DeclaredMultiplicityBound::NonIntegerLiteral
    } else {
        DeclaredMultiplicityBound::Unevaluated
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclaredFeatureValue {
    pub kind: DeclaredFeatureValueKind,
    pub expression: DeclaredExpression,
    pub range: TextRange,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeclaredFeatureValueKind {
    Default,
    Initial,
    Bound,
    Override,
}

/// Lossless-enough normalized expression tree for semantic projection. The
/// original AST stays in the parser layer; this public form has no debug text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclaredExpression {
    pub kind: DeclaredExpressionKind,
    pub range: TextRange,
    #[serde(default)]
    pub literal: Option<DeclaredLiteral>,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub operator: Option<DeclaredExpressionOperator>,
    #[serde(default)]
    pub children: Vec<DeclaredExpression>,
    #[serde(default)]
    pub arguments: Vec<DeclaredExpressionArgument>,
}

/// Parser-authored literal retained without admitting an unbounded transport value into the
/// semantic graph. Real text stays lexical here; evaluation performs the finite numeric parse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
pub enum DeclaredLiteral {
    Integer(i64),
    Real(String),
    String(String),
    Boolean(bool),
}

impl DeclaredLiteral {
    pub const fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            Self::Real(_) | Self::String(_) | Self::Boolean(_) => None,
        }
    }

    pub const fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            Self::Integer(_) | Self::Real(_) | Self::String(_) => None,
        }
    }
}

/// Normalized parser operator. Its family is explicit so an operator for one expression shape
/// cannot be accepted accidentally by another.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "family", content = "operator")]
pub enum DeclaredExpressionOperator {
    Unary(DeclaredUnaryOperator),
    Binary(DeclaredBinaryOperator),
    Collection(DeclaredCollectionOperator),
    TypeCheck(DeclaredTypeCheckOperator),
}

impl DeclaredExpressionOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unary(operator) => operator.as_str(),
            Self::Binary(operator) => operator.as_str(),
            Self::Collection(operator) => operator.as_str(),
            Self::TypeCheck(operator) => operator.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeclaredUnaryOperator {
    Plus,
    Minus,
    Not,
    BitNot,
    Other(String),
}

impl DeclaredUnaryOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Not => "not",
            Self::BitNot => "~",
            Self::Other(value) => value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeclaredBinaryOperator {
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    Power,
    And,
    Or,
    Xor,
    Implies,
    Range,
    BitOr,
    BitAnd,
    Other(String),
}

impl DeclaredBinaryOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::StrictEqual => "===",
            Self::StrictNotEqual => "!==",
            Self::Less => "<",
            Self::LessOrEqual => "<=",
            Self::Greater => ">",
            Self::GreaterOrEqual => ">=",
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::Exponent => "**",
            Self::Power => "^",
            Self::And => "&&",
            Self::Or => "||",
            Self::Xor => "xor",
            Self::Implies => "implies",
            Self::Range => "..",
            Self::BitOr => "|",
            Self::BitAnd => "&",
            Self::Other(value) => value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeclaredCollectionOperator {
    Collect,
    Select,
    SelectOne,
    Size,
    IsEmpty,
    NotEmpty,
    Includes,
    Including,
    Excludes,
    Excluding,
    ExcludingAt,
    ExcludingOnce,
    Equals,
    ForAll,
    Exists,
    Sum,
    Sort,
    Filter,
    Reduce,
    Other(String),
}

impl DeclaredCollectionOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Collect => "collect",
            Self::Select => "select",
            Self::SelectOne => "selectOne",
            Self::Size => "size",
            Self::IsEmpty => "isEmpty",
            Self::NotEmpty => "notEmpty",
            Self::Includes => "includes",
            Self::Including => "including",
            Self::Excludes => "excludes",
            Self::Excluding => "excluding",
            Self::ExcludingAt => "excludingAt",
            Self::ExcludingOnce => "excludingOnce",
            Self::Equals => "equals",
            Self::ForAll => "forAll",
            Self::Exists => "exists",
            Self::Sum => "sum",
            Self::Sort => "sort",
            Self::Filter => "filter",
            Self::Reduce => "reduce",
            Self::Other(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeclaredTypeCheckOperator {
    IsType,
    HasType,
    As,
}

impl DeclaredTypeCheckOperator {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IsType => "istype",
            Self::HasType => "hastype",
            Self::As => "as",
        }
    }
}

/// Parser-normalized expression form. This is an exhaustive semantic contract rather than a
/// string tag so evaluators and other semantic consumers cannot silently drift from the builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeclaredExpressionKind {
    IntegerLiteral,
    RealLiteral,
    StringLiteral,
    BooleanLiteral,
    Null,
    FeatureReference,
    FeatureChain,
    Classification,
    MemberAccess,
    Select,
    Collect,
    MetadataAccess,
    Parenthesized,
    Bracket,
    Unary,
    Binary,
    Index,
    LiteralWithUnit,
    Tuple,
    Invocation,
    Constructor,
    CollectionOperation,
    MetaCast,
    TypeCheck,
    Conditional,
    Extent,
}

impl DeclaredExpressionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IntegerLiteral => "integerLiteral",
            Self::RealLiteral => "realLiteral",
            Self::StringLiteral => "stringLiteral",
            Self::BooleanLiteral => "booleanLiteral",
            Self::Null => "null",
            Self::FeatureReference => "featureReference",
            Self::FeatureChain => "featureChain",
            Self::Classification => "classification",
            Self::MemberAccess => "memberAccess",
            Self::Select => "select",
            Self::Collect => "collect",
            Self::MetadataAccess => "metadataAccess",
            Self::Parenthesized => "parenthesized",
            Self::Bracket => "bracket",
            Self::Unary => "unary",
            Self::Binary => "binary",
            Self::Index => "index",
            Self::LiteralWithUnit => "literalWithUnit",
            Self::Tuple => "tuple",
            Self::Invocation => "invocation",
            Self::Constructor => "constructor",
            Self::CollectionOperation => "collectionOperation",
            Self::MetaCast => "metaCast",
            Self::TypeCheck => "typeCheck",
            Self::Conditional => "conditional",
            Self::Extent => "extent",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclaredExpressionArgument {
    pub name: Option<String>,
    pub value: DeclaredExpression,
}

/// `DeclaredExpression` is self-referential via `children`/`arguments`, and as of
/// `sysml-v2-parser` 0.47.0 the parser no longer caps expression nesting depth (only structural
/// brace nesting is bounded), so a pathologically deep source expression can now produce an
/// equally deep `DeclaredExpression` tree. Rust's default derived drop glue for a recursive type
/// like this walks it via native call-stack recursion -- one frame per nesting level, unbounded --
/// which is exactly the class of bug `sysml_v2_parser::ast::Expression` had (and was fixed for) via
/// a custom iterative `Drop`. This mirrors that fix: unwind the tree through an explicit heap `Vec`
/// instead of the call stack, so dropping a `DeclaredExpression` tree can't overflow the stack no
/// matter how deep it is.
impl Drop for DeclaredExpression {
    fn drop(&mut self) {
        let mut pending: Vec<DeclaredExpression> = Vec::new();
        pending.append(&mut self.children);
        pending.extend(
            std::mem::take(&mut self.arguments)
                .into_iter()
                .map(|arg| arg.value),
        );
        while let Some(mut node) = pending.pop() {
            pending.append(&mut node.children);
            pending.extend(
                std::mem::take(&mut node.arguments)
                    .into_iter()
                    .map(|arg| arg.value),
            );
            // `node` drops here: its children/arguments were already moved out above, so this is
            // a shallow, non-recursive drop no matter how deep the original tree was.
        }
    }
}

/// Typed source-fidelity/documentation text retained from the parser AST, parallel to
/// [`DeclaredSemanticFacts`]. These fields deliberately remain separate from the legacy
/// display-oriented `attributes` map: they are the sole semantic-node-owned source of
/// documentation and representation text for presentation consumers (see
/// `crates/language_service/src/presentation_hover.rs`), and must not be reconstructed from
/// JSON attribute projections.
///
/// `keyword` here covers only the hover/doc-comment presentation use (opaque member / action
/// body decl spelling). The *semantic* classification use of a `keyword` value -- feature/
/// classifier-decl user-defined-keyword detection and metadata-keyword-usage matching in
/// `sysml_diagnostics::checks::view_metadata_conformance` -- reads a distinct producer/consumer
/// pair on the untyped `attributes` map and is out of scope here; see
/// `UNIFY_CACHE_PROGRESS.md` chunk F.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceTextFacts {
    /// Documentation comment text (`doc` member of a `TextualRepresentation`/annotation, or the
    /// combined text attached by `attach_doc_comment`). Multiple `///`/`doc` blocks on the same
    /// element are joined with a blank line.
    #[serde(default)]
    pub doc: Option<String>,
    /// Free-form body/content text retained for source-fidelity presentation (e.g. a `ref
    /// redefinition` body, or a synthesized `documentation` node's own text).
    #[serde(default)]
    pub body: Option<String>,
    /// Raw source text of a declaration or textual representation (feature/classifier decl body,
    /// action body decl, opaque member, or `TextualRepresentation` content).
    #[serde(default)]
    pub text: Option<String>,
    /// Declared language tag for a `TextualRepresentation`.
    #[serde(default)]
    pub language: Option<String>,
    /// Declared keyword spelling for an opaque member / action body decl (hover-only use).
    #[serde(default)]
    pub keyword: Option<String>,
}

/// Debug-rendered expression text kept for consumers that still compare against the raw
/// rendered text rather than a structured tree (boolean-literal detection, unit-bracket
/// parsing, `::`-qualification checks, LHS/RHS assignment-target text). Replaces
/// `attributes["value"]`, `["defaultValue"]`, `["lhs"]`, `["rhs"]`, `["condition"]`,
/// `["isThen"]` (`UNIFY_CACHE_PROGRESS.md` chunk E).
///
/// Where a producer also has a parsed expression AST available, the structured tree is (and
/// remains) recorded separately as [`DeclaredSemanticFacts::feature_value`] or
/// `::own_expression`; this struct duplicates the *same* already-computed rendered string 1:1
/// with the previous JSON attribute value, so no consumer-visible text changes. A handful of
/// producers (`use_case.rs` `isThen`/"actor redefinition" `rhs`, `analysis_case.rs`'s
/// `parse_analysis_attributes_from_other`, `requirement_body.rs`'s span-fallback `defaultValue`)
/// never had a parsed expression to begin with -- only authored/derived text -- so this struct
/// is their sole representation, matching [`TransitionEndpointFacts`]'s authored-text pattern.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredExpressionText {
    /// Rendered declared/initial value text (was `attributes["value"]`).
    #[serde(default)]
    pub value: Option<String>,
    /// Rendered attribute-def default-value text (was `attributes["defaultValue"]`).
    #[serde(default)]
    pub default_value: Option<String>,
    /// Rendered LHS of a `verify`/`assign` comparison (was `attributes["lhs"]`).
    #[serde(default)]
    pub lhs: Option<String>,
    /// Rendered RHS of a `verify`/`assign` comparison, or of an actor redefinition (was
    /// `attributes["rhs"]`).
    #[serde(default)]
    pub rhs: Option<String>,
    /// Rendered guard/filter condition text (was `attributes["condition"]`).
    #[serde(default)]
    pub condition: Option<String>,
    /// True when a `verify`/`assign` comparison, or a chained use case, was declared with
    /// `then` (was `attributes["isThen"]`).
    #[serde(default)]
    pub is_then: Option<bool>,
}

/// A node in the semantic graph representing a model element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticNode {
    pub id: NodeId,
    pub element_kind: ElementKind,
    /// Identifier text authored in the declaration, if any. This is distinct
    /// from [`Self::name`], which is the effective name used for identity and
    /// resolution (and may be inherited by an unnamed redefinition).
    #[serde(default)]
    pub declared_name: Option<String>,
    pub name: String,
    pub range: TextRange,
    pub attributes: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub declared_facts: DeclaredSemanticFacts,
    /// Typed source-fidelity/documentation text. See [`SourceTextFacts`].
    #[serde(default)]
    pub source_text: SourceTextFacts,
    /// Typed rendered-expression-text facts. See [`DeclaredExpressionText`].
    #[serde(default)]
    pub expression_text: DeclaredExpressionText,
    pub parent_id: Option<NodeId>,
}

/// True if `node`'s declared name or short name (see [`DeclaredSemanticFacts::short_name`])
/// equals `target`. Use this instead of `node.name == target` anywhere a simple-name match is
/// used for reference resolution, so short names declared alongside a regular name
/// (`part def <'CB'> ControlBoard;`) resolve as real alternate identifiers, matching SysML v2/
/// KerML semantics, instead of only being findable in the raw source text.
pub fn node_matches_simple_name(node: &SemanticNode, target: &str) -> bool {
    node.name == target || node.declared_facts.short_name.as_deref() == Some(target)
}
