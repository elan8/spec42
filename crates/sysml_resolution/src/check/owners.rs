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
use sysml_contract::ElementKind;

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
        if kind == DeclarationKind::MetadataUsage {
            self.collect_metadata_feature_typing(id, diagnostics)?;
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
