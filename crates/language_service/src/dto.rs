use serde::{Deserialize, Serialize};
use semantic_core::TextRange;

/// A source location using a logical path string and neutral text range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub path: String,
    pub range: TextRange,
}

/// Hover result with Markdown contents and optional word range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverResult {
    pub contents: String,
    pub range: Option<TextRange>,
}

/// Go-to-definition result: one or more target locations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefinitionResult {
    pub locations: Vec<SourceLocation>,
}

/// Find-references result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencesResult {
    pub locations: Vec<SourceLocation>,
}
