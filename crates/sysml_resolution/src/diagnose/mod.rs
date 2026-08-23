//! Phase 8: diagnostics, decided from the assembled model and nowhere else.

use crate::diagnostics::UNCODED_PARSE_ERROR;
use crate::lower::facts::LineIndex;
use crate::lower::facts::UnsupportedFamily;
use crate::lower::storage::ParsedSources;
use crate::lower::storage::SemanticModelStorage;
use crate::model::resolver::SemanticModel;
use crate::model::resolver::RELATED_AMBIGUOUS_CANDIDATE;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DocumentId;
use crate::model::ReferenceKind;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::Diagnostic;
use crate::DiagnosticCategory;
use crate::DiagnosticCode;
use crate::DiagnosticLocation;
use crate::DiagnosticOrigin;
use crate::DiagnosticSeverity;
use crate::TextPosition;
use crate::TextRange;
use sysml_v2_parser::ast::Span;
use sysml_v2_parser::ParseError;
use sysml_v2_parser::ParsedDocument;

impl<D> SemanticModel<D> {
    /// Every diagnostic this publication settled, as facts rather than rendered text.
    ///
    /// Derived once while the model is sealed, for the same reason the other indexes are: a query
    /// against a settled publication reads a fact instead of recomputing one, and a storage
    /// inconsistency fails the build rather than surfacing later as a silently missing diagnostic.
    ///
    /// Only workspace-authored documents contribute. Library sources take part in the same
    /// semantic system, but their own diagnostics are not the authoring surface, and this also
    /// keeps the barrier's cost proportional to the workspace rather than to the library.
    pub(crate) fn derive_diagnostics(
        &self,
        sources: &ParsedSources,
        reported: &[Box<str>],
    ) -> Result<DerivedDiagnostics, ResolutionError> {
        let mut diagnostics = Vec::new();
        let mut by_document = vec![(0u32, 0u32); self.storage.documents.len()];
        // Built once, then sliced per document. Every rule below asks "what did this document
        // declare", and answering that by scanning every declaration each time made the barrier
        // quadratic in the admitted corpus -- invisible while only workspace documents were
        // derived, and the reason a library's own documents could not be.
        let declarations_by_document = self.declarations_by_document()?;
        for document_index in self.reported_document_indices(reported) {
            let document = &self.storage.documents[document_index];
            let document_id = DocumentId(document_index as u32);
            let first = diagnostics.len();

            for error in sources.parse_errors(document_id) {
                let parsed = sources
                    .parsed(document_id)
                    .ok_or(ResolutionError::InvalidStorage)?;
                let range =
                    parse_error_range(parsed, error).ok_or(ResolutionError::InvalidStorage)?;
                diagnostics.push(Diagnostic {
                    // The parser owns both the code and the sentence; neither is re-derived here.
                    message: error.message.as_str().into(),
                    subject: None,
                    code: DiagnosticCode::Parser {
                        code: match error.code.as_deref() {
                            Some(code) => code.into(),
                            None => UNCODED_PARSE_ERROR.into(),
                        },
                        category: parser_diagnostic_category(error.category),
                    },
                    severity: match error.severity {
                        Some(sysml_v2_parser::DiagnosticSeverity::Warning) => {
                            DiagnosticSeverity::Warning
                        }
                        Some(sysml_v2_parser::DiagnosticSeverity::Error) | None => {
                            DiagnosticSeverity::Error
                        }
                    },
                    origin: DiagnosticOrigin::Parser,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range,
                    },
                    related: Box::default(),
                });
            }

            for record in self
                .storage
                .unsupported
                .iter()
                .filter(|record| record.document == document_id)
            {
                let code = unsupported_construct_code(record.family);
                diagnostics.push(Diagnostic {
                    message: code.describe().into(),
                    subject: None,
                    code,
                    severity: DiagnosticSeverity::Warning,
                    origin: DiagnosticOrigin::Semantic,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range: document_range(&self.storage, document_id, &record.span)?,
                    },
                    related: Box::default(),
                });
            }

            for (index, reference) in self.storage.references.iter().enumerate() {
                let source = self
                    .storage
                    .declaration(reference.source)
                    .ok_or(ResolutionError::InvalidStorage)?;
                if source.document != document_id {
                    continue;
                }
                let reference_id = AuthoredReferenceId::from_index(index)
                    .map_err(|_| ResolutionError::Capacity)?;
                let status = self
                    .resolution
                    .outcome(reference_id)
                    .ok_or(ResolutionError::InvalidStorage)?;
                let Some((severity, code)) = reference_diagnostic(reference.kind, status) else {
                    continue;
                };
                let mut related = Vec::new();
                if let ResolutionStatus::Ambiguous(candidates) = status {
                    // Every candidate, in the resolver's canonical candidate order: choosing one
                    // would settle an ambiguity the publication deliberately left open.
                    for target in self.resolution.ambiguous_candidates(candidates) {
                        related
                            .push(self.related_declaration(*target, RELATED_AMBIGUOUS_CANDIDATE)?);
                    }
                }
                diagnostics.push(Diagnostic {
                    message: code.describe().into(),
                    subject: self.symbol_id(reference.source),
                    code,
                    severity,
                    origin: DiagnosticOrigin::Semantic,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range: document_range(&self.storage, document_id, &reference.span)?,
                    },
                    related: related.into_boxed_slice(),
                });
            }

            let declared = declarations_by_document
                .get(document_index)
                .map(Vec::as_slice)
                .unwrap_or_default();
            self.collect_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_structural_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_expression_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_host_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_library_context(document_id, first, &mut diagnostics)?;

            // Ordering is owned here so no consumer has to sort, and so the order cannot vary with
            // which storage collection a diagnostic happened to come from. The sort is stable, so
            // parser, unsupported-construct, and reference diagnostics that share a range keep the
            // order they were derived in.
            diagnostics[first..].sort_by(|left, right| {
                left.location
                    .range
                    .cmp(&right.location.range)
                    .then_with(|| left.code.as_str().cmp(right.code.as_str()))
            });
            by_document[document_index] = (
                u32::try_from(first).map_err(|_| ResolutionError::Capacity)?,
                u32::try_from(diagnostics.len()).map_err(|_| ResolutionError::Capacity)?,
            );
        }
        Ok((
            diagnostics.into_boxed_slice(),
            by_document.into_boxed_slice(),
        ))
    }
}

/// The settled diagnostics of one publication with the per-document ranges that index them.
///
/// The two are derived together and are only correct together, so they are returned together
/// rather than as two calls a caller could interleave.
pub(crate) type DerivedDiagnostics = (Box<[Diagnostic]>, Box<[(u32, u32)]>);

pub(crate) fn document_range(
    storage: &SemanticModelStorage,
    document: DocumentId,
    span: &Span,
) -> Result<TextRange, ResolutionError> {
    // From the settled line index, not from the parse tree: a sealed publication answers a
    // location question from a fact it kept, never by re-reading source it no longer owns.
    storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .lines
        .range(span)
        .ok_or(ResolutionError::InvalidStorage)
}

pub(crate) fn parser_diagnostic_category(
    category: Option<sysml_v2_parser::DiagnosticCategory>,
) -> DiagnosticCategory {
    match category {
        Some(sysml_v2_parser::DiagnosticCategory::ParseError) => {
            DiagnosticCategory::MalformedSyntax
        }
        Some(sysml_v2_parser::DiagnosticCategory::UnsupportedGrammarForm) => {
            DiagnosticCategory::UnsupportedSyntax
        }
        Some(sysml_v2_parser::DiagnosticCategory::UnresolvedSymbol) => {
            DiagnosticCategory::Unresolved
        }
        None => DiagnosticCategory::UnclassifiedParser,
    }
}

pub(crate) fn parse_error_range(
    document: &ParsedDocument,
    error: &ParseError,
) -> Option<TextRange> {
    let start_offset = error.offset?;
    let end_offset = start_offset.checked_add(error.length.unwrap_or(1))?;
    let start = document.source.position_at(start_offset)?;
    let end = document.source.position_at(end_offset).unwrap_or(start);
    Some(TextRange {
        start: TextPosition {
            line: start.line.saturating_sub(1),
            character: u32::try_from(start.column.saturating_sub(1)).ok()?,
        },
        end: TextPosition {
            line: end.line.saturating_sub(1),
            character: u32::try_from(end.column.saturating_sub(1)).ok()?,
        },
    })
}

/// The public code for a construct this publication does not model.
///
/// Exhaustive by construction: a new lowering family cannot be added without deciding its code.
pub(crate) fn unsupported_construct_code(family: UnsupportedFamily) -> DiagnosticCode {
    match family {
        UnsupportedFamily::PackageMember => DiagnosticCode::UnsupportedPackageMember,
        UnsupportedFamily::PartDefinitionMember => DiagnosticCode::UnsupportedPartDefinitionMember,
        UnsupportedFamily::PartUsageMember => DiagnosticCode::UnsupportedPartUsageMember,
        UnsupportedFamily::AttributeMember => DiagnosticCode::UnsupportedAttributeMember,
        UnsupportedFamily::RequirementDefinitionMember => {
            DiagnosticCode::UnsupportedRequirementDefinitionMember
        }
        UnsupportedFamily::PortDefinitionMember => DiagnosticCode::UnsupportedPortDefinitionMember,
        UnsupportedFamily::PortUsageMember => DiagnosticCode::UnsupportedPortUsageMember,
        UnsupportedFamily::ActionDefinitionMember => {
            DiagnosticCode::UnsupportedActionDefinitionMember
        }
        UnsupportedFamily::ActionUsageMember => DiagnosticCode::UnsupportedActionUsageMember,
        UnsupportedFamily::StateDefinitionMember => {
            DiagnosticCode::UnsupportedStateDefinitionMember
        }
        UnsupportedFamily::ConnectionDefinitionMember => {
            DiagnosticCode::UnsupportedConnectionDefinitionMember
        }
        UnsupportedFamily::InterfaceDefinitionMember => {
            DiagnosticCode::UnsupportedInterfaceDefinitionMember
        }
        UnsupportedFamily::ViewDefinitionMember => DiagnosticCode::UnsupportedViewDefinitionMember,
        UnsupportedFamily::ConstraintDefinitionMember => {
            DiagnosticCode::UnsupportedConstraintDefinitionMember
        }
        UnsupportedFamily::CalcDefinitionMember => DiagnosticCode::UnsupportedCalcDefinitionMember,
        UnsupportedFamily::RenderingDefinitionMember => {
            DiagnosticCode::UnsupportedRenderingDefinitionMember
        }
        UnsupportedFamily::OccurrenceDefinitionMember => {
            DiagnosticCode::UnsupportedOccurrenceDefinitionMember
        }
        UnsupportedFamily::AnalysisCaseDefinitionMember => {
            DiagnosticCode::UnsupportedAnalysisCaseDefinitionMember
        }
        UnsupportedFamily::CaseDefinitionMember => DiagnosticCode::UnsupportedCaseDefinitionMember,
        UnsupportedFamily::VerificationCaseDefinitionMember => {
            DiagnosticCode::UnsupportedVerificationCaseDefinitionMember
        }
        UnsupportedFamily::UseCaseDefinitionMember => {
            DiagnosticCode::UnsupportedUseCaseDefinitionMember
        }
        UnsupportedFamily::ReferenceUsageMember => DiagnosticCode::UnsupportedReferenceUsageMember,
        UnsupportedFamily::RelationshipBodyMember => {
            DiagnosticCode::UnsupportedRelationshipBodyMember
        }
        UnsupportedFamily::ParserUnsupported => DiagnosticCode::UnsupportedParserConstruct,
    }
}

/// What one authored reference's settled outcome reports, or `None` when it resolved.
///
/// A resolved reference has nothing to report; that is not the same answer as any of the failure
/// states below, and the three failure classes stay distinct all the way to the consumer.
pub(crate) fn reference_diagnostic(
    kind: ReferenceKind,
    status: ResolutionStatus,
) -> Option<(DiagnosticSeverity, DiagnosticCode)> {
    match status {
        ResolutionStatus::Resolved(_) => None,
        ResolutionStatus::Unresolved => Some((
            DiagnosticSeverity::Warning,
            match kind {
                ReferenceKind::FeatureTyping => DiagnosticCode::UnresolvedTypeReference,
                ReferenceKind::Subclassification => DiagnosticCode::UnresolvedSpecializesReference,
                ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
                    DiagnosticCode::UnresolvedImportTarget
                }
                // A view names what it shows; an expose target that resolves to nothing means the
                // view shows nothing, which is a different thing to a reader than an unresolved
                // name in a declaration.
                ReferenceKind::ViewExpose => DiagnosticCode::ViewExposeUnresolved,
                _ => DiagnosticCode::UnresolvedReference,
            },
        )),
        ResolutionStatus::Unsupported => Some((
            DiagnosticSeverity::Warning,
            match kind {
                ReferenceKind::NamespaceImport
                | ReferenceKind::MembershipImport
                | ReferenceKind::FilterImport => DiagnosticCode::UnsupportedFilteredImport,
                _ => DiagnosticCode::UnsupportedReference,
            },
        )),
        ResolutionStatus::NonConverged => Some((
            DiagnosticSeverity::Error,
            DiagnosticCode::NonConvergedResolution,
        )),
        ResolutionStatus::Ambiguous(_) => Some((
            DiagnosticSeverity::Error,
            match kind {
                ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
                    DiagnosticCode::AmbiguousImportTarget
                }
                _ => DiagnosticCode::AmbiguousReference,
            },
        )),
    }
}

/// Where a *declaration* writes its own name.
///
/// Distinct from [`identifier_range`], which searches a reference span and takes the last
/// word-boundary match because a qualified path names its target in the final segment. A
/// declaration span covers the whole declaration including its body, so the same rule finds the
/// last mention of the name anywhere inside -- for `part def Vehicle { part engine : Vehicle; }`
/// it points at the body's reference rather than at the declared name.
///
/// The declared name is in the header, after the keywords and after an optional `<shortName>`, so
/// the search is bounded to the text before the body opener and skips the short-name group. A
/// declaration whose header is unrecoverable -- a parse recovery that lost its `{` or `;` -- falls
/// back to the whole-span search rather than losing its location entirely.
pub(crate) fn declaration_identifier_range(
    storage: &SemanticModelStorage,
    sources: &ParsedSources,
    document: DocumentId,
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

/// Whether `start` falls inside an unclosed `<`...`>` short-name group.
pub(crate) fn inside_short_name(header: &str, start: usize) -> bool {
    let before = &header[..start];
    before
        .rfind('<')
        .is_some_and(|open| !before[open..].contains('>'))
}

/// Every occurrence of `identifier` in `text` that is not part of a longer identifier.
pub(crate) fn word_boundary_matches<'a>(
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
    document: DocumentId,
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

pub(crate) fn identifier_text_range(
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

pub(crate) fn valid_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|first| first.is_alphabetic() || first == '_')
        && characters.all(identifier_character)
}

pub(crate) fn declaration_qualified_name(
    storage: &SemanticModelStorage,
    mut declaration: DeclarationId,
) -> Option<String> {
    let mut names = Vec::new();
    loop {
        let value = storage.declaration(declaration)?;
        if let Some(name) = value.name.and_then(|name| storage.symbol(name)) {
            names.push(name);
        }
        let Some(owner) = value.owner else {
            break;
        };
        declaration = owner;
    }
    names.reverse();
    Some(names.join("::"))
}
