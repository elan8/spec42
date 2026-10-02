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

use crate::check::conformance;
use crate::index::expressions::conforms;
use crate::index::expressions::RequiredMeasurement;
use crate::index::expressions::UnitOutcome;
use crate::index::types::TypeIndex;
use crate::lower::facts::ExpressionOperandRole;
use crate::lower::facts::FilterForm;
use crate::lower::facts::ParameterDirection;
use crate::model::element_kind::element_kind;
use crate::model::render as writer;
use crate::model::resolver::SemanticModel;
use crate::model::span::document_range;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::ReferenceKind;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::Diagnostic;
use crate::DiagnosticCode;
use crate::DiagnosticLocation;
use crate::DiagnosticOrigin;
use crate::DiagnosticSeverity;
use sysml_contract::ElementKind;

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
pub(crate) fn comparable(types: &TypeIndex, left: DeclarationId, right: DeclarationId) -> bool {
    conforms(types, left, right) || conforms(types, right, left)
}

impl<D> SemanticModel<D> {
    /// Appends every expression-conformance diagnostic authored in `document`.
    ///
    /// Ordering is the caller's: [`Self::derive_diagnostics`] sorts each document's diagnostics by
    /// range and code once every producer has contributed.
    pub(crate) fn collect_expression_conformance(
        &self,
        document: DocumentIdx,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        self.collect_value_conformance(document, diagnostics)?;
        self.collect_assignment_time_variation(document, diagnostics)?;
        self.collect_unit_conformance(document, diagnostics)?;
        self.collect_boolean_expressions(document, declared, diagnostics)?;
        self.collect_invocation_arity(document, diagnostics)?;
        self.collect_invocation_instantiated_types(document, diagnostics)?;
        self.collect_feature_reference_referents(document, diagnostics)?;
        self.collect_instantiation_argument_redefinitions(document, diagnostics)?;
        self.collect_trigger_invocation_arguments(document, diagnostics)?;
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
        document: DocumentIdx,
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
                payload: None,
                message: DiagnosticCode::AttributeValueTypeIncompatible
                    .describe()
                    .into(),
                code: DiagnosticCode::AttributeValueTypeIncompatible,
                severity: DiagnosticSeverity::Error,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(value.declaration),
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
                payload: None,
                message: DiagnosticCode::AssignmentValueIncompatible
                    .describe()
                    .into(),
                code: DiagnosticCode::AssignmentValueIncompatible,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(reference.source),
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

    /// Reports an assignment whose referent cannot have time-varying values.
    ///
    /// SysML `validateAssignmentActionUsage` is `referent <> null implies
    /// referent.featureTarget.isVariable`. The referent is the settled `AssignTarget` reference
    /// (a qualified or dotted target already resolves to its featureTarget), and `isVariable` is
    /// the canonical [`crate::index::types::TypeIndex::feature_is_variable`] fact. A target that
    /// is unresolved, ambiguous, not a Feature, or whose variability depends on an unresolved
    /// library anchor is left unanswered rather than reported.
    pub(crate) fn collect_assignment_time_variation(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
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
            let Some(ResolutionStatus::Resolved(target)) = self.resolution.outcome(id) else {
                continue;
            };
            if !matches!(
                self.types.feature_is_variable(&self.storage, target),
                Some(crate::index::types::UsageTimeVariationOutcome::Resolved(
                    false
                ))
            ) {
                continue;
            }
            diagnostics.push(Diagnostic {
                payload: None,
                message: DiagnosticCode::AssignmentTargetNotTimeVarying
                    .describe()
                    .into(),
                code: DiagnosticCode::AssignmentTargetNotTimeVarying,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(reference.source),
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
        !declared.any(|target| comparable(&self.types, target, value_type))
    }

    /// Reports unit tokens that name no unit, name several, or name one of the wrong dimension.
    pub(crate) fn collect_unit_conformance(
        &self,
        document: DocumentIdx,
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
                    payload: None,
                    message: DiagnosticCode::UnknownUnitSymbol.describe().into(),
                    code: DiagnosticCode::UnknownUnitSymbol,
                    severity: DiagnosticSeverity::Warning,
                    origin: DiagnosticOrigin::Semantic,
                    subject: self.symbol_id(unit.declaration),
                    location,
                    related: Box::default(),
                }),
                UnitOutcome::Ambiguous(candidates) => {
                    let mut related = Vec::with_capacity(candidates.len());
                    for candidate in candidates.iter() {
                        related.push(self.related_declaration(*candidate, RELATED_UNIT_CANDIDATE)?);
                    }
                    diagnostics.push(Diagnostic {
                        payload: None,
                        message: DiagnosticCode::AmbiguousUnitSymbol.describe().into(),
                        code: DiagnosticCode::AmbiguousUnitSymbol,
                        severity: DiagnosticSeverity::Warning,
                        origin: DiagnosticOrigin::Semantic,
                        subject: self.symbol_id(unit.declaration),
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
                    let receiver = self
                        .storage
                        .declaration_facts(unit.declaration)
                        .filter(|facts| facts.expression_result.is_some())
                        .and_then(|_| self.storage.declaration(unit.declaration))
                        .and_then(|declaration| declaration.owner)
                        .unwrap_or(unit.declaration);
                    let RequiredMeasurement::Required(expected) =
                        self.expressions.required_measurement(receiver)
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
                            .any(|want| conforms(&self.types, *dimension, *want))
                    }) {
                        continue;
                    }
                    let mut related = Vec::with_capacity(expected.len());
                    for want in expected.iter() {
                        related.push(self.related_declaration(*want, RELATED_EXPECTED_DIMENSION)?);
                    }
                    diagnostics.push(Diagnostic {
                        payload: None,
                        message: DiagnosticCode::IncompatibleUnitDimension.describe().into(),
                        code: DiagnosticCode::IncompatibleUnitDimension,
                        severity: DiagnosticSeverity::Warning,
                        origin: DiagnosticOrigin::Semantic,
                        subject: self.symbol_id(receiver),
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
        document: DocumentIdx,
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
            // Every form is an ElementFilterMembership, whose condition must be model-level
            // evaluable; only a settled `NotEvaluable` is reported.
            if crate::index::expressions::model_level_evaluability(
                &self.storage,
                &self.types,
                &self.expressions.anchors,
                &filter.evaluability,
            ) == crate::index::expressions::ModelLevelEvaluability::NotEvaluable
            {
                let code = DiagnosticCode::FilterConditionNotModelLevelEvaluable;
                diagnostics.push(Diagnostic {
                    payload: None,
                    message: code.describe().into(),
                    code,
                    severity: DiagnosticSeverity::Warning,
                    origin: DiagnosticOrigin::Semantic,
                    subject: self.symbol_id(filter.owner),
                    location: DiagnosticLocation {
                        document: writer::document_identity(self, document).into(),
                        range: document_range(&self.storage, document, &filter.span)?,
                    },
                    related: Box::default(),
                });
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
                payload: None,
                message: code.describe().into(),
                code,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(filter.owner),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &filter.span)?,
                },
                related: Box::default(),
            });
        }
        Ok(())
    }

    /// KerML 8.3.4.8.8 `validateInvocationExpressionParameterRedefinition` /
    /// `validateInvocationExpressionNoDuplicateParameterRedefinition` and KerML 8.3.4.8.3
    /// `validateConstructorExpressionNoDuplicateFeatureRedefinition` over named arguments.
    ///
    /// A named argument's Feature owns the authored Redefinition of the parameter it names, which
    /// resolves among the members of the instantiated type. For an invocation that target must be
    /// an input (`in`/`inout`) parameter; two arguments of one container must not redefine the
    /// same target. A positional argument's redefinition is implied by its position
    /// (`checkFeatureParameterRedefinition`), which this publication does not derive, so it is
    /// left unanswered, as is a named argument whose parameter did not settle.
    pub(crate) fn collect_instantiation_argument_redefinitions(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        use crate::lower::facts::InstantiationForm;
        let mut parameters = std::collections::BTreeMap::new();
        for (index, reference) in self.storage.references.iter().enumerate() {
            if reference.kind == ReferenceKind::Redefinition
                && crate::resolve::root_narrowing(reference).is_some()
            {
                let id = AuthoredReferenceId::from_index(index)
                    .map_err(|_| ResolutionError::Capacity)?;
                parameters.insert(reference.source, id);
            }
        }
        let mut bound: std::collections::BTreeSet<(DeclarationId, DeclarationId)> =
            std::collections::BTreeSet::new();
        for (index, declaration) in self.storage.declarations.iter().enumerate() {
            if declaration.document != document {
                continue;
            }
            let Some(argument) = self.storage.declaration_facts[index].instantiation_argument
            else {
                continue;
            };
            if !argument.named {
                continue;
            }
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let (Some(container), Some(reference)) = (declaration.owner, parameters.get(&id))
            else {
                return Err(ResolutionError::InvalidStorage);
            };
            let Some(ResolutionStatus::Resolved(parameter)) = self.resolution.outcome(*reference)
            else {
                continue;
            };
            let code = if argument.form == InstantiationForm::Invocation
                && !matches!(
                    crate::model::element_kind::effective_direction(
                        self.effective_membership_role(parameter),
                        self.storage
                            .declaration_facts(parameter)
                            .and_then(|facts| facts.direction),
                    ),
                    Some(ParameterDirection::In) | Some(ParameterDirection::InOut)
                ) {
                DiagnosticCode::InvocationArgumentRedefinesNoParameter
            } else if !bound.insert((container, parameter)) {
                match argument.form {
                    InstantiationForm::Invocation => {
                        DiagnosticCode::InvocationDuplicateParameterRedefinition
                    }
                    InstantiationForm::Constructor => {
                        DiagnosticCode::ConstructorDuplicateFeatureRedefinition
                    }
                }
            } else {
                continue;
            };
            diagnostics.push(Diagnostic {
                payload: None,
                message: code.describe().into(),
                code,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(id),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &declaration.span)?,
                },
                related: Box::from([
                    self.related_declaration(parameter, conformance::RELATED_DECLARED)?
                ]),
            });
        }
        Ok(())
    }

    /// KerML 8.3.4.8.5 `validateFeatureReferenceExpressionReferentIsFeature`: the referent of a
    /// `FeatureReferenceExpression` is a Feature.
    ///
    /// Only an `ExpressionOperand` lowered in feature-reference position
    /// ([`ExpressionOperandRole::FeatureReference`]) is such an expression; the operand of `meta`,
    /// a `->f g` function reference and an `accept T` payload type legitimately name other
    /// elements. An unresolved or ambiguous referent is its own diagnostic and is left unanswered.
    pub(crate) fn collect_feature_reference_referents(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for (index, reference) in self.storage.references.iter().enumerate() {
            if reference.kind != ReferenceKind::ExpressionOperand
                || reference.flags.operand_role != Some(ExpressionOperandRole::FeatureReference)
                || reference.target.document != document
            {
                continue;
            }
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let Some(ResolutionStatus::Resolved(target)) = self.resolution.outcome(id) else {
                continue;
            };
            let Some(kind) = self.kind_of(target).map(element_kind) else {
                return Err(ResolutionError::InvalidStorage);
            };
            if kind.conforms_to(ElementKind::Feature) {
                continue;
            }
            diagnostics.push(Diagnostic {
                payload: None,
                message: DiagnosticCode::FeatureReferenceReferentNotFeature
                    .describe()
                    .into(),
                code: DiagnosticCode::FeatureReferenceReferentNotFeature,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(reference.source),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &reference.span)?,
                },
                related: Box::from([
                    self.related_declaration(target, conformance::RELATED_DECLARED)?
                ]),
            });
        }
        Ok(())
    }

    /// KerML 8.3.4.8.8 `validateInvocationExpressionInstantiatedType`: the instantiated type of
    /// an invocation is a Behavior, or a Feature typed by a Behavior.
    ///
    /// A Feature callee's types are its canonical non-redundant type set
    /// ([`TypeIndex::feature_types`], the Pilot's `Feature::type`). The Pilot additionally requires
    /// that set to be a single type; this rule accepts any set containing a Behavior. A Feature
    /// callee with no settled type -- an unresolved typing, or the implied library typing of an
    /// untyped step when the library is not admitted -- is left unanswered rather than reported.
    pub(crate) fn collect_invocation_instantiated_types(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for invocation in self.expressions.invocations().iter() {
            if invocation.document != document {
                continue;
            }
            let Some(kind) = self.kind_of(invocation.callee).map(element_kind) else {
                return Err(ResolutionError::InvalidStorage);
            };
            if kind.conforms_to(ElementKind::Behavior) {
                continue;
            }
            if kind.conforms_to(ElementKind::Feature) {
                if self.specialization_hierarchy_is_unsettled(invocation.callee) {
                    continue;
                }
                let types = self.types.feature_types(invocation.callee);
                if types.is_empty()
                    || types.iter().any(|general| {
                        self.kind_of(*general).is_some_and(|kind| {
                            element_kind(kind).conforms_to(ElementKind::Behavior)
                        })
                    })
                {
                    continue;
                }
            }
            diagnostics.push(Diagnostic {
                payload: None,
                message: DiagnosticCode::InvocationInstantiatedTypeNotBehavior
                    .describe()
                    .into(),
                code: DiagnosticCode::InvocationInstantiatedTypeNotBehavior,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(invocation.declaration),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, document).into(),
                    range: document_range(&self.storage, document, &invocation.span)?,
                },
                related: Box::from([self.related_declaration(invocation.callee, RELATED_CALLEE)?]),
            });
        }
        Ok(())
    }

    /// SysML 8.3.17.17 `validateTriggerInvocationExpressionWhenArgument`, `...AtArgument` and
    /// `...AfterArgument`: the argument of a `when` trigger is Boolean, of an `at` trigger a
    /// `Time::TimeInstantValue`, of an `after` trigger an `ISQBase::DurationValue`.
    ///
    /// The argument's result type is known for a literal (its `LiteralExpression` type) and for a
    /// feature reference (the settled feature's canonical types). Any other argument -- a quantity
    /// with a unit, an operator, an invocation -- has a result type this publication does not
    /// derive, and is left unanswered, as is a reference that did not settle, a feature whose
    /// types or specializations did not, and a library type this publication does not admit.
    pub(crate) fn collect_trigger_invocation_arguments(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        use crate::lower::facts::LiteralKind;
        use crate::lower::facts::TriggerArgument;
        use crate::lower::facts::TriggerInvocationKind;
        use crate::resolve::implied::resolve_library_specialization_anchor;
        use crate::resolve::implied::LibrarySpecializationAnchor;
        let triggers = self
            .storage
            .trigger_invocations
            .iter()
            .filter(|trigger| {
                self.storage
                    .declaration(trigger.expression)
                    .is_some_and(|declaration| declaration.document == document)
            })
            .collect::<Vec<_>>();
        if triggers.is_empty() {
            return Ok(());
        }
        let anchor = |path| match resolve_library_specialization_anchor(&self.storage, path) {
            LibrarySpecializationAnchor::Resolved(anchor) => Some(anchor),
            LibrarySpecializationAnchor::Missing | LibrarySpecializationAnchor::Ambiguous(_) => {
                None
            }
        };
        let boolean = anchor("ScalarValues::Boolean");
        let time_instant = anchor("Time::TimeInstantValue");
        let duration = anchor("ISQBase::DurationValue");
        for trigger in triggers {
            let (expected, code) = match trigger.kind {
                TriggerInvocationKind::When => {
                    (boolean, DiagnosticCode::TriggerWhenArgumentNotBoolean)
                }
                TriggerInvocationKind::At => (
                    time_instant,
                    DiagnosticCode::TriggerAtArgumentNotTimeInstant,
                ),
                TriggerInvocationKind::After => {
                    (duration, DiagnosticCode::TriggerAfterArgumentNotDuration)
                }
            };
            let conforms = match trigger.argument {
                // A Boolean literal is a `LiteralBoolean`; no other literal's type is Boolean, a
                // time instant or a duration, whatever the library admits.
                TriggerArgument::Literal(kind) => Some(
                    kind == LiteralKind::Boolean && trigger.kind == TriggerInvocationKind::When,
                ),
                TriggerArgument::FeatureReference(reference) => {
                    match (self.resolution.outcome(reference), expected) {
                        (Some(ResolutionStatus::Resolved(feature)), Some(expected))
                            if self.kind_of(feature).is_some_and(|kind| {
                                element_kind(kind).conforms_to(ElementKind::Feature)
                            }) && !self.specialization_hierarchy_is_unsettled(feature) =>
                        {
                            let types = self.types.feature_types(feature);
                            let mut verdict = if types.is_empty() { None } else { Some(false) };
                            for general in types {
                                match self.conformance(
                                    general,
                                    expected,
                                    crate::SpecializationScope::AnySpecialization,
                                ) {
                                    crate::Conformance::Conforms => {
                                        verdict = Some(true);
                                        break;
                                    }
                                    crate::Conformance::DoesNotConform => {}
                                    crate::Conformance::Indeterminate(_) => verdict = None,
                                }
                            }
                            verdict
                        }
                        _ => None,
                    }
                }
                TriggerArgument::Other => None,
            };
            if conforms == Some(false) {
                diagnostics.push(self.declaration_diagnostic(
                    trigger.expression,
                    code,
                    DiagnosticSeverity::Warning,
                )?);
            }
        }
        Ok(())
    }

    /// Reports a calculation invocation that binds fewer arguments than the callee has parameters.
    pub(crate) fn collect_invocation_arity(
        &self,
        document: DocumentIdx,
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
                payload: None,
                message: DiagnosticCode::CalculationArgumentsIncomplete
                    .describe()
                    .into(),
                code: DiagnosticCode::CalculationArgumentsIncomplete,
                severity: DiagnosticSeverity::Warning,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(invocation.declaration),
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
