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
            // Bare (`entry;`) and effect (`entry assign ...;`) state actions are not yet lowered
            // as StateSubactionMembership members, so the occupants this publication knows are
            // not all of them.
            RedefinitionCheckKind::ActionUsageStateAction => {
                RedefinitionCheckOutcome::Unsupported {
                    prerequisite: RedefinitionCheckPrerequisite::StateSubactionMembershipAndKind,
                }
            }
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
            RedefinitionCheckKind::FeatureParameter => RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::ParameterDirectionAndInheritedPosition,
            },
            RedefinitionCheckKind::FeatureResult => RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::FunctionOrExpressionResult,
            },
            RedefinitionCheckKind::ConstructorExpressionResultFeature => {
                RedefinitionCheckOutcome::Unsupported {
                    prerequisite:
                        RedefinitionCheckPrerequisite::ConstructorResultAndInstantiatedTypeFeatures,
                }
            }
            RedefinitionCheckKind::AssignmentActionUsageAccessedFeature
            | RedefinitionCheckKind::AssignmentActionUsageReferent
            | RedefinitionCheckKind::AssignmentActionUsageStartingAt => {
                RedefinitionCheckOutcome::Unsupported {
                    prerequisite:
                        RedefinitionCheckPrerequisite::AssignmentActionInputParameterEndpoints,
                }
            }
            RedefinitionCheckKind::RequirementUsageObjective => {
                RedefinitionCheckOutcome::Unsupported {
                    prerequisite:
                        RedefinitionCheckPrerequisite::ObjectiveMembershipAndCaseObjective,
                }
            }
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
