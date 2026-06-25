//! Core semantic graph node identity and relationship kinds.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::semantic::text_span::TextRange;
use url::Url;

/// Unique identifier for a node in the semantic graph.
/// Combines document URI and qualified name for workspace-wide uniqueness.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct NodeId {
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
    Connection,
    Bind,
    /// Control/data flow relationship inside behaviors (e.g. `flow`, `first ... then ...`).
    Flow,
    Perform,
    Allocate,
    Satisfy,
    Subject,
    Reference,
    Derivation,
    Transition,
    /// `then` initial state in a state composite (`transition` without `first` uses the same resolution path with [`RelationshipKind::Transition`]).
    InitialState,
    /// Metadata usage annotates a model element (`annotatedElement` per SysML §7.27).
    Annotation,
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
    ItemDef,
    AttributeDef,
    ActionDef,
    ActorDef,
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
    // Sub-elements / structural
    Transition,
    FinalState,
    ForLoop,
    Assign,
    Assert,
    AssertConstraint,
    RequireConstraint,
    Binding,
    Merge,
    Filter,
    Import,
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
            ElementKind::ItemDef => "item def",
            ElementKind::AttributeDef => "attribute def",
            ElementKind::ActionDef => "action def",
            ElementKind::ActorDef => "actor def",
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
            ElementKind::Transition => "transition",
            ElementKind::FinalState => "final state",
            ElementKind::ForLoop => "for loop",
            ElementKind::Assign => "assign",
            ElementKind::Assert => "assert",
            ElementKind::AssertConstraint => "assert constraint",
            ElementKind::RequireConstraint => "require constraint",
            ElementKind::Binding => "binding",
            ElementKind::Merge => "merge",
            ElementKind::Filter => "filter",
            ElementKind::Import => "import",
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
            "item def" => ElementKind::ItemDef,
            "attribute def" => ElementKind::AttributeDef,
            "action def" => ElementKind::ActionDef,
            "actor def" => ElementKind::ActorDef,
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
            "requirement" => ElementKind::Requirement,
            "use case" => ElementKind::UseCase,
            "concern" => ElementKind::Concern,
            "analysis" => ElementKind::Analysis,
            "verification" => ElementKind::Verification,
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
            "transition" => ElementKind::Transition,
            "final state" => ElementKind::FinalState,
            "for loop" => ElementKind::ForLoop,
            "assign" => ElementKind::Assign,
            "assert" => ElementKind::Assert,
            "assert constraint" => ElementKind::AssertConstraint,
            "require constraint" => ElementKind::RequireConstraint,
            "binding" => ElementKind::Binding,
            "merge" => ElementKind::Merge,
            "filter" => ElementKind::Filter,
            "import" => ElementKind::Import,
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
    pub declaring_uri: Url,
    pub range: TextRange,
    pub source_expression: String,
    pub target_expression: String,
    pub container_prefix: Option<String>,
}

/// Edge weight in the semantic graph: relationship kind plus optional connect metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEdge {
    pub kind: RelationshipKind,
    /// Set when this `Connection` came from a resolved `connect` (or pending-expression resolve).
    pub connect: Option<ConnectStatementDetail>,
}

impl SemanticEdge {
    pub fn plain(kind: RelationshipKind) -> Self {
        Self {
            kind,
            connect: None,
        }
    }

    pub fn connection_with_connect(connect: ConnectStatementDetail) -> Self {
        Self {
            kind: RelationshipKind::Connection,
            connect: Some(connect),
        }
    }
}

impl RelationshipKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationshipKind::Typing => "typing",
            RelationshipKind::Specializes => "specializes",
            RelationshipKind::Connection => "connection",
            RelationshipKind::Bind => "bind",
            RelationshipKind::Flow => "flow",
            RelationshipKind::Perform => "perform",
            RelationshipKind::Allocate => "allocate",
            RelationshipKind::Satisfy => "satisfy",
            RelationshipKind::Subject => "subject",
            RelationshipKind::Reference => "reference",
            RelationshipKind::Derivation => "derivation",
            RelationshipKind::Transition => "transition",
            RelationshipKind::InitialState => "initialState",
            RelationshipKind::Annotation => "annotation",
        }
    }

    /// Parses persisted relationship type strings (host projection storage).
    pub fn from_persisted_type(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "typing" => Some(RelationshipKind::Typing),
            "specializes" => Some(RelationshipKind::Specializes),
            "connection" => Some(RelationshipKind::Connection),
            "bind" => Some(RelationshipKind::Bind),
            "flow" => Some(RelationshipKind::Flow),
            "perform" => Some(RelationshipKind::Perform),
            "allocate" => Some(RelationshipKind::Allocate),
            "satisfy" => Some(RelationshipKind::Satisfy),
            "subject" => Some(RelationshipKind::Subject),
            "reference" => Some(RelationshipKind::Reference),
            "derivation" => Some(RelationshipKind::Derivation),
            "transition" | "initialstate" => Some(RelationshipKind::Transition),
            "annotation" => Some(RelationshipKind::Annotation),
            _ => None,
        }
    }
}

/// A node in the semantic graph representing a model element.
#[derive(Debug, Clone)]
pub struct SemanticNode {
    pub id: NodeId,
    pub element_kind: ElementKind,
    pub name: String,
    pub range: TextRange,
    pub attributes: HashMap<String, serde_json::Value>,
    pub parent_id: Option<NodeId>,
}
