#![recursion_limit = "256"]

//! Protocol-neutral SysML editor language services.
//!
//! Hosts (LSP, embedding services, CLI) call these APIs instead of depending on `tower-lsp`.

pub mod code_actions;
pub mod completion;
pub mod dto;
pub mod formatting;
pub mod keywords;
pub mod lookup;
pub mod navigation;
mod outline;
pub mod references;
pub mod rename;
pub mod symbol;
pub mod text;
pub mod uri;
pub mod workspace;
pub mod workspace_symbols;

pub use code_actions::{
    suggest_add_import_quick_fixes, suggest_add_missing_case_subject_quick_fix,
    suggest_create_definition_for_unresolved_type_quick_fix,
    suggest_create_matching_part_def_quick_fix, suggest_create_usage_from_definition,
    suggest_create_verification_case, suggest_explicit_redefinition_quick_fix,
    suggest_qualify_ambiguous_name_quick_fixes, suggest_wrap_in_package, DiagnosticLine,
};
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
pub use navigation::{find_references, goto_definition, hover};
pub use outline::{document_symbols, folding_ranges};
pub use rename::{apply_rename, prepare_rename, rename_target, RenameTarget};
pub use symbol::{
    find_reference_ranges, symbol_entries_for_uri, symbol_hover_markdown, SymbolEntry,
};
pub use text::{
    completion_prefix, line_prefix_at_position, position_to_byte_offset,
    unit_value_suffix_at_position, unit_value_suffix_selection_at_position, utf16_len,
    word_at_position,
};
pub use workspace::{InMemoryWorkspace, WorkspaceSnapshot};
pub use workspace_symbols::search_workspace_symbols;
