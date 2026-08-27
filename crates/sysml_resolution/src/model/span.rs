//! Source-range projection shared by semantic phases and settled-model readers.
//!
//! These helpers read lowering's settled line and parsed-source stores. They do not decide a
//! diagnostic, so their owner is the earliest phase holding all prerequisites rather than the
//! diagnostics phase that first needed them.

use crate::lower::facts::LineIndex;
use crate::lower::storage::{ParsedSources, SemanticModelStorage};
use crate::model::DocumentIdx;
use crate::resolve::results::ResolutionError;
use crate::TextRange;
use sysml_v2_parser::ast::Span;

pub(crate) fn document_range(
    storage: &SemanticModelStorage,
    document: DocumentIdx,
    span: &Span,
) -> Result<TextRange, ResolutionError> {
    storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .lines
        .range(span)
        .ok_or(ResolutionError::InvalidStorage)
}

/// Projects the authored identifier in a declaration header.
pub(crate) fn declaration_identifier_range(
    storage: &SemanticModelStorage,
    sources: &ParsedSources,
    document: DocumentIdx,
    span: &Span,
    identifier: &str,
) -> Result<TextRange, ResolutionError> {
    let lines = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .lines;
    let source = sources
        .parsed(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .source
        .slice(span)
        .ok_or(ResolutionError::InvalidStorage)?;
    let header = source
        .find(['{', ';'])
        .map_or(source, |body| &source[..body]);
    let relative = word_boundary_matches(header, identifier)
        .find(|start| !inside_short_name(header, *start))
        .or_else(|| word_boundary_matches(header, identifier).next())
        .or_else(|| word_boundary_matches(source, identifier).last())
        .ok_or(ResolutionError::InvalidStorage)?;
    identifier_text_range(lines, span, relative, identifier.len())
}

fn inside_short_name(header: &str, start: usize) -> bool {
    let before = &header[..start];
    before
        .rfind('<')
        .is_some_and(|open| !before[open..].contains('>'))
}

fn word_boundary_matches<'a>(
    text: &'a str,
    identifier: &'a str,
) -> impl Iterator<Item = usize> + 'a {
    text.match_indices(identifier)
        .filter(move |(start, _)| {
            let before = text[..*start].chars().next_back();
            let after = text[*start + identifier.len()..].chars().next();
            !before.is_some_and(identifier_character) && !after.is_some_and(identifier_character)
        })
        .map(|(start, _)| start)
}

pub(crate) fn identifier_range(
    storage: &SemanticModelStorage,
    sources: &ParsedSources,
    document: DocumentIdx,
    span: &Span,
    identifier: &str,
) -> Result<TextRange, ResolutionError> {
    let lines = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .lines;
    let source = sources
        .parsed(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .source
        .slice(span)
        .ok_or(ResolutionError::InvalidStorage)?;
    let relative = word_boundary_matches(source, identifier)
        .last()
        .ok_or(ResolutionError::InvalidStorage)?;
    identifier_text_range(lines, span, relative, identifier.len())
}

fn identifier_text_range(
    lines: &LineIndex,
    span: &Span,
    relative: usize,
    length: usize,
) -> Result<TextRange, ResolutionError> {
    let start_offset = span
        .offset
        .checked_add(relative)
        .ok_or(ResolutionError::Capacity)?;
    let end_offset = start_offset
        .checked_add(length)
        .ok_or(ResolutionError::Capacity)?;
    let start = lines
        .position(start_offset)
        .ok_or(ResolutionError::InvalidStorage)?;
    let end = lines
        .position(end_offset)
        .ok_or(ResolutionError::InvalidStorage)?;
    Ok(TextRange { start, end })
}

pub(crate) fn identifier_character(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '-')
}
