//! Resolution of readable KerML qualified references to canonical publication identities.
//!
//! A qualified name is a semantic lookup key, not an element identity. The optional document
//! identity narrows the lookup to one admitted source; without it, resolution ranges over every
//! source domain in this immutable publication. The result always carries the publication-owned
//! opaque [`SymbolId`](crate::SymbolId).

use crate::{
    ElementKind, ElementSearch, ElementSource, PublishedResolution, QueryAnswer, SourceLocation,
    SymbolEntry, SymbolId,
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
            Some(document) => match self.document_symbols(document).answer {
                QueryAnswer::Resolved(entries) => entries.into_vec(),
                QueryAnswer::Unresolved => return QualifiedReferenceOutcome::Unresolved,
                QueryAnswer::Ambiguous(_) => return QualifiedReferenceOutcome::Incomplete,
                QueryAnswer::Unsupported => return QualifiedReferenceOutcome::Unsupported,
                QueryAnswer::Recovery => return QualifiedReferenceOutcome::Recovery,
                QueryAnswer::Incomplete => return QualifiedReferenceOutcome::Incomplete,
            },
            None => self.qualified_reference_candidates(),
        };
        let mut named = entries
            .into_iter()
            .filter(|entry| {
                self.qualified_name(entry.identity) == Some(reference.qualified_name.as_ref())
            })
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
                match self.search_elements(ElementSearch { kind, source }).answer {
                    QueryAnswer::Resolved(found) => entries.extend(found.into_vec()),
                    QueryAnswer::Unresolved
                    | QueryAnswer::Ambiguous(_)
                    | QueryAnswer::Unsupported
                    | QueryAnswer::Recovery
                    | QueryAnswer::Incomplete => {}
                }
            }
        }
        entries
    }

    fn absent_qualified_reference_outcome(&self) -> QualifiedReferenceOutcome {
        let completeness = self.completeness();
        if completeness.contains(crate::PublicationObstacle::NonConverged) {
            QualifiedReferenceOutcome::Incomplete
        } else if completeness.contains(crate::PublicationObstacle::UnsupportedSyntax) {
            QualifiedReferenceOutcome::Unsupported
        } else if completeness.contains(crate::PublicationObstacle::ParseRecovery) {
            QualifiedReferenceOutcome::Recovery
        } else {
            QualifiedReferenceOutcome::Unresolved
        }
    }

    fn settled_qualified_reference_outcome(
        &self,
        target: QualifiedReferenceTarget,
    ) -> QualifiedReferenceOutcome {
        let completeness = self.completeness();
        if completeness.contains(crate::PublicationObstacle::NonConverged) {
            QualifiedReferenceOutcome::Incomplete
        } else if completeness.contains(crate::PublicationObstacle::UnsupportedSyntax) {
            QualifiedReferenceOutcome::UnsupportedWith(target)
        } else if completeness.contains(crate::PublicationObstacle::ParseRecovery) {
            QualifiedReferenceOutcome::Recovered(target)
        } else {
            QualifiedReferenceOutcome::Resolved(target)
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
