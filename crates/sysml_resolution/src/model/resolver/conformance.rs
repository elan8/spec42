//! Typed conformance decisions, settled at the publication barrier.
//!
//! Every rule here is a function of facts the publication already owns: a declaration's metaclass,
//! its authored modifiers, multiplicity and direction, the settled outcome of an authored
//! reference, and the specialization closure. None of them re-resolves a name, reads authored
//! text, scans the model, or accepts a caller-chosen list of acceptable kinds.

use super::*;

impl ResolvedSemanticModel {
    /// Appends every conformance diagnostic authored in `document`.
    ///
    /// Ordering is the caller's: [`Self::derive_diagnostics`] sorts each document's diagnostics by
    /// range and code once every producer has contributed.
    pub(super) fn collect_conformance(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        self.collect_type_relationship_cardinality(document, diagnostics)?;
        Ok(())
    }

    /// Reports a type that owns exactly one `unions`, `intersects` or `differences` operand.
    ///
    /// KerML permits zero or at least two. One operand makes the operation the identity, so the
    /// declaration states a generalization the author did not write; the rule is a cardinality
    /// constraint on the relationship, not a conformance question, which is why it reads the
    /// authored references rather than the entailment.
    ///
    /// `disjoint from` is deliberately absent: disjointness is a pairwise statement about the
    /// owner and each target, so one target is meaningful.
    fn collect_type_relationship_cardinality(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for index in 0..self.storage.declarations.len() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            if declaration.document != document {
                continue;
            }
            for kind in [
                ReferenceKind::Unioning,
                ReferenceKind::Intersecting,
                ReferenceKind::Differencing,
            ] {
                // Counted over authored references rather than settled operands: whether the
                // author wrote one target does not depend on whether it resolved.
                let mut authored = self
                    .storage
                    .references
                    .iter()
                    .filter(|reference| reference.source == id && reference.kind == kind);
                let Some(only) = authored.next() else {
                    continue;
                };
                if authored.next().is_some() {
                    continue;
                }
                diagnostics.push(self.reference_diagnostic(
                    only,
                    DiagnosticCode::SingleTypeRelationshipOperand,
                    DiagnosticSeverity::Error,
                    None,
                )?);
            }
        }
        Ok(())
    }

    /// Applies the redefinition rules to the redefinitions the resolver derived.
    ///
    /// A feature that redeclares an inherited feature's name redefines it without writing `:>>`,
    /// and that implied redefinition is where narrowing is usually authored -- `part def
    /// NarrowedFleet :> Fleet { part vehicles[3..3] : Chassis; }` states the whole redefinition in
    /// the multiplicity. Checking only authored references would leave the common case unchecked.
    ///
    /// Reported at the redefining declaration rather than at a reference span, because there is no
    /// authored reference: the relationship's provenance is `Implied`, and pointing at a range the
    /// author did not write would misattribute it.
    pub(super) fn reference_diagnostic(
        &self,
        reference: &AuthoredReference,
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        related: Option<DeclarationId>,
    ) -> Result<Diagnostic, ResolutionError> {
        let source = self
            .storage
            .declaration(reference.source)
            .ok_or(ResolutionError::InvalidStorage)?;
        Ok(Diagnostic {
            code,
            severity,
            origin: DiagnosticOrigin::Semantic,
            location: DiagnosticLocation {
                document: writer::document_identity(self, source.document).into(),
                range: document_range(&self.storage, source.document, &reference.span)?,
            },
            related: match related {
                Some(target) => Box::from([self.declaration_location(target)?]),
                None => Box::default(),
            },
        })
    }

    /// The declaration site one diagnostic points at as related information.
    pub(super) fn declaration_location(
        &self,
        declaration: DeclarationId,
    ) -> Result<DiagnosticLocation, ResolutionError> {
        let declaration = self
            .storage
            .declaration(declaration)
            .ok_or(ResolutionError::InvalidStorage)?;
        Ok(DiagnosticLocation {
            document: writer::document_identity(self, declaration.document).into(),
            range: document_range(&self.storage, declaration.document, &declaration.span)?,
        })
    }
}
