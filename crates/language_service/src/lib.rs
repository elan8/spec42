//! Protocol-neutral SysML editor language services.
//!
//! Hosts (LSP, embedding services, CLI) call these APIs instead of depending on `tower-lsp`.

pub mod code_actions;
pub mod completion;
pub mod dto;
pub mod formatting;
pub mod keywords;
pub mod library_search;
pub mod lookup;
pub mod navigation;
mod outline;
pub mod presentation_hover;
pub mod references;
pub mod rename;
mod source_display;
pub mod symbol;
pub mod text;
pub mod uri;
pub mod workspace;
pub mod workspace_symbols;

pub use code_actions::{explicit_redefinition_quick_fix, reference_quick_fixes};
pub use completion::{
    complete, completion_edit_shape, detect_completion_context, CompletionContext,
    ATTRIBUTE_TYPE_LOOKUP_KINDS, PART_TYPE_LOOKUP_KINDS, PORT_TYPE_LOOKUP_KINDS,
};
pub use dto::{
    CompletionItemDto, CompletionItemKindDto, CompletionResult, DefinitionResult, FoldingRangeDto,
    FoldingRangeKindDto, HoverResult, OutlineSymbol, ReferencesResult, SourceLocation, TextEditDto,
    TextEditSuggestion, WorkspaceSymbolMatch,
};
pub use formatting::{format_document_text, FormatOptions};
pub use keywords::{
    is_reserved_keyword, keyword_doc, keyword_help, keyword_hover_markdown, sysml_keywords,
    KeywordHelp, RESERVED_KEYWORDS,
};
pub use library_search::{
    build_library_tree, library_search_score, library_source_label,
    search_symbols_from_recovered_short_names, LibrarySearchItem, LibrarySearchPackage,
    LibrarySearchSource, RecoverySearchSymbol,
};
pub use navigation::{find_references, goto_definition, hover, hover_report};
pub use outline::{document_symbols, folding_ranges};
pub use presentation_hover::{
    render_hover_markdown, render_hover_sexpr, HoverBlock, HoverLink, HoverRelation, HoverReport,
    HoverResolutionState, HoverUnitOutcome,
};
pub use rename::{apply_rename, prepare_rename, rename_target, RenameTarget};
pub use symbol::{symbol_entries_for_uri, symbol_hover_markdown, SymbolEntry};
pub use text::{completion_prefix, line_prefix_at_position, position_to_byte_offset, utf16_len};
pub use workspace::{InMemoryWorkspace, WorkspaceSnapshot};
pub use workspace_symbols::search_workspace_symbols;
