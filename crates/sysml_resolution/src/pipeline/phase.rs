//! The phase order of semantic construction, expressed as types.
//!
//! Each phase's writer consumes the previous phase's product **by value** and yields the next as a
//! distinct type:
//!
//! ```text
//! Lowered â”€â–º Resolved â”€â–º Evaluated â”€â–º Indexed â”€â–º Complete
//! ```
//!
//! Two properties follow by construction rather than by discipline:
//!
//! * no phase can write back into an earlier phase's store, because that store has been moved; and
//! * there is no half-built model to observe â€” `ResolvedSemanticModel` is reachable only out of
//!   `Complete`, and `Complete` cannot be built without a settled diagnostic product.

use crate::evaluate::compute_evaluation;
use crate::evaluate::EvaluationFact;
use crate::evaluate::SettledEvaluation;
use crate::evaluation::EvaluationPolicy;
use crate::index::bindings::BindingConnectorIndex;
use crate::index::connector_context::context_featuring_candidates;
use crate::index::connector_context::synthesize_connector_context_featurings;
use crate::index::documents::DocumentIndex;
use crate::index::elements::ElementFactIndex;
use crate::index::expressions::ExpressionIndex;
use crate::index::expressions::ExpressionInputs;
use crate::index::expressions::SettledFilter;
use crate::index::identity::IdentityIndex;
use crate::index::qualified::QualifiedNameIndex;
use crate::index::resolved_expressions::ResolvedExpressionIndex;
use crate::index::resolved_expressions::ResolvedExpressionInputs;
use crate::index::reverse_references::ReverseReferenceIndex;
use crate::index::types::TypeIndex;
use crate::lower::storage::ParsedSources;
use crate::lower::storage::SemanticModelStorage;
use crate::model::resolver::Indexed;
use crate::model::resolver::NotYetDiagnosed;
use crate::model::resolver::PublicationMetadata;
use crate::model::resolver::PublicationPhase;
use crate::model::resolver::ResolvedSemanticModel;
use crate::model::resolver::SettledDiagnostics;
use crate::resolve::effective_types::derive_effective_types;
use crate::resolve::implied::library_specialization_anchors;
use crate::resolve::implied::synthesize_constructor_expression_result_specializations;
use crate::resolve::implied::synthesize_feature_chain_expression_result_specializations;
use crate::resolve::implied::synthesize_feature_reference_expression_result_specializations;
use crate::resolve::implied::synthesize_implied_relationships;
use crate::resolve::implied::synthesize_invocation_expression_specializations;
use crate::resolve::implied::synthesize_operator_expression_result_specializations;
use crate::resolve::implied::synthesize_owned_cross_feature_type_featurings;
use crate::resolve::implied::synthesize_owned_cross_feature_typings;
use crate::resolve::implied::synthesize_semantic_metadata_specializations;
use crate::resolve::implied::synthesize_succession_endpoint_subsettings;
use crate::resolve::implied::synthesize_transition_feature_specializations;
use crate::resolve::implied::synthesize_transition_payload_subsettings;
use crate::resolve::implied::synthesize_transition_succession_sources;
use crate::resolve::library_seed::SettledLibrary;
use crate::resolve::names::EffectiveScopeIndex;
use crate::resolve::names::MembershipIndex;
use crate::resolve::names::NameIndex;
use crate::resolve::resolve_dense;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::results::SolverStatus;
use sysml_contract::PublicationCompleteness;

/// Phase 2's product: the frozen storage of authored facts, plus the parse product the phases
/// after it still read.
///
/// The two travel together only as far as the publication barrier. `design.md`: a sealed
/// publication holds no parse tree, so [`Indexed::diagnose`] consumes the parse product and the
/// model that comes out of it cannot name one.
pub(crate) struct Lowered {
    storage: SemanticModelStorage,
    sources: ParsedSources,
}

/// Phase 3 + 4's product: settled name-resolution outcomes and implied relationships.
pub(crate) struct Resolved {
    storage: SemanticModelStorage,
    sources: ParsedSources,
    direct_names: NameIndex,
    effective_imports: NameIndex,
    exported_imports: NameIndex,
    memberships: MembershipIndex,
    resolution: ResolutionResults,
    completeness: PublicationCompleteness,
}

/// Phase 5's product: the evaluated values, decided here and only here.
pub(crate) struct Evaluated {
    storage: SemanticModelStorage,
    sources: ParsedSources,
    direct_names: NameIndex,
    effective_imports: NameIndex,
    exported_imports: NameIndex,
    memberships: MembershipIndex,
    resolution: ResolutionResults,
    evaluation: Box<[EvaluationFact]>,
    filter_conditions: Option<Box<[SettledFilter]>>,
    completeness: PublicationCompleteness,
}

/// Phase 8's product: the only value a `ResolvedSemanticModel` can be taken out of.
pub(crate) struct Complete {
    model: ResolvedSemanticModel,
    namespace_checks: std::time::Duration,
    /// Kept only so a library-stratum build can hand the trees to the next publication; a
    /// workspace build drops them with this value.
    sources: ParsedSources,
}

impl From<(SemanticModelStorage, ParsedSources)> for Lowered {
    fn from((storage, sources): (SemanticModelStorage, ParsedSources)) -> Self {
        Self { storage, sources }
    }
}

/// The canonical source-independent resolution phase, shared by publication and import plans.
pub(crate) struct ResolvedFacts {
    pub(crate) direct_names: NameIndex,
    pub(crate) effective_imports: NameIndex,
    pub(crate) exported_imports: NameIndex,
    pub(crate) memberships: MembershipIndex,
    pub(crate) resolution: ResolutionResults,
}

pub(crate) fn resolve_facts(
    storage: &SemanticModelStorage,
    library: Option<&SettledLibrary>,
    proposed_imports: &[(crate::resolve::names::NameKey, crate::model::DeclarationId)],
) -> Result<ResolvedFacts, ResolutionError> {
    let admitted_library = library.filter(|library| library.admits(storage));
    let seed = admitted_library.map(|library| library.outcomes.as_ref());
    let library_anchors = library_specialization_anchors(storage);
    let provisional_library_specializations =
        crate::resolve::implied::provisional_library_specializations(storage, &library_anchors)?;
    let shadowed_library_roots = crate::resolve::library_seed::shadowed_library_roots(storage);
    // Semantic metadata can introduce supertypes whose inherited members are needed by
    // authored references, including further metadata baseType redefinitions. Feed the
    // canonical synthesis back into name resolution until the exact relationship set is
    // stable; no intermediate resolution is published (SysML 7.27.3â€“7.27.4).
    let feature_type_dependencies =
        crate::resolve::effective_types::feature_chain_typing_dependencies(storage);
    let mut provisional_relationships = provisional_library_specializations.to_vec();
    let mut previous_metadata_relationships = Vec::new();
    let mut previous_expression_relationships = admitted_library
        .map(|library| library.expression_relationships.to_vec())
        .unwrap_or_default();
    provisional_relationships.extend(previous_expression_relationships.iter().copied());
    provisional_relationships.sort_by_key(|edge| (edge.kind, edge.source.0, edge.target.0));
    provisional_relationships.dedup();
    let has_expression_roots = storage.references.iter().any(|reference| {
        reference.member_access_narrowings.iter().any(|narrowing| {
            matches!(
                narrowing.target,
                crate::lower::facts::MemberAccessTarget::ExpressionResult(_)
            )
        })
    });
    // Metadata contributes at most one dependency link per annotation. An expression-result
    // relationship is sourced at the result of one feature-reference, feature-chain or
    // operator expression, and a pass can only add one whose prerequisite settled in the
    // previous pass, so a dependency chain visits each of those expressions at most once.
    // Two extra passes confirm the fixed point. Both families share this barrier so lookup
    // never publishes stale result types; exhausting the bound leaves an explicit
    // nonconverged publication.
    let expression_links = if has_expression_roots {
        storage
            .feature_reference_expressions
            .len()
            .saturating_add(storage.feature_chain_expressions.len())
            .saturating_add(storage.operator_expressions.len())
    } else {
        0
    };
    let mut relationship_passes_remaining = storage
        .metadata_annotations
        .len()
        .saturating_add(expression_links)
        .saturating_add(2);
    let (direct_names, effective_imports, exported_imports, memberships, resolution) = loop {
        let (direct_names, effective_imports, exported_imports, memberships, resolution) =
            resolve_dense(
                &storage.declarations,
                Some(&storage.declaration_facts),
                &storage.memberships,
                &storage.paths,
                &storage.references,
                crate::resolve::ResolutionStartingState {
                    provisional_relationships: &provisional_relationships,
                    settled_outcomes: seed,
                    settled_ambiguous_candidates: admitted_library
                        .map(|library| library.ambiguous_candidates.as_ref())
                        .unwrap_or_default(),
                    shadowed_library_roots: &shadowed_library_roots,
                    owned_end_features: &storage.owned_end_features,
                    feature_type_dependencies: &feature_type_dependencies,
                    proposed_imports,
                },
            )?;
        let resolution = resolution.follow_aliases(&storage.declarations, &storage.references);
        let authored_relationships = storage
            .relationship_declarations
            .iter()
            .filter_map(|relationship| {
                let ResolutionStatus::Resolved(source) = resolution.outcome(relationship.source)?
                else {
                    return None;
                };
                let ResolutionStatus::Resolved(target) = resolution.outcome(relationship.target)?
                else {
                    return None;
                };
                Some(crate::resolve::results::AuthoredRelationship {
                    kind: relationship.kind,
                    source,
                    target,
                    declaration: relationship.source,
                })
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let resolution = resolution.settle_authored_relationships(authored_relationships);
        // `checkPartDefinitionSpecialization` is an implied semantic fact, so its anchor and
        // relationships are settled here, before every index and diagnostic consumer below. The
        // lookup is owned by semantic construction: neither a renderer nor a validation rule gets
        // to rediscover `Parts::Part` from text or a display path.
        let base_implied =
            synthesize_implied_relationships(storage, &resolution, &library_anchors)?;
        // Edges derived without semantic metadata. The previous pass's metadata edges are
        // only prerequisites for the derivations below and are replaced, not kept, once this
        // pass's synthesis is known.
        let mut independent = base_implied.to_vec();
        independent.extend(previous_expression_relationships.iter().copied());
        let mut implied = base_implied.to_vec();
        implied.extend(previous_metadata_relationships.iter().copied());
        implied.extend(previous_expression_relationships.iter().copied());
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        let resolution = resolution.settle(implied.into_boxed_slice(), library_anchors.clone());
        // Owned cross-feature typing depends on the owning end's effective types, including types
        // inherited through the implied relationships settled above. Consume the canonical type
        // index at an explicit sub-barrier, then replace the provisional relationship set before
        // publishing `Resolved`; no reader can observe the intermediate state.
        let resolution = if storage
            .declaration_facts
            .iter()
            .any(|facts| facts.cross_feature_projection.is_some())
        {
            let prerequisite_types = derive_effective_types(storage, &resolution)?;
            let mut implied = resolution.implied_relationships.to_vec();
            let cross_feature =
                synthesize_owned_cross_feature_typings(storage, &prerequisite_types)?
                    .into_vec()
                    .into_iter()
                    .chain(
                        synthesize_owned_cross_feature_type_featurings(
                            storage,
                            &prerequisite_types,
                        )?
                        .into_vec(),
                    )
                    .collect::<Vec<_>>();
            independent.extend(cross_feature.iter().copied());
            implied.extend(cross_feature);
            implied.sort_by_key(|relationship| {
                (
                    relationship.kind,
                    relationship.source.0,
                    relationship.target.0,
                )
            });
            implied.dedup();
            let library_anchors = resolution.library_specialization_anchors.clone();
            resolution.settle(implied.into_boxed_slice(), library_anchors)
        } else {
            resolution
        };
        let expression_relationships = if has_expression_roots {
            crate::resolve::implied::synthesize_access_expression_result_relationships(
                storage,
                &resolution,
            )?
        } else {
            Vec::new()
        };
        let mut metadata_relationships = Vec::new();
        let mut resolution = if storage.metadata_annotations.is_empty() {
            resolution
        } else {
            let prerequisite_types = derive_effective_types(storage, &resolution)?;
            let synthesis = synthesize_semantic_metadata_specializations(
                storage,
                &resolution,
                &prerequisite_types,
            )?;
            metadata_relationships = synthesis.implied_relationships.to_vec();
            // Prior metadata edges are prerequisites, not an accumulating truth store: the
            // settled set is the independently derived edges plus this pass's canonical
            // synthesis, so an edge also derived another way survives and the result does
            // not depend on pass history.
            let mut implied = independent;
            implied.extend(synthesis.implied_relationships);
            implied.sort_by_key(|relationship| {
                (
                    relationship.kind,
                    relationship.source.0,
                    relationship.target.0,
                )
            });
            implied.dedup();
            resolution.settle_semantic_metadata(
                implied.into_boxed_slice(),
                synthesis.projections,
                synthesis.status,
            )
        };
        let mut next_relationships = provisional_library_specializations.to_vec();
        next_relationships.extend(metadata_relationships.iter().copied());
        next_relationships.extend(expression_relationships.iter().copied());
        next_relationships.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        next_relationships.dedup();
        relationship_passes_remaining -= 1;
        if next_relationships == provisional_relationships {
            break (
                direct_names,
                effective_imports,
                exported_imports,
                memberships,
                resolution,
            );
        }
        if relationship_passes_remaining == 0 {
            resolution.solver_status = SolverStatus::NonConverged;
            break (
                direct_names,
                effective_imports,
                exported_imports,
                memberships,
                resolution,
            );
        }
        provisional_relationships = next_relationships;
        previous_metadata_relationships = metadata_relationships;
        previous_expression_relationships = expression_relationships;
    };
    let resolution = if storage
        .declaration_facts
        .iter()
        .all(|facts| !facts.is_transition_payload_parameter)
    {
        resolution
    } else {
        let synthesis = synthesize_transition_payload_subsettings(storage)?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(synthesis.implied_relationships);
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        resolution.settle_transition_payload_subsettings(
            implied.into_boxed_slice(),
            synthesis.projections,
            synthesis.status,
        )
    };
    let resolution = if storage
        .declaration_facts
        .iter()
        .all(|facts| !facts.is_transition_succession)
    {
        resolution
    } else {
        let synthesis = synthesize_transition_succession_sources(storage, &resolution)?;
        resolution.settle_transition_succession_sources(synthesis.projections, synthesis.status)
    };
    let resolution = if storage
        .declaration_facts
        .iter()
        .all(|facts| facts.transition_feature_role.is_none())
    {
        resolution
    } else {
        let synthesis = synthesize_transition_feature_specializations(storage)?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(synthesis.implied_relationships);
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        resolution.settle_transition_feature_specializations(
            implied.into_boxed_slice(),
            synthesis.projections,
            synthesis.status,
        )
    };
    let resolution = if storage.operator_expressions.is_empty() {
        resolution
    } else {
        let synthesis =
            synthesize_operator_expression_result_specializations(storage, &resolution)?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(synthesis.implied_relationships);
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        resolution.settle_expression_arguments(
            implied.into_boxed_slice(),
            synthesis.select_status,
            synthesis.index_status,
            synthesis.array_anchor,
        )
    };
    let resolution = if storage.constructor_expressions.is_empty() {
        resolution
    } else {
        let synthesis =
            synthesize_constructor_expression_result_specializations(storage, &resolution)?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(synthesis.implied_relationships);
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        resolution.settle_constructor_expressions(
            implied.into_boxed_slice(),
            synthesis.projections,
            synthesis.status,
            synthesis.specialization_status,
            Some(synthesis.anchor),
        )
    };
    let resolution = if storage.feature_chain_expressions.is_empty() {
        resolution
    } else {
        let synthesis =
            synthesize_feature_chain_expression_result_specializations(storage, &resolution)?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(synthesis.implied_relationships);
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        resolution.settle_feature_chain_expressions(
            implied.into_boxed_slice(),
            synthesis.projections,
            synthesis.status,
        )
    };
    let resolution = if storage.feature_reference_expressions.is_empty() {
        resolution
    } else {
        let synthesis =
            synthesize_feature_reference_expression_result_specializations(storage, &resolution)?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(synthesis.implied_relationships);
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        resolution.settle_feature_reference_expressions(
            implied.into_boxed_slice(),
            synthesis.projections,
            synthesis.status,
        )
    };
    let resolution = if storage.invocations.is_empty() {
        resolution
    } else {
        let prerequisite_types = derive_effective_types(storage, &resolution)?;
        let synthesis = synthesize_invocation_expression_specializations(
            storage,
            &resolution,
            &prerequisite_types,
        )?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(synthesis.implied_relationships);
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        resolution.settle_invocation_expressions(
            implied.into_boxed_slice(),
            synthesis.projections,
            synthesis.status,
        )
    };
    // `checkFeatureParameterRedefinition` then `checkFeatureResultRedefinition`: a parameter
    // redefines the parameter at its position, and a result the result, of every Behavior or
    // Step (Function or Expression) its owner directly specializes. That includes the
    // FeatureTyping of an invocation by its callee settled just above, so both run once every
    // specialization of a Behavior or Step is known; a positional parameter redefinition is
    // itself a specialization of a parameter that may own a result, so results run last.
    let resolution = if !matches!(resolution.solver_status, SolverStatus::Converged) {
        resolution
    } else {
        let settled = crate::resolve::SettledTypeEdges::collect(
            &storage.references,
            &resolution.outcomes,
            &resolution.implied_relationships,
        )?;
        let parameters = crate::resolve::parameter_positions::synthesize_parameter_redefinitions(
            storage,
            settled.edges,
            &settled.authored_redefinitions,
        )?;
        let mut with_parameters = resolution.implied_relationships.to_vec();
        with_parameters.extend(parameters.iter().copied());
        let settled = crate::resolve::SettledTypeEdges::collect(
            &storage.references,
            &resolution.outcomes,
            &with_parameters,
        )?;
        let mut synthesized = parameters;
        synthesized.extend(
            crate::resolve::result_parameters::synthesize_result_redefinitions(
                storage,
                settled.edges.iter().copied(),
                &settled.authored_redefinitions,
            )?,
        );
        // `checkConstructorExpressionResultFeatureRedefinition`: a positional constructor
        // argument redefines the public feature of the instantiated type at its position.
        // Which features a type has excludes the ones its members redefine, so this reads
        // every redefinition settled so far, the ones just synthesized included.
        let mut redefinitions = settled.authored_redefinitions.clone();
        redefinitions.extend(
            with_parameters
                .iter()
                .chain(synthesized.iter())
                .filter(|relationship| {
                    relationship.kind == crate::model::ReferenceKind::Redefinition
                })
                .map(|relationship| (relationship.source, relationship.target)),
        );
        synthesized.extend(
            crate::resolve::constructor_features::synthesize_constructor_argument_redefinitions(
                storage,
                &resolution.constructor_expression_projections,
                settled.edges,
                &redefinitions,
            )?,
        );
        if synthesized.is_empty() {
            resolution
        } else {
            let mut implied = resolution.implied_relationships.to_vec();
            implied.extend(synthesized);
            implied.sort_by_key(|relationship| {
                (
                    relationship.kind,
                    relationship.source.0,
                    relationship.target.0,
                )
            });
            implied.dedup();
            let library_anchors = resolution.library_specialization_anchors.clone();
            resolution.settle(implied.into_boxed_slice(), library_anchors)
        }
    };
    let resolution = if storage
        .declarations
        .iter()
        .all(|declaration| declaration.kind != crate::model::DeclarationKind::Succession)
    {
        resolution
    } else {
        let synthesis = synthesize_succession_endpoint_subsettings(storage, &resolution)?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(synthesis.implied_relationships);
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        resolution.settle_succession_endpoint_subsettings(
            implied.into_boxed_slice(),
            synthesis.projections,
            synthesis.decision_status,
            synthesis.merge_status,
            synthesis.control_node_successions,
        )
    };
    // A Connector with no owning Type is featured by the innermost featuring type its related
    // features share (`checkConnectorTypeFeaturing`). That reads the settled featuring rows,
    // end features and specialization closure, so it consumes a prerequisite type index at
    // this final sub-barrier; nothing else depends on a Connector's own featuring type.
    let connector_candidates = context_featuring_candidates(storage)?;
    let resolution = if connector_candidates.is_empty() {
        resolution
    } else {
        let prerequisite_types = TypeIndex::build(storage, &resolution)?;
        let mut implied = resolution.implied_relationships.to_vec();
        implied.extend(
            synthesize_connector_context_featurings(
                storage,
                &resolution,
                &prerequisite_types,
                &connector_candidates,
            )?
            .into_vec(),
        );
        implied.sort_by_key(|relationship| {
            (
                relationship.kind,
                relationship.source.0,
                relationship.target.0,
            )
        });
        implied.dedup();
        let library_anchors = resolution.library_specialization_anchors.clone();
        resolution.settle(implied.into_boxed_slice(), library_anchors)
    };
    Ok(ResolvedFacts {
        direct_names,
        effective_imports,
        exported_imports,
        memberships,
        resolution,
    })
}

impl Lowered {
    /// Phases 3 and 4: solve names to convergence, then settle implied relationships.
    ///
    /// The implied store is built from the frozen solver product and handed to
    /// [`ResolutionResults::settle`], which yields a new value; the solver's own product is moved
    /// out of scope rather than edited, so the pre-synthesis set is not observable.
    pub(crate) fn resolve(
        self,
        library: Option<&SettledLibrary>,
    ) -> Result<Resolved, ResolutionError> {
        let storage = self.storage;
        let sources = self.sources;
        let has_recovery = sources.any_parse_errors() || !storage.recovery.is_empty();
        let has_unsupported = !storage.unsupported.is_empty();
        let ResolvedFacts {
            direct_names,
            effective_imports,
            exported_imports,
            memberships,
            resolution,
        } = resolve_facts(&storage, library, &[])?;
        let mut completeness = PublicationCompleteness::Complete;
        if has_recovery {
            completeness = completeness.with(sysml_contract::PublicationObstacle::ParseRecovery);
        }
        if has_unsupported {
            completeness =
                completeness.with(sysml_contract::PublicationObstacle::UnsupportedSyntax);
        }
        if !matches!(resolution.solver_status, SolverStatus::Converged) {
            completeness = completeness.with(sysml_contract::PublicationObstacle::NonConverged);
        }
        Ok(Resolved {
            storage,
            sources,
            direct_names,
            effective_imports,
            exported_imports,
            memberships,
            resolution,
            completeness,
        })
    }
}

impl Resolved {
    /// Phase 5: the single evaluation writer.
    pub(crate) fn evaluate(self, policy: EvaluationPolicy) -> Evaluated {
        let (evaluation, filter_conditions) =
            match compute_evaluation(&self.storage, &self.sources, &self.resolution, policy) {
                SettledEvaluation::Settled { facts, filters } => (facts, Some(filters)),
                SettledEvaluation::Vacuous => (Box::default(), None),
            };
        Evaluated {
            storage: self.storage,
            sources: self.sources,
            direct_names: self.direct_names,
            effective_imports: self.effective_imports,
            exported_imports: self.exported_imports,
            memberships: self.memberships,
            resolution: self.resolution,
            evaluation,
            filter_conditions,
            completeness: self.completeness,
        }
    }
}

impl Evaluated {
    /// Phase 6: every derived-fact index, assembled at one barrier.
    pub(crate) fn index(self) -> Result<(Indexed, ParsedSources), ResolutionError> {
        let has_evaluation = !self.evaluation.is_empty();
        let identities = IdentityIndex::build(&self.storage)?;
        let qualified_names = QualifiedNameIndex::build(&self.storage)?;
        let documents = DocumentIndex::build(&self.storage, &self.sources)?;
        let reverse_references =
            ReverseReferenceIndex::build(self.storage.declarations.len(), &self.resolution)?;
        let effective_scopes = EffectiveScopeIndex::build(
            self.storage.declarations.len(),
            &self.direct_names,
            &self.effective_imports,
            &self.resolution.inherited_names,
        )?;
        let facts = ElementFactIndex::build(&self.storage, &self.resolution, &self.evaluation)?;
        // A barrier product, not a solver family: every type fact here is derived from settled
        // outcomes and feeds nothing back into scope, imports or inheritance. The resolver's own
        // ancestor closure for inherited names stays separate and unchanged -- widening that one
        // would silently change name resolution.
        let types = TypeIndex::build(&self.storage, &self.resolution)?;
        // Result binding connectors relate a Function's or Expression's result, which may be
        // inherited over the settled type closure.
        let bindings = BindingConnectorIndex::build(&self.storage, &self.resolution, &types)?;
        // Expression facts read the type closure and the settled evaluation, so they name those
        // three inputs rather than borrowing a model that does not exist yet.
        let expressions = ExpressionIndex::build(
            &ExpressionInputs {
                storage: &self.storage,
                resolution: &self.resolution,
                types: &types,
            },
            self.filter_conditions,
        )?;
        let resolved_expressions = ResolvedExpressionIndex::build(&ResolvedExpressionInputs {
            storage: &self.storage,
            sources: &self.sources,
            resolution: &self.resolution,
            types: &types,
            scopes: &effective_scopes,
            direct_names: &self.direct_names,
            effective_imports: &self.effective_imports,
        })?;
        let indexed = Indexed {
            storage: self.storage,
            direct_names: self.direct_names,
            effective_imports: self.effective_imports,
            exported_imports: self.exported_imports,
            identities,
            qualified_names,
            documents,
            memberships: self.memberships,
            reverse_references,
            effective_scopes,
            facts,
            bindings,
            types,
            resolution: self.resolution,
            evaluation: self.evaluation,
            expressions,
            resolved_expressions,
            diagnostics: NotYetDiagnosed,
            metadata: PublicationMetadata {
                phase: PublicationPhase::Resolved,
                completeness: self.completeness,
                has_evaluation,
            },
        };
        Ok((indexed, self.sources))
    }
}

impl Indexed {
    /// Phase 8: diagnostics reported from everything the earlier phases settled.
    ///
    /// The settled product is put into the model at construction; there is no window in which a
    /// model exists with an empty diagnostic sequence standing in for an underived one.
    pub(crate) fn diagnose(
        self,
        sources: ParsedSources,
        reported: &[Box<str>],
        standard_library_availability: sysml_contract::StandardLibraryAvailability,
    ) -> Result<Complete, ResolutionError> {
        let crate::diagnose::DerivedDiagnostics {
            diagnostics,
            by_document,
            namespace_checks,
        } = self.derive_diagnostics(&sources, reported, standard_library_availability)?;
        // The parse product is consumed here and goes no further: this is the barrier that makes
        // "a sealed publication holds no parse tree" true by construction rather than by review.
        Ok(Complete {
            sources,
            namespace_checks,
            model: ResolvedSemanticModel {
                storage: self.storage,
                direct_names: self.direct_names,
                effective_imports: self.effective_imports,
                exported_imports: self.exported_imports,
                identities: self.identities,
                qualified_names: self.qualified_names,
                documents: self.documents,
                memberships: self.memberships,
                reverse_references: self.reverse_references,
                effective_scopes: self.effective_scopes,
                facts: self.facts,
                bindings: self.bindings,
                types: self.types,
                resolution: self.resolution,
                evaluation: self.evaluation,
                expressions: self.expressions,
                resolved_expressions: self.resolved_expressions,
                diagnostics: SettledDiagnostics {
                    diagnostics,
                    by_document,
                },
                metadata: self.metadata,
            },
        })
    }
}

impl Complete {
    pub(crate) fn into_parts(self) -> (ResolvedSemanticModel, ParsedSources) {
        (self.model, self.sources)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SemanticPhaseDurations {
    pub(crate) resolve: std::time::Duration,
    pub(crate) evaluation: std::time::Duration,
    pub(crate) indexing: std::time::Duration,
    pub(crate) diagnostics: std::time::Duration,
    pub(crate) namespace_checks: std::time::Duration,
}

/// The whole phase order, with measurements taken at its owning barriers.
pub(crate) fn build_model(
    storage: SemanticModelStorage,
    sources: ParsedSources,
    policy: EvaluationPolicy,
    standard_library_availability: sysml_contract::StandardLibraryAvailability,
    library: Option<&SettledLibrary>,
    reported: &[Box<str>],
    controls: super::BuildControls<'_>,
) -> Result<(ResolvedSemanticModel, ParsedSources, SemanticPhaseDurations), crate::BuildFailure> {
    let identities: Vec<_> = storage
        .documents
        .iter()
        .map(|document| document.identity.clone())
        .collect();
    let failure = |phase, error| resolution_failure(phase, &identities, error);
    controls.checkpoint(crate::ConstructionPhase::Resolution)?;
    let started = std::time::Instant::now();
    let resolved = Lowered::from((storage, sources))
        .resolve(library)
        .map_err(|error| failure(crate::ConstructionPhase::Resolution, error))?;
    let resolve = started.elapsed();
    let started = std::time::Instant::now();
    controls.checkpoint(crate::ConstructionPhase::Evaluation)?;
    let evaluated = resolved.evaluate(policy);
    let evaluation = started.elapsed();
    let started = std::time::Instant::now();
    controls.checkpoint(crate::ConstructionPhase::Indexing)?;
    let (indexed, sources) = evaluated
        .index()
        .map_err(|error| failure(crate::ConstructionPhase::Indexing, error))?;
    let indexing = started.elapsed();
    let started = std::time::Instant::now();
    controls.checkpoint(crate::ConstructionPhase::Diagnostics)?;
    let complete = indexed
        .diagnose(sources, reported, standard_library_availability)
        .map_err(|error| failure(crate::ConstructionPhase::Diagnostics, error))?;
    let diagnostics = started.elapsed();
    controls.checkpoint(crate::ConstructionPhase::Diagnostics)?;
    let namespace_checks = complete.namespace_checks;
    let complete = complete.into_parts();
    Ok((
        complete.0,
        complete.1,
        SemanticPhaseDurations {
            resolve,
            evaluation,
            indexing,
            diagnostics,
            namespace_checks,
        },
    ))
}

fn resolution_failure(
    phase: crate::ConstructionPhase,
    identities: &[Box<str>],
    error: ResolutionError,
) -> crate::BuildFailure {
    let documents: Vec<&str> = match error
        .document()
        .and_then(|document| identities.get(document.index()))
    {
        Some(identity) => vec![identity.as_ref()],
        None => identities.iter().map(|uri| uri.as_ref()).collect(),
    };
    crate::BuildFailure::construction(phase, documents, error)
}

#[cfg(test)]
mod failure_tests {
    use super::{resolution_failure, ResolutionError};
    use crate::model::DocumentIdx;
    use crate::{BuildFailure, ConstructionPhase};

    #[test]
    fn invalid_storage_reports_the_owning_phase_and_failed_document() {
        let identities = ["memory://short.sysml".into(), "memory://long.sysml".into()];
        let failure = resolution_failure(
            ConstructionPhase::Diagnostics,
            &identities,
            ResolutionError::InvalidStorage.in_document(DocumentIdx(1)),
        );
        let BuildFailure::ConstructionFailed {
            phase,
            documents,
            cause,
        } = &failure
        else {
            panic!("construction failure");
        };
        assert_eq!(*phase, ConstructionPhase::Diagnostics);
        assert_eq!(
            documents.as_ref(),
            [Box::<str>::from("memory://long.sysml")]
        );
        assert_eq!(cause, "InvalidDocumentStorage(DocumentIdx(1))");
        assert_eq!(failure.to_string(), "Diagnostics failed for documents [\"memory://long.sysml\"]: InvalidDocumentStorage(DocumentIdx(1))");
    }

    #[test]
    fn model_wide_failure_reports_all_inputs_without_guessing_a_failed_source() {
        let identities = ["memory://a.sysml".into(), "memory://b.sysml".into()];
        let failure = resolution_failure(
            ConstructionPhase::Indexing,
            &identities,
            ResolutionError::Capacity,
        );
        let BuildFailure::ConstructionFailed {
            phase,
            documents,
            cause,
        } = failure
        else {
            panic!("construction failure");
        };
        assert_eq!(phase, ConstructionPhase::Indexing);
        assert_eq!(documents.as_ref(), identities);
        assert_eq!(cause, "Capacity");
    }

    #[test]
    fn model_wide_failure_message_names_a_bounded_document_prefix() {
        let identities: Vec<Box<str>> = (0..5)
            .map(|index| format!("memory://{index}.sysml").into())
            .collect();
        let failure = resolution_failure(
            ConstructionPhase::Resolution,
            &identities,
            ResolutionError::Capacity,
        );
        assert_eq!(
            failure.to_string(),
            "Resolution failed for documents [\"memory://0.sysml\", \"memory://1.sysml\", \"memory://2.sysml\"] and 2 more: Capacity"
        );
    }
}
