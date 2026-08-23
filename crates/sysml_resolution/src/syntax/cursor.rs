//! What is under the cursor, and where a name occurs.
//!
//! These are lexical answers, and they are here because they are lexical answers *about SysML*:
//! which characters continue an identifier, that `::` is part of a qualified name, that `10 [kg]`
//! carries a unit, that a `//` starts a comment. Every consumer that answered them for itself had
//! a slightly different rule, which is how a rename came to match inside comments and a hover came
//! to trigger on a word inside a string.
//!
//! The pinned grammar exposes no lexer, so the scan is here, behind the authority, with one
//! vocabulary — not in the hosts.

use super::{SyntaxRange, SyntaxRole};

/// The token under a cursor: what it spells, where it sits, and what the grammar calls it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxToken {
    /// The token's extent, always within one line.
    pub range: SyntaxRange,
    /// The token as authored, qualified-name separators included.
    pub text: String,
    /// The role the grammar gives the span, when the parser classified one there.
    pub role: Option<SyntaxRole>,
    /// Whether the token spells a reserved keyword.
    pub is_keyword: bool,
}

impl SyntaxToken {
    /// The last segment of a qualified name — what a lookup by simple name asks for.
    pub fn simple_name(&self) -> &str {
        self.text.rsplit("::").next().unwrap_or(&self.text)
    }
}

/// A value-with-unit literal such as `10 [kg]`: the unit expression, and where it is written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxUnitLiteral {
    /// The unit expression inside the brackets, trimmed.
    pub unit: String,
    /// The range of the unit expression itself, brackets excluded.
    pub range: SyntaxRange,
}

fn continues_identifier(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == ':' || ch == '>'
}

pub(super) fn token_at(
    source: &str,
    roles: &[(SyntaxRange, SyntaxRole)],
    line: u32,
    character: u32,
) -> Option<SyntaxToken> {
    let line_text = source.lines().nth(line as usize)?;
    let chars: Vec<char> = line_text.chars().collect();
    let cursor = character as usize;
    if chars.is_empty() || cursor > chars.len() {
        return None;
    }
    let mut start = cursor;
    while start > 0 && continues_identifier(chars[start - 1]) {
        start -= 1;
    }
    let mut end = cursor;
    while end < chars.len() && continues_identifier(chars[end]) {
        end += 1;
    }
    if start >= end {
        return None;
    }
    let text: String = chars[start..end].iter().collect();
    let range = SyntaxRange {
        start_line: line,
        start_character: start as u32,
        end_line: line,
        end_character: end as u32,
    };
    let role = roles
        .iter()
        .find(|(span, _)| covers(span, line, start as u32))
        .map(|(_, role)| *role);
    Some(SyntaxToken {
        is_keyword: super::is_reserved_keyword(&text),
        text,
        range,
        role,
    })
}

fn covers(span: &SyntaxRange, line: u32, character: u32) -> bool {
    span.start_line <= line
        && line <= span.end_line
        && (span.start_line < line || span.start_character <= character)
        && (line < span.end_line || character < span.end_character)
}

pub(super) fn unit_literal_at(source: &str, line: u32, character: u32) -> Option<SyntaxUnitLiteral> {
    let line_text = source.lines().nth(line as usize)?;
    let chars: Vec<char> = line_text.chars().collect();
    let cursor = character as usize;
    if cursor > chars.len() {
        return None;
    }
    let mut innermost: Option<(usize, usize)> = None;
    let mut open_brackets = Vec::new();
    for (index, &ch) in chars.iter().enumerate() {
        if ch == '[' {
            open_brackets.push(index);
        } else if ch == ']' {
            if let Some(open) = open_brackets.pop() {
                if cursor >= open && cursor <= index && preceded_by_number(&chars, open) {
                    if innermost.is_none_or(|(best, _)| open > best) {
                        innermost = Some((open, index));
                    }
                }
            }
        }
    }
    let (open, close) = innermost?;
    let inner: String = chars[open + 1..close].iter().collect();
    let leading = inner.chars().take_while(|ch| ch.is_whitespace()).count();
    let trailing = inner.chars().rev().take_while(|ch| ch.is_whitespace()).count();
    let unit = inner.trim();
    if unit.is_empty() {
        return None;
    }
    Some(SyntaxUnitLiteral {
        unit: unit.to_string(),
        range: SyntaxRange {
            start_line: line,
            start_character: (open + 1 + leading) as u32,
            end_line: line,
            end_character: (close - trailing) as u32,
        },
    })
}

/// Whether the token immediately before `open` is a numeric literal, which is what makes the
/// bracket a unit suffix rather than a multiplicity.
fn preceded_by_number(chars: &[char], open: usize) -> bool {
    let before: String = chars[..open].iter().collect();
    let Some(last) = before.trim_end().split_whitespace().last() else {
        return false;
    };
    let mut token = last.chars();
    let Some(first) = token.next() else {
        return false;
    };
    if !(first.is_ascii_digit() || ((first == '+' || first == '-') && last.len() > 1)) {
        return false;
    }
    last.chars()
        .all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | 'e' | 'E' | '+' | '-'))
}

/// Every whole-word occurrence of `name` in code — comments and string literals excluded.
pub(super) fn occurrences_of(source: &str, name: &str) -> Vec<SyntaxRange> {
    if name.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut in_block_comment = false;
    for (line_number, line) in source.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut index = 0usize;
        let mut in_string: Option<char> = None;
        let mut code = vec![false; chars.len()];
        while index < chars.len() {
            let ch = chars[index];
            if in_block_comment {
                if ch == '*' && chars.get(index + 1) == Some(&'/') {
                    in_block_comment = false;
                    index += 2;
                    continue;
                }
                index += 1;
                continue;
            }
            if let Some(quote) = in_string {
                if ch == '\\' {
                    index += 2;
                    continue;
                }
                if ch == quote {
                    in_string = None;
                }
                index += 1;
                continue;
            }
            if ch == '/' && chars.get(index + 1) == Some(&'/') {
                break;
            }
            if ch == '/' && chars.get(index + 1) == Some(&'*') {
                in_block_comment = true;
                index += 2;
                continue;
            }
            // Only `"` opens a string. A `'` opens a *quoted name*, which is code: a name that
            // needs quoting is still the name a rename must reach.
            if ch == '"' {
                in_string = Some(ch);
                index += 1;
                continue;
            }
            code[index] = true;
            index += 1;
        }

        let needle: Vec<char> = name.chars().collect();
        let mut at = 0usize;
        while at + needle.len() <= chars.len() {
            if chars[at..at + needle.len()] == needle[..]
                && code[at..at + needle.len()].iter().all(|inside| *inside)
                && !at
                    .checked_sub(1)
                    .and_then(|before| chars.get(before))
                    .copied()
                    .is_some_and(is_word_char)
                && !chars.get(at + needle.len()).copied().is_some_and(is_word_char)
            {
                out.push(SyntaxRange {
                    start_line: line_number as u32,
                    start_character: at as u32,
                    end_line: line_number as u32,
                    end_character: (at + needle.len()) as u32,
                });
                at += needle.len();
                continue;
            }
            at += 1;
        }
    }
    out
}

fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '-'
}

#[cfg(test)]
mod tests {
    use crate::syntax::SyntaxAuthority;

    fn parse(text: &str) -> crate::syntax::ParsedSource {
        SyntaxAuthority::new().parse_text(text)
    }

    #[test]
    fn the_token_under_the_cursor_spans_the_whole_identifier() {
        let parsed = parse("package P { part foo : Bar; }");
        let token = parsed.token_at(0, 18).expect("token");
        assert_eq!(token.text, "foo");
        assert_eq!((token.range.start_character, token.range.end_character), (17, 20));
        assert!(!token.is_keyword);
        assert!(parsed.token_at(0, 13).expect("keyword token").is_keyword);
    }

    #[test]
    fn a_qualified_name_is_one_token_and_reports_its_simple_name() {
        let parsed = parse("package P { part foo : A::B::C; }");
        let token = parsed.token_at(0, 26).expect("token");
        assert_eq!(token.text, "A::B::C");
        assert_eq!(token.simple_name(), "C");
    }

    #[test]
    fn a_non_ascii_identifier_is_measured_in_characters() {
        let parsed = parse("package P { part caf\u{00E9} : T; }");
        let token = parsed.token_at(0, 18).expect("token");
        assert_eq!(token.text, "caf\u{00E9}");
    }

    #[test]
    fn a_unit_suffix_is_a_unit_literal_and_a_multiplicity_is_not() {
        let parsed = parse("package P { attribute v = 10 [kV]; }");
        let bracket = "package P { attribute v = 10 ".len() as u32;
        let literal = parsed.unit_literal_at(0, bracket + 1).expect("unit literal");
        assert_eq!(literal.unit, "kV");
        assert_eq!(
            (literal.range.start_character, literal.range.end_character),
            (bracket + 1, bracket + 3)
        );
        assert!(parse("part p[0..1];").unit_literal_at(0, 8).is_none());
    }

    #[test]
    fn occurrences_are_whole_words_in_code_only() {
        let parsed = parse("part foo; part foobar; // foo\npart foo;");
        let ranges = parsed.occurrences_of("foo");
        assert_eq!(
            ranges
                .iter()
                .map(|range| (range.start_line, range.start_character))
                .collect::<Vec<_>>(),
            vec![(0, 5), (1, 5)],
            "a longer identifier and a comment are not occurrences"
        );
        assert!(parsed.occurrences_of("").is_empty());
    }

    #[test]
    fn a_name_inside_a_string_literal_is_not_an_occurrence() {
        let parsed = parse("part foo { doc /* foo */ \"foo\"; }");
        assert_eq!(parsed.occurrences_of("foo").len(), 1);
    }
}
