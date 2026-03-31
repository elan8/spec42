//! Semantic tokenization for SysML: classifies tokens so the editor can apply
//! semantic highlighting (keyword, string, number, comment, operator, variable, type, namespace,
//! class, interface, property, function).
//!
//! We use the AST when available: the parser provides (SourceRange, SemanticRole) for definition
//! names, type references, etc., and we override the lexer's token types for those spans. This
//! matches how other language servers (C#, TypeScript, Rust) drive semantic highlighting from
//! the AST. When the parse fails, we fall back to lexer-only heuristics (e.g. identifier after
//! `:` → type, identifier after `package` → namespace).

mod ast_ranges;
mod lexer;
mod types;

pub use ast_ranges::ast_semantic_ranges;

use crate::syntax::ast_util::SourceRange;
use tower_lsp::lsp_types::{
    SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensLegend,
};

use lexer::tokenize_line;
use types::*;

/// Legend: token types we emit (indices 0..=11). Order must match the TYPE_* constants.
pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            SemanticTokenType::KEYWORD,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::COMMENT,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::TYPE,
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::CLASS,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::FUNCTION,
        ],
        token_modifiers: vec![],
    }
}

/// Returns the AST semantic type for a token span if it is entirely inside one of the given ranges.
/// When multiple ranges match (e.g. a bogus large TYPE range and a correct PROPERTY range),
/// returns the one with the smallest span (most specific). Also returns that span's length
/// so the caller can skip overriding when the range is much larger than the token (parser span bug).
fn token_ast_type(
    line: u32,
    start_char: u32,
    length: u32,
    ast_ranges: &[(SourceRange, u32)],
) -> Option<(u32, usize, u32)> {
    let end_char = start_char + length;
    let mut best: Option<(u32, usize, u32)> = None; // (token_type, index, span_length)
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

/// Override token types using AST-derived (range, type) pairs. VARIABLE, NAMESPACE, and KEYWORD
/// tokens are overridden when the AST has a role for that span (e.g. Property, Class).
/// When log_out is Some, appends debug lines (AST ranges and each override) for LSP log_message.
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
        // Override lexer type when AST has a more specific role. Include TYPE so that property
        // names wrongly lexed as type (e.g. "current" / "velocity" after "out name :") get PROPERTY.
        let can_override = *type_idx == TYPE_VARIABLE
            || *type_idx == TYPE_NAMESPACE
            || *type_idx == TYPE_KEYWORD
            || *type_idx == TYPE_TYPE;
        if can_override {
            if let Some((ast_type, range_idx, span_len)) =
                token_ast_type(*line, *start, *len, ast_ranges)
            {
                // Skip override when the matching range is much larger than the token (parser
                // sometimes attaches a parent span to type_ref, so we'd wrongly override "in"/name).
                if span_len > 2 * *len {
                    continue;
                }
                // Allow VARIABLE -> TYPE when the AST range is tight (e.g. specializes_span,
                // type_ref_span); the span_len check above already skips wrong coarse spans.
                // Never override TYPE -> PROPERTY: PartUsage/PortUsage AST ranges use the whole
                // declaration span (e.g. "cmd : MotorCommandPort {") as PROPERTY, so the type name
                // is contained in that span and would be wrongly overridden. Keep TYPE for type
                // references (e.g. after ":" in port/part usages).
                if *type_idx == TYPE_TYPE && ast_type == TYPE_PROPERTY {
                    continue;
                }
                // Never override KEYWORD -> PROPERTY or KEYWORD -> TYPE: language keywords (e.g.
                // "attribute", "def") must stay KEYWORD; wrong AST spans sometimes cover them.
                // Can be removed/relaxed once parser semantic ranges no longer misattribute these.
                if *type_idx == TYPE_KEYWORD && (ast_type == TYPE_PROPERTY || ast_type == TYPE_TYPE)
                {
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

/// Convert character indices to UTF-16 code units (LSP spec requires UTF-16).
/// For ASCII/BMP, char count equals UTF-16; for non-BMP (e.g. emoji), 1 char = 2 UTF-16 units.
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

/// Encode tokens into LSP semantic tokens (delta encoding).
/// Positions are converted to UTF-16 code units per LSP spec.
fn encode(tokens: &[(u32, u32, u32, u32)], lines: &[&str]) -> Vec<SemanticToken> {
    let mut data = Vec::with_capacity(tokens.len());
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
        data.push(SemanticToken {
            delta_line,
            delta_start,
            length: len_utf16,
            token_type,
            token_modifiers_bitset: 0,
        });
        prev_line = line;
        prev_start_utf16 = start_utf16;
    }

    data
}

/// Produce semantic tokens for the full document.
/// When `ast_ranges` is Some (from a successful parse), those (range, token_type) pairs
/// override variable/namespace tokens for AST-driven highlighting; otherwise the lexer uses heuristics.
/// Returns (tokens, debug_log_lines). When ast_ranges is Some, debug_log_lines contains AST range and override details for LSP logging.
pub fn semantic_tokens_full(
    text: &str,
    ast_ranges: Option<&[(SourceRange, u32)]>,
) -> (SemanticTokens, Vec<String>) {
    let lines: Vec<&str> = text.lines().collect();
    let mut all_tokens = Vec::new();
    let mut in_block_comment = false;
    for (line_index, line) in lines.iter().enumerate() {
        let (line_tokens, still_in) = tokenize_line(line, line_index as u32, in_block_comment);
        in_block_comment = still_in;
        all_tokens.extend(line_tokens);
    }
    let mut log_lines = Vec::new();
    if let Some(ranges) = ast_ranges {
        apply_ast_semantic_ranges(&mut all_tokens, ranges, &lines, Some(&mut log_lines));
    }
    let tokens = SemanticTokens {
        result_id: None,
        data: encode(&all_tokens, &lines),
    };
    (tokens, log_lines)
}

/// Compute whether we're inside a block comment at the end of the given line index (0 = after first line).
fn block_comment_state_after_line(lines: &[&str], through_line: u32) -> bool {
    let mut in_block = false;
    for (line_index, line) in lines.iter().take(through_line as usize + 1).enumerate() {
        let (_, still_in) = tokenize_line(line, line_index as u32, in_block);
        in_block = still_in;
    }
    in_block
}

/// Produce semantic tokens for a range (for textDocument/semanticTokens/range).
/// Only tokens that overlap the given range are included.
/// When `ast_ranges` is Some, those (range, type) pairs override variable/namespace tokens.
/// Returns (tokens, debug_log_lines) like semantic_tokens_full.
pub fn semantic_tokens_range(
    text: &str,
    start_line: u32,
    start_character: u32,
    end_line: u32,
    end_character: u32,
    ast_ranges: Option<&[(SourceRange, u32)]>,
) -> (SemanticTokens, Vec<String>) {
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

    let mut log_lines = Vec::new();
    if let Some(ranges) = ast_ranges {
        apply_ast_semantic_ranges(&mut all_tokens, ranges, &lines, Some(&mut log_lines));
    }

    let tokens = SemanticTokens {
        result_id: None,
        data: encode(&all_tokens, &lines),
    };
    (tokens, log_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf16_conversion_ascii() {
        // ASCII: char count = UTF-16 count
        let lines = &["\tin panAngle : Real;"];
        let (start, len) = to_utf16_units(lines, 0, 4, 8);
        assert_eq!((start, len), (4, 8), "panAngle at char 4, len 8");
    }

    #[test]
    fn test_utf16_conversion_non_bmp() {
        // Non-BMP: 1 char = 2 UTF-16 units (e.g. emoji U+1F44D)
        let lines = &["in pan\u{1F44D}gle : Real;"];
        let (start, len) = to_utf16_units(lines, 0, 4, 7); // "pan👋gle" = 7 chars
        assert_eq!(
            (start, len),
            (4, 8),
            "pan👋gle: p at 4, 7 chars = 8 UTF-16 (👋=2)"
        );
    }

    #[test]
    fn test_position_is_not_a_keyword() {
        // Regression: "position" is a common attribute name (e.g. telemetry),
        // and is not a reserved keyword in SysML v2. It should not be colored as keyword
        // in the lexer-only fallback.
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
}
