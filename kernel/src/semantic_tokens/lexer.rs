//! Lexer-based tokenization for semantic highlighting fallback (when parse fails).

use super::types::*;

fn is_keyword(w: &str) -> bool {
    crate::language::is_reserved_keyword(w)
}

/// Token: (line, start_char, length, type_index).
/// Returns (tokens, still_inside_block_comment).
pub fn tokenize_line(
    line: &str,
    line_index: u32,
    in_block_comment: bool,
) -> (Vec<(u32, u32, u32, u32)>, bool) {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let mut still_in_block_comment = in_block_comment;
    let mut already_continued_block = false;
    let mut last_was_colon = false;
    let mut expect_package_name = false;

    while i < n {
        // If we're inside a block comment (from a previous line), look for */ or treat rest of line as comment (once per line)
        if !already_continued_block && (in_block_comment || still_in_block_comment) {
            already_continued_block = true;
            let start = i;
            while i + 1 < n {
                if chars[i] == '*' && chars[i + 1] == '/' {
                    i += 2;
                    still_in_block_comment = false;
                    tokens.push((line_index, start as u32, (i - start) as u32, TYPE_COMMENT));
                    break;
                }
                i += 1;
            }
            if still_in_block_comment {
                tokens.push((line_index, start as u32, (n - start) as u32, TYPE_COMMENT));
                i = n;
            }
            continue;
        }

        // Skip whitespace
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Line comment (not block)
        if i + 1 < n
            && chars[i] == '/'
            && chars[i + 1] == '/'
            && (i + 2 >= n || chars[i + 2] != '*')
        {
            let start = i;
            while i < n {
                i += 1;
            }
            last_was_colon = false;
            tokens.push((line_index, start as u32, (i - start) as u32, TYPE_COMMENT));
            continue;
        }

        // Block comment /* ... */ or //* ... */
        if chars[i] == '/' && i + 1 < n {
            if chars[i + 1] == '*' {
                let start = i;
                i += 2;
                let mut found_closing = false;
                while i + 1 < n {
                    if chars[i] == '*' && chars[i + 1] == '/' {
                        i += 2;
                        found_closing = true;
                        break;
                    }
                    i += 1;
                }
                if !found_closing {
                    still_in_block_comment = true;
                }
                last_was_colon = false;
                tokens.push((line_index, start as u32, (i - start) as u32, TYPE_COMMENT));
                continue;
            }
            if i + 2 < n && chars[i + 1] == '/' && chars[i + 2] == '*' {
                let start = i;
                i += 3;
                let mut found_closing = false;
                while i + 1 < n {
                    if chars[i] == '*' && chars[i + 1] == '/' {
                        i += 2;
                        found_closing = true;
                        break;
                    }
                    i += 1;
                }
                if !found_closing {
                    still_in_block_comment = true;
                }
                last_was_colon = false;
                tokens.push((line_index, start as u32, (i - start) as u32, TYPE_COMMENT));
                continue;
            }
        }

        // Single-quoted string
        if chars[i] == '\'' {
            let start = i;
            i += 1;
            while i < n && chars[i] != '\'' {
                if chars[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            if i < n {
                i += 1;
            }
            last_was_colon = false;
            tokens.push((line_index, start as u32, (i - start) as u32, TYPE_STRING));
            continue;
        }

        // Double-quoted string
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < n && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            if i < n {
                i += 1;
            }
            last_was_colon = false;
            tokens.push((line_index, start as u32, (i - start) as u32, TYPE_STRING));
            continue;
        }

        // Metadata @name
        if chars[i] == '@' {
            let start = i;
            i += 1;
            while i < n
                && (chars[i].is_alphanumeric()
                    || chars[i] == '_'
                    || (chars[i] == ':' && i + 1 < n && chars[i + 1] == ':'))
            {
                if chars[i] == ':' && i + 1 < n && chars[i + 1] == ':' {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            last_was_colon = false;
            tokens.push((line_index, start as u32, (i - start) as u32, TYPE_VARIABLE));
            continue;
        }

        // Operators :> :>> :: .. ->
        if chars[i] == ':' && i + 1 < n && chars[i + 1] == '>' {
            let start = i;
            i += 2;
            if i < n && chars[i] == '>' {
                i += 1; // :>>
            }
            last_was_colon = false;
            tokens.push((line_index, start as u32, (i - start) as u32, TYPE_OPERATOR));
            continue;
        }
        if chars[i] == ':' && i + 1 < n && chars[i + 1] == ':' {
            last_was_colon = false;
            tokens.push((line_index, i as u32, 2, TYPE_OPERATOR));
            i += 2;
            continue;
        }
        if chars[i] == '.' && i + 1 < n && chars[i + 1] == '.' {
            last_was_colon = false;
            tokens.push((line_index, i as u32, 2, TYPE_OPERATOR));
            i += 2;
            continue;
        }
        if chars[i] == '-' && i + 1 < n && chars[i + 1] == '>' {
            last_was_colon = false;
            tokens.push((line_index, i as u32, 2, TYPE_OPERATOR));
            i += 2;
            continue;
        }

        // Numbers: integer or float
        if chars[i].is_ascii_digit()
            || (chars[i] == '-' && i + 1 < n && chars[i + 1].is_ascii_digit())
        {
            let start = i;
            if chars[i] == '-' {
                i += 1;
            }
            while i < n && chars[i].is_ascii_digit() {
                i += 1;
            }
            if i < n && chars[i] == '.' && i + 1 < n && chars[i + 1].is_ascii_digit() {
                i += 1;
                while i < n && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            if i < n && (chars[i] == 'e' || chars[i] == 'E') {
                i += 1;
                if i < n && (chars[i] == '+' || chars[i] == '-') {
                    i += 1;
                }
                while i < n && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            last_was_colon = false;
            tokens.push((line_index, start as u32, (i - start) as u32, TYPE_NUMBER));
            continue;
        }

        // Identifier or keyword: letters/numbers/underscore/hyphen (word boundary)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let len = (i - start) as u32;
            let token_type = if is_keyword(&word) {
                last_was_colon = false;
                if word == "package" {
                    expect_package_name = true;
                }
                TYPE_KEYWORD
            } else if expect_package_name {
                expect_package_name = false;
                TYPE_NAMESPACE
            } else if last_was_colon {
                last_was_colon = false;
                TYPE_TYPE
            } else {
                TYPE_VARIABLE
            };
            tokens.push((line_index, start as u32, len, token_type));
            continue;
        }

        // Single-char operator or other: skip one char so we don't loop forever
        if chars[i] == ';'
            || chars[i] == ','
            || chars[i] == '('
            || chars[i] == ')'
            || chars[i] == '{'
            || chars[i] == '}'
            || chars[i] == '['
            || chars[i] == ']'
            || chars[i] == '='
            || chars[i] == '.'
            || chars[i] == ':'
            || chars[i] == '>'
            || chars[i] == '-'
        {
            let start = i;
            last_was_colon = chars[i] == ':';
            i += 1;
            tokens.push((line_index, start as u32, (i - start) as u32, TYPE_OPERATOR));
            continue;
        }

        i += 1;
    }

    (tokens, still_in_block_comment)
}
