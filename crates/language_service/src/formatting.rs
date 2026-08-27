/// Formatting options (protocol-neutral subset of LSP `FormattingOptions`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatOptions {
    pub tab_size: u32,
    pub insert_spaces: bool,
}

/// Formats a whole SysML/KerML document without changing its parsed meaning.
///
/// This deliberately remains a lightweight formatter: Spec42 keeps the pinned
/// parser as its only compiler frontend.  The lexical pass is nevertheless
/// aware of strings and both comment forms, so braces in those regions do not
/// affect layout. Editor recovery must never turn malformed input into a
/// different partial model, so a candidate layout is retained only when the
/// parser's recovered tree and diagnostic kinds remain unchanged.
pub fn format_document_text(source: &str, options: FormatOptions) -> String {
    let analysis = match analyze(source) {
        Some(analysis) => analysis,
        None => return source.to_string(),
    };

    if source.is_empty() {
        return "\n".to_string();
    }

    let indent_unit = if options.insert_spaces {
        " ".repeat(options.tab_size as usize)
    } else {
        "\t".to_string()
    };
    let mut depth = 0i32;
    let mut formatted_lines = Vec::with_capacity(analysis.len());

    for line in analysis {
        // Physical lines that begin within a multiline string or block comment
        // are payload, not layout.  Keep their whitespace byte-for-byte.
        if line.starts_in_protected_region {
            formatted_lines.push(line.text.to_string());
            depth += line.opens - line.closes;
            continue;
        }

        // A line that opens a multiline protected region owns everything after
        // the delimiter. Its trailing spaces are payload and must survive.
        let content = if line.ends_in_protected_region {
            line.text.trim_start()
        } else {
            line.text.trim()
        };
        if content.is_empty() {
            formatted_lines.push(String::new());
            continue;
        }

        let indent_depth = (depth - line.leading_closes).max(0);
        formatted_lines.push(format!(
            "{}{}",
            indent_unit.repeat(indent_depth as usize),
            content
        ));
        depth += line.opens - line.closes;
    }

    // Retain intentional separation, but
    // collapse runs of blank lines and do not leave whitespace-only EOF lines.
    let mut collapsed = Vec::with_capacity(formatted_lines.len());
    let mut previous_blank = false;
    for line in formatted_lines {
        let blank = line.is_empty();
        if blank && previous_blank {
            continue;
        }
        previous_blank = blank;
        collapsed.push(line);
    }
    while collapsed.last().is_some_and(|line| line.is_empty()) {
        collapsed.pop();
    }

    let candidate = if collapsed.is_empty() {
        "\n".to_string()
    } else {
        format!("{}\n", collapsed.join("\n"))
    };

    if preserves_parse_meaning(source, &candidate) {
        candidate
    } else {
        source.to_string()
    }
}

fn preserves_parse_meaning(source: &str, candidate: &str) -> bool {
    // The syntax service answers this: whether a reformat changes what the parser sees is a
    // question about the grammar, and answering it here would mean parsing the same text twice
    // against an AST this crate would have to keep in step with the pinned revision.
    let syntax = sysml_query::syntax::SyntaxService::new();
    syntax.reformatting_preserves_meaning(&syntax.parse_text(source), candidate)
}

#[derive(Debug)]
struct LineAnalysis<'a> {
    text: &'a str,
    starts_in_protected_region: bool,
    ends_in_protected_region: bool,
    opens: i32,
    closes: i32,
    leading_closes: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexState {
    Code,
    String { escaped: bool },
    UnrestrictedName { escaped: bool },
    BlockComment,
}

/// Returns `None` for incomplete lexical structure. This is a conservative
/// recovery boundary because an unfinished string, unrestricted name, or
/// comment can contain whitespace payload.
fn analyze(source: &str) -> Option<Vec<LineAnalysis<'_>>> {
    let mut state = LexState::Code;
    let mut lines = Vec::new();

    for raw_line in source.split('\n') {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let starts_in_protected_region = state != LexState::Code;
        let mut opens = 0;
        let mut closes = 0;
        let mut leading_closes = 0;
        let mut only_leading_closes = true;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            match state {
                LexState::Code => match ch {
                    '/' if chars.peek() == Some(&'/') => break,
                    '/' if chars.peek() == Some(&'*') => {
                        chars.next();
                        state = LexState::BlockComment;
                    }
                    '"' => state = LexState::String { escaped: false },
                    '\'' => state = LexState::UnrestrictedName { escaped: false },
                    '{' => {
                        opens += 1;
                        only_leading_closes = false;
                    }
                    '}' => {
                        closes += 1;
                        if only_leading_closes {
                            leading_closes += 1;
                        }
                    }
                    c if c.is_whitespace() => {}
                    _ => only_leading_closes = false,
                },
                LexState::String { escaped } => {
                    state = match (escaped, ch) {
                        (true, _) => LexState::String { escaped: false },
                        (false, '\\') => LexState::String { escaped: true },
                        (false, '"') => LexState::Code,
                        _ => LexState::String { escaped: false },
                    };
                }
                LexState::UnrestrictedName { escaped } => {
                    state = match (escaped, ch) {
                        (true, _) => LexState::UnrestrictedName { escaped: false },
                        (false, '\\') => LexState::UnrestrictedName { escaped: true },
                        (false, '\'') => LexState::Code,
                        _ => LexState::UnrestrictedName { escaped: false },
                    };
                }
                LexState::BlockComment => {
                    if ch == '*' && chars.peek() == Some(&'/') {
                        chars.next();
                        state = LexState::Code;
                    }
                }
            }
        }

        lines.push(LineAnalysis {
            text: line,
            starts_in_protected_region,
            ends_in_protected_region: state != LexState::Code,
            opens,
            closes,
            leading_closes,
        });
    }

    (state == LexState::Code).then_some(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_options() -> FormatOptions {
        FormatOptions {
            tab_size: 4,
            insert_spaces: true,
        }
    }

    #[test]
    fn format_document_empty() {
        assert_eq!(format_document_text("", default_options()), "\n");
    }

    #[test]
    fn format_document_trim_trailing_whitespace() {
        let source = "package P {   \n  part x;  \n}";
        let formatted = format_document_text(source, default_options());
        assert!(!formatted.contains("   \n"));
        assert!(formatted.ends_with('\n'));
    }

    #[test]
    fn format_document_indent_by_braces() {
        let source = "package P {\npart x;\n}";
        let formatted = format_document_text(source, default_options());
        assert!(formatted.contains("    part x;"));
    }

    #[test]
    fn format_document_is_idempotent() {
        let source = "package P {\n    part x;\n}\n";
        let once = format_document_text(source, default_options());
        let twice = format_document_text(&once, default_options());
        assert_eq!(once, twice);
    }

    #[test]
    fn format_document_nested_blocks() {
        let source = "package P {\npart a;\npart b {\nattr x;\n}\n}\n";
        let formatted = format_document_text(source, default_options());
        assert!(formatted.contains("    part a;"));
        assert!(formatted.contains("        attr x;"));
    }

    #[test]
    fn format_document_normalizes_trailing_newline() {
        let source = "package P { part x; }";
        let formatted = format_document_text(source, default_options());
        assert!(formatted.ends_with('\n'));
        assert_eq!(formatted.matches('\n').count(), 1);
    }

    #[test]
    fn format_document_ignores_braces_in_strings_and_comments() {
        let source = "package P {\nattr text = \"{ not a block }\"; // }\n/* { */\npart x;\n}";
        assert_eq!(
            format_document_text(source, default_options()),
            "package P {\n    attr text = \"{ not a block }\"; // }\n    /* { */\n    part x;\n}\n"
        );
    }

    #[test]
    fn format_document_ignores_syntax_inside_unrestricted_names() {
        let source = r"package P {
feature 'brace { } // /* and slash \\ then escaped \' quote';
part x;
}";
        assert_eq!(
            format_document_text(source, default_options()),
            r"package P {
    feature 'brace { } // /* and slash \\ then escaped \' quote';
    part x;
}
"
        );
    }

    #[test]
    fn format_document_preserves_multiline_string_and_comment_payload() {
        let source = "package P {\ndoc /* first\n  { payload }  \n*/\nattr text = \"first  \n  { payload }  \nlast\";\n}";
        let formatted = format_document_text(source, default_options());
        assert!(formatted.contains("attr text = \"first  \n"));
        assert!(formatted.contains("  { payload }  \n"));
        assert_eq!(
            formatted,
            format_document_text(&formatted, default_options())
        );
    }

    #[test]
    fn format_document_preserves_incomplete_source_verbatim() {
        let source = "package P {\n  attr text = \"unfinished";
        assert_eq!(format_document_text(source, default_options()), source);
    }

    #[test]
    fn format_document_preserves_unfinished_unrestricted_name_verbatim() {
        let source = "package P {\n  feature 'brace { // unfinished";
        assert_eq!(format_document_text(source, default_options()), source);
    }

    /// This fuzz-derived source used to parse strictly, and the test existed because reformatting
    /// it produced a different tree -- the negative case of `preserves_parse_meaning`'s `Ok` arm.
    ///
    /// The pinned parser rejects `in<f;` outright, so the input now travels the
    /// `recovery_equivalent` path instead: original and candidate fail identically, recover to the
    /// same normalized tree, and carry the same diagnostic signature, so reformatting is provably
    /// safe and is applied. That is the same guarantee
    /// `format_document_recovers_unbalanced_blocks_without_changing_tokens` pins.
    ///
    /// No known *valid* source now trips the strict-parse negative branch, so that branch has no
    /// fixture; the branch itself is still implemented and still correct. Recorded in
    /// planning/UPSTREAM_PARSER_GAPS.md rather than left as a silently dead assertion.
    #[test]
    fn format_document_reformats_a_recovery_equivalent_source() {
        let source = "package ion {\n  class A {\n    in<f;\n  }\n\n  class A { in #su f;\n  }\n}";
        assert!(
            !sysml_query::syntax::SyntaxService::new()
                .parse_text(source)
                .is_clean(),
            "the parser accepted this again; restore the strict-parse arm of this test"
        );
        assert_eq!(
            format_document_text(source, default_options()),
            "package ion {\n    class A {\n        in<f;\n    }\n\n    class A { in #su f;\n    }\n}\n"
        );
    }

    #[test]
    fn format_document_recovers_unbalanced_blocks_without_changing_tokens() {
        let source = "package P {\npart x;\n";
        let formatted = format_document_text(source, default_options());
        assert_eq!(formatted, "package P {\n    part x;\n");
        assert_eq!(
            formatted,
            format_document_text(&formatted, default_options())
        );
    }

    #[test]
    fn format_document_collapses_blank_line_runs() {
        let source = "package P {\n\n\npart x;\n\n\n}";
        assert_eq!(
            format_document_text(source, default_options()),
            "package P {\n\n    part x;\n\n}\n"
        );
    }

    #[test]
    fn format_document_honors_tabs() {
        let source = "package P {\npart x;\n}";
        assert_eq!(
            format_document_text(
                source,
                FormatOptions {
                    tab_size: 8,
                    insert_spaces: false,
                },
            ),
            "package P {\n\tpart x;\n}\n"
        );
    }
}
