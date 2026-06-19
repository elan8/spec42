//! Protocol-neutral SysML editor language services.
//!
//! Hosts (LSP, Babel42 HTTP, CLI) call these APIs instead of depending on `tower-lsp`.

pub mod dto;
pub mod keywords;
pub mod lookup;
pub mod navigation;
mod presentation_hover;
pub mod references;
pub mod symbol;
pub mod text;
pub mod uri;
pub mod workspace;

pub use dto::{
    DefinitionResult, HoverResult, ReferencesResult, SourceLocation,
};
pub use keywords::{
    is_reserved_keyword, keyword_doc, keyword_hover_markdown, sysml_keywords, RESERVED_KEYWORDS,
};
pub use navigation::{find_references, goto_definition, hover};
pub use presentation_hover::{hover_markdown_for_node, signature_from_node};
pub use symbol::{find_reference_ranges, symbol_entries_for_uri, symbol_hover_markdown, SymbolEntry};
pub use text::{
    completion_prefix, line_prefix_at_position, position_to_byte_offset,
    unit_value_suffix_at_position, word_at_position,
};
pub use workspace::{InMemoryWorkspace, WorkspaceSnapshot};
