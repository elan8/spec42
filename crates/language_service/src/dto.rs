use serde::{Deserialize, Serialize};
use sysml_model::TextRange;

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

/// Neutral completion item kind (not LSP-specific).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionItemKindDto {
    Keyword,
    Snippet,
    Module,
    Class,
    Interface,
    Function,
    Property,
    Variable,
    Event,
    Reference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionItemLabelDetailsDto {
    pub detail: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionTextEditDto {
    pub range: TextRange,
    pub new_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionItemDto {
    pub label: String,
    pub kind: Option<CompletionItemKindDto>,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub documentation_is_markdown: bool,
    pub label_details: Option<CompletionItemLabelDetailsDto>,
    pub filter_text: Option<String>,
    pub text_edit: Option<CompletionTextEditDto>,
    pub insert_text_format_snippet: bool,
    pub sort_text: Option<String>,
    pub preselect: bool,
    pub deprecated: bool,
    pub resolve_detail: Option<String>,
    pub resolve_documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionResult {
    pub items: Vec<CompletionItemDto>,
    pub is_incomplete: bool,
}

/// Replace range for completion edits at a cursor position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionEditShape {
    pub replace_range: TextRange,
}

/// Outline symbol from document AST (protocol-neutral).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutlineSymbol {
    pub name: String,
    pub kind: String,
    pub range: TextRange,
    pub selection_range: TextRange,
    pub children: Vec<OutlineSymbol>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FoldingRangeKindDto {
    Region,
    Imports,
    Comment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldingRangeDto {
    pub start_line: u32,
    pub end_line: u32,
    pub kind: Option<FoldingRangeKindDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSymbolMatch {
    pub name: String,
    pub path: String,
    pub uri: String,
    pub range: TextRange,
    pub container: Option<String>,
    pub detail: Option<String>,
}

/// Neutral text edit for rename and quick-fix operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEditDto {
    pub path: String,
    pub range: TextRange,
    pub replacement: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEditSuggestion {
    pub title: String,
    pub edits: Vec<TextEditDto>,
}
