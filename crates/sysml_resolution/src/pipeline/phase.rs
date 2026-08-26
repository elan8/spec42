//! The phase order of semantic construction, expressed as types.
//!
//! Each phase's writer consumes the previous phase's product **by value** and yields the next as a
//! distinct type:
//!
//! ```text
//! Lowered ─► Resolved ─► Evaluated ─► Indexed ─► Complete
//! ```
//!
//! Two properties follow by construction rather than by discipline:
//!
//! * no phase can write back into an earlier phase's store, because that store has been moved; and
//! * there is no half-built model to observe — `ResolvedSemanticModel` is reachable only out of
//!   `Complete`, and `Complete` cannot be built without a settled diagnostic product.

use crate::evaluate::compute_evaluation;
use crate::evaluate::EvaluationFact;
use crate::evaluate::SettledEvaluation;
use crate::evaluation::EvaluationPolicy;
use crate::index::bindings::BindingConnectorIndex;
use crate::index::documents::DocumentIndex;
use crate::index::elements::ElementFactIndex;
use crate::index::expressions::ExpressionIndex;
use crate::index::expressions::ExpressionInputs;
use crate::index::expressions::SettledFilter;
use crate::index::identity::IdentityIndex;
use crate::index::qualified::QualifiedNameIndex;
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
use crate::resolve::implied::synthesize_invocation_expression_result_specializations;
use crate::resolve::implied::synthesize_operator_expression_result_specializations;
use crate::resolve::implied::synthesize_owned_cross_feature_typings;
use crate::resolve::implied::synthesize_semantic_metadata_specializations;
use crate::resolve::library_seed::SettledLibrary;
use crate::resolve::names::EffectiveScopeIndex;
use crate::resolve::names::MembershipIndex;
use crate::resolve::names::NameIndex;
use crate::resolve::resolve_dense;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
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
    memberships: MembershipIndex,
    resolution: ResolutionResults,
    evaluation: Box<[EvaluationFact]>,
    filter_conditions: Option<Box<[SettledFilter]>>,
    completeness: PublicationCompleteness,
}

/// Phase 8's product: the only value a `ResolvedSemanticModel` can be taken out of.
pub(crate) struct Complete {
    model: ResolvedSemanticModel,
    /// Kept only so a library-stratum build can hand the trees to the next publication; a
    /// workspace build drops them with this value.
    sources: ParsedSources,
}

impl From<(SemanticModelStorage, ParsedSources)> for Lowered {
    fn from((storage, sources): (SemanticModelStorage, ParsedSources)) -> Self {
        Self { storage, sources }
    }
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
        let seed = library
            .filter(|library| library.admits(&storage))
            .map(|library| library.outcomes.as_ref());
        let (direct_names, effective_imports, memberships, resolution) = resolve_dense(
            &storage.declarations,
            &storage.memberships,
            &storage.paths,
            &storage.references,
            seed,
        )?;
        // `checkPartDefinitionSpecialization` is an implied semantic fact, so its anchor and
        // relationships are settled here, before every index and diagnostic consumer below. The
        // lookup is owned by semantic construction: neither a renderer nor a validation rule gets
        // to rediscover `Parts::Part` from text or a display path.
        let library_anchors = library_specialization_anchors(&storage);
        let implied = synthesize_implied_relationships(&storage, &resolution, &library_anchors)?;
        let resolution = resolution.settle(implied, library_anchors);
        // Owned cross-feature typing depends on the owning end's effective types, including types
        // inherited through the implied relationships settled above. Consume the canonical type
        // index at an explicit sub-barrier, then replace the provisional relationship set before
        // publishing `Resolved`; no reader can observe the intermediate state.
        let resolution = if storage
            .declaration_facts
            .iter()
            .any(|facts| facts.cross_feature_projection.is_some())
        {
            let prerequisite_types = derive_effective_types(&storage, &resolution)?;
            let mut implied = resolution.implied_relationships.to_vec();
            implied.extend(
                synthesize_owned_cross_feature_typings(&storage, &prerequisite_types)?.into_vec(),
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
        } else {
            resolution
        };
        let resolution = if storage.metadata_annotations.is_empty() {
            resolution
        } else {
            let prerequisite_types = derive_effective_types(&storage, &resolution)?;
            let synthesis = synthesize_semantic_metadata_specializations(
                &storage,
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
            resolution.settle_semantic_metadata(
                implied.into_boxed_slice(),
                synthesis.projections,
                synthesis.status,
            )
        };
        let resolution = if storage.operator_expressions.is_empty() {
            resolution
        } else {
            let synthesis =
                synthesize_operator_expression_result_specializations(&storage, &resolution)?;
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
                synthesize_constructor_expression_result_specializations(&storage, &resolution)?;
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
                synthesize_feature_chain_expression_result_specializations(&storage, &resolution)?;
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
            let synthesis = synthesize_feature_reference_expression_result_specializations(
                &storage,
                &resolution,
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
            resolution.settle_feature_reference_expressions(
                implied.into_boxed_slice(),
                synthesis.projections,
                synthesis.status,
            )
        };
        let resolution = if storage.invocations.is_empty() {
            resolution
        } else {
            let prerequisite_types = derive_effective_types(&storage, &resolution)?;
            let synthesis = synthesize_invocation_expression_result_specializations(
                &storage,
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
        let bindings = BindingConnectorIndex::build(&self.storage, &self.resolution)?;
        // A barrier product, not a solver family: every type fact here is derived from settled
        // outcomes and feeds nothing back into scope, imports or inheritance. The resolver's own
        // ancestor closure for inherited names stays separate and unchanged -- widening that one
        // would silently change name resolution.
        let types = TypeIndex::build(&self.storage, &self.resolution)?;
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
        let indexed = Indexed {
            storage: self.storage,
            direct_names: self.direct_names,
            effective_imports: self.effective_imports,
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
    ) -> Result<Complete, ResolutionError> {
        let (diagnostics, by_document) = self.derive_diagnostics(&sources, reported)?;
        // The parse product is consumed here and goes no further: this is the barrier that makes
        // "a sealed publication holds no parse tree" true by construction rather than by review.
        Ok(Complete {
            sources,
            model: ResolvedSemanticModel {
                storage: self.storage,
                direct_names: self.direct_names,
                effective_imports: self.effective_imports,
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

/// The whole phase order in one expression, for the coordinator and for tests.
pub(crate) fn build_model(
    storage: SemanticModelStorage,
    sources: ParsedSources,
    policy: EvaluationPolicy,
    library: Option<&SettledLibrary>,
    reported: &[Box<str>],
) -> Result<(ResolvedSemanticModel, ParsedSources), ResolutionError> {
    let (indexed, sources) = Lowered::from((storage, sources))
        .resolve(library)?
        .evaluate(policy)
        .index()?;
    Ok(indexed.diagnose(sources, reported)?.into_parts())
}
