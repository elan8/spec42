//! Helpers for hover and completion: position/word resolution, keywords, and AST name collection.
//! Also provides definition/reference ranges for Go to definition and Find references.

mod symbols;

pub use language_service::{
    completion_prefix, is_reserved_keyword, keyword_doc, keyword_hover_markdown, line_prefix_at_position,
    position_to_byte_offset, sysml_keywords, unit_value_suffix_at_position, word_at_position,
    RESERVED_KEYWORDS,
};
#[cfg(test)]
mod position {
    use tower_lsp::lsp_types::{Position, Range};

    #[derive(Debug, Clone)]
    pub struct SourcePosition {
        pub line: u32,
        pub character: u32,
        pub length: u32,
    }

    pub fn source_position_to_range(pos: &SourcePosition) -> Range {
        Range::new(
            Position::new(pos.line, pos.character),
            Position::new(pos.line, pos.character + pos.length),
        )
    }

    #[derive(Debug, Clone)]
    pub struct SourceRange {
        pub start_line: u32,
        pub start_character: u32,
        pub end_line: u32,
        pub end_character: u32,
    }

    pub fn source_range_to_range(r: &SourceRange) -> Range {
        Range::new(
            Position::new(r.start_line, r.start_character),
            Position::new(r.end_line, r.end_character),
        )
    }
}
#[cfg(test)]
pub use position::{source_position_to_range, source_range_to_range, SourcePosition, SourceRange};
pub use symbols::{
    collect_definition_ranges, collect_document_symbols, collect_folding_ranges,
    find_reference_ranges, SymbolEntry,
};
#[cfg(test)]
pub use symbols::{collect_named_elements, collect_symbol_entries};

use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, Command, Diagnostic, FormattingOptions, OneOf,
    OptionalVersionedTextDocumentIdentifier, Position, Range, TextDocumentEdit, TextEdit, Url,
    WorkspaceEdit,
};
use language_service::{
    format_document_text, DiagnosticLine, FormatOptions, TextEditSuggestion,
};

fn suggestion_to_code_action(
    suggestion: TextEditSuggestion,
    uri: &Url,
    diagnostic: Option<&Diagnostic>,
) -> CodeAction {
    let edits: Vec<OneOf<TextEdit, tower_lsp::lsp_types::AnnotatedTextEdit>> = suggestion
        .edits
        .into_iter()
        .map(|edit| {
            OneOf::Left(TextEdit {
                range: crate::common::text_span::to_lsp_range(edit.range),
                new_text: edit.replacement,
            })
        })
        .collect();
    CodeAction {
        title: suggestion.title,
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: diagnostic.map(|d| vec![d.clone()]),
        edit: Some(WorkspaceEdit {
            changes: None,
            document_changes: Some(tower_lsp::lsp_types::DocumentChanges::Edits(vec![
                TextDocumentEdit {
                    text_document: OptionalVersionedTextDocumentIdentifier {
                        uri: uri.clone(),
                        version: None,
                    },
                    edits,
                },
            ])),
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    }
}

fn wrap_refactor_action(suggestion: TextEditSuggestion, uri: &Url) -> CodeAction {
    let edits: Vec<OneOf<TextEdit, tower_lsp::lsp_types::AnnotatedTextEdit>> = suggestion
        .edits
        .into_iter()
        .map(|edit| {
            OneOf::Left(TextEdit {
                range: crate::common::text_span::to_lsp_range(edit.range),
                new_text: edit.replacement,
            })
        })
        .collect();
    CodeAction {
        title: suggestion.title,
        kind: Some(CodeActionKind::REFACTOR),
        diagnostics: None,
        edit: Some(WorkspaceEdit {
            changes: None,
            document_changes: Some(tower_lsp::lsp_types::DocumentChanges::Edits(vec![
                TextDocumentEdit {
                    text_document: OptionalVersionedTextDocumentIdentifier {
                        uri: uri.clone(),
                        version: None,
                    },
                    edits,
                },
            ])),
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    }
}

/// Formats the whole document: trim trailing whitespace per line, single trailing newline, indent by brace depth.
pub fn format_document(source: &str, options: &FormattingOptions) -> Vec<TextEdit> {
    let new_text = format_document_text(
        source,
        FormatOptions {
            tab_size: options.tab_size,
            insert_spaces: options.insert_spaces,
        },
    );
    let lines: Vec<&str> = source.lines().collect();
    if lines.is_empty() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 0));
        return vec![TextEdit { range, new_text }];
    }
    let last_line = (lines.len() - 1) as u32;
    let last_char = lines.last().map(|l| l.len()).unwrap_or(0) as u32;
    let range = Range::new(Position::new(0, 0), Position::new(last_line, last_char));
    vec![TextEdit { range, new_text }]
}

pub fn suggest_wrap_in_package(source: &str, uri: &Url) -> Option<CodeAction> {
    let path = uri.path().trim_start_matches('/').to_string();
    language_service::suggest_wrap_in_package(source, &path).map(|s| wrap_refactor_action(s, uri))
}

pub fn suggest_create_definition_for_unresolved_type_quick_fix(
    source: &str,
    uri: &Url,
    diagnostic: &Diagnostic,
) -> Option<CodeAction> {
    let path = uri.path().trim_start_matches('/').to_string();
    language_service::suggest_create_definition_for_unresolved_type_quick_fix(
        source,
        &path,
        DiagnosticLine {
            line: diagnostic.range.start.line,
        },
    )
    .map(|s| suggestion_to_code_action(s, uri, Some(diagnostic)))
}

pub fn suggest_create_matching_part_def_quick_fix(
    source: &str,
    uri: &Url,
    diagnostic: &Diagnostic,
) -> Option<CodeAction> {
    let path = uri.path().trim_start_matches('/').to_string();
    language_service::suggest_create_matching_part_def_quick_fix(
        source,
        &path,
        DiagnosticLine {
            line: diagnostic.range.start.line,
        },
    )
    .map(|s| suggestion_to_code_action(s, uri, Some(diagnostic)))
}

pub fn suggest_explicit_redefinition_quick_fix(
    source: &str,
    uri: &Url,
    diagnostic: &Diagnostic,
) -> Option<CodeAction> {
    let path = uri.path().trim_start_matches('/').to_string();
    language_service::suggest_explicit_redefinition_quick_fix(
        source,
        &path,
        DiagnosticLine {
            line: diagnostic.range.start.line,
        },
    )
    .map(|s| suggestion_to_code_action(s, uri, Some(diagnostic)))
}

pub fn suggest_manage_custom_libraries_quick_fix(diagnostic: &Diagnostic) -> CodeAction {
    CodeAction {
        title: "Configure SysML library paths".to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: Some(vec![diagnostic.clone()]),
        edit: None,
        command: Some(Command {
            title: "Configure SysML library paths".to_string(),
            command: "sysml.library.managePaths".to_string(),
            arguments: None,
        }),
        is_preferred: Some(false),
        disabled: None,
        data: None,
    }
}

pub fn suggest_show_standard_library_info_quick_fix(diagnostic: &Diagnostic) -> CodeAction {
    CodeAction {
        title: "Show bundled standard library information".to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: Some(vec![diagnostic.clone()]),
        edit: None,
        command: Some(Command {
            title: "Show bundled standard library information".to_string(),
            command: "sysml.library.showStdLibStatus".to_string(),
            arguments: None,
        }),
        is_preferred: Some(false),
        disabled: None,
        data: None,
    }
}

pub fn suggest_open_library_view_quick_fix(diagnostic: &Diagnostic) -> CodeAction {
    CodeAction {
        title: "Open Spec42 Library view".to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: Some(vec![diagnostic.clone()]),
        edit: None,
        command: Some(Command {
            title: "Open Spec42 Library view".to_string(),
            command: "sysml.library.search".to_string(),
            arguments: None,
        }),
        is_preferred: Some(false),
        disabled: None,
        data: None,
    }
}

fn library_search_symbol_from_diagnostic(diagnostic: &Diagnostic) -> Option<String> {
    let message = diagnostic.message.as_str();
    for quote in ['\'', '`', '"'] {
        let Some(start) = message.find(quote) else {
            continue;
        };
        let rest = &message[start + quote.len_utf8()..];
        let Some(end) = rest.find(quote) else {
            continue;
        };
        let candidate = rest[..end].trim();
        if candidate
            .chars()
            .all(|ch| ch.is_alphanumeric() || ch == '_' || ch == ':')
            && !candidate.is_empty()
        {
            return Some(
                candidate
                    .rsplit("::")
                    .next()
                    .unwrap_or(candidate)
                    .to_string(),
            );
        }
    }
    None
}

pub fn suggest_search_library_for_symbol_quick_fix(diagnostic: &Diagnostic) -> Option<CodeAction> {
    let symbol = library_search_symbol_from_diagnostic(diagnostic)?;
    Some(CodeAction {
        title: format!("Search Library for `{symbol}`"),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: Some(vec![diagnostic.clone()]),
        edit: None,
        command: Some(Command {
            title: format!("Search Library for `{symbol}`"),
            command: "sysml.library.search".to_string(),
            arguments: Some(vec![serde_json::Value::String(symbol)]),
        }),
        is_preferred: Some(false),
        disabled: None,
        data: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Url};

    #[test]
    fn test_position_to_byte_offset() {
        let text = "abc\ndef\nghi";
        assert_eq!(position_to_byte_offset(text, 0, 0), Some(0));
        assert_eq!(position_to_byte_offset(text, 0, 2), Some(2));
        assert_eq!(position_to_byte_offset(text, 1, 0), Some(4));
        assert_eq!(position_to_byte_offset(text, 1, 3), Some(7));
        assert_eq!(position_to_byte_offset(text, 2, 0), Some(8));
        assert_eq!(position_to_byte_offset(text, 3, 0), None);
        assert_eq!(position_to_byte_offset(text, 0, 10), None);
    }

    #[test]
    fn test_position_to_byte_offset_multibyte_utf8() {
        // "caf\u{00E9}" = c,a,f,é = 4 chars, 5 bytes.
        let text = "caf\u{00E9}\n";
        assert_eq!(position_to_byte_offset(text, 0, 0), Some(0));
        assert_eq!(position_to_byte_offset(text, 0, 3), Some(3));
        assert_eq!(position_to_byte_offset(text, 0, 4), Some(5));
        assert_eq!(position_to_byte_offset(text, 0, 5), None);
        // Japanese: 2 chars, 6 bytes
        let text2 = "\u{65E5}\u{672C}\n";
        assert_eq!(position_to_byte_offset(text2, 0, 2), Some(6));
    }

    #[test]
    fn test_position_to_byte_offset_utf16_surrogate_pair() {
        let text = "a\u{1F600}b\n";
        assert_eq!(position_to_byte_offset(text, 0, 0), Some(0));
        assert_eq!(position_to_byte_offset(text, 0, 1), Some(1));
        assert_eq!(position_to_byte_offset(text, 0, 2), None);
        assert_eq!(position_to_byte_offset(text, 0, 3), Some(5));
        assert_eq!(position_to_byte_offset(text, 0, 4), Some(6));
    }

    #[test]
    fn test_unit_value_suffix_at_position() {
        let text = "package P { attribute v = 10 [kV]; }";
        let bracket_start = text.find("[kV]").expect("unit suffix") as u32;
        assert_eq!(
            unit_value_suffix_at_position(text, 0, bracket_start + 1),
            Some("kV".to_string())
        );
        assert_eq!(
            unit_value_suffix_at_position(text, 0, bracket_start + 2),
            Some("kV".to_string())
        );
        assert!(unit_value_suffix_at_position("multiplicity [0..1]", 0, 12).is_none());
    }

    #[test]
    fn test_word_at_position() {
        let text = "  part foo : Bar  ";
        let (line, start, end, word) = word_at_position(text, 0, 5).unwrap();
        assert_eq!(line, 0);
        assert_eq!(start, 2);
        assert_eq!(end, 6);
        assert_eq!(word, "part");

        let (_, _, _, w) = word_at_position(text, 0, 8).unwrap();
        assert_eq!(w, "foo");
        let (_, _, _, w) = word_at_position(text, 0, 13).unwrap();
        assert_eq!(w, "Bar");
    }

    #[test]
    fn test_word_at_position_non_ascii() {
        let text = "part caf\u{00E9} : String";
        let (_, _, _, w) = word_at_position(text, 0, 6).unwrap();
        assert_eq!(w, "caf\u{00E9}");
        let text2 = "part \u{54C1}\u{8A5E} : Type";
        let (_, _, _, w2) = word_at_position(text2, 0, 6).unwrap();
        assert_eq!(w2, "\u{54C1}\u{8A5E}");
    }

    #[test]
    fn test_word_at_position_empty_line() {
        let text = "abc";
        assert!(word_at_position(text, 0, 0).is_some());
        let (_, _, _, w) = word_at_position(text, 0, 0).unwrap();
        assert_eq!(w, "abc");
    }

    #[test]
    fn test_line_prefix_at_position() {
        let text = "  part foo";
        let prefix = line_prefix_at_position(text, 0, 7);
        assert_eq!(prefix, "  part ");
        let prefix = line_prefix_at_position(text, 0, 8);
        assert_eq!(prefix, "  part f");
    }

    #[test]
    fn test_completion_prefix() {
        assert_eq!(completion_prefix("  part "), "part");
        assert_eq!(completion_prefix("  part f"), "f");
        assert_eq!(completion_prefix("  pac"), "pac");
    }

    #[test]
    fn test_completion_prefix_multibyte() {
        assert_eq!(completion_prefix("  caf\u{00E9} "), "caf\u{00E9}");
        assert_eq!(
            completion_prefix("part \u{54C1}\u{8A5E} "),
            "\u{54C1}\u{8A5E}"
        );
    }

    #[test]
    fn test_keyword_doc() {
        assert!(keyword_doc("part").is_some());
        assert!(keyword_doc("unknown").is_none());
    }

    #[test]
    fn test_sysml_keywords_contains_common() {
        let kw = sysml_keywords();
        assert!(kw.contains(&"package"));
        assert!(kw.contains(&"part"));
        assert!(kw.contains(&"attribute"));
    }

    #[test]
    fn test_sysml_keywords_subset_of_reserved() {
        for kw in sysml_keywords() {
            assert!(
                is_reserved_keyword(kw),
                "sysml_keywords() must only contain reserved keywords; '{}' is not reserved",
                kw
            );
        }
    }

    #[test]
    fn test_position_not_reserved() {
        assert!(!is_reserved_keyword("position"));
    }

    #[test]
    fn test_collect_named_elements_empty() {
        let root = sysml_v2_parser::RootNamespace { elements: vec![] };
        let el = collect_named_elements(&root);
        assert!(el.is_empty());
    }

    #[test]
    fn test_collect_named_elements_from_package() {
        let text = "package P { part def Engine { } }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let el = collect_named_elements(&root);
        assert_eq!(el.len(), 2); // package P + part Engine
        let names: Vec<_> = el.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"P"));
        assert!(names.contains(&"Engine"));
    }

    #[test]
    fn test_collect_named_elements_feature_and_classifier_decls() {
        let text = "package P { feature myFeature : BaseFeature; class VehicleClass; }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let el = collect_named_elements(&root);
        let pairs: Vec<_> = el.iter().map(|(n, d)| (n.as_str(), d.as_str())).collect();
        assert!(pairs.contains(&("myFeature", "feature decl 'myFeature'")));
        assert!(pairs.contains(&("VehicleClass", "classifier decl 'VehicleClass'")));
    }

    #[test]
    fn test_source_position_to_range() {
        let pos = SourcePosition {
            line: 0,
            character: 2,
            length: 5,
        };
        let range = source_position_to_range(&pos);
        assert_eq!(range.start.line, 0);
        assert_eq!(range.start.character, 2);
        assert_eq!(range.end.line, 0);
        assert_eq!(range.end.character, 7);
    }

    #[test]
    fn test_collect_definition_ranges_empty() {
        let root = sysml_v2_parser::RootNamespace { elements: vec![] };
        let ranges = collect_definition_ranges(&root);
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_collect_definition_ranges_package() {
        let text = "package P { }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let ranges = collect_definition_ranges(&root);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].0, "P");
    }

    #[test]
    fn test_collect_definition_ranges_part_def() {
        // sysml-v2-parser requires package/namespace at root; part def must be nested
        let text = "package P { part def Engine { } }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let ranges = collect_definition_ranges(&root);
        assert_eq!(ranges.len(), 2); // package P + part Engine
        assert_eq!(ranges[0].0, "P");
        assert_eq!(ranges[1].0, "Engine");
    }

    #[test]
    fn test_collect_definition_ranges_feature_and_classifier_decls() {
        let text = "package P { feature myFeature : BaseFeature; class VehicleClass; }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let ranges = collect_definition_ranges(&root);
        let names: Vec<_> = ranges.iter().map(|(name, _)| name.as_str()).collect();
        assert!(names.contains(&"myFeature"));
        assert!(names.contains(&"VehicleClass"));
    }

    #[test]
    fn test_find_reference_ranges_empty() {
        let ranges = find_reference_ranges("hello world", "foo");
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_find_reference_ranges_once() {
        let ranges = find_reference_ranges("hello foo world", "foo");
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start.character, 6);
        assert_eq!(ranges[0].end.character, 9);
    }

    #[test]
    fn test_find_reference_ranges_multiple() {
        let ranges = find_reference_ranges("foo bar foo baz foo", "foo");
        assert_eq!(ranges.len(), 3);
    }

    #[test]
    fn test_find_reference_ranges_word_boundary() {
        // "foo" in "foobar" must not match
        let ranges = find_reference_ranges("foobar", "foo");
        assert!(ranges.is_empty());
        // "foo" in "foo bar" must match
        let ranges = find_reference_ranges("foo bar", "foo");
        assert_eq!(ranges.len(), 1);
    }

    #[test]
    fn test_collect_document_symbols_empty() {
        let root = sysml_v2_parser::RootNamespace { elements: vec![] };
        let symbols = collect_document_symbols(&root);
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_collect_document_symbols_package() {
        let text = "package P { }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let symbols = collect_document_symbols(&root);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "P");
        assert_eq!(symbols[0].detail.as_deref(), Some("package"));
        assert_eq!(symbols[0].kind, tower_lsp::lsp_types::SymbolKind::MODULE);
    }

    #[test]
    fn test_collect_document_symbols_nested() {
        let text = "package P { part def Engine { } }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let symbols = collect_document_symbols(&root);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "P");
        let children = symbols[0].children.as_ref().expect("children");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "Engine");
        assert_eq!(children[0].detail.as_deref(), Some("part def"));
        assert_eq!(children[0].kind, tower_lsp::lsp_types::SymbolKind::CLASS);
    }

    #[test]
    fn test_collect_document_symbols_feature_and_classifier_decls() {
        let text = "package P { feature myFeature : BaseFeature; class VehicleClass; }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let symbols = collect_document_symbols(&root);
        let children = symbols[0].children.as_ref().expect("children");
        assert!(children.iter().any(|child| {
            child.name == "myFeature"
                && child.detail.as_deref() == Some("feature decl")
                && child.kind == tower_lsp::lsp_types::SymbolKind::PROPERTY
        }));
        assert!(children.iter().any(|child| {
            child.name == "VehicleClass"
                && child.detail.as_deref() == Some("classifier decl")
                && child.kind == tower_lsp::lsp_types::SymbolKind::CLASS
        }));
    }

    #[test]
    fn test_collect_symbol_entries_empty() {
        let root = sysml_v2_parser::RootNamespace { elements: vec![] };
        let uri = Url::parse("file:///test.sysml").unwrap();
        let entries = collect_symbol_entries(&root, &uri);
        assert!(entries.is_empty());
    }

    #[test]
    fn test_collect_symbol_entries_package() {
        let text = "package P { }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let uri = Url::parse("file:///test.sysml").unwrap();
        let entries = collect_symbol_entries(&root, &uri);
        // collect_symbol_entries is currently stubbed (returns empty)
        assert!(entries.is_empty());
    }

    #[test]
    fn test_collect_symbol_entries_nested() {
        let text = "package P { part def Engine { } }";
        let root = sysml_v2_parser::parse(text).expect("parse");
        let uri = Url::parse("file:///test.sysml").unwrap();
        let entries = collect_symbol_entries(&root, &uri);
        // collect_symbol_entries is currently stubbed (returns empty)
        assert!(entries.is_empty());
    }

    #[test]
    fn test_suggest_wrap_in_package_empty() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let action = suggest_wrap_in_package("", &uri);
        assert!(action.is_none());
    }

    #[test]
    fn test_suggest_wrap_in_package_named_package() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let action = suggest_wrap_in_package("package P { }", &uri);
        assert!(action.is_none());
    }

    #[test]
    fn test_suggest_wrap_in_package_unwrapped_member() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        // When source is a single top-level part def, sysml-v2-parser may parse it as one anonymous package
        // with one member, in which case we suggest "Wrap in package".
        let source = "part def X { }";
        if let Some(action) = suggest_wrap_in_package(source, &uri) {
            assert!(action.title.contains("Wrap"));
            let edit = action.edit.expect("has edit");
            let doc_edits = edit.document_changes.as_ref().expect("document_changes");
            use tower_lsp::lsp_types::DocumentChanges;
            let edits = match doc_edits {
                DocumentChanges::Edits(v) => v,
                _ => panic!("expected Edits"),
            };
            assert_eq!(edits.len(), 1);
            assert_eq!(edits[0].edits.len(), 1);
            let text_edit = match &edits[0].edits[0] {
                tower_lsp::lsp_types::OneOf::Left(te) => te,
                _ => panic!("expected TextEdit"),
            };
            assert!(text_edit.new_text.contains("package Generated"));
            assert!(text_edit.new_text.contains("part def X"));
        }
    }

    #[test]
    fn test_suggest_create_matching_part_def_quick_fix_creates_def_and_types_usage() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let source = "package P {\n  part def Laptop {\n    part display;\n  }\n}\n";
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(2, 4), Position::new(2, 17)),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("untyped_part_usage".to_string())),
            code_description: None,
            source: Some("sysml".to_string()),
            message: "untyped".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action =
            suggest_create_matching_part_def_quick_fix(source, &uri, &diagnostic).expect("action");
        let edit = action.edit.expect("has edit");
        let doc_edits = edit.document_changes.expect("document changes");
        let edits = match doc_edits {
            tower_lsp::lsp_types::DocumentChanges::Edits(v) => v,
            _ => panic!("expected edits"),
        };
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].edits.len(), 2);
        let inserted = match &edits[0].edits[0] {
            OneOf::Left(te) => {
                assert_eq!(te.range.start.line, 1);
                assert_eq!(te.range.start.character, 0);
                te.new_text.clone()
            }
            _ => panic!("expected text edit"),
        };
        let rewritten = match &edits[0].edits[1] {
            OneOf::Left(te) => te.new_text.clone(),
            _ => panic!("expected text edit"),
        };
        assert!(inserted.contains("part def Display { }"));
        assert_eq!(rewritten.trim(), "part display : Display;");
    }

    #[test]
    fn test_suggest_create_matching_part_def_quick_fix_respects_four_space_indent() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let source = "package P {\n    part def Laptop {\n        part display;\n    }\n}\n";
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(2, 8), Position::new(2, 21)),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("untyped_part_usage".to_string())),
            code_description: None,
            source: Some("sysml".to_string()),
            message: "untyped".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action =
            suggest_create_matching_part_def_quick_fix(source, &uri, &diagnostic).expect("action");
        let edit = action.edit.expect("has edit");
        let doc_edits = edit.document_changes.expect("document changes");
        let edits = match doc_edits {
            tower_lsp::lsp_types::DocumentChanges::Edits(v) => v,
            _ => panic!("expected edits"),
        };
        let inserted = match &edits[0].edits[0] {
            OneOf::Left(te) => te.new_text.clone(),
            _ => panic!("expected text edit"),
        };
        assert_eq!(inserted, "    part def Display { }\n");
    }

    #[test]
    fn test_suggest_create_matching_part_def_quick_fix_no_package_uses_usage_indent() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let source = "part def Outer {\n    part display;\n}\n";
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(1, 4), Position::new(1, 17)),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("untyped_part_usage".to_string())),
            code_description: None,
            source: Some("sysml".to_string()),
            message: "untyped".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action =
            suggest_create_matching_part_def_quick_fix(source, &uri, &diagnostic).expect("action");
        let edit = action.edit.expect("has edit");
        let doc_edits = edit.document_changes.expect("document changes");
        let edits = match doc_edits {
            tower_lsp::lsp_types::DocumentChanges::Edits(v) => v,
            _ => panic!("expected edits"),
        };
        let inserted = match &edits[0].edits[0] {
            OneOf::Left(te) => {
                assert_eq!(te.range.start.line, 2);
                te.new_text.clone()
            }
            _ => panic!("expected text edit"),
        };
        assert_eq!(inserted, "    part def Display { }\n");
    }

    #[test]
    fn test_suggest_create_matching_part_def_quick_fix_noop_for_typed_usage() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let source = "package P {\n  part def Laptop {\n    part display : Display;\n  }\n}\n";
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(2, 4), Position::new(2, 27)),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("untyped_part_usage".to_string())),
            code_description: None,
            source: Some("sysml".to_string()),
            message: "untyped".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action = suggest_create_matching_part_def_quick_fix(source, &uri, &diagnostic);
        assert!(action.is_none());
    }

    #[test]
    fn test_suggest_create_definition_for_unresolved_type_creates_part_def() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let source = "package P {\n  part car : Vehicle;\n}\n";
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(1, 13), Position::new(1, 20)),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String(
                "unresolved_type_reference".to_string(),
            )),
            code_description: None,
            source: Some("semantic".to_string()),
            message: "unresolved".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action =
            suggest_create_definition_for_unresolved_type_quick_fix(source, &uri, &diagnostic)
                .expect("action");
        assert_eq!(action.title, "Create `part def Vehicle`");
        let edit = action.edit.expect("has edit");
        let doc_edits = edit.document_changes.expect("document changes");
        let edits = match doc_edits {
            tower_lsp::lsp_types::DocumentChanges::Edits(v) => v,
            _ => panic!("expected edits"),
        };
        let inserted = match &edits[0].edits[0] {
            OneOf::Left(te) => te.new_text.clone(),
            _ => panic!("expected text edit"),
        };
        assert_eq!(inserted, "  part def Vehicle { }\n");
    }

    #[test]
    fn test_suggest_create_definition_for_unresolved_type_creates_port_def() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let source = "package P {\n  port command : CommandPort;\n}\n";
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(1, 17), Position::new(1, 28)),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String(
                "unresolved_type_reference".to_string(),
            )),
            code_description: None,
            source: Some("semantic".to_string()),
            message: "unresolved".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action =
            suggest_create_definition_for_unresolved_type_quick_fix(source, &uri, &diagnostic)
                .expect("action");
        assert_eq!(action.title, "Create `port def CommandPort`");
        let edit = action.edit.expect("has edit");
        let doc_edits = edit.document_changes.expect("document changes");
        let edits = match doc_edits {
            tower_lsp::lsp_types::DocumentChanges::Edits(v) => v,
            _ => panic!("expected edits"),
        };
        let inserted = match &edits[0].edits[0] {
            OneOf::Left(te) => te.new_text.clone(),
            _ => panic!("expected text edit"),
        };
        assert_eq!(inserted, "  port def CommandPort;\n");
    }

    #[test]
    fn test_suggest_show_standard_library_info_quick_fix_uses_command() {
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 1)),
            severity: Some(DiagnosticSeverity::INFORMATION),
            code: Some(NumberOrString::String(
                "missing_library_context".to_string(),
            )),
            code_description: None,
            source: Some("semantic".to_string()),
            message: "missing library".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action = suggest_show_standard_library_info_quick_fix(&diagnostic);
        assert_eq!(
            action.command.expect("command").command,
            "sysml.library.showStdLibStatus"
        );
    }

    #[test]
    fn test_suggest_open_library_view_quick_fix_uses_search_command() {
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 1)),
            severity: Some(DiagnosticSeverity::INFORMATION),
            code: Some(NumberOrString::String(
                "missing_library_context".to_string(),
            )),
            code_description: None,
            source: Some("semantic".to_string()),
            message: "missing library".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action = suggest_open_library_view_quick_fix(&diagnostic);
        assert_eq!(
            action.command.expect("command").command,
            "sysml.library.search"
        );
    }

    #[test]
    fn test_suggest_search_library_for_symbol_quick_fix_uses_symbol_argument() {
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 1)),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String(
                "unresolved_type_reference".to_string(),
            )),
            code_description: None,
            source: Some("semantic".to_string()),
            message: "Type reference 'ScalarValues::Real' could not be resolved.".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action = suggest_search_library_for_symbol_quick_fix(&diagnostic).expect("action");
        let command = action.command.expect("command");
        assert_eq!(command.command, "sysml.library.search");
        assert_eq!(
            command.arguments.expect("args"),
            vec![serde_json::Value::String("Real".to_string())]
        );
    }

    #[test]
    fn test_suggest_explicit_redefinition_quick_fix_rewrites_line() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let source =
            "package P {\n  part def Child :> Base {\n    attribute mass = 1200;\n  }\n}\n";
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(2, 4), Position::new(2, 25)),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String(
                "implicit_redefinition_without_operator".to_string(),
            )),
            code_description: None,
            source: Some("semantic".to_string()),
            message: "missing :>>".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action =
            suggest_explicit_redefinition_quick_fix(source, &uri, &diagnostic).expect("action");
        let edit = action.edit.expect("has edit");
        let doc_edits = edit.document_changes.expect("document changes");
        let edits = match doc_edits {
            tower_lsp::lsp_types::DocumentChanges::Edits(v) => v,
            _ => panic!("expected edits"),
        };
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].edits.len(), 1);
        let rewritten = match &edits[0].edits[0] {
            OneOf::Left(te) => te.new_text.clone(),
            _ => panic!("expected text edit"),
        };
        assert_eq!(rewritten.trim(), "attribute :>> mass = 1200;");
    }

    #[test]
    fn test_suggest_explicit_redefinition_quick_fix_noop_when_already_explicit() {
        let uri = Url::parse("file:///test.sysml").unwrap();
        let source =
            "package P {\n  part def Child :> Base {\n    attribute :>> mass = 1200;\n  }\n}\n";
        let diagnostic = Diagnostic {
            range: Range::new(Position::new(2, 4), Position::new(2, 29)),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String(
                "implicit_redefinition_without_operator".to_string(),
            )),
            code_description: None,
            source: Some("semantic".to_string()),
            message: "missing :>>".to_string(),
            related_information: None,
            tags: None,
            data: None,
        };
        let action = suggest_explicit_redefinition_quick_fix(source, &uri, &diagnostic);
        assert!(action.is_none());
    }

    #[test]
    fn test_format_document_empty() {
        let options = tower_lsp::lsp_types::FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        };
        let edits = format_document("", &options);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "\n");
    }

    #[test]
    fn test_format_document_trim_trailing_whitespace() {
        let options = tower_lsp::lsp_types::FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        };
        let edits = format_document("package P {   \n  part def X { }  \n", &options);
        assert_eq!(edits.len(), 1);
        assert!(edits[0].new_text.contains("package P {"));
        assert!(edits[0].new_text.contains("part def X { }"));
        assert!(!edits[0].new_text.contains("   \n"));
        assert!(!edits[0].new_text.contains("  \n"));
    }

    #[test]
    fn test_format_document_indent_by_braces() {
        let options = tower_lsp::lsp_types::FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        };
        let source = "package P {\npart def X {\n}\n}\n";
        let edits = format_document(source, &options);
        assert_eq!(edits.len(), 1);
        let expected = "package P {\n  part def X {\n  }\n}\n";
        assert_eq!(edits[0].new_text, expected);
    }

    #[test]
    fn test_format_document_indents_members_with_inline_braces() {
        let options = tower_lsp::lsp_types::FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        };
        let source = "package IT{\npart def Motherboard { }\npart def Display { }\n}\n";
        let edits = format_document(source, &options);
        let text = &edits[0].new_text;
        assert!(text.contains("package IT{"));
        assert!(text.contains("  part def Motherboard { }"));
        assert!(text.contains("  part def Display { }"));
    }

    #[test]
    fn test_format_document_is_idempotent() {
        let options = tower_lsp::lsp_types::FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        };
        let source = "package P {\npart def Engine {\nattribute mass;\n}\n}\n";
        let first = format_document(source, &options)
            .into_iter()
            .next()
            .expect("first edit")
            .new_text;
        let second = format_document(&first, &options)
            .into_iter()
            .next()
            .expect("second edit")
            .new_text;
        assert_eq!(first, second);
    }

    #[test]
    fn test_format_document_normalizes_crlf_to_lf_and_single_trailing_newline() {
        let options = tower_lsp::lsp_types::FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        };
        let edits = format_document("package P {\r\npart def X;\r\n}\r\n\r\n", &options);
        assert_eq!(edits[0].new_text, "package P {\n  part def X;\n}\n");
    }

    #[test]
    fn test_format_document_nested_blocks() {
        let options = tower_lsp::lsp_types::FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        };
        let source = "package P {\npart def Vehicle {\npart engine {\nattribute rpm;\n}\n}\n}\n";
        let expected = "package P {\n    part def Vehicle {\n        part engine {\n            attribute rpm;\n        }\n    }\n}\n";
        let edits = format_document(source, &options);
        assert_eq!(edits[0].new_text, expected);
    }

    #[test]
    fn test_format_document_trims_comment_trailing_whitespace() {
        let options = tower_lsp::lsp_types::FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        };
        let source = "package P {\n// comment with brace {   \npart def X;  \n}\n";
        let edits = format_document(source, &options);
        assert!(edits[0].new_text.contains("  // comment with brace {"));
        assert!(edits[0].new_text.contains("  part def X;"));
        assert!(!edits[0].new_text.contains("   \n"));
        assert!(!edits[0].new_text.contains(";  \n"));
    }

    /// Validation test: parse VehicleDefinitions.sysml and write semantic tokens and symbol table
    /// to target/ for review (semantic_tokens_vehicle_definitions.txt, symbol_table_vehicle_definitions.txt).
    #[test]
    fn test_vehicle_definitions_validation_output() {
        let release_root = std::env::var_os("SYSML_V2_RELEASE_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join("temp")
                    .join("SysML-v2-Release-2026-01")
            });
        let path = release_root
            .join("sysml")
            .join("src")
            .join("examples")
            .join("Vehicle Example")
            .join("VehicleDefinitions.sysml");
        if !path.exists() {
            return; // skip when Vehicle Example not present (e.g. SYSML_V2_RELEASE_DIR unset)
        }
        let content = std::fs::read_to_string(&path).expect("read VehicleDefinitions.sysml");
        let root = sysml_v2_parser::parse(&content).expect("parse");
        let uri = Url::from_file_path(&path)
            .unwrap_or_else(|_| Url::parse("file:///VehicleDefinitions.sysml").unwrap());

        // Semantic tokens (using server's ast_semantic_ranges)
        let ranges = crate::semantic_tokens::ast_semantic_ranges(&root, &content);
        let target_dir = std::env::var_os("CARGO_TARGET_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join("target")
            });
        let _ = std::fs::create_dir_all(&target_dir);
        let tokens_path = target_dir.join("semantic_tokens_vehicle_definitions.txt");
        write_semantic_ranges_for_review(&content, &ranges, &tokens_path);

        // Symbol table (stubbed collect_symbol_entries returns empty)
        let entries = collect_symbol_entries(&root, &uri);
        let table_path = target_dir.join("symbol_table_vehicle_definitions.txt");
        write_symbol_table_for_review(&entries, &table_path);
    }

    #[cfg(test)]
    fn range_text_from_source(source: &str, r: &crate::semantic_tokens::SourceRange) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let line = match lines.get(r.start_line as usize) {
            Some(l) => l,
            None => return String::new(),
        };
        let start = r.start_character as usize;
        let end = r.end_character as usize;
        let n_chars = line.chars().count();
        if start >= n_chars || end > n_chars || start >= end {
            return String::new();
        }
        line.chars().skip(start).take(end - start).collect()
    }

    #[cfg(test)]
    fn write_semantic_ranges_for_review(
        source: &str,
        ranges: &[(crate::semantic_tokens::SourceRange, u32)],
        out_path: &std::path::Path,
    ) {
        use std::io::Write;
        if let Ok(mut f) = std::fs::File::create(out_path) {
            let _ = writeln!(
                f,
                "# Semantic token ranges (line/char 0-based, type index)\n"
            );
            for (r, type_index) in ranges {
                let text = range_text_from_source(source, r);
                let text_escaped = text.replace('\n', "\\n").replace('\r', "\\r");
                let _ = writeln!(
                    f,
                    "{}:{}..{}:{} type_index={} \"{}\"",
                    r.start_line,
                    r.start_character,
                    r.end_line,
                    r.end_character,
                    type_index,
                    text_escaped
                );
            }
        }
    }

    #[cfg(test)]
    fn write_symbol_table_for_review(entries: &[SymbolEntry], out_path: &std::path::Path) {
        use std::io::Write;
        if let Ok(mut f) = std::fs::File::create(out_path) {
            let _ = writeln!(
                f,
                "# Symbol table (name | kind | container | range | signature)\n"
            );
            for e in entries {
                let range_str = format!(
                    "{}:{}..{}:{}",
                    e.range.start.line,
                    e.range.start.character,
                    e.range.end.line,
                    e.range.end.character
                );
                let kind_str = format!("{:?}", e.kind);
                let container = e.container_name.as_deref().unwrap_or("-");
                let sig = e.signature.as_deref().unwrap_or("-");
                let _ = writeln!(
                    f,
                    "{} | {} | {} | {} | {}",
                    e.name, kind_str, container, range_str, sig
                );
            }
        }
    }
}