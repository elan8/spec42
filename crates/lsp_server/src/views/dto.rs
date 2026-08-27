//! DTOs for the supported `sysml/*` extension requests.

use serde::{Deserialize, Serialize};

/// The neutral position/range wire structs every `sysml/*` DTO in this module shares.
///
/// Re-exported so a protocol adapter can spell them without naming the mutable-model crate: they
/// are serialization shapes, but a module that must not reach semantic state should not have to
/// import from the crate that owns it to say where something is.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionDto {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RangeDto {
    pub start: PositionDto,
    pub end: PositionDto,
}

pub fn range_to_dto(range: sysml_query::resolved_slice::TextRange) -> RangeDto {
    RangeDto {
        start: PositionDto {
            line: range.start.line,
            character: range.start.character,
        },
        end: PositionDto {
            line: range.end.line,
            character: range.end.character,
        },
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifierDto {
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorParamsDto {
    #[serde(default)]
    pub text_document: Option<TextDocumentIdentifierDto>,
    /// Accepted for compatibility with early 0.46 clients that sent the document URI flat.
    #[serde(default)]
    pub uri: Option<String>,
    pub position: PositionDto,
}

/// A published element, addressed by the identity the publication assigns it.
///
/// `id` is that identity; `qualifiedName` is the `::`-joined display path, which is not unique.
/// Keeping them apart is what lets a client round-trip a selection without re-deriving it from a
/// name.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorElementRefDto {
    pub id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    pub uri: String,
    pub range: RangeDto,
}

/// What one authored relationship family of an element settled to.
///
/// `status` is the publication's own outcome, so an empty `targets` never has to mean both "the
/// author wrote nothing" and "what the author wrote did not resolve". `candidates` carries the
/// alternatives of an ambiguous family, which are deliberately not promoted into `targets`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorResolutionDto {
    pub status: String,
    pub targets: Vec<SysmlFeatureInspectorElementRefDto>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<SysmlFeatureInspectorElementRefDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorRelationshipDto {
    #[serde(rename = "type")]
    pub rel_type: String,
    pub peer: SysmlFeatureInspectorElementRefDto,
    /// `authored` or `implied`: an inspector must not show a relationship the resolver synthesized
    /// as one the author wrote.
    pub provenance: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorInheritedFeatureDto {
    pub feature: SysmlFeatureInspectorElementRefDto,
    pub declared_in: SysmlFeatureInspectorElementRefDto,
}

/// The publication's evaluation state for one element, projected at the transport boundary.
///
/// One variant per published state. The value is carried only by the states that have one, so a
/// missing value cannot be read as a successful evaluation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum SysmlFeatureInspectorEvaluationDto {
    /// The element carries no expression.
    NotApplicable,
    /// The build did not evaluate.
    NotRun,
    /// The author wrote a value.
    Literal {
        value: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    /// The expression folded to a constant.
    Evaluated {
        value: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    /// A supported expression over an operand that has no constant value.
    NonConstant,
    /// The value depends on itself.
    Cyclic,
    /// The expression's shape is outside the evaluated slice.
    Unsupported,
    /// Evaluation ran and could not produce a value.
    Failed { reason: String },
}

/// The verdict channel of an analysis case, verification case, requirement or constraint.
///
/// Separate from the value channel because the two answer different questions. An element whose
/// kind states no verdict is `notApplicable`, which is not the same as one whose expression did
/// not settle.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum SysmlFeatureInspectorAnalysisDto {
    NotApplicable,
    NotRun,
    Verdict {
        passed: bool,
    },
    Computed {
        value: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    /// Verdict-bearing, and its expression did not settle. `evaluation` names the published
    /// evaluation state that says why.
    Unsettled {
        evaluation: String,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorElementDto {
    pub id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    pub role: String,
    pub declaration: String,
    pub uri: String,
    pub range: RangeDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<SysmlFeatureInspectorElementRefDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplicity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    pub modifiers: Vec<String>,
    pub evaluation: SysmlFeatureInspectorEvaluationDto,
    pub analysis: SysmlFeatureInspectorAnalysisDto,
    pub typing: SysmlFeatureInspectorResolutionDto,
    pub effective_typing: SysmlFeatureInspectorResolutionDto,
    pub specialization: SysmlFeatureInspectorResolutionDto,
    pub subsetting: SysmlFeatureInspectorResolutionDto,
    pub redefinition: SysmlFeatureInspectorResolutionDto,
    pub inherited_features: Vec<SysmlFeatureInspectorInheritedFeatureDto>,
    pub metadata: Vec<SysmlFeatureInspectorElementRefDto>,
    pub incoming_relationships: Vec<SysmlFeatureInspectorRelationshipDto>,
    pub outgoing_relationships: Vec<SysmlFeatureInspectorRelationshipDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorSelectionDto {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorLanguageHelpDto {
    pub keyword: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
}

/// What a reference under the cursor points at.
///
/// A tagged outcome rather than a nullable element: "there is no reference here" and "there is a
/// reference here that did not resolve" are different answers, and an ambiguous reference keeps
/// every candidate instead of presenting one of them as the target.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum SysmlFeatureInspectorReferenceDto {
    None,
    Resolved {
        element: Box<SysmlFeatureInspectorElementDto>,
    },
    Ambiguous {
        candidates: Vec<SysmlFeatureInspectorElementDto>,
    },
    Unresolved,
    Unsupported,
    /// The publication did not converge, so the reference has no settled answer.
    Incomplete,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlFeatureInspectorResultDto {
    pub version: u32,
    pub source_uri: String,
    pub requested_position: PositionDto,
    pub semantic_status: language_service::dto::SemanticResultStatus,
    pub selection: SysmlFeatureInspectorSelectionDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_help: Option<SysmlFeatureInspectorLanguageHelpDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containing_element: Option<SysmlFeatureInspectorElementDto>,
    pub referenced: SysmlFeatureInspectorReferenceDto,
}

#[derive(Debug, Serialize)]
pub struct SysmlServerStatsDto {
    pub uptime: u64,
    pub memory: SysmlServerMemoryDto,
    pub caches: SysmlServerCachesDto,
}

#[derive(Debug, Serialize)]
pub struct SysmlServerMemoryDto {
    pub rss: u64,
}

#[derive(Debug, Serialize)]
pub struct SysmlServerCachesDto {
    pub documents: usize,
    #[serde(rename = "symbolTables")]
    pub symbol_tables: usize,
    #[serde(rename = "semanticTokens")]
    pub semantic_tokens: usize,
}

#[derive(Debug, Serialize)]
pub struct SysmlClearCacheResultDto {
    pub documents: usize,
    #[serde(rename = "symbolTables")]
    pub symbol_tables: usize,
    #[serde(rename = "semanticTokens")]
    pub semantic_tokens: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchParamsDto {
    pub query: String,
    pub limit: Option<usize>,
    /// Owning document URI used to select a project publication in monorepo workspaces.
    #[serde(default)]
    pub project_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchItemDto {
    pub name: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    pub uri: String,
    pub range: RangeDto,
    pub score: i64,
    pub source: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchPackageDto {
    pub name: String,
    pub path: String,
    pub source: String,
    pub symbols: Vec<SysmlLibrarySearchItemDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchSourceDto {
    pub source: String,
    pub packages: Vec<SysmlLibrarySearchPackageDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysmlLibrarySearchResultDto {
    pub sources: Vec<SysmlLibrarySearchSourceDto>,
    pub symbol_total: usize,
    pub total: usize,
}

/// Client notification emitted when the workspace semantic index reaches `Ready`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SemanticIndexReadyNotificationDto {
    pub lifecycle: String,
    pub semantic_state_version: u64,
    pub workspace_file_count: usize,
}
