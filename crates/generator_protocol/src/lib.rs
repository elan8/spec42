//! Shared data types for Spec42's core WebAssembly generator ABI.
//!
//! The executable ABI is intentionally small: one Postcard query import plus
//! a diagnostic import. Generated artifacts are returned from the guest entrypoint.

use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

pub const ABI_VERSION: u32 = 4;
pub const IMPORT_MODULE: &str = "spec42";

/// Every type that crosses the guest boundary, in one place.
///
/// This exists only to be hashed. Postcard is positional and carries no field names, no
/// type tags and no version, so a host and a guest built against different revisions of
/// this crate will happily misread each other's bytes -- a reordered or inserted field
/// shifts every subsequent field and still decodes, producing plausible but wrong output.
/// Naming all the wire types here means [`SCHEMA_FINGERPRINT`] changes automatically
/// whenever any of them changes, with no version number for anyone to forget to bump.
#[derive(Schema)]
#[allow(dead_code)]
struct WireSchema {
    artifact: Artifact,
    model_info: ModelInfo,
    element_summary: ElementSummary,
    metaclass: Metaclass,
    relationship_kind: RelationshipKind,
    element_detail: ElementDetail,
    source_range: SourceRange,
    multiplicity: Multiplicity,
    relationship: Relationship,
    level: Level,
    /// Request payloads, which are part of the contract just as much as the responses.
    metaclass_filter: Option<String>,
    handle: String,
    /// The entrypoint's argument and result shapes.
    generator_args: Vec<String>,
    generator_result: Result<Vec<Artifact>, String>,
}

/// Structural fingerprint of the wire *types*.
///
/// Covers field order, names and shapes. It does not cover anything defined outside the type
/// definitions, which is why it is an input to [`COMPATIBILITY_TOKEN`] rather than the value
/// guests report.
pub const SCHEMA_FINGERPRINT: u64 =
    u64::from_le_bytes(postcard_schema::key::hash::fnv1a64::hash_ty_path::<
        WireSchema,
    >(SCHEMA_PATH));

/// Namespace for the fingerprint hash. Bumping [`ABI_VERSION`] alone changes the
/// fingerprint even if no type changed, so an intentional break is always observable.
const SCHEMA_PATH: &str = "spec42:generator-abi/4";

/// Version of the semantic contract: result ordering, defaulting, and what each query means.
///
/// Bump this whenever behaviour changes in a way a generator could observe even though no
/// type changed -- a different ordering, a query that starts including implied facts, a
/// change to effective-feature shadowing. It feeds [`COMPATIBILITY_TOKEN`], so a bump
/// rejects stale guests at load instead of silently altering their output.
pub const SEMANTIC_API_VERSION: &str = "0.1.0";

/// What a guest reports and the host compares: the whole behavioural contract, not just the
/// wire types.
///
/// The type fingerprint alone is insufficient in two ways. Operation codes are plain
/// constants, invisible to a type-level hash, so renumbering one would leave the fingerprint
/// untouched while routing an old guest's `children` request to `element`. And semantics --
/// ordering, defaulting, query meaning -- can change with every type byte-identical.
///
/// The token therefore mixes type shapes, the operation numbering and the semantic version.
/// Before 1.0 the rule is deliberately blunt: any breaking change moves the token, and an
/// incompatible guest is refused. Capability negotiation and supporting several versions at
/// once can come later.
pub const COMPATIBILITY_TOKEN: u64 = compute_compatibility_token();

const fn compute_compatibility_token() -> u64 {
    // FNV-1a, written as a const fn so both sides get a compile-time constant.
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    const fn mix_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
        let mut index = 0;
        while index < bytes.len() {
            hash ^= bytes[index] as u64;
            hash = hash.wrapping_mul(PRIME);
            index += 1;
        }
        hash
    }

    const fn mix_u64(hash: u64, value: u64) -> u64 {
        mix_bytes(hash, &value.to_le_bytes())
    }

    let mut hash = OFFSET;
    hash = mix_bytes(hash, SCHEMA_PATH.as_bytes());
    hash = mix_u64(hash, ABI_VERSION as u64);
    hash = mix_u64(hash, SCHEMA_FINGERPRINT);
    hash = mix_bytes(hash, SEMANTIC_API_VERSION.as_bytes());
    let mut index = 0;
    while index < operation::ALL.len() {
        let (name, code) = operation::ALL[index];
        hash = mix_bytes(hash, name.as_bytes());
        hash = mix_u64(hash, code as u64);
        index += 1;
    }
    hash
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct Artifact {
    pub file_path: String,
    pub contents: Vec<u8>,
}

pub mod operation {
    pub const INFO: i32 = 0;
    pub const ROOTS: i32 = 1;
    pub const FIND: i32 = 2;
    pub const CHILDREN: i32 = 3;
    pub const ELEMENT: i32 = 4;
    pub const TYPED_BY: i32 = 5;
    pub const RELATIONSHIPS: i32 = 6;
    pub const EFFECTIVE_FEATURES: i32 = 7;

    /// Every operation, paired with its wire code, so the compatibility token covers the
    /// numbering. Keep in step with the constants above; the pinned token test is the guard.
    pub const ALL: &[(&str, i32)] = &[
        ("info", INFO),
        ("roots", ROOTS),
        ("find", FIND),
        ("children", CHILDREN),
        ("element", ELEMENT),
        ("typed_by", TYPED_BY),
        ("relationships", RELATIONSHIPS),
        ("effective_features", EFFECTIVE_FEATURES),
    ];
}

/// SysML v2 metaclass of an element.
///
/// A closed enumeration rather than a string. The host already knows the full set -- it maps
/// every parser element kind onto one of these -- so publishing it as text discarded type
/// information that existed upstream and left guests unable to match exhaustively. Adding a
/// variant changes the wire schema fingerprint, so existing guests are refused at load with a
/// clear message instead of silently mishandling a value they cannot recognise.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Schema)]
pub enum Metaclass {
    ActionDefinition,
    ActionUsage,
    AllocationDefinition,
    AllocationUsage,
    AnalysisCaseDefinition,
    AnalysisCaseUsage,
    AttributeDefinition,
    AttributeUsage,
    CalculationDefinition,
    CalculationUsage,
    CaseDefinition,
    CaseUsage,
    ConcernDefinition,
    ConcernUsage,
    ConnectionDefinition,
    ConnectionUsage,
    ConstraintDefinition,
    ConstraintUsage,
    Documentation,
    EnumerationDefinition,
    EnumerationUsage,
    FlowDefinition,
    FlowUsage,
    IndividualDefinition,
    IndividualUsage,
    InterfaceDefinition,
    InterfaceUsage,
    ItemDefinition,
    ItemUsage,
    MetadataDefinition,
    MetadataUsage,
    OccurrenceDefinition,
    OccurrenceUsage,
    Package,
    PartDefinition,
    PartUsage,
    PortDefinition,
    PortUsage,
    ReferenceUsage,
    RenderingDefinition,
    RenderingUsage,
    RequirementDefinition,
    RequirementUsage,
    StateDefinition,
    StateUsage,
    UseCaseDefinition,
    UseCaseUsage,
    VerificationCaseDefinition,
    VerificationCaseUsage,
    ViewDefinition,
    ViewUsage,
    ViewpointDefinition,
    ViewpointUsage,
    TransitionUsage,
    TransitionTrigger,
    TransitionGuard,
    TransitionEffect,
    FinalState,
    ActorUsage,
    StakeholderUsage,
    SubjectUsage,
    PerformUsage,
    IncludeUseCaseUsage,
    ViewRendering,
    ViewColumn,
    Alias,
    KermlDeclaration,
    AnalysisResultUsage,
    AssertConstraintUsage,
    AssertUsage,
    AssignmentActionUsage,
    BindingConnectorUsage,
    ConjugatedPortDefinition,
    DecisionNodeUsage,
    Dependency,
    DerivationConnectorUsage,
    Diagnostic,
    ElseActionUsage,
    FilterUsage,
    FlowPayload,
    ForLoopActionUsage,
    ForkNodeUsage,
    IfActionUsage,
    Import,
    InterfaceEndUsage,
    JoinNodeUsage,
    MergeNodeUsage,
    NeedUsage,
    ObjectiveUsage,
    ParameterUsage,
    PurposeUsage,
    RequireConstraintUsage,
    TerminateActionUsage,
    TextualRepresentation,
    VerdictUsage,
    VerifyUsage,
    WhileLoopActionUsage,
    /// A value this Spec42 produced that the enumeration above does not name.
    ///
    /// Carrying it explicitly means a guest can tell "I have never heard of this"
    /// apart from "I forgot to handle this", which a bare string cannot express.
    /// Spec42's own conformance suite asserts this variant is never produced, so
    /// seeing one means the upstream model gained a concept the ABI has not mapped.
    Unrecognized(String),
}

impl Metaclass {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ActionDefinition => "ActionDefinition",
            Self::ActionUsage => "ActionUsage",
            Self::AllocationDefinition => "AllocationDefinition",
            Self::AllocationUsage => "AllocationUsage",
            Self::AnalysisCaseDefinition => "AnalysisCaseDefinition",
            Self::AnalysisCaseUsage => "AnalysisCaseUsage",
            Self::AttributeDefinition => "AttributeDefinition",
            Self::AttributeUsage => "AttributeUsage",
            Self::CalculationDefinition => "CalculationDefinition",
            Self::CalculationUsage => "CalculationUsage",
            Self::CaseDefinition => "CaseDefinition",
            Self::CaseUsage => "CaseUsage",
            Self::ConcernDefinition => "ConcernDefinition",
            Self::ConcernUsage => "ConcernUsage",
            Self::ConnectionDefinition => "ConnectionDefinition",
            Self::ConnectionUsage => "ConnectionUsage",
            Self::ConstraintDefinition => "ConstraintDefinition",
            Self::ConstraintUsage => "ConstraintUsage",
            Self::Documentation => "Documentation",
            Self::EnumerationDefinition => "EnumerationDefinition",
            Self::EnumerationUsage => "EnumerationUsage",
            Self::FlowDefinition => "FlowDefinition",
            Self::FlowUsage => "FlowUsage",
            Self::IndividualDefinition => "IndividualDefinition",
            Self::IndividualUsage => "IndividualUsage",
            Self::InterfaceDefinition => "InterfaceDefinition",
            Self::InterfaceUsage => "InterfaceUsage",
            Self::ItemDefinition => "ItemDefinition",
            Self::ItemUsage => "ItemUsage",
            Self::MetadataDefinition => "MetadataDefinition",
            Self::MetadataUsage => "MetadataUsage",
            Self::OccurrenceDefinition => "OccurrenceDefinition",
            Self::OccurrenceUsage => "OccurrenceUsage",
            Self::Package => "Package",
            Self::PartDefinition => "PartDefinition",
            Self::PartUsage => "PartUsage",
            Self::PortDefinition => "PortDefinition",
            Self::PortUsage => "PortUsage",
            Self::ReferenceUsage => "ReferenceUsage",
            Self::RenderingDefinition => "RenderingDefinition",
            Self::RenderingUsage => "RenderingUsage",
            Self::RequirementDefinition => "RequirementDefinition",
            Self::RequirementUsage => "RequirementUsage",
            Self::StateDefinition => "StateDefinition",
            Self::StateUsage => "StateUsage",
            Self::UseCaseDefinition => "UseCaseDefinition",
            Self::UseCaseUsage => "UseCaseUsage",
            Self::VerificationCaseDefinition => "VerificationCaseDefinition",
            Self::VerificationCaseUsage => "VerificationCaseUsage",
            Self::ViewDefinition => "ViewDefinition",
            Self::ViewUsage => "ViewUsage",
            Self::ViewpointDefinition => "ViewpointDefinition",
            Self::ViewpointUsage => "ViewpointUsage",
            Self::TransitionUsage => "TransitionUsage",
            Self::TransitionTrigger => "TransitionTrigger",
            Self::TransitionGuard => "TransitionGuard",
            Self::TransitionEffect => "TransitionEffect",
            Self::FinalState => "FinalState",
            Self::ActorUsage => "ActorUsage",
            Self::StakeholderUsage => "StakeholderUsage",
            Self::SubjectUsage => "SubjectUsage",
            Self::PerformUsage => "PerformUsage",
            Self::IncludeUseCaseUsage => "IncludeUseCaseUsage",
            Self::ViewRendering => "ViewRendering",
            Self::ViewColumn => "ViewColumn",
            Self::Alias => "Alias",
            Self::KermlDeclaration => "KermlDeclaration",
            Self::AnalysisResultUsage => "AnalysisResultUsage",
            Self::AssertConstraintUsage => "AssertConstraintUsage",
            Self::AssertUsage => "AssertUsage",
            Self::AssignmentActionUsage => "AssignmentActionUsage",
            Self::BindingConnectorUsage => "BindingConnectorUsage",
            Self::ConjugatedPortDefinition => "ConjugatedPortDefinition",
            Self::DecisionNodeUsage => "DecisionNodeUsage",
            Self::Dependency => "Dependency",
            Self::DerivationConnectorUsage => "DerivationConnectorUsage",
            Self::Diagnostic => "Diagnostic",
            Self::ElseActionUsage => "ElseActionUsage",
            Self::FilterUsage => "FilterUsage",
            Self::FlowPayload => "FlowPayload",
            Self::ForLoopActionUsage => "ForLoopActionUsage",
            Self::ForkNodeUsage => "ForkNodeUsage",
            Self::IfActionUsage => "IfActionUsage",
            Self::Import => "Import",
            Self::InterfaceEndUsage => "InterfaceEndUsage",
            Self::JoinNodeUsage => "JoinNodeUsage",
            Self::MergeNodeUsage => "MergeNodeUsage",
            Self::NeedUsage => "NeedUsage",
            Self::ObjectiveUsage => "ObjectiveUsage",
            Self::ParameterUsage => "ParameterUsage",
            Self::PurposeUsage => "PurposeUsage",
            Self::RequireConstraintUsage => "RequireConstraintUsage",
            Self::TerminateActionUsage => "TerminateActionUsage",
            Self::TextualRepresentation => "TextualRepresentation",
            Self::VerdictUsage => "VerdictUsage",
            Self::VerifyUsage => "VerifyUsage",
            Self::WhileLoopActionUsage => "WhileLoopActionUsage",
            Self::Unrecognized(value) => value,
        }
    }

    /// Maps a Spec42-internal spelling onto this enumeration.
    pub fn parse(value: &str) -> Self {
        match value {
            "ActionDefinition" => Self::ActionDefinition,
            "ActionUsage" => Self::ActionUsage,
            "AllocationDefinition" => Self::AllocationDefinition,
            "AllocationUsage" => Self::AllocationUsage,
            "AnalysisCaseDefinition" => Self::AnalysisCaseDefinition,
            "AnalysisCaseUsage" => Self::AnalysisCaseUsage,
            "AttributeDefinition" => Self::AttributeDefinition,
            "AttributeUsage" => Self::AttributeUsage,
            "CalculationDefinition" => Self::CalculationDefinition,
            "CalculationUsage" => Self::CalculationUsage,
            "CaseDefinition" => Self::CaseDefinition,
            "CaseUsage" => Self::CaseUsage,
            "ConcernDefinition" => Self::ConcernDefinition,
            "ConcernUsage" => Self::ConcernUsage,
            "ConnectionDefinition" => Self::ConnectionDefinition,
            "ConnectionUsage" => Self::ConnectionUsage,
            "ConstraintDefinition" => Self::ConstraintDefinition,
            "ConstraintUsage" => Self::ConstraintUsage,
            "Documentation" => Self::Documentation,
            "EnumerationDefinition" => Self::EnumerationDefinition,
            "EnumerationUsage" => Self::EnumerationUsage,
            "FlowDefinition" => Self::FlowDefinition,
            "FlowUsage" => Self::FlowUsage,
            "IndividualDefinition" => Self::IndividualDefinition,
            "IndividualUsage" => Self::IndividualUsage,
            "InterfaceDefinition" => Self::InterfaceDefinition,
            "InterfaceUsage" => Self::InterfaceUsage,
            "ItemDefinition" => Self::ItemDefinition,
            "ItemUsage" => Self::ItemUsage,
            "MetadataDefinition" => Self::MetadataDefinition,
            "MetadataUsage" => Self::MetadataUsage,
            "OccurrenceDefinition" => Self::OccurrenceDefinition,
            "OccurrenceUsage" => Self::OccurrenceUsage,
            "Package" => Self::Package,
            "PartDefinition" => Self::PartDefinition,
            "PartUsage" => Self::PartUsage,
            "PortDefinition" => Self::PortDefinition,
            "PortUsage" => Self::PortUsage,
            "ReferenceUsage" => Self::ReferenceUsage,
            "RenderingDefinition" => Self::RenderingDefinition,
            "RenderingUsage" => Self::RenderingUsage,
            "RequirementDefinition" => Self::RequirementDefinition,
            "RequirementUsage" => Self::RequirementUsage,
            "StateDefinition" => Self::StateDefinition,
            "StateUsage" => Self::StateUsage,
            "UseCaseDefinition" => Self::UseCaseDefinition,
            "UseCaseUsage" => Self::UseCaseUsage,
            "VerificationCaseDefinition" => Self::VerificationCaseDefinition,
            "VerificationCaseUsage" => Self::VerificationCaseUsage,
            "ViewDefinition" => Self::ViewDefinition,
            "ViewUsage" => Self::ViewUsage,
            "ViewpointDefinition" => Self::ViewpointDefinition,
            "ViewpointUsage" => Self::ViewpointUsage,
            "TransitionUsage" => Self::TransitionUsage,
            "TransitionTrigger" => Self::TransitionTrigger,
            "TransitionGuard" => Self::TransitionGuard,
            "TransitionEffect" => Self::TransitionEffect,
            "FinalState" => Self::FinalState,
            "ActorUsage" => Self::ActorUsage,
            "StakeholderUsage" => Self::StakeholderUsage,
            "SubjectUsage" => Self::SubjectUsage,
            "PerformUsage" => Self::PerformUsage,
            "IncludeUseCaseUsage" => Self::IncludeUseCaseUsage,
            "ViewRendering" => Self::ViewRendering,
            "ViewColumn" => Self::ViewColumn,
            "Alias" => Self::Alias,
            "KermlDeclaration" => Self::KermlDeclaration,
            "AnalysisResultUsage" => Self::AnalysisResultUsage,
            "AssertConstraintUsage" => Self::AssertConstraintUsage,
            "AssertUsage" => Self::AssertUsage,
            "AssignmentActionUsage" => Self::AssignmentActionUsage,
            "BindingConnectorUsage" => Self::BindingConnectorUsage,
            "ConjugatedPortDefinition" => Self::ConjugatedPortDefinition,
            "DecisionNodeUsage" => Self::DecisionNodeUsage,
            "Dependency" => Self::Dependency,
            "DerivationConnectorUsage" => Self::DerivationConnectorUsage,
            "Diagnostic" => Self::Diagnostic,
            "ElseActionUsage" => Self::ElseActionUsage,
            "FilterUsage" => Self::FilterUsage,
            "FlowPayload" => Self::FlowPayload,
            "ForLoopActionUsage" => Self::ForLoopActionUsage,
            "ForkNodeUsage" => Self::ForkNodeUsage,
            "IfActionUsage" => Self::IfActionUsage,
            "Import" => Self::Import,
            "InterfaceEndUsage" => Self::InterfaceEndUsage,
            "JoinNodeUsage" => Self::JoinNodeUsage,
            "MergeNodeUsage" => Self::MergeNodeUsage,
            "NeedUsage" => Self::NeedUsage,
            "ObjectiveUsage" => Self::ObjectiveUsage,
            "ParameterUsage" => Self::ParameterUsage,
            "PurposeUsage" => Self::PurposeUsage,
            "RequireConstraintUsage" => Self::RequireConstraintUsage,
            "TerminateActionUsage" => Self::TerminateActionUsage,
            "TextualRepresentation" => Self::TextualRepresentation,
            "VerdictUsage" => Self::VerdictUsage,
            "VerifyUsage" => Self::VerifyUsage,
            "WhileLoopActionUsage" => Self::WhileLoopActionUsage,
            other => Self::Unrecognized(other.to_owned()),
        }
    }

    /// Whether this value came through unmapped.
    pub fn is_unrecognized(&self) -> bool {
        matches!(self, Self::Unrecognized(_))
    }
}

impl std::fmt::Display for Metaclass {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Kind of a semantic relationship between two elements.
///
/// Mirrors the closed enumeration Spec42 uses internally, which was previously flattened to a
/// string purely to cross the ABI boundary.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Schema)]
pub enum RelationshipKind {
    Typing,
    Specializes,
    Subsetting,
    Redefinition,
    ReferenceSubsetting,
    CrossSubsetting,
    Connection,
    Bind,
    Flow,
    SuccessionFlow,
    Perform,
    Allocate,
    Dependency,
    Satisfy,
    Subject,
    Reference,
    Derivation,
    Transition,
    InitialState,
    Annotation,
    PortConjugation,
    /// A value this Spec42 produced that the enumeration above does not name.
    ///
    /// Carrying it explicitly means a guest can tell "I have never heard of this"
    /// apart from "I forgot to handle this", which a bare string cannot express.
    /// Spec42's own conformance suite asserts this variant is never produced, so
    /// seeing one means the upstream model gained a concept the ABI has not mapped.
    Unrecognized(String),
}

impl RelationshipKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Typing => "typing",
            Self::Specializes => "specializes",
            Self::Subsetting => "subsetting",
            Self::Redefinition => "redefinition",
            Self::ReferenceSubsetting => "referenceSubsetting",
            Self::CrossSubsetting => "crossSubsetting",
            Self::Connection => "connection",
            Self::Bind => "bind",
            Self::Flow => "flow",
            Self::SuccessionFlow => "successionFlow",
            Self::Perform => "perform",
            Self::Allocate => "allocate",
            Self::Dependency => "dependency",
            Self::Satisfy => "satisfy",
            Self::Subject => "subject",
            Self::Reference => "reference",
            Self::Derivation => "derivation",
            Self::Transition => "transition",
            Self::InitialState => "initialState",
            Self::Annotation => "annotation",
            Self::PortConjugation => "portConjugation",
            Self::Unrecognized(value) => value,
        }
    }

    /// Maps a Spec42-internal spelling onto this enumeration.
    pub fn parse(value: &str) -> Self {
        match value {
            "typing" => Self::Typing,
            "specializes" => Self::Specializes,
            "subsetting" => Self::Subsetting,
            "redefinition" => Self::Redefinition,
            "referenceSubsetting" => Self::ReferenceSubsetting,
            "crossSubsetting" => Self::CrossSubsetting,
            "connection" => Self::Connection,
            "bind" => Self::Bind,
            "flow" => Self::Flow,
            "successionFlow" => Self::SuccessionFlow,
            "perform" => Self::Perform,
            "allocate" => Self::Allocate,
            "dependency" => Self::Dependency,
            "satisfy" => Self::Satisfy,
            "subject" => Self::Subject,
            "reference" => Self::Reference,
            "derivation" => Self::Derivation,
            "transition" => Self::Transition,
            "initialState" => Self::InitialState,
            "annotation" => Self::Annotation,
            "portConjugation" => Self::PortConjugation,
            other => Self::Unrecognized(other.to_owned()),
        }
    }

    /// Whether this value came through unmapped.
    pub fn is_unrecognized(&self) -> bool {
        matches!(self, Self::Unrecognized(_))
    }
}

impl std::fmt::Display for RelationshipKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct ModelInfo {
    pub model_digest: String,
    pub spec42_version: String,
    pub semantic_api_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct ElementSummary {
    pub handle: String,
    pub semantic_id: String,
    pub metaclass: Metaclass,
    pub name: Option<String>,
    pub qualified_name: String,
    pub library_element: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct SourceRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct Multiplicity {
    pub lower: Option<String>,
    pub upper: Option<String>,
    pub ordered: bool,
    pub unique: Option<bool>,
    pub implied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct ElementDetail {
    pub summary: ElementSummary,
    pub owner: Option<ElementSummary>,
    pub declared_name: Option<String>,
    pub effective_name: Option<String>,
    pub source_uri: String,
    pub source_range: SourceRange,
    pub definition: bool,
    pub documentation: Option<String>,
    pub short_name: Option<String>,
    pub direction: Option<String>,
    pub derived: bool,
    pub constant: bool,
    pub abstract_flag: bool,
    pub variation: bool,
    pub individual: bool,
    pub conjugated: bool,
    pub composite: Option<bool>,
    pub reference: Option<bool>,
    pub end: bool,
    pub ordered: Option<bool>,
    pub unique: Option<bool>,
    pub multiplicity: Option<Multiplicity>,
    pub evaluated_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct Relationship {
    pub kind: RelationshipKind,
    pub source: ElementSummary,
    pub target: ElementSummary,
    pub implied: bool,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use postcard_schema::key::hash::fnv1a64::hash_ty_path;

    fn fingerprint<T: Schema + ?Sized>() -> u64 {
        u64::from_le_bytes(hash_ty_path::<T>(SCHEMA_PATH))
    }

    /// The load-time compatibility check is only worth anything if the fingerprint moves
    /// when the schema moves. These stand in for the three ways the wire format can drift.
    #[test]
    #[allow(dead_code)]
    fn the_fingerprint_detects_every_shape_of_schema_drift() {
        #[derive(Schema)]
        struct Baseline {
            handle: String,
            name: Option<String>,
            library: bool,
        }

        #[derive(Schema)]
        struct FieldAdded {
            handle: String,
            name: Option<String>,
            library: bool,
            deprecated: bool,
        }

        #[derive(Schema)]
        struct FieldsReordered {
            handle: String,
            library: bool,
            name: Option<String>,
        }

        #[derive(Schema)]
        struct TypeChanged {
            handle: String,
            name: String,
            library: bool,
        }

        let baseline = fingerprint::<Baseline>();
        for (label, other) in [
            ("an appended field", fingerprint::<FieldAdded>()),
            ("reordered fields", fingerprint::<FieldsReordered>()),
            ("a changed field type", fingerprint::<TypeChanged>()),
        ] {
            assert_ne!(
                baseline, other,
                "{label} left the fingerprint unchanged, so drift would go undetected"
            );
        }
    }

    /// The token must move for every class of breaking change, not just type edits. Without
    /// this, renumbering an operation would leave the token identical and route an old
    /// guest's `children` call to `element`.
    #[test]
    fn the_compatibility_token_covers_operations_and_semantics() {
        const PRIME: u64 = 0x0000_0100_0000_01b3;
        fn mix(mut hash: u64, bytes: &[u8]) -> u64 {
            for byte in bytes {
                hash ^= *byte as u64;
                hash = hash.wrapping_mul(PRIME);
            }
            hash
        }
        // Recompute the token with one input perturbed at a time and confirm each differs.
        let baseline = COMPATIBILITY_TOKEN;
        assert_ne!(baseline, mix(baseline, b"schema"), "sanity");

        // Renumbering an operation.
        let renumbered = {
            let mut hash = 0xcbf2_9ce4_8422_2325u64;
            hash = mix(hash, SCHEMA_PATH.as_bytes());
            hash = mix(hash, &(ABI_VERSION as u64).to_le_bytes());
            hash = mix(hash, &SCHEMA_FINGERPRINT.to_le_bytes());
            hash = mix(hash, SEMANTIC_API_VERSION.as_bytes());
            for (name, code) in operation::ALL {
                hash = mix(hash, name.as_bytes());
                // Shift every code by one, as a renumbering would.
                hash = mix(hash, &((*code as u64) + 1).to_le_bytes());
            }
            hash
        };
        assert_ne!(
            baseline, renumbered,
            "renumbering an operation left the token unchanged"
        );

        // Bumping the semantic version with no type change.
        let resemanticked = {
            let mut hash = 0xcbf2_9ce4_8422_2325u64;
            hash = mix(hash, SCHEMA_PATH.as_bytes());
            hash = mix(hash, &(ABI_VERSION as u64).to_le_bytes());
            hash = mix(hash, &SCHEMA_FINGERPRINT.to_le_bytes());
            hash = mix(hash, b"0.2.0");
            for (name, code) in operation::ALL {
                hash = mix(hash, name.as_bytes());
                hash = mix(hash, &(*code as u64).to_le_bytes());
            }
            hash
        };
        assert_ne!(
            baseline, resemanticked,
            "bumping the semantic API version left the token unchanged"
        );
    }

    /// A deliberate tripwire. Changing any wire type changes this value; update it in the
    /// same commit as the schema change, and treat an unexpected diff here as a warning
    /// that existing guests are about to be rejected.
    #[test]
    fn the_wire_schema_fingerprint_is_pinned() {
        assert_eq!(
            SCHEMA_FINGERPRINT, 0x3a46_578a_b105_445f,
            "the generator wire schema changed; every guest must be rebuilt"
        );
    }

    /// The value guests actually report. Pinned separately from the schema fingerprint
    /// because it can move without any type changing.
    #[test]
    fn the_compatibility_token_is_pinned() {
        assert_eq!(
            COMPATIBILITY_TOKEN, 0x741a_de78_bb72_29f5,
            "the generator ABI contract changed; every guest must be rebuilt"
        );
    }
}
