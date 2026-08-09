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
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Set when a structural expression relationship (`connect` or `bind`) was resolved, retaining
    /// its declaring context and endpoint expressions for instance projection.
    pub connect: Option<ConnectStatementDetail>,
    /// Set when this `Flow`/`SuccessionFlow` came from a resolved `flow` usage.
    #[serde(default)]
    pub flow: Option<FlowStatementDetail>,
}

impl SemanticEdge {
    pub fn plain(kind: RelationshipKind) -> Self {
        Self {
            kind,
            connect: None,
            flow: None,
        }
    }

    pub fn connection_with_connect(connect: ConnectStatementDetail) -> Self {
        Self {
            kind: RelationshipKind::Connection,
            connect: Some(connect),
            flow: None,
        }
    }

    pub fn interconnection_with_detail(
        kind: RelationshipKind,
        connect: ConnectStatementDetail,
    ) -> Self {
        Self {
            kind,
            connect: Some(connect),
            flow: None,
        }
    }

    pub fn flow_with_detail(kind: RelationshipKind, flow: FlowStatementDetail) -> Self {
        Self {
            kind,
            connect: None,
            flow: Some(flow),
        }
    }
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
    /// Set when this node's own declared substance *is* an expression (e.g. a
    /// `TransitionGuard` child), as opposed to `feature_value`, which represents "this
    /// feature's value is X". Projected as `HostElementFacts::content_expression_id`.
    #[serde(default)]
    pub own_expression: Option<DeclaredExpression>,
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
/// effective private default published by [`SemanticGraph::effective_membership_visibility_for`].
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DeclaredMembershipKind {
    Owning,
    Feature,
    Import,
    Alias,
    Variant,
    Actor,
    /// A graph-owned synthesized membership whose parser grammar has no Membership wrapper.
    Synthesized,
}

impl Default for DeclaredMembershipKind {
    fn default() -> Self {
        Self::Synthesized
    }
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
    /// Exact target span when the parser exposes it; otherwise the enclosing import span.
    #[serde(default)]
    pub range: Option<TextRange>,
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
/// combined result through [`SemanticGraph::effective_feature_ownership_for`].
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
    if expression.kind == "integerLiteral" {
        return expression
            .literal
            .as_ref()
            .and_then(serde_json::Value::as_i64)
            .map(DeclaredMultiplicityBound::Integer)
            .unwrap_or(DeclaredMultiplicityBound::NonIntegerLiteral);
    }
    if expression.kind == "unary" && expression.children.len() == 1 {
        let child = direct_multiplicity_bound(expression.children.first());
        return match (expression.operator.as_deref(), child) {
            (Some("+"), DeclaredMultiplicityBound::Integer(value)) => {
                DeclaredMultiplicityBound::Integer(value)
            }
            (Some("-"), DeclaredMultiplicityBound::Integer(value)) => value
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
    pub kind: String,
    pub range: TextRange,
    #[serde(default)]
    pub literal: Option<serde_json::Value>,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub children: Vec<DeclaredExpression>,
    #[serde(default)]
    pub arguments: Vec<DeclaredExpressionArgument>,
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
    pub parent_id: Option<NodeId>,
}

/// True if `node`'s declared name or short name (see `ast_util::attach_short_name_attribute`)
/// equals `target`. Use this instead of `node.name == target` anywhere a simple-name match is
/// used for reference resolution, so short names declared alongside a regular name
/// (`part def <'CB'> ControlBoard;`) resolve as real alternate identifiers, matching SysML v2/
/// KerML semantics, instead of only being findable in the raw source text.
pub fn node_matches_simple_name(node: &SemanticNode, target: &str) -> bool {
    node.name == target
        || node
            .attributes
            .get("shortName")
            .and_then(serde_json::Value::as_str)
            == Some(target)
}
