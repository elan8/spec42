//! Semantic tokenization for SysML: classifies tokens so editors can apply
//! semantic highlighting (keyword, string, number, comment, operator, variable, type,
//! namespace, class, interface, property, function).
//!
//! AST-driven ranges override lexer heuristics when a parse succeeds.

mod ast_ranges;
mod ast_util;
mod keywords;
mod lexer;
mod types;

pub use ast_ranges::ast_semantic_ranges;
pub use ast_util::{identification_name, refine_declaration_ranges, span_to_source_range, SourceRange};
pub use types::*;

use lexer::tokenize_line;

/// Flat LSP semantic token payload: five `u32` values per token
/// (deltaLine, deltaStart, length, tokenType, tokenModifiers).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SemanticTokensDto {
    pub data: Vec<u32>,
}

/// Legend token type names (indices 0..=11). Order must match the TYPE_* constants.
pub fn legend_token_types() -> &'static [&'static str] {
    &[
        "keyword",
        "string",
        "number",
        "comment",
        "operator",
        "variable",
        "type",
        "namespace",
        "class",
        "interface",
        "property",
        "function",
    ]
}

fn token_ast_type(
    line: u32,
    start_char: u32,
    length: u32,
    ast_ranges: &[(SourceRange, u32)],
) -> Option<(u32, usize, u32)> {
    let end_char = start_char + length;
    let mut best: Option<(u32, usize, u32)> = None;
    for (i, (r, token_type)) in ast_ranges.iter().enumerate() {
        if line >= r.start_line && line <= r.end_line {
            let range_start = if line == r.start_line {
                r.start_character
            } else {
                0
            };
            let range_end = if line == r.end_line {
                r.end_character
            } else {
                u32::MAX
            };
            if start_char >= range_start && end_char <= range_end {
                let span_len = range_end.saturating_sub(range_start);
                let replace = best.is_none_or(|(_, _, len)| span_len < len);
                if replace {
                    best = Some((*token_type, i, span_len));
                }
            }
        }
    }
    best
}

fn apply_ast_semantic_ranges(
    tokens: &mut [(u32, u32, u32, u32)],
    ast_ranges: &[(SourceRange, u32)],
    lines: &[&str],
    mut log_out: Option<&mut Vec<String>>,
) {
    if let Some(log) = log_out.as_mut() {
        if !ast_ranges.is_empty() {
            log.push(format!(
                "[SYSML semantic tokens] AST ranges ({} total, first 20):",
                ast_ranges.len()
            ));
            for (i, (r, ty)) in ast_ranges.iter().enumerate().take(20) {
                let text: String = lines
                    .get(r.start_line as usize)
                    .map(|l| {
                        l.chars()
                            .skip(r.start_character as usize)
                            .take((r.end_character.saturating_sub(r.start_character)) as usize)
                            .collect::<String>()
                    })
                    .unwrap_or_default()
                    .replace('\n', "\\n");
                log.push(format!(
                    "  #{} {}:{}..{} {} \"{}\"",
                    i,
                    r.start_line,
                    r.start_character,
                    r.end_character,
                    TYPE_NAMES[*ty as usize],
                    text
                ));
            }
            if ast_ranges.len() > 20 {
                log.push(format!("  ... and {} more", ast_ranges.len() - 20));
            }
        }
    }
    for (line, start, len, type_idx) in tokens.iter_mut() {
        let can_override = *type_idx == TYPE_VARIABLE
            || *type_idx == TYPE_NAMESPACE
            || *type_idx == TYPE_KEYWORD
            || *type_idx == TYPE_TYPE;
        if can_override {
            if let Some((ast_type, range_idx, span_len)) =
                token_ast_type(*line, *start, *len, ast_ranges)
            {
                if span_len > 2 * *len && ast_type != TYPE_NAMESPACE {
                    continue;
                }
                if *type_idx == TYPE_TYPE && ast_type == TYPE_PROPERTY {
                    continue;
                }
                if *type_idx == TYPE_KEYWORD && ast_type != TYPE_KEYWORD {
                    continue;
                }
                if let Some(log) = log_out.as_mut() {
                    let token_text: String = lines
                        .get(*line as usize)
                        .map(|l| {
                            l.chars()
                                .skip(*start as usize)
                                .take(*len as usize)
                                .collect::<String>()
                        })
                        .unwrap_or_default();
                    let (r, _) = &ast_ranges[range_idx];
                    log.push(format!(
                        "[SYSML semantic tokens] OVERRIDE token \"{}\" at {}:{} len {}: {} -> {} (matched AST range #{}: {}:{}..{} {})",
                        token_text.replace('\n', "\\n"),
                        line,
                        start,
                        len,
                        TYPE_NAMES[*type_idx as usize],
                        TYPE_NAMES[ast_type as usize],
                        range_idx,
                        r.start_line,
                        r.start_character,
                        r.end_character,
                        TYPE_NAMES[ast_type as usize]
                    ));
                }
                *type_idx = ast_type;
            }
        }
    }
}

fn to_utf16_units(lines: &[&str], line: u32, start_char: u32, length: u32) -> (u32, u32) {
    let line_str = lines.get(line as usize).unwrap_or(&"");
    let mut start_utf16 = 0u32;
    let mut len_utf16 = 0u32;
    for (i, c) in line_str.chars().enumerate() {
        let u16 = c.len_utf16() as u32;
        if i < start_char as usize {
            start_utf16 += u16;
        } else if i < (start_char + length) as usize {
            len_utf16 += u16;
        } else {
            break;
        }
    }
    (start_utf16, len_utf16)
}

fn encode_flat(tokens: &[(u32, u32, u32, u32)], lines: &[&str]) -> Vec<u32> {
    let mut data = Vec::with_capacity(tokens.len() * 5);
    let mut prev_line = 0u32;
    let mut prev_start_utf16 = 0u32;

    for &(line, start_char, length, token_type) in tokens {
        let (start_utf16, len_utf16) = to_utf16_units(lines, line, start_char, length);
        let delta_line = line - prev_line;
        let delta_start = if line == prev_line {
            start_utf16.saturating_sub(prev_start_utf16)
        } else {
            start_utf16
        };
        data.push(delta_line);
        data.push(delta_start);
        data.push(len_utf16);
        data.push(token_type);
        data.push(0);
        prev_line = line;
        prev_start_utf16 = start_utf16;
    }

    data
}

/// Produce semantic tokens for the full document.
pub fn semantic_tokens_full(
    text: &str,
    ast_ranges: Option<&[(SourceRange, u32)]>,
) -> (SemanticTokensDto, Vec<String>) {
    let lines: Vec<&str> = text.lines().collect();
    let mut all_tokens = Vec::new();
    let mut in_block_comment = false;
    for (line_index, line) in lines.iter().enumerate() {
        let (line_tokens, still_in) = tokenize_line(line, line_index as u32, in_block_comment);
        in_block_comment = still_in;
        all_tokens.extend(line_tokens);
    }
    let log_lines = Vec::new();
    if let Some(ranges) = ast_ranges {
        let refined = refine_declaration_ranges(text, ranges);
        apply_ast_semantic_ranges(&mut all_tokens, &refined, &lines, None);
    }
    (
        SemanticTokensDto {
            data: encode_flat(&all_tokens, &lines),
        },
        log_lines,
    )
}

fn block_comment_state_after_line(lines: &[&str], through_line: u32) -> bool {
    let mut in_block = false;
    for (line_index, line) in lines.iter().take(through_line as usize + 1).enumerate() {
        let (_, still_in) = tokenize_line(line, line_index as u32, in_block);
        in_block = still_in;
    }
    in_block
}

/// Produce semantic tokens overlapping the given range.
pub fn semantic_tokens_range(
    text: &str,
    start_line: u32,
    start_character: u32,
    end_line: u32,
    end_character: u32,
    ast_ranges: Option<&[(SourceRange, u32)]>,
) -> (SemanticTokensDto, Vec<String>) {
    let mut all_tokens = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let max_line = lines.len().saturating_sub(1) as u32;
    let line_end = end_line.min(max_line);

    let mut in_block_comment = if start_line > 0 {
        block_comment_state_after_line(&lines, start_line - 1)
    } else {
        false
    };

    for line_index in start_line..=line_end {
        let line = lines.get(line_index as usize).unwrap_or(&"");
        let (line_tokens, still_in) = tokenize_line(line, line_index, in_block_comment);
        in_block_comment = still_in;

        for (ln, start_char, length, token_type) in line_tokens {
            let token_end_char = start_char + length;
            let range_start_char = if ln == start_line { start_character } else { 0 };
            let range_end_char = if ln == end_line {
                end_character
            } else {
                u32::MAX
            };
            if token_end_char <= range_start_char || start_char >= range_end_char {
                continue;
            }
            all_tokens.push((ln, start_char, length, token_type));
        }
    }

    let log_lines = Vec::new();
    if let Some(ranges) = ast_ranges {
        let refined = refine_declaration_ranges(text, ranges);
        apply_ast_semantic_ranges(&mut all_tokens, &refined, &lines, None);
    }

    (
        SemanticTokensDto {
            data: encode_flat(&all_tokens, &lines),
        },
        log_lines,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf16_conversion_ascii() {
        let lines = &["\tin panAngle : Real;"];
        let (start, len) = to_utf16_units(lines, 0, 4, 8);
        assert_eq!((start, len), (4, 8), "panAngle at char 4, len 8");
    }

    #[test]
    fn test_utf16_conversion_non_bmp() {
        let lines = &["in pan\u{1F44D}gle : Real;"];
        let (start, len) = to_utf16_units(lines, 0, 4, 7);
        assert_eq!((start, len), (4, 8), "pan👋gle: p at 4, 7 chars = 8 UTF-16");
    }

    #[test]
    fn test_position_is_not_a_keyword() {
        let line = "\tout position : String;";
        let (tokens, _still_in_comment) = tokenize_line(line, 0, false);

        let mut saw_position = false;
        for (ln, start, len, ty) in tokens {
            assert_eq!(ln, 0);
            if &line[start as usize..(start + len) as usize] == "position" {
                saw_position = true;
                assert_eq!(ty, TYPE_VARIABLE);
            }
        }
        assert!(saw_position, "expected to tokenize 'position'");
    }

    #[test]
    fn flat_encoding_is_multiple_of_five() {
        let text = "package P { part def Robot; }";
        let (tokens, _) = semantic_tokens_full(text, None);
        assert!(!tokens.data.is_empty());
        assert_eq!(tokens.data.len() % 5, 0);
    }
}
