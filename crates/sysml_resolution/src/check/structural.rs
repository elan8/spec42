//! Structural feature-conformance decisions, settled at the publication barrier.
//!
//! The KerML feature rules and the SysML variation and connection-end rules, over the typed facts
//! the publication owns: authored modifiers, direction, uniqueness, effective positional ends,
//! featuring types, and settled reference outcomes.
//!
//! # What changed relative to the legacy check
//!
//! The legacy versions of these rules were written against a graph that could not represent an
//! effective end closure, an effective featuring type, or uniqueness for every feature kind, and
//! each rule fell silent wherever its prerequisite was missing. Where this publication can state
//! the fact, the rule now answers:
//!
//! - the end-count rules read the derived effective end count, so a declaration whose ends are
//!   entirely inherited is checked rather than skipped;
//! - the uniqueness rule reads uniqueness with its provenance, so KerML's default applies where no
//!   `nonunique` was authored rather than the question being abandoned.
//!
//! Where the fact genuinely is not available the rule still declines to answer, and says so in a
//! comment naming what is missing.

use crate::check::conformance;
use crate::diagnose::document_range;
use crate::lower::facts::ParameterDirection;
use crate::model::resolver::SemanticModel;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentId;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::type_query::Conformance;
use crate::type_query::SpecializationScope;
use crate::Diagnostic;
use crate::DiagnosticCode;
use crate::DiagnosticSeverity;

/// Whether a declaration is a connection-like definition: one whose members include connector
/// ends.
pub(crate) fn is_connection_like(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ConnectionDefinition
            | DeclarationKind::InterfaceDefinition
            | DeclarationKind::FlowDefinition
            | DeclarationKind::AllocationDefinition
    )
}

/// Whether a connection-like definition is binary, so its end count is fixed at two.
///
/// SysML flow and allocation definitions relate exactly two ends. A connection or interface
/// definition may be n-ary, so only the incomplete-pair rule constrains it.
pub(crate) fn requires_exactly_two_ends(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::FlowDefinition | DeclarationKind::AllocationDefinition
    )
}

/// Whether a redefining feature's direction is the redefined feature's or narrows it.
///
/// KerML lets a redefinition restrict direction, so this is a conformance test rather than
/// equality: `inout` permits values in both directions, and a redefinition narrowing it to `in` or
/// `out` removes one rather than contradicting it. The Kernel Semantic Library relies on exactly
/// that -- `out feature afterValues redefines values;` narrows an `inout` parameter -- and an
/// equality test, which is what the legacy check used, reported it as a mismatch.
pub(crate) fn direction_conforms(actual: ParameterDirection, expected: ParameterDirection) -> bool {
    match expected {
        // Already the widest, so any direction narrows it.
        ParameterDirection::InOut => true,
        ParameterDirection::In => actual == ParameterDirection::In,
        ParameterDirection::Out => actual == ParameterDirection::Out,
    }
}

/// Whether uniqueness was authored or is KerML's default.
///
/// The pinned parser cannot express `unique` at all -- it consumes the keyword without recording it
/// (`planning/UPSTREAM_PARSER_GAPS.md`, Gap 52) -- so an authored `unique` and an unwritten one are
/// indistinguishable. `nonunique` *is* a field, which is what makes the pair decidable: a feature
/// is non-unique exactly when `nonunique` was authored, and unique otherwise by KerML's default.
/// The provenance is published so the two are never confused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Uniqueness {
    /// `nonunique` was written.
    NonUniqueAuthored,
    /// Nothing was written, so KerML's default applies.
    UniqueByDefault,
}

impl<D> SemanticModel<D> {
    pub(crate) fn uniqueness(&self, declaration: DeclarationId) -> Uniqueness {
        match self
            .storage
            .declaration_facts(declaration)
            .is_some_and(|facts| facts.modifiers.nonunique)
        {
            true => Uniqueness::NonUniqueAuthored,
            false => Uniqueness::UniqueByDefault,
        }
    }

    /// Whether any parser-recovery span falls inside `declaration`'s own span.
    ///
    /// Recovery is a published fact, so a rule whose operands could have been swallowed by it can
    /// ask. This is barrier work over one document's recovery records, not a query-time scan.
    pub(crate) fn contains_recovery(
        &self,
        declaration: DeclarationId,
    ) -> Result<bool, ResolutionError> {
        let record = self
            .storage
            .declaration(declaration)
            .ok_or(ResolutionError::InvalidStorage)?;
        if self.storage.recovery.is_empty() {
            return Ok(false);
        }
        let span = document_range(&self.storage, record.document, &record.span)?;
        for recovery in self.storage.recovery.iter() {
            if recovery.document != record.document {
                continue;
            }
            let recovered = document_range(&self.storage, recovery.document, &recovery.span)?;
            if recovered.start >= span.start && recovered.end <= span.end {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether a declaration is a feature of its owner rather than an owned type.
    pub(crate) fn is_feature_member(&self, declaration: DeclarationId) -> bool {
        self.storage
            .memberships
            .iter()
            .find(|membership| membership.member == declaration)
            .is_some_and(|membership| membership.kind == MembershipKind::Feature)
    }

    /// Whether a declaration is an end feature.
    ///
    /// Three authored forms mean it, and all three count: the `end` modifier prefix on a feature,
    /// an `end` member of a connection-like body (which carries a positional-end fact rather than
    /// the prefix), and a KerML `end` association member.
    pub(crate) fn is_end_feature(&self, declaration: DeclarationId) -> bool {
        let Some(facts) = self.storage.declaration_facts(declaration) else {
            return false;
        };
        facts.modifiers.end
            || facts.positional_end.is_some()
            || self
                .storage
                .declaration(declaration)
                .is_some_and(|value| value.kind == DeclarationKind::KermlEnd)
    }

    /// Appends every structural feature-conformance diagnostic authored in `document`.
    pub(crate) fn collect_structural_conformance(
        &self,
        document: DocumentId,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        self.collect_declaration_structure(declared, diagnostics)?;
        self.collect_structural_reference_rules(document, diagnostics)?;
        self.collect_implied_structural_rules(document, diagnostics)?;
        Ok(())
    }

    /// The rules whose whole operand set is one declaration's own facts.
    pub(crate) fn collect_declaration_structure(
        &self,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            let facts = self
                .storage
                .declaration_facts(id)
                .ok_or(ResolutionError::InvalidStorage)?;

            // KerML 8.3.3.3.1: an end feature is neither derived, abstract nor composite.
            //
            // The same clause also forbids a direction on an end feature, which this publication
            // does not report: the pinned parser accepts no spelling that authors both. `in end
            // feature x : T;`, `end in feature x : T;` and SysML's `end in port x : T;` are all
            // rejected before semantics sees them (planning/UPSTREAM_PARSER_GAPS.md, Gap 59), so
            // the rule would be a code that can never fire.
            if self.is_end_feature(id)
                && (facts.modifiers.derived
                    || facts.modifiers.is_abstract
                    || facts.modifiers.composite)
            {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::EndFeatureInvalidRestrictions,
                    DiagnosticSeverity::Warning,
                )?);
            }

            // An abstract declaration is deliberately incomplete, so its end count states nothing.
            if !is_connection_like(declaration.kind) || facts.modifiers.is_abstract {
                continue;
            }
            // The abstract guard above now fires for all four connection-like kinds:
            // `ConnectionDef`, `FlowDef`, `AllocationDef` and `InterfaceDef` each carry a
            // `definition_prefix`, and `lower_connection_def` and its siblings publish it as the
            // `is_abstract` modifier. An `abstract connection def C { end a; }` is deliberately
            // incomplete and is no longer reported.
            //
            // A recovered member inside the declaration means the parser could not read part of its
            // body, so the ends it authored are unknown rather than few. `connection def AB { end
            // [1] item a : A { @M; } ... }` recovers its first end and would otherwise be reported
            // as having one end when it authors two.
            if self.contains_recovery(id)? {
                continue;
            }
            let ends = self.types.effective_ends(id);
            // One end is an incomplete pair whatever the arity: a connector relates at least two
            // things. Zero ends is a declaration that states no connection structure at all, which
            // is a different thing from an incomplete one.
            if ends == 1 {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::IncompleteConnectionLikeEndPair,
                    DiagnosticSeverity::Warning,
                )?);
            }
            if requires_exactly_two_ends(declaration.kind) && ends > 2 {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::InvalidBinaryConnectionLikeEndCount,
                    DiagnosticSeverity::Warning,
                )?);
            }
        }
        Ok(())
    }

    /// The rules whose operands are an authored reference and its settled target.
    pub(crate) fn collect_structural_reference_rules(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for reference in self.storage.references.iter() {
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if source.document != document {
                continue;
            }
            let index = self.reference_index(reference)?;
            let Some(ResolutionStatus::Resolved(target)) = self.resolution.outcome(index) else {
                continue;
            };
            match reference.kind {
                // SysML 8.4.4: a flow payload carries an occurrence, not a value. The occurrence
                // families are exactly those descending from `Occurrence` in the metamodel, so this
                // asks the same hierarchy the kind rules do rather than naming library types.
                ReferenceKind::FlowPayloadType => {
                    let Some((family, _)) = self.declaration_family(target) else {
                        continue;
                    };
                    if !conformance::descends_from_occurrence(family) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            DiagnosticCode::FlowPayloadTypeNotOccurrence,
                            DiagnosticSeverity::Error,
                            Some(target),
                        )?);
                    }
                }
                // SysML 8.4.3: a typed `variant` member uses the variation's own usage kind.
                ReferenceKind::Variant => {
                    if !self
                        .storage
                        .declaration_facts(reference.source)
                        .is_some_and(|facts| facts.modifiers.variation)
                    {
                        continue;
                    }
                    let (Some((variation, _)), Some((variant, _))) = (
                        self.declaration_family(reference.source),
                        self.declaration_family(target),
                    ) else {
                        continue;
                    };
                    if !conformance::families_are_comparable(variation, variant) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            DiagnosticCode::InvalidVariationMemberKind,
                            DiagnosticSeverity::Warning,
                            Some(target),
                        )?);
                    }
                }
                ReferenceKind::Redefinition => {
                    for code in self.redefinition_structure(reference.source, target) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            code,
                            DiagnosticSeverity::Warning,
                            Some(target),
                        )?);
                    }
                    if let Some(code) = self.featuring_structure(reference.source, target) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            code,
                            DiagnosticSeverity::Error,
                            Some(target),
                        )?);
                    }
                }
                // KerML 8.3.3.3.10: a non-unique feature cannot subset a unique one, since it would
                // admit repeated values the subsetted feature excludes.
                ReferenceKind::Subsetting
                    if self.uniqueness(reference.source) == Uniqueness::NonUniqueAuthored
                        && self.uniqueness(target) == Uniqueness::UniqueByDefault =>
                {
                    diagnostics.push(self.reference_diagnostic(
                        reference,
                        DiagnosticCode::SubsettingUniquenessMismatch,
                        DiagnosticSeverity::Warning,
                        Some(target),
                    )?);
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// The redefinition rules, over the redefinitions the resolver derived.
    ///
    /// Reported at the redefining declaration, since an implied relationship has no authored range.
    pub(crate) fn collect_implied_structural_rules(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for relationship in self.resolution.implied_relationships.iter() {
            if relationship.kind != ReferenceKind::Redefinition {
                continue;
            }
            if self
                .storage
                .declaration(relationship.source)
                .ok_or(ResolutionError::InvalidStorage)?
                .document
                != document
            {
                continue;
            }
            for code in self.redefinition_structure(relationship.source, relationship.target) {
                diagnostics.push(self.declaration_diagnostic(
                    relationship.source,
                    code,
                    DiagnosticSeverity::Warning,
                )?);
            }
            if let Some(code) = self.featuring_structure(relationship.source, relationship.target) {
                diagnostics.push(self.declaration_diagnostic(
                    relationship.source,
                    code,
                    DiagnosticSeverity::Error,
                )?);
            }
        }
        Ok(())
    }

    /// The end and direction rules of one redefinition.
    pub(crate) fn redefinition_structure(
        &self,
        source: DeclarationId,
        target: DeclarationId,
    ) -> Vec<DiagnosticCode> {
        let mut codes = Vec::new();
        // KerML 8.3.3.3.8: redefining an end requires an end.
        if self.is_end_feature(target) && !self.is_end_feature(source) {
            codes.push(DiagnosticCode::RedefinitionEndMismatch);
        }
        // Only authored directions are compared. Effective direction under port conjugation is not
        // a published fact, so a feature that inherits its direction rather than declaring one
        // leaves this question unanswered rather than answered from a missing operand.
        let direction = |declaration: DeclarationId| {
            self.storage
                .declaration_facts(declaration)
                .and_then(|facts| facts.direction)
        };
        if let (Some(actual), Some(expected)) = (direction(source), direction(target)) {
            if !direction_conforms(actual, expected) {
                codes.push(DiagnosticCode::RedefinitionDirectionMismatch);
            }
        }
        codes
    }

    /// KerML 8.3.3.3: a redefinition is introduced by the featuring type of the redefined feature,
    /// or by one that specializes it.
    ///
    /// A feature owned only by namespaces has no featuring type, so the rule has nothing to compare
    /// and cannot be violated. An indeterminate conformance answer -- a cyclic hierarchy -- also
    /// reports nothing: the obstacle is already its own diagnostic.
    ///
    /// Both sides must be featured by a *type*. A feature nested in another feature -- `snapshot s
    /// { var feature :>> x = 0; }` -- is featured by that enclosing feature, and its effective
    /// featuring type is the enclosing feature's type, which is a further derivation this
    /// publication does not own. Comparing the enclosing feature against a type instead would fail
    /// for every such declaration; the corpus authors 33 of them across the KerML time-varying
    /// fixtures alone, none of them a modelling error.
    pub(crate) fn featuring_structure(
        &self,
        source: DeclarationId,
        target: DeclarationId,
    ) -> Option<DiagnosticCode> {
        let redefining = self.types.featuring_type(source)?;
        let redefined = self.types.featuring_type(target)?;
        if self.is_feature_member(redefining) || self.is_feature_member(redefined) {
            return None;
        }
        match self.conformance(
            redefining,
            redefined,
            SpecializationScope::AnySpecialization,
        ) {
            Conformance::DoesNotConform => {
                Some(DiagnosticCode::RedefinitionFeaturingTypeIncompatible)
            }
            Conformance::Conforms | Conformance::Indeterminate(_) => None,
        }
    }
}
