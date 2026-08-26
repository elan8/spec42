//! Phase 4: specialization anchors and implied-relationship synthesis.

use crate::lower::facts::AuthoredReference;
use crate::lower::facts::Declaration;
use crate::lower::facts::MembershipRecord;
use crate::lower::facts::PortionKind;
use crate::lower::storage::SemanticModelStorage;
use crate::model::element_kind;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::namespace_query::NamespaceDerivedElementCollection;
use crate::redefinition_query::RedefinitionCheckKind;
use crate::requirement_query::RequirementDerivedFactCollection;
use crate::resolve::build_ancestor_closures;
use crate::resolve::effective_types::EffectiveTypes;
use crate::resolve::names::NameIndex;
use crate::resolve::results::ConstructorExpressionProjection;
use crate::resolve::results::ConstructorExpressionProjectionStatus;
use crate::resolve::results::ConstructorExpressionSpecializationStatus;
use crate::resolve::results::ExpressionArgumentProjectionStatus;
use crate::resolve::results::FeatureChainExpressionProjection;
use crate::resolve::results::FeatureChainExpressionSpecializationStatus;
use crate::resolve::results::FeatureReferenceExpressionProjection;
use crate::resolve::results::FeatureReferenceExpressionSpecializationStatus;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::InvocationExpressionProjection;
use crate::resolve::results::InvocationExpressionProjectionStatus;
use crate::resolve::results::InvocationInstantiatedTypeKind;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::results::SemanticMetadataProjection;
use crate::resolve::results::SemanticMetadataProjectionStatus;
use crate::resolve::results::SolverStatus;
use crate::resolve::ResolutionReferenceFact;
use crate::specialization_query::SpecializationCheckKind;
use crate::traceability::BindingConnectorCheckKind;
use crate::type_query::TypeDerivedElementCollection;
use crate::type_query::TypeDerivedFactCollection;
use crate::type_query::TypeDerivedRelationshipCollection;
use crate::type_query::TypeFeaturingCheckKind;
use crate::ActionDerivedFactCollection;
use crate::DefinitionUsageDerivedKind;
use crate::ElementDerivedDocumentationCollection;
use crate::FeatureDerivedRelationshipCollection;
use crate::LibrarySpecializationAnchorBranch;
use crate::MembershipRole;
use crate::RequirementConstraintKind;
use source_identity::SourceRole;
use spec42_constraint_manifest::ElementDerivedOwnerKind;
use spec42_constraint_manifest::LibrarySpecializationPredicate;
use spec42_constraint_manifest::NamespaceImportDerivedElementKind;

/// One exact unconditional `specializesFromLibrary` check extracted from the pinned XMI through
/// `specifications/constraint_manifest.toml`. Conditional OCL bodies never enter this table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LibrarySpecializationRuleKey(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LibrarySpecializationAnchorKey {
    pub(crate) rule: LibrarySpecializationRuleKey,
    pub(crate) branch: LibrarySpecializationAnchorBranch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LibrarySpecializationRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) anchor: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/library_specialization_rules.rs"));

/// One exact conditional `specializesFromLibrary` check whose predicate is a closed manifest
/// contract. The resolver evaluates the predicate from its owned declaration facts; it never
/// reparses OCL or infers applicability from a rule name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ConditionalLibrarySpecializationRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) predicate: LibrarySpecializationPredicate,
    pub(crate) owner_metaclasses: &'static [&'static str],
    pub(crate) true_anchor: Option<&'static str>,
    pub(crate) anchor: &'static str,
}

include!(concat!(
    env!("OUT_DIR"),
    "/conditional_library_specialization_rules.rs"
));

/// One exact unconditional `redefinesFromLibrary` check extracted from the pinned XMI. Rules are
/// generated separately from specializations: the function name is a distinct normative contract
/// and a consumer must not reinterpret one as the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LibraryRedefinitionRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) anchor: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/library_redefinition_rules.rs"));

/// One closed exact Feature relationship-collection derivation emitted from the pinned manifest.
/// The generated table, rather than a query-side rule-name convention, decides which collection
/// exists and which source metaclass owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FeatureDerivedRelationshipRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) collection: FeatureDerivedRelationshipCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/feature_derived_relationship_rules.rs"
));

/// One closed Type relationship derivation emitted from the pinned manifest. The exact rule ID
/// determines the query collection; consumers never infer it from a display name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TypeDerivedRelationshipRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) collection: TypeDerivedRelationshipCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/type_derived_relationship_rules.rs"
));

/// One exact final element-valued Type derivation emitted from the pinned manifest. Intermediate
/// Membership relationship identities remain private; this table therefore admits only element
/// projections that canonical owner and membership facts can answer losslessly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TypeDerivedElementRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) collection: TypeDerivedElementCollection,
}

include!(concat!(env!("OUT_DIR"), "/type_derived_element_rules.rs"));

/// One exact Type derivation that cannot yet return values because its canonical fact owner is
/// intentionally absent. The generated table preserves rule identity and result category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TypeDerivedFactRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) collection: TypeDerivedFactCollection,
}

include!(concat!(env!("OUT_DIR"), "/type_derived_fact_rules.rs"));

/// A complete Definition/Usage derivation selected only by the generated manifest projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DefinitionUsageDerivedRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) kind: DefinitionUsageDerivedKind,
}

include!(concat!(
    env!("OUT_DIR"),
    "/definition_usage_derived_rules.rs"
));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ActionDerivedFactRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) collection: ActionDerivedFactCollection,
}

include!(concat!(env!("OUT_DIR"), "/action_derived_fact_rules.rs"));

/// One exact Systems::Requirements derivation emitted by the manifest. The query collection is
/// closed and rule-keyed; it is not an interpretation of arbitrary membership names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RequirementDerivedFactRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) collection: RequirementDerivedFactCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/requirement_derived_fact_rules.rs"
));

/// One exact TypeFeaturing check emitted from the manifest. The table owns the normative rule
/// identity and metaclass; the resolver consumes only canonical FeatureMembership and effective
/// TypeFeaturing facts to decide it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TypeFeaturingCheckRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) kind: TypeFeaturingCheckKind,
}

include!(concat!(env!("OUT_DIR"), "/type_featuring_check_rules.rs"));

/// One exact redefinition check body emitted from the manifest. This only identifies a normative
/// predicate; it never acts as a second relationship store or an OCL interpreter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RedefinitionCheckRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) kind: RedefinitionCheckKind,
}

include!(concat!(env!("OUT_DIR"), "/redefinition_check_rules.rs"));

/// One exact specialization predicate emitted from the manifest.  This generated table is the
/// sole rule-to-kind binding; it never contains semantic relationship values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpecializationCheckRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) kind: SpecializationCheckKind,
}

include!(concat!(env!("OUT_DIR"), "/specialization_check_rules.rs"));

/// The closed exact `Element::owner` derivation emitted from the pinned manifest.
///
/// The table is deliberately separate from relationship collections: `owner` is a derived
/// scalar over canonical declaration structure, not a synthetic relationship or a path lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ElementDerivedOwnerRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) kind: ElementDerivedOwnerKind,
}

include!(concat!(env!("OUT_DIR"), "/element_derived_owner_rules.rs"));

/// One exact Element documentation-form derivation emitted from the pinned manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ElementDerivedDocumentationRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) collection: ElementDerivedDocumentationCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/element_derived_documentation_rules.rs"
));

/// One exact Namespace element-valued derivation emitted from the pinned manifest. The table
/// selects which direct canonical structural projection is available; query callers do not map
/// rule names to declaration filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NamespaceDerivedElementRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) collection: NamespaceDerivedElementCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/namespace_derived_element_rules.rs"
));

/// The closed exact `NamespaceImport::importedElement` projection emitted from the pinned
/// manifest. It is deliberately separate from Namespace collections because its source is one
/// anonymous import declaration and its target retains the canonical reference outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NamespaceImportDerivedElementRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) kind: NamespaceImportDerivedElementKind,
}

include!(concat!(
    env!("OUT_DIR"),
    "/namespace_import_derived_element_rules.rs"
));

/// One exact BindingConnector validation body emitted from the pinned manifest.
///
/// The generated contract proves that a caller-selected rule kind has a single normative rule
/// and metaclass. The binding index remains the sole owner of connector facts and validation
/// prerequisites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BindingConnectorCheckRule {
    pub(crate) rule_id: &'static str,
    pub(crate) metaclass: &'static str,
    pub(crate) kind: BindingConnectorCheckKind,
}

include!(concat!(
    env!("OUT_DIR"),
    "/binding_connector_check_rules.rs"
));

pub(crate) fn feature_derived_relationship_rule(
    collection: FeatureDerivedRelationshipCollection,
) -> Option<&'static FeatureDerivedRelationshipRule> {
    GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

pub(crate) fn namespace_derived_element_rule(
    collection: NamespaceDerivedElementCollection,
) -> Option<&'static NamespaceDerivedElementRule> {
    GENERATED_NAMESPACE_DERIVED_ELEMENT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

pub(crate) fn namespace_import_derived_element_rule(
) -> Option<&'static NamespaceImportDerivedElementRule> {
    match GENERATED_NAMESPACE_IMPORT_DERIVED_ELEMENT_RULES {
        [rule] => Some(rule),
        _ => None,
    }
}

pub(crate) fn binding_connector_check_rule(
    kind: BindingConnectorCheckKind,
) -> Option<&'static BindingConnectorCheckRule> {
    GENERATED_BINDING_CONNECTOR_CHECK_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

pub(crate) fn feature_derived_relationship_kinds(
    collection: FeatureDerivedRelationshipCollection,
) -> &'static [ReferenceKind] {
    match collection {
        FeatureDerivedRelationshipCollection::OwnedFeatureChaining => {
            &[ReferenceKind::FeatureChaining]
        }
        FeatureDerivedRelationshipCollection::OwnedRedefinition => &[ReferenceKind::Redefinition],
        // KerML `Redefinition` is a subtype of `Subsetting`, so its owned relationship belongs in
        // `ownedSubsetting` as well. The storage preserves its more specific reference kind.
        FeatureDerivedRelationshipCollection::OwnedSubsetting => {
            &[ReferenceKind::Subsetting, ReferenceKind::Redefinition]
        }
        FeatureDerivedRelationshipCollection::OwnedTyping => &[ReferenceKind::FeatureTyping],
        FeatureDerivedRelationshipCollection::OwnedTypeFeaturing => &[ReferenceKind::TypeFeaturing],
    }
}

pub(crate) fn type_derived_relationship_rule(
    collection: TypeDerivedRelationshipCollection,
) -> Option<&'static TypeDerivedRelationshipRule> {
    GENERATED_TYPE_DERIVED_RELATIONSHIP_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

pub(crate) fn type_derived_element_rule(
    collection: TypeDerivedElementCollection,
) -> Option<&'static TypeDerivedElementRule> {
    GENERATED_TYPE_DERIVED_ELEMENT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

pub(crate) fn type_derived_fact_rule(
    collection: TypeDerivedFactCollection,
) -> Option<&'static TypeDerivedFactRule> {
    GENERATED_TYPE_DERIVED_FACT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

pub(crate) fn definition_usage_derived_rule(
    kind: DefinitionUsageDerivedKind,
) -> Option<&'static DefinitionUsageDerivedRule> {
    GENERATED_DEFINITION_USAGE_DERIVED_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

pub(crate) fn action_derived_fact_rule(
    collection: ActionDerivedFactCollection,
) -> Option<&'static ActionDerivedFactRule> {
    GENERATED_ACTION_DERIVED_FACT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

pub(crate) fn requirement_derived_fact_rule(
    collection: RequirementDerivedFactCollection,
) -> Option<&'static RequirementDerivedFactRule> {
    GENERATED_REQUIREMENT_DERIVED_FACT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

pub(crate) fn type_featuring_check_rule(
    kind: TypeFeaturingCheckKind,
) -> Option<&'static TypeFeaturingCheckRule> {
    GENERATED_TYPE_FEATURING_CHECK_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

pub(crate) fn redefinition_check_rule(
    kind: RedefinitionCheckKind,
) -> Option<&'static RedefinitionCheckRule> {
    GENERATED_REDEFINITION_CHECK_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

pub(crate) fn specialization_check_rule(
    kind: SpecializationCheckKind,
) -> Option<&'static SpecializationCheckRule> {
    GENERATED_SPECIALIZATION_CHECK_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

pub(crate) fn element_derived_owner_rule() -> Option<&'static ElementDerivedOwnerRule> {
    GENERATED_ELEMENT_DERIVED_OWNER_RULES.first()
}

pub(crate) fn element_derived_documentation_rule(
    collection: ElementDerivedDocumentationCollection,
) -> Option<&'static ElementDerivedDocumentationRule> {
    GENERATED_ELEMENT_DERIVED_DOCUMENTATION_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

pub(crate) fn type_derived_relationship_kinds(
    collection: TypeDerivedRelationshipCollection,
) -> &'static [ReferenceKind] {
    match collection {
        TypeDerivedRelationshipCollection::OwnedSpecialization => &[
            ReferenceKind::Subclassification,
            ReferenceKind::Subsetting,
            ReferenceKind::Redefinition,
            ReferenceKind::FeatureTyping,
        ],
        TypeDerivedRelationshipCollection::OwnedUnioning
        | TypeDerivedRelationshipCollection::UnioningType => &[ReferenceKind::Unioning],
        TypeDerivedRelationshipCollection::OwnedIntersecting
        | TypeDerivedRelationshipCollection::IntersectingType => &[ReferenceKind::Intersecting],
        TypeDerivedRelationshipCollection::OwnedDifferencing
        | TypeDerivedRelationshipCollection::DifferencingType => &[ReferenceKind::Differencing],
        TypeDerivedRelationshipCollection::OwnedDisjoining => &[ReferenceKind::Disjoining],
    }
}

pub(crate) fn library_specialization_rules(
    metaclass: &str,
) -> impl Iterator<Item = &'static LibrarySpecializationRule> + '_ {
    GENERATED_LIBRARY_SPECIALIZATION_RULES
        .iter()
        .filter(move |rule| rule.metaclass == metaclass)
}

pub(crate) fn conditional_library_specialization_rules(
    metaclass: &str,
) -> impl Iterator<Item = &'static ConditionalLibrarySpecializationRule> + '_ {
    GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
        .iter()
        .filter(move |rule| rule.metaclass == metaclass)
}

pub(crate) fn library_redefinition_rules(
    metaclass: &str,
) -> impl Iterator<Item = &'static LibraryRedefinitionRule> + '_ {
    GENERATED_LIBRARY_REDEFINITION_RULES
        .iter()
        .filter(move |rule| rule.metaclass == metaclass)
}

/// The root standard-library packages every generated library rule anchors into, deduplicated
/// and sorted. The library-closure authority seeds these so that a publication always admits
/// the documents its implied specializations and redefinitions resolve against.
pub(crate) fn library_anchor_packages() -> Vec<&'static str> {
    let mut packages = GENERATED_LIBRARY_SPECIALIZATION_RULES
        .iter()
        .map(|rule| rule.anchor)
        .chain(
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .flat_map(|rule| [Some(rule.anchor), rule.true_anchor].into_iter().flatten()),
        )
        .chain(
            GENERATED_LIBRARY_REDEFINITION_RULES
                .iter()
                .map(|rule| rule.anchor),
        )
        .filter_map(|anchor| anchor.split("::").next())
        .collect::<Vec<_>>();
    packages.sort_unstable();
    packages.dedup();
    packages
}

/// Maps an exact XMI source metaclass to the parser's owned declaration projection.
///
/// `PayloadFeature` is currently not a lowered declaration kind. Keeping that absence explicit
/// is intentional: a same-named feature, a generic `Feature`, or an enclosing declaration must
/// never be substituted as the source of a normative implied redefinition. Extend this adapter
/// only when lowering publishes an exact `PayloadFeature` projection.
pub(crate) fn lowered_redefinition_source_kind(metaclass: &str) -> Option<DeclarationKind> {
    match metaclass {
        "PayloadFeature" => None,
        _ => None,
    }
}

#[cfg(test)]
pub(crate) fn generated_library_specialization_rule_count() -> usize {
    GENERATED_LIBRARY_SPECIALIZATION_RULES.len()
}

#[cfg(test)]
pub(crate) fn generated_conditional_library_specialization_rule_count() -> usize {
    GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES.len()
}

#[cfg(test)]
pub(crate) fn generated_library_redefinition_rule_count() -> usize {
    GENERATED_LIBRARY_REDEFINITION_RULES.len()
}

/// The canonical standard-library target named by one `specializesFromLibrary` check.
///
/// Anchor outcomes are an owned semantic fact. A workspace declaration with matching spelling can
/// never substitute for a language anchor, and several standard-library candidates remain an
/// explicit ambiguity rather than an accidental traversal choice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LibrarySpecializationAnchor {
    Resolved(DeclarationId),
    Missing,
    Ambiguous(Box<[DeclarationId]>),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LibrarySpecializationAnchorFacts {
    /// Every generated rule branch receives exactly one outcome at the semantic publication
    /// barrier. The typed `(rule, branch)` identity, rather than rendered anchor or metaclass,
    /// owns identity: an anchor can be deliberately shared by independent normative rules.
    pub(crate) by_rule:
        std::collections::BTreeMap<LibrarySpecializationAnchorKey, LibrarySpecializationAnchor>,
}

impl LibrarySpecializationAnchorFacts {
    /// Whether any generated specialization rule has a usable standard-library anchor.
    ///
    /// Workspaces compiled without the standard library have no possible provisional library
    /// edges. Detect that once instead of scanning every declaration against every generated
    /// rule; this is only a phase guard, not a cache or an alternate semantic path.
    pub(crate) fn has_resolved_anchor(&self) -> bool {
        self.by_rule
            .values()
            .any(|outcome| matches!(outcome, LibrarySpecializationAnchor::Resolved(_)))
    }

    /// Compatibility projection for legacy single-anchor rules and the `else` branch of exact
    /// polarity contracts.
    pub(crate) fn outcome(&self, rule_id: &str) -> Option<&LibrarySpecializationAnchor> {
        self.outcome_for(rule_id, LibrarySpecializationAnchorBranch::Default)
    }

    fn generated_outcome(&self, rule_id: &'static str) -> Option<&LibrarySpecializationAnchor> {
        self.generated_outcome_for(rule_id, LibrarySpecializationAnchorBranch::Default)
    }

    fn generated_outcome_for(
        &self,
        rule_id: &'static str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> Option<&LibrarySpecializationAnchor> {
        self.by_rule.get(&LibrarySpecializationAnchorKey {
            rule: LibrarySpecializationRuleKey(rule_id),
            branch,
        })
    }

    pub(crate) fn outcome_for(
        &self,
        rule_id: &str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> Option<&LibrarySpecializationAnchor> {
        self.by_rule.iter().find_map(|(key, outcome)| {
            (key.rule.0 == rule_id && key.branch == branch).then_some(outcome)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LibrarySpecializationDiagnosticKey {
    pub(crate) anchor: &'static str,
    pub(crate) document: DocumentIdx,
}

/// Synthesizes implied same-name inherited-member redefinition facts.
///
/// Scope: a feature member `f` directly owned by a type `Child`, where `Child` has a resolved
/// `Subclassification` reference to `Parent`, and `Parent` directly (not transitively) owns
/// exactly one feature member also named `f`. This deliberately does not chase multi-level or
/// diamond ancestry: if the immediate parent has zero or more than one directly owned same-name
/// feature candidate, no implied fact is synthesized for that pair rather than guessing. A member
/// that already carries an explicit `:>>` redefinition to any target is left to that authored fact
/// and is never also given an implied one.
pub(crate) fn synthesize_implied_redefinitions<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    memberships: &[MembershipRecord],
    references: &[R],
    direct_names: &NameIndex,
    outcomes: &[ResolutionStatus],
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let mut membership_kind: Vec<Option<MembershipKind>> = vec![None; declarations.len()];
    for membership in memberships {
        if let Some(slot) = membership_kind.get_mut(membership.member.index()) {
            *slot = Some(membership.kind);
        }
    }
    let is_feature = |id: DeclarationId| {
        membership_kind.get(id.index()).copied().flatten() == Some(MembershipKind::Feature)
    };

    let mut explicitly_redefines: std::collections::BTreeSet<DeclarationId> = Default::default();
    for reference in references {
        if reference.kind() == ReferenceKind::Redefinition {
            explicitly_redefines.insert(reference.source());
        }
    }

    let mut implied = Vec::new();
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() != ReferenceKind::Subclassification {
            continue;
        }
        let ResolutionStatus::Resolved(parent) = outcomes[index] else {
            continue;
        };
        let child = reference.source();
        for (name, member_candidates) in direct_names.entries_for_owner(Some(child)) {
            for &member in member_candidates {
                if !is_feature(member) || explicitly_redefines.contains(&member) {
                    continue;
                }
                let parent_candidates = direct_names.candidates(Some(parent), name);
                let mut matches = parent_candidates.iter().copied().filter(|c| is_feature(*c));
                let Some(single_match) = matches.next() else {
                    continue;
                };
                if matches.next().is_some() {
                    // Ambiguous immediate-parent candidates: leave unresolved rather than guess.
                    continue;
                }
                implied.push(ImpliedRelationship {
                    kind: ReferenceKind::Redefinition,
                    source: member,
                    target: single_match,
                });
            }
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Detects `alias` targets that eventually cycle back to their own starting alias declaration
/// (`alias A for B; alias B for A;`). Alias bindings form a functional graph -- each alias
/// declaration has at most one outgoing edge, its own resolved `AliasBinding` target -- so a walk
/// from any alias source bounded by `declarations.len() + 1` hops either terminates at a non-alias
/// target, runs off an unresolved edge, or revisits its own start, which is the only case flagged
/// here. Only alias declarations that themselves author a resolved `AliasBinding` reference are
/// candidates, so the returned set is always a subset of alias declarations.
pub(crate) fn detect_cyclic_alias_bindings<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    references: &[R],
    outcomes: &[ResolutionStatus],
) -> Result<std::collections::BTreeSet<DeclarationId>, ResolutionError> {
    let mut direct_target: Vec<Option<DeclarationId>> = vec![None; declarations.len()];
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() != ReferenceKind::AliasBinding {
            continue;
        }
        if let ResolutionStatus::Resolved(target) = outcomes[index] {
            if let Some(slot) = direct_target.get_mut(reference.source().index()) {
                *slot = Some(target);
            }
        }
    }
    let pass_limit = declarations
        .len()
        .checked_add(1)
        .ok_or(ResolutionError::Capacity)?;
    let mut cyclic = std::collections::BTreeSet::new();
    for index in 0..declarations.len() {
        let Some(mut current) = direct_target[index] else {
            continue;
        };
        let start = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let mut steps = 0usize;
        loop {
            if current == start {
                cyclic.insert(start);
                break;
            }
            let Some(next) = direct_target.get(current.index()).copied().flatten() else {
                break;
            };
            current = next;
            steps = steps.checked_add(1).ok_or(ResolutionError::Capacity)?;
            if steps > pass_limit {
                // Defensive only: a functional graph over a bounded declaration count cannot
                // require more hops than there are declarations without having already revisited
                // `start` above.
                cyclic.insert(start);
                break;
            }
        }
    }
    Ok(cyclic)
}

/// Synthesizes implied "typing/specialization/... through an alias" relationship facts: when an
/// authored reference (for example a `FeatureTyping` on `device : DeviceAlias`) resolves to an
/// alias declaration, this follows that alias's own resolved `AliasBinding` chain -- transitively,
/// through alias-of-alias -- to the ultimate non-alias target and publishes an `implied` (per
/// provenance) relationship of the *same* reference kind
/// straight from the original source to that ultimate target. This makes aliasing "transparent"
/// for downstream typing without weakening or replacing the alias's own authored `AliasBinding`
/// fact, which remains published as its own (authored-provenance) reference/relationship. A cycle
/// in the alias chain (already reported via `detect_cyclic_alias_bindings`) or an unresolved link
/// simply yields no implied fact for that source, rather than guessing.
pub(crate) fn synthesize_implied_alias_bindings<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    references: &[R],
    outcomes: &[ResolutionStatus],
    cyclic_alias_sources: &std::collections::BTreeSet<DeclarationId>,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let mut alias_target: std::collections::BTreeMap<DeclarationId, DeclarationId> =
        Default::default();
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() != ReferenceKind::AliasBinding {
            continue;
        }
        if cyclic_alias_sources.contains(&reference.source()) {
            continue;
        }
        if let ResolutionStatus::Resolved(target) = outcomes[index] {
            alias_target.insert(reference.source(), target);
        }
    }
    let is_alias = |id: DeclarationId| {
        declarations
            .get(id.index())
            .is_some_and(|declaration| declaration.kind == DeclarationKind::Alias)
    };

    let mut implied = Vec::new();
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() == ReferenceKind::AliasBinding {
            continue;
        }
        let ResolutionStatus::Resolved(mut current) = outcomes[index] else {
            continue;
        };
        if !is_alias(current) {
            continue;
        }
        let mut visited = std::collections::BTreeSet::new();
        let mut ultimate = None;
        loop {
            if !visited.insert(current) {
                // Cyclic alias chain: leave unresolved rather than guess.
                ultimate = None;
                break;
            }
            match alias_target.get(&current) {
                Some(&next) if is_alias(next) => current = next,
                Some(&next) => {
                    ultimate = Some(next);
                    break;
                }
                None => break,
            }
        }
        if let Some(target) = ultimate {
            implied.push(ImpliedRelationship {
                kind: reference.kind(),
                source: reference.source(),
                target,
            });
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Every implied relationship this publication settles, as one value.
///
/// Phase 4 reads the frozen phase-3 product and returns a complete store; it never writes back
/// into the value the solver returned, so no reader can observe a partially synthesized set.
/// Synthesis only runs on a converged solve: an unconverged one has no settled outcome to derive
/// an implied relationship from, and guessing one would publish a fact that is not resolved.
pub(crate) fn synthesize_implied_relationships(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    anchors: &LibrarySpecializationAnchorFacts,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    if !matches!(resolution.solver_status, SolverStatus::Converged) {
        return Ok(resolution.implied_relationships.clone());
    }
    let mut implied = resolution.implied_relationships.to_vec();
    implied.extend(
        synthesize_generated_library_specializations(
            storage,
            &storage.references,
            &resolution.outcomes,
            anchors,
        )?
        .into_vec(),
    );
    implied.extend(
        synthesize_generated_library_redefinitions(storage, &storage.references, anchors)?
            .into_vec(),
    );
    implied.extend(
        synthesize_feature_membership_type_featurings(storage, &storage.references)?.into_vec(),
    );
    implied.extend(synthesize_feature_valuation_specializations(storage)?.into_vec());
    implied.sort_by_key(|relationship| {
        (
            relationship.kind,
            relationship.source.0,
            relationship.target.0,
        )
    });
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Synthesizes `checkFeatureValuationSpecialization` (KerML 8.3.3.3.4): a non-default
/// FeatureValue on an undirected Feature with no explicit specialization subsets the canonical
/// result Feature of its owned value Expression.
pub(crate) fn synthesize_feature_valuation_specializations(
    storage: &SemanticModelStorage,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let mut implied = Vec::new();
    for value in storage.feature_values.iter() {
        if !feature_valuation_specialization_applies(storage, value)? {
            continue;
        }
        implied.push(ImpliedRelationship {
            kind: ReferenceKind::Subsetting,
            source: value.declaration,
            target: value.result,
        });
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Canonical applicability predicate shared by synthesis and the exact rule query.
pub(crate) fn feature_valuation_specialization_applies(
    storage: &SemanticModelStorage,
    value: &crate::lower::facts::FeatureValueRecord,
) -> Result<bool, ResolutionError> {
    let facts = storage
        .declaration_facts(value.declaration)
        .ok_or(ResolutionError::InvalidStorage)?;
    Ok(!value.is_default
        && facts.direction.is_none()
        && !storage.references.iter().any(|reference| {
            reference.source == value.declaration
                && matches!(
                    reference.kind,
                    ReferenceKind::Subclassification
                        | ReferenceKind::FeatureTyping
                        | ReferenceKind::Subsetting
                        | ReferenceKind::Redefinition
                )
        }))
}

/// Synthesizes the FeatureTyping relationships required by
/// `checkFeatureOwnedCrossFeatureSpecialization` (KerML 8.3.3.3.4).
///
/// This runs after the first implied-relationship barrier because an owning end Feature's
/// effective types can themselves be inherited through an implied Redefinition. The `TypeIndex`
/// is the canonical owner of that closure; this derivation consumes it instead of rebuilding a
/// second effective-typing algorithm from references.
pub(crate) fn synthesize_owned_cross_feature_typings(
    storage: &SemanticModelStorage,
    types: &EffectiveTypes,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let mut implied = Vec::new();
    for (index, facts) in storage.declaration_facts.iter().enumerate() {
        let Some(projection) = facts.cross_feature_projection else {
            continue;
        };
        let owner = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        implied.extend(
            types
                .row(owner)
                .iter()
                .map(|(target, _)| ImpliedRelationship {
                    kind: ReferenceKind::FeatureTyping,
                    source: projection.owned_cross_feature,
                    target: *target,
                }),
        );
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Publishes the joined semantic-metadata projection required by KerML 8.3.4.12.3 and the
/// implied specialization it denotes. Every endpoint comes from an owned typed fact: annotation
/// identity, annotated element, resolved metadata typing, resolved `baseType` redefinition,
/// expression operand, and effective feature typing.
pub(crate) struct SemanticMetadataSynthesis {
    pub(crate) projections: Box<[SemanticMetadataProjection]>,
    pub(crate) implied_relationships: Box<[ImpliedRelationship]>,
    pub(crate) status: SemanticMetadataProjectionStatus,
}

pub(crate) struct OperatorExpressionSynthesis {
    pub(crate) implied_relationships: Box<[ImpliedRelationship]>,
    pub(crate) select_status: ExpressionArgumentProjectionStatus,
    pub(crate) index_status: ExpressionArgumentProjectionStatus,
    pub(crate) array_anchor: Option<LibrarySpecializationAnchor>,
}

pub(crate) struct ConstructorExpressionSynthesis {
    pub(crate) implied_relationships: Box<[ImpliedRelationship]>,
    pub(crate) projections: Box<[ConstructorExpressionProjection]>,
    pub(crate) status: ConstructorExpressionProjectionStatus,
    pub(crate) specialization_status: ConstructorExpressionSpecializationStatus,
    pub(crate) anchor: LibrarySpecializationAnchor,
}

pub(crate) fn synthesize_constructor_expression_result_specializations(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> Result<ConstructorExpressionSynthesis, ResolutionError> {
    let mut implied = Vec::new();
    let mut projections = Vec::new();
    let mut status = ConstructorExpressionProjectionStatus::Complete;
    let anchor =
        resolve_library_specialization_anchor(storage, "Performances::constructorEvaluations");
    let mut specialization_status = match &anchor {
        LibrarySpecializationAnchor::Resolved(_) => {
            ConstructorExpressionSpecializationStatus::Complete
        }
        LibrarySpecializationAnchor::Missing | LibrarySpecializationAnchor::Ambiguous(_) => {
            ConstructorExpressionSpecializationStatus::Unresolved
        }
    };
    for constructor in storage.constructor_expressions.iter() {
        if let LibrarySpecializationAnchor::Resolved(anchor) = &anchor {
            implied.push(ImpliedRelationship {
                kind: ReferenceKind::Subsetting,
                source: constructor.expression,
                target: *anchor,
            });
        } else {
            specialization_status = ConstructorExpressionSpecializationStatus::Unresolved;
        }
        let mut references = storage
            .references
            .iter()
            .enumerate()
            .filter(|(_, reference)| {
                reference.source == constructor.expression
                    && reference.kind == ReferenceKind::InvocationCallee
            });
        let Some((index, _)) = references.next() else {
            status = ConstructorExpressionProjectionStatus::Unresolved;
            continue;
        };
        if references.next().is_some() {
            return Err(ResolutionError::InvalidStorage);
        }
        let id = AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let Some(ResolutionStatus::Resolved(instantiated_type)) = resolution.outcome(id) else {
            status = ConstructorExpressionProjectionStatus::Unresolved;
            continue;
        };
        let Some(target) = storage.declaration(instantiated_type) else {
            return Err(ResolutionError::InvalidStorage);
        };
        let kind = if crate::resolve::is_feature_declaration(target.kind) {
            ReferenceKind::Subsetting
        } else if crate::resolve::DeclarationDomain::Type.accepts(target.kind) {
            ReferenceKind::FeatureTyping
        } else {
            status = ConstructorExpressionProjectionStatus::Unresolved;
            continue;
        };
        implied.push(ImpliedRelationship {
            kind,
            source: constructor.result,
            target: instantiated_type,
        });
        projections.push(ConstructorExpressionProjection {
            expression: constructor.expression,
            result: constructor.result,
            instantiated_type,
        });
    }
    implied.sort_by_key(|relationship| {
        (
            relationship.kind,
            relationship.source.0,
            relationship.target.0,
        )
    });
    implied.dedup();
    projections.sort_by_key(|projection| projection.expression.0);
    Ok(ConstructorExpressionSynthesis {
        implied_relationships: implied.into_boxed_slice(),
        projections: projections.into_boxed_slice(),
        status,
        specialization_status,
        anchor,
    })
}

pub(crate) struct FeatureChainExpressionSynthesis {
    pub(crate) implied_relationships: Box<[ImpliedRelationship]>,
    pub(crate) projections: Box<[FeatureChainExpressionProjection]>,
    pub(crate) status: FeatureChainExpressionSpecializationStatus,
}

pub(crate) fn synthesize_feature_chain_expression_result_specializations(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> Result<FeatureChainExpressionSynthesis, ResolutionError> {
    let mut implied = Vec::new();
    let mut projections = Vec::new();
    let mut status = FeatureChainExpressionSpecializationStatus::Complete;
    for chain in storage.feature_chain_expressions.iter() {
        implied.extend([
            ImpliedRelationship {
                kind: ReferenceKind::FeatureChaining,
                source: chain.subsetting_chain,
                target: chain.input_parameter,
            },
            ImpliedRelationship {
                kind: ReferenceKind::FeatureChaining,
                source: chain.subsetting_chain,
                target: chain.source_target,
            },
            ImpliedRelationship {
                kind: ReferenceKind::Subsetting,
                source: chain.result,
                target: chain.subsetting_chain,
            },
        ]);
        let mut references = storage
            .references
            .iter()
            .enumerate()
            .filter(|(_, reference)| {
                reference.source == chain.expression
                    && reference.kind == ReferenceKind::MemberAccessOperand
            });
        let Some((index, _)) = references.next() else {
            status = FeatureChainExpressionSpecializationStatus::Unresolved;
            continue;
        };
        if references.next().is_some() {
            return Err(ResolutionError::InvalidStorage);
        }
        let id = AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let Some(ResolutionStatus::Resolved(target_feature)) = resolution.outcome(id) else {
            status = FeatureChainExpressionSpecializationStatus::Unresolved;
            continue;
        };
        implied.push(ImpliedRelationship {
            kind: ReferenceKind::Redefinition,
            source: chain.source_target,
            target: target_feature,
        });
        projections.push(FeatureChainExpressionProjection {
            expression: chain.expression,
            result: chain.result,
            input_parameter: chain.input_parameter,
            source_target: chain.source_target,
            target_feature,
            subsetting_chain: chain.subsetting_chain,
        });
    }
    implied.sort_by_key(|relationship| {
        (
            relationship.kind,
            relationship.source.0,
            relationship.target.0,
        )
    });
    implied.dedup();
    projections.sort_by_key(|projection| projection.expression.0);
    Ok(FeatureChainExpressionSynthesis {
        implied_relationships: implied.into_boxed_slice(),
        projections: projections.into_boxed_slice(),
        status,
    })
}

pub(crate) struct FeatureReferenceExpressionSynthesis {
    pub(crate) implied_relationships: Box<[ImpliedRelationship]>,
    pub(crate) projections: Box<[FeatureReferenceExpressionProjection]>,
    pub(crate) status: FeatureReferenceExpressionSpecializationStatus,
}

pub(crate) fn synthesize_feature_reference_expression_result_specializations(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> Result<FeatureReferenceExpressionSynthesis, ResolutionError> {
    let mut implied = Vec::new();
    let mut projections = Vec::new();
    let mut status = FeatureReferenceExpressionSpecializationStatus::Complete;
    for expression in storage.feature_reference_expressions.iter() {
        let mut references = storage
            .references
            .iter()
            .enumerate()
            .filter(|(_, reference)| {
                reference.source == expression.expression
                    && reference.kind == ReferenceKind::ExpressionOperand
            });
        let Some((index, _)) = references.next() else {
            status = FeatureReferenceExpressionSpecializationStatus::Unresolved;
            continue;
        };
        if references.next().is_some() {
            return Err(ResolutionError::InvalidStorage);
        }
        let id = AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let Some(ResolutionStatus::Resolved(referent)) = resolution.outcome(id) else {
            status = FeatureReferenceExpressionSpecializationStatus::Unresolved;
            continue;
        };
        let Some(target) = storage.declaration(referent) else {
            return Err(ResolutionError::InvalidStorage);
        };
        if !crate::resolve::is_feature_declaration(target.kind) {
            status = FeatureReferenceExpressionSpecializationStatus::Unresolved;
            continue;
        }
        implied.push(ImpliedRelationship {
            kind: ReferenceKind::Subsetting,
            source: expression.result,
            target: referent,
        });
        projections.push(FeatureReferenceExpressionProjection {
            expression: expression.expression,
            result: expression.result,
            referent,
        });
    }
    implied.sort_by_key(|relationship| {
        (
            relationship.kind,
            relationship.source.0,
            relationship.target.0,
        )
    });
    implied.dedup();
    projections.sort_by_key(|projection| projection.expression.0);
    Ok(FeatureReferenceExpressionSynthesis {
        implied_relationships: implied.into_boxed_slice(),
        projections: projections.into_boxed_slice(),
        status,
    })
}

pub(crate) struct InvocationExpressionSynthesis {
    pub(crate) implied_relationships: Box<[ImpliedRelationship]>,
    pub(crate) projections: Box<[InvocationExpressionProjection]>,
    pub(crate) status: InvocationExpressionProjectionStatus,
}

/// Publishes the InvocationExpression's instantiated type, result, and Function classification as
/// one phase-4 fact. This is the sole derivation of the OCL `is Function` predicate: a type is a
/// Function when its concrete declaration or a Subclassification ancestor is one, while a Feature
/// is Function-valued when any canonical effective type has that classification.
pub(crate) fn synthesize_invocation_expression_result_specializations(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    effective_types: &EffectiveTypes,
) -> Result<InvocationExpressionSynthesis, ResolutionError> {
    let (ancestors, cyclic) = build_ancestor_closures(
        &storage.declarations,
        &storage.references,
        &resolution.outcomes,
    )?;
    let is_function_type = |target: DeclarationId| {
        if cyclic.contains(&target) {
            return None;
        }
        let direct = storage
            .declaration(target)
            .is_some_and(|declaration| declaration.kind == DeclarationKind::KermlFunction);
        let inherited = ancestors.get(target.index()).is_some_and(|values| {
            values.iter().any(|ancestor| {
                storage
                    .declaration(*ancestor)
                    .is_some_and(|declaration| declaration.kind == DeclarationKind::KermlFunction)
            })
        });
        Some(direct || inherited)
    };

    let mut implied = Vec::new();
    let mut projections = Vec::new();
    let mut status = InvocationExpressionProjectionStatus::Complete;
    for invocation in storage.invocations.iter() {
        let expression = invocation.declaration;
        let Some(result) = storage
            .declaration_facts(expression)
            .and_then(|facts| facts.expression_result)
        else {
            status = InvocationExpressionProjectionStatus::Unresolved;
            continue;
        };
        let Some(ResolutionStatus::Resolved(instantiated_type)) =
            resolution.outcome(invocation.callee)
        else {
            status = InvocationExpressionProjectionStatus::Unresolved;
            continue;
        };
        let Some(target) = storage.declaration(instantiated_type) else {
            return Err(ResolutionError::InvalidStorage);
        };
        let feature = crate::resolve::is_feature_declaration(target.kind);
        let instantiated_type_kind = if feature {
            let mut typed_by_function = false;
            let mut classification_complete = true;
            for (effective_type, _) in effective_types.row(instantiated_type) {
                match is_function_type(*effective_type) {
                    Some(true) => typed_by_function = true,
                    Some(false) => {}
                    None => classification_complete = false,
                }
            }
            if !classification_complete {
                status = InvocationExpressionProjectionStatus::Unresolved;
                continue;
            }
            if typed_by_function {
                InvocationInstantiatedTypeKind::FeatureTypedByFunction
            } else {
                InvocationInstantiatedTypeKind::NonFunctionFeature
            }
        } else if !crate::resolve::DeclarationDomain::Type.accepts(target.kind) {
            status = InvocationExpressionProjectionStatus::Unresolved;
            continue;
        } else {
            match is_function_type(instantiated_type) {
                Some(true) => InvocationInstantiatedTypeKind::Function,
                Some(false) => InvocationInstantiatedTypeKind::NonFunctionType,
                None => {
                    status = InvocationExpressionProjectionStatus::Unresolved;
                    continue;
                }
            }
        };
        if !instantiated_type_kind.is_function() {
            implied.push(ImpliedRelationship {
                kind: if feature {
                    ReferenceKind::Subsetting
                } else {
                    ReferenceKind::FeatureTyping
                },
                source: result,
                target: instantiated_type,
            });
        }
        projections.push(InvocationExpressionProjection {
            expression,
            result,
            instantiated_type,
            instantiated_type_kind,
        });
    }
    implied.sort_by_key(|relationship| {
        (
            relationship.kind,
            relationship.source.0,
            relationship.target.0,
        )
    });
    implied.dedup();
    projections.sort_by_key(|projection| projection.expression.0);
    Ok(InvocationExpressionSynthesis {
        implied_relationships: implied.into_boxed_slice(),
        projections: projections.into_boxed_slice(),
        status,
    })
}

pub(crate) fn synthesize_operator_expression_result_specializations(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> Result<OperatorExpressionSynthesis, ResolutionError> {
    let has_index = storage
        .operator_expressions
        .iter()
        .any(|record| record.kind == crate::lower::facts::OperatorExpressionKind::Index);
    let array =
        has_index.then(|| resolve_library_specialization_anchor(storage, "Collections::Array"));
    let mut implied = Vec::new();
    let mut select_status = ExpressionArgumentProjectionStatus::Complete;
    let mut index_status = ExpressionArgumentProjectionStatus::Complete;
    for operator in storage.operator_expressions.iter() {
        let mut first = storage
            .expression_arguments
            .iter()
            .filter(|argument| argument.expression == operator.expression && argument.ordinal == 0);
        let Some(argument) = first.next() else {
            match operator.kind {
                crate::lower::facts::OperatorExpressionKind::Index => {
                    index_status = ExpressionArgumentProjectionStatus::Unresolved
                }
                crate::lower::facts::OperatorExpressionKind::Select => {
                    select_status = ExpressionArgumentProjectionStatus::Unresolved
                }
            }
            continue;
        };
        if first.next().is_some() {
            return Err(ResolutionError::InvalidStorage);
        }
        if operator.kind == crate::lower::facts::OperatorExpressionKind::Index {
            match array.as_ref() {
                Some(LibrarySpecializationAnchor::Resolved(array)) => {
                    match settled_specializes(storage, resolution, argument.result, *array)? {
                        SettledSpecialization::Conforms => continue,
                        SettledSpecialization::DoesNotConform => {}
                        SettledSpecialization::Unresolved => {
                            index_status = ExpressionArgumentProjectionStatus::Unresolved;
                            continue;
                        }
                    }
                }
                _ => {
                    index_status = ExpressionArgumentProjectionStatus::Unresolved;
                    continue;
                }
            }
        }
        implied.push(ImpliedRelationship {
            kind: ReferenceKind::Subsetting,
            source: operator.result,
            target: argument.result,
        });
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(OperatorExpressionSynthesis {
        implied_relationships: implied.into_boxed_slice(),
        select_status,
        index_status,
        array_anchor: array,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettledSpecialization {
    Conforms,
    DoesNotConform,
    Unresolved,
}

fn settled_specializes(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    specific: DeclarationId,
    general: DeclarationId,
) -> Result<SettledSpecialization, ResolutionError> {
    let mut pending = vec![specific];
    let mut visited = std::collections::BTreeSet::new();
    let mut unresolved = false;
    while let Some(current) = pending.pop() {
        if current == general {
            return Ok(SettledSpecialization::Conforms);
        }
        if !visited.insert(current) {
            continue;
        }
        for (index, reference) in storage.references.iter().enumerate() {
            if reference.source != current
                || !matches!(
                    reference.kind,
                    ReferenceKind::Subclassification
                        | ReferenceKind::Subsetting
                        | ReferenceKind::Redefinition
                        | ReferenceKind::FeatureTyping
                )
            {
                continue;
            }
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            match resolution.outcome(id) {
                Some(ResolutionStatus::Resolved(target)) => pending.push(target),
                Some(
                    ResolutionStatus::Unresolved
                    | ResolutionStatus::Ambiguous(_)
                    | ResolutionStatus::Unsupported
                    | ResolutionStatus::NonConverged,
                )
                | None => unresolved = true,
            }
        }
        pending.extend(
            resolution
                .implied_relationships
                .iter()
                .filter(|edge| edge.source == current)
                .map(|edge| edge.target),
        );
    }
    Ok(if unresolved {
        SettledSpecialization::Unresolved
    } else {
        SettledSpecialization::DoesNotConform
    })
}

pub(crate) fn synthesize_semantic_metadata_specializations(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    types: &EffectiveTypes,
) -> Result<SemanticMetadataSynthesis, ResolutionError> {
    if storage.metadata_annotations.is_empty() {
        return Ok(SemanticMetadataSynthesis {
            projections: Box::default(),
            implied_relationships: Box::default(),
            status: SemanticMetadataProjectionStatus::Complete,
        });
    }
    let LibrarySpecializationAnchor::Resolved(semantic_metadata) =
        resolve_library_specialization_anchor(storage, "Metaobjects::SemanticMetadata")
    else {
        return Ok(SemanticMetadataSynthesis {
            projections: Box::default(),
            implied_relationships: Box::default(),
            status: SemanticMetadataProjectionStatus::Unresolved,
        });
    };
    let LibrarySpecializationAnchor::Resolved(base_type) =
        resolve_library_specialization_anchor(storage, "Metaobjects::SemanticMetadata::baseType")
    else {
        return Ok(SemanticMetadataSynthesis {
            projections: Box::default(),
            implied_relationships: Box::default(),
            status: SemanticMetadataProjectionStatus::Unresolved,
        });
    };

    let mut projections = Vec::new();
    let mut implied = Vec::new();
    let mut status = SemanticMetadataProjectionStatus::Complete;
    for record in storage.metadata_annotations.iter() {
        let metadata_type = storage
            .references
            .iter()
            .enumerate()
            .find_map(|(index, reference)| {
                (reference.source == record.annotation
                    && reference.kind == ReferenceKind::MetadataAnnotation)
                    .then(|| {
                        let id = AuthoredReferenceId::from_index(index).ok()?;
                        match resolution.outcome(id) {
                            Some(ResolutionStatus::Resolved(target)) => Some(target),
                            _ => None,
                        }
                    })
                    .flatten()
            });
        let Some(metadata_type) = metadata_type else {
            status = SemanticMetadataProjectionStatus::Unresolved;
            continue;
        };
        match settled_specializes(storage, resolution, metadata_type, semantic_metadata)? {
            SettledSpecialization::Conforms => {}
            SettledSpecialization::DoesNotConform => continue,
            SettledSpecialization::Unresolved => {
                status = SemanticMetadataProjectionStatus::Unresolved;
                continue;
            }
        }
        let mut value = None;
        for candidate in storage.feature_values.iter() {
            let Some(owner) = storage
                .declaration(candidate.declaration)
                .and_then(|decl| decl.owner)
            else {
                continue;
            };
            if matches!(
                settled_specializes(storage, resolution, metadata_type, owner)?,
                SettledSpecialization::Conforms
            ) && storage
                .references
                .iter()
                .enumerate()
                .any(|(index, reference)| {
                    reference.source == candidate.declaration
                        && reference.kind == ReferenceKind::Redefinition
                        && AuthoredReferenceId::from_index(index)
                            .ok()
                            .is_some_and(|id| {
                                resolution.outcome(id)
                                    == Some(ResolutionStatus::Resolved(base_type))
                            })
                })
            {
                value = Some(candidate);
                break;
            }
        }
        let Some(value) = value else {
            status = SemanticMetadataProjectionStatus::Unresolved;
            continue;
        };
        let syntax_element =
            storage
                .references
                .iter()
                .enumerate()
                .find_map(|(index, reference)| {
                    (reference.source == value.value
                        && reference.kind == ReferenceKind::ExpressionOperand)
                        .then(|| {
                            let id = AuthoredReferenceId::from_index(index).ok()?;
                            match resolution.outcome(id) {
                                Some(ResolutionStatus::Resolved(target)) => Some(target),
                                _ => None,
                            }
                        })
                        .flatten()
                });
        let Some(syntax_element) = syntax_element else {
            status = SemanticMetadataProjectionStatus::Unresolved;
            continue;
        };
        let is_feature = |declaration: DeclarationId| {
            storage.declaration(declaration).is_some_and(|decl| {
                crate::resolve::is_usage_declaration(decl.kind)
                    || matches!(
                        element_kind::element_kind(decl.kind),
                        crate::ElementKind::Feature
                            | crate::ElementKind::Step
                            | crate::ElementKind::Expression
                            | crate::ElementKind::BooleanExpression
                            | crate::ElementKind::Connector
                            | crate::ElementKind::BindingConnector
                            | crate::ElementKind::Invariant
                    )
            })
        };
        let annotated_is_feature = is_feature(record.annotated_element);
        let syntax_is_feature = is_feature(syntax_element);
        let targets = if !annotated_is_feature && syntax_is_feature {
            types
                .row(syntax_element)
                .iter()
                .map(|(target, _)| *target)
                .collect::<Vec<_>>()
        } else {
            vec![syntax_element]
        };
        if targets.is_empty() {
            status = SemanticMetadataProjectionStatus::Unresolved;
        }
        for target in targets {
            projections.push(SemanticMetadataProjection {
                annotation: record.annotation,
                annotated_element: record.annotated_element,
                syntax_element,
                specialization_target: target,
            });
            implied.push(ImpliedRelationship {
                kind: if annotated_is_feature {
                    ReferenceKind::Subsetting
                } else {
                    ReferenceKind::Subclassification
                },
                source: record.annotated_element,
                target,
            });
        }
    }
    projections
        .sort_by_key(|projection| (projection.annotation.0, projection.specialization_target.0));
    implied.sort_by_key(|relationship| {
        (
            relationship.kind,
            relationship.source.0,
            relationship.target.0,
        )
    });
    implied.dedup();
    Ok(SemanticMetadataSynthesis {
        projections: projections.into_boxed_slice(),
        implied_relationships: implied.into_boxed_slice(),
        status,
    })
}

pub(crate) fn library_specialization_anchors(
    storage: &SemanticModelStorage,
) -> LibrarySpecializationAnchorFacts {
    let anchors = GENERATED_LIBRARY_SPECIALIZATION_RULES
        .iter()
        .map(|rule| {
            (
                rule.rule_id,
                LibrarySpecializationAnchorBranch::Default,
                rule.anchor,
            )
        })
        .chain(
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .flat_map(|rule| {
                    std::iter::once((
                        rule.rule_id,
                        LibrarySpecializationAnchorBranch::Default,
                        rule.anchor,
                    ))
                    .chain(rule.true_anchor.map(|anchor| {
                        (
                            rule.rule_id,
                            LibrarySpecializationAnchorBranch::PredicateTrue,
                            anchor,
                        )
                    }))
                }),
        )
        .chain(GENERATED_LIBRARY_REDEFINITION_RULES.iter().map(|rule| {
            (
                rule.rule_id,
                LibrarySpecializationAnchorBranch::Default,
                rule.anchor,
            )
        }))
        .map(|(rule_id, branch, anchor)| {
            (
                LibrarySpecializationAnchorKey {
                    rule: LibrarySpecializationRuleKey(rule_id),
                    branch,
                },
                resolve_library_specialization_anchor(storage, anchor),
            )
        })
        .collect();
    LibrarySpecializationAnchorFacts { by_rule: anchors }
}

pub(crate) fn resolve_library_specialization_anchor(
    storage: &SemanticModelStorage,
    anchor: &'static str,
) -> LibrarySpecializationAnchor {
    let parts = anchor.split("::").collect::<Vec<_>>();
    let Some((&last, owners)) = parts.split_last() else {
        return LibrarySpecializationAnchor::Missing;
    };
    let mut candidates = storage
        .declarations
        .iter()
        .enumerate()
        .filter_map(|(index, declaration)| {
            (storage
                .document(declaration.document)
                .is_some_and(|document| document.role == SourceRole::StandardLibrary)
                && declaration
                    .name
                    .is_some_and(|name| storage.symbol(name) == Some(last))
                && anchor_owner_path_matches(storage, declaration.owner, owners))
            .then(|| DeclarationId::from_index(index).ok())
            .flatten()
        })
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates.dedup();
    match candidates.len() {
        0 => LibrarySpecializationAnchor::Missing,
        1 => LibrarySpecializationAnchor::Resolved(candidates[0]),
        _ => LibrarySpecializationAnchor::Ambiguous(candidates.into_boxed_slice()),
    }
}

/// Checks the structural containment path of a normative anchor, outermost first.
///
/// This deliberately stops at the source root and therefore cannot treat an arbitrary nested
/// package named `Parts` as the language-owned library namespace.
pub(crate) fn anchor_owner_path_matches(
    storage: &SemanticModelStorage,
    owner: Option<DeclarationId>,
    expected: &[&str],
) -> bool {
    let mut cursor = owner;
    for name in expected.iter().rev() {
        let Some(current) = cursor else {
            return false;
        };
        let Some(declaration) = storage.declaration(current) else {
            return false;
        };
        if !declaration
            .name
            .is_some_and(|id| storage.symbol(id) == Some(*name))
        {
            return false;
        }
        cursor = declaration.owner;
    }
    cursor.is_none()
}

/// Applies every exact unconditional manifest rule through the typed declaration-kind projection.
/// The generated table is the authority for both applicability metaclass and library anchor.
pub(crate) fn synthesize_generated_library_specializations(
    storage: &SemanticModelStorage,
    references: &[AuthoredReference],
    outcomes: &[ResolutionStatus],
    anchor_facts: &LibrarySpecializationAnchorFacts,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let (ancestors, cyclic) = build_ancestor_closures(&storage.declarations, references, outcomes)?;
    let mut implied = Vec::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        for metaclass in std::iter::once(library_rule_metaclass(declaration.kind))
            .chain((declaration.kind == DeclarationKind::Flow).then_some("Flow"))
        {
            for rule in library_specialization_rules(metaclass) {
                let Some(LibrarySpecializationAnchor::Resolved(anchor)) =
                    anchor_facts.generated_outcome(rule.rule_id)
                else {
                    continue;
                };
                if source == *anchor
                    || cyclic.contains(&source)
                    || ancestors
                        .get(source.index())
                        .is_some_and(|set| set.contains(anchor))
                {
                    continue;
                }
                implied.push(ImpliedRelationship {
                    kind: implied_library_specialization_kind(storage, source, *anchor)?,
                    source,
                    target: *anchor,
                });
            }
            for rule in conditional_library_specialization_rules(metaclass) {
                if !conditional_library_specialization_predicate_holds_with_resolution(
                    storage, source, rule, references, outcomes,
                ) {
                    continue;
                }
                let branch =
                    conditional_library_specialization_anchor_branch(storage, source, rule);
                let Some(LibrarySpecializationAnchor::Resolved(anchor)) =
                    anchor_facts.generated_outcome_for(rule.rule_id, branch)
                else {
                    continue;
                };
                if source == *anchor
                    || cyclic.contains(&source)
                    || ancestors
                        .get(source.index())
                        .is_some_and(|set| set.contains(anchor))
                {
                    continue;
                }
                implied.push(ImpliedRelationship {
                    kind: implied_library_specialization_kind(storage, source, *anchor)?,
                    source,
                    target: *anchor,
                });
            }
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Dependency-complete library edges whose prerequisites are settled before authored names resolve.
///
/// These are provisional solver inputs, not a second published relationship store. The final
/// synthesis below re-evaluates necessity against settled authored ancestry and publishes the
/// canonical implied set. Conditional rules whose predicates require resolved relationships stay
/// absent here; predicates owned entirely by lowered declaration facts participate immediately.
pub(crate) fn provisional_library_specializations(
    storage: &SemanticModelStorage,
    anchor_facts: &LibrarySpecializationAnchorFacts,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    if !anchor_facts.has_resolved_anchor() {
        return Ok(Box::default());
    }
    let mut implied = Vec::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        for metaclass in std::iter::once(library_rule_metaclass(declaration.kind))
            .chain((declaration.kind == DeclarationKind::Flow).then_some("Flow"))
        {
            for rule in library_specialization_rules(metaclass) {
                let Some(LibrarySpecializationAnchor::Resolved(anchor)) =
                    anchor_facts.generated_outcome(rule.rule_id)
                else {
                    continue;
                };
                if source == *anchor {
                    continue;
                }
                implied.push(ImpliedRelationship {
                    kind: implied_library_specialization_kind(storage, source, *anchor)?,
                    source,
                    target: *anchor,
                });
            }
            for rule in conditional_library_specialization_rules(metaclass) {
                if !conditional_library_specialization_predicate_holds(storage, source, rule) {
                    continue;
                }
                let branch =
                    conditional_library_specialization_anchor_branch(storage, source, rule);
                let Some(LibrarySpecializationAnchor::Resolved(anchor)) =
                    anchor_facts.generated_outcome_for(rule.rule_id, branch)
                else {
                    continue;
                };
                if source == *anchor {
                    continue;
                }
                implied.push(ImpliedRelationship {
                    kind: implied_library_specialization_kind(storage, source, *anchor)?,
                    source,
                    target: *anchor,
                });
            }
        }
    }
    implied.sort_by_key(|relationship| {
        (
            relationship.kind,
            relationship.source.0,
            relationship.target.0,
        )
    });
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// The concrete specialization relationship required by the source and target metaclasses.
///
/// KerML Table 10 does not make every `specializesFromLibrary` constraint a
/// Subclassification: Feature-to-Feature specialization is Subsetting, while Classifier-to-
/// Classifier specialization is Subclassification. A Feature specialized by a Classifier is
/// typed by it. Keeping that distinction on the canonical edge is what lets feature inheritance
/// and effective typing consume the generated rule without a private reinterpretation.
fn implied_library_specialization_kind(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    target: DeclarationId,
) -> Result<ReferenceKind, ResolutionError> {
    let source = storage
        .declaration(source)
        .ok_or(ResolutionError::InvalidStorage)?;
    let target = storage
        .declaration(target)
        .ok_or(ResolutionError::InvalidStorage)?;
    match (
        crate::resolve::is_feature_declaration(source.kind),
        crate::resolve::is_feature_declaration(target.kind),
    ) {
        (true, true) => Ok(ReferenceKind::Subsetting),
        (true, false) => Ok(ReferenceKind::FeatureTyping),
        (false, false) => Ok(ReferenceKind::Subclassification),
        // A Classifier cannot specialize a Feature with one direct relationship. The only
        // normative form needing that shape specializes the Classifier by the Feature's types and
        // is synthesized by its dedicated metadata rule, not this one-anchor contract.
        (false, true) => Err(ResolutionError::InvalidStorage),
    }
}

/// Evaluates only the exact predicate vocabulary emitted by the pinned-manifest extractor.
/// The predicate is the manifest's closed enum, so adding a contract requires an exhaustive
/// resolver decision rather than falling through a similarly spelled string label.
pub(crate) fn conditional_library_specialization_predicate_holds(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    rule: &ConditionalLibrarySpecializationRule,
) -> bool {
    let Some(declaration) = storage.declaration(source) else {
        return false;
    };
    let Some(facts) = storage.declaration_facts(source) else {
        return false;
    };
    match rule.predicate {
        LibrarySpecializationPredicate::IsIndividual => {
            declaration.kind == DeclarationKind::OccurrenceDefinition && facts.modifiers.individual
        }
        LibrarySpecializationPredicate::PortionKindSnapshot => {
            declaration.kind == DeclarationKind::OccurrenceUsage
                && facts.portion_kind == Some(PortionKind::Snapshot)
        }
        LibrarySpecializationPredicate::PortionKindTimeslice => {
            declaration.kind == DeclarationKind::OccurrenceUsage
                && facts.portion_kind == Some(PortionKind::Timeslice)
        }
        LibrarySpecializationPredicate::CompositeOwnedBy => {
            facts.modifiers.composite
                && declaration.owner.is_some_and(|owner| {
                    storage.declaration(owner).is_some_and(|owner| {
                        rule.owner_metaclasses
                            .contains(&library_rule_metaclass(owner.kind))
                    })
                })
        }
        LibrarySpecializationPredicate::OwnedBy => declaration.owner.is_some_and(|owner| {
            storage.declaration(owner).is_some_and(|owner| {
                rule.owner_metaclasses
                    .contains(&library_rule_metaclass(owner.kind))
            })
        }),
        LibrarySpecializationPredicate::IsSubactionUsage => {
            facts.modifiers.composite
                && declaration.owner.is_some_and(|owner| {
                    storage.declaration(owner).is_some_and(|owner| {
                        matches!(
                            owner.kind,
                            DeclarationKind::ActionDefinition
                                | DeclarationKind::ActionUsage
                                | DeclarationKind::AcceptActionUsage
                                | DeclarationKind::SendActionUsage
                                | DeclarationKind::TerminateActionUsage
                        )
                    })
                })
        }
        LibrarySpecializationPredicate::IsNotTriggerAction => {
            declaration.kind == DeclarationKind::AcceptActionUsage
                && facts.is_trigger_action == Some(false)
        }
        LibrarySpecializationPredicate::IsSubactionUsageAndNotTriggerAction => {
            declaration.kind == DeclarationKind::AcceptActionUsage
                && facts.is_trigger_action == Some(false)
                && facts.modifiers.composite
                && declaration.owner.is_some_and(|owner| {
                    storage.declaration(owner).is_some_and(|owner| {
                        matches!(
                            owner.kind,
                            DeclarationKind::ActionDefinition
                                | DeclarationKind::ActionUsage
                                | DeclarationKind::AcceptActionUsage
                                | DeclarationKind::SendActionUsage
                                | DeclarationKind::TerminateActionUsage
                        )
                    })
                })
        }
        LibrarySpecializationPredicate::IsTriggerAction => {
            declaration.kind == DeclarationKind::AcceptActionUsage
                && facts.is_trigger_action == Some(true)
        }
        LibrarySpecializationPredicate::HasElseActionBranch => {
            declaration.kind == DeclarationKind::If && facts.has_else_action.is_some()
        }
        LibrarySpecializationPredicate::OwnedEndFeatureCountIsTwo
        | LibrarySpecializationPredicate::ConnectorEndCountIsTwo
        | LibrarySpecializationPredicate::AssociationEndCountIsTwo
        | LibrarySpecializationPredicate::EndFeatureCountIsTwo => {
            positional_end_count(storage, source) == 2
        }
        LibrarySpecializationPredicate::FlowEndCountIsTwo => {
            declaration.kind == DeclarationKind::FlowDefinition
                && positional_end_count(storage, source) == 2
        }
        LibrarySpecializationPredicate::OwnedEndFeaturesNotEmpty => {
            declaration.kind == DeclarationKind::Flow
                && facts.owned_end_feature_count.is_some_and(|count| count > 0)
        }
        LibrarySpecializationPredicate::OwnedTypingDataType
        | LibrarySpecializationPredicate::OwnedTypingClass
        | LibrarySpecializationPredicate::OwnedTypingStructure
        | LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector
        | LibrarySpecializationPredicate::ConnectorAssociationStructure => false,
        LibrarySpecializationPredicate::PolarityBranch => facts.negated.is_some(),
        LibrarySpecializationPredicate::FramedConcernMembership => {
            element_kind::membership_role(declaration.kind) == Some(MembershipRole::FramedConcern)
        }
        LibrarySpecializationPredicate::RequirementConstraintMembershipKind => matches!(
            element_kind::membership_role(declaration.kind),
            Some(MembershipRole::RequirementConstraint(
                RequirementConstraintKind::Assumption | RequirementConstraintKind::Requirement
            ))
        ),
        LibrarySpecializationPredicate::ActorMembershipOwningRequirement => {
            element_kind::membership_role(declaration.kind) == Some(MembershipRole::Actor)
        }
        LibrarySpecializationPredicate::StakeholderMembership => {
            element_kind::membership_role(declaration.kind) == Some(MembershipRole::Stakeholder)
        }
        LibrarySpecializationPredicate::RequirementVerificationMembership => {
            element_kind::membership_role(declaration.kind)
                == Some(MembershipRole::RequirementVerification)
        }
    }
}

/// Evaluates the exact predicates whose prerequisite is a direct, already-settled
/// `FeatureTyping` relationship. The relationship and its target are canonical authored and
/// resolved facts; this owner never rereads syntax or derives a type from a display name.
pub(crate) fn conditional_library_specialization_predicate_holds_with_resolution(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    rule: &ConditionalLibrarySpecializationRule,
    references: &[AuthoredReference],
    outcomes: &[ResolutionStatus],
) -> bool {
    match rule.predicate {
        LibrarySpecializationPredicate::OwnedTypingDataType => {
            direct_owned_typing_targets(storage, source, references, outcomes).any(|target| {
                storage
                    .declaration(target)
                    .is_some_and(|declaration| declaration.kind == DeclarationKind::KermlDataType)
            })
        }
        LibrarySpecializationPredicate::OwnedTypingClass => {
            direct_owned_typing_targets(storage, source, references, outcomes).any(|target| {
                storage
                    .declaration(target)
                    .is_some_and(|declaration| declaration_kind_is_class(declaration.kind))
            })
        }
        LibrarySpecializationPredicate::OwnedTypingStructure => {
            direct_owned_typing_targets(storage, source, references, outcomes).any(|target| {
                storage
                    .declaration(target)
                    .is_some_and(|declaration| declaration_kind_is_structure(declaration.kind))
            })
        }
        LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector => storage
            .declaration(source)
            .zip(storage.declaration_facts(source))
            .is_some_and(|(declaration, facts)| {
                facts.modifiers.end
                    && declaration.owner.is_some_and(|owner| {
                        storage.declaration(owner).is_some_and(|owner| {
                            matches!(
                                owner.kind,
                                DeclarationKind::KermlAssociation | DeclarationKind::KermlConnector
                            )
                        })
                    })
            }),
        LibrarySpecializationPredicate::ConnectorAssociationStructure => {
            declaration_has_direct_association_structure_typing(
                storage, source, references, outcomes,
            )
        }
        _ => conditional_library_specialization_predicate_holds(storage, source, rule),
    }
}

/// `Connector::association` is a derived subset of the connector's `type` collection constrained
/// to `Association`. The canonical direct `FeatureTyping` relationship already owns that source
/// collection; filtering its settled target by the exact `AssociationStructure` metaclass here
/// gives the two closed Connector predicates their sole semantic input without inspecting syntax.
pub(crate) fn declaration_has_direct_association_structure_typing(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    references: &[AuthoredReference],
    outcomes: &[ResolutionStatus],
) -> bool {
    storage
        .declaration(source)
        .is_some_and(|declaration| declaration.kind == DeclarationKind::KermlConnector)
        && direct_owned_typing_targets(storage, source, references, outcomes).any(|target| {
            storage.declaration(target).is_some_and(|declaration| {
                declaration.kind == DeclarationKind::KermlAssociationStructure
            })
        })
}

pub(crate) fn direct_owned_typing_targets<'a>(
    storage: &'a SemanticModelStorage,
    source: DeclarationId,
    references: &'a [AuthoredReference],
    outcomes: &'a [ResolutionStatus],
) -> impl Iterator<Item = DeclarationId> + 'a {
    references
        .iter()
        .enumerate()
        .filter(move |(_, reference)| {
            reference.source == source && reference.kind == ReferenceKind::FeatureTyping
        })
        .filter_map(move |(index, _)| match outcomes.get(index) {
            Some(ResolutionStatus::Resolved(target)) => Some(*target),
            _ => None,
        })
        .filter(move |target| storage.declaration(*target).is_some())
}

/// KerML's static metaclass test for the concrete declaration kinds represented by this model.
/// This is language schema, not model typing: it must not be inferred from a declaration name or
/// from whichever library happens to be admitted.
pub(crate) const fn declaration_kind_is_class(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ClassDefinition
            | DeclarationKind::KermlStructure
            | DeclarationKind::KermlAssociationStructure
            | DeclarationKind::KermlBehavior
            | DeclarationKind::KermlFunction
            | DeclarationKind::KermlPredicate
            | DeclarationKind::KermlInteraction
    )
}

/// KerML `Structure` and its represented concrete subtype `AssociationStructure`.
pub(crate) const fn declaration_kind_is_structure(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::KermlStructure | DeclarationKind::KermlAssociationStructure
    )
}

/// Selects the already-published branch of a closed conditional contract. The `then` branch is a
/// typed key shared by exact polarity and membership-role contracts; every other predicate retains
/// the compatibility default anchor. A missing canonical fact was rejected by predicate
/// evaluation above, rather than silently selecting a branch.
pub(crate) fn conditional_library_specialization_anchor_branch(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    rule: &ConditionalLibrarySpecializationRule,
) -> LibrarySpecializationAnchorBranch {
    let predicate_true = match rule.predicate {
        LibrarySpecializationPredicate::PolarityBranch => storage
            .declaration_facts(source)
            .is_some_and(|facts| facts.negated == Some(true)),
        LibrarySpecializationPredicate::HasElseActionBranch => storage
            .declaration_facts(source)
            .is_some_and(|facts| facts.has_else_action == Some(true)),
        LibrarySpecializationPredicate::RequirementConstraintMembershipKind => {
            storage.declaration(source).is_some_and(|declaration| {
                element_kind::membership_role(declaration.kind)
                    == Some(MembershipRole::RequirementConstraint(
                        RequirementConstraintKind::Assumption,
                    ))
            })
        }
        LibrarySpecializationPredicate::ActorMembershipOwningRequirement => storage
            .declaration(source)
            .and_then(|declaration| declaration.owner)
            .and_then(|owner| storage.declaration(owner))
            .is_some_and(|owner| {
                matches!(
                    owner.kind,
                    DeclarationKind::RequirementDefinition | DeclarationKind::RequirementUsage
                )
            }),
        LibrarySpecializationPredicate::IsIndividual
        | LibrarySpecializationPredicate::PortionKindSnapshot
        | LibrarySpecializationPredicate::PortionKindTimeslice
        | LibrarySpecializationPredicate::CompositeOwnedBy
        | LibrarySpecializationPredicate::OwnedEndFeatureCountIsTwo
        | LibrarySpecializationPredicate::ConnectorEndCountIsTwo
        | LibrarySpecializationPredicate::AssociationEndCountIsTwo
        | LibrarySpecializationPredicate::EndFeatureCountIsTwo
        | LibrarySpecializationPredicate::FlowEndCountIsTwo
        | LibrarySpecializationPredicate::OwnedEndFeaturesNotEmpty
        | LibrarySpecializationPredicate::OwnedTypingDataType
        | LibrarySpecializationPredicate::OwnedTypingClass
        | LibrarySpecializationPredicate::OwnedTypingStructure
        | LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector
        | LibrarySpecializationPredicate::ConnectorAssociationStructure
        | LibrarySpecializationPredicate::OwnedBy
        | LibrarySpecializationPredicate::IsSubactionUsage
        | LibrarySpecializationPredicate::IsNotTriggerAction
        | LibrarySpecializationPredicate::IsSubactionUsageAndNotTriggerAction
        | LibrarySpecializationPredicate::IsTriggerAction
        | LibrarySpecializationPredicate::FramedConcernMembership
        | LibrarySpecializationPredicate::StakeholderMembership
        | LibrarySpecializationPredicate::RequirementVerificationMembership => false,
    };
    if predicate_true {
        LibrarySpecializationAnchorBranch::PredicateTrue
    } else {
        LibrarySpecializationAnchorBranch::Default
    }
}

/// The canonical structural representation for the exact XMI end collections is each child
/// declaration's owned positional-end fact. The generated rule's metaclass and closed predicate
/// still distinguish `connectorEnd`, `associationEnd`, `endFeature`, and `ownedEndFeature`; this
/// helper only owns their shared storage projection.
pub(crate) fn positional_end_count(storage: &SemanticModelStorage, owner: DeclarationId) -> usize {
    storage
        .declarations
        .iter()
        .enumerate()
        .filter(|(index, declaration)| {
            declaration.owner == Some(owner)
                && DeclarationId::from_index(*index)
                    .ok()
                    .and_then(|member| storage.declaration_facts(member))
                    .is_some_and(|facts| facts.positional_end.is_some())
        })
        .count()
}

/// Applies exact unconditional `redefinesFromLibrary` rules after authored references have
/// settled. An authored redefinition always suppresses the generated edge for that source, so an
/// implied fact never disguises or competes with source intent. Rules whose metaclass has no
/// lowered declaration projection produce no edge; their canonical anchor remains queryable by
/// stable rule ID rather than being guessed from a similarly named declaration.
pub(crate) fn synthesize_generated_library_redefinitions(
    storage: &SemanticModelStorage,
    references: &[AuthoredReference],
    anchor_facts: &LibrarySpecializationAnchorFacts,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let authored_sources: std::collections::BTreeSet<_> = references
        .iter()
        .filter(|reference| reference.kind == ReferenceKind::Redefinition)
        .map(|reference| reference.source)
        .collect();
    let mut implied = Vec::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if authored_sources.contains(&source) {
            continue;
        }
        for metaclass in std::iter::once(library_rule_metaclass(declaration.kind))
            .chain((declaration.kind == DeclarationKind::Flow).then_some("Flow"))
        {
            for rule in library_redefinition_rules(metaclass) {
                let Some(LibrarySpecializationAnchor::Resolved(anchor)) =
                    anchor_facts.generated_outcome(rule.rule_id)
                else {
                    continue;
                };
                if source != *anchor {
                    implied.push(ImpliedRelationship {
                        kind: ReferenceKind::Redefinition,
                        source,
                        target: *anchor,
                    });
                }
            }
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Materializes KerML's `checkFeatureFeatureMembershipTypeFeaturing` semantic consequence.
///
/// A non-`var` Feature owned through a FeatureMembership is featured by that membership's owning
/// Type. This runs after reference convergence and writes the same canonical relationship store as
/// authored `featured by` clauses. Any authored TypeFeaturing for the source suppresses the
/// boilerplate edge, preserving explicit source intent and avoiding a second competing fact.
///
/// `var` Features have a different normative target (the owning type's `snapshots` feature). That
/// prerequisite is not lowered yet, so this function publishes no guessed edge; the type-featuring
/// query reports the corresponding explicit unsupported outcome.
pub(crate) fn synthesize_feature_membership_type_featurings(
    storage: &SemanticModelStorage,
    references: &[AuthoredReference],
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let authored_sources = references
        .iter()
        .filter(|reference| reference.kind == ReferenceKind::TypeFeaturing)
        .map(|reference| reference.source)
        .collect::<std::collections::BTreeSet<_>>();
    let mut implied = Vec::new();
    for membership in storage.memberships.iter() {
        if membership.kind != MembershipKind::Feature
            || authored_sources.contains(&membership.member)
        {
            continue;
        }
        let Some(feature) = storage.declaration(membership.member) else {
            return Err(ResolutionError::InvalidStorage);
        };
        if storage
            .declaration_facts(membership.member)
            .is_some_and(|facts| facts.modifiers.var)
        {
            continue;
        }
        let Some(owner) = feature.owner else {
            continue;
        };
        let Some(owner_declaration) = storage.declaration(owner) else {
            return Err(ResolutionError::InvalidStorage);
        };
        if matches!(
            owner_declaration.kind,
            DeclarationKind::Namespace
                | DeclarationKind::Package
                | DeclarationKind::LibraryPackage
                | DeclarationKind::Import
                | DeclarationKind::Alias
        ) {
            continue;
        }
        implied.push(ImpliedRelationship {
            kind: ReferenceKind::TypeFeaturing,
            source: membership.member,
            target: owner,
        });
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// The generated schema-v3 metaclass spelling for a lowered declaration.
///
/// The public kind is the canonical projection for ordinary declarations. Two source forms retain
/// a more descriptive public spelling (`FlowConnection*` and `Calculation*`) while the XMI uses
/// the shorter abstract-syntax names; normalize those at this semantic boundary once rather than
/// letting generated-rule consumers maintain aliases.
pub(crate) fn library_rule_metaclass(kind: DeclarationKind) -> &'static str {
    match kind {
        DeclarationKind::FlowDefinition => "FlowDefinition",
        DeclarationKind::Flow => "FlowUsage",
        DeclarationKind::CalcDefinition => "CalculationDefinition",
        DeclarationKind::CalcUsage => "CalculationUsage",
        _ => element_kind::element_kind(kind).as_str(),
    }
}

/// Compatibility spelling for specialization-only consumers. New generated-rule owners use
/// `library_rule_metaclass`, which is the single normalization boundary for both exact families.
pub(crate) fn library_specialization_metaclass(kind: DeclarationKind) -> &'static str {
    library_rule_metaclass(kind)
}
