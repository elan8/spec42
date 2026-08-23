//! Where in a document a fact was found.
//!
//! A position is a zero-based line and UTF-16 code-unit offset, matching the shape every editor
//! protocol already speaks, and a range is a pair of them. Neither knows which document it is in:
//! the document identity travels with the fact, so the same range type serves a diagnostic, a
//! declaration and a reference without one of them inventing a role it does not have.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextPosition {
    pub line: u32,
    pub character: u32,
}

impl TextPosition {
    pub const fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextRange {
    pub start: TextPosition,
    pub end: TextPosition,
}

impl TextRange {
    pub const fn new(start: TextPosition, end: TextPosition) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OccurrenceRole {
    Declaration,
    Reference,
}
