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
    Connection,
    Bind,
    /// Control/data flow relationship inside behaviors (e.g. `flow`, `first ... then ...`).
    Flow,
    /// `FlowUsageKind::SuccessionFlow` — a flow that also implies a succession/ordering
    /// constraint between the connected actions, distinct from a plain data/control [`Flow`](Self::Flow).
    SuccessionFlow,
    Perform,
    Allocate,
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
    /// Set when this `Connection` came from a resolved `connect` (or pending-expression resolve).
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
            RelationshipKind::Connection => "connection",
            RelationshipKind::Bind => "bind",
            RelationshipKind::Flow => "flow",
            RelationshipKind::SuccessionFlow => "successionFlow",
            RelationshipKind::Perform => "perform",
            RelationshipKind::Allocate => "allocate",
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
            "connection" => Some(RelationshipKind::Connection),
            "bind" => Some(RelationshipKind::Bind),
            "flow" => Some(RelationshipKind::Flow),
            "successionflow" => Some(RelationshipKind::SuccessionFlow),
            "perform" => Some(RelationshipKind::Perform),
            "allocate" => Some(RelationshipKind::Allocate),
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
    #[serde(default)]
    pub multiplicity: Option<DeclaredMultiplicity>,
    #[serde(default)]
    pub feature_value: Option<DeclaredFeatureValue>,
    /// Explicit declaration modifiers retained from the parser AST.
    /// Absent for packages and other non-feature/definition nodes.
    #[serde(default)]
    pub feature_properties: Option<DeclaredFeatureProperties>,
    /// Set when this node's own declared substance *is* an expression (e.g. a
    /// `TransitionGuard` child), as opposed to `feature_value`, which represents "this
    /// feature's value is X". Projected as `HostElementFacts::content_expression_id`.
    #[serde(default)]
    pub own_expression: Option<DeclaredExpression>,
}

/// Explicit feature/definition modifiers from the textual declaration.
///
/// Composite/reference ownership is inferred from ordinary usage vs `RefDecl`
/// materialization (`ElementKind::Ref`). Conjugation is set when a type
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

/// A node in the semantic graph representing a model element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticNode {
    pub id: NodeId,
    pub element_kind: ElementKind,
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
