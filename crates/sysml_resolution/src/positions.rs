//! Source coordinates shared by syntax and immutable semantic publications.

use crate::{TextPosition, TextRange};
use sysml_v2_parser::Span;

#[derive(Debug)]
struct Multibyte {
    start: u32,
    end: u32,
    utf16_end: u32,
    excess: u32,
}

/// A compact position index retaining no source text or parser tree.
#[derive(Debug, Default)]
pub(crate) struct LineIndex {
    starts: Box<[u32]>,
    length: u32,
    multibyte: Box<[Multibyte]>,
}

impl LineIndex {
    pub(crate) fn build(text: &str) -> Self {
        let length =
            u32::try_from(text.len()).expect("source exceeds the 32-bit source coordinate domain");
        let mut starts = vec![0];
        let mut multibyte = Vec::new();
        let mut excess = 0;
        for (offset, character) in text.char_indices() {
            if character == '\n' {
                starts.push((offset + 1) as u32);
            }
            if !character.is_ascii() {
                excess += (character.len_utf8() - character.len_utf16()) as u32;
                let end = (offset + character.len_utf8()) as u32;
                multibyte.push(Multibyte {
                    start: offset as u32,
                    end,
                    utf16_end: end - excess,
                    excess,
                });
            }
        }
        Self {
            starts: starts.into_boxed_slice(),
            length,
            multibyte: multibyte.into_boxed_slice(),
        }
    }

    fn excess_at(&self, offset: u32) -> Option<u32> {
        let index = self.multibyte.partition_point(|entry| entry.end <= offset);
        if self
            .multibyte
            .get(index)
            .is_some_and(|entry| entry.start < offset)
        {
            return None;
        }
        Some(
            index
                .checked_sub(1)
                .map_or(0, |previous| self.multibyte[previous].excess),
        )
    }

    /// Project a byte offset onto a zero-based UTF-16 editor position.
    pub(crate) fn position(&self, offset: usize) -> Option<TextPosition> {
        let offset = u32::try_from(offset).ok()?;
        if offset > self.length {
            return None;
        }
        let line = self
            .starts
            .partition_point(|start| *start <= offset)
            .checked_sub(1)?;
        let start = self.starts[line];
        Some(TextPosition::new(
            line as u32,
            offset - start - (self.excess_at(offset)? - self.excess_at(start)?),
        ))
    }

    /// Resolve a UTF-16 position, rejecting surrogate interiors and out-of-line columns.
    pub(crate) fn offset(&self, position: TextPosition) -> Option<usize> {
        let start = *self.starts.get(position.line as usize)?;
        let utf16 = (start - self.excess_at(start)?).checked_add(position.character)?;
        let index = self
            .multibyte
            .partition_point(|entry| entry.utf16_end <= utf16);
        let excess = index
            .checked_sub(1)
            .map_or(0, |previous| self.multibyte[previous].excess);
        let offset = utf16.checked_add(excess)? as usize;
        (self.position(offset)? == position).then_some(offset)
    }

    pub(crate) fn range(&self, span: &Span) -> Option<TextRange> {
        Some(TextRange::new(
            self.position(span.offset)?,
            self.position(span.offset.checked_add(span.len)?)?,
        ))
    }

    /// Parser range columns are byte offsets. Legacy single-line token windows may run past
    /// that line; clamp those windows to the authored line before projecting them.
    pub(crate) fn syntax_range(
        &self,
        range: crate::syntax::SyntaxRange,
    ) -> Option<crate::syntax::SyntaxRange> {
        let project = |line: u32, column: u32| {
            let start = *self.starts.get(line as usize)?;
            let end = self
                .starts
                .get(line as usize + 1)
                .map_or(self.length, |next| next - 1);
            self.position(start.checked_add(column)?.min(end) as usize)
        };
        let start = project(range.start_line, range.start_character)?;
        let end = project(range.end_line, range.end_character)?;
        Some(crate::syntax::SyntaxRange {
            start_line: start.line,
            start_character: start.character,
            end_line: end.line,
            end_character: end.character,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_and_utf16_positions_round_trip_at_every_character_boundary() {
        for text in ["", "ascii\n", "aé😀z\nβ𐐀\n", "é\r\n😀"] {
            let index = LineIndex::build(text);
            for offset in text
                .char_indices()
                .map(|(offset, _)| offset)
                .chain([text.len()])
            {
                let prefix = &text[..offset];
                let expected = TextPosition::new(
                    prefix.bytes().filter(|byte| *byte == b'\n').count() as u32,
                    prefix.rsplit('\n').next().unwrap().encode_utf16().count() as u32,
                );
                assert_eq!(index.position(offset), Some(expected));
                assert_eq!(index.offset(expected), Some(offset));
            }
        }
    }

    #[test]
    fn invalid_byte_and_surrogate_positions_are_rejected() {
        let index = LineIndex::build("é😀\nz");
        assert_eq!(index.position(1), None);
        assert_eq!(index.offset(TextPosition::new(0, 2)), None);
        assert_eq!(index.offset(TextPosition::new(0, 8)), None);
        assert_eq!(index.position(100), None);
    }
}
