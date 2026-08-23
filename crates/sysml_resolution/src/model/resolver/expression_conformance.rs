//! The expression-conformance rules, decided from settled facts at the publication barrier.
//!
//! Every rule here reads what [`super::expression`] settled -- an evaluation state, a unit's
//! resolution, the measurement reference a type requires -- and asks a specialization question
//! about it. None of them reads authored text, compares a type by name, or re-resolves anything.
//!
//! # What the legacy check did instead
//!
//! The deleted graph-backed expression check decided a feature's expected type by
//! testing whether its authored type reference ended in `::Boolean`, decided a value's type by
//! testing whether the value text spelled `true`, matched enumeration values by comparing a string
//! literal against member names, and found units by searching the whole graph for a node whose
//! name ended in `Unit`. Those inferences agree with the model often enough to look right and are
//! wrong exactly where a model is unusual -- an inherited type, a renamed library, a value that is
//! a computed constant rather than a literal.
//!
//! # Why comparability, not conformance
//!
//! KerML types the literal `3` as `ScalarValues::Integer`, and a feature declared `Natural` is
//! *narrower* than that. Asking "does the value's type conform to the declared type" would report
//! every authored natural number; asking the reverse would report every widening. What is
//! genuinely wrong is a value from an unrelated branch of the hierarchy, which is exactly a pair
//! with no specialization path between them in either direction.

use super::expression::{conforms, RequiredMeasurement, UnitOutcome};
use super::*;
use crate::evaluation::EvaluatedScalar;

/// The note attached to each admitted unit an ambiguous token could have named.
pub(crate) const RELATED_UNIT_CANDIDATE: &str = "Admitted unit with this symbol.";
/// The note attached to each measurement reference the feature's type admits.
pub(crate) const RELATED_EXPECTED_DIMENSION: &str =
    "Measurement reference the feature's type admits.";
/// The note attached to the calculation an incomplete invocation calls.
pub(crate) const RELATED_CALLEE: &str = "Calculation invoked here.";

/// Whether two types are comparable: one is the other, or one specialises the other.
///
/// The value rules ask this rather than "does the value's type conform to the declared type".
/// KerML types a literal `3` as `ScalarValues::Integer`, and a feature declared `Natural` is
/// narrower than that, so a conformance test in either single direction would report every
/// authored natural number. What is genuinely wrong is a value from an unrelated branch of the
/// type hierarchy -- a string where a number belongs, a boolean where a mass does -- and those are
/// exactly the pairs with no path between them.
pub(crate) fn comparable(
    model: &ResolvedSemanticModel,
    left: DeclarationId,
    right: DeclarationId,
) -> bool {
    conforms(model, left, right) || conforms(model, right, left)
}

impl ResolvedSemanticModel {
    /// Appends every expression-conformance diagnostic authored in `document`.
    ///
    /// Ordering is the caller's: [`Self::derive_diagnostics`] sorts each document's diagnostics by
    /// range and code once every producer has contributed.
    pub(crate) fn collect_expression_conformance(
        &self,
        document: DocumentId,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        self.collect_value_conformance(document, diagnostics)?;
        self.collect_unit_conformance(document, diagnostics)?;
        self.collect_boolean_expressions(document, declared, diagnostics)?;
        self.collect_invocation_arity(document, diagnostics)?;
        Ok(())
    }

    /// The evaluated value one declaration settled to, if it settled to one at all.
    pub(crate) fn evaluated_value(&self, declaration: DeclarationId) -> Option<EvaluatedScalar> {
        self.evaluation_for(declaration).value().cloned()
    }

    /// Reports an authored value whose type is unrelated to the feature it is assigned to.
    ///
    /// Two authored forms, one rule: `attribute mass : Mass = "heavy";` binds a value to the
    /// feature it is declared on, and `assign target := "heavy";` binds one to the feature its
    /// left-hand side resolves to. Both compare the same two facts -- what the value evaluated to,
    /// and what the receiving feature is typed by -- so they differ only in where the receiving
    /// feature comes from and where the diagnostic is reported.
    ///
    /// A quantity-valued feature is deliberately excluded: its values are checked by dimension,
    /// which is what a measurement reference means, and comparing a mass against the datatype of
    /// its magnitude would report every unit-bearing value in the model.
    pub(crate) fn collect_value_conformance(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for value in self.storage.feature_values.iter() {
            let declaration = self
                .storage
                .declaration(value.declaration)
                .ok_or(ResolutionError::InvalidStorage)?;
            if declaration.document != document {
                continue;
            }
            if !self.value_type_conflicts(value.declaration, value.declaration) {
                continue;
            }
            diagnostics.push(Diagnostic {
                message: DiagnosticCode::AttributeValueTypeIncompatible
                    .describe()
                    .into(),
                code: DiagnosticCode::AttributeValueTypeIncompatible,
                severity: DiagnosticSeverity::Error,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_identity(value.declaration),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &value.span)?,
                },
                related: Box::default(),
            });
        }

        for (index, reference) in self.storage.references.iter().enumerate() {
            if reference.kind != ReferenceKind::AssignTarget {
                continue;
            }
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if source.document != document {
                continue;
            }
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            // Only a settled target carries a type to judge. An unresolved or ambiguous left-hand
            // side is already its own published diagnostic.
            let Some(ResolutionStatus::Resolved(target)) = self.resolution.outcome(id) else {
                continue;
            };
            if !self.value_type_conflicts(reference.source, target) {
                continue;
            }
            diagnostics.push(Diagnostic {
                message: DiagnosticCode::AssignmentValueIncompatible
                    .describe()
                    .into(),
                code: DiagnosticCode::AssignmentValueIncompatible,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_identity(reference.source),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &source.span)?,
                },
                related: Box::from([
                    self.related_declaration(target, conformance::RELATED_DECLARED)?
                ]),
            });
        }
        Ok(())
    }

    /// Whether `expression`'s settled value has a type unrelated to every type `receiver` has.
    ///
    /// Answers `false` for every state that is not a settled value, for a value whose datatype the
    /// admitted libraries do not declare, and for a receiver with no effective type: each of those
    /// is a question this publication cannot answer, and answering it as "incompatible" would
    /// report the absence of an input as a fault in the model.
    pub(crate) fn value_type_conflicts(
        &self,
        expression: DeclarationId,
        receiver: DeclarationId,
    ) -> bool {
        if !matches!(
            self.expressions.required_measurement(receiver),
            RequiredMeasurement::NotApplicable
        ) {
            return false;
        }
        let Some(value) = self.evaluated_value(expression) else {
            return false;
        };
        let Some(value_type) = self.expressions.scalar_type(&value) else {
            return false;
        };
        let mut declared = self
            .types
            .effective_types(receiver)
            .iter()
            .map(|(target, _)| *target)
            .peekable();
        if declared.peek().is_none() {
            return false;
        }
        !declared.any(|target| comparable(self, target, value_type))
    }

    /// Reports unit tokens that name no unit, name several, or name one of the wrong dimension.
    pub(crate) fn collect_unit_conformance(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for unit in self.expressions.all_units().iter() {
            if unit.document != document {
                continue;
            }
            let location = DiagnosticLocation {
                document: writer::document_identity(self, document).into(),
                range: document_range(&self.storage, document, &unit.span)?,
            };
            match &unit.outcome {
                UnitOutcome::UnknownSymbol => diagnostics.push(Diagnostic {
                    message: DiagnosticCode::UnknownUnitSymbol.describe().into(),
                    code: DiagnosticCode::UnknownUnitSymbol,
                    severity: DiagnosticSeverity::Warning,
                    origin: DiagnosticOrigin::Semantic,
                    subject: self.symbol_identity(unit.declaration),
                    location,
                    related: Box::default(),
                }),
                UnitOutcome::Ambiguous(candidates) => {
                    let mut related = Vec::with_capacity(candidates.len());
                    for candidate in candidates.iter() {
                        related.push(self.related_declaration(*candidate, RELATED_UNIT_CANDIDATE)?);
                    }
                    diagnostics.push(Diagnostic {
                        message: DiagnosticCode::AmbiguousUnitSymbol.describe().into(),
                        code: DiagnosticCode::AmbiguousUnitSymbol,
                        severity: DiagnosticSeverity::Warning,
                        origin: DiagnosticOrigin::Semantic,
                        subject: self.symbol_identity(unit.declaration),
                        location,
                        related: related.into_boxed_slice(),
                    });
                }
                // A unit this layer cannot decode, and a publication with no catalog to decode it
                // against, are published states rather than faults in the model: the first is a
                // parser contract this engine does not extend, the second an input the workspace
                // did not supply.
                UnitOutcome::UnsupportedExpression | UnitOutcome::CatalogUnavailable => {}
                UnitOutcome::Resolved { dimensions, .. } => {
                    let RequiredMeasurement::Required(expected) =
                        self.expressions.required_measurement(unit.declaration)
                    else {
                        continue;
                    };
                    // Only a value that is itself a quantity is measured by this token. An
                    // expression that merely mentions a unit somewhere -- `mass > 0 [kg]` -- states
                    // a comparison, not the feature's own value.
                    if !matches!(
                        self.evaluated_value(unit.declaration),
                        Some(EvaluatedScalar::Quantity { .. })
                    ) {
                        continue;
                    }
                    if dimensions.iter().any(|dimension| {
                        expected
                            .iter()
                            .any(|want| conforms(self, *dimension, *want))
                    }) {
                        continue;
                    }
                    let mut related = Vec::with_capacity(expected.len());
                    for want in expected.iter() {
                        related.push(self.related_declaration(*want, RELATED_EXPECTED_DIMENSION)?);
                    }
                    diagnostics.push(Diagnostic {
                        message: DiagnosticCode::IncompatibleUnitDimension.describe().into(),
                        code: DiagnosticCode::IncompatibleUnitDimension,
                        severity: DiagnosticSeverity::Warning,
                        origin: DiagnosticOrigin::Semantic,
                        subject: self.symbol_identity(unit.declaration),
                        location,
                        related: related.into_boxed_slice(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Reports constraint bodies and view filters that evaluate to something other than a Boolean.
    ///
    /// Both rules read the settled evaluation rather than the authored text. Only a state that
    /// carries a value can be judged: an expression that is not constant, could not be folded, or
    /// is outside the evaluated slice has no Boolean-ness to report, and calling it non-Boolean
    /// would turn every limit of this evaluator into a fault in the model.
    pub(crate) fn collect_boolean_expressions(
        &self,
        document: DocumentId,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            if !states_a_constraint(declaration.kind) {
                continue;
            }
            if !matches!(
                self.evaluated_value(id),
                Some(value) if !matches!(value, EvaluatedScalar::Boolean(_))
            ) {
                continue;
            }
            diagnostics.push(self.declaration_diagnostic(
                id,
                DiagnosticCode::NonBooleanConstraintExpression,
                DiagnosticSeverity::Warning,
            )?);
        }

        for filter in self.expressions.filters().iter() {
            if filter.document != document {
                continue;
            }
            // A view filter and a package-level import filter are the same Boolean question at two
            // sites, and each keeps its own code because a consumer suppresses them separately.
            let code = match filter.form {
                FilterForm::View => DiagnosticCode::NonBooleanViewFilter,
                FilterForm::PackageImport => DiagnosticCode::InvalidImportFilter,
                FilterForm::Rendering => continue,
            };
            let Some(value) = filter.state.value() else {
                continue;
            };
            if matches!(value, EvaluatedScalar::Boolean(_)) {
                continue;
            }
            diagnostics.push(Diagnostic {
                message: code.describe().into(),
                code,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_identity(filter.owner),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &filter.span)?,
                },
                related: Box::default(),
            });
        }
        Ok(())
    }

    /// Reports a calculation invocation that binds fewer arguments than the callee has parameters.
    pub(crate) fn collect_invocation_arity(
        &self,
        document: DocumentId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for invocation in self.expressions.invocations().iter() {
            if invocation.document != document || invocation.supplied >= invocation.required {
                continue;
            }
            let callee = self
                .storage
                .declaration(invocation.callee)
                .ok_or(ResolutionError::InvalidStorage)?;
            // Only a calculation binds its parameters positionally at the call site. A part or an
            // attribute invoked as a constructor binds its features by name in a body instead, and
            // counting its features as unbound arguments would report every constructor.
            if !matches!(
                callee.kind,
                DeclarationKind::CalcDefinition | DeclarationKind::CalcUsage
            ) {
                continue;
            }
            diagnostics.push(Diagnostic {
                message: DiagnosticCode::CalculationArgumentsIncomplete
                    .describe()
                    .into(),
                code: DiagnosticCode::CalculationArgumentsIncomplete,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_identity(invocation.declaration),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &invocation.span)?,
                },
                related: Box::from([self.related_declaration(invocation.callee, RELATED_CALLEE)?]),
            });
        }
        Ok(())
    }
}

/// Whether a declaration states a constraint whose expression must be Boolean.
///
/// The definition and usage of a constraint, and the three ways a usage is asserted. A requirement
/// is deliberately absent even though it specialises a constraint: its body states subject,
/// stakeholders and nested requirements rather than one Boolean expression.
pub(crate) fn states_a_constraint(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ConstraintDefinition
            | DeclarationKind::ConstraintUsage
            | DeclarationKind::AssertConstraintUsage
            | DeclarationKind::AssumeConstraintUsage
            | DeclarationKind::RequireConstraintUsage
    )
}
