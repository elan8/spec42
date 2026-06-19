//! Position and word resolution for editor services (line/character to byte offset, word at cursor, etc.).

/// Converts an LSP-style (line, character) position to a byte offset in `text`.
/// Positions are expressed in UTF-16 code units, so this helper only returns offsets that
/// land on valid UTF-8 boundaries.
pub fn position_to_byte_offset(source: &str, line: u32, character: u32) -> Option<usize> {
    let lines: Vec<&str> = source.split('\n').collect();
    let line_str = *lines.get(line as usize)?;
    let target_utf16 = character;
    let mut seen_utf16 = 0u32;
    let mut byte_in_line = line_str.len();

    for (byte_idx, ch) in line_str.char_indices() {
        if seen_utf16 == target_utf16 {
            byte_in_line = byte_idx;
            break;
        }
        seen_utf16 += ch.len_utf16() as u32;
        if seen_utf16 > target_utf16 {
            return None;
        }
    }
    let line_utf16_len = line_str.encode_utf16().count() as u32;
    if seen_utf16 != target_utf16 && target_utf16 != line_utf16_len {
        return None;
    }

    let line_start = lines
        .iter()
        .take(line as usize)
        .map(|l| l.len() + 1)
        .sum::<usize>();
    Some(line_start + byte_in_line)
}
/// Returns the (line, start_char, end_char) and the word at the given position.
/// A word is a contiguous run of identifier characters (alphanumeric, underscore, or `:` for qualified names).
pub fn word_at_position(text: &str, line: u32, character: u32) -> Option<(u32, u32, u32, String)> {
    fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == ':' || c == '>'
    }
    let line_str = text.lines().nth(line as usize)?;
    let char_in_line = character as usize;
    let line_chars: Vec<char> = line_str.chars().collect();
    if line_chars.is_empty() || char_in_line > line_chars.len() {
        return None;
    }
    let mut start = char_in_line;
    while start > 0 && is_ident_char(line_chars[start - 1]) {
        start -= 1;
    }
    let mut end = char_in_line;
    while end < line_chars.len() && is_ident_char(line_chars[end]) {
        end += 1;
    }
    if start >= end {
        return None;
    }
    let word: String = line_chars[start..end].iter().collect();
    Some((line, start as u32, end as u32, word))
}

/// Unit expression inside a value suffix `[...]` when the cursor is within the brackets and
/// a numeric literal immediately precedes `[` on the same line (e.g. `10 [kV]`).
pub fn unit_value_suffix_at_position(text: &str, line: u32, character: u32) -> Option<String> {
    let line_str = text.lines().nth(line as usize)?;
    let chars: Vec<char> = line_str.chars().collect();
    let pos = character as usize;
    if pos > chars.len() {
        return None;
    }

    let mut best: Option<(usize, usize)> = None;
    let mut stack = Vec::new();
    for (i, &c) in chars.iter().enumerate() {
        if c == '[' {
            stack.push(i);
        } else if c == ']' {
            if let Some(open) = stack.pop() {
                if pos >= open && pos <= i && is_likely_unit_suffix_before_bracket(&chars, open) {
                    match best {
                        None => best = Some((open, i)),
                        Some((best_open, _)) if open > best_open => best = Some((open, i)),
                        _ => {}
                    }
                }
            }
        }
    }

    let (open, close) = best?;
    let inner_start = open + 1;
    let inner_end = close;
    if inner_start >= inner_end {
        return None;
    }
    let inner_text: String = chars[inner_start..inner_end].iter().collect();
    let inner_text = inner_text.trim();
    if inner_text.is_empty() {
        return None;
    }
    Some(inner_text.to_string())
}

fn is_likely_unit_suffix_before_bracket(chars: &[char], open_idx: usize) -> bool {
    let before: String = chars[..open_idx].iter().collect();
    let before = before.trim_end();
    let Some(last_token) = before.split_whitespace().last() else {
        return false;
    };
    let mut chars = last_token.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_digit() || ((first == '+' || first == '-') && last_token.len() > 1)) {
        return false;
    }
    last_token
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | 'E' | '+' | '-'))
}

/// Returns the text of the line up to (but not including) the given (line, character).
pub fn line_prefix_at_position(text: &str, line: u32, character: u32) -> String {
    let line_str = match text.lines().nth(line as usize) {
        Some(l) => l,
        None => return String::new(),
    };
    line_str.chars().take(character as usize).collect()
}

/// Returns the last token (identifier or keyword prefix) before the cursor for completion.
pub fn completion_prefix(line_prefix: &str) -> &str {
    fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == ':' || c == '>'
    }
    let trimmed = line_prefix.trim_end();
    let chars: Vec<char> = trimmed.chars().collect();
    if chars.is_empty() {
        return trimmed;
    }
    let mut n_trailing = 0;
    for c in chars.iter().rev() {
        if is_ident_char(*c) {
            n_trailing += 1;
        } else {
            break;
        }
    }
    let start_char_idx = chars.len().saturating_sub(n_trailing);
    let byte_start = trimmed
        .char_indices()
        .nth(start_char_idx)
        .map(|(o, _)| o)
        .unwrap_or(trimmed.len());
    trimmed.get(byte_start..).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_byte_offset() {
        let text = "abc\ndef\nghi";
        assert_eq!(position_to_byte_offset(text, 0, 0), Some(0));
        assert_eq!(position_to_byte_offset(text, 0, 2), Some(2));
        assert_eq!(position_to_byte_offset(text, 1, 0), Some(4));
    }

    #[test]
    fn test_word_at_position() {
        let text = "  part foo : Bar  ";
        let (_, _, _, word) = word_at_position(text, 0, 5).unwrap();
        assert_eq!(word, "part");
    }
}
