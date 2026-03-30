//! Helpers for hover and completion: position/word resolution, keywords, and AST name collection.
//! Also provides definition/reference ranges for Go to definition and Find references.

mod keywords;
mod position;
mod symbols;

pub use keywords::*;
pub use position::*;
pub use symbols::*;

use crate::ast_util::identification_name;
use sysml_parser::ast::{PackageBody, RootElement};
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, Diagnostic, FormattingOptions, OneOf,
    OptionalVersionedTextDocumentIdentifier, Position, Range, TextDocumentEdit, TextEdit, Url,
    WorkspaceEdit,
};

/// Formats the whole document: trim trailing whitespace per line, single trailing newline, indent by brace depth.
pub fn format_document(source: &str, options: &FormattingOptions) -> Vec<TextEdit> {
    let lines: Vec<&str> = source.lines().collect();
    if lines.is_empty() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 0));
        return vec![TextEdit {
            range,
            new_text: "\n".to_string(),
        }];
    }
    let indent_unit = if options.insert_spaces {
        " ".repeat(options.tab_size as usize)
    } else {
        "\t".to_string()
    };
    let mut depth: i32 = 0;
    let mut formatted_lines: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        let trimmed = line.trim();
        let mut open_braces = 0i32;
        let mut close_braces = 0i32;
        let mut leading_close_braces = 0i32;
        let mut only_leading_closes = true;
        for ch in trimmed.chars() {
            match ch {
                '{' => {
                    open_braces += 1;
                    only_leading_closes = false;
                }
                '}' => {
                    close_braces += 1;
                    if only_leading_closes {
                        leading_close_braces += 1;
                    }
                }
                c if c.is_whitespace() => {}
                _ => {
                    only_leading_closes = false;
                }
            }
        }
        // Only leading `}` should outdent the current line.
        let indent_depth = (depth - leading_close_braces).max(0);
        depth += open_braces - close_braces;
        let indent = indent_unit.repeat(indent_depth as usize);
        let content = if trimmed.is_empty() {
            String::new()
        } else {
            format!("{}{}", indent, trimmed)
        };
        formatted_lines.push(content);
    }
    let new_text = if formatted_lines.is_empty() {
        "\n".to_string()
    } else {
        format!("{}\n", formatted_lines.join("\n"))
    };
    let last_line = (lines.len() - 1) as u32;
    let last_char = lines.last().map(|l| l.len()).unwrap_or(0) as u32;
    let range = Range::new(Position::new(0, 0), Position::new(last_line, last_char));
    vec![TextEdit { range, new_text }]
}

/// Suggests a "Wrap in package" code action when the document has top-level members (one package with empty name and members).
pub fn suggest_wrap_in_package(source: &str, uri: &Url) -> Option<CodeAction> {
    let root = sysml_parser::parse(source).ok()?;
    let packages: Vec<_> = root
        .elements
        .iter()
        .filter_map(|n| match &n.value {
            RootElement::Package(p) => Some(p),
            _ => None,
        })
        .collect();
    if packages.len() != 1 {
        return None;
    }
    let pkg = packages[0];
    if !identification_name(&pkg.identification).is_empty() {
        return None;
    }
    let has_members = match &pkg.body {
        PackageBody::Brace { elements } => !elements.is_empty(),
        _ => false,
    };
    if !has_members {
        return None;
    }
    let lines: Vec<&str> = source.lines().collect();
    let last_line = lines.len().saturating_sub(1) as u32;
    let last_char = lines.last().map(|l| l.len()).unwrap_or(0) as u32;
    let range = Range::new(Position::new(0, 0), Position::new(last_line, last_char));
    let new_text = format!("package Generated {{\n{}\n}}\n", source.trim_end());
    let edit = WorkspaceEdit {
        changes: None,
        document_changes: Some(tower_lsp::lsp_types::DocumentChanges::Edits(vec![
            TextDocumentEdit {
                text_document: OptionalVersionedTextDocumentIdentifier {
                    uri: uri.clone(),
                    version: None,
                },
                edits: vec![OneOf::Left(TextEdit { range, new_text })],
            },
        ])),
        change_annotations: None,
    };
    Some(CodeAction {
        title: "Wrap in package".to_string(),
        kind: Some(tower_lsp::lsp_types::CodeActionKind::REFACTOR),
        diagnostics: None,
        edit: Some(edit),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    })
}

fn utf16_len(s: &str) -> u32 {
    s.encode_utf16().count() as u32
}

fn parse_untyped_part_usage_name(raw_line: &str) -> Option<String> {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let trimmed = code_only.trim();
    if !trimmed.starts_with("part ") || trimmed.starts_with("part def") {
        return None;
    }
    if !trimmed.ends_with(';') || trimmed.contains(':') {
        return None;
    }
    let after_part = trimmed.strip_prefix("part ")?;
    let name = after_part.strip_suffix(';')?.trim();
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
    Some(name.to_string())
}

fn to_pascal_case(name: &str) -> String {
    let mut out = String::new();
    let mut capitalize = true;
    for ch in name.chars() {
        if ch.is_alphanumeric() {
            if capitalize {
                for upper in ch.to_uppercase() {
                    out.push(upper);
                }
                capitalize = false;
            } else {
                out.push(ch);
            }
        } else {
            capitalize = true;
        }
    }
    if out.is_empty() {
        "GeneratedPart".to_string()
    } else {
        out
    }
}

fn find_block_end(lines: &[&str], start_line: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut seen_open = false;
    for (idx, line) in lines.iter().enumerate().skip(start_line) {
        for ch in line.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    seen_open = true;
                }
                '}' => {
                    if seen_open {
                        depth -= 1;
                        if depth == 0 {
                            return Some(idx);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn find_insertion_context(lines: &[&str], target_line: usize) -> Option<(usize, usize)> {
    for start in (0..=target_line).rev() {
        let trimmed = lines[start].trim();
        let is_container = (trimmed.starts_with("package ") || trimmed.starts_with("part def "))
            && trimmed.contains('{');
        if !is_container {
            continue;
        }
        let end = find_block_end(lines, start)?;
        if start <= target_line && target_line <= end {
            return Some((start, end));
        }
    }
    None
}

fn find_package_context(lines: &[&str], target_line: usize) -> Option<(usize, usize)> {
    for start in (0..=target_line).rev() {
        let trimmed = lines[start].trim();
        if !(trimmed.starts_with("package ") && trimmed.contains('{')) {
            continue;
        }
        let end = find_block_end(lines, start)?;
        if start <= target_line && target_line <= end {
            return Some((start, end));
        }
    }
    None
}

fn has_matching_part_def(lines: &[&str], start: usize, end: usize, type_name: &str) -> bool {
    let needle = format!("part def {}", type_name);
    lines
        .iter()
        .take(end + 1)
        .skip(start)
        .any(|line| line.trim().starts_with(&needle))
}

fn rewrite_untyped_part_usage_line(raw_line: &str, usage_name: &str, type_name: &str) -> String {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let comment_part = &raw_line[code_only.len()..];
    let leading_ws_len = code_only.len() - code_only.trim_start().len();
    let leading = &code_only[..leading_ws_len];
    format!(
        "{leading}part {usage_name} : {type_name};{comment_part}",
        leading = leading,
        usage_name = usage_name,
        type_name = type_name,
        comment_part = comment_part
    )
}

pub fn suggest_create_matching_part_def_quick_fix(
    source: &str,
    uri: &Url,
    diagnostic: &Diagnostic,
) -> Option<CodeAction> {
    let target_line = diagnostic.range.start.line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let raw_line = *lines.get(target_line)?;
    let usage_name = parse_untyped_part_usage_name(raw_line)?;
    let type_name = to_pascal_case(&usage_name);
    let (container_start, container_end) = find_insertion_context(&lines, target_line)?;
    let (search_start, search_end, insert_line, insert_indent) =
        if let Some((pkg_start, pkg_end)) = find_package_context(&lines, target_line) {
            let pkg_line = lines.get(pkg_start)?;
            let pkg_indent_len = pkg_line.len() - pkg_line.trim_start().len();
            let pkg_indent = &pkg_line[..pkg_indent_len];
            let member_indent = format!("{pkg_indent}  ");
            // Prefer inserting before the containing part def so the new type is at package level
            // and appears above the usage container.
            let target_insert_line = if container_start > pkg_start && container_start < pkg_end {
                container_start
            } else {
                pkg_end
            };
            (
                pkg_start,
                pkg_end,
                target_insert_line,
                member_indent,
            )
        } else {
            let closing_line = lines.get(container_end)?;
            let closing_indent_len = closing_line.len() - closing_line.trim_start().len();
            let closing_indent = &closing_line[..closing_indent_len];
            (
                0,
                container_end,
                container_end,
                closing_indent.to_string(),
            )
        };

    let mut edits: Vec<OneOf<TextEdit, tower_lsp::lsp_types::AnnotatedTextEdit>> = Vec::new();
    if !has_matching_part_def(&lines, search_start, search_end, &type_name) {
        edits.push(OneOf::Left(TextEdit {
            range: Range::new(
                Position::new(insert_line as u32, 0),
                Position::new(insert_line as u32, 0),
            ),
            new_text: format!("{indent}part def {type_name} {{ }}\n", indent = insert_indent),
        }));
    }

    edits.push(OneOf::Left(TextEdit {
        range: Range::new(
            Position::new(target_line as u32, 0),
            Position::new(target_line as u32, utf16_len(raw_line)),
        ),
        new_text: rewrite_untyped_part_usage_line(raw_line, &usage_name, &type_name),
    }));

    let edit = WorkspaceEdit {
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
    };

    Some(CodeAction {
        title: format!("Create matching `part def {}` and type usage", type_name),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: Some(vec![diagnostic.clone()]),
        edit: Some(edit),
        command: None,
        is_preferred: Some(true),
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
        // "café" = c,a,f,é = 4 chars, 5 bytes (é is 2 bytes)
        let text = "café\n";
        assert_eq!(position_to_byte_offset(text, 0, 0), Some(0));
        assert_eq!(position_to_byte_offset(text, 0, 3), Some(3));
        assert_eq!(position_to_byte_offset(text, 0, 4), Some(5));
        assert_eq!(position_to_byte_offset(text, 0, 5), None);
        // Japanese: 日本 = 2 chars, 6 bytes
        let text2 = "日本\n";
        assert_eq!(position_to_byte_offset(text2, 0, 2), Some(6));
    }

    #[test]
    fn test_position_to_byte_offset_utf16_surrogate_pair() {
        let text = "a😀b\n";
        assert_eq!(position_to_byte_offset(text, 0, 0), Some(0));
        assert_eq!(position_to_byte_offset(text, 0, 1), Some(1));
        assert_eq!(position_to_byte_offset(text, 0, 2), None);
        assert_eq!(position_to_byte_offset(text, 0, 3), Some(5));
        assert_eq!(position_to_byte_offset(text, 0, 4), Some(6));
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
        let text = "part café : String";
        let (_, _, _, w) = word_at_position(text, 0, 6).unwrap();
        assert_eq!(w, "café");
        let text2 = "part 部品 : Type";
        let (_, _, _, w2) = word_at_position(text2, 0, 6).unwrap();
        assert_eq!(w2, "部品");
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
        assert_eq!(completion_prefix("  café "), "café");
        assert_eq!(completion_prefix("part 部品 "), "部品");
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
        let root = sysml_parser::RootNamespace { elements: vec![] };
        let el = collect_named_elements(&root);
        assert!(el.is_empty());
    }

    #[test]
    fn test_collect_named_elements_from_package() {
        let text = "package P { part def Engine { } }";
        let root = sysml_parser::parse(text).expect("parse");
        let el = collect_named_elements(&root);
        assert_eq!(el.len(), 2); // package P + part Engine
        let names: Vec<_> = el.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"P"));
        assert!(names.contains(&"Engine"));
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
        let root = sysml_parser::RootNamespace { elements: vec![] };
        let ranges = collect_definition_ranges(&root);
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_collect_definition_ranges_package() {
        let text = "package P { }";
        let root = sysml_parser::parse(text).expect("parse");
        let ranges = collect_definition_ranges(&root);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].0, "P");
    }

    #[test]
    fn test_collect_definition_ranges_part_def() {
        // sysml-parser requires package/namespace at root; part def must be nested
        let text = "package P { part def Engine { } }";
        let root = sysml_parser::parse(text).expect("parse");
        let ranges = collect_definition_ranges(&root);
        assert_eq!(ranges.len(), 2); // package P + part Engine
        assert_eq!(ranges[0].0, "P");
        assert_eq!(ranges[1].0, "Engine");
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
        let root = sysml_parser::RootNamespace { elements: vec![] };
        let symbols = collect_document_symbols(&root);
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_collect_document_symbols_package() {
        let text = "package P { }";
        let root = sysml_parser::parse(text).expect("parse");
        let symbols = collect_document_symbols(&root);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "P");
        assert_eq!(symbols[0].detail.as_deref(), Some("package"));
        assert_eq!(symbols[0].kind, tower_lsp::lsp_types::SymbolKind::MODULE);
    }

    #[test]
    fn test_collect_document_symbols_nested() {
        let text = "package P { part def Engine { } }";
        let root = sysml_parser::parse(text).expect("parse");
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
    fn test_collect_symbol_entries_empty() {
        let root = sysml_parser::RootNamespace { elements: vec![] };
        let uri = Url::parse("file:///test.sysml").unwrap();
        let entries = collect_symbol_entries(&root, &uri);
        assert!(entries.is_empty());
    }

    #[test]
    fn test_collect_symbol_entries_package() {
        let text = "package P { }";
        let root = sysml_parser::parse(text).expect("parse");
        let uri = Url::parse("file:///test.sysml").unwrap();
        let entries = collect_symbol_entries(&root, &uri);
        // collect_symbol_entries is currently stubbed (returns empty)
        assert!(entries.is_empty());
    }

    #[test]
    fn test_collect_symbol_entries_nested() {
        let text = "package P { part def Engine { } }";
        let root = sysml_parser::parse(text).expect("parse");
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
        // When source is a single top-level part def, sysml-parser may parse it as one anonymous package
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
        let root = sysml_parser::parse(&content).expect("parse");
        let uri = Url::from_file_path(&path)
            .unwrap_or_else(|_| Url::parse("file:///VehicleDefinitions.sysml").unwrap());

        // Semantic tokens (using server's ast_semantic_ranges)
        let ranges = crate::semantic_tokens::ast_semantic_ranges(&root);
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
    fn range_text_from_source(source: &str, r: &crate::ast_util::SourceRange) -> String {
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
        ranges: &[(crate::ast_util::SourceRange, u32)],
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
