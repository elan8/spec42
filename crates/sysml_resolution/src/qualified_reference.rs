//! Resolution of readable KerML qualified references to canonical publication identities.
//!
//! A qualified name is a semantic lookup key, not an element identity. The optional document
//! identity narrows the lookup to one admitted source; without it, resolution ranges over every
//! source domain in this immutable publication. The result always carries the publication-owned
//! opaque [`SymbolId`](crate::SymbolId).

use crate::{
    ElementKind, ElementSearch, ElementSource, PublicationCompleteness, PublishedResolution,
    QueryOutcome, SourceLocation, SymbolEntry, SymbolId,
};

/// A readable element reference interpreted against one immutable publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedElementReference {
    /// The normalized identity of one admitted document, or `None` for publication-wide lookup.
    pub document: Option<Box<str>>,
    /// A KerML ownership-qualified name such as `Vehicle::views::overview`.
    pub qualified_name: Box<str>,
    /// When present, a declaration of any other kind produces [`QualifiedReferenceOutcome::WrongKind`].
    pub expected_kind: Option<ElementKind>,
}

/// One canonical candidate for a readable qualified reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedReferenceTarget {
    /// The element this candidate names.
    ///
    /// The qualified name is not handed back: the caller supplied it in the request, and the
    /// publication already stores it once. Read it with
    /// [`PublishedResolution::qualified_name`] where a renderer needs the text.
    pub identity: SymbolId,
    pub kind: ElementKind,
    pub location: SourceLocation,
}

/// Explicit result of resolving a [`QualifiedElementReference`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualifiedReferenceOutcome {
    Resolved(QualifiedReferenceTarget),
    /// A unique target from a parse-recovery publication.
    Recovered(QualifiedReferenceTarget),
    /// A unique target from a publication containing unsupported syntax.
    UnsupportedWith(QualifiedReferenceTarget),
    Unresolved,
    Ambiguous(Box<[QualifiedReferenceTarget]>),
    WrongKind(Box<[QualifiedReferenceTarget]>),
    Unsupported,
    Recovery,
    Incomplete,
}

impl PublishedResolution {
    /// Resolves an authored qualified name without exposing or reconstructing identity encoding.
    ///
    /// With `document`, lookup is confined to that normalized document identity. Without it, all
    /// workspace, standard-library, library and external declarations participate. Candidate
    /// ordering is canonical by source location and then opaque identity.
    pub fn resolve_qualified_reference(
        &self,
        reference: &QualifiedElementReference,
    ) -> QualifiedReferenceOutcome {
        if reference.qualified_name.is_empty() {
            return QualifiedReferenceOutcome::Unresolved;
        }

        let entries = match &reference.document {
            Some(document) => match self.document_symbols(document) {
                QueryOutcome::Resolved(entries)
                | QueryOutcome::Recovered(entries)
                | QueryOutcome::UnsupportedWith(entries) => entries.into_vec(),
                QueryOutcome::Unresolved => return QualifiedReferenceOutcome::Unresolved,
                QueryOutcome::Ambiguous(_) => return QualifiedReferenceOutcome::Incomplete,
                QueryOutcome::Unsupported => return QualifiedReferenceOutcome::Unsupported,
                QueryOutcome::Recovery => return QualifiedReferenceOutcome::Recovery,
                QueryOutcome::Incomplete => return QualifiedReferenceOutcome::Incomplete,
            },
            None => self.qualified_reference_candidates(),
        };

        let mut named = entries
            .into_iter()
            .filter(|entry| entry.qualified_name == reference.qualified_name)
            .map(QualifiedReferenceTarget::from)
            .collect::<Vec<_>>();
        canonicalize_targets(&mut named);

        let mut matching = named
            .iter()
            .filter(|candidate| {
                reference
                    .expected_kind
                    .is_none_or(|kind| candidate.kind == kind)
            })
            .cloned()
            .collect::<Vec<_>>();
        canonicalize_targets(&mut matching);

        match matching.len() {
            0 if !named.is_empty() => {
                QualifiedReferenceOutcome::WrongKind(named.into_boxed_slice())
            }
            0 => self.absent_qualified_reference_outcome(),
            1 => self.settled_qualified_reference_outcome(matching.pop().expect("one target")),
            _ => QualifiedReferenceOutcome::Ambiguous(matching.into_boxed_slice()),
        }
    }

    fn qualified_reference_candidates(&self) -> Vec<SymbolEntry> {
        let mut entries = Vec::new();
        for source in [
            ElementSource::Workspace,
            ElementSource::StandardLibrary,
            ElementSource::Library,
            ElementSource::External,
        ] {
            for &kind in ElementKind::ALL {
                match self.search_elements(ElementSearch { kind, source }) {
                    QueryOutcome::Resolved(found)
                    | QueryOutcome::Recovered(found)
                    | QueryOutcome::UnsupportedWith(found) => entries.extend(found.into_vec()),
                    QueryOutcome::Unresolved
                    | QueryOutcome::Ambiguous(_)
                    | QueryOutcome::Unsupported
                    | QueryOutcome::Recovery
                    | QueryOutcome::Incomplete => {}
                }
            }
        }
        entries
    }

    fn absent_qualified_reference_outcome(&self) -> QualifiedReferenceOutcome {
        match self.completeness() {
            PublicationCompleteness::Complete => QualifiedReferenceOutcome::Unresolved,
            PublicationCompleteness::ParseRecovery => QualifiedReferenceOutcome::Recovery,
            PublicationCompleteness::UnsupportedSyntax => QualifiedReferenceOutcome::Unsupported,
            PublicationCompleteness::NonConverged => QualifiedReferenceOutcome::Incomplete,
        }
    }

    fn settled_qualified_reference_outcome(
        &self,
        target: QualifiedReferenceTarget,
    ) -> QualifiedReferenceOutcome {
        match self.completeness() {
            PublicationCompleteness::Complete => QualifiedReferenceOutcome::Resolved(target),
            PublicationCompleteness::ParseRecovery => QualifiedReferenceOutcome::Recovered(target),
            PublicationCompleteness::UnsupportedSyntax => {
                QualifiedReferenceOutcome::UnsupportedWith(target)
            }
            PublicationCompleteness::NonConverged => QualifiedReferenceOutcome::Incomplete,
        }
    }
}

impl From<SymbolEntry> for QualifiedReferenceTarget {
    fn from(entry: SymbolEntry) -> Self {
        Self {
            identity: entry.identity,
            kind: entry.kind,
            location: entry.location,
        }
    }
}

fn canonicalize_targets(targets: &mut [QualifiedReferenceTarget]) {
    targets.sort_by(|left, right| {
        left.location
            .document
            .cmp(&right.location.document)
            .then_with(|| left.location.range.cmp(&right.location.range))
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.identity.cmp(&right.identity))
    });
}
