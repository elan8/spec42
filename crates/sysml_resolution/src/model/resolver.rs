//! Private batch resolution over the dense canonical semantic storage.
//!
//! Resolution first materializes canonical direct-name ranges, then solves import targets and
//! effective imported scopes together to a fixed point. Each pass visits the preclassified import
//! slots and indexed candidate ranges only; it never rescans declarations or all references for
//! an individual lookup. Downstream reference families read the frozen effective index after the
//! import barrier converges.

#[cfg(test)]
use crate::diagnose::parser_diagnostic_category;
#[cfg(test)]
#[cfg(test)]
use crate::evaluate::compute_evaluation;
use crate::evaluate::EvaluationFact;
#[cfg(test)]
use crate::evaluate::SettledEvaluation;
#[cfg(test)]
use crate::evaluation::EvaluationPolicy;
#[cfg(test)]
use crate::evaluation::EvaluationState;
use crate::index::bindings as binding;
use crate::index::documents::DocumentIndex;
use crate::index::elements as inspection;
use crate::index::expressions as expression;
use crate::index::identity::IdentityIndex;
use crate::index::qualified::QualifiedNameIndex;
use crate::index::reverse_references::ReverseReferenceIndex;
use crate::index::types;
#[cfg(test)]
use crate::lower::facts::AuthoredFilterCondition;
#[cfg(test)]
use crate::lower::facts::Declaration;
#[cfg(test)]
use crate::lower::facts::DeclarationFacts;
#[cfg(test)]
use crate::lower::facts::DeclarationModifiers;
#[cfg(test)]
use crate::lower::facts::FilterForm;
#[cfg(test)]
use crate::lower::facts::FilterPredicate;
#[cfg(test)]
use crate::lower::facts::MembershipRecord;
#[cfg(test)]
use crate::lower::facts::RelationshipFlags;
#[cfg(test)]
use crate::lower::intern::SymbolPathArena;
#[cfg(test)]
use crate::lower::intern::SymbolPathArenaBuilder;
#[cfg(test)]
use crate::lower::intern::SymbolTableBuilder;
use crate::lower::storage::SemanticModelStorage;
use crate::model::AuthoredReferenceId;
#[cfg(test)]
use crate::model::DeclarationId;
#[cfg(test)]
use crate::model::DeclarationKind;
#[cfg(test)]
use crate::model::DocumentIdx;
#[cfg(test)]
#[cfg(test)]
use crate::model::MembershipKind;
#[cfg(test)]
use crate::model::NameId;
#[cfg(test)]
use crate::model::ReferenceKind;
#[cfg(test)]
use crate::model::SymbolPathId;
#[cfg(test)]
use crate::model::Visibility;
#[cfg(test)]
use crate::resolve::implied::conditional_library_specialization_anchor_branch;
#[cfg(test)]
use crate::resolve::implied::conditional_library_specialization_predicate_holds;
#[cfg(test)]
use crate::resolve::implied::generated_conditional_library_specialization_rule_count;
#[cfg(test)]
use crate::resolve::implied::generated_library_redefinition_rule_count;
#[cfg(test)]
use crate::resolve::implied::generated_library_specialization_rule_count;
#[cfg(test)]
use crate::resolve::implied::library_specialization_anchors;
#[cfg(test)]
use crate::resolve::implied::library_specialization_rules;
#[cfg(test)]
use crate::resolve::implied::LibraryRedefinitionRule;
#[cfg(test)]
use crate::resolve::implied::LibrarySpecializationAnchor;
#[cfg(test)]
use crate::resolve::implied::LibrarySpecializationAnchorFacts;
#[cfg(test)]
use crate::resolve::implied::GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES;
#[cfg(test)]
use crate::resolve::implied::GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES;
#[cfg(test)]
use crate::resolve::implied::GENERATED_LIBRARY_REDEFINITION_RULES;
#[cfg(test)]
use crate::resolve::implied::GENERATED_LIBRARY_SPECIALIZATION_RULES;
#[cfg(test)]
use crate::resolve::implied::GENERATED_TYPE_DERIVED_FACT_RULES;
use crate::resolve::library_seed::SettledLibrary;
#[cfg(test)]
use crate::resolve::names::name_entry_sort_key;
use crate::resolve::names::EffectiveScopeIndex;
use crate::resolve::names::MembershipIndex;
use crate::resolve::names::NameIndex;
#[cfg(test)]
use crate::resolve::names::NameKey;
#[cfg(test)]
use crate::resolve::resolve_dense;
#[cfg(test)]
use crate::resolve::resolve_dense_with_limit;
#[cfg(test)]
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
#[cfg(test)]
use crate::resolve::results::ResolutionWork;
#[cfg(test)]
use crate::resolve::results::SolverStatus;
#[cfg(test)]
use crate::resolve::ResolutionReferenceFact;
use crate::Diagnostic;
#[cfg(test)]
use crate::DiagnosticCategory;
#[cfg(test)]
use crate::FeatureDerivedRelationshipCollection;
#[cfg(test)]
use crate::LibrarySpecializationAnchorBranch;
#[cfg(test)]
use spec42_constraint_manifest::LibrarySpecializationPredicate;
use sysml_contract::PublicationCompleteness;
#[cfg(test)]
use sysml_v2_parser::ast::Span;

pub(crate) mod details;

/// The note attached to each declaration an ambiguous reference could have named.
pub(crate) const RELATED_AMBIGUOUS_CANDIDATE: &str = "Candidate this reference could name.";

/// The diagnostics of a publication that has not reached the diagnose barrier yet.
///
/// A distinct type rather than an empty store: an empty diagnostic sequence is a real answer
/// ("this model reported nothing"), and a model that has not derived its diagnostics must not be
/// able to give it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NotYetDiagnosed;

/// The settled phase-8 product, carried as one value so the sequence and its per-document ranges
/// are published together or not at all.
#[derive(Debug, Default)]
pub(crate) struct SettledDiagnostics {
    pub(crate) diagnostics: Box<[Diagnostic]>,
    /// Where each document's diagnostics begin and end inside `diagnostics`, by `DocumentIdx`.
    ///
    /// The derivation groups one document's diagnostics contiguously, so a document-scoped query
    /// is a slice of the settled sequence rather than a scan of it. Built at the same barrier so
    /// the two can never disagree.
    pub(crate) by_document: Box<[(u32, u32)]>,
}

/// The assembled model, parameterised by how far the diagnose barrier has got.
///
/// `Indexed` is the phase-6 product; `ResolvedSemanticModel` is the phase-8 one. Every read-only
/// method that does not consult diagnostics is available on both, and no code path can construct
/// the latter without a settled `SettledDiagnostics` value to put in it.
#[derive(Debug)]
pub(crate) struct SemanticModel<D> {
    pub(crate) storage: SemanticModelStorage,
    pub(crate) direct_names: NameIndex,
    pub(crate) effective_imports: NameIndex,
    pub(crate) identities: IdentityIndex,
    /// Every declaration's `::`-joined display path, settled at the barrier into one blob.
    pub(crate) qualified_names: QualifiedNameIndex,
    pub(crate) documents: DocumentIndex,
    pub(crate) memberships: MembershipIndex,
    pub(crate) reverse_references: ReverseReferenceIndex,
    pub(crate) effective_scopes: EffectiveScopeIndex,
    pub(crate) facts: inspection::ElementFactIndex,
    /// Canonical paired binding-connector endpoints, assembled once after resolution.
    pub(crate) bindings: binding::BindingConnectorIndex,
    pub(crate) types: types::TypeIndex,
    pub(crate) resolution: ResolutionResults,
    pub(crate) evaluation: Box<[EvaluationFact]>,
    /// Settled unit, measurement and filter facts over the expressions this publication admitted.
    pub(crate) expressions: expression::ExpressionIndex,
    /// Settled at the publication barrier alongside the indexes, so reading them is a lookup and
    /// a broken storage invariant fails the build instead of a later query.
    pub(crate) diagnostics: D,
    pub(crate) metadata: PublicationMetadata,
}

/// The phase-6 product: every index settled, diagnostics not yet derived.
pub(crate) type Indexed = SemanticModel<NotYetDiagnosed>;

/// The phase-8 product: the only shape the query surface and the facade ever see.
pub(crate) type ResolvedSemanticModel = SemanticModel<SettledDiagnostics>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublicationPhase {
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PublicationMetadata {
    pub(crate) phase: PublicationPhase,
    pub(crate) completeness: PublicationCompleteness,
    pub(crate) has_evaluation: bool,
}

impl<D> SemanticModel<D> {
    /// Everything a later publication needs to reuse this one as a library.
    ///
    /// The parsed documents come out by reference-counted handle rather than by copy, so reuse
    /// shares one parse of the library across every publication built against it.
    ///
    /// The trees come from the build's own parse product rather than from the model: the model does
    /// not have them, which is the point -- a sealed publication holds no parse tree, so reuse is
    /// arranged by the builder that still owns them.
    pub(crate) fn prepared_library(
        &self,
        sources: crate::lower::storage::ParsedSources,
    ) -> Result<crate::pipeline::PreparedLibrary, crate::pipeline::CoordinatorError> {
        let documents = sources
            .into_documents()
            .into_iter()
            .map(|document| crate::pipeline::PreparedDocument {
                identity: document.identity,
                role: document.role,
                digest: document.digest,
                parsed: document.parsed,
                parse_errors: document.parse_errors.into_vec(),
            })
            .collect();
        Ok(crate::pipeline::PreparedLibrary {
            documents,
            settled: self
                .settled_library()
                .map_err(|_| crate::pipeline::CoordinatorError::ConstructionFailed)?,
        })
    }

    /// The reusable settled state of a library-only publication.
    pub(crate) fn settled_library(&self) -> Result<SettledLibrary, ResolutionError> {
        let mut root_names = std::collections::BTreeSet::new();
        for declaration in self.storage.declarations.iter() {
            if declaration.owner.is_some() {
                continue;
            }
            if let Some(name) = declaration.name.and_then(|name| self.storage.symbol(name)) {
                root_names.insert(name.into());
            }
        }
        let mut unsettled_roots = std::collections::BTreeSet::new();
        for (index, reference) in self.storage.references.iter().enumerate() {
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let settled = matches!(
                self.resolution.outcome(id),
                Some(ResolutionStatus::Resolved(_)) | Some(ResolutionStatus::Unsupported)
            );
            if settled {
                continue;
            }
            let Some((segments, _)) = self.storage.paths.get(reference.path) else {
                continue;
            };
            let Some(first) = segments.first().and_then(|id| self.storage.symbol(*id)) else {
                continue;
            };
            unsettled_roots.insert(first.into());
        }
        Ok(SettledLibrary {
            outcomes: self.resolution.outcomes.clone(),
            root_names,
            unsettled_roots,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower::facts::AuthoredExpression;
    use crate::lower::facts::CanonicalDocument;
    use crate::lower::facts::ExpressionGrammar;
    use source_identity::SourceRole;
    use sysml_v2_parser::ast::{Expression, QualifiedReferenceArena, RootNamespace, SourceStorage};
    use sysml_v2_parser::ParsedDocument;

    #[test]
    fn generated_feature_relationship_collection_contracts_are_complete_and_closed() {
        assert_eq!(GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES.len(), 5);
        assert_eq!(
            GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES
                .iter()
                .map(|rule| (rule.rule_id, rule.metaclass, rule.collection))
                .collect::<Vec<_>>(),
            vec![
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedFeatureChaining,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedRedefinition",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedRedefinition,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedSubsetting",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedSubsetting,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTypeFeaturing",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedTypeFeaturing,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedTyping,
                ),
            ]
        );
    }

    #[test]
    fn generated_type_fact_contract_rows_are_canonical_rule_id_ordered() {
        assert!(GENERATED_TYPE_DERIVED_FACT_RULES
            .windows(2)
            .all(|pair| pair[0].rule_id < pair[1].rule_id));
    }

    #[test]
    fn generated_unconditional_library_specialization_table_covers_all_manifest_check_rules() {
        assert_eq!(generated_library_specialization_rule_count(), 85);
        assert_eq!(
            generated_conditional_library_specialization_rule_count(),
            58
        );
        let unique_rules = GENERATED_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .map(|rule| rule.rule_id)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            unique_rules.len(),
            85,
            "rule IDs must remain unique map keys"
        );
        assert!(GENERATED_LIBRARY_SPECIALIZATION_RULES.iter().all(|rule| {
            !rule.rule_id.is_empty() && !rule.metaclass.is_empty() && !rule.anchor.is_empty()
        }));
        assert!(GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .all(|rule| {
                !rule.rule_id.is_empty()
                    && !rule.metaclass.is_empty()
                    && !rule.anchor.is_empty()
                    && (rule.predicate != LibrarySpecializationPredicate::CompositeOwnedBy
                        || rule.owner_metaclasses.len() == 2)
            }));
        assert_eq!(generated_library_redefinition_rule_count(), 1);
        assert_eq!(
            GENERATED_LIBRARY_REDEFINITION_RULES,
            &[LibraryRedefinitionRule {
                rule_id: "kerml-1.0:8.3.4.9.5:checkPayloadFeatureRedefinition",
                metaclass: "PayloadFeature",
                anchor: "Transfers::Transfer::payload",
            }]
        );
        for rule in GENERATED_LIBRARY_SPECIALIZATION_RULES {
            assert_eq!(
                library_specialization_rules(rule.metaclass).count(),
                1,
                "{} must retain its exact generated applicability metaclass",
                rule.rule_id
            );
        }

        // The stored fact owner is total over the generated table even when the admitted model
        // contains no library declarations. That makes every missing prerequisite explicit, and
        // catches a manifest/generator change that adds a rule without publication coverage.
        let anchors = library_specialization_anchors(&storage_with_one_filter());
        assert_eq!(anchors.by_rule.len(), 150);
        for rule in GENERATED_LIBRARY_SPECIALIZATION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
        }
        for rule in GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
            if rule.true_anchor.is_some() {
                assert!(matches!(
                    anchors.outcome_for(
                        rule.rule_id,
                        LibrarySpecializationAnchorBranch::PredicateTrue,
                    ),
                    Some(LibrarySpecializationAnchor::Missing)
                ));
            }
        }
        for rule in GENERATED_LIBRARY_REDEFINITION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
        }
        let shared_anchor_rules = GENERATED_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .filter(|rule| rule.anchor == "Performances::literalIntegerEvaluations")
            .collect::<Vec<_>>();
        assert_eq!(shared_anchor_rules.len(), 2);
        assert_ne!(
            shared_anchor_rules[0].rule_id,
            shared_anchor_rules[1].rule_id
        );
        assert!(matches!(
            anchors.outcome(shared_anchor_rules[0].rule_id),
            Some(LibrarySpecializationAnchor::Missing)
        ));
        assert!(matches!(
            anchors.outcome(shared_anchor_rules[1].rule_id),
            Some(LibrarySpecializationAnchor::Missing)
        ));
    }

    #[test]
    fn generated_conditional_specialization_rows_preserve_the_typed_manifest_contract() {
        let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("specifications/constraint_manifest.toml");
        let manifest = spec42_constraint_manifest::ConstraintManifest::load_toml(&manifest_path)
            .expect("load typed manifest for generated-row drift test");
        let mut expected = manifest
            .specifications
            .iter()
            .flat_map(|specification| &specification.constraints)
            .filter_map(|entry| {
                entry
                    .conditional_specializes_from_library
                    .as_ref()
                    .map(|contract| {
                        (
                            entry.rule_id.clone(),
                            entry.metaclass.clone(),
                            contract.predicate,
                            contract.owner_metaclasses.clone(),
                            contract.true_anchor.clone(),
                            contract.anchor.clone(),
                        )
                    })
            })
            .collect::<Vec<_>>();
        expected.sort_by(|left, right| left.0.cmp(&right.0));
        let actual = GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .map(|rule| {
                (
                    rule.rule_id.to_string(),
                    rule.metaclass.to_string(),
                    rule.predicate,
                    rule.owner_metaclasses
                        .iter()
                        .map(|metaclass| (*metaclass).to_string())
                        .collect::<Vec<_>>(),
                    rule.true_anchor.map(str::to_string),
                    rule.anchor.to_string(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn membership_role_specialization_predicates_use_the_canonical_role_and_owner_facts() {
        let storage = storage_with_membership_role_specializations();
        let rule = |id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == id)
                .expect("generated membership-role rule")
        };
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");

        let framed = rule("sysml-2.0:8.3.21.4:checkConcernUsageFramedConcernSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(2),
            framed
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(9),
            framed
        ));

        let constraint =
            rule("sysml-2.0:8.3.20.4:checkConstraintUsageRequirementConstraintSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            constraint
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(3), constraint),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(4), constraint),
            LibrarySpecializationAnchorBranch::Default,
        );

        let actor = rule("sysml-2.0:8.3.11.3:checkPartUsageActorSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            actor
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(5), actor),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(6), actor),
            LibrarySpecializationAnchorBranch::Default,
        );
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(9),
            actor
        ));

        let stakeholder = rule("sysml-2.0:8.3.11.3:checkPartUsageStakeholderSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(8),
            stakeholder
        ));

        let verification =
            rule("sysml-2.0:8.3.21.9:checkRequirementUsageRequirementVerificationSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(10),
            verification
        ));
    }

    #[test]
    fn accept_action_specialization_predicates_use_trigger_and_subaction_facts() {
        let storage = storage_with_accept_action_specializations();
        let rule = |id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == id)
                .expect("generated accept-action rule")
        };
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let ordinary = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageSpecialization");
        let subaction = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageSubactionSpecialization");
        let trigger = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageTriggerActionSpecialization");

        // A top-level accept action is non-trigger but not a subaction.
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            ordinary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            subaction
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            trigger
        ));

        // The exact owner/composite facts add the subaction specialization without changing the
        // non-trigger one.
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            ordinary
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            subaction
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            trigger
        ));

        // A transition trigger is explicitly suppressed from both non-trigger rules.
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            ordinary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            subaction
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            trigger
        ));
    }

    #[test]
    fn if_action_specialization_selects_the_canonical_else_action_branch() {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let storage = SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentIdx(0), None, None, DeclarationKind::Package),
                declaration(DocumentIdx(0), Some(id(0)), None, DeclarationKind::If),
                declaration(DocumentIdx(0), Some(id(0)), None, DeclarationKind::If),
                declaration(DocumentIdx(0), Some(id(0)), None, DeclarationKind::If),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                DeclarationFacts::none(),
                DeclarationFacts {
                    has_else_action: Some(false),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    has_else_action: Some(true),
                    ..DeclarationFacts::none()
                },
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            relationship_declarations: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            operator_expressions: Box::new([]),
            expression_arguments: Box::new([]),
            constructor_expressions: Box::new([]),
            feature_chain_expressions: Box::new([]),
            feature_reference_expressions: Box::new([]),
            metadata_annotations: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        };
        let rule = GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .find(|rule| rule.rule_id == "sysml-2.0:8.3.17.10:checkIfActionUsageSpecialization")
            .expect("generated if-action rule");

        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            rule
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(2),
            rule
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            rule
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(2), rule),
            LibrarySpecializationAnchorBranch::Default,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(3), rule),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
    }

    #[test]
    fn flow_specialization_predicates_use_owned_endpoint_facts_and_suppress_incomplete_forms() {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let storage = SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentIdx(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentIdx(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::FlowDefinition,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::FlowDefinition,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(4)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(DocumentIdx(0), Some(id(0)), None, DeclarationKind::Flow),
                declaration(DocumentIdx(0), Some(id(0)), None, DeclarationKind::Flow),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                DeclarationFacts::none(),
                DeclarationFacts {
                    positional_end: Some(0),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    positional_end: Some(1),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts::none(),
                DeclarationFacts {
                    positional_end: Some(0),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    owned_end_feature_count: Some(2),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts::none(),
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            relationship_declarations: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            operator_expressions: Box::new([]),
            expression_arguments: Box::new([]),
            constructor_expressions: Box::new([]),
            feature_chain_expressions: Box::new([]),
            feature_reference_expressions: Box::new([]),
            metadata_annotations: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        };
        let rule = |rule_id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == rule_id)
                .expect("generated flow rule")
        };
        let binary = rule("sysml-2.0:8.3.16.2:checkFlowDefinitionBinarySpecialization");
        let flow_usage = rule("sysml-2.0:8.3.16.3:checkFlowUsageFlowSpecialization");
        let flow_with_ends = rule("kerml-1.0:8.3.4.9.2:checkFlowWithEndsSpecialization");

        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            binary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(4),
            binary
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(6),
            flow_usage
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(6),
            flow_with_ends
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(7),
            flow_usage
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(7),
            flow_with_ends
        ));
    }

    #[test]
    fn parser_categories_are_mapped_without_code_or_message_heuristics() {
        assert_eq!(
            parser_diagnostic_category(Some(sysml_v2_parser::DiagnosticCategory::ParseError)),
            DiagnosticCategory::MalformedSyntax
        );
        assert_eq!(
            parser_diagnostic_category(Some(
                sysml_v2_parser::DiagnosticCategory::UnsupportedGrammarForm
            )),
            DiagnosticCategory::UnsupportedSyntax
        );
        assert_eq!(
            parser_diagnostic_category(Some(sysml_v2_parser::DiagnosticCategory::UnresolvedSymbol)),
            DiagnosticCategory::Unresolved
        );
        assert_eq!(
            parser_diagnostic_category(None),
            DiagnosticCategory::UnclassifiedParser
        );
    }

    #[derive(Debug, Clone, Copy)]
    struct TestReference {
        source: DeclarationId,
        kind: ReferenceKind,
        path: SymbolPathId,
        flags: RelationshipFlags,
    }

    impl ResolutionReferenceFact for TestReference {
        fn source(&self) -> DeclarationId {
            self.source
        }

        fn kind(&self) -> ReferenceKind {
            self.kind
        }

        fn path(&self) -> SymbolPathId {
            self.path
        }

        fn flags(&self) -> RelationshipFlags {
            self.flags
        }
    }

    fn declaration(
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        name: Option<NameId>,
        kind: DeclarationKind,
    ) -> Declaration {
        Declaration {
            document,
            owner,
            name,
            anonymous_ordinal: name.is_none().then_some(0),
            kind,
            span: Span::dummy(),
        }
    }

    /// A storage holding nothing but one authored `filter` condition.
    ///
    /// Enough for the evaluation pass, which reads the condition table, the evaluation candidates
    /// and the references, and nothing else.
    /// A document with an empty parser arena, enough for a fixture whose expressions are
    /// self-contained literals.
    fn empty_canonical_document() -> CanonicalDocument {
        CanonicalDocument {
            identity: "test".into(),
            role: SourceRole::Workspace,
            lines: crate::lower::facts::LineIndex::build(""),
        }
    }

    /// The matching parse product: classification reads the owning document's parser arena, which
    /// is no longer part of the storage.
    fn empty_parsed_sources() -> crate::lower::storage::ParsedSources {
        crate::lower::storage::ParsedSources::new(vec![crate::lower::facts::AdmittedDocument {
            digest: source_identity::ContentDigest::of_bytes(&[]),
            identity: "test".into(),
            role: SourceRole::Workspace,
            parsed: std::sync::Arc::new(ParsedDocument {
                source: SourceStorage::default(),
                qualified_references: QualifiedReferenceArena::default(),
                root: RootNamespace {
                    elements: Vec::new(),
                },
            }),
            parse_errors: Box::new([]),
        }])
    }

    fn storage_with_one_filter() -> SemanticModelStorage {
        SemanticModelStorage {
            // One empty document, because classifying the condition below reads its owning
            // document's parser arena -- classification is evaluation's, and it happens over the
            // authored site rather than over anything lowering pre-computed.
            documents: Box::new([empty_canonical_document()]),
            declarations: Box::new([]),
            declaration_facts: Box::new([]),
            memberships: Box::new([]),
            references: Box::new([]),
            relationship_declarations: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            operator_expressions: Box::new([]),
            expression_arguments: Box::new([]),
            constructor_expressions: Box::new([]),
            feature_chain_expressions: Box::new([]),
            feature_reference_expressions: Box::new([]),
            metadata_annotations: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([AuthoredFilterCondition {
                owner: DeclarationId(0),
                document: DocumentIdx(0),
                form: FilterForm::View,
                span: Span::dummy(),
                expression: AuthoredExpression {
                    document: DocumentIdx(0),
                    grammar: ExpressionGrammar::Constraint,
                    operand_start: 0,
                    node: Expression::LiteralInteger(5),
                },
                predicate: FilterPredicate::Unsupported,
            }]),
            invocations: Box::new([]),
        }
    }

    /// One semantic storage slice with every typed membership role used by the exact generated
    /// specialization contracts. No parser text or declaration name participates in the test:
    /// applicability and branch selection consume the canonical declaration kind/owner facts.
    fn storage_with_membership_role_specializations() -> SemanticModelStorage {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentIdx(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentIdx(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::RequirementDefinition,
                ),
                declaration(DocumentIdx(0), Some(id(1)), None, DeclarationKind::Frame),
                declaration(
                    DocumentIdx(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::AssumeConstraintUsage,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::RequireConstraintUsage,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::RequirementActor,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::CaseDefinition,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(6)),
                    None,
                    DeclarationKind::CaseActor,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::StakeholderUsage,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::PartUsage,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::VerifyRequirement,
                ),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![DeclarationFacts::none(); 11].into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            relationship_declarations: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            operator_expressions: Box::new([]),
            expression_arguments: Box::new([]),
            constructor_expressions: Box::new([]),
            feature_chain_expressions: Box::new([]),
            feature_reference_expressions: Box::new([]),
            metadata_annotations: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        }
    }

    /// Canonical fact slice for the three exact `AcceptActionUsage` predicates. It separates the
    /// action's metaclass, its explicit trigger membership fact, and the independently owned
    /// composite/owner facts used by `isSubactionUsage()`.
    fn storage_with_accept_action_specializations() -> SemanticModelStorage {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let non_trigger = |composite| DeclarationFacts {
            modifiers: DeclarationModifiers {
                composite,
                ..DeclarationModifiers::default()
            },
            is_trigger_action: Some(false),
            ..DeclarationFacts::none()
        };
        SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentIdx(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentIdx(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::ActionDefinition,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(2)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::Transition,
                ),
                declaration(
                    DocumentIdx(0),
                    Some(id(4)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                non_trigger(true),
                DeclarationFacts::none(),
                non_trigger(true),
                DeclarationFacts::none(),
                DeclarationFacts {
                    modifiers: DeclarationModifiers {
                        composite: true,
                        ..DeclarationModifiers::default()
                    },
                    is_trigger_action: Some(true),
                    ..DeclarationFacts::none()
                },
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            relationship_declarations: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            operator_expressions: Box::new([]),
            expression_arguments: Box::new([]),
            constructor_expressions: Box::new([]),
            feature_chain_expressions: Box::new([]),
            feature_reference_expressions: Box::new([]),
            metadata_annotations: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        }
    }

    fn resolution_with_status(status: SolverStatus) -> ResolutionResults {
        ResolutionResults {
            outcomes: Box::new([]),
            ambiguous_candidates: Box::new([]),
            inherited_names: NameIndex::build(Vec::new()).unwrap(),
            solver_status: status,
            implied_relationships: Box::new([]),
            authored_relationships: Box::new([]),
            library_specialization_anchors: LibrarySpecializationAnchorFacts::default(),
            semantic_metadata_projections: Box::new([]),
            semantic_metadata_projection_status: Default::default(),
            select_expression_projection_status: Default::default(),
            index_expression_projection_status: Default::default(),
            index_expression_array_anchor: None,
            constructor_expression_projection_status: Default::default(),
            constructor_expression_projections: Box::default(),
            constructor_expression_specialization_status: Default::default(),
            constructor_expression_anchor: None,
            feature_chain_expression_specialization_status: Default::default(),
            feature_chain_expression_projections: Box::default(),
            feature_reference_expression_status: Default::default(),
            feature_reference_expression_projections: Box::default(),
            invocation_expression_projection_status: Default::default(),
            invocation_expression_projections: Box::default(),
            work: ResolutionWork::default(),
        }
    }

    /// A converged publication settles every authored condition.
    #[test]
    fn every_authored_filter_condition_settles_when_resolution_converges() {
        let storage = storage_with_one_filter();
        let settled = compute_evaluation(
            &storage,
            &empty_parsed_sources(),
            &resolution_with_status(SolverStatus::Converged),
            EvaluationPolicy::Evaluate,
        );
        let SettledEvaluation::Settled { filters, .. } = settled else {
            panic!("a converged publication settles its filter conditions");
        };
        assert_eq!(filters.len(), storage.filter_conditions.len());
        assert_eq!(
            filters[0].state,
            EvaluationState::Literal(crate::evaluation::EvaluatedScalar::Integer(5))
        );
    }

    /// A publication whose resolution did not converge has no outcomes to publish, and saying so
    /// is not the same as failing to build.
    ///
    /// The evaluation pass and the filter table used to be joined by index, so the branch that
    /// could not produce an outcome per condition published none and the join rejected the whole
    /// publication -- turning an explicitly supported incomplete state into a construction error
    /// for any model that authored a `filter`.
    #[test]
    fn a_non_converged_publication_settles_nothing_rather_than_failing() {
        let storage = storage_with_one_filter();
        let settled = compute_evaluation(
            &storage,
            &empty_parsed_sources(),
            &resolution_with_status(SolverStatus::NonConverged),
            EvaluationPolicy::Evaluate,
        );
        assert!(
            matches!(settled, SettledEvaluation::Vacuous),
            "a non-converged publication must settle no expression outcome"
        );
    }

    fn reference(
        source: DeclarationId,
        kind: ReferenceKind,
        path: SymbolPathId,
        wildcard: bool,
    ) -> TestReference {
        TestReference {
            source,
            kind,
            path,
            flags: RelationshipFlags {
                wildcard,
                ..RelationshipFlags::default()
            },
        }
    }

    struct ResolverFixture {
        declarations: Box<[Declaration]>,
        memberships: Box<[MembershipRecord]>,
        paths: SymbolPathArena,
        references: Box<[TestReference]>,
    }

    fn memberships_for(
        declarations: &[Declaration],
        public_imports: &[DeclarationId],
    ) -> Box<[MembershipRecord]> {
        declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                MembershipRecord {
                    member,
                    kind: if declaration.kind == DeclarationKind::Import {
                        MembershipKind::Import
                    } else {
                        MembershipKind::Owning
                    },
                    visibility: if public_imports.contains(&member) {
                        Visibility::Public
                    } else {
                        Visibility::Default
                    },
                    role: None,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn cross_file_fixture(duplicate_vehicle: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let definitions_name = symbols.intern("Definitions").unwrap();
        let usage_name = symbols.intern("Usage").unwrap();
        let vehicle_name = symbols.intern("Vehicle").unwrap();
        let v_name = symbols.intern("v").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let definitions_path = paths.push(&[definitions_name], false).unwrap();
        let vehicle_path = paths.push(&[vehicle_name], false).unwrap();

        let definition_document = DocumentIdx(0);
        let usage_document = DocumentIdx(1);
        let definitions = DeclarationId(0);
        let usage = DeclarationId(2);
        let import = DeclarationId(3);
        let v = DeclarationId(4);
        let mut declarations = vec![
            declaration(
                definition_document,
                None,
                Some(definitions_name),
                DeclarationKind::Package,
            ),
            declaration(
                definition_document,
                Some(definitions),
                Some(vehicle_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                usage_document,
                None,
                Some(usage_name),
                DeclarationKind::Package,
            ),
            declaration(usage_document, Some(usage), None, DeclarationKind::Import),
            declaration(
                usage_document,
                Some(usage),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        if duplicate_vehicle {
            declarations.push(declaration(
                definition_document,
                Some(definitions),
                Some(vehicle_name),
                DeclarationKind::PartDefinition,
            ));
        }

        let references = vec![
            reference(
                import,
                ReferenceKind::NamespaceImport,
                definitions_path,
                true,
            ),
            reference(v, ReferenceKind::FeatureTyping, vehicle_path, false),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn resolve_fixture(
        fixture: &ResolverFixture,
    ) -> (NameIndex, NameIndex, MembershipIndex, ResolutionResults) {
        resolve_dense(
            &fixture.declarations,
            None,
            &fixture.memberships,
            &fixture.paths,
            &fixture.references,
            None,
        )
        .unwrap()
    }

    fn transitive_import_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let c_name = symbols.intern("C").unwrap();
        let use_name = symbols.intern("Use").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();
        let c_path = paths.push(&[c_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentIdx(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(0)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentIdx(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(1),
                Some(DeclarationId(2)),
                None,
                DeclarationKind::Import,
            ),
            declaration(DocumentIdx(2), None, Some(c_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(2),
                Some(DeclarationId(4)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentIdx(3),
                None,
                Some(use_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(3),
                Some(DeclarationId(6)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentIdx(3),
                Some(DeclarationId(6)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(3),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(5),
                ReferenceKind::NamespaceImport,
                b_path,
                true,
            ),
            reference(
                DeclarationId(7),
                ReferenceKind::NamespaceImport,
                c_path,
                true,
            ),
            reference(
                DeclarationId(8),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(3), DeclarationId(5)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn imported_target_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let nested_name = symbols.intern("Nested").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let nested_path = paths.push(&[nested_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentIdx(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(0)),
                Some(nested_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(1)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentIdx(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentIdx(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentIdx(1),
                Some(DeclarationId(3)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(4),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(5),
                ReferenceKind::NamespaceImport,
                nested_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn cyclic_import_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let type_a_name = symbols.intern("TypeA").unwrap();
        let type_b_name = symbols.intern("TypeB").unwrap();
        let usage_name = symbols.intern("value").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let b_path = paths.push(&[b_name], false).unwrap();
        let a_path = paths.push(&[a_name], false).unwrap();
        let type_b_path = paths.push(&[type_b_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentIdx(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(0)),
                Some(type_a_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(0)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(0)),
                Some(usage_name),
                DeclarationKind::PartUsage,
            ),
            declaration(DocumentIdx(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(1),
                Some(DeclarationId(4)),
                Some(type_b_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(1),
                Some(DeclarationId(4)),
                None,
                DeclarationKind::Import,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(2),
                ReferenceKind::NamespaceImport,
                b_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(3),
                ReferenceKind::FeatureTyping,
                type_b_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(2), DeclarationId(6)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn qualified_import_target_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let nested_name = symbols.intern("Nested").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let use_name = symbols.intern("Use").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let qualified_nested_path = paths.push(&[b_name, nested_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentIdx(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(0)),
                Some(nested_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(1)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentIdx(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentIdx(2),
                None,
                Some(use_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(2),
                Some(DeclarationId(5)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentIdx(2),
                Some(DeclarationId(5)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(4),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::NamespaceImport,
                qualified_nested_path,
                true,
            ),
            reference(
                DeclarationId(7),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(4)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn redefinition_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let p_name = symbols.intern("P").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let child_name = symbols.intern("Child").unwrap();
        let mass_name = symbols.intern("mass").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let child = DeclarationId(3);
        let declarations = vec![
            declaration(DocumentIdx(0), None, Some(p_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(base),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(child_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(child),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::PartUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    role: None,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            child,
            ReferenceKind::Subclassification,
            base_path,
            false,
        )];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn same_name_direct_parent_feature_synthesizes_implied_redefinition() {
        let fixture = redefinition_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.implied_relationships.as_ref(),
            &[ImpliedRelationship {
                kind: ReferenceKind::Redefinition,
                source: DeclarationId(4),
                target: DeclarationId(2),
            }]
        );
    }

    #[test]
    fn explicit_redefinition_suppresses_implied_duplicate() {
        let mut fixture = redefinition_fixture();
        let mut references = fixture.references.into_vec();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Redefinition,
            references[0].path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert!(resolution.implied_relationships.is_empty());
    }

    #[test]
    fn ambiguous_immediate_parent_candidates_leave_no_implied_fact() {
        let mut fixture = redefinition_fixture();
        let mut declarations = fixture.declarations.into_vec();
        // A second directly owned `mass` feature on Base makes the immediate-parent same-name
        // lookup ambiguous; the synthesis must not guess a target.
        declarations.push(declaration(
            DocumentIdx(0),
            Some(DeclarationId(1)),
            Some(declarations[2].name.unwrap()),
            DeclarationKind::AttributeUsage,
        ));
        fixture.declarations = declarations.into_boxed_slice();
        let mut memberships = fixture.memberships.into_vec();
        memberships.push(MembershipRecord {
            member: DeclarationId(5),
            kind: MembershipKind::Feature,
            visibility: Visibility::Default,
            role: None,
            span: Span::dummy(),
        });
        fixture.memberships = memberships.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert!(resolution.implied_relationships.is_empty());
    }

    /// Like `redefinition_fixture`, but exposes an unqualified single-segment `mass` symbol path
    /// (in addition to the `Base` path used for the Subclassification reference) so callers can
    /// author an unqualified `Subsetting`/`Redefinition` reference to the inherited `mass`
    /// feature, resolved through lexical/ancestor lookup rather than a qualified path. `Child`'s
    /// own redefining/subsetting attribute (`DeclarationId(4)`) is deliberately left unnamed --
    /// matching an authored `attribute :>> mass = ...;`/`attribute :> mass;`, whose usage has no
    /// name of its own -- so it is never itself indexed under the `mass` name and cannot shadow
    /// the inherited `Base::mass` target it is trying to reach.
    fn redefinition_fixture_with_mass_path() -> (ResolverFixture, SymbolPathId) {
        let mut symbols = SymbolTableBuilder::default();
        let p_name = symbols.intern("P").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let child_name = symbols.intern("Child").unwrap();
        let mass_name = symbols.intern("mass").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();
        let mass_path = paths.push(&[mass_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let child = DeclarationId(3);
        let declarations = vec![
            declaration(DocumentIdx(0), None, Some(p_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(base),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(child_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(child),
                None,
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::PartUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    role: None,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            child,
            ReferenceKind::Subclassification,
            base_path,
            false,
        )];
        let _symbols = symbols.freeze();
        (
            ResolverFixture {
                declarations: declarations.into_boxed_slice(),
                memberships,
                paths: paths.freeze(),
                references: references.into_boxed_slice(),
            },
            mass_path,
        )
    }

    #[test]
    fn explicit_redefinition_resolves_through_inherited_ancestor_lookup() {
        // `attribute :>> mass = ...;` on `Child` (which specializes `Base`) must resolve its
        // authored `Redefinition` reference against `Base::mass`, an inherited member reachable
        // only through the ancestor-closure lookup built for Subclassification -- not just a
        // directly owned member of `Child` itself.
        let (mut fixture, mass_path) = redefinition_fixture_with_mass_path();
        let mut references = fixture.references.into_vec();
        let redefinition_index = u32::try_from(references.len()).unwrap();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Redefinition,
            mass_path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(redefinition_index)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn subsetting_reference_resolves_to_an_inherited_feature_target() {
        // `attribute simpleMass :> mass;` subsets another *feature*, not a type/definition, so
        // `Subsetting` must resolve against `DeclarationDomain::Any` rather than the
        // Subclassification/FeatureTyping `Type` domain, and must reach the inherited `Base::mass`
        // feature through the same ancestor-scoped inherited lookup used by
        // `FeatureTyping`/`Redefinition`.
        let (mut fixture, mass_path) = redefinition_fixture_with_mass_path();
        let mut references = fixture.references.into_vec();
        let subsetting_index = u32::try_from(references.len()).unwrap();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Subsetting,
            mass_path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(subsetting_index)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn redefinition_inside_a_usage_body_resolves_through_the_usages_feature_typing_target() {
        // `need : Need { attribute :>> status = ...; }` -- the redefining attribute is owned by a
        // *usage* (`need`), not a def/type, so it has no Subclassification ancestors of its own.
        // `status` is only reachable by first following `need`'s own `FeatureTyping` reference to
        // `Need`, then walking `Need`'s ancestor closure (`Need -> UserRequirement ->
        // ManagedRequirement`) to find `ManagedRequirement::status`. Mirrors
        // tests/snapshots/resolution/enum_status_redefinition.md.
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let managed_requirement_name = symbols.intern("ManagedRequirement").unwrap();
        let status_name = symbols.intern("status").unwrap();
        let user_requirement_name = symbols.intern("UserRequirement").unwrap();
        let need_def_name = symbols.intern("Need").unwrap();
        let need_usage_name = symbols.intern("need").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let managed_requirement_path = paths.push(&[managed_requirement_name], false).unwrap();
        let user_requirement_path = paths.push(&[user_requirement_name], false).unwrap();
        let need_def_path = paths.push(&[need_def_name], false).unwrap();
        let status_path = paths.push(&[status_name], false).unwrap();

        let demo = DeclarationId(0);
        let managed_requirement = DeclarationId(1);
        let status = DeclarationId(2);
        let user_requirement = DeclarationId(3);
        let need_def = DeclarationId(4);
        let need_usage = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(managed_requirement_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(managed_requirement),
                Some(status_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(user_requirement_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(need_def_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(need_usage_name),
                DeclarationKind::RequirementUsage,
            ),
            declaration(
                DocumentIdx(0),
                Some(need_usage),
                None,
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::RequirementUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    role: None,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let references = vec![
            reference(
                user_requirement,
                ReferenceKind::Subclassification,
                managed_requirement_path,
                false,
            ),
            reference(
                need_def,
                ReferenceKind::Subclassification,
                user_requirement_path,
                false,
            ),
            reference(
                need_usage,
                ReferenceKind::FeatureTyping,
                need_def_path,
                false,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::Redefinition,
                status_path,
                false,
            ),
        ];
        let redefinition_index = u32::try_from(references.len() - 1).unwrap();

        let _symbols = symbols.freeze();
        let fixture = ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        };

        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(redefinition_index)),
            Some(ResolutionStatus::Resolved(status))
        );
    }

    #[test]
    fn qualified_reference_resolves_to_an_enum_defs_owned_literal_member() {
        // `enum def StatusKind { enum approved; }` -- StatusKind::approved is looked up through
        // exactly the same generic multi-segment lexical lookup as any other owned member; no
        // enum-specific resolver code is needed once EnumerationDefinition/EnumerationLiteral are
        // lowered as ordinary owned declarations.
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let status_kind_name = symbols.intern("StatusKind").unwrap();
        let approved_name = symbols.intern("approved").unwrap();
        let alias_name = symbols.intern("aliasToApproved").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let qualified_path = paths
            .push(&[status_kind_name, approved_name], false)
            .unwrap();

        let demo = DeclarationId(0);
        let status_kind = DeclarationId(1);
        let approved = DeclarationId(2);
        let alias = DeclarationId(3);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(status_kind_name),
                DeclarationKind::EnumerationDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(status_kind),
                Some(approved_name),
                DeclarationKind::EnumerationLiteral,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(alias_name),
                DeclarationKind::Alias,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = match declaration.kind {
                    DeclarationKind::EnumerationLiteral => MembershipKind::Feature,
                    DeclarationKind::Alias => MembershipKind::Alias,
                    _ => MembershipKind::Owning,
                };
                // Interior/final segments of a multi-segment qualified name are looked up through
                // the exported-names index (`build_effective_import_indexes`'s sibling,
                // `build_direct_name_index(.., Some(&memberships))`), which only admits publicly
                // visible members -- the same rule every other owned-member kind is subject to, not
                // an enum-specific one. Publicize the literal explicitly here to exercise that
                // generic path rather than asserting anything enum-specific about visibility
                // defaults.
                let visibility = if declaration.kind == DeclarationKind::EnumerationLiteral {
                    Visibility::Public
                } else {
                    Visibility::Default
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility,
                    role: None,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            alias,
            ReferenceKind::AliasBinding,
            qualified_path,
            false,
        )];
        let _symbols = symbols.freeze();
        let fixture = ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        };

        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(approved))
        );
    }

    /// Builds a `Demo { port def Base; port def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference is authored with `conjugated` set per `typing_conjugated`,
    /// exercising `port def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `part def`.
    fn port_def_specialization_fixture(typing_conjugated: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::PortDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::PortDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags {
                conjugated: typing_conjugated,
                ..RelationshipFlags::default()
            },
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn port_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = port_def_specialization_fixture(false);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { occurrence def Base; occurrence def Derived :> Base; }`-shaped fixture,
    /// exercising `occurrence def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `port def`/`state def`.
    fn occurrence_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::OccurrenceDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::OccurrenceDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn occurrence_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = occurrence_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { analysis def Base; analysis def Derived :> Base; }`-shaped fixture,
    /// exercising `analysis def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `occurrence def`.
    fn analysis_case_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::AnalysisCaseDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::AnalysisCaseDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn analysis_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = analysis_case_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { Base; Derived :> Base; }`-shaped fixture for the given case-family
    /// `DeclarationKind` (`CaseDefinition`/`VerificationCaseDefinition`/`UseCaseDefinition`),
    /// exercising its participation in the shared Subclassification/FeatureTyping lexical lookup
    /// fixed point (`DeclarationDomain::Type`) exactly like `analysis def`/`occurrence def`.
    fn case_family_def_specialization_fixture(kind: DeclarationKind) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(DocumentIdx(0), Some(demo), Some(base_name), kind),
            declaration(DocumentIdx(0), Some(demo), Some(derived_name), kind),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = case_family_def_specialization_fixture(DeclarationKind::CaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn verification_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture =
            case_family_def_specialization_fixture(DeclarationKind::VerificationCaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn use_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = case_family_def_specialization_fixture(DeclarationKind::UseCaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn conjugated_port_typing_reference_resolves_to_the_correct_target_and_carries_the_flag() {
        // `port source : ~InputPort;` -- the conjugated `~` polarity must be visible as an
        // explicit fact on the authored reference, distinct from the (unconjugated) target
        // declaration itself, which the resolved outcome still names correctly.
        let fixture = port_def_specialization_fixture(true);
        assert!(fixture.references[0].flags.conjugated);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn non_conjugated_port_typing_reference_does_not_carry_the_conjugated_flag() {
        // Regression guard: an ordinary (non-`~`) port typing/specialization reference must not
        // spuriously pick up the conjugated flag.
        let fixture = port_def_specialization_fixture(false);
        assert!(!fixture.references[0].flags.conjugated);
    }

    /// Builds a `Demo { item def Base; item def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `item def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `part def`/`port def`.
    fn item_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ItemDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ItemDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn item_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = item_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { class Base; class Derived :> Base; }`-shaped fixture: `Derived`'s `:>`
    /// specialization reference exercises KerML `class def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `item def`.
    fn class_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ClassDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ClassDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn class_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = class_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { action def Base; action def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `action def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `item def`/`part def`/`port def`.
    fn action_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ActionDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ActionDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn action_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = action_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { state def Base; state def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `state def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `action def`/`item def`/`part def`/`port def`.
    fn state_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::StateDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::StateDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn state_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = state_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { metadata def Base; metadata def Derived :> Base; }`-shaped fixture:
    /// `Derived`'s `:>` specialization reference exercises `metadata def`'s participation in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`) exactly like `item def`/`action def`/`part def`/`port def`.
    fn metadata_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::MetadataDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::MetadataDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn metadata_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = metadata_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { metadata def Safety; part seatBelt {@Safety;} }`-shaped fixture: the part
    /// usage's `@Safety` metadata annotation exercises `ReferenceKind::MetadataAnnotation`'s
    /// participation in the shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`), sourced directly at the part usage's own declaration (not an
    /// anonymous nested feature).
    fn metadata_annotation_fixture(target_name: &str) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let safety_name = symbols.intern("Safety").unwrap();
        let seat_belt_name = symbols.intern("seatBelt").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let target_symbol = symbols.intern(target_name).unwrap();
        let target_path = paths.push(&[target_symbol], false).unwrap();

        let demo = DeclarationId(0);
        let seat_belt = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(safety_name),
                DeclarationKind::MetadataDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(seat_belt_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: seat_belt,
            kind: ReferenceKind::MetadataAnnotation,
            path: target_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn metadata_annotation_on_part_usage_resolves_to_metadata_def() {
        let fixture = metadata_annotation_fixture("Safety");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn metadata_annotation_with_unresolvable_target_stays_unresolved() {
        let fixture = metadata_annotation_fixture("NoSuchMetadata");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    /// Builds a `Demo { connection def Base; connection def Derived :> Base; }`-shaped fixture:
    /// `Derived`'s `:>` specialization reference exercises `connection def`'s participation in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`) exactly like `item def`/`action def`/`part def`/`port def`.
    fn connection_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ConnectionDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ConnectionDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn connection_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = connection_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    fn interface_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::InterfaceDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::InterfaceDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn interface_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = interface_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { part d1; connection bus connect d1 to d1; }`-shaped fixture: `bus`'s
    /// `ConnectorEnd` reference exercises the `DeclarationDomain::Any` resolution slot
    /// (`connector_end_slots`) exactly like `AliasBinding` -- a connector end can reference any
    /// feature, not just a Type, so it must not join the Subclassification/FeatureTyping `Type`
    /// domain passes.
    fn connector_end_reference_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let d1_name = symbols.intern("d1").unwrap();
        let bus_name = symbols.intern("bus").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let d1_path = paths.push(&[d1_name], false).unwrap();

        let demo = DeclarationId(0);
        let bus = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(d1_name),
                DeclarationKind::PartUsage,
            ),
            declaration(
                DocumentIdx(0),
                Some(demo),
                Some(bus_name),
                DeclarationKind::ConnectionUsage,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: bus,
            kind: ReferenceKind::ConnectorEnd,
            path: d1_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn connector_end_reference_resolves_to_its_target() {
        let fixture = connector_end_reference_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn namespace_import_populates_index_used_by_feature_typing() {
        let fixture = cross_file_fixture(false);
        let (direct_names, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(0)))
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            direct_names.candidates(Some(DeclarationId(0)), NameId(2)),
            &[DeclarationId(1)]
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(2)), NameId(2)),
            &[DeclarationId(1)]
        );
        assert_eq!(fixture.paths.paths.len(), 2);
        assert_eq!(
            fixture.paths.get(SymbolPathId(0)),
            Some((&[NameId(0)][..], false))
        );
        assert_eq!(resolution.outcomes.len(), fixture.references.len());
    }

    #[test]
    fn default_visibility_is_settled_once_from_membership_context() {
        let declarations = [
            declaration(
                DocumentIdx(0),
                None,
                Some(NameId(0)),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(0)),
                Some(NameId(1)),
                DeclarationKind::Namespace,
            ),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(1)),
                Some(NameId(2)),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(DeclarationId(0)),
                None,
                DeclarationKind::Import,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let index = MembershipIndex::build(&declarations, &memberships).unwrap();

        assert!(index.is_public(DeclarationId(0)));
        assert!(index.is_public(DeclarationId(1)));
        assert!(!index.is_public(DeclarationId(2)));
        assert!(!index.is_public(DeclarationId(3)));
    }

    #[test]
    fn namespace_import_excludes_explicitly_private_members() {
        let mut fixture = cross_file_fixture(false);
        fixture.memberships[1].visibility = Visibility::Private;
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);

        assert!(effective_imports
            .candidates(Some(DeclarationId(2)), NameId(2))
            .is_empty());
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    #[test]
    fn duplicate_imported_type_is_canonically_ambiguous() {
        let fixture = cross_file_fixture(true);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        let Some(ResolutionStatus::Ambiguous(range)) = resolution.outcome(AuthoredReferenceId(1))
        else {
            panic!("feature typing must retain ambiguity");
        };
        assert_eq!(
            resolution.ambiguous_candidates(range),
            &[DeclarationId(1), DeclarationId(5)]
        );
    }

    #[test]
    fn transitive_namespace_imports_converge_without_reference_scans() {
        let fixture = transitive_import_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(resolution.work.passes, 4);
        assert_eq!(resolution.work.import_evaluations, 12);
        assert_eq!(resolution.work.downstream_evaluations, 1);
        assert_eq!(resolution.work.direct_index_entries, 6);
        assert_eq!(resolution.work.effective_index_entries, 3);
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(6)), NameId(4)),
            &[DeclarationId(1)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(3)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn an_import_target_can_become_visible_through_an_earlier_import() {
        let fixture = imported_target_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(3)), NameId(2)),
            &[DeclarationId(2)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn cyclic_namespace_imports_reach_a_finite_canonical_closure() {
        let fixture = cyclic_import_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(resolution.work.passes, 3);
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(0)), NameId(3)),
            &[DeclarationId(5)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(5)))
        );
    }

    #[test]
    fn later_qualified_segments_use_the_effective_import_index() {
        let fixture = qualified_import_target_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(5)), NameId(2)),
            &[DeclarationId(2)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn exhausted_bound_is_a_typed_non_converged_publication_state() {
        let fixture = cross_file_fixture(false);
        let (_, _, _memberships, resolution) = resolve_dense_with_limit(
            &fixture.declarations,
            None,
            &fixture.memberships,
            &fixture.paths,
            &fixture.references,
            1,
            None,
        )
        .unwrap();
        assert_eq!(resolution.solver_status, SolverStatus::NonConverged);
        assert_eq!(resolution.work.passes, 1);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::NonConverged)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::NonConverged)
        );
    }

    #[test]
    fn missing_and_filtered_references_remain_explicit() {
        let mut fixture = cross_file_fixture(false);
        fixture.references[0].kind = ReferenceKind::FilterImport;
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Unsupported)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    /// Builds `package Diamond { part def Base { part def Member; } part def Left :> Base;
    /// part def Right :> Base; part def Diamond :> Left, Right { part <feature_name> : <typed>; } }`.
    /// `feature_name`/`typed` let the ambiguous-diamond test override the leaf feature and its
    /// authored typing target while sharing the rest of the diamond shape.
    fn diamond_fixture(feature_name: &str, typed: &str) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let diamond_pkg = symbols.intern("Diamond").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let member_name = symbols.intern("Member").unwrap();
        let left_name = symbols.intern("Left").unwrap();
        let right_name = symbols.intern("Right").unwrap();
        let diamond_name = symbols.intern("Diamond").unwrap();
        let feature = symbols.intern(feature_name).unwrap();
        let typed_name = symbols.intern(typed).unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();
        let left_path = paths.push(&[left_name], false).unwrap();
        let right_path = paths.push(&[right_name], false).unwrap();
        let typed_path = paths.push(&[typed_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let left = DeclarationId(3);
        let right = DeclarationId(4);
        let diamond = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(diamond_pkg),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(base),
                Some(member_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(left_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(right_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(diamond_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(diamond),
                Some(feature),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(left, ReferenceKind::Subclassification, base_path, false),
            reference(right, ReferenceKind::Subclassification, base_path, false),
            reference(diamond, ReferenceKind::Subclassification, left_path, false),
            reference(diamond, ReferenceKind::Subclassification, right_path, false),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                typed_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn diamond_inherited_member_lookup_dedups_to_a_single_target() {
        let fixture = diamond_fixture("p", "Member");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // Member is owned only by Base (id 2), reached via both Left -> Base and Right -> Base;
        // the diamond must dedup to exactly one Resolved outcome rather than an Ambiguous one.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(4)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn single_ancestor_inherited_lookup_resolves_through_one_specialization_hop() {
        // A minimal non-diamond case: Diamond specializes only Left (drop the Right edge by
        // reusing the diamond fixture's Left -> Base -> Member chain through a direct
        // single-parent shape) still exercises the same inherited-lookup path.
        let mut fixture = diamond_fixture("p", "Member");
        // Remove the `Diamond :> Right` edge (reference index 3) and the `Right :> Base` edge
        // (reference index 1) so only one specialization hop feeds the closure.
        let mut references = fixture.references.into_vec();
        references.remove(3);
        references.remove(1);
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    /// `package P { part def Left { part def Special; } part def Right { part def Special; }
    /// part def Diamond :> Left, Right { part q : Special; } }`. Left and Right each directly own
    /// their own distinct `Special` member (no `Base`), so the diamond conflict is genuine: two
    /// different ancestors reach two different same-named targets, not one target through two
    /// paths.
    fn diamond_conflict_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let left_name = symbols.intern("Left").unwrap();
        let right_name = symbols.intern("Right").unwrap();
        let diamond_name = symbols.intern("Diamond").unwrap();
        let special_name = symbols.intern("Special").unwrap();
        let q_name = symbols.intern("q").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let left_path = paths.push(&[left_name], false).unwrap();
        let right_path = paths.push(&[right_name], false).unwrap();
        let special_path = paths.push(&[special_name], false).unwrap();

        let package = DeclarationId(0);
        let left = DeclarationId(1);
        let right = DeclarationId(3);
        let diamond = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(left_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(left),
                Some(special_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(right_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(right),
                Some(special_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(diamond_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(diamond),
                Some(q_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(diamond, ReferenceKind::Subclassification, left_path, false),
            reference(diamond, ReferenceKind::Subclassification, right_path, false),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                special_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn diamond_with_conflicting_ancestor_members_publishes_an_explicit_ambiguous_outcome() {
        let fixture = diamond_conflict_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        match resolution.outcome(AuthoredReferenceId(2)) {
            Some(ResolutionStatus::Ambiguous(range)) => {
                let mut candidates = resolution.ambiguous_candidates(range).to_vec();
                candidates.sort_by_key(|id| id.0);
                assert_eq!(candidates, vec![DeclarationId(2), DeclarationId(4)]);
            }
            other => panic!("expected an explicit ambiguous outcome, got {other:?}"),
        }
    }

    fn cyclic_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let f_name = symbols.intern("f").unwrap();
        let x_name = symbols.intern("X").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();
        let x_path = paths.push(&[x_name], false).unwrap();

        let package = DeclarationId(0);
        let a = DeclarationId(1);
        let b = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(a_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(b_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(a),
                Some(f_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(a, ReferenceKind::Subclassification, b_path, false),
            reference(b, ReferenceKind::Subclassification, a_path, false),
            reference(
                DeclarationId(3),
                ReferenceKind::FeatureTyping,
                x_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn cyclic_specialization_yields_a_typed_non_converged_typing_outcome_not_a_loop() {
        let fixture = cyclic_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        // The import/typing fixed point above this family still converges; only the
        // ancestor-closure-dependent FeatureTyping outcome for the cyclically-specialized owner
        // is explicitly NonConverged, rather than the solver looping forever or silently guessing
        // an inherited candidate through the self-referential closure.
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::NonConverged)
        );
    }

    /// `package P { part def Device; alias DeviceAlias for Device; part device : DeviceAlias; }`
    /// — mirrors `tests/snapshots/resolution/alias_target_binding.md`.
    fn alias_binding_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let device_name = symbols.intern("Device").unwrap();
        let alias_name = symbols.intern("DeviceAlias").unwrap();
        let device_usage_name = symbols.intern("device").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let device_path = paths.push(&[device_name], false).unwrap();
        let alias_path = paths.push(&[alias_name], false).unwrap();

        let package = DeclarationId(0);
        let alias = DeclarationId(2);
        let device_usage = DeclarationId(3);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(device_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(alias_name),
                DeclarationKind::Alias,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(device_usage_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(alias, ReferenceKind::AliasBinding, device_path, false),
            reference(
                device_usage,
                ReferenceKind::FeatureTyping,
                alias_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn alias_binding_resolves_through_the_shared_lexical_lookup_fixed_point() {
        let fixture = alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // DeviceAlias's own authored `AliasBinding` reference resolves to Device (id 1), using
        // the same fixed point as every other authored reference kind rather than a separate
        // ad hoc path.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn typing_through_an_alias_resolves_transitively_to_the_ultimate_target() {
        let fixture = alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // `device : DeviceAlias`'s own FeatureTyping outcome targets the alias declaration (id 2)
        // itself -- the alias's own authored fact is never weakened or bypassed.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
        // Downstream typing is nonetheless transparent: an implied FeatureTyping fact chases the
        // alias chain to publish device -> Device directly, with implied provenance.
        assert_eq!(
            resolution.implied_relationships.as_ref(),
            &[ImpliedRelationship {
                kind: ReferenceKind::FeatureTyping,
                source: DeclarationId(3),
                target: DeclarationId(1),
            }],
        );
    }

    /// `package P { alias A for B; alias B for A; }` — a two-hop alias cycle, mirroring the
    /// specialization-cycle shape of `cyclic_specialization_fixture` above.
    fn cyclic_alias_binding_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();

        let package = DeclarationId(0);
        let a = DeclarationId(1);
        let b = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentIdx(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(a_name),
                DeclarationKind::Alias,
            ),
            declaration(
                DocumentIdx(0),
                Some(package),
                Some(b_name),
                DeclarationKind::Alias,
            ),
        ];
        let references = vec![
            reference(a, ReferenceKind::AliasBinding, b_path, false),
            reference(b, ReferenceKind::AliasBinding, a_path, false),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn alias_cycle_yields_a_typed_non_converged_outcome_not_a_hang() {
        // Bounded by `detect_cyclic_alias_bindings`'s `declarations.len() + 1` hop limit: this
        // test would time out (rather than merely fail an assertion) if alias cycle detection
        // ever degenerated into an unbounded chase.
        let fixture = cyclic_alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::NonConverged)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::NonConverged)
        );
        assert!(resolution.implied_relationships.is_empty());
    }

    /// `package A { part def T; } package C { import A::*; part T; part p : T; }` — mirrors
    /// `tests/snapshots/resolution/lexical_inner_shadow.md`. `nested` controls whether the
    /// FeatureTyping reference lives directly on `C::p` (false) or one namespace level deeper, on
    /// a feature owned by an intermediate `Inner` namespace inside `C` (true), so the same local
    /// binding is reached by walking one extra step of the enclosing-namespace chain.
    fn local_shadow_fixture(nested: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let t_name = symbols.intern("T").unwrap();
        let c_name = symbols.intern("C").unwrap();
        let p_name = symbols.intern("p").unwrap();
        let inner_name = symbols.intern("Inner").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let t_path = paths.push(&[t_name], false).unwrap();

        let a = DeclarationId(0);
        let c = DeclarationId(2);
        let mut declarations = vec![
            declaration(DocumentIdx(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentIdx(0),
                Some(a),
                Some(t_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentIdx(0), None, Some(c_name), DeclarationKind::Package),
            declaration(DocumentIdx(0), Some(c), None, DeclarationKind::Import),
            declaration(
                DocumentIdx(0),
                Some(c),
                Some(t_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let p_owner = if nested {
            let inner = DeclarationId(u32::try_from(declarations.len()).unwrap());
            declarations.push(declaration(
                DocumentIdx(0),
                Some(c),
                Some(inner_name),
                DeclarationKind::Namespace,
            ));
            inner
        } else {
            c
        };
        declarations.push(declaration(
            DocumentIdx(0),
            Some(p_owner),
            Some(p_name),
            DeclarationKind::PartUsage,
        ));
        let p = DeclarationId(u32::try_from(declarations.len() - 1).unwrap());
        let references = vec![
            reference(
                DeclarationId(3),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(p, ReferenceKind::FeatureTyping, t_path, false),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn local_feature_shadows_an_incompatible_imported_type_of_the_same_name() {
        // C::T (a PartUsage feature) is domain-incompatible as a FeatureTyping target, but it is
        // still owned directly by C, the reference's enclosing namespace, so per
        // it must shadow the imported, domain-compatible
        // A::T rather than being silently discarded in favor of the import or left Unresolved.
        let fixture = local_shadow_fixture(false);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(4)))
        );
    }

    #[test]
    fn without_a_local_binding_lookup_still_falls_through_to_the_import() {
        // Regression guard: removing C::T (and its membership record) must not disturb the
        // fallback to the imported A::T once no local/inherited candidate exists at any tier.
        let mut fixture = local_shadow_fixture(false);
        let mut declarations = fixture.declarations.into_vec();
        declarations.remove(4);
        fixture.declarations = declarations.into_boxed_slice();
        // Rebuild memberships from scratch rather than splicing: `MembershipRecord::member` is a
        // `DeclarationId` into the just-mutated declarations array, so it must be recomputed
        // against the post-removal indices, not the pre-removal ones.
        fixture.memberships = memberships_for(&fixture.declarations, &[]);
        // The FeatureTyping reference source shifts down by one index after the removal.
        let mut references = fixture.references.into_vec();
        references[1].source = DeclarationId(4);
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn intermediate_namespace_local_binding_shadows_the_outer_import() {
        // p lives inside C::Inner, one level below C itself. The local C::T binding is neither
        // owned by nor inherited into Inner directly, so the walk must climb the enclosing-scope
        // chain to C, find C::T there, and shadow A::T at that outer tier before ever consulting
        // imports.
        let fixture = local_shadow_fixture(true);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(4)))
        );
    }

    #[test]
    fn candidate_ranges_are_canonical_regardless_of_input_order() {
        let index = NameIndex::build(vec![
            (
                NameKey {
                    owner: None,
                    name: NameId(0),
                },
                DeclarationId(2),
            ),
            (
                NameKey {
                    owner: None,
                    name: NameId(0),
                },
                DeclarationId(1),
            ),
        ])
        .unwrap();
        assert_eq!(
            index.candidates(None, NameId(0)),
            &[DeclarationId(1), DeclarationId(2)]
        );
    }

    #[test]
    fn packed_name_entry_order_matches_the_canonical_tuple_order() {
        let entries = vec![
            (
                NameKey {
                    owner: Some(DeclarationId(u32::MAX)),
                    name: NameId(u32::MAX),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: Some(DeclarationId(0)),
                    name: NameId(u32::MAX),
                },
                DeclarationId(0),
            ),
            (
                NameKey {
                    owner: None,
                    name: NameId(u32::MAX),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: Some(DeclarationId(0)),
                    name: NameId(0),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: None,
                    name: NameId(0),
                },
                DeclarationId(0),
            ),
        ];
        let mut canonical = entries.clone();
        canonical.sort_unstable();
        let mut packed = entries;
        packed.sort_unstable_by_key(name_entry_sort_key);
        assert_eq!(packed, canonical);
    }
}
