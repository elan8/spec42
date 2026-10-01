//! Membership owner rules and metadata-feature typing rules, settled at the publication barrier.
//!
//! Each rule reads one canonical fact: the declaration's owner, the published metaclass of a
//! declaration ([`element_kind`] and the metaclass generalization hierarchy
//! [`ElementKind::conforms_to`]), the effective membership role, and settled reference outcomes.
//! A prerequisite that did not settle (an unresolved transition source or metadata type) leaves
//! the rule unanswered; resolution already reports the unresolved reference.

use crate::model::element_kind::element_kind;
use crate::model::resolver::SemanticModel;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::resolve::results::ResolutionError;
use crate::Diagnostic;
use crate::DiagnosticCode;
use crate::DiagnosticSeverity;
use crate::MembershipRole;
use crate::StateSubactionKind;
use sysml_contract::ElementKind;

/// Which result-owning metaclass a type conforms to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResultOwner {
    Function,
    Expression,
}

impl<D> SemanticModel<D> {
    /// The published metaclass of a declaration, when it exists.
    fn metaclass_of(&self, id: DeclarationId) -> Option<ElementKind> {
        self.kind_of(id).map(element_kind)
    }

    /// Whether a declaration's metaclass conforms to `general`.
    fn metaclass_conforms(&self, id: DeclarationId, general: ElementKind) -> bool {
        self.metaclass_of(id)
            .is_some_and(|kind| kind.conforms_to(general))
    }

    /// The owner rules and metadata typing rules of one declaration.
    pub(crate) fn collect_owner_rules(
        &self,
        id: DeclarationId,
        kind: DeclarationKind,
        owner: Option<DeclarationId>,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        // SysML 8.3.26.2 `validateExposeOwningNamespace`: the importOwningNamespace of an
        // Expose is a ViewUsage.
        if kind == DeclarationKind::Expose
            && !owner.is_some_and(|owner| self.metaclass_conforms(owner, ElementKind::ViewUsage))
        {
            diagnostics.push(self.declaration_diagnostic(
                id,
                DiagnosticCode::ExposeInvalidOwner,
                DiagnosticSeverity::Warning,
            )?);
        }
        match self.effective_membership_role(id) {
            // SysML 8.3.24.2 `validateRequirementVerificationMembershipOwningType`: the owning
            // type is a RequirementUsage owned through an ObjectiveMembership by a verification
            // case definition or usage (the Pilot's `UsageUtil.isLegalVerification`).
            Some(MembershipRole::RequirementVerification) => {
                let legal = owner.is_some_and(|objective| {
                    self.metaclass_conforms(objective, ElementKind::RequirementUsage)
                        && self.effective_membership_role(objective)
                            == Some(MembershipRole::Objective)
                        && self
                            .storage
                            .declaration(objective)
                            .and_then(|objective| objective.owner)
                            .is_some_and(|case| {
                                self.metaclass_conforms(
                                    case,
                                    ElementKind::VerificationCaseDefinition,
                                ) || self
                                    .metaclass_conforms(case, ElementKind::VerificationCaseUsage)
                            })
                });
                if !legal {
                    diagnostics.push(self.declaration_diagnostic(
                        id,
                        DiagnosticCode::VerificationMembershipInvalidOwner,
                        DiagnosticSeverity::Warning,
                    )?);
                }
            }
            // SysML 8.3.21.7 `validateRequirementConstraintMembershipOwningType`.
            Some(MembershipRole::RequirementConstraint(_)) => {
                if !owner.is_some_and(|owner| {
                    self.metaclass_conforms(owner, ElementKind::RequirementDefinition)
                        || self.metaclass_conforms(owner, ElementKind::RequirementUsage)
                }) {
                    diagnostics.push(self.declaration_diagnostic(
                        id,
                        DiagnosticCode::RequirementConstraintInvalidOwner,
                        DiagnosticSeverity::Warning,
                    )?);
                }
            }
            // SysML 8.3.18.9 `validateTransitionUsageTriggerActions`: a transition owning a
            // trigger membership has a StateUsage source. Reported at the trigger member, as the
            // Pilot reports it at the trigger TransitionFeatureMembership.
            Some(MembershipRole::TransitionTriggerAction) => {
                if let Some(transition) = owner {
                    if let Some(source) = self
                        .settled_targets(transition, &[ReferenceKind::TransitionSource])
                        .first()
                    {
                        if !self.metaclass_conforms(*source, ElementKind::StateUsage) {
                            diagnostics.push(self.declaration_diagnostic(
                                id,
                                DiagnosticCode::TransitionTriggerSourceNotState,
                                DiagnosticSeverity::Warning,
                            )?);
                        }
                    }
                }
            }
            _ => {}
        }
        // KerML 8.3.4.7.8 `validateReturnParameterMembershipOwningType`.
        if self.effective_membership_role(id) == Some(MembershipRole::ReturnParameter)
            && !owner.is_some_and(|owner| {
                self.metaclass_conforms(owner, ElementKind::Function)
                    || self.metaclass_conforms(owner, ElementKind::Expression)
            })
        {
            diagnostics.push(self.declaration_diagnostic(
                id,
                DiagnosticCode::ReturnParameterMembershipInvalidOwner,
                DiagnosticSeverity::Warning,
            )?);
        }
        // KerML 8.3.4.7.4 `validateFunctionResultParameterMembership` and 8.3.4.7.3
        // `validateExpressionResultParameterMembership`, as the Pilot checks them: at most one
        // owned ReturnParameterMembership, every one after the first reported. A Function or
        // Expression that owns none inherits its result from its general type.
        let result_owner = if self.metaclass_conforms(id, ElementKind::Function) {
            Some(ResultOwner::Function)
        } else if self.metaclass_conforms(id, ElementKind::Expression) {
            Some(ResultOwner::Expression)
        } else {
            None
        };
        if let Some(result_owner) = result_owner {
            for member in self
                .child_declarations(id)
                .iter()
                .copied()
                .filter(|member| {
                    self.effective_membership_role(*member) == Some(MembershipRole::ReturnParameter)
                })
                .skip(1)
            {
                diagnostics.push(self.declaration_diagnostic(
                    member,
                    match result_owner {
                        ResultOwner::Function => DiagnosticCode::FunctionResultParameterCount,
                        ResultOwner::Expression => DiagnosticCode::ExpressionResultParameterCount,
                    },
                    DiagnosticSeverity::Warning,
                )?);
            }
        }
        // SysML 8.3.18.5 `validateStateDefinitionStateSubactionKind` and 8.3.18.6
        // `validateStateUsageStateSubactionKind`: a state owns at most one StateSubactionMembership
        // of each kind. As the Pilot reports it, every membership after the first of a kind is
        // reported.
        if self.metaclass_conforms(id, ElementKind::StateDefinition)
            || self.metaclass_conforms(id, ElementKind::StateUsage)
        {
            for subaction_kind in [
                StateSubactionKind::Entry,
                StateSubactionKind::Do,
                StateSubactionKind::Exit,
            ] {
                for member in self
                    .child_declarations(id)
                    .iter()
                    .copied()
                    .filter(|member| {
                        self.effective_membership_role(*member)
                            == Some(MembershipRole::StateSubaction(subaction_kind))
                    })
                    .skip(1)
                {
                    diagnostics.push(self.declaration_diagnostic(
                        member,
                        DiagnosticCode::StateDuplicateSubactionKind,
                        DiagnosticSeverity::Warning,
                    )?);
                }
            }
        }
        // SysML 8.3.20.2 `validateAssertConstraintUsageReference`: the feature an
        // `assert <path>;` reference-subsets is a ConstraintUsage.
        if self.metaclass_conforms(id, ElementKind::AssertConstraintUsage) {
            self.collect_reference_subsetting_type(
                id,
                ElementKind::ConstraintUsage,
                DiagnosticCode::AssertTargetInvalidKind,
                diagnostics,
            )?;
        }
        if kind == DeclarationKind::MetadataUsage {
            self.collect_metadata_feature_typing(id, diagnostics)?;
        }
        Ok(())
    }

    /// The Pilot's `checkReferenceType`: when a usage owns a ReferenceSubsetting, its referenced
    /// feature conforms to `required`. Reported at the reference subsetting; an unsettled target
    /// leaves the rule unanswered, since resolution already reports it.
    fn collect_reference_subsetting_type(
        &self,
        id: DeclarationId,
        required: ElementKind,
        code: DiagnosticCode,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for (reference_id, reference) in self.authored_references(id, &[ReferenceKind::References])
        {
            let Some(target) = self.settled_target(reference_id) else {
                continue;
            };
            if !self.metaclass_conforms(target, required) {
                diagnostics.push(self.reference_diagnostic(
                    reference,
                    code.clone(),
                    DiagnosticSeverity::Warning,
                    Some(target),
                )?);
            }
        }
        Ok(())
    }

    /// KerML 8.3.4.12.3 `validateMetadataFeatureMetaclass` and
    /// `validateMetadataFeatureMetaclassNotAbstract`, over the metadata feature's authored typing
    /// (`metadata x : T`, `@T`, `#T`). An unsettled typing leaves both rules unanswered.
    fn collect_metadata_feature_typing(
        &self,
        id: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let typings = self.authored_references(
            id,
            &[
                ReferenceKind::FeatureTyping,
                ReferenceKind::MetadataAnnotation,
            ],
        );
        // The grammar requires a typing on every metadata feature. None is authored only for
        // the `metadata T about x;` shorthand, which the pinned parser reads as a declared name
        // (planning/UPSTREAM_PARSER_GAPS.md `metadata-usage-shorthand`); the type is then
        // unknown rather than absent.
        if typings.is_empty() {
            return Ok(());
        }
        let mut types = Vec::with_capacity(typings.len());
        for (reference, _) in typings {
            let Some(target) = self.settled_target(reference) else {
                return Ok(());
            };
            types.push(target);
        }
        let metaclasses = types
            .iter()
            .filter(|target| self.metaclass_conforms(**target, ElementKind::Metaclass))
            .count();
        if metaclasses != 1 {
            diagnostics.push(self.declaration_diagnostic(
                id,
                DiagnosticCode::MetadataTypeNotMetaclass,
                DiagnosticSeverity::Warning,
            )?);
        }
        let abstract_type = types.iter().any(|target| {
            match (
                self.storage.declaration(*target),
                self.storage.declaration_facts(*target),
            ) {
                (Some(declaration), Some(facts)) => {
                    facts.modifiers.effectively_abstract(declaration.kind)
                }
                _ => false,
            }
        });
        if abstract_type {
            diagnostics.push(self.declaration_diagnostic(
                id,
                DiagnosticCode::MetadataMetaclassAbstract,
                DiagnosticSeverity::Warning,
            )?);
        }
        Ok(())
    }
}
