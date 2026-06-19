/// Formatting options (protocol-neutral subset of LSP `FormattingOptions`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatOptions {
    pub tab_size: u32,
    pub insert_spaces: bool,
}

/// Formats a whole document: trim trailing whitespace per line, single trailing newline, indent by brace depth.
pub fn format_document_text(source: &str, options: FormatOptions) -> String {
    let lines: Vec<&str> = source.lines().collect();
    if lines.is_empty() {
        return "\n".to_string();
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
        for ch in code_chars_before_comment(trimmed) {
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
    while formatted_lines.last().is_some_and(|line| line.is_empty()) {
        formatted_lines.pop();
    }
    if formatted_lines.is_empty() {
        "\n".to_string()
    } else {
        format!("{}\n", formatted_lines.join("\n"))
    }
}

fn code_chars_before_comment(line: &str) -> Vec<char> {
    let mut chars = Vec::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut iter = line.chars().peekable();
    while let Some(ch) = iter.next() {
        if escaped {
            chars.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' && in_string {
            chars.push(ch);
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            chars.push(ch);
            continue;
        }
        if !in_string && ch == '/' && iter.peek() == Some(&'/') {
            break;
        }
        chars.push(ch);
    }
    chars
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
}
