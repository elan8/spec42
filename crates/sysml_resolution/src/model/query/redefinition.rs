//! Exact normative redefinition checks, evaluated over the settled redefinition edges.
//!
//! Each check selects the Features playing one metamodel role and the Feature each must redefine.
//! The roles are canonical semantic facts (see [`crate::resolve::role_redefinitions`] and the
//! expression projections); the edges are the publication's authored and implied `Redefinition`
//! relationships read through [`SemanticModel::collect_redefined_members`]. A check never inspects
//! names or source text, and a role or target this publication cannot settle answers `Unresolved`
//! instead of a guessed verdict. A check whose role facts are not yet published reports the
//! missing prerequisite as `Unsupported`.

use std::collections::BTreeSet;

use crate::lower::facts::OwnedEndFeature;
use crate::model::resolver::SemanticModel;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::redefinition_query::RedefinitionCheckKind;
use crate::redefinition_query::RedefinitionCheckOutcome;
use crate::redefinition_query::RedefinitionCheckPrerequisite;
use crate::resolve::end_features::cross_feature_of;
use crate::resolve::end_features::owned_cross_features;
use crate::resolve::end_features::positional_end_obligations;
use crate::resolve::end_features::CrossFeature;
use crate::resolve::implied::redefinition_check_rule;
use crate::resolve::implied::LibrarySpecializationAnchor;
use crate::resolve::objective_redefinitions::derive_objective_requirements;
use crate::resolve::objective_redefinitions::objective_obligations;
use crate::resolve::objective_redefinitions::ObjectiveRequirement;
use crate::resolve::results::FeatureChainExpressionSpecializationStatus;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::role_redefinitions::library_role_occupants;
use crate::resolve::role_redefinitions::LibraryRoleOccupant;
use crate::QueryOutcome;

/// The running verdict of one check over every Feature it applies to.
///
/// A definite violation outranks an unresolved obligation: one Feature that provably fails the
/// rule makes the model violate it whatever the unresolved ones would turn out to be.
#[derive(Debug, Default, Clone, Copy)]
struct CheckTally {
    violated: bool,
    unresolved: bool,
}

impl CheckTally {
    fn require(&mut self, satisfied: Option<bool>) {
        match satisfied {
            Some(true) => {}
            Some(false) => self.violated = true,
            None => self.unresolved = true,
        }
    }

    fn outcome(self) -> RedefinitionCheckOutcome {
        if self.violated {
            RedefinitionCheckOutcome::Violated
        } else if self.unresolved {
            RedefinitionCheckOutcome::Unresolved
        } else {
            RedefinitionCheckOutcome::Satisfied
        }
    }
}

impl<D> SemanticModel<D> {
    /// Evaluates one exact redefinition check over this publication.
    pub(crate) fn redefinition_check(
        &self,
        kind: RedefinitionCheckKind,
    ) -> QueryOutcome<RedefinitionCheckOutcome> {
        if redefinition_check_rule(kind).is_none() {
            return self.resolved_outcome(RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::RuleNotPublished,
            });
        }
        let outcome = match kind {
            RedefinitionCheckKind::ForLoopActionUsageVar => self.for_loop_variable_check(),
            RedefinitionCheckKind::FeatureChainExpressionTarget => self.library_role_check(kind),
            RedefinitionCheckKind::ActionUsageStateAction => self.library_role_check(kind),
            RedefinitionCheckKind::FeatureChainExpressionSourceTarget => {
                self.feature_chain_source_target_check()
            }
            RedefinitionCheckKind::FeatureEnd => self.feature_end_check(),
            RedefinitionCheckKind::FeatureFlowFeature => RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::FlowEndOrdinalAndLibraryAnchors,
            },
            RedefinitionCheckKind::FeatureOwnedCrossFeatureSpecialization => {
                self.owned_cross_feature_redefinition_check()
            }
            RedefinitionCheckKind::FeatureParameter => self.feature_parameter_check(),
            RedefinitionCheckKind::FeatureResult => self.feature_result_check(),
            RedefinitionCheckKind::ConstructorExpressionResultFeature => {
                self.constructor_result_feature_check()
            }
            RedefinitionCheckKind::AssignmentActionUsageAccessedFeature
            | RedefinitionCheckKind::AssignmentActionUsageStartingAt => {
                match self.assignments_without_target_parameter() {
                    true => RedefinitionCheckOutcome::Unresolved,
                    false => self.library_role_check(kind),
                }
            }
            RedefinitionCheckKind::AssignmentActionUsageReferent => {
                self.assignment_referent_check()
            }
            RedefinitionCheckKind::RequirementUsageObjective => self.objective_redefinition_check(),
            RedefinitionCheckKind::RenderingUsage => RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::ViewRenderingMembership,
            },
        };
        self.resolved_outcome(outcome)
    }

    /// Whether `source` directly redefines `target` in this publication, authored or implied.
    fn redefines(&self, source: DeclarationId, target: DeclarationId) -> bool {
        let mut redefined = BTreeSet::new();
        self.collect_redefined_members(source, &mut redefined);
        redefined.contains(&target)
    }

    /// Whether `source` directly subsets `target` in this publication, authored or implied:
    /// KerML `ownedSubsetting`, which every Subsetting subkind (Redefinition, ReferenceSubsetting,
    /// CrossSubsetting) belongs to.
    fn subsets(&self, source: DeclarationId, target: DeclarationId) -> bool {
        let is_subsetting = |kind: ReferenceKind| {
            matches!(
                kind,
                ReferenceKind::Subsetting
                    | ReferenceKind::Redefinition
                    | ReferenceKind::References
                    | ReferenceKind::Crosses
            )
        };
        self.outgoing_reference_ids(source).iter().any(|reference| {
            self.storage
                .references
                .get(reference.index())
                .is_some_and(|authored| is_subsetting(authored.kind))
                && self.resolution.outcome(*reference) == Some(ResolutionStatus::Resolved(target))
        }) || self.outgoing_implied_indices(source).iter().any(|index| {
            self.resolution
                .implied_relationships
                .get(*index as usize)
                .is_some_and(|implied| is_subsetting(implied.kind) && implied.target == target)
        })
    }

    fn role_occupants(&self) -> Option<Vec<LibraryRoleOccupant>> {
        library_role_occupants(&self.storage).ok()
    }

    /// `redefinesFromLibrary(anchor)` for every occupant of the roles `kind` constrains.
    ///
    /// The library's own declaration of an anchor occupies the role too (`entry action
    /// entryAction` in `States::StateAction`); it is the anchor, and no Feature redefines itself.
    fn library_role_check(&self, kind: RedefinitionCheckKind) -> RedefinitionCheckOutcome {
        let Some(occupants) = self.role_occupants() else {
            return RedefinitionCheckOutcome::Unresolved;
        };
        let anchors = &self.resolution.library_specialization_anchors.roles;
        let mut tally = CheckTally::default();
        for occupant in occupants
            .iter()
            .filter(|occupant| occupant.role.check() == kind)
        {
            tally.require(match anchors.anchor(occupant.role) {
                LibrarySpecializationAnchor::Resolved(anchor) => {
                    Some(occupant.source == *anchor || self.redefines(occupant.source, *anchor))
                }
                LibrarySpecializationAnchor::Missing
                | LibrarySpecializationAnchor::Ambiguous(_) => None,
            });
        }
        tally.outcome()
    }

    /// SysML `checkForLoopActionUsageVarRedefinition`: every `ForLoopActionUsage` has a
    /// `loopVariable`, and it redefines `Actions::ForLoopAction::var`.
    fn for_loop_variable_check(&self) -> RedefinitionCheckOutcome {
        let Some(occupants) = self.role_occupants() else {
            return RedefinitionCheckOutcome::Unresolved;
        };
        let loops_with_variable = occupants
            .iter()
            .filter(|occupant| {
                occupant.role.check() == RedefinitionCheckKind::ForLoopActionUsageVar
            })
            .filter_map(|occupant| self.storage.declaration(occupant.source)?.owner)
            .collect::<BTreeSet<_>>();
        let mut tally = CheckTally::default();
        for (index, declaration) in self.storage.declarations.iter().enumerate() {
            if declaration.kind != DeclarationKind::ForLoop {
                continue;
            }
            let Ok(for_loop) = DeclarationId::from_index(index) else {
                return RedefinitionCheckOutcome::Unresolved;
            };
            tally.require(Some(loops_with_variable.contains(&for_loop)));
        }
        match tally.outcome() {
            RedefinitionCheckOutcome::Satisfied => {
                self.library_role_check(RedefinitionCheckKind::ForLoopActionUsageVar)
            }
            outcome => outcome,
        }
    }

    /// KerML `checkFeatureEndRedefinition`: each owned end redefines the `endFeature` at its
    /// position of every direct supertype of its owning Type.
    ///
    /// Positions come from the canonical owned end collection and each supertype's `endFeature`
    /// from the published `TypeIndex`, through the same pairing the synthesis uses. A bare
    /// connector end mints no Feature, so an obligation with a bare end on either side has no
    /// declaration whose redefinitions could be read and is unresolved, as is every end of a Type
    /// whose specializations did not all settle or whose body was recovered.
    fn feature_end_check(&self) -> RedefinitionCheckOutcome {
        let owned = &self.storage.owned_end_features;
        let mut owners = owned.iter().map(|record| record.owner).collect::<Vec<_>>();
        owners.dedup();
        let mut tally = CheckTally::default();
        for owner in owners {
            if self.specialization_hierarchy_is_unsettled(owner)
                || self.contains_recovery(owner).unwrap_or(true)
            {
                tally.require(None);
                continue;
            }
            for obligation in positional_end_obligations(
                owned,
                owner,
                |declaration| {
                    let mut generals = self
                        .types
                        .supertypes(declaration)
                        .iter()
                        .map(|(general, _)| *general)
                        .collect::<Vec<_>>();
                    generals.dedup();
                    generals
                },
                |general| self.types.end_features(general).collect(),
            ) {
                tally.require(match (obligation.source, obligation.target) {
                    (OwnedEndFeature::Declared(source), OwnedEndFeature::Declared(target)) => {
                        Some(self.redefines(source, target))
                    }
                    _ => None,
                });
            }
        }
        tally.outcome()
    }

    /// KerML `checkFeatureOwnedCrossFeatureRedefinitionSpecialization`: an owned cross feature
    /// subsets the `crossFeature` of every Feature its end redefines.
    ///
    /// The redefined Features are the end's published Redefinitions, authored and implied; an end
    /// with an unsettled specialization, or owned by a Type with one, may redefine more than are
    /// published, and a redefined Feature whose cross feature is not a settled fact is unknown, so
    /// either answers unresolved.
    fn owned_cross_feature_redefinition_check(&self) -> RedefinitionCheckOutcome {
        let Ok(owned) = owned_cross_features(&self.storage) else {
            return RedefinitionCheckOutcome::Unresolved;
        };
        let mut tally = CheckTally::default();
        for owned in owned {
            let owner_unsettled = self
                .storage
                .declaration(owned.end)
                .and_then(|end| end.owner)
                .is_some_and(|owner| self.specialization_hierarchy_is_unsettled(owner));
            if owner_unsettled || self.specialization_hierarchy_is_unsettled(owned.end) {
                tally.require(None);
                continue;
            }
            let mut redefined = BTreeSet::new();
            self.collect_redefined_members(owned.end, &mut redefined);
            for redefined in redefined {
                tally.require(match cross_feature_of(&self.storage, redefined) {
                    Ok(CrossFeature::Resolved(cross)) => Some(
                        cross == owned.cross_feature || self.subsets(owned.cross_feature, cross),
                    ),
                    Ok(CrossFeature::Absent) => Some(true),
                    Ok(CrossFeature::Unpublished) | Err(_) => None,
                });
            }
        }
        tally.outcome()
    }

    /// Whether some AssignmentActionUsage has no lowered target parameter: an `assign` effect of a
    /// transition is not lowered beyond its own declaration, so its `startingAt`,
    /// `accessedFeature` and referent are not facts of this publication.
    fn assignments_without_target_parameter(&self) -> bool {
        let lowered = self
            .storage
            .assignments
            .iter()
            .map(|assignment| assignment.assignment)
            .collect::<BTreeSet<_>>();
        self.storage
            .declarations
            .iter()
            .enumerate()
            .any(|(index, declaration)| {
                declaration.kind == DeclarationKind::Assign
                    && DeclarationId::from_index(index)
                        .map_or(true, |assignment| !lowered.contains(&assignment))
            })
    }

    /// SysML `checkAssignmentActionUsageReferentRedefinition`: the `accessedFeature` of every
    /// assignment redefines the assignment's `referent`. A referent that did not settle, or an
    /// assignment whose target parameter is not lowered, answers unresolved.
    fn assignment_referent_check(&self) -> RedefinitionCheckOutcome {
        let mut tally = CheckTally::default();
        if self.assignments_without_target_parameter() {
            tally.require(None);
        }
        for assignment in self.storage.assignments.iter() {
            tally.require(
                match assignment
                    .referent
                    .and_then(|referent| self.resolution.outcome(referent))
                {
                    Some(ResolutionStatus::Resolved(referent)) => {
                        Some(self.redefines(assignment.accessed_feature, referent))
                    }
                    _ => None,
                },
            );
        }
        tally.outcome()
    }

    /// Every settled authored Redefinition, as `(source, target)`.
    fn authored_redefinition_pairs(&self) -> BTreeSet<(DeclarationId, DeclarationId)> {
        let mut authored = BTreeSet::new();
        for (index, reference) in self.storage.references.iter().enumerate() {
            if reference.kind != ReferenceKind::Redefinition {
                continue;
            }
            if let Some(ResolutionStatus::Resolved(target)) =
                self.resolution.outcomes.get(index).copied()
            {
                authored.insert((reference.source, target));
            }
        }
        authored
    }

    /// The deduplicated direct supertypes of `declaration` in this publication.
    fn direct_generals(&self, declaration: DeclarationId) -> Vec<DeclarationId> {
        let mut generals = self
            .types
            .supertypes(declaration)
            .iter()
            .map(|(general, _)| *general)
            .collect::<Vec<_>>();
        generals.sort();
        generals.dedup();
        generals
    }

    /// KerML `checkConstructorExpressionResultFeatureRedefinition`: each owned Feature of a
    /// constructor result redefines exactly one public feature of the instantiated type.
    ///
    /// The public features come from the same derivation the synthesis uses, over the published
    /// supertypes and redefinitions. A constructor whose instantiated type did not settle, an
    /// instantiated type whose specializations did not all settle, and a named argument whose
    /// feature did not settle answer unresolved.
    fn constructor_result_feature_check(&self) -> RedefinitionCheckOutcome {
        use crate::resolve::constructor_features::constructor_arguments;
        use crate::resolve::constructor_features::public_features;
        use crate::resolve::results::ConstructorExpressionProjectionStatus;
        let mut tally = CheckTally::default();
        if !self.storage.constructor_expressions.is_empty()
            && self.resolution.constructor_expression_projection_status
                == ConstructorExpressionProjectionStatus::Unresolved
        {
            tally.require(None);
        }
        let projections = &self.resolution.constructor_expression_projections;
        let redefined = |feature: DeclarationId| {
            let mut redefined = BTreeSet::new();
            self.collect_redefined_members(feature, &mut redefined);
            redefined
        };
        let (Ok(features), Ok(arguments)) = (
            public_features(
                &self.storage,
                projections
                    .iter()
                    .map(|projection| projection.instantiated_type),
                |declaration| self.direct_generals(declaration),
                |feature| redefined(feature).into_iter().collect(),
            ),
            constructor_arguments(&self.storage, projections),
        ) else {
            return RedefinitionCheckOutcome::Unresolved;
        };
        for projection in projections.iter() {
            let Some(arguments) = arguments.get(&projection.result) else {
                continue;
            };
            if self.specialization_hierarchy_is_unsettled(projection.instantiated_type) {
                tally.require(None);
                continue;
            }
            let features = features
                .get(&projection.instantiated_type)
                .map(|features| features.iter().copied().collect::<BTreeSet<_>>())
                .unwrap_or_default();
            for argument in arguments.iter().copied() {
                let named_unsettled =
                    self.outgoing_reference_ids(argument)
                        .iter()
                        .any(|reference| {
                            self.storage.references[reference.index()].kind
                                == ReferenceKind::Redefinition
                                && !matches!(
                                    self.resolution.outcome(*reference),
                                    Some(ResolutionStatus::Resolved(_))
                                )
                        });
                tally.require(if named_unsettled {
                    None
                } else {
                    Some(redefined(argument).intersection(&features).count() == 1)
                });
            }
        }
        tally.outcome()
    }

    /// KerML `checkFeatureParameterRedefinition`: each parameter of a Behavior or Step redefines
    /// the parameter at its position of every Behavior or Step its owner directly specializes.
    ///
    /// Obligations come from the same derivation the synthesis uses, over the published direct
    /// supertypes. A publication with a grammar-defined parameter lowering does not mint has
    /// obligations that are not facts, which is reported as the unsupported prerequisite; an
    /// owner whose specializations did not all settle may have supertypes this publication does
    /// not know, and answers unresolved.
    fn feature_parameter_check(&self) -> RedefinitionCheckOutcome {
        use crate::resolve::parameter_positions::grammar_parameters_are_unlowered;
        use crate::resolve::parameter_positions::is_behavior_or_step;
        use crate::resolve::parameter_positions::owned_parameters;
        use crate::resolve::parameter_positions::parameter_obligations;
        if grammar_parameters_are_unlowered(&self.storage) {
            return RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::GrammarParameters,
            };
        }
        let generals = |declaration: DeclarationId| self.direct_generals(declaration);
        let (Ok(owned), Ok(obligations)) = (
            owned_parameters(&self.storage),
            parameter_obligations(&self.storage, generals),
        ) else {
            return RedefinitionCheckOutcome::Unresolved;
        };
        let mut tally = CheckTally::default();
        for (index, parameters) in owned.iter().enumerate() {
            let Ok(owner) = DeclarationId::from_index(index) else {
                return RedefinitionCheckOutcome::Unresolved;
            };
            if !parameters.is_empty()
                && self
                    .storage
                    .declaration(owner)
                    .is_some_and(|declaration| is_behavior_or_step(declaration.kind))
                && self.specialization_hierarchy_is_unsettled(owner)
            {
                tally.require(None);
            }
        }
        for obligation in obligations {
            tally.require(Some(self.redefines(obligation.source, obligation.target)));
        }
        tally.outcome()
    }

    /// KerML `checkFeatureResultRedefinition`: the result of a Function or Expression redefines
    /// the result of every Function or Expression its owner directly specializes.
    ///
    /// Obligations come from the same derivation the synthesis uses, over the published direct
    /// supertypes. A publication with an expression lowering does not represent as an Expression
    /// element has results that are not facts, which is reported as the unsupported
    /// prerequisite. An owner whose specializations did not all settle -- including an
    /// instantiation Expression whose callee, and so its typing, did not settle -- may have
    /// supertypes this publication does not know, and an owner with several results, or a
    /// supertype whose inherited result is ambiguous, has no single result to pair, so each
    /// answers unresolved.
    fn feature_result_check(&self) -> RedefinitionCheckOutcome {
        use crate::resolve::inherited_members::InheritedMember;
        use crate::resolve::result_parameters::is_function_or_expression;
        use crate::resolve::result_parameters::owned_result_parameters;
        use crate::resolve::result_parameters::result_obligations;
        if self
            .storage
            .unlowered_expressions
            .iter()
            .any(|site| site.kind == crate::lower::facts::UnloweredExpression::Element)
        {
            return RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::ExpressionElements,
            };
        }
        let generals = |declaration: DeclarationId| self.direct_generals(declaration);
        let authored = self.authored_redefinition_pairs();
        let (Ok(owned), Ok(obligations)) = (
            owned_result_parameters(&self.storage),
            result_obligations(&self.storage, generals, &authored),
        ) else {
            return RedefinitionCheckOutcome::Unresolved;
        };
        let callee_unsettled = |owner: DeclarationId| {
            self.outgoing_reference_ids(owner).iter().any(|reference| {
                self.storage
                    .references
                    .get(reference.index())
                    .is_some_and(|authored| authored.kind == ReferenceKind::InvocationCallee)
                    && !matches!(
                        self.resolution.outcome(*reference),
                        Some(ResolutionStatus::Resolved(_))
                    )
            })
        };
        let mut tally = CheckTally::default();
        for owner in owned
            .iter()
            .map(|(owner, _)| owner)
            .chain(owned.owners_with_several())
        {
            let Some(declaration) = self.storage.declaration(owner) else {
                return RedefinitionCheckOutcome::Unresolved;
            };
            if is_function_or_expression(declaration.kind)
                && (owned.owned(owner).is_none()
                    || self.specialization_hierarchy_is_unsettled(owner)
                    || callee_unsettled(owner))
            {
                tally.require(None);
            }
        }
        for obligation in obligations {
            tally.require(match obligation.target {
                InheritedMember::Resolved(target) => {
                    Some(self.redefines(obligation.result, target))
                }
                InheritedMember::Ambiguous => None,
                InheritedMember::Absent => Some(true),
            });
        }
        tally.outcome()
    }

    /// SysML `checkRequirementUsageObjectiveRedefinition`: an objective redefines the
    /// `objectiveRequirement` of every case its owning Type directly specializes.
    ///
    /// Obligations come from the same derivation the synthesis uses, over the published direct
    /// supertypes. An ambiguous inherited objective, or an owner whose specializations did not all
    /// settle, answers unresolved.
    fn objective_redefinition_check(&self) -> RedefinitionCheckOutcome {
        let generals = |declaration: DeclarationId| {
            let mut generals = self
                .types
                .supertypes(declaration)
                .iter()
                .map(|(general, _)| *general)
                .collect::<Vec<_>>();
            generals.dedup();
            generals
        };
        let authored = self.authored_redefinition_pairs();
        let Ok(objectives) = derive_objective_requirements(&self.storage, generals, &authored)
        else {
            return RedefinitionCheckOutcome::Unresolved;
        };
        let Ok(obligations) = objective_obligations(&self.storage, generals, &objectives) else {
            return RedefinitionCheckOutcome::Unresolved;
        };
        let mut tally = CheckTally::default();
        for declaration in self.storage.declarations.iter() {
            if declaration.kind == DeclarationKind::ObjectiveRequirement
                && declaration
                    .owner
                    .is_some_and(|owner| self.specialization_hierarchy_is_unsettled(owner))
            {
                tally.require(None);
            }
        }
        for obligation in obligations {
            tally.require(match obligation.target {
                ObjectiveRequirement::Resolved(target) => {
                    Some(self.redefines(obligation.objective, target))
                }
                ObjectiveRequirement::Ambiguous => None,
                ObjectiveRequirement::Absent => Some(true),
            });
        }
        tally.outcome()
    }

    /// KerML `checkFeatureChainExpressionSourceTargetRedefinition`: the source-target feature of
    /// every `FeatureChainExpression` redefines the expression's `targetFeature`.
    fn feature_chain_source_target_check(&self) -> RedefinitionCheckOutcome {
        let mut tally = CheckTally::default();
        for chain in self.storage.feature_chain_expressions.iter() {
            let projection = self
                .resolution
                .feature_chain_expression_projections
                .iter()
                .find(|projection| projection.expression == chain.expression);
            tally.require(match projection {
                Some(projection) => Some(
                    projection.source_target == chain.source_target
                        && self.redefines(chain.source_target, projection.target_feature),
                ),
                // No projection means `targetFeature` did not settle to one Feature.
                None => {
                    debug_assert!(matches!(
                        self.resolution
                            .feature_chain_expression_specialization_status,
                        FeatureChainExpressionSpecializationStatus::Unresolved
                    ));
                    None
                }
            });
        }
        tally.outcome()
    }
}
