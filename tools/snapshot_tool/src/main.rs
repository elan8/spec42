//! Standalone source-to-snapshot harness for Spec42.
//!
//! Snapshot Markdown files are the test cases. The runner reads each file's SOURCE section,
//! builds the immutable semantic model, renders each owned derived section, and either reports
//! stale files (`check`) or rewrites them (`update`). It is intentionally a binary rather than a
//! Rust test: review happens through the normal `git diff` of the Markdown files.
//!
//! A fixture may admit the standard library by declaring `libraries=standard` in its META block.
//! The library sources are then admitted as `StandardLibrary`-role documents, so the fixture's
//! references resolve against them while the owned projections keep reporting only the fixture's
//! own authored documents.
#![recursion_limit = "256"]

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use clap::{Parser, Subcommand};
use generator_api::{ArtifactLimits, DiagramSemanticReference, GeneratorModelView, QueryLimits};
use generator_host::{CancellationHandle, GeneratorRuntime, RuntimeLimits};
use language_service::{
    hover_report, render_hover_markdown, render_hover_sexpr, InMemoryWorkspace,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use spec42_constraint_manifest::ConstraintManifestEntry;
use spec42_constraint_manifest::{
    ActionDerivedFactKind, BindingConnectorCheckKind, ConstraintFamily, ConstraintManifest,
    ConstraintQueryFamily, DefinitionUsageDerivedKind, ElementDerivedDocumentationKind,
    ElementDerivedOwnerKind, FeatureDerivedRelationshipKind, NamespaceDerivedElementKind,
    NamespaceImportDerivedElementKind, RedefinitionCheckKind, RequirementDerivedFactKind,
    SpecializationCheckKind, SpecificationId, TypeDerivedElementKind, TypeDerivedFactKind,
    TypeDerivedRelationshipKind,
};
use sysml_query::resolved_slice::{
    ActionDerivedFactCollection, ActionDerivedFactOutcome, AnnotationForm,
    BindingConnectorValidationOutcome, BindingConnectorValidationPrerequisite,
    DefinitionUsageDerivedOutcome, DefinitionUsageDerivedPrerequisite, DerivedElementOwner,
    Documentation, EditorProbe, ElementDerivedDocumentationCollection, ElementKind,
    FeatureDerivedRelationshipCollection, NamespaceDerivedElementCollection, PublishedModel,
    QualifiedElementReference, QualifiedReferenceOutcome, QualifiedReferenceProbe, QueryAnswer,
    QueryOutcome, RedefinitionCheckOutcome, RedefinitionCheckPrerequisite, RelationshipProvenance,
    RelationshipTarget, RequirementDerivedFactCollection, RequirementDerivedFactOutcome,
    RequirementDerivedFactPrerequisite, SourceKind, SpecializationCheckOutcome,
    SpecializationCheckPrerequisite, SymbolId, TextPosition, TypeDerivedElementCollection,
    TypeDerivedFactCollection, TypeDerivedFactOutcome, TypeDerivedFactValue,
    TypeDerivedRelationshipCollection,
};
use sysml_query::source::{SourceDocument as AdmittedDocument, SourceService};
type QuerySourceDocument = AdmittedDocument;

#[derive(Debug, Parser)]
#[command(
    name = "spec42-snapshot",
    about = "Regenerate Spec42 Markdown source snapshots"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    /// Root directory containing Markdown snapshots.
    #[arg(long, default_value = "tests/snapshots", global = true)]
    root: PathBuf,
    /// Restrict the operation to one path relative to --root (or an explicit path).
    #[arg(long, global = true)]
    fixture: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Compute derived sections and fail if any snapshot would change.
    Check,
    /// Rewrite all owned derived sections in place. Review with `git diff`.
    Update,
    /// Evaluate fixtures and emit their authored-expectation status without writing snapshots.
    Report {
        /// Report encoding. JSON is stable and intended for automation.
        #[arg(long, value_enum, default_value_t = ReportFormat::Text)]
        format: ReportFormat,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum ReportFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceDocument {
    name: String,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EditorProbes {
    queries: Vec<EditorProbe>,
    hover: BTreeSet<usize>,
}

/// Which libraries a fixture admits alongside its authored `SOURCE` documents.
///
/// A closed set with no default beyond `None`: an unrecognised `libraries` value is an error, so a
/// typo cannot silently produce a workspace-only publication that looks like a library-admitting
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LibrarySelection {
    None,
    Standard,
}

/// Closed repository-owned generator selection. Fixtures never supply filesystem paths.
#[derive(Debug, Clone, PartialEq, Eq)]
enum GeneratorPlugin {
    Conformance(String),
    RepositoryDiagram,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiagramSelection {
    kind: String,
    document: String,
    qualified_name: String,
}

/// Generator selection parsed from fixture metadata. Execution is deliberately kept separate from
/// Markdown parsing so the runner can provide the immutable publication to whichever WASM host it
/// uses without making the snapshot format depend on that host.
#[derive(Debug, Clone, PartialEq, Eq)]
struct GenerationRequest {
    plugin: GeneratorPlugin,
    diagram_selection: Option<DiagramSelection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FixtureMeta {
    libraries: LibrarySelection,
    repository_sources: Vec<String>,
    generation: Option<GenerationRequest>,
    standard_library_documents: BTreeSet<String>,
    normative_expectation: Option<NormativeExpectation>,
    legacy_rule_ids: Vec<String>,
}

/// The source-level contract a normative fixture is intended to exercise. This is intentionally
/// separate from the parser's observed result: a parser error alone cannot prove valid source was
/// malformed, unsupported, or affected by an upstream parser gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum SourceExpectation {
    Accepted,
    Malformed,
    Unsupported,
}

impl SourceExpectation {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "accepted" => Ok(Self::Accepted),
            "malformed" => Ok(Self::Malformed),
            "unsupported" => Ok(Self::Unsupported),
            _ => Err(format!(
                "{fixture}: unknown META source_expectation value {value:?} (expected accepted, malformed, or unsupported)"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RuleFamily {
    Derive,
    Check,
    Validate,
}

impl RuleFamily {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "derive" => Ok(Self::Derive),
            "check" => Ok(Self::Check),
            "validate" => Ok(Self::Validate),
            _ => Err(format!(
                "{fixture}: unknown META rule_family value {value:?} (expected derive, check, or validate)"
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Derive => "derive",
            Self::Check => "check",
            Self::Validate => "validate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ExpectationKind {
    Diagnostics,
    Semantics,
    ByConstruction,
}

impl ExpectationKind {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "diagnostics" => Ok(Self::Diagnostics),
            "semantics" => Ok(Self::Semantics),
            "by_construction" => Ok(Self::ByConstruction),
            _ => Err(format!(
                "{fixture}: unknown META expectation value {value:?} (expected diagnostics, semantics, or by_construction)"
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Diagnostics => "diagnostics",
            Self::Semantics => "semantics",
            Self::ByConstruction => "by_construction",
        }
    }
}

/// Whether a fixture is the canonical evidence for a rule or an additional focused case.  This
/// keeps manifest coverage strict without treating deliberately complementary regression cases
/// as accidental duplicate evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CoverageRole {
    Primary,
    Secondary,
}

impl CoverageRole {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "primary" => Ok(Self::Primary),
            "secondary" => Ok(Self::Secondary),
            _ => Err(format!(
                "{fixture}: unknown META coverage_role value {value:?} (expected primary or secondary)"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormativeExpectation {
    source_expectation: SourceExpectation,
    rule_family: RuleFamily,
    expectation: ExpectationKind,
    rule_ids: Vec<String>,
    coverage_role: CoverageRole,
    blocked_by: Option<String>,
    evidence: Option<ByConstructionEvidence>,
    specification_id: Option<SpecificationId>,
}

/// An evidence reference is deliberately closed and repository-relative.  `test:` identifies an
/// executable owning test; `file:` is for a checked-in executable harness or fixture that owns
/// the construction guarantee.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ByConstructionEvidence {
    Test(PathBuf),
    File(PathBuf),
}

impl ByConstructionEvidence {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        let (kind, path) = value.split_once(':').ok_or_else(|| {
            format!(
                "{fixture}: META evidence_reference must be test:<repository-relative-path> or file:<repository-relative-path>"
            )
        })?;
        let path = PathBuf::from(path);
        if path.as_os_str().is_empty()
            || path.is_absolute()
            || path.components().any(|component| {
                matches!(
                    component,
                    std::path::Component::ParentDir | std::path::Component::RootDir
                )
            })
        {
            return Err(format!(
                "{fixture}: META evidence_reference must name a repository-relative file"
            ));
        }
        match kind {
            "test" => Ok(Self::Test(path)),
            "file" => Ok(Self::File(path)),
            _ => Err(format!(
                "{fixture}: META evidence_reference kind {kind:?} is not supported (expected test or file)"
            )),
        }
    }

    fn path(&self) -> &Path {
        match self {
            Self::Test(path) | Self::File(path) => path,
        }
    }
}

/// An authored semantic relationship assertion. The fixture names elements readably, but the
/// evaluator resolves those names through the publication and compares only its opaque identities.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SemanticExpectations {
    relationships: Vec<RelationshipExpectation>,
    feature_derived_relationships: Vec<FeatureDerivedRelationshipExpectation>,
    type_derived_relationships: Vec<TypeDerivedRelationshipExpectation>,
    type_derived_elements: Vec<TypeDerivedElementExpectation>,
    type_derived_facts: Vec<TypeDerivedFactExpectation>,
    action_derived_facts: Vec<ActionDerivedFactExpectation>,
    definition_usage_derived: Vec<DefinitionUsageDerivedExpectation>,
    requirement_derived_facts: Vec<RequirementDerivedFactExpectation>,
    element_derived_owners: Vec<ElementDerivedOwnerExpectation>,
    element_derived_documentation: Vec<ElementDerivedDocumentationExpectation>,
    namespace_derived_elements: Vec<NamespaceDerivedElementExpectation>,
    namespace_import_derived_elements: Vec<NamespaceImportDerivedElementExpectation>,
    binding_connector_checks: Vec<BindingConnectorCheckExpectation>,
    redefinition_checks: Vec<RedefinitionCheckExpectation>,
    specialization_checks: Vec<SpecializationCheckExpectation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RelationshipExpectation {
    kind: SemanticRelationshipKind,
    source: String,
    target: Option<String>,
    provenance: Option<RelationshipProvenance>,
    outcome: SemanticRelationshipOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SemanticRelationshipKind {
    Specialization,
    FeatureTyping,
    Subsetting,
    Redefinition,
    FeatureChaining,
    TypeFeaturing,
    NamespaceImport,
    Unioning,
    Intersecting,
    Differencing,
    Disjoining,
}

impl SemanticRelationshipKind {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "specialization" => Ok(Self::Specialization),
            "feature_typing" => Ok(Self::FeatureTyping),
            "subsetting" => Ok(Self::Subsetting),
            "redefinition" => Ok(Self::Redefinition),
            "feature_chaining" => Ok(Self::FeatureChaining),
            "type_featuring" => Ok(Self::TypeFeaturing),
            "unioning" => Ok(Self::Unioning),
            "intersecting" => Ok(Self::Intersecting),
            "differencing" => Ok(Self::Differencing),
            "disjoining" => Ok(Self::Disjoining),
            _ => Err(format!(
                "{fixture}: unknown semantic relationship kind {value:?} (expected specialization, feature_typing, subsetting, redefinition, feature_chaining, type_featuring, unioning, intersecting, differencing, or disjoining)"
            )),
        }
    }

    fn query_name(self) -> &'static str {
        match self {
            Self::Specialization => "specialization",
            Self::FeatureTyping => "featureTyping",
            Self::Subsetting => "subsetting",
            Self::Redefinition => "redefinition",
            Self::FeatureChaining => "featureChaining",
            Self::TypeFeaturing => "typeFeaturing",
            Self::NamespaceImport => "namespaceImport",
            Self::Unioning => "unioning",
            Self::Intersecting => "intersecting",
            Self::Differencing => "differencing",
            Self::Disjoining => "disjoining",
        }
    }
}

/// Maps a manifest-owned typed query family to the public semantic-query selector. Rule IDs are
/// resolved by the manifest before this dispatch; the runner has no private rule-to-query table.
fn feature_collection(
    kind: FeatureDerivedRelationshipKind,
) -> FeatureDerivedRelationshipCollection {
    match kind {
        FeatureDerivedRelationshipKind::OwnedFeatureChaining => {
            FeatureDerivedRelationshipCollection::OwnedFeatureChaining
        }
        FeatureDerivedRelationshipKind::OwnedRedefinition => {
            FeatureDerivedRelationshipCollection::OwnedRedefinition
        }
        FeatureDerivedRelationshipKind::OwnedSubsetting => {
            FeatureDerivedRelationshipCollection::OwnedSubsetting
        }
        FeatureDerivedRelationshipKind::OwnedTyping => {
            FeatureDerivedRelationshipCollection::OwnedTyping
        }
        FeatureDerivedRelationshipKind::OwnedTypeFeaturing => {
            FeatureDerivedRelationshipCollection::OwnedTypeFeaturing
        }
    }
}

fn feature_collection_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<FeatureDerivedRelationshipCollection, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic derived relationship collection requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic derived relationship collection rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::FeatureDerivedRelationship(kind)) => Ok(feature_collection(kind)),
        _ => Err(format!(
            "{fixture}: semantic derived relationship collection rule_id {rule_id:?} does not own an exact Feature relationship query"
        )),
    }
}

/// One asserted member of an exact Feature-derived relationship collection. The relationship
/// itself is always read through `sysml_query::FeatureDerivedRelationshipCollection`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FeatureDerivedRelationshipExpectation {
    collection: FeatureDerivedRelationshipCollection,
    source: String,
    kind: SemanticRelationshipKind,
    target: Option<String>,
    provenance: Option<RelationshipProvenance>,
    outcome: SemanticRelationshipOutcome,
}

fn type_collection(kind: TypeDerivedRelationshipKind) -> TypeDerivedRelationshipCollection {
    match kind {
        TypeDerivedRelationshipKind::OwnedSpecialization => {
            TypeDerivedRelationshipCollection::OwnedSpecialization
        }
        TypeDerivedRelationshipKind::OwnedUnioning => {
            TypeDerivedRelationshipCollection::OwnedUnioning
        }
        TypeDerivedRelationshipKind::OwnedIntersecting => {
            TypeDerivedRelationshipCollection::OwnedIntersecting
        }
        TypeDerivedRelationshipKind::OwnedDifferencing => {
            TypeDerivedRelationshipCollection::OwnedDifferencing
        }
        TypeDerivedRelationshipKind::OwnedDisjoining => {
            TypeDerivedRelationshipCollection::OwnedDisjoining
        }
        TypeDerivedRelationshipKind::UnioningType => {
            TypeDerivedRelationshipCollection::UnioningType
        }
        TypeDerivedRelationshipKind::IntersectingType => {
            TypeDerivedRelationshipCollection::IntersectingType
        }
        TypeDerivedRelationshipKind::DifferencingType => {
            TypeDerivedRelationshipCollection::DifferencingType
        }
    }
}

fn type_element_collection(kind: TypeDerivedElementKind) -> TypeDerivedElementCollection {
    match kind {
        TypeDerivedElementKind::OwnedFeature => TypeDerivedElementCollection::OwnedFeature,
        TypeDerivedElementKind::OwnedEndFeature => TypeDerivedElementCollection::OwnedEndFeature,
    }
}

fn type_fact_collection(kind: TypeDerivedFactKind) -> TypeDerivedFactCollection {
    match kind {
        TypeDerivedFactKind::OwnedFeatureMembership => {
            TypeDerivedFactCollection::OwnedFeatureMembership
        }
        TypeDerivedFactKind::FeatureMembership => TypeDerivedFactCollection::FeatureMembership,
        TypeDerivedFactKind::Feature => TypeDerivedFactCollection::Feature,
        TypeDerivedFactKind::EndFeature => TypeDerivedFactCollection::EndFeature,
        TypeDerivedFactKind::DirectedFeature => TypeDerivedFactCollection::DirectedFeature,
        TypeDerivedFactKind::InheritedMembership => TypeDerivedFactCollection::InheritedMembership,
        TypeDerivedFactKind::InheritedFeature => TypeDerivedFactCollection::InheritedFeature,
        TypeDerivedFactKind::Input => TypeDerivedFactCollection::Input,
        TypeDerivedFactKind::Output => TypeDerivedFactCollection::Output,
        TypeDerivedFactKind::Multiplicity => TypeDerivedFactCollection::Multiplicity,
        TypeDerivedFactKind::OwnedConjugator => TypeDerivedFactCollection::OwnedConjugator,
    }
}

fn action_fact_collection(kind: ActionDerivedFactKind) -> ActionDerivedFactCollection {
    match kind {
        ActionDerivedFactKind::ActionDefinitionAction => {
            ActionDerivedFactCollection::ActionDefinitionAction
        }
        ActionDerivedFactKind::AssignmentValueExpression => {
            ActionDerivedFactCollection::AssignmentValueExpression
        }
        ActionDerivedFactKind::AssignmentTargetArgument => {
            ActionDerivedFactCollection::AssignmentTargetArgument
        }
        ActionDerivedFactKind::AssignmentReferent => {
            ActionDerivedFactCollection::AssignmentReferent
        }
        ActionDerivedFactKind::ForLoopVariable => ActionDerivedFactCollection::ForLoopVariable,
        ActionDerivedFactKind::ForLoopSeqArgument => {
            ActionDerivedFactCollection::ForLoopSeqArgument
        }
        ActionDerivedFactKind::LoopBodyAction => ActionDerivedFactCollection::LoopBodyAction,
        ActionDerivedFactKind::TerminateOccurrenceArgument => {
            ActionDerivedFactCollection::TerminateOccurrenceArgument
        }
        ActionDerivedFactKind::AcceptPayloadArgument => {
            ActionDerivedFactCollection::AcceptPayloadArgument
        }
        ActionDerivedFactKind::AcceptPayloadParameter => {
            ActionDerivedFactCollection::AcceptPayloadParameter
        }
        ActionDerivedFactKind::AcceptReceiverArgument => {
            ActionDerivedFactCollection::AcceptReceiverArgument
        }
        ActionDerivedFactKind::WhileArgument => ActionDerivedFactCollection::WhileArgument,
        ActionDerivedFactKind::UntilArgument => ActionDerivedFactCollection::UntilArgument,
        ActionDerivedFactKind::SendSenderArgument => {
            ActionDerivedFactCollection::SendSenderArgument
        }
        ActionDerivedFactKind::SendReceiverArgument => {
            ActionDerivedFactCollection::SendReceiverArgument
        }
        ActionDerivedFactKind::SendPayloadArgument => {
            ActionDerivedFactCollection::SendPayloadArgument
        }
        ActionDerivedFactKind::IfThenAction => ActionDerivedFactCollection::IfThenAction,
        ActionDerivedFactKind::IfElseAction => ActionDerivedFactCollection::IfElseAction,
        ActionDerivedFactKind::IfArgument => ActionDerivedFactCollection::IfArgument,
    }
}

fn type_collection_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<TypeDerivedRelationshipCollection, String> {
    let manifest = manifest.ok_or_else(|| {
        format!(
            "{fixture}: semantic type derived relationship collection requires a loaded manifest"
        )
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic type derived relationship collection rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::TypeDerivedRelationship(kind)) => Ok(type_collection(kind)),
        _ => Err(format!(
            "{fixture}: semantic type derived relationship collection rule_id {rule_id:?} does not own an exact Type relationship query"
        )),
    }
}

fn type_element_collection_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<TypeDerivedElementCollection, String> {
    let manifest = manifest
        .ok_or_else(|| format!("{fixture}: semantic Type element requires a loaded manifest"))?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic Type element rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::TypeDerivedElement(kind)) => Ok(type_element_collection(kind)),
        _ => Err(format!(
            "{fixture}: semantic Type element rule_id {rule_id:?} does not own an exact Type element query"
        )),
    }
}

fn type_fact_collection_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<TypeDerivedFactCollection, String> {
    let manifest = manifest
        .ok_or_else(|| format!("{fixture}: semantic Type fact requires a loaded manifest"))?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic Type fact rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::TypeDerivedFact(kind)) => Ok(type_fact_collection(kind)),
        _ => Err(format!(
            "{fixture}: semantic Type fact rule_id {rule_id:?} does not own an exact Type fact query"
        )),
    }
}

fn action_fact_collection_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<ActionDerivedFactCollection, String> {
    let manifest = manifest
        .ok_or_else(|| format!("{fixture}: semantic Action fact requires a loaded manifest"))?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic Action fact rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::ActionDerivedFact(kind)) => Ok(action_fact_collection(kind)),
        _ => Err(format!(
            "{fixture}: semantic Action fact rule_id {rule_id:?} does not own an exact Action fact query"
        )),
    }
}

fn definition_usage_derived_kind_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<DefinitionUsageDerivedKind, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic Definition/Usage derivation requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic Definition/Usage derivation rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::DefinitionUsageDerived(kind)) => Ok(kind),
        _ => Err(format!(
            "{fixture}: semantic Definition/Usage derivation rule_id {rule_id:?} does not own an exact Definition/Usage query"
        )),
    }
}

fn requirement_derived_fact_collection_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<RequirementDerivedFactCollection, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic Requirements derivation requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic Requirements derivation rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::RequirementDerivedFact(kind)) => Ok(match kind {
            RequirementDerivedFactKind::DefinitionActorParameter => RequirementDerivedFactCollection::DefinitionActorParameter,
            RequirementDerivedFactKind::DefinitionSubjectParameter => RequirementDerivedFactCollection::DefinitionSubjectParameter,
            RequirementDerivedFactKind::DefinitionText => RequirementDerivedFactCollection::DefinitionText,
            RequirementDerivedFactKind::DefinitionRequiredConstraint => RequirementDerivedFactCollection::DefinitionRequiredConstraint,
            RequirementDerivedFactKind::DefinitionAssumedConstraint => RequirementDerivedFactCollection::DefinitionAssumedConstraint,
            RequirementDerivedFactKind::DefinitionFramedConcern => RequirementDerivedFactCollection::DefinitionFramedConcern,
            RequirementDerivedFactKind::UsageActorParameter => RequirementDerivedFactCollection::UsageActorParameter,
            RequirementDerivedFactKind::UsageSubjectParameter => RequirementDerivedFactCollection::UsageSubjectParameter,
            RequirementDerivedFactKind::UsageText => RequirementDerivedFactCollection::UsageText,
            RequirementDerivedFactKind::UsageRequiredConstraint => RequirementDerivedFactCollection::UsageRequiredConstraint,
            RequirementDerivedFactKind::UsageAssumedConstraint => RequirementDerivedFactCollection::UsageAssumedConstraint,
            RequirementDerivedFactKind::UsageFramedConcern => RequirementDerivedFactCollection::UsageFramedConcern,
        }),
        _ => Err(format!(
            "{fixture}: semantic Requirements derivation rule_id {rule_id:?} does not own an exact Requirements query"
        )),
    }
}

fn element_owner_kind_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<ElementDerivedOwnerKind, String> {
    let manifest = manifest
        .ok_or_else(|| format!("{fixture}: semantic element owner requires a loaded manifest"))?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic element owner rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::ElementDerivedOwner(kind)) => Ok(kind),
        _ => Err(format!(
            "{fixture}: semantic element owner rule_id {rule_id:?} does not own an exact Element owner query"
        )),
    }
}

fn element_documentation_collection_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<ElementDerivedDocumentationCollection, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic element documentation requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic element documentation rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::ElementDerivedDocumentation(kind)) => Ok(match kind {
            ElementDerivedDocumentationKind::Documentation => {
                ElementDerivedDocumentationCollection::Documentation
            }
            ElementDerivedDocumentationKind::TextualRepresentation => {
                ElementDerivedDocumentationCollection::TextualRepresentation
            }
        }),
        _ => Err(format!(
            "{fixture}: semantic element documentation rule_id {rule_id:?} does not own an exact Element documentation query"
        )),
    }
}

fn namespace_element_collection_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<NamespaceDerivedElementCollection, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic Namespace element query requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!(
            "{fixture}: semantic Namespace element rule_id {rule_id:?} is absent from the manifest"
        )
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::NamespaceDerivedElement(kind)) => Ok(match kind {
            NamespaceDerivedElementKind::OwnedMember => {
                NamespaceDerivedElementCollection::OwnedMember
            }
            NamespaceDerivedElementKind::OwnedImport => {
                NamespaceDerivedElementCollection::OwnedImport
            }
        }),
        _ => Err(format!(
            "{fixture}: semantic Namespace element rule_id {rule_id:?} does not own an exact Namespace element query"
        )),
    }
}

fn namespace_import_element_kind_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<NamespaceImportDerivedElementKind, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic NamespaceImport element query requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic NamespaceImport element rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::NamespaceImportDerivedElement(kind)) => Ok(kind),
        _ => Err(format!(
            "{fixture}: semantic NamespaceImport element rule_id {rule_id:?} does not own an exact NamespaceImport element query"
        )),
    }
}

fn binding_connector_check_kind_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<BindingConnectorCheckKind, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic BindingConnector check requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic BindingConnector check rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::BindingConnectorCheck(kind)) => Ok(kind),
        _ => Err(format!(
            "{fixture}: semantic BindingConnector check rule_id {rule_id:?} does not own an exact BindingConnector query"
        )),
    }
}

fn redefinition_check_kind_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<RedefinitionCheckKind, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic redefinition check requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic redefinition check rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::RedefinitionCheck(kind)) => Ok(kind),
        _ => Err(format!(
            "{fixture}: semantic redefinition check rule_id {rule_id:?} does not own an exact redefinition query"
        )),
    }
}

fn specialization_check_kind_for_rule(
    manifest: Option<&ConstraintManifest>,
    rule_id: &str,
    fixture: &str,
) -> Result<SpecializationCheckKind, String> {
    let manifest = manifest.ok_or_else(|| {
        format!("{fixture}: semantic specialization check requires a loaded manifest")
    })?;
    let entry = manifest.find_rule(rule_id).ok_or_else(|| {
        format!("{fixture}: semantic specialization check rule_id {rule_id:?} is absent from the manifest")
    })?;
    match entry.query_family() {
        Some(ConstraintQueryFamily::SpecializationCheck(kind)) => Ok(kind),
        _ => Err(format!(
            "{fixture}: semantic specialization check rule_id {rule_id:?} does not own an exact specialization query"
        )),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypeDerivedRelationshipExpectation {
    collection: TypeDerivedRelationshipCollection,
    source: String,
    kind: SemanticRelationshipKind,
    target: Option<String>,
    provenance: Option<RelationshipProvenance>,
    outcome: SemanticRelationshipOutcome,
}

/// One asserted member of a closed Type element-valued collection. The selector comes from the
/// manifest, and the runner observes the opaque result only through `sysml_query`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TypeDerivedElementExpectation {
    collection: TypeDerivedElementCollection,
    source: String,
    target: Option<String>,
    outcome: TypeDerivedElementOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypeDerivedFactExpectation {
    collection: TypeDerivedFactCollection,
    source: String,
    /// A member/original-Type endpoint when the exact normative result is addressable in source.
    target: Option<String>,
    outcome: TypeDerivedElementOutcome,
}

/// A desired Systems::Actions fact selected by a manifest-owned closed collection. Action
/// arguments and parameters are often anonymous, so their canonical one-based position can be
/// asserted without inventing a declaration identity.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ActionDerivedFactExpectation {
    collection: ActionDerivedFactCollection,
    source: String,
    target: Option<String>,
    position: Option<u32>,
    outcome: TypeDerivedElementOutcome,
}

/// An exact Definition/Usage derivation observed solely through the manifest-selected public
/// query. Element targets retain canonical identities; scalar and unavailable-fact outcomes do
/// not invent a synthetic element endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DefinitionUsageDerivedExpectation {
    kind: DefinitionUsageDerivedKind,
    source: String,
    target: Option<String>,
    outcome: DefinitionUsageDerivedExpectationOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DefinitionUsageDerivedExpectationOutcome {
    Resolved,
    Absent,
    True,
    False,
    Incomplete,
    Unsupported(DefinitionUsageDerivedPrerequisite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RequirementDerivedFactExpectation {
    collection: RequirementDerivedFactCollection,
    source: String,
    target: Option<String>,
    text: Option<String>,
    outcome: RequirementDerivedFactExpectationOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RequirementDerivedFactExpectationOutcome {
    Resolved,
    Absent,
    Text,
    Incomplete,
    Unsupported(RequirementDerivedFactPrerequisite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ElementDerivedOwnerExpectation {
    kind: ElementDerivedOwnerKind,
    source: String,
    owner: Option<String>,
    outcome: ElementDerivedOwnerOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ElementDerivedDocumentationExpectation {
    collection: ElementDerivedDocumentationCollection,
    source: String,
    expected: Option<ExpectedDocumentation>,
    outcome: ElementDerivedDocumentationOutcome,
}

/// One asserted member of an exact Namespace element-valued collection. The selected collection
/// comes only from the manifest-owned query family; this fixture syntax has no rule-name map.
#[derive(Debug, Clone, PartialEq, Eq)]
struct NamespaceDerivedElementExpectation {
    collection: NamespaceDerivedElementCollection,
    source: String,
    target: Option<String>,
    outcome: NamespaceDerivedElementOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NamespaceImportDerivedElementExpectation {
    kind: NamespaceImportDerivedElementKind,
    owner: String,
    target: Option<String>,
    provenance: Option<RelationshipProvenance>,
    outcome: SemanticRelationshipOutcome,
}

/// An exact named BindingConnector check selected by the manifest-owned query family.
///
/// There is intentionally no fixture-authored connector or endpoint interpretation here: the
/// opaque query returns the resolver-owned rule-scoped result over canonical paired facts.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BindingConnectorCheckExpectation {
    rule: BindingConnectorCheckKind,
    outcome: BindingConnectorCheckOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BindingConnectorCheckOutcome {
    Satisfied,
    Violated,
    Unresolved,
    Unsupported(BindingConnectorValidationPrerequisite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BindingConnectorCheckObservation {
    Outcome(BindingConnectorValidationOutcome),
    Incomplete,
}

/// An exact named redefinition check selected by the manifest-owned query family.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RedefinitionCheckExpectation {
    rule: RedefinitionCheckKind,
    outcome: RedefinitionCheckExpectationOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RedefinitionCheckExpectationOutcome {
    Satisfied,
    Violated,
    Unresolved,
    Unsupported(RedefinitionCheckPrerequisite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RedefinitionCheckObservation {
    Outcome(RedefinitionCheckOutcome),
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpecializationCheckExpectation {
    rule: SpecializationCheckKind,
    outcome: SpecializationCheckExpectationOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SpecializationCheckExpectationOutcome {
    Satisfied,
    Violated,
    Unresolved,
    Unsupported(SpecializationCheckPrerequisite),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SpecializationCheckObservation {
    Outcome(SpecializationCheckOutcome),
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExpectedDocumentation {
    form: AnnotationForm,
    locale: Option<String>,
    language: Option<String>,
    text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ElementDerivedDocumentationOutcome {
    Resolved,
    Absent,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NamespaceDerivedElementOutcome {
    Resolved,
    Absent,
    Incomplete,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeDerivedElementOutcome {
    Resolved,
    Absent,
    Incomplete,
    Unsupported,
}

fn parse_binding_connector_prerequisite(
    value: &str,
    fixture: &str,
) -> Result<BindingConnectorValidationPrerequisite, String> {
    match value {
        "feature_reference_expression_target_and_result" => {
            Ok(BindingConnectorValidationPrerequisite::FeatureReferenceExpressionTargetAndResult)
        }
        "feature_value_endpoint_facts" => {
            Ok(BindingConnectorValidationPrerequisite::FeatureValueEndpointFacts)
        }
        "expression_result_endpoint_facts" => {
            Ok(BindingConnectorValidationPrerequisite::ExpressionResultEndpointFacts)
        }
        "function_result_endpoint_facts" => {
            Ok(BindingConnectorValidationPrerequisite::FunctionResultEndpointFacts)
        }
        "invocation_expression_behavior_endpoint_facts" => {
            Ok(BindingConnectorValidationPrerequisite::InvocationExpressionBehaviorEndpointFacts)
        }
        "accept_action_usage_receiver_endpoint_facts" => {
            Ok(BindingConnectorValidationPrerequisite::AcceptActionUsageReceiverEndpointFacts)
        }
        "transition_usage_source_endpoint_facts" => {
            Ok(BindingConnectorValidationPrerequisite::TransitionUsageSourceEndpointFacts)
        }
        "transition_usage_succession_endpoint_facts" => {
            Ok(BindingConnectorValidationPrerequisite::TransitionUsageSuccessionEndpointFacts)
        }
        "satisfy_requirement_usage_endpoint_facts" => {
            Ok(BindingConnectorValidationPrerequisite::SatisfyRequirementUsageEndpointFacts)
        }
        "normative_specification_tbd" => {
            Ok(BindingConnectorValidationPrerequisite::NormativeSpecificationTbd)
        }
        "rule_not_published" => Ok(BindingConnectorValidationPrerequisite::RuleNotPublished),
        _ => Err(format!(
            "{fixture}: unknown BindingConnector check prerequisite {value:?}"
        )),
    }
}

fn parse_binding_connector_check_outcome(
    outcome: &str,
    prerequisite: Option<&String>,
    fixture: &str,
) -> Result<BindingConnectorCheckOutcome, String> {
    match outcome {
        "satisfied" => {
            if prerequisite.is_some() {
                return Err(format!(
                    "{fixture}: satisfied BindingConnector check must not declare prerequisite"
                ));
            }
            Ok(BindingConnectorCheckOutcome::Satisfied)
        }
        "violated" => {
            if prerequisite.is_some() {
                return Err(format!(
                    "{fixture}: violated BindingConnector check must not declare prerequisite"
                ));
            }
            Ok(BindingConnectorCheckOutcome::Violated)
        }
        "unresolved" => {
            if prerequisite.is_some() {
                return Err(format!(
                    "{fixture}: unresolved BindingConnector check must not declare prerequisite"
                ));
            }
            Ok(BindingConnectorCheckOutcome::Unresolved)
        }
        "unsupported" => Ok(BindingConnectorCheckOutcome::Unsupported(
            parse_binding_connector_prerequisite(
                prerequisite.ok_or_else(|| {
                    format!(
                        "{fixture}: unsupported BindingConnector check requires prerequisite"
                    )
                })?,
                fixture,
            )?,
        )),
        _ => Err(format!(
            "{fixture}: unknown BindingConnector check outcome {outcome:?} (expected satisfied, violated, unresolved, or unsupported)"
        )),
    }
}

fn parse_redefinition_check_prerequisite(
    value: &str,
    fixture: &str,
) -> Result<RedefinitionCheckPrerequisite, String> {
    match value {
        "end_feature_position_and_inherited_ends" => {
            Ok(RedefinitionCheckPrerequisite::EndFeaturePositionAndInheritedEnds)
        }
        "flow_end_ordinal_and_library_anchors" => {
            Ok(RedefinitionCheckPrerequisite::FlowEndOrdinalAndLibraryAnchors)
        }
        "cross_feature_and_subsetting_endpoints" => {
            Ok(RedefinitionCheckPrerequisite::CrossFeatureAndSubsettingEndpoints)
        }
        "parameter_direction_and_inherited_position" => {
            Ok(RedefinitionCheckPrerequisite::ParameterDirectionAndInheritedPosition)
        }
        "function_or_expression_result" => {
            Ok(RedefinitionCheckPrerequisite::FunctionOrExpressionResult)
        }
        "constructor_result_and_instantiated_type_features" => {
            Ok(RedefinitionCheckPrerequisite::ConstructorResultAndInstantiatedTypeFeatures)
        }
        "feature_chain_source_target" => {
            Ok(RedefinitionCheckPrerequisite::FeatureChainSourceTarget)
        }
        "feature_chain_source_target_and_library_anchor" => {
            Ok(RedefinitionCheckPrerequisite::FeatureChainSourceTargetAndLibraryAnchor)
        }
        "state_subaction_membership_and_kind" => {
            Ok(RedefinitionCheckPrerequisite::StateSubactionMembershipAndKind)
        }
        "assignment_action_input_parameter_endpoints" => {
            Ok(RedefinitionCheckPrerequisite::AssignmentActionInputParameterEndpoints)
        }
        "for_loop_variable_projection" => {
            Ok(RedefinitionCheckPrerequisite::ForLoopVariableProjection)
        }
        "objective_membership_and_case_objective" => {
            Ok(RedefinitionCheckPrerequisite::ObjectiveMembershipAndCaseObjective)
        }
        "view_rendering_membership" => Ok(RedefinitionCheckPrerequisite::ViewRenderingMembership),
        "rule_not_published" => Ok(RedefinitionCheckPrerequisite::RuleNotPublished),
        _ => Err(format!(
            "{fixture}: unknown redefinition check prerequisite {value:?}"
        )),
    }
}

fn parse_specialization_check_prerequisite(
    value: &str,
    fixture: &str,
) -> Result<SpecializationCheckPrerequisite, String> {
    match value {
        "owned_cross_feature_owner_types" => {
            Ok(SpecializationCheckPrerequisite::OwnedCrossFeatureOwnerTypes)
        }
        "feature_modifiers_owner_typing_and_library_anchor" => {
            Ok(SpecializationCheckPrerequisite::FeatureModifiersOwnerTypingAndLibraryAnchor)
        }
        "feature_value_evaluation_results" => {
            Ok(SpecializationCheckPrerequisite::FeatureValueEvaluationResults)
        }
        "semantic_metadata_projection" => {
            Ok(SpecializationCheckPrerequisite::SemanticMetadataProjection)
        }
        "connector_association_projection_and_library_anchor" => {
            Ok(SpecializationCheckPrerequisite::ConnectorAssociationProjectionAndLibraryAnchor)
        }
        "step_ownership_typing_and_library_anchor" => {
            Ok(SpecializationCheckPrerequisite::StepOwnershipTypingAndLibraryAnchor)
        }
        "expression_argument_result" => {
            Ok(SpecializationCheckPrerequisite::ExpressionArgumentResult)
        }
        "expression_result_and_instantiated_type" => {
            Ok(SpecializationCheckPrerequisite::ExpressionResultAndInstantiatedType)
        }
        "library_anchor_and_implied_specialization" => {
            Ok(SpecializationCheckPrerequisite::LibraryAnchorAndImpliedSpecialization)
        }
        "feature_chain_source_target_and_subsetting" => {
            Ok(SpecializationCheckPrerequisite::FeatureChainSourceTargetAndSubsetting)
        }
        "feature_reference_referent_and_result" => {
            Ok(SpecializationCheckPrerequisite::FeatureReferenceReferentAndResult)
        }
        "invocation_instantiated_type_and_result" => {
            Ok(SpecializationCheckPrerequisite::InvocationInstantiatedTypeAndResult)
        }
        "invocation_instantiated_type" => {
            Ok(SpecializationCheckPrerequisite::InvocationInstantiatedType)
        }
        "succession_endpoint_and_subsetting" => {
            Ok(SpecializationCheckPrerequisite::SuccessionEndpointAndSubsetting)
        }
        "state_subaction_kind_and_library_anchor" => {
            Ok(SpecializationCheckPrerequisite::StateSubactionKindAndLibraryAnchor)
        }
        "transition_owner_source_and_library_anchor" => {
            Ok(SpecializationCheckPrerequisite::TransitionOwnerSourceAndLibraryAnchor)
        }
        "transition_trigger_payload_endpoints" => {
            Ok(SpecializationCheckPrerequisite::TransitionTriggerPayloadEndpoints)
        }
        "transition_succession_source" => {
            Ok(SpecializationCheckPrerequisite::TransitionSuccessionSource)
        }
        "transition_feature_roles_and_library_anchors" => {
            Ok(SpecializationCheckPrerequisite::TransitionFeatureRolesAndLibraryAnchors)
        }
        "use_case_owner_and_library_anchor" => {
            Ok(SpecializationCheckPrerequisite::UseCaseOwnerAndLibraryAnchor)
        }
        "usage_variation_owner" => Ok(SpecializationCheckPrerequisite::UsageVariationOwner),
        "individual_multiplicity_and_library_anchor" => {
            Ok(SpecializationCheckPrerequisite::IndividualMultiplicityAndLibraryAnchor)
        }
        "occurrence_owner_typing_and_library_anchor" => {
            Ok(SpecializationCheckPrerequisite::OccurrenceOwnerTypingAndLibraryAnchor)
        }
        "rule_not_published" => Ok(SpecializationCheckPrerequisite::RuleNotPublished),
        _ => Err(format!(
            "{fixture}: unknown specialization check prerequisite {value:?}"
        )),
    }
}

fn parse_specialization_check_outcome(
    outcome: &str,
    prerequisite: Option<&String>,
    fixture: &str,
) -> Result<SpecializationCheckExpectationOutcome, String> {
    match outcome {
        "satisfied" if prerequisite.is_none() => Ok(SpecializationCheckExpectationOutcome::Satisfied),
        "violated" if prerequisite.is_none() => Ok(SpecializationCheckExpectationOutcome::Violated),
        "unresolved" if prerequisite.is_none() => Ok(SpecializationCheckExpectationOutcome::Unresolved),
        "unsupported" => Ok(SpecializationCheckExpectationOutcome::Unsupported(
            parse_specialization_check_prerequisite(
                prerequisite.ok_or_else(|| format!("{fixture}: unsupported specialization check requires prerequisite"))?,
                fixture,
            )?,
        )),
        "satisfied" | "violated" | "unresolved" => Err(format!(
            "{fixture}: {outcome} specialization check must not declare prerequisite"
        )),
        _ => Err(format!(
            "{fixture}: unknown specialization check outcome {outcome:?} (expected satisfied, violated, unresolved, or unsupported)"
        )),
    }
}

fn parse_redefinition_check_outcome(
    outcome: &str,
    prerequisite: Option<&String>,
    fixture: &str,
) -> Result<RedefinitionCheckExpectationOutcome, String> {
    match outcome {
        "satisfied" if prerequisite.is_none() => Ok(RedefinitionCheckExpectationOutcome::Satisfied),
        "violated" if prerequisite.is_none() => Ok(RedefinitionCheckExpectationOutcome::Violated),
        "unresolved" if prerequisite.is_none() => Ok(RedefinitionCheckExpectationOutcome::Unresolved),
        "unsupported" => Ok(RedefinitionCheckExpectationOutcome::Unsupported(
            parse_redefinition_check_prerequisite(
                prerequisite.ok_or_else(|| format!("{fixture}: unsupported redefinition check requires prerequisite"))?,
                fixture,
            )?,
        )),
        "satisfied" | "violated" | "unresolved" => Err(format!(
            "{fixture}: {outcome} redefinition check must not declare prerequisite"
        )),
        _ => Err(format!(
            "{fixture}: unknown redefinition check outcome {outcome:?} (expected satisfied, violated, unresolved, or unsupported)"
        )),
    }
}

impl NamespaceDerivedElementOutcome {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "resolved" => Ok(Self::Resolved),
            "absent" => Ok(Self::Absent),
            "incomplete" => Ok(Self::Incomplete),
            "unsupported" => Ok(Self::Unsupported),
            _ => Err(format!(
                "{fixture}: unknown semantic Namespace element outcome {value:?} (expected resolved, absent, incomplete, or unsupported)"
            )),
        }
    }
}

impl TypeDerivedElementOutcome {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "resolved" => Ok(Self::Resolved),
            "absent" => Ok(Self::Absent),
            "incomplete" => Ok(Self::Incomplete),
            "unsupported" => Ok(Self::Unsupported),
            _ => Err(format!(
                "{fixture}: unknown semantic Type element outcome {value:?} (expected resolved, absent, incomplete, or unsupported)"
            )),
        }
    }
}

fn parse_definition_usage_prerequisite(
    value: &str,
    fixture: &str,
) -> Result<DefinitionUsageDerivedPrerequisite, String> {
    match value {
        "effective_feature_membership_closure" => {
            Ok(DefinitionUsageDerivedPrerequisite::EffectiveFeatureMembershipClosure)
        }
        "rule_not_published" => Ok(DefinitionUsageDerivedPrerequisite::RuleNotPublished),
        _ => Err(format!(
            "{fixture}: unknown Definition/Usage derivation prerequisite {value:?}"
        )),
    }
}

fn parse_definition_usage_outcome(
    outcome: &str,
    prerequisite: Option<&String>,
    fixture: &str,
) -> Result<DefinitionUsageDerivedExpectationOutcome, String> {
    match outcome {
        "resolved" => Ok(DefinitionUsageDerivedExpectationOutcome::Resolved),
        "absent" => Ok(DefinitionUsageDerivedExpectationOutcome::Absent),
        "true" => Ok(DefinitionUsageDerivedExpectationOutcome::True),
        "false" => Ok(DefinitionUsageDerivedExpectationOutcome::False),
        "incomplete" => Ok(DefinitionUsageDerivedExpectationOutcome::Incomplete),
        "unsupported" => Ok(DefinitionUsageDerivedExpectationOutcome::Unsupported(
            parse_definition_usage_prerequisite(
                prerequisite.ok_or_else(|| {
                    format!(
                        "{fixture}: unsupported Definition/Usage derivation requires prerequisite"
                    )
                })?,
                fixture,
            )?,
        )),
        _ => Err(format!(
            "{fixture}: unknown Definition/Usage derivation outcome {outcome:?}"
        )),
    }
}

fn parse_requirement_derived_prerequisite(
    value: &str,
    fixture: &str,
) -> Result<RequirementDerivedFactPrerequisite, String> {
    match value {
        "rule_not_published" => Ok(RequirementDerivedFactPrerequisite::RuleNotPublished),
        "canonical_membership_role" => {
            Ok(RequirementDerivedFactPrerequisite::CanonicalMembershipRole)
        }
        "documentation_records" => Ok(RequirementDerivedFactPrerequisite::DocumentationRecords),
        _ => Err(format!(
            "{fixture}: unknown Requirements derivation prerequisite {value:?}"
        )),
    }
}

fn parse_requirement_derived_outcome(
    outcome: &str,
    prerequisite: Option<&String>,
    fixture: &str,
) -> Result<RequirementDerivedFactExpectationOutcome, String> {
    match outcome {
        "resolved" => Ok(RequirementDerivedFactExpectationOutcome::Resolved),
        "absent" => Ok(RequirementDerivedFactExpectationOutcome::Absent),
        "text" => Ok(RequirementDerivedFactExpectationOutcome::Text),
        "incomplete" => Ok(RequirementDerivedFactExpectationOutcome::Incomplete),
        "unsupported" => Ok(RequirementDerivedFactExpectationOutcome::Unsupported(
            parse_requirement_derived_prerequisite(
                prerequisite.ok_or_else(|| {
                    format!("{fixture}: unsupported Requirements derivation requires prerequisite")
                })?,
                fixture,
            )?,
        )),
        _ => Err(format!(
            "{fixture}: unknown Requirements derivation outcome {outcome:?}"
        )),
    }
}

impl ElementDerivedDocumentationOutcome {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "resolved" => Ok(Self::Resolved),
            "absent" => Ok(Self::Absent),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err(format!(
                "{fixture}: unknown semantic element documentation outcome {value:?} (expected resolved, absent, or incomplete)"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ElementDerivedOwnerOutcome {
    Resolved,
    Absent,
    Incomplete,
}

impl ElementDerivedOwnerOutcome {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "resolved" => Ok(Self::Resolved),
            "absent" => Ok(Self::Absent),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err(format!(
                "{fixture}: unknown semantic element-owner outcome {value:?} (expected resolved, absent, or incomplete)"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SemanticRelationshipOutcome {
    Resolved,
    Unresolved,
    Ambiguous,
    Unsupported,
    Absent,
    Incomplete,
}

impl SemanticRelationshipOutcome {
    fn parse(value: &str, fixture: &str) -> Result<Self, String> {
        match value {
            "resolved" => Ok(Self::Resolved),
            "unresolved" => Ok(Self::Unresolved),
            "ambiguous" => Ok(Self::Ambiguous),
            "unsupported" => Ok(Self::Unsupported),
            "absent" => Ok(Self::Absent),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err(format!(
                "{fixture}: unknown semantic relationship outcome {value:?} (expected resolved, unresolved, ambiguous, unsupported, absent, or incomplete)"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SemanticRelationshipObservation {
    Relationship {
        source: SymbolId,
        kind: SemanticRelationshipKind,
        provenance: RelationshipProvenance,
        target: RelationshipTarget,
        expected_target: Option<SymbolId>,
    },
    Absent {
        source: SymbolId,
        kind: SemanticRelationshipKind,
        provenance: RelationshipProvenance,
    },
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SemanticIdentityStatus {
    Unresolved,
    Ambiguous,
    WrongKind,
    Unsupported,
    Recovery,
    Incomplete,
}

impl SemanticIdentityStatus {
    fn description(self) -> &'static str {
        match self {
            Self::Unresolved => "unresolved",
            Self::Ambiguous => "ambiguous",
            Self::WrongKind => "wrong-kind",
            Self::Unsupported => "unsupported",
            Self::Recovery => "recovery",
            Self::Incomplete => "incomplete",
        }
    }
}

/// Stable remediation categories for open normative expectations. New variants require a
/// materially distinct owner and completion condition; there is intentionally no catch-all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum IssueKind {
    ParserGap,
    LoweringGap,
    SemanticNotImplemented,
    DiagnosticNotImplemented,
    SemanticQueryGap,
    LibraryGap,
    AbstractSyntaxCoverageGap,
    /// The pinned normative artifact deliberately supplies no evaluable body (`TBD`).
    /// Completion belongs to the OMG specification, not parser or semantic implementation work.
    NormativeSpecificationGap,
}

/// Closed ownership set for the issue registry. The value records the system that must supply the
/// missing fact or behavior, rather than the snapshot fixture that observed it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum IssueOwner {
    SysmlV2Parser,
    Spec42Semantic,
    Spec42Diagnostics,
    Spec42Query,
    Spec42Libraries,
    Spec42Snapshot,
    OmgSpecification,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct IssueEntry {
    id: String,
    kind: IssueKind,
    owner: IssueOwner,
    summary: String,
    tracking: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct IssueRegistryFile {
    schema_version: u32,
    #[serde(default)]
    issue: Vec<IssueEntry>,
}

#[derive(Debug)]
struct IssueRegistry {
    issues: BTreeMap<String, IssueEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ExpectationState {
    NotApplicable,
    /// A fixture in the validation corpus without a complete normative contract.  This is kept
    /// distinct from ordinary, non-normative snapshots so migration debt cannot look exercised.
    Unclassified,
    Passed,
    Blocked,
    Stale,
    Failed,
}

#[derive(Debug, Serialize)]
struct FixtureReport {
    path: String,
    rule_ids: Vec<String>,
    source_expectation: Option<SourceExpectation>,
    rule_family: Option<RuleFamily>,
    expectation: Option<ExpectationKind>,
    state: ExpectationState,
    blocked_by: Option<ReportBlocker>,
    by_construction_evidence: Option<ByConstructionEvidenceStatus>,
    diagnostics: Vec<ObservedDiagnostic>,
}

/// The report deliberately distinguishes an executable construction proof from a rule whose
/// invalid abstract shape cannot currently be authored.  A blocked abstract case is coverage
/// debt, not executable evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ByConstructionEvidenceStatus {
    Executable,
    AbstractSyntaxCoverageGap,
    Missing,
}

#[derive(Debug, Clone, Serialize)]
struct ReportBlocker {
    id: String,
    kind: IssueKind,
    owner: IssueOwner,
    summary: String,
}

#[derive(Debug, Clone, Serialize)]
struct ObservedDiagnostic {
    category: String,
    origin: String,
    severity: String,
}

#[derive(Debug, Serialize)]
struct ProcessingFailure {
    path: String,
    error: String,
}

#[derive(Debug, Serialize)]
struct SnapshotReport {
    schema_version: u32,
    fixtures: Vec<FixtureReport>,
    aggregate: AggregateReport,
    processing_failures: Vec<ProcessingFailure>,
    stale_generated_snapshots: Vec<String>,
    manifest_audit: Option<ManifestAuditOutcome>,
}

#[derive(Debug, Serialize)]
struct AggregateReport {
    selected_fixtures: usize,
    successfully_evaluated_fixtures: usize,
    processing_failures: usize,
    unclassified_fixtures: usize,
    stale_generated_snapshots: usize,
    expectations: BTreeMap<String, usize>,
    normative_coverage: BTreeMap<String, NormativeCoverage>,
    outstanding_issues: Vec<IssueImpact>,
    observed_diagnostics: BTreeMap<String, DiagnosticAggregate>,
}

#[derive(Debug, Default, Serialize)]
struct NormativeCoverage {
    fixture_count: usize,
    unique_rule_count: usize,
    diagnostics_fixture_count: usize,
    semantics_fixture_count: usize,
    by_construction_fixture_count: usize,
    by_construction_executable_evidence_fixture_count: usize,
    by_construction_abstract_coverage_gap_fixture_count: usize,
    by_construction_missing_evidence_fixture_count: usize,
    passed_fixture_count: usize,
    blocked_fixture_count: usize,
    stale_fixture_count: usize,
    failed_fixture_count: usize,
    not_applicable_fixture_count: usize,
}

#[derive(Debug, Serialize)]
struct IssueImpact {
    id: String,
    kind: IssueKind,
    owner: IssueOwner,
    summary: String,
    affected_fixture_count: usize,
    affected_rule_count: usize,
    fixtures: Vec<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct DiagnosticAggregate {
    occurrences: usize,
    affected_fixture_count: usize,
    origins: BTreeMap<String, DiagnosticOriginAggregate>,
}

/// A diagnostic's origin and severity jointly describe its production path.  Keeping this
/// nested prevents a category-level severity total from implying a severity for every origin.
#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct DiagnosticOriginAggregate {
    occurrences: usize,
    affected_fixture_count: usize,
    severities: BTreeMap<String, usize>,
}

#[derive(Debug, Serialize)]
struct ManifestAuditReport {
    manifest_rule_occurrences: usize,
    manifest_unique_rule_ids: usize,
    manifest_rule_occurrences_by_family: BTreeMap<String, usize>,
    fixture_rule_occurrences: usize,
    fixture_unique_rule_ids: usize,
    selected_fixture_rule_occurrences: usize,
    selected_fixture_unique_rule_ids: usize,
    missing_rule_ids: Vec<String>,
    missing_rule_ids_by_family: BTreeMap<String, usize>,
    duplicate_primary_rule_ids: BTreeMap<String, Vec<String>>,
    orphan_secondary_rule_ids: BTreeMap<String, Vec<String>>,
    unknown_rule_ids: BTreeMap<String, Vec<String>>,
    family_mismatches: BTreeMap<String, Vec<String>>,
    clause_mismatches: BTreeMap<String, Vec<String>>,
    constraint_mismatches: BTreeMap<String, Vec<String>>,
    specification_mismatches: BTreeMap<String, Vec<String>>,
    formal_document_mismatches: BTreeMap<String, Vec<String>>,
}

/// Report commands must publish evaluated fixture state even when the manifest itself cannot be
/// loaded or audited.  This makes manifest failure explicit instead of discarding independent
/// fixture evidence.
#[derive(Debug, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
enum ManifestAuditOutcome {
    Complete { audit: Box<ManifestAuditReport> },
    Failed { error: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManifestAuditHealth {
    Clean,
    CoverageDebt,
    Failed,
}

impl ManifestAuditOutcome {
    fn health(&self) -> ManifestAuditHealth {
        match self {
            Self::Complete { audit } if audit.is_clean() => ManifestAuditHealth::Clean,
            Self::Complete { .. } => ManifestAuditHealth::CoverageDebt,
            Self::Failed { .. } => ManifestAuditHealth::Failed,
        }
    }
}

fn audit_manifest_coverage(
    manifest: &ConstraintManifest,
    paths: &[PathBuf],
    selected_paths: &[PathBuf],
) -> Result<ManifestAuditReport, String> {
    let mut manifest_rule_ids = BTreeSet::new();
    let mut manifest_rule_occurrences = 0;
    let mut manifest_rule_occurrences_by_family = BTreeMap::new();
    for specification in &manifest.specifications {
        for entry in &specification.constraints {
            manifest_rule_occurrences += 1;
            *manifest_rule_occurrences_by_family
                .entry(manifest_family_name(entry.family).to_string())
                .or_insert(0) += 1;
            if !manifest_rule_ids.insert(entry.rule_id.clone()) {
                return Err(format!(
                    "constraint manifest contains duplicate rule_id {:?}",
                    entry.rule_id
                ));
            }
        }
    }

    let mut fixture_rule_occurrences = 0;
    let mut fixture_rule_paths: BTreeMap<String, Vec<(String, CoverageRole)>> = BTreeMap::new();
    let mut unknown_rule_ids = BTreeMap::new();
    let mut family_mismatches = BTreeMap::new();
    let mut clause_mismatches = BTreeMap::new();
    let mut constraint_mismatches = BTreeMap::new();
    let mut specification_mismatches = BTreeMap::new();
    let mut formal_document_mismatches = BTreeMap::new();
    for path in paths {
        let fixture = fs::read_to_string(path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        let fallback_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("snapshot.md");
        let meta = parse_fixture_meta(&fixture, fallback_name)?;
        if is_validation_fixture(path) && meta.normative_expectation.is_none() {
            return Err(format!(
                "{}: unclassified validation fixture: complete normative META is required",
                path.display()
            ));
        }
        let Some(expectation) = meta.normative_expectation else {
            continue;
        };
        for rule_id in expectation.rule_ids {
            fixture_rule_occurrences += 1;
            let path_text = normalized_report_path(path);
            fixture_rule_paths
                .entry(rule_id.clone())
                .or_default()
                .push((path_text.clone(), expectation.coverage_role));
            let Some(manifest_rule) = manifest.find_rule_with_specification(&rule_id) else {
                unknown_rule_ids
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(path_text);
                continue;
            };
            let entry = manifest_rule.entry;
            let manifest_specification = manifest_rule.specification;
            let Some(rule_specification) = manifest_rule.specification_id() else {
                specification_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} is owned by an unpinned manifest specification {} {}",
                        path.display(),
                        manifest_specification.name,
                        manifest_specification.version
                    ));
                continue;
            };
            let fixture_specification = expectation.specification_id.unwrap_or(rule_specification);
            if fixture_specification != rule_specification {
                specification_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} declares specification_id {:?}, but rule_id derives {:?}",
                        path.display(),
                        fixture_specification,
                        rule_specification
                    ));
            }
            let (expected_name, expected_version) = fixture_specification.name_version();
            if manifest_specification.name != expected_name
                || manifest_specification.version != expected_version
            {
                specification_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} expects {} {}, manifest declares {} {}",
                        path.display(),
                        expected_name,
                        expected_version,
                        manifest_specification.name,
                        manifest_specification.version
                    ));
            }
            if manifest_specification.formal_document_id
                != fixture_specification.formal_document_id()
            {
                formal_document_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} expects formal document {}, manifest declares {}",
                        path.display(),
                        fixture_specification.formal_document_id(),
                        manifest_specification.formal_document_id
                    ));
            }
            if !rule_family_matches_manifest(expectation.rule_family, entry.family) {
                family_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} declares {:?}, manifest declares {:?}",
                        path.display(),
                        expectation.rule_family,
                        entry.family
                    ));
            }
            match clause_from_rule_id(&rule_id) {
                Some(clause) if clause == entry.clause => {}
                Some(clause) => clause_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} declares clause {clause}, manifest declares {}",
                        path.display(),
                        entry.clause
                    )),
                None => clause_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} does not use <spec>-<version>:<clause>:<constraint>",
                        path.display()
                    )),
            }
            match constraint_from_rule_id(&rule_id) {
                Some(constraint) if constraint == entry.constraint => {}
                Some(constraint) => constraint_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} derives constraint {constraint}, manifest declares {}",
                        path.display(),
                        entry.constraint
                    )),
                None => constraint_mismatches
                    .entry(rule_id.clone())
                    .or_insert_with(Vec::new)
                    .push(format!(
                        "{} does not use <spec>-<version>:<clause>:<constraint>",
                        path.display()
                    )),
            }
        }
    }
    let fixture_unique_rule_ids = fixture_rule_paths.len();
    let duplicate_primary_rule_ids = fixture_rule_paths
        .iter()
        .filter_map(|(rule_id, paths)| {
            let primary_paths: Vec<_> = paths
                .iter()
                .filter(|(_, role)| *role == CoverageRole::Primary)
                .map(|(path, _)| path.clone())
                .collect();
            (primary_paths.len() > 1).then(|| (rule_id.clone(), primary_paths))
        })
        .collect();
    let orphan_secondary_rule_ids = fixture_rule_paths
        .iter()
        .filter_map(|(rule_id, paths)| {
            let secondary_paths: Vec<_> = paths
                .iter()
                .filter(|(_, role)| *role == CoverageRole::Secondary)
                .map(|(path, _)| path.clone())
                .collect();
            let has_primary = paths.iter().any(|(_, role)| *role == CoverageRole::Primary);
            (!has_primary && !secondary_paths.is_empty())
                .then(|| (rule_id.clone(), secondary_paths))
        })
        .collect();
    // A secondary case strengthens an existing proof; it cannot by itself satisfy the manifest's
    // requirement for a canonical evidence fixture.
    let covered_rule_ids: BTreeSet<_> = fixture_rule_paths
        .iter()
        .filter(|(_, paths)| paths.iter().any(|(_, role)| *role == CoverageRole::Primary))
        .map(|(rule_id, _)| rule_id.clone())
        .collect();
    let missing_rule_ids: Vec<_> = manifest_rule_ids
        .difference(&covered_rule_ids)
        .cloned()
        .collect();
    let mut missing_rule_ids_by_family = BTreeMap::new();
    for rule_id in &missing_rule_ids {
        if let Some(entry) = manifest.find_rule(rule_id) {
            *missing_rule_ids_by_family
                .entry(manifest_family_name(entry.family).to_string())
                .or_insert(0) += 1;
        }
    }
    let mut selected_rule_ids = BTreeSet::new();
    let mut selected_fixture_rule_occurrences = 0;
    for path in selected_paths {
        let fixture = fs::read_to_string(path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        let fallback_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("snapshot.md");
        if let Some(expectation) =
            parse_fixture_meta(&fixture, fallback_name)?.normative_expectation
        {
            selected_fixture_rule_occurrences += expectation.rule_ids.len();
            selected_rule_ids.extend(expectation.rule_ids);
        }
    }
    Ok(ManifestAuditReport {
        manifest_rule_occurrences,
        manifest_unique_rule_ids: manifest_rule_ids.len(),
        manifest_rule_occurrences_by_family,
        fixture_rule_occurrences,
        fixture_unique_rule_ids,
        selected_fixture_rule_occurrences,
        selected_fixture_unique_rule_ids: selected_rule_ids.len(),
        missing_rule_ids,
        missing_rule_ids_by_family,
        duplicate_primary_rule_ids,
        orphan_secondary_rule_ids,
        unknown_rule_ids,
        family_mismatches,
        clause_mismatches,
        constraint_mismatches,
        specification_mismatches,
        formal_document_mismatches,
    })
}

fn is_validation_fixture(path: &Path) -> bool {
    let components: Vec<_> = path.components().collect();
    components
        .windows(2)
        .any(|pair| pair[0].as_os_str() == "snapshots" && pair[1].as_os_str() == "validation")
}

fn manifest_family_name(family: ConstraintFamily) -> &'static str {
    match family {
        ConstraintFamily::Derive => "derive",
        ConstraintFamily::Check => "check",
        ConstraintFamily::Validate => "validate",
    }
}

fn rule_family_matches_manifest(fixture: RuleFamily, manifest: ConstraintFamily) -> bool {
    matches!(
        (fixture, manifest),
        (RuleFamily::Derive, ConstraintFamily::Derive)
            | (RuleFamily::Check, ConstraintFamily::Check)
            | (RuleFamily::Validate, ConstraintFamily::Validate)
    )
}

fn clause_from_rule_id(rule_id: &str) -> Option<&str> {
    rule_id
        .split_once(':')
        .and_then(|(_, remainder)| remainder.split_once(':'))
        .map(|(clause, _)| clause)
        .filter(|clause| clause.starts_with("8.3."))
}

fn constraint_from_rule_id(rule_id: &str) -> Option<&str> {
    rule_id
        .split_once(':')
        .and_then(|(_, remainder)| remainder.split_once(':'))
        .map(|(_, constraint)| constraint)
        .filter(|constraint| !constraint.is_empty())
}

impl ManifestAuditReport {
    fn is_clean(&self) -> bool {
        self.missing_rule_ids.is_empty()
            && self.duplicate_primary_rule_ids.is_empty()
            && self.orphan_secondary_rule_ids.is_empty()
            && self.unknown_rule_ids.is_empty()
            && self.family_mismatches.is_empty()
            && self.clause_mismatches.is_empty()
            && self.constraint_mismatches.is_empty()
            && self.specification_mismatches.is_empty()
            && self.formal_document_mismatches.is_empty()
    }
}

/// Complete in-memory output of a generator invocation. A sorted map makes artifact order part of
/// the snapshot contract rather than an accident of plugin emission order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct GeneratedArtifacts {
    files: BTreeMap<String, String>,
}

impl GeneratedArtifacts {
    fn insert_utf8(&mut self, path: impl Into<String>, contents: String) -> Result<(), String> {
        let path = path.into();
        validate_artifact_path(&path)?;
        if self.files.insert(path.clone(), contents).is_some() {
            return Err(format!(
                "generator emitted duplicate artifact path {path:?}"
            ));
        }
        Ok(())
    }
}

/// The directory of the checked-in standard-library corpus, relative to the snapshot root.
///
/// The runner deliberately admits only checked-in source fixtures rather than reaching into host
/// library packaging. Library fixtures carry the pinned library text in their own `SOURCE`
/// sections, so they are the admission input as well as fixtures in their own right.
const STANDARD_LIBRARY_DIRECTORY: &str = "sysml.library";

/// Lazily loaded library sources, shared by every fixture that admits them.
struct LibraryCorpus {
    root: PathBuf,
    services: sysml_query::Services,
    standard_documents: OnceLock<Result<Vec<AdmittedDocument>, String>>,
}

impl LibraryCorpus {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            services: sysml_query::Services::new(),
            standard_documents: OnceLock::new(),
        }
    }

    fn documents(&self, selection: LibrarySelection) -> Result<&[AdmittedDocument], String> {
        match selection {
            LibrarySelection::None => Ok(&[]),
            LibrarySelection::Standard => self
                .standard_documents
                .get_or_init(|| load_standard_library_documents(&self.root, &self.services.source))
                .as_deref()
                .map_err(Clone::clone),
        }
    }
}

fn load_standard_library_documents(
    root: &Path,
    source: &sysml_query::source::SourceService,
) -> Result<Vec<AdmittedDocument>, String> {
    let directory = root.join(STANDARD_LIBRARY_DIRECTORY);
    let mut paths = Vec::new();
    visit_markdown(&directory, &mut paths)?;
    paths.sort();
    if paths.is_empty() {
        return Err(format!(
            "no standard-library fixtures found under {}",
            directory.display()
        ));
    }
    let mut documents = Vec::new();
    for path in paths {
        let fallback_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("library.md");
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        for document in parse_source_documents(&text, fallback_name)? {
            let name = format!("{STANDARD_LIBRARY_DIRECTORY}/{}", document.name);
            documents.push(
                source
                    .admit_memory(
                        "snapshot",
                        &name,
                        document.text,
                        SourceKind::StandardLibrary,
                    )
                    .map_err(|error| error.to_string())?,
            );
        }
    }
    Ok(documents)
}

impl IssueRegistry {
    fn load(root: &Path) -> Result<Self, String> {
        let path = root.join("issues.toml");
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        let parsed: IssueRegistryFile = toml::from_str(&text)
            .map_err(|error| format!("{}: invalid issue registry: {error}", path.display()))?;
        if parsed.schema_version != 1 {
            return Err(format!(
                "{}: unsupported issue registry schema_version {} (expected 1)",
                path.display(),
                parsed.schema_version
            ));
        }
        let mut issues = BTreeMap::new();
        for issue in parsed.issue {
            validate_issue_owner(&issue)?;
            if !valid_issue_id(&issue.id) {
                return Err(format!(
                    "{}: issue id {:?} must contain only lowercase ASCII letters, digits, and hyphens",
                    path.display(),
                    issue.id
                ));
            }
            if issue.summary.trim().is_empty() {
                return Err(format!(
                    "{}: issue {:?} must have a non-empty summary",
                    path.display(),
                    issue.id
                ));
            }
            if issue
                .tracking
                .as_deref()
                .is_some_and(|tracking| tracking.trim().is_empty())
            {
                return Err(format!(
                    "{}: issue {:?} has an empty tracking reference",
                    path.display(),
                    issue.id
                ));
            }
            let id = issue.id.clone();
            if issues.insert(id.clone(), issue).is_some() {
                return Err(format!("{}: duplicate issue id {id:?}", path.display()));
            }
        }
        Ok(Self { issues })
    }

    fn blocker(&self, id: &str) -> Option<ReportBlocker> {
        self.issues.get(id).map(|issue| ReportBlocker {
            id: issue.id.clone(),
            kind: issue.kind,
            owner: issue.owner,
            summary: issue.summary.clone(),
        })
    }
}

fn validate_issue_owner(issue: &IssueEntry) -> Result<(), String> {
    let valid = matches!(
        (issue.kind, issue.owner),
        (IssueKind::ParserGap, IssueOwner::SysmlV2Parser)
            | (
                IssueKind::LoweringGap | IssueKind::SemanticNotImplemented,
                IssueOwner::Spec42Semantic
            )
            | (
                IssueKind::DiagnosticNotImplemented,
                IssueOwner::Spec42Diagnostics
            )
            | (IssueKind::SemanticQueryGap, IssueOwner::Spec42Query)
            | (IssueKind::LibraryGap, IssueOwner::Spec42Libraries)
            | (
                IssueKind::AbstractSyntaxCoverageGap,
                IssueOwner::Spec42Snapshot
            )
            | (
                IssueKind::NormativeSpecificationGap,
                IssueOwner::OmgSpecification
            )
    );
    valid.then_some(()).ok_or_else(|| {
        format!(
            "issue {:?}: kind {:?} is not owned by {:?}",
            issue.id, issue.kind, issue.owner
        )
    })
}

fn validate_blocker_contract(
    expectation: &NormativeExpectation,
    registry: &IssueRegistry,
    fixture: &str,
    has_supplemental_diagnostics: bool,
) -> Result<(), String> {
    let Some(id) = expectation.blocked_by.as_deref() else {
        return Ok(());
    };
    let Some(blocker) = registry.blocker(id) else {
        return Err(format!(
            "{fixture}: META blocked_by references unknown issue {id:?}"
        ));
    };
    let compatible = match blocker.kind {
        IssueKind::ParserGap => {
            expectation.source_expectation == SourceExpectation::Accepted
                && expectation.expectation != ExpectationKind::ByConstruction
        }
        IssueKind::LoweringGap | IssueKind::SemanticNotImplemented => {
            matches!(
                expectation.expectation,
                ExpectationKind::Diagnostics | ExpectationKind::Semantics
            )
        }
        IssueKind::DiagnosticNotImplemented => {
            expectation.expectation == ExpectationKind::Diagnostics || has_supplemental_diagnostics
        }
        IssueKind::SemanticQueryGap => expectation.expectation == ExpectationKind::Semantics,
        IssueKind::LibraryGap => matches!(
            expectation.expectation,
            ExpectationKind::Diagnostics | ExpectationKind::Semantics
        ),
        IssueKind::AbstractSyntaxCoverageGap => {
            expectation.expectation == ExpectationKind::ByConstruction
        }
        IssueKind::NormativeSpecificationGap => {
            expectation.expectation == ExpectationKind::Semantics
        }
    };
    compatible.then_some(()).ok_or_else(|| {
        format!(
            "{fixture}: issue {id:?} ({}) cannot block expectation={:?} with source_expectation={:?}",
            blocker.kind.as_str(), expectation.expectation, expectation.source_expectation
        )
    })
}

fn valid_issue_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn validate_registry_references(registry: &IssueRegistry, paths: &[PathBuf]) -> Result<(), String> {
    let mut referenced = BTreeSet::new();
    for path in paths {
        let fixture = fs::read_to_string(path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        let fallback_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("snapshot.md");
        let meta = parse_fixture_meta(&fixture, fallback_name)?;
        if is_validation_fixture(path) && meta.normative_expectation.is_none() {
            return Err(format!(
                "{}: unclassified validation fixture: complete normative META is required",
                path.display()
            ));
        }
        if let Some(id) = meta
            .normative_expectation
            .as_ref()
            .and_then(|expectation| expectation.blocked_by.as_deref())
        {
            if !registry.issues.contains_key(id) {
                return Err(format!(
                    "{}: META blocked_by references unknown issue {id:?}",
                    path.display()
                ));
            }
            referenced.insert(id.to_string());
        }
    }
    for id in registry.issues.keys() {
        if !referenced.contains(id) {
            return Err(format!(
                "issue registry entry {id:?} is not referenced by any snapshot fixture"
            ));
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let paths = snapshot_paths(&cli.root, cli.fixture.as_deref())?;
    if paths.is_empty() {
        return Err(format!(
            "no Markdown snapshots found under {}",
            cli.root.display()
        ));
    }
    let registry = IssueRegistry::load(&cli.root)?;
    let registry_paths = snapshot_paths(&cli.root, None)?;
    validate_registry_references(&registry, &registry_paths)?;
    let libraries = LibraryCorpus::new(cli.root.clone());
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("specifications/constraint_manifest.toml");
    // Report keeps fixture evidence visible when this fails. Semantic collection expectations
    // receive the absence explicitly and cannot fall back to a private rule table.
    let manifest = ConstraintManifest::load_toml(&manifest_path);
    let semantic_manifest = manifest.as_ref().ok();

    // Fixture isolation is parallel, but each fixture also builds sequential and parallel
    // semantic publications. Use an explicit worker stack rather than Rayon defaults so deep,
    // valid semantic graphs cannot make report behaviour depend on outer scheduling.
    let fixture_pool = rayon::ThreadPoolBuilder::new()
        .stack_size(16 * 1024 * 1024)
        .build()
        .map_err(|error| format!("snapshot fixture worker pool failed: {error}"))?;
    let mut results: Vec<_> = fixture_pool.install(|| {
        paths
            .par_iter()
            .map(|path| FixtureWorkResult {
                path: path.clone(),
                result: evaluate_fixture(path, &libraries, &registry, semantic_manifest),
            })
            .collect()
    });
    sort_work_results(&mut results);

    let mut expectation_failures = Vec::new();
    let mut processing_failures = Vec::new();
    let mut stale = Vec::new();
    let mut writes = Vec::new();
    let mut reports = Vec::new();
    for result in results {
        match result.result {
            Ok(FixtureOutcome {
                updated,
                report,
                expectation_failure,
            }) => {
                reports.push(report);
                if let Some(error) = expectation_failure {
                    expectation_failures.push((result.path, error));
                    continue;
                }
                if let Some(updated) = updated {
                    match &cli.command {
                        Command::Check => stale.push(result.path),
                        Command::Update => writes.push((result.path, updated.into_bytes())),
                        Command::Report { .. } => stale.push(result.path),
                    }
                }
            }
            Err(error) => processing_failures.push((result.path, error)),
        }
    }

    if let Command::Report { format } = cli.command {
        let manifest_audit = match manifest
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|manifest| audit_manifest_coverage(manifest, &registry_paths, &paths))
        {
            Ok(audit) => ManifestAuditOutcome::Complete {
                audit: Box::new(audit),
            },
            Err(error) => ManifestAuditOutcome::Failed { error },
        };
        let manifest_health = manifest_audit.health();
        let report =
            SnapshotReport::new(reports, &processing_failures, &stale, Some(manifest_audit));
        emit_report(&report, format)?;
        if let Err(error) = report_exit_result(
            !processing_failures.is_empty() || !expectation_failures.is_empty(),
            manifest_health,
        ) {
            emit_failures(&processing_failures, &expectation_failures);
            return Err(error);
        }
        return Ok(());
    }

    let check_report = SnapshotReport::new(reports, &processing_failures, &stale, None);
    emit_check_summary(&check_report);
    if !processing_failures.is_empty() || !expectation_failures.is_empty() {
        emit_failures(&processing_failures, &expectation_failures);
        return Err("snapshot processing failed".to_string());
    }

    replace_snapshot_files(writes)?;

    if stale.is_empty() {
        return Ok(());
    }
    eprintln!("stale snapshots (run `cargo run -p spec42-snapshot -- update`):");
    for path in stale {
        eprintln!("  {}", path.display());
    }
    Err("snapshot check failed".to_string())
}

/// Stages the complete update set before replacing any fixture.
///
/// Each temporary file lives beside its target, so persistence is one atomic filesystem rename.
/// If any staging write fails, dropping the staged files leaves every target unchanged. Filesystem
/// APIs cannot atomically rename a set of paths together, but readers can never observe a torn or
/// partially written individual snapshot.
fn replace_snapshot_files(writes: Vec<(PathBuf, Vec<u8>)>) -> Result<(), String> {
    let mut staged = Vec::with_capacity(writes.len());
    for (path, bytes) in writes {
        let parent = path
            .parent()
            .ok_or_else(|| format!("{}: snapshot path has no parent directory", path.display()))?;
        let mut temporary = tempfile::Builder::new()
            .prefix(".spec42-snapshot-")
            .tempfile_in(parent)
            .map_err(|error| {
                format!("{}: staging file creation failed: {error}", path.display())
            })?;
        temporary
            .write_all(&bytes)
            .map_err(|error| format!("{}: staging write failed: {error}", path.display()))?;
        temporary
            .as_file()
            .sync_all()
            .map_err(|error| format!("{}: staging sync failed: {error}", path.display()))?;
        if let Ok(metadata) = fs::metadata(&path) {
            temporary
                .as_file()
                .set_permissions(metadata.permissions())
                .map_err(|error| {
                    format!("{}: staging permissions failed: {error}", path.display())
                })?;
        }
        // Retain the staged path, not an open handle: a corpus update may contain hundreds of
        // fixtures and must not exhaust the process file-descriptor limit before the commit pass.
        staged.push((path, temporary.into_temp_path()));
    }

    for (path, temporary) in staged {
        temporary.persist(&path).map_err(|error| {
            format!(
                "{}: atomic replacement failed: {}",
                path.display(),
                error.error
            )
        })?;
    }
    Ok(())
}

/// `report` is read-only coverage and expectation auditing. It records stale generated sections,
/// but snapshot freshness is the responsibility of `check`; otherwise an unrelated generated
/// rewrite could hide manifest coverage debt or change report exit semantics.
fn report_exit_result(
    has_evaluation_failures: bool,
    manifest_health: ManifestAuditHealth,
) -> Result<(), String> {
    if has_evaluation_failures {
        Err("snapshot report found failed expectations".to_string())
    } else if manifest_health == ManifestAuditHealth::CoverageDebt {
        Err("snapshot report found manifest coverage debt".to_string())
    } else if manifest_health == ManifestAuditHealth::Failed {
        Err("snapshot report found manifest audit failure".to_string())
    } else {
        Ok(())
    }
}

struct FixtureOutcome {
    updated: Option<String>,
    report: FixtureReport,
    expectation_failure: Option<String>,
}

struct FixtureWorkResult {
    path: PathBuf,
    result: Result<FixtureOutcome, String>,
}

fn sort_work_results(results: &mut [FixtureWorkResult]) {
    results.sort_by(|left, right| left.path.cmp(&right.path));
}

fn evaluate_fixture(
    path: &Path,
    libraries: &LibraryCorpus,
    registry: &IssueRegistry,
    manifest: Option<&ConstraintManifest>,
) -> Result<FixtureOutcome, String> {
    let bytes = fs::read(path).map_err(|error| format!("read failed: {error}"))?;
    let original =
        String::from_utf8(bytes).map_err(|error| format!("snapshot is not UTF-8: {error}"))?;
    let regenerated = regenerate_snapshot(&original, path, libraries, registry, manifest)?;
    Ok(FixtureOutcome {
        updated: (regenerated.text != original).then_some(regenerated.text),
        report: regenerated.report,
        expectation_failure: regenerated.expectation_failure,
    })
}

struct RegeneratedSnapshot {
    text: String,
    report: FixtureReport,
    expectation_failure: Option<String>,
}

fn snapshot_paths(root: &Path, fixture: Option<&Path>) -> Result<Vec<PathBuf>, String> {
    let root = if let Some(fixture) = fixture {
        if fixture.is_absolute() {
            fixture.to_path_buf()
        } else {
            let under_root = root.join(fixture);
            if under_root.exists() {
                under_root
            } else {
                fixture.to_path_buf()
            }
        }
    } else {
        root.to_path_buf()
    };
    if !root.exists() {
        return Err(format!("snapshot path does not exist: {}", root.display()));
    }
    if root.is_file() {
        return (root.extension().is_some_and(|extension| extension == "md"))
            .then_some(vec![root.clone()])
            .ok_or_else(|| format!("snapshot is not Markdown: {}", root.display()));
    }
    let mut paths = Vec::new();
    visit_markdown(&root, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn visit_markdown(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(directory)
        .map_err(|error| format!("{}: read directory failed: {error}", directory.display()))?
    {
        let path = entry
            .map_err(|error| format!("{}: directory entry failed: {error}", directory.display()))?
            .path();
        if path.is_dir() {
            visit_markdown(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            paths.push(path);
        }
    }
    Ok(())
}

fn regenerate_snapshot(
    fixture: &str,
    path: &Path,
    libraries: &LibraryCorpus,
    registry: &IssueRegistry,
    manifest: Option<&ConstraintManifest>,
) -> Result<RegeneratedSnapshot, String> {
    let fallback_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("snapshot.md");
    let meta = parse_fixture_meta(fixture, fallback_name)?;
    let semantic_expectations =
        parse_expected_semantics_with_manifest(fixture, fallback_name, manifest)?;
    let mut documents = if meta.repository_sources.is_empty()
        || raw_section(fixture, "SOURCE")
            .and_then(fenced_block)
            .is_some()
    {
        parse_source_documents(fixture, fallback_name)?
    } else {
        Vec::new()
    };
    documents.extend(load_repository_sources(&meta.repository_sources, path)?);
    for name in &meta.standard_library_documents {
        if !documents.iter().any(|document| &document.name == name) {
            return Err(format!(
                "{}: META standard_library_document references unknown SOURCE document {:?}",
                path.display(),
                name
            ));
        }
    }
    let services = libraries.services.clone();
    let mut admitted_documents = documents
        .iter()
        .map(|document| {
            services
                .source
                .admit_memory(
                    "snapshot",
                    &document.name,
                    &document.text,
                    if meta.standard_library_documents.contains(&document.name) {
                        SourceKind::StandardLibrary
                    } else {
                        SourceKind::Workspace
                    },
                )
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    admitted_documents.extend_from_slice(libraries.documents(meta.libraries)?);
    let source_documents = admitted_documents.clone();
    let probes = parse_editor_probes(fixture, &documents, fallback_name)?;
    let qualified_reference_probes =
        parse_qualified_reference_probes(fixture, &documents, fallback_name)?;
    // One build per fixture through the shared publication owner. That owner alone partitions and
    // reuses library inputs; schedule and cold/warm parity remain authority invariants rather than
    // fixture-level construction choices.
    let canonical_model = services
        .publication
        .publish(&admitted_documents, std::iter::empty::<Box<str>>())
        .map_err(|error| {
            format!(
                "{}: canonical semantic build failed: {error}",
                path.display()
            )
        })?;
    let hover_workspace = InMemoryWorkspace::from_documents_and_publication(
        source_documents.clone(),
        Arc::clone(&canonical_model),
    )
    .map_err(|error| format!("{}: hover workspace failed: {error}", path.display()))?;
    let canonical = render_owned_sections(
        &canonical_model,
        &documents,
        &source_documents,
        &hover_workspace,
        &probes,
        &qualified_reference_probes,
    )?;
    ensure_sections_balanced(&canonical).map_err(|error| format!("{}: {error}", path.display()))?;

    let semantic_mismatch = if let Some(expectations) = &semantic_expectations {
        match observe_semantic_expectations(&canonical_model, expectations) {
            Err(error) => Some(error),
            Ok(canonical_observations) => {
                compare_semantic_expectations(expectations, &canonical_observations).err()
            }
        }
    } else {
        None
    };

    let observed_diagnostics: Vec<_> = canonical_model
        .diagnostics()
        .published()
        .iter()
        .map(|diagnostic| ObservedDiagnostic {
            category: diagnostic.category().as_str().to_string(),
            origin: diagnostic.origin().as_str().to_string(),
            severity: diagnostic.severity().as_str().to_string(),
        })
        .collect();
    let mut expectation = check_fixture_expectation(
        fixture,
        &canonical.diagnostics,
        &meta,
        registry,
        semantic_expectations.is_some(),
        semantic_mismatch.as_deref(),
        fallback_name,
        path,
    )?;
    if let Some(error) =
        validate_source_intent(&meta, registry, &observed_diagnostics, fallback_name)?
    {
        expectation.state = ExpectationState::Failed;
        expectation.failure = Some(error);
    }

    let fixture = replace_or_insert_section(fixture, "SMG", &canonical.smg)
        .ok_or_else(|| format!("{}: missing SOURCE/SMG section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "DIAGNOSTICS", &canonical.diagnostics)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "TYPES", &canonical.types)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "NAVIGATION", &canonical.navigation)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = if probes.queries.is_empty() {
        fixture
    } else {
        replace_or_insert_section(&fixture, "EDITOR RESULTS", &canonical.editor_queries)
            .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?
    };
    let fixture = if probes.hover.is_empty() {
        fixture
    } else {
        replace_or_insert_section(&fixture, "HOVER RESULTS", &canonical.hover_reports)
            .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?
    };
    let fixture = if probes.hover.is_empty() {
        fixture
    } else {
        replace_or_insert_raw_section(&fixture, "HOVER MARKDOWN", &canonical.hover_markdown)
    };
    let fixture = if qualified_reference_probes.is_empty() {
        fixture
    } else {
        replace_or_insert_section(
            &fixture,
            "QUALIFIED REFERENCE RESULTS",
            &canonical.qualified_references,
        )
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?
    };
    let fixture = if let Some(generation) = &meta.generation {
        let canonical_generated =
            execute_generation(Arc::clone(&canonical_model), generation, path)?;
        replace_or_insert_generated_section(&fixture, &canonical_generated)
    } else {
        fixture
    };
    Ok(RegeneratedSnapshot {
        text: canonicalize_sections(&fixture),
        report: FixtureReport::from_meta(
            path,
            &meta,
            expectation.state,
            registry,
            observed_diagnostics,
        ),
        expectation_failure: expectation.failure,
    })
}

struct CheckedExpectation {
    state: ExpectationState,
    failure: Option<String>,
}

// These inputs are separate contract owners; grouping them would obscure their provenance.
#[allow(clippy::too_many_arguments)]
fn check_fixture_expectation(
    fixture: &str,
    actual: &str,
    meta: &FixtureMeta,
    registry: &IssueRegistry,
    has_semantic_expectations: bool,
    semantic_mismatch: Option<&str>,
    fallback_name: &str,
    fixture_path: &Path,
) -> Result<CheckedExpectation, String> {
    if let Some(expectation) = &meta.normative_expectation {
        return check_normative_expectation(
            fixture,
            actual,
            expectation,
            registry,
            has_semantic_expectations,
            semantic_mismatch,
            fallback_name,
            fixture_path,
        );
    }
    check_untagged_diagnostics(fixture, actual, fallback_name)
}

fn validate_source_intent(
    meta: &FixtureMeta,
    registry: &IssueRegistry,
    diagnostics: &[ObservedDiagnostic],
    fallback_name: &str,
) -> Result<Option<String>, String> {
    let Some(expectation) = &meta.normative_expectation else {
        return Ok(None);
    };
    let has_category = |category| {
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.category == category)
    };
    let parser_gap = expectation
        .blocked_by
        .as_deref()
        .and_then(|id| registry.blocker(id))
        .is_some_and(|blocker| blocker.kind == IssueKind::ParserGap);
    let contradiction = match expectation.source_expectation {
        SourceExpectation::Accepted
            if !parser_gap && (has_category("malformed_syntax") || has_category("unsupported_syntax")) =>
        {
            Some("accepted source produced malformed_syntax or unsupported_syntax without a parser_gap blocker")
        }
        SourceExpectation::Malformed if !has_category("malformed_syntax") => {
            Some("malformed source produced no malformed_syntax diagnostic")
        }
        SourceExpectation::Unsupported if !has_category("unsupported_syntax") => {
            Some("unsupported source produced no unsupported_syntax diagnostic")
        }
        _ => None,
    };
    Ok(contradiction
        .map(|reason| format!("{fallback_name}: source-intent contradiction: {reason}")))
}

// These inputs are separate contract owners; grouping them would obscure their provenance.
#[allow(clippy::too_many_arguments)]
fn check_normative_expectation(
    fixture: &str,
    actual: &str,
    expectation: &NormativeExpectation,
    registry: &IssueRegistry,
    has_semantic_expectations: bool,
    semantic_mismatch: Option<&str>,
    fallback_name: &str,
    fixture_path: &Path,
) -> Result<CheckedExpectation, String> {
    validate_blocker_contract(
        expectation,
        registry,
        fallback_name,
        raw_section(fixture, "EXPECTED DIAGNOSTICS").is_some()
            && expectation.expectation != ExpectationKind::Diagnostics,
    )?;
    match expectation.expectation {
        ExpectationKind::Diagnostics => {
            if raw_section(fixture, "EXPECTED DIAGNOSTICS").is_none() {
                return Ok(CheckedExpectation {
                    state: ExpectationState::Failed,
                    failure: Some(format!(
                        "{fallback_name}: META expectation=diagnostics requires an EXPECTED DIAGNOSTICS section"
                    )),
                });
            }
            check_diagnostic_expectation(
                fixture,
                actual,
                expectation.blocked_by.as_deref(),
                registry,
                fallback_name,
            )
        }
        ExpectationKind::Semantics => check_semantic_expectation(
            expectation.blocked_by.as_deref(),
            registry,
            has_semantic_expectations,
            semantic_mismatch,
            fallback_name,
        )
        .and_then(|primary| {
            check_supplemental_diagnostic_expectation(fixture, actual, primary, fallback_name)
        }),
        ExpectationKind::ByConstruction => {
            check_by_construction_expectation(expectation, registry, fallback_name, fixture_path)
                .and_then(|primary| {
                    check_supplemental_diagnostic_expectation(
                        fixture,
                        actual,
                        primary,
                        fallback_name,
                    )
                })
        }
    }
}

/// A semantic or by-construction rule may additionally pin the canonical diagnostics caused by
/// that rule's fact state.  The assertion is deliberately supplemental: the closed primary
/// expectation-family combinations remain unchanged.  A typed blocker is stale only after both
/// the primary assertion and this exact diagnostic assertion pass.
fn check_supplemental_diagnostic_expectation(
    fixture: &str,
    actual: &str,
    primary: CheckedExpectation,
    fallback_name: &str,
) -> Result<CheckedExpectation, String> {
    if raw_section(fixture, "EXPECTED DIAGNOSTICS").is_none() {
        return Ok(primary);
    }
    let mismatch = check_diagnostics_equal(fixture, actual, fallback_name)?;
    match (primary.state, mismatch) {
        (ExpectationState::Passed, None) | (ExpectationState::Stale, None) => Ok(primary),
        // A blocker remains active when either authored assertion is unmet. In particular, a
        // formerly-stale primary semantic expectation is not stale if its root-cause diagnostic
        // is still wrong or absent.
        (ExpectationState::Blocked, _) | (ExpectationState::Stale, Some(_)) => {
            Ok(CheckedExpectation {
                state: ExpectationState::Blocked,
                failure: None,
            })
        }
        (ExpectationState::Passed, Some(error)) => Ok(CheckedExpectation {
            state: ExpectationState::Failed,
            failure: Some(error),
        }),
        (ExpectationState::Failed, Some(diagnostic_error)) => {
            let failure = primary
                .failure
                .map(|primary_error| format!("{primary_error}\n{diagnostic_error}"))
                .unwrap_or(diagnostic_error);
            Ok(CheckedExpectation {
                state: ExpectationState::Failed,
                failure: Some(failure),
            })
        }
        (ExpectationState::Failed, None) => Ok(primary),
        (ExpectationState::NotApplicable, _) => Err(format!(
            "{fallback_name}: supplemental diagnostics have no typed primary expectation"
        )),
        (ExpectationState::Unclassified, _) => Err(format!(
            "{fallback_name}: supplemental diagnostics have no classified primary expectation"
        )),
    }
}

fn check_semantic_expectation(
    blocked_by: Option<&str>,
    registry: &IssueRegistry,
    has_semantic_expectations: bool,
    semantic_mismatch: Option<&str>,
    fallback_name: &str,
) -> Result<CheckedExpectation, String> {
    if !has_semantic_expectations {
        return Ok(CheckedExpectation {
            state: ExpectationState::Failed,
            failure: Some(format!(
                "{fallback_name}: META expectation=semantics requires an EXPECTED SEMANTICS section"
            )),
        });
    }
    match (semantic_mismatch, blocked_by) {
        (None, Some(id)) => Ok(CheckedExpectation {
            state: ExpectationState::Stale,
            failure: Some(format!(
                "{fallback_name}: semantic expectation now passes; remove META blocked_by={id}"
            )),
        }),
        (None, None) => Ok(CheckedExpectation {
            state: ExpectationState::Passed,
            failure: None,
        }),
        (Some(_), Some(id)) => {
            if registry.blocker(id).is_none() {
                return Err(format!(
                    "{fallback_name}: META blocked_by references unknown issue {id:?}"
                ));
            }
            Ok(CheckedExpectation {
                state: ExpectationState::Blocked,
                failure: None,
            })
        }
        (Some(error), None) => Ok(CheckedExpectation {
            state: ExpectationState::Failed,
            failure: Some(error.to_string()),
        }),
    }
}

fn check_by_construction_expectation(
    expectation: &NormativeExpectation,
    registry: &IssueRegistry,
    fallback_name: &str,
    fixture_path: &Path,
) -> Result<CheckedExpectation, String> {
    if let Some(id) = expectation.blocked_by.as_deref() {
        let Some(blocker) = registry.blocker(id) else {
            return Err(format!(
                "{fallback_name}: META blocked_by references unknown issue {id:?}"
            ));
        };
        if blocker.kind != IssueKind::AbstractSyntaxCoverageGap {
            return Err(format!(
                "{fallback_name}: META expectation=by_construction may be blocked only by an abstract_syntax_coverage_gap"
            ));
        }
        if let Some(evidence) = &expectation.evidence {
            validate_by_construction_evidence(evidence, fixture_path, fallback_name)?;
        }
        return Ok(CheckedExpectation {
            state: ExpectationState::Blocked,
            failure: None,
        });
    }
    let Some(evidence) = &expectation.evidence else {
        return Ok(CheckedExpectation {
            state: ExpectationState::Failed,
            failure: Some(format!(
                "{fallback_name}: META expectation=by_construction requires evidence_reference=test:<repository-relative-path> or file:<repository-relative-path>"
            )),
        });
    };
    validate_by_construction_evidence(evidence, fixture_path, fallback_name)?;
    Ok(CheckedExpectation {
        state: ExpectationState::Passed,
        failure: None,
    })
}

fn validate_by_construction_evidence(
    evidence: &ByConstructionEvidence,
    fixture_path: &Path,
    fallback_name: &str,
) -> Result<(), String> {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let evidence_path = repository_root.join(evidence.path());
    let metadata = fs::metadata(&evidence_path).map_err(|error| {
        format!(
            "{fallback_name}: META evidence_reference {} cannot be read: {error}",
            evidence.path().display()
        )
    })?;
    if !metadata.is_file() {
        return Err(format!(
            "{fallback_name}: META evidence_reference {} must name a file",
            evidence.path().display()
        ));
    }
    if fs::canonicalize(&evidence_path).ok() == fs::canonicalize(fixture_path).ok() {
        return Err(format!(
            "{fallback_name}: META evidence_reference must point to the owning executable test/file, not this fixture"
        ));
    }
    let contents = fs::read_to_string(&evidence_path).map_err(|error| {
        format!(
            "{fallback_name}: META evidence_reference {} is not UTF-8 text: {error}",
            evidence.path().display()
        )
    })?;
    match evidence {
        ByConstructionEvidence::Test(path) => {
            if path.extension().and_then(|extension| extension.to_str()) != Some("rs")
                || !(contents.contains("#[test]") || contents.contains("mod tests"))
            {
                return Err(format!(
                    "{fallback_name}: META test evidence must name a Rust file containing executable test coverage"
                ));
            }
        }
        ByConstructionEvidence::File(path) => {
            let is_test_source = path.extension().and_then(|extension| extension.to_str())
                == Some("rs")
                && (contents.contains("#[test]") || contents.contains("mod tests"));
            let is_abstract_fixture = path.extension().and_then(|extension| extension.to_str())
                == Some("md")
                && path.starts_with("tests/");
            if !is_test_source && !is_abstract_fixture {
                return Err(format!(
                    "{fallback_name}: META file evidence must be an executable Rust test or tests/*.md abstract/interchange fixture"
                ));
            }
        }
    }
    Ok(())
}

fn check_untagged_diagnostics(
    fixture: &str,
    actual: &str,
    fallback_name: &str,
) -> Result<CheckedExpectation, String> {
    if raw_section(fixture, "EXPECTED DIAGNOSTICS").is_none() {
        return Ok(CheckedExpectation {
            state: ExpectationState::NotApplicable,
            failure: None,
        });
    }
    let checked = check_diagnostics_equal(fixture, actual, fallback_name)?;
    match checked {
        None => Ok(CheckedExpectation {
            state: ExpectationState::Passed,
            failure: None,
        }),
        Some(error) => Ok(CheckedExpectation {
            state: ExpectationState::Failed,
            failure: Some(error),
        }),
    }
}

fn check_diagnostic_expectation(
    fixture: &str,
    actual: &str,
    blocked_by: Option<&str>,
    registry: &IssueRegistry,
    fallback_name: &str,
) -> Result<CheckedExpectation, String> {
    let checked = check_diagnostics_equal(fixture, actual, fallback_name)?;
    match (checked, blocked_by) {
        (None, Some(id)) => Ok(CheckedExpectation {
            state: ExpectationState::Stale,
            failure: Some(format!(
                "{fallback_name}: expectation now passes; remove META blocked_by={id}"
            )),
        }),
        (None, None) => Ok(CheckedExpectation {
            state: ExpectationState::Passed,
            failure: None,
        }),
        (Some(_), Some(id)) => {
            if registry.blocker(id).is_none() {
                return Err(format!(
                    "{fallback_name}: META blocked_by references unknown issue {id:?}"
                ));
            }
            Ok(CheckedExpectation {
                state: ExpectationState::Blocked,
                failure: None,
            })
        }
        (Some(error), None) => Ok(CheckedExpectation {
            state: ExpectationState::Failed,
            failure: Some(error),
        }),
    }
}

/// `Ok(None)` means the fixture has no authored diagnostic assertion. The caller decides whether
/// that is valid for its typed expectation contract.
fn check_diagnostics_equal(
    fixture: &str,
    actual: &str,
    fallback_name: &str,
) -> Result<Option<String>, String> {
    let Some(section) = raw_section(fixture, "EXPECTED DIAGNOSTICS") else {
        return Ok(None);
    };
    let Some((expected, _)) = fenced_block(section) else {
        return Err(format!(
            "{fallback_name}: malformed EXPECTED DIAGNOSTICS fence"
        ));
    };
    if expected.trim() == actual.trim() {
        return Ok(None);
    }
    Ok(Some(format!(
        "{fallback_name}: EXPECTED DIAGNOSTICS do not match the canonical diagnostics\nexpected:\n{}\nactual:\n{}",
        expected.trim(),
        actual.trim()
    )))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AuthoredSexpr {
    Atom(String),
    String(String),
    List(Vec<AuthoredSexpr>),
}

fn parse_expected_semantics_with_manifest(
    fixture: &str,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<Option<SemanticExpectations>, String> {
    let Some(section) = raw_section(fixture, "EXPECTED SEMANTICS") else {
        return Ok(None);
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!(
            "{fallback_name}: malformed EXPECTED SEMANTICS fence"
        ));
    };
    let expression = parse_authored_sexpr(&text)
        .map_err(|error| format!("{fallback_name}: malformed EXPECTED SEMANTICS: {error}"))?;
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: EXPECTED SEMANTICS must be a (fixture-semantics ...) list"
        ));
    };
    if items.first().and_then(authored_atom) != Some("fixture-semantics") {
        return Err(format!(
            "{fallback_name}: EXPECTED SEMANTICS must start with fixture-semantics"
        ));
    }
    let mut relationships = Vec::new();
    let mut feature_derived_relationships = Vec::new();
    let mut type_derived_relationships = Vec::new();
    let mut type_derived_elements = Vec::new();
    let mut type_derived_facts = Vec::new();
    let mut action_derived_facts = Vec::new();
    let mut definition_usage_derived = Vec::new();
    let mut requirement_derived_facts = Vec::new();
    let mut element_derived_owners = Vec::new();
    let mut element_derived_documentation = Vec::new();
    let mut namespace_derived_elements = Vec::new();
    let mut namespace_import_derived_elements = Vec::new();
    let mut binding_connector_checks = Vec::new();
    let mut redefinition_checks = Vec::new();
    let mut specialization_checks = Vec::new();
    for item in &items[1..] {
        match item
            .as_list_name()
            .ok_or_else(|| format!("{fallback_name}: semantic expectation must be a list"))?
        {
            "relationship" => {
                relationships.push(parse_relationship_expectation(item, fallback_name)?)
            }
            "derived-relationship-collection" => feature_derived_relationships.push(
                parse_feature_derived_relationship_expectation(item, fallback_name, manifest)?,
            ),
            "type-derived-relationship-collection" => type_derived_relationships.push(
                parse_type_derived_relationship_expectation(item, fallback_name, manifest)?,
            ),
            "type-derived-element" => type_derived_elements.push(
                parse_type_derived_element_expectation(item, fallback_name, manifest)?,
            ),
            "type-derived-fact" => type_derived_facts.push(parse_type_derived_fact_expectation(
                item,
                fallback_name,
                manifest,
            )?),
            "action-derived-fact" => action_derived_facts.push(
                parse_action_derived_fact_expectation(item, fallback_name, manifest)?,
            ),
            "definition-usage-derived" => definition_usage_derived.push(
                parse_definition_usage_derived_expectation(item, fallback_name, manifest)?,
            ),
            "requirement-derived-fact" => requirement_derived_facts.push(
                parse_requirement_derived_fact_expectation(item, fallback_name, manifest)?,
            ),
            "element-owner" => element_derived_owners.push(
                parse_element_derived_owner_expectation(item, fallback_name, manifest)?,
            ),
            "element-documentation" => element_derived_documentation.push(
                parse_element_derived_documentation_expectation(item, fallback_name, manifest)?,
            ),
            "namespace-derived-element" => namespace_derived_elements.push(
                parse_namespace_derived_element_expectation(item, fallback_name, manifest)?,
            ),
            "namespace-import-derived-element" => namespace_import_derived_elements.push(
                parse_namespace_import_derived_element_expectation(item, fallback_name, manifest)?,
            ),
            "binding-connector-check" => binding_connector_checks.push(
                parse_binding_connector_check_expectation(item, fallback_name, manifest)?,
            ),
            "redefinition-check" => redefinition_checks.push(parse_redefinition_check_expectation(
                item,
                fallback_name,
                manifest,
            )?),
            "specialization-check" => specialization_checks.push(
                parse_specialization_check_expectation(item, fallback_name, manifest)?,
            ),
            name => {
                return Err(format!(
                    "{fallback_name}: EXPECTED SEMANTICS does not accept {name:?} expectations"
                ))
            }
        }
    }
    if relationships.is_empty()
        && feature_derived_relationships.is_empty()
        && type_derived_relationships.is_empty()
        && type_derived_elements.is_empty()
        && type_derived_facts.is_empty()
        && action_derived_facts.is_empty()
        && definition_usage_derived.is_empty()
        && requirement_derived_facts.is_empty()
        && element_derived_owners.is_empty()
        && element_derived_documentation.is_empty()
        && namespace_derived_elements.is_empty()
        && namespace_import_derived_elements.is_empty()
        && binding_connector_checks.is_empty()
        && redefinition_checks.is_empty()
        && specialization_checks.is_empty()
    {
        return Err(format!(
            "{fallback_name}: EXPECTED SEMANTICS must contain at least one assertion"
        ));
    }
    Ok(Some(SemanticExpectations {
        relationships,
        feature_derived_relationships,
        type_derived_relationships,
        type_derived_elements,
        type_derived_facts,
        action_derived_facts,
        definition_usage_derived,
        requirement_derived_facts,
        element_derived_owners,
        element_derived_documentation,
        namespace_derived_elements,
        namespace_import_derived_elements,
        binding_connector_checks,
        redefinition_checks,
        specialization_checks,
    }))
}

#[cfg(test)]
fn parse_expected_semantics(
    fixture: &str,
    fallback_name: &str,
) -> Result<Option<SemanticExpectations>, String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("specifications/constraint_manifest.toml");
    let manifest = ConstraintManifest::load_toml(&path)?;
    parse_expected_semantics_with_manifest(fixture, fallback_name, Some(&manifest))
}

trait AuthoredSexprListName {
    fn as_list_name(&self) -> Option<&str>;
}

impl AuthoredSexprListName for AuthoredSexpr {
    fn as_list_name(&self) -> Option<&str> {
        match self {
            Self::List(items) => items.first().and_then(authored_atom),
            Self::Atom(_) | Self::String(_) => None,
        }
    }
}

fn parse_relationship_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
) -> Result<RelationshipExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: semantic expectation must be a list"
        ));
    };
    if items.first().and_then(authored_atom) != Some("relationship") {
        return Err(format!(
            "{fallback_name}: EXPECTED SEMANTICS only accepts relationship expectations"
        ));
    }
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["kind", "source", "target", "provenance", "outcome"],
        "semantic relationship",
        fallback_name,
    )?;
    let kind = SemanticRelationshipKind::parse(
        fields
            .get("kind")
            .ok_or_else(|| format!("{fallback_name}: semantic relationship requires kind"))?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| format!("{fallback_name}: semantic relationship requires source"))?;
    let outcome = SemanticRelationshipOutcome::parse(
        fields
            .get("outcome")
            .ok_or_else(|| format!("{fallback_name}: semantic relationship requires outcome"))?,
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    let provenance = match fields.get("provenance") {
        Some(value) => Some(parse_relationship_provenance(value, fallback_name)?),
        None => None,
    };
    match outcome {
        SemanticRelationshipOutcome::Resolved => {
            if target.as_deref().is_none_or(str::is_empty) || provenance.is_none() {
                return Err(format!(
                    "{fallback_name}: resolved semantic relationship requires target and provenance"
                ));
            }
        }
        SemanticRelationshipOutcome::Absent => {
            if target.is_some() || provenance.is_none() {
                return Err(format!(
                    "{fallback_name}: absent semantic relationship requires provenance and no target"
                ));
            }
        }
        SemanticRelationshipOutcome::Incomplete => {
            if target.is_some() || provenance.is_some() {
                return Err(format!(
                    "{fallback_name}: incomplete semantic relationship must not declare target or provenance"
                ));
            }
        }
        SemanticRelationshipOutcome::Unresolved
        | SemanticRelationshipOutcome::Ambiguous
        | SemanticRelationshipOutcome::Unsupported => {
            if target.is_some() || provenance.is_none() {
                return Err(format!(
                    "{fallback_name}: {outcome:?} semantic relationship requires provenance and no target"
                ));
            }
        }
    }
    Ok(RelationshipExpectation {
        kind,
        source,
        target,
        provenance,
        outcome,
    })
}

fn parse_feature_derived_relationship_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<FeatureDerivedRelationshipExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: derived relationship collection expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &[
            "rule_id",
            "kind",
            "source",
            "target",
            "provenance",
            "outcome",
        ],
        "semantic derived relationship collection",
        fallback_name,
    )?;
    let rule_id = fields.get("rule_id").ok_or_else(|| {
        format!("{fallback_name}: semantic derived relationship collection requires rule_id")
    })?;
    let collection = feature_collection_for_rule(manifest, rule_id, fallback_name)?;
    let kind = SemanticRelationshipKind::parse(
        fields.get("kind").ok_or_else(|| {
            format!("{fallback_name}: semantic derived relationship collection requires kind")
        })?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| {
            format!("{fallback_name}: semantic derived relationship collection requires source")
        })?;
    let outcome = SemanticRelationshipOutcome::parse(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic derived relationship collection requires outcome")
        })?,
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    let provenance = match fields.get("provenance") {
        Some(value) => Some(parse_relationship_provenance(value, fallback_name)?),
        None => None,
    };
    validate_semantic_relationship_shape(
        outcome,
        target.as_deref(),
        provenance,
        "semantic derived relationship collection",
        fallback_name,
    )?;
    Ok(FeatureDerivedRelationshipExpectation {
        collection,
        source,
        kind,
        target,
        provenance,
        outcome,
    })
}

fn parse_type_derived_relationship_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<TypeDerivedRelationshipExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: type derived relationship collection expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &[
            "rule_id",
            "kind",
            "source",
            "target",
            "provenance",
            "outcome",
        ],
        "semantic type derived relationship collection",
        fallback_name,
    )?;
    let rule_id = fields.get("rule_id").ok_or_else(|| {
        format!("{fallback_name}: semantic type derived relationship collection requires rule_id")
    })?;
    let collection = type_collection_for_rule(manifest, rule_id, fallback_name)?;
    let kind = SemanticRelationshipKind::parse(
        fields.get("kind").ok_or_else(|| {
            format!("{fallback_name}: semantic type derived relationship collection requires kind")
        })?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| {
            format!(
                "{fallback_name}: semantic type derived relationship collection requires source"
            )
        })?;
    let outcome = SemanticRelationshipOutcome::parse(
        fields.get("outcome").ok_or_else(|| {
            format!(
                "{fallback_name}: semantic type derived relationship collection requires outcome"
            )
        })?,
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    let provenance = match fields.get("provenance") {
        Some(value) => Some(parse_relationship_provenance(value, fallback_name)?),
        None => None,
    };
    validate_semantic_relationship_shape(
        outcome,
        target.as_deref(),
        provenance,
        "semantic type derived relationship collection",
        fallback_name,
    )?;
    Ok(TypeDerivedRelationshipExpectation {
        collection,
        source,
        kind,
        target,
        provenance,
        outcome,
    })
}

fn parse_type_derived_element_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<TypeDerivedElementExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: Type element expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "source", "target", "outcome"],
        "semantic Type element",
        fallback_name,
    )?;
    let collection = type_element_collection_for_rule(
        manifest,
        fields
            .get("rule_id")
            .ok_or_else(|| format!("{fallback_name}: semantic Type element requires rule_id"))?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| format!("{fallback_name}: semantic Type element requires source"))?;
    let outcome = TypeDerivedElementOutcome::parse(
        fields
            .get("outcome")
            .ok_or_else(|| format!("{fallback_name}: semantic Type element requires outcome"))?,
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    match outcome {
        TypeDerivedElementOutcome::Resolved if target.as_deref().is_none_or(str::is_empty) => {
            return Err(format!(
                "{fallback_name}: resolved semantic Type element requires target"
            ));
        }
        TypeDerivedElementOutcome::Absent
        | TypeDerivedElementOutcome::Incomplete
        | TypeDerivedElementOutcome::Unsupported
            if target.is_some() =>
        {
            return Err(format!(
                "{fallback_name}: {outcome:?} semantic Type element must not declare target"
            ));
        }
        _ => {}
    }
    Ok(TypeDerivedElementExpectation {
        collection,
        source,
        target,
        outcome,
    })
}

/// Parses a desired exact Type-derived fact.  Fixtures name only the public endpoint when the
/// normative result has one; the collection itself supplies the closed result shape.  In
/// particular, multiplicity is a scalar fact, so `resolved` deliberately needs no display-name
/// surrogate for its canonical identity.
fn parse_type_derived_fact_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<TypeDerivedFactExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: Type fact expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "source", "target", "outcome"],
        "semantic Type fact",
        fallback_name,
    )?;
    let collection = type_fact_collection_for_rule(
        manifest,
        fields
            .get("rule_id")
            .ok_or_else(|| format!("{fallback_name}: semantic Type fact requires rule_id"))?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| format!("{fallback_name}: semantic Type fact requires source"))?;
    let outcome = TypeDerivedElementOutcome::parse(
        fields
            .get("outcome")
            .ok_or_else(|| format!("{fallback_name}: semantic Type fact requires outcome"))?,
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    let endpoint_required = collection != TypeDerivedFactCollection::Multiplicity;
    match outcome {
        TypeDerivedElementOutcome::Resolved
            if endpoint_required && target.as_deref().is_none_or(str::is_empty) =>
        {
            return Err(format!(
                "{fallback_name}: resolved semantic Type fact requires target except for multiplicity"
            ));
        }
        TypeDerivedElementOutcome::Absent
        | TypeDerivedElementOutcome::Incomplete
        | TypeDerivedElementOutcome::Unsupported
            if target.is_some() =>
        {
            return Err(format!(
                "{fallback_name}: {outcome:?} semantic Type fact must not declare target"
            ));
        }
        _ => {}
    }
    Ok(TypeDerivedFactExpectation {
        collection,
        source,
        target,
        outcome,
    })
}

/// Parses an authored Systems::Actions desired result. The manifest fixes the result shape;
/// source is a public anchor into the canonical publication, never a reconstructed anonymous
/// action or argument identity.
fn parse_action_derived_fact_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<ActionDerivedFactExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: Action fact expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "source", "target", "position", "outcome"],
        "semantic Action fact",
        fallback_name,
    )?;
    let collection = action_fact_collection_for_rule(
        manifest,
        fields
            .get("rule_id")
            .ok_or_else(|| format!("{fallback_name}: semantic Action fact requires rule_id"))?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| format!("{fallback_name}: semantic Action fact requires source"))?;
    let outcome = TypeDerivedElementOutcome::parse(
        fields
            .get("outcome")
            .ok_or_else(|| format!("{fallback_name}: semantic Action fact requires outcome"))?,
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    let position = fields
        .get("position")
        .map(|value| {
            value
                .parse::<u32>()
                .ok()
                .filter(|position| *position > 0)
                .ok_or_else(|| {
                    format!(
                        "{fallback_name}: semantic Action fact position must be a positive integer"
                    )
                })
        })
        .transpose()?;
    if matches!(
        outcome,
        TypeDerivedElementOutcome::Absent
            | TypeDerivedElementOutcome::Incomplete
            | TypeDerivedElementOutcome::Unsupported
    ) && (target.is_some() || position.is_some())
    {
        return Err(format!(
            "{fallback_name}: {outcome:?} semantic Action fact must not declare target or position"
        ));
    }
    if target.is_some() && position.is_some() {
        return Err(format!(
            "{fallback_name}: semantic Action fact must not declare both target and position"
        ));
    }
    Ok(ActionDerivedFactExpectation {
        collection,
        source,
        target,
        position,
        outcome,
    })
}

fn parse_requirement_derived_fact_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<RequirementDerivedFactExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: Requirements fact expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &[
            "rule_id",
            "source",
            "target",
            "text",
            "outcome",
            "prerequisite",
        ],
        "semantic Requirements fact",
        fallback_name,
    )?;
    let collection = requirement_derived_fact_collection_for_rule(
        manifest,
        fields.get("rule_id").ok_or_else(|| {
            format!("{fallback_name}: semantic Requirements fact requires rule_id")
        })?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| format!("{fallback_name}: semantic Requirements fact requires source"))?;
    let outcome = parse_requirement_derived_outcome(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic Requirements fact requires outcome")
        })?,
        fields.get("prerequisite"),
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    let text = fields.get("text").cloned();
    match outcome {
        RequirementDerivedFactExpectationOutcome::Resolved => {
            if target.as_deref().is_none_or(str::is_empty) || text.is_some() {
                return Err(format!(
                    "{fallback_name}: resolved Requirements fact requires target only"
                ));
            }
        }
        RequirementDerivedFactExpectationOutcome::Text => {
            if text.as_deref().is_none_or(str::is_empty) || target.is_some() {
                return Err(format!(
                    "{fallback_name}: text Requirements fact requires text only"
                ));
            }
        }
        RequirementDerivedFactExpectationOutcome::Absent
        | RequirementDerivedFactExpectationOutcome::Incomplete
        | RequirementDerivedFactExpectationOutcome::Unsupported(_) => {
            if target.is_some() || text.is_some() {
                return Err(format!(
                    "{fallback_name}: non-value Requirements fact must not declare target or text"
                ));
            }
        }
    }
    if !matches!(
        outcome,
        RequirementDerivedFactExpectationOutcome::Unsupported(_)
    ) && fields.contains_key("prerequisite")
    {
        return Err(format!(
            "{fallback_name}: only unsupported Requirements fact may declare prerequisite"
        ));
    }
    Ok(RequirementDerivedFactExpectation {
        collection,
        source,
        target,
        text,
        outcome,
    })
}

fn parse_definition_usage_derived_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<DefinitionUsageDerivedExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: Definition/Usage derivation expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "source", "target", "outcome", "prerequisite"],
        "semantic Definition/Usage derivation",
        fallback_name,
    )?;
    let kind = definition_usage_derived_kind_for_rule(
        manifest,
        fields.get("rule_id").ok_or_else(|| {
            format!("{fallback_name}: semantic Definition/Usage derivation requires rule_id")
        })?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| {
            format!("{fallback_name}: semantic Definition/Usage derivation requires source")
        })?;
    let outcome = parse_definition_usage_outcome(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic Definition/Usage derivation requires outcome")
        })?,
        fields.get("prerequisite"),
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    let target_required = matches!(outcome, DefinitionUsageDerivedExpectationOutcome::Resolved);
    if target_required && target.as_deref().is_none_or(str::is_empty) {
        return Err(format!(
            "{fallback_name}: resolved semantic Definition/Usage derivation requires target"
        ));
    }
    if !target_required && target.is_some() {
        return Err(format!(
            "{fallback_name}: non-element Definition/Usage derivation must not declare target"
        ));
    }
    if !matches!(
        outcome,
        DefinitionUsageDerivedExpectationOutcome::Unsupported(_)
    ) && fields.contains_key("prerequisite")
    {
        return Err(format!(
            "{fallback_name}: only unsupported Definition/Usage derivation may declare prerequisite"
        ));
    }
    Ok(DefinitionUsageDerivedExpectation {
        kind,
        source,
        target,
        outcome,
    })
}

fn parse_element_derived_owner_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<ElementDerivedOwnerExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: element owner expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "source", "owner", "outcome"],
        "semantic element owner",
        fallback_name,
    )?;
    let kind = element_owner_kind_for_rule(
        manifest,
        fields
            .get("rule_id")
            .ok_or_else(|| format!("{fallback_name}: semantic element owner requires rule_id"))?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| format!("{fallback_name}: semantic element owner requires source"))?;
    let outcome = ElementDerivedOwnerOutcome::parse(
        fields
            .get("outcome")
            .ok_or_else(|| format!("{fallback_name}: semantic element owner requires outcome"))?,
        fallback_name,
    )?;
    let owner = fields.get("owner").cloned();
    match outcome {
        ElementDerivedOwnerOutcome::Resolved if owner.as_deref().is_none_or(str::is_empty) => {
            return Err(format!(
                "{fallback_name}: resolved semantic element owner requires owner"
            ));
        }
        ElementDerivedOwnerOutcome::Absent | ElementDerivedOwnerOutcome::Incomplete
            if owner.is_some() =>
        {
            return Err(format!(
                "{fallback_name}: {outcome:?} semantic element owner must not declare owner"
            ));
        }
        _ => {}
    }
    Ok(ElementDerivedOwnerExpectation {
        kind,
        source,
        owner,
        outcome,
    })
}

fn parse_element_derived_documentation_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<ElementDerivedDocumentationExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: element documentation expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &[
            "rule_id", "source", "form", "locale", "language", "text", "outcome",
        ],
        "semantic element documentation",
        fallback_name,
    )?;
    let collection = element_documentation_collection_for_rule(
        manifest,
        fields.get("rule_id").ok_or_else(|| {
            format!("{fallback_name}: semantic element documentation requires rule_id")
        })?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| {
            format!("{fallback_name}: semantic element documentation requires source")
        })?;
    let outcome = ElementDerivedDocumentationOutcome::parse(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic element documentation requires outcome")
        })?,
        fallback_name,
    )?;
    let expected = match outcome {
        ElementDerivedDocumentationOutcome::Resolved => {
            let form = match fields.get("form").map(String::as_str) {
                Some("documentation") => AnnotationForm::Documentation,
                Some("textual_representation") => AnnotationForm::TextualRepresentation,
                Some(value) => {
                    return Err(format!(
                        "{fallback_name}: unknown semantic element documentation form {value:?} (expected documentation or textual_representation)"
                    ));
                }
                None => {
                    return Err(format!(
                        "{fallback_name}: resolved semantic element documentation requires form"
                    ));
                }
            };
            let required_form = match collection {
                ElementDerivedDocumentationCollection::Documentation => {
                    AnnotationForm::Documentation
                }
                ElementDerivedDocumentationCollection::TextualRepresentation => {
                    AnnotationForm::TextualRepresentation
                }
            };
            if form != required_form {
                return Err(format!(
                    "{fallback_name}: semantic element documentation form does not match its manifest-owned collection"
                ));
            }
            let optional = |name: &str| -> Result<Option<String>, String> {
                let value = fields.get(name).ok_or_else(|| {
                    format!(
                        "{fallback_name}: resolved semantic element documentation requires {name}"
                    )
                })?;
                Ok((value != "none").then(|| value.clone()))
            };
            Some(ExpectedDocumentation {
                form,
                locale: optional("locale")?,
                language: optional("language")?,
                text: fields
                    .get("text")
                    .filter(|value| !value.is_empty())
                    .cloned()
                    .ok_or_else(|| {
                        format!(
                            "{fallback_name}: resolved semantic element documentation requires text"
                        )
                    })?,
            })
        }
        ElementDerivedDocumentationOutcome::Absent
        | ElementDerivedDocumentationOutcome::Incomplete => {
            if ["form", "locale", "language", "text"]
                .into_iter()
                .any(|field| fields.contains_key(field))
            {
                return Err(format!(
                    "{fallback_name}: {outcome:?} semantic element documentation must not declare form, locale, language, or text"
                ));
            }
            None
        }
    };
    Ok(ElementDerivedDocumentationExpectation {
        collection,
        source,
        expected,
        outcome,
    })
}

fn parse_namespace_derived_element_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<NamespaceDerivedElementExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: Namespace element expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "source", "target", "outcome"],
        "semantic Namespace element",
        fallback_name,
    )?;
    let collection = namespace_element_collection_for_rule(
        manifest,
        fields.get("rule_id").ok_or_else(|| {
            format!("{fallback_name}: semantic Namespace element requires rule_id")
        })?,
        fallback_name,
    )?;
    let source = fields
        .get("source")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| format!("{fallback_name}: semantic Namespace element requires source"))?;
    let outcome = NamespaceDerivedElementOutcome::parse(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic Namespace element requires outcome")
        })?,
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    match outcome {
        NamespaceDerivedElementOutcome::Resolved if target.as_deref().is_none_or(str::is_empty) => {
            return Err(format!(
                "{fallback_name}: resolved semantic Namespace element requires target"
            ));
        }
        NamespaceDerivedElementOutcome::Absent
        | NamespaceDerivedElementOutcome::Incomplete
        | NamespaceDerivedElementOutcome::Unsupported
            if target.is_some() =>
        {
            return Err(format!(
                "{fallback_name}: {outcome:?} semantic Namespace element must not declare target"
            ));
        }
        _ => {}
    }
    Ok(NamespaceDerivedElementExpectation {
        collection,
        source,
        target,
        outcome,
    })
}

fn parse_namespace_import_derived_element_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<NamespaceImportDerivedElementExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: NamespaceImport element expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "owner", "target", "provenance", "outcome"],
        "semantic NamespaceImport element",
        fallback_name,
    )?;
    let kind = namespace_import_element_kind_for_rule(
        manifest,
        fields.get("rule_id").ok_or_else(|| {
            format!("{fallback_name}: semantic NamespaceImport element requires rule_id")
        })?,
        fallback_name,
    )?;
    let owner = fields
        .get("owner")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| {
            format!("{fallback_name}: semantic NamespaceImport element requires owner")
        })?;
    let outcome = SemanticRelationshipOutcome::parse(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic NamespaceImport element requires outcome")
        })?,
        fallback_name,
    )?;
    let target = fields.get("target").cloned();
    let provenance = fields
        .get("provenance")
        .map(|value| parse_relationship_provenance(value, fallback_name))
        .transpose()?;
    validate_semantic_relationship_shape(
        outcome,
        target.as_deref(),
        provenance,
        "semantic NamespaceImport element",
        fallback_name,
    )?;
    Ok(NamespaceImportDerivedElementExpectation {
        kind,
        owner,
        target,
        provenance,
        outcome,
    })
}

fn parse_binding_connector_check_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<BindingConnectorCheckExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: BindingConnector check expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "outcome", "prerequisite"],
        "semantic BindingConnector check",
        fallback_name,
    )?;
    let rule = binding_connector_check_kind_for_rule(
        manifest,
        fields.get("rule_id").ok_or_else(|| {
            format!("{fallback_name}: semantic BindingConnector check requires rule_id")
        })?,
        fallback_name,
    )?;
    let outcome = parse_binding_connector_check_outcome(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic BindingConnector check requires outcome")
        })?,
        fields.get("prerequisite"),
        fallback_name,
    )?;
    Ok(BindingConnectorCheckExpectation { rule, outcome })
}

fn parse_redefinition_check_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<RedefinitionCheckExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: redefinition check expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "outcome", "prerequisite"],
        "semantic redefinition check",
        fallback_name,
    )?;
    let rule = redefinition_check_kind_for_rule(
        manifest,
        fields.get("rule_id").ok_or_else(|| {
            format!("{fallback_name}: semantic redefinition check requires rule_id")
        })?,
        fallback_name,
    )?;
    let outcome = parse_redefinition_check_outcome(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic redefinition check requires outcome")
        })?,
        fields.get("prerequisite"),
        fallback_name,
    )?;
    Ok(RedefinitionCheckExpectation { rule, outcome })
}

fn parse_specialization_check_expectation(
    expression: &AuthoredSexpr,
    fallback_name: &str,
    manifest: Option<&ConstraintManifest>,
) -> Result<SpecializationCheckExpectation, String> {
    let AuthoredSexpr::List(items) = expression else {
        return Err(format!(
            "{fallback_name}: specialization check expectation must be a list"
        ));
    };
    let fields = parse_semantic_assertion_fields(
        &items[1..],
        &["rule_id", "outcome", "prerequisite"],
        "semantic specialization check",
        fallback_name,
    )?;
    let rule = specialization_check_kind_for_rule(
        manifest,
        fields.get("rule_id").ok_or_else(|| {
            format!("{fallback_name}: semantic specialization check requires rule_id")
        })?,
        fallback_name,
    )?;
    let outcome = parse_specialization_check_outcome(
        fields.get("outcome").ok_or_else(|| {
            format!("{fallback_name}: semantic specialization check requires outcome")
        })?,
        fields.get("prerequisite"),
        fallback_name,
    )?;
    Ok(SpecializationCheckExpectation { rule, outcome })
}

fn parse_semantic_assertion_fields(
    fields: &[AuthoredSexpr],
    allowed: &[&str],
    description: &str,
    fallback_name: &str,
) -> Result<BTreeMap<String, String>, String> {
    let mut parsed = BTreeMap::new();
    for field in fields {
        let AuthoredSexpr::List(field_items) = field else {
            return Err(format!(
                "{fallback_name}: {description} field must be a list"
            ));
        };
        if field_items.len() != 2 {
            return Err(format!(
                "{fallback_name}: {description} fields require exactly one value"
            ));
        }
        let key = authored_atom(&field_items[0])
            .ok_or_else(|| format!("{fallback_name}: {description} field name must be an atom"))?;
        let value = authored_value(&field_items[1]).ok_or_else(|| {
            format!("{fallback_name}: {description} field value must be an atom or string")
        })?;
        if parsed.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!(
                "{fallback_name}: duplicate {description} field {key:?}"
            ));
        }
    }
    for key in parsed.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!(
                "{fallback_name}: unknown {description} field {key:?}"
            ));
        }
    }
    Ok(parsed)
}

fn validate_semantic_relationship_shape(
    outcome: SemanticRelationshipOutcome,
    target: Option<&str>,
    provenance: Option<RelationshipProvenance>,
    description: &str,
    fallback_name: &str,
) -> Result<(), String> {
    match outcome {
        SemanticRelationshipOutcome::Resolved => {
            if target.is_none_or(str::is_empty) || provenance.is_none() {
                return Err(format!(
                    "{fallback_name}: resolved {description} requires target and provenance"
                ));
            }
        }
        SemanticRelationshipOutcome::Absent => {
            if target.is_some() || provenance.is_none() {
                return Err(format!(
                    "{fallback_name}: absent {description} requires provenance and no target"
                ));
            }
        }
        SemanticRelationshipOutcome::Incomplete => {
            if target.is_some() || provenance.is_some() {
                return Err(format!(
                    "{fallback_name}: incomplete {description} must not declare target or provenance"
                ));
            }
        }
        SemanticRelationshipOutcome::Unresolved
        | SemanticRelationshipOutcome::Ambiguous
        | SemanticRelationshipOutcome::Unsupported => {
            if target.is_some() || provenance.is_none() {
                return Err(format!(
                    "{fallback_name}: {outcome:?} {description} requires provenance and no target"
                ));
            }
        }
    }
    Ok(())
}

fn parse_relationship_provenance(
    value: &str,
    fixture: &str,
) -> Result<RelationshipProvenance, String> {
    match value {
        "authored" => Ok(RelationshipProvenance::Authored),
        "implied" => Ok(RelationshipProvenance::Implied),
        _ => Err(format!(
            "{fixture}: unknown semantic relationship provenance {value:?} (expected authored or implied)"
        )),
    }
}

fn authored_atom(expression: &AuthoredSexpr) -> Option<&str> {
    match expression {
        AuthoredSexpr::Atom(value) => Some(value),
        _ => None,
    }
}

fn authored_value(expression: &AuthoredSexpr) -> Option<&str> {
    match expression {
        AuthoredSexpr::Atom(value) | AuthoredSexpr::String(value) => Some(value),
        AuthoredSexpr::List(_) => None,
    }
}

fn parse_authored_sexpr(input: &str) -> Result<AuthoredSexpr, String> {
    let mut parser = AuthoredSexprParser { input, offset: 0 };
    let expression = parser.expression()?;
    parser.whitespace();
    if parser.offset != input.len() {
        return Err("trailing input".to_string());
    }
    Ok(expression)
}

struct AuthoredSexprParser<'a> {
    input: &'a str,
    offset: usize,
}

impl AuthoredSexprParser<'_> {
    fn whitespace(&mut self) {
        while let Some(character) = self.input[self.offset..].chars().next() {
            if !character.is_whitespace() {
                break;
            }
            self.offset += character.len_utf8();
        }
    }

    fn expression(&mut self) -> Result<AuthoredSexpr, String> {
        self.whitespace();
        match self.input[self.offset..].chars().next() {
            Some('(') => self.list(),
            Some('"') => self.string(),
            Some(')') => Err("unexpected ')'".to_string()),
            Some(_) => self.atom(),
            None => Err("expected expression".to_string()),
        }
    }

    fn list(&mut self) -> Result<AuthoredSexpr, String> {
        self.offset += 1;
        let mut items = Vec::new();
        loop {
            self.whitespace();
            match self.input[self.offset..].chars().next() {
                Some(')') => {
                    self.offset += 1;
                    return Ok(AuthoredSexpr::List(items));
                }
                Some(_) => items.push(self.expression()?),
                None => return Err("unterminated list".to_string()),
            }
        }
    }

    fn atom(&mut self) -> Result<AuthoredSexpr, String> {
        let start = self.offset;
        while let Some(character) = self.input[self.offset..].chars().next() {
            if character.is_whitespace() || matches!(character, '(' | ')') {
                break;
            }
            self.offset += character.len_utf8();
        }
        (start != self.offset)
            .then(|| AuthoredSexpr::Atom(self.input[start..self.offset].to_string()))
            .ok_or_else(|| "expected atom".to_string())
    }

    fn string(&mut self) -> Result<AuthoredSexpr, String> {
        self.offset += 1;
        let mut value = String::new();
        while let Some(character) = self.input[self.offset..].chars().next() {
            self.offset += character.len_utf8();
            match character {
                '"' => return Ok(AuthoredSexpr::String(value)),
                '\\' => {
                    let escaped = self.input[self.offset..]
                        .chars()
                        .next()
                        .ok_or_else(|| "unterminated escape".to_string())?;
                    self.offset += escaped.len_utf8();
                    match escaped {
                        '"' => value.push('"'),
                        '\\' => value.push('\\'),
                        'n' => value.push('\n'),
                        _ => return Err(format!("unsupported escape \\{escaped}")),
                    }
                }
                _ => value.push(character),
            }
        }
        Err("unterminated string".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SemanticExpectationObservations {
    relationships: Vec<SemanticRelationshipObservation>,
    feature_derived_relationships: Vec<SemanticRelationshipObservation>,
    type_derived_relationships: Vec<SemanticRelationshipObservation>,
    type_derived_elements: Vec<TypeDerivedElementObservation>,
    type_derived_facts: Vec<TypeDerivedFactObservation>,
    action_derived_facts: Vec<ActionDerivedFactObservation>,
    definition_usage_derived: Vec<DefinitionUsageDerivedObservation>,
    requirement_derived_facts: Vec<RequirementDerivedFactObservation>,
    element_derived_owners: Vec<ElementDerivedOwnerObservation>,
    element_derived_documentation: Vec<ElementDerivedDocumentationObservation>,
    namespace_derived_elements: Vec<NamespaceDerivedElementObservation>,
    namespace_import_derived_elements: Vec<SemanticRelationshipObservation>,
    binding_connector_checks: Vec<BindingConnectorCheckObservation>,
    redefinition_checks: Vec<RedefinitionCheckObservation>,
    specialization_checks: Vec<SpecializationCheckObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ElementDerivedOwnerObservation {
    Owner {
        source: SymbolId,
        actual: SymbolId,
        expected: Option<SymbolId>,
    },
    Absent {
        source: SymbolId,
    },
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ElementDerivedDocumentationObservation {
    Values {
        source: SymbolId,
        values: Box<[Documentation]>,
        /// The authored text of each value, read through the publication at observation time.
        texts: Box<[String]>,
        expected: Option<ExpectedDocumentation>,
    },
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NamespaceDerivedElementObservation {
    Values {
        values: Box<[SymbolId]>,
        expected: Option<SymbolId>,
    },
    Incomplete,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeDerivedElementObservation {
    Values {
        values: Box<[SymbolId]>,
        expected: Option<SymbolId>,
    },
    Incomplete,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeDerivedFactObservation {
    Outcome {
        value: TypeDerivedFactOutcome,
        expected: Option<SymbolId>,
    },
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ActionDerivedFactObservation {
    Outcome {
        value: ActionDerivedFactOutcome,
        expected: Option<SymbolId>,
    },
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DefinitionUsageDerivedObservation {
    Outcome {
        value: DefinitionUsageDerivedOutcome,
        expected: Option<SymbolId>,
        membership_members: Box<[SymbolId]>,
    },
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RequirementDerivedFactObservation {
    Outcome {
        value: RequirementDerivedFactOutcome,
        expected: Option<SymbolId>,
    },
    Incomplete,
}

fn observe_semantic_expectations(
    model: &PublishedModel,
    expectations: &SemanticExpectations,
) -> Result<SemanticExpectationObservations, String> {
    Ok(SemanticExpectationObservations {
        relationships: expectations
            .relationships
            .iter()
            .map(|expectation| observe_semantic_relationship(model, expectation))
            .collect::<Result<_, _>>()?,
        feature_derived_relationships: expectations
            .feature_derived_relationships
            .iter()
            .map(|expectation| observe_feature_derived_relationship(model, expectation))
            .collect::<Result<_, _>>()?,
        type_derived_relationships: expectations
            .type_derived_relationships
            .iter()
            .map(|expectation| observe_type_derived_relationship(model, expectation))
            .collect::<Result<_, _>>()?,
        type_derived_elements: expectations
            .type_derived_elements
            .iter()
            .map(|expectation| observe_type_derived_element(model, expectation))
            .collect::<Result<_, _>>()?,
        type_derived_facts: expectations
            .type_derived_facts
            .iter()
            .map(|expectation| observe_type_derived_fact(model, expectation))
            .collect::<Result<_, _>>()?,
        action_derived_facts: expectations
            .action_derived_facts
            .iter()
            .map(|expectation| observe_action_derived_fact(model, expectation))
            .collect::<Result<_, _>>()?,
        definition_usage_derived: expectations
            .definition_usage_derived
            .iter()
            .map(|expectation| observe_definition_usage_derived(model, expectation))
            .collect::<Result<_, _>>()?,
        requirement_derived_facts: expectations
            .requirement_derived_facts
            .iter()
            .map(|expectation| observe_requirement_derived_fact(model, expectation))
            .collect::<Result<_, _>>()?,
        element_derived_owners: expectations
            .element_derived_owners
            .iter()
            .map(|expectation| observe_element_derived_owner(model, expectation))
            .collect::<Result<_, _>>()?,
        element_derived_documentation: expectations
            .element_derived_documentation
            .iter()
            .map(|expectation| observe_element_derived_documentation(model, expectation))
            .collect::<Result<_, _>>()?,
        namespace_derived_elements: expectations
            .namespace_derived_elements
            .iter()
            .map(|expectation| observe_namespace_derived_element(model, expectation))
            .collect::<Result<_, _>>()?,
        namespace_import_derived_elements: expectations
            .namespace_import_derived_elements
            .iter()
            .map(|expectation| observe_namespace_import_derived_element(model, expectation))
            .collect::<Result<_, _>>()?,
        binding_connector_checks: expectations
            .binding_connector_checks
            .iter()
            .map(|expectation| observe_binding_connector_check(model, expectation))
            .collect::<Result<_, _>>()?,
        redefinition_checks: expectations
            .redefinition_checks
            .iter()
            .map(|expectation| observe_redefinition_check(model, expectation))
            .collect::<Result<_, _>>()?,
        specialization_checks: expectations
            .specialization_checks
            .iter()
            .map(|expectation| observe_specialization_check(model, expectation))
            .collect::<Result<_, _>>()?,
    })
}

fn observe_semantic_relationship(
    model: &PublishedModel,
    expectation: &RelationshipExpectation,
) -> Result<SemanticRelationshipObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(SemanticRelationshipObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let inspection = match snapshot_answer(model.inspection().inspect(source)) {
        QueryAnswer::Resolved(inspection) => inspection,
        QueryAnswer::Unresolved => return Err("source inspection is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => return Err("source inspection is ambiguous".to_string()),
        QueryAnswer::Unsupported => return Err("source inspection is unsupported".to_string()),
        QueryAnswer::Recovery => return Err("source inspection is recovery".to_string()),
        QueryAnswer::Incomplete => return Ok(SemanticRelationshipObservation::Incomplete),
    };
    observe_expected_relationship(
        model,
        source,
        expectation.kind,
        expectation.target.as_deref(),
        expectation.provenance,
        expectation.outcome,
        inspection.relationships.as_ref(),
    )
}

/// Snapshot metadata renders publication completeness once for the whole model. Individual
/// expectation checks consume the independent typed answer without recreating completeness from
/// it, while unresolved, ambiguous, unsupported, recovery, and incomplete answers remain distinct.
fn snapshot_answer<T>(outcome: QueryOutcome<T>) -> QueryAnswer<T> {
    let _publication_completeness = outcome.completeness;
    outcome.answer
}

fn observe_feature_derived_relationship(
    model: &PublishedModel,
    expectation: &FeatureDerivedRelationshipExpectation,
) -> Result<SemanticRelationshipObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(SemanticRelationshipObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let relationships = match snapshot_answer(
        model
            .inspection()
            .feature_derived_relationships(source, expectation.collection),
    ) {
        QueryAnswer::Resolved(relationships) => relationships,
        QueryAnswer::Incomplete => return Ok(SemanticRelationshipObservation::Incomplete),
        QueryAnswer::Unresolved => {
            return Err("derived relationship collection is unresolved".to_string())
        }
        QueryAnswer::Ambiguous(_) => {
            return Err("derived relationship collection is ambiguous".to_string())
        }
        QueryAnswer::Unsupported => {
            return Err("derived relationship collection is unsupported".to_string())
        }
        QueryAnswer::Recovery => {
            return Err("derived relationship collection is recovery".to_string())
        }
    };
    observe_expected_relationship(
        model,
        source,
        expectation.kind,
        expectation.target.as_deref(),
        expectation.provenance,
        expectation.outcome,
        relationships.as_ref(),
    )
}

fn observe_type_derived_relationship(
    model: &PublishedModel,
    expectation: &TypeDerivedRelationshipExpectation,
) -> Result<SemanticRelationshipObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(SemanticRelationshipObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let relationships = match snapshot_answer(
        model
            .inspection()
            .type_derived_relationships(source, expectation.collection),
    ) {
        QueryAnswer::Resolved(relationships) => relationships,
        QueryAnswer::Incomplete => return Ok(SemanticRelationshipObservation::Incomplete),
        QueryAnswer::Unresolved => {
            return Err("Type derived relationship collection is unresolved".to_string())
        }
        QueryAnswer::Ambiguous(_) => {
            return Err("Type derived relationship collection is ambiguous".to_string())
        }
        QueryAnswer::Unsupported => {
            return Err("Type derived relationship collection is unsupported".to_string())
        }
        QueryAnswer::Recovery => {
            return Err("Type derived relationship collection is recovery".to_string())
        }
    };
    observe_expected_relationship(
        model,
        source,
        expectation.kind,
        expectation.target.as_deref(),
        expectation.provenance,
        expectation.outcome,
        relationships.as_ref(),
    )
}

fn observe_type_derived_element(
    model: &PublishedModel,
    expectation: &TypeDerivedElementExpectation,
) -> Result<TypeDerivedElementObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(TypeDerivedElementObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let values = match snapshot_answer(
        model
            .inspection()
            .type_derived_elements(source, expectation.collection),
    ) {
        QueryAnswer::Resolved(values) => values,
        QueryAnswer::Incomplete => return Ok(TypeDerivedElementObservation::Incomplete),
        QueryAnswer::Unsupported => return Ok(TypeDerivedElementObservation::Unsupported),
        QueryAnswer::Unresolved => return Err("Type element query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => return Err("Type element query is ambiguous".to_string()),
        QueryAnswer::Recovery => return Err("Type element query is recovery".to_string()),
    };
    let expected = expectation
        .target
        .as_deref()
        .map(|target| {
            resolve_semantic_identity(model, target)
                .map_err(|status| format!("target reference is {}", status.description()))
        })
        .transpose()?;
    Ok(TypeDerivedElementObservation::Values { values, expected })
}

fn observe_type_derived_fact(
    model: &PublishedModel,
    expectation: &TypeDerivedFactExpectation,
) -> Result<TypeDerivedFactObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(TypeDerivedFactObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let value = match snapshot_answer(
        model
            .inspection()
            .type_derived_fact(source, expectation.collection),
    ) {
        QueryAnswer::Resolved(value) => value,
        QueryAnswer::Incomplete => return Ok(TypeDerivedFactObservation::Incomplete),
        QueryAnswer::Unresolved => return Err("Type fact query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => return Err("Type fact query is ambiguous".to_string()),
        QueryAnswer::Unsupported => return Err("Type fact query is unsupported".to_string()),
        QueryAnswer::Recovery => return Err("Type fact query is recovery".to_string()),
    };
    let expected = expectation
        .target
        .as_deref()
        .map(|target| {
            resolve_semantic_identity(model, target)
                .map_err(|status| format!("target reference is {}", status.description()))
        })
        .transpose()?;
    Ok(TypeDerivedFactObservation::Outcome { value, expected })
}

fn observe_action_derived_fact(
    model: &PublishedModel,
    expectation: &ActionDerivedFactExpectation,
) -> Result<ActionDerivedFactObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(ActionDerivedFactObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let value = match snapshot_answer(
        model
            .inspection()
            .action_derived_fact(source, expectation.collection),
    ) {
        QueryAnswer::Resolved(value) => value,
        QueryAnswer::Incomplete => return Ok(ActionDerivedFactObservation::Incomplete),
        QueryAnswer::Unresolved => return Err("Action fact query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => return Err("Action fact query is ambiguous".to_string()),
        QueryAnswer::Unsupported => return Err("Action fact query is unsupported".to_string()),
        QueryAnswer::Recovery => return Err("Action fact query is recovery".to_string()),
    };
    let expected = expectation
        .target
        .as_deref()
        .map(|target| {
            resolve_semantic_identity(model, target)
                .map_err(|status| format!("target reference is {}", status.description()))
        })
        .transpose()?;
    Ok(ActionDerivedFactObservation::Outcome { value, expected })
}

fn observe_definition_usage_derived(
    model: &PublishedModel,
    expectation: &DefinitionUsageDerivedExpectation,
) -> Result<DefinitionUsageDerivedObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(DefinitionUsageDerivedObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let value = match snapshot_answer(
        model
            .inspection()
            .definition_usage_derived(source, expectation.kind),
    ) {
        QueryAnswer::Resolved(value) => value,
        QueryAnswer::Incomplete => return Ok(DefinitionUsageDerivedObservation::Incomplete),
        QueryAnswer::Unresolved => {
            return Err("Definition/Usage derivation query is unresolved".to_string())
        }
        QueryAnswer::Ambiguous(_) => {
            return Err("Definition/Usage derivation query is ambiguous".to_string())
        }
        QueryAnswer::Unsupported => {
            return Err("Definition/Usage derivation query is unsupported".to_string())
        }
        QueryAnswer::Recovery => {
            return Err("Definition/Usage derivation query is recovery".to_string())
        }
    };
    let expected = expectation
        .target
        .as_deref()
        .map(|target| {
            resolve_semantic_identity(model, target)
                .map_err(|status| format!("target reference is {}", status.description()))
        })
        .transpose()?;
    let membership_members = match &value {
        DefinitionUsageDerivedOutcome::Memberships(values) => values
            .iter()
            .filter_map(
                |identity| match model.inspection().membership(*identity).answer {
                    QueryAnswer::Resolved(membership) => Some(membership.member),
                    _ => None,
                },
            )
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        _ => Box::new([]),
    };
    Ok(DefinitionUsageDerivedObservation::Outcome {
        value,
        expected,
        membership_members,
    })
}

fn observe_requirement_derived_fact(
    model: &PublishedModel,
    expectation: &RequirementDerivedFactExpectation,
) -> Result<RequirementDerivedFactObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(RequirementDerivedFactObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let value = match snapshot_answer(
        model
            .inspection()
            .requirement_derived_fact(source, expectation.collection),
    ) {
        QueryAnswer::Resolved(value) => value,
        QueryAnswer::Incomplete => return Ok(RequirementDerivedFactObservation::Incomplete),
        QueryAnswer::Unresolved => return Err("Requirements fact query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => return Err("Requirements fact query is ambiguous".to_string()),
        QueryAnswer::Unsupported => {
            return Err("Requirements fact query is unsupported".to_string())
        }
        QueryAnswer::Recovery => return Err("Requirements fact query is recovery".to_string()),
    };
    let expected = expectation
        .target
        .as_deref()
        .map(|target| {
            resolve_semantic_identity(model, target)
                .map_err(|status| format!("target reference is {}", status.description()))
        })
        .transpose()?;
    Ok(RequirementDerivedFactObservation::Outcome { value, expected })
}

fn observe_element_derived_owner(
    model: &PublishedModel,
    expectation: &ElementDerivedOwnerExpectation,
) -> Result<ElementDerivedOwnerObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(ElementDerivedOwnerObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let ElementDerivedOwnerKind::Owner = expectation.kind;
    let owner = match snapshot_answer(model.inspection().derived_element_owner(source)) {
        QueryAnswer::Resolved(owner) => owner,
        QueryAnswer::Incomplete => return Ok(ElementDerivedOwnerObservation::Incomplete),
        QueryAnswer::Unresolved => return Err("element owner query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => return Err("element owner query is ambiguous".to_string()),
        QueryAnswer::Unsupported => return Err("element owner query is unsupported".to_string()),
        QueryAnswer::Recovery => return Err("element owner query is recovery".to_string()),
    };
    match owner {
        DerivedElementOwner::Owner(actual) => {
            let expected = expectation
                .owner
                .as_deref()
                .map(|owner| {
                    resolve_semantic_identity(model, owner)
                        .map_err(|status| format!("owner reference is {}", status.description()))
                })
                .transpose()?;
            Ok(ElementDerivedOwnerObservation::Owner {
                source,
                actual,
                expected,
            })
        }
        DerivedElementOwner::NoOwner => Ok(ElementDerivedOwnerObservation::Absent { source }),
    }
}

fn observe_element_derived_documentation(
    model: &PublishedModel,
    expectation: &ElementDerivedDocumentationExpectation,
) -> Result<ElementDerivedDocumentationObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(ElementDerivedDocumentationObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let values = match snapshot_answer(
        model
            .inspection()
            .element_derived_documentation(source, expectation.collection),
    ) {
        QueryAnswer::Resolved(values) => values,
        QueryAnswer::Incomplete => return Ok(ElementDerivedDocumentationObservation::Incomplete),
        QueryAnswer::Unresolved => {
            return Err("element documentation query is unresolved".to_string())
        }
        QueryAnswer::Ambiguous(_) => {
            return Err("element documentation query is ambiguous".to_string())
        }
        QueryAnswer::Unsupported => {
            return Err("element documentation query is unsupported".to_string())
        }
        QueryAnswer::Recovery => return Err("element documentation query is recovery".to_string()),
    };
    let texts = values
        .iter()
        .map(|value| model.text(value.text).unwrap_or_default().to_owned())
        .collect();
    Ok(ElementDerivedDocumentationObservation::Values {
        source,
        values,
        texts,
        expected: expectation.expected.clone(),
    })
}

fn observe_namespace_derived_element(
    model: &PublishedModel,
    expectation: &NamespaceDerivedElementExpectation,
) -> Result<NamespaceDerivedElementObservation, String> {
    let source = match resolve_semantic_identity(model, &expectation.source) {
        Ok(source) => source,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(NamespaceDerivedElementObservation::Incomplete)
        }
        Err(status) => return Err(format!("source reference is {}", status.description())),
    };
    let values = match snapshot_answer(
        model
            .inspection()
            .namespace_derived_elements(source, expectation.collection),
    ) {
        QueryAnswer::Resolved(values) => values,
        QueryAnswer::Incomplete => return Ok(NamespaceDerivedElementObservation::Incomplete),
        QueryAnswer::Unsupported => return Ok(NamespaceDerivedElementObservation::Unsupported),
        QueryAnswer::Unresolved => return Err("Namespace element query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => return Err("Namespace element query is ambiguous".to_string()),
        QueryAnswer::Recovery => return Err("Namespace element query is recovery".to_string()),
    };
    let expected = expectation
        .target
        .as_deref()
        .map(|target| {
            resolve_semantic_identity(model, target)
                .map_err(|status| format!("target reference is {}", status.description()))
        })
        .transpose()?;
    Ok(NamespaceDerivedElementObservation::Values { values, expected })
}

fn observe_namespace_import_derived_element(
    model: &PublishedModel,
    expectation: &NamespaceImportDerivedElementExpectation,
) -> Result<SemanticRelationshipObservation, String> {
    let owner = match resolve_semantic_identity(model, &expectation.owner) {
        Ok(owner) => owner,
        Err(SemanticIdentityStatus::Incomplete) => {
            return Ok(SemanticRelationshipObservation::Incomplete)
        }
        Err(status) => return Err(format!("owner reference is {}", status.description())),
    };
    let NamespaceImportDerivedElementKind::ImportedElement = expectation.kind;
    let values = match snapshot_answer(model.inspection().namespace_import_derived_elements(owner))
    {
        QueryAnswer::Resolved(values) => values,
        QueryAnswer::Incomplete => return Ok(SemanticRelationshipObservation::Incomplete),
        QueryAnswer::Unresolved => {
            return Err("NamespaceImport element query is unresolved".to_string())
        }
        QueryAnswer::Ambiguous(_) => {
            return Err("NamespaceImport element query is ambiguous".to_string())
        }
        QueryAnswer::Unsupported => {
            return Err("NamespaceImport element query is unsupported".to_string())
        }
        QueryAnswer::Recovery => {
            return Err("NamespaceImport element query is recovery".to_string())
        }
    };
    let relationships = values
        .iter()
        .map(|value| value.relationship.clone())
        .collect::<Vec<_>>();
    observe_expected_relationship(
        model,
        owner,
        SemanticRelationshipKind::NamespaceImport,
        expectation.target.as_deref(),
        expectation.provenance,
        expectation.outcome,
        &relationships,
    )
}

fn observe_binding_connector_check(
    model: &PublishedModel,
    expectation: &BindingConnectorCheckExpectation,
) -> Result<BindingConnectorCheckObservation, String> {
    match snapshot_answer(
        model
            .inspection()
            .binding_connector_validation(expectation.rule),
    ) {
        QueryAnswer::Resolved(outcome) => Ok(BindingConnectorCheckObservation::Outcome(outcome)),
        QueryAnswer::Incomplete => Ok(BindingConnectorCheckObservation::Incomplete),
        QueryAnswer::Unresolved => Err("BindingConnector check query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => Err("BindingConnector check query is ambiguous".to_string()),
        QueryAnswer::Unsupported => Err("BindingConnector check query is unsupported".to_string()),
        QueryAnswer::Recovery => Err("BindingConnector check query is recovery".to_string()),
    }
}

fn observe_redefinition_check(
    model: &PublishedModel,
    expectation: &RedefinitionCheckExpectation,
) -> Result<RedefinitionCheckObservation, String> {
    match snapshot_answer(model.inspection().redefinition_check(expectation.rule)) {
        QueryAnswer::Resolved(outcome) => Ok(RedefinitionCheckObservation::Outcome(outcome)),
        QueryAnswer::Incomplete => Ok(RedefinitionCheckObservation::Incomplete),
        QueryAnswer::Unresolved => Err("redefinition check query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => Err("redefinition check query is ambiguous".to_string()),
        QueryAnswer::Unsupported => Err("redefinition check query is unsupported".to_string()),
        QueryAnswer::Recovery => Err("redefinition check query is recovery".to_string()),
    }
}

fn observe_specialization_check(
    model: &PublishedModel,
    expectation: &SpecializationCheckExpectation,
) -> Result<SpecializationCheckObservation, String> {
    match snapshot_answer(model.inspection().specialization_check(expectation.rule)) {
        QueryAnswer::Resolved(outcome) => Ok(SpecializationCheckObservation::Outcome(outcome)),
        QueryAnswer::Incomplete => Ok(SpecializationCheckObservation::Incomplete),
        QueryAnswer::Unresolved => Err("specialization check query is unresolved".to_string()),
        QueryAnswer::Ambiguous(_) => Err("specialization check query is ambiguous".to_string()),
        QueryAnswer::Unsupported => Err("specialization check query is unsupported".to_string()),
        QueryAnswer::Recovery => Err("specialization check query is recovery".to_string()),
    }
}

fn observe_expected_relationship(
    model: &PublishedModel,
    source: SymbolId,
    kind: SemanticRelationshipKind,
    expected_target_name: Option<&str>,
    expected_provenance: Option<RelationshipProvenance>,
    outcome: SemanticRelationshipOutcome,
    relationships: &[sysml_query::resolved_slice::ElementRelationship],
) -> Result<SemanticRelationshipObservation, String> {
    let expected_target = expected_target_name
        .map(|target| {
            resolve_semantic_identity(model, target)
                .map_err(|status| format!("target reference is {}", status.description()))
        })
        .transpose()?;
    let provenance = expected_provenance.unwrap_or(RelationshipProvenance::Authored);
    let relationship = relationships.iter().find(|relationship| {
        if relationship.kind != kind.query_name() || relationship.provenance != provenance {
            return false;
        }
        match outcome {
            SemanticRelationshipOutcome::Resolved => {
                matches!(
                    (&relationship.target, &expected_target),
                    (RelationshipTarget::Resolved(actual), Some(expected)) if actual == expected
                )
            }
            SemanticRelationshipOutcome::Unresolved => {
                matches!(relationship.target, RelationshipTarget::Unresolved)
            }
            SemanticRelationshipOutcome::Ambiguous => {
                matches!(relationship.target, RelationshipTarget::Ambiguous(_))
            }
            SemanticRelationshipOutcome::Unsupported => {
                matches!(relationship.target, RelationshipTarget::Unsupported)
            }
            SemanticRelationshipOutcome::Absent | SemanticRelationshipOutcome::Incomplete => true,
        }
    });
    Ok(match relationship {
        Some(relationship) => SemanticRelationshipObservation::Relationship {
            source,
            kind,
            provenance: relationship.provenance,
            target: relationship.target.clone(),
            expected_target,
        },
        None => SemanticRelationshipObservation::Absent {
            source,
            kind,
            provenance,
        },
    })
}

fn resolve_semantic_identity(
    model: &PublishedModel,
    qualified_name: &str,
) -> Result<SymbolId, SemanticIdentityStatus> {
    match model
        .inspection()
        .resolve_qualified_reference(&QualifiedElementReference {
            document: None,
            qualified_name: qualified_name.into(),
            expected_kind: None,
        }) {
        QualifiedReferenceOutcome::Resolved(target)
        | QualifiedReferenceOutcome::Recovered(target)
        | QualifiedReferenceOutcome::UnsupportedWith(target) => Ok(target.identity),
        QualifiedReferenceOutcome::Unresolved => Err(SemanticIdentityStatus::Unresolved),
        QualifiedReferenceOutcome::Ambiguous(_) => Err(SemanticIdentityStatus::Ambiguous),
        QualifiedReferenceOutcome::WrongKind(_) => Err(SemanticIdentityStatus::WrongKind),
        QualifiedReferenceOutcome::Unsupported => Err(SemanticIdentityStatus::Unsupported),
        QualifiedReferenceOutcome::Recovery => Err(SemanticIdentityStatus::Recovery),
        QualifiedReferenceOutcome::Incomplete => Err(SemanticIdentityStatus::Incomplete),
    }
}

fn compare_semantic_expectations(
    expectations: &SemanticExpectations,
    observations: &SemanticExpectationObservations,
) -> Result<(), String> {
    for (expectation, observation) in expectations
        .relationships
        .iter()
        .zip(&observations.relationships)
    {
        compare_semantic_relationship_observation(
            expectation.outcome,
            &expectation.source,
            expectation.kind,
            observation,
        )?;
    }
    for (expectation, observation) in expectations
        .feature_derived_relationships
        .iter()
        .zip(&observations.feature_derived_relationships)
    {
        compare_semantic_relationship_observation(
            expectation.outcome,
            &expectation.source,
            expectation.kind,
            observation,
        )?;
    }
    for (expectation, observation) in expectations
        .type_derived_relationships
        .iter()
        .zip(&observations.type_derived_relationships)
    {
        compare_semantic_relationship_observation(
            expectation.outcome,
            &expectation.source,
            expectation.kind,
            observation,
        )?;
    }
    for (expectation, observation) in expectations
        .type_derived_elements
        .iter()
        .zip(&observations.type_derived_elements)
    {
        compare_type_derived_element_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .type_derived_facts
        .iter()
        .zip(&observations.type_derived_facts)
    {
        compare_type_derived_fact_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .action_derived_facts
        .iter()
        .zip(&observations.action_derived_facts)
    {
        compare_action_derived_fact_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .definition_usage_derived
        .iter()
        .zip(&observations.definition_usage_derived)
    {
        compare_definition_usage_derived_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .requirement_derived_facts
        .iter()
        .zip(&observations.requirement_derived_facts)
    {
        compare_requirement_derived_fact_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .element_derived_owners
        .iter()
        .zip(&observations.element_derived_owners)
    {
        compare_element_derived_owner_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .element_derived_documentation
        .iter()
        .zip(&observations.element_derived_documentation)
    {
        compare_element_derived_documentation_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .namespace_derived_elements
        .iter()
        .zip(&observations.namespace_derived_elements)
    {
        compare_namespace_derived_element_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .namespace_import_derived_elements
        .iter()
        .zip(&observations.namespace_import_derived_elements)
    {
        compare_semantic_relationship_observation(
            expectation.outcome,
            &expectation.owner,
            SemanticRelationshipKind::NamespaceImport,
            observation,
        )?;
    }
    for (expectation, observation) in expectations
        .binding_connector_checks
        .iter()
        .zip(&observations.binding_connector_checks)
    {
        compare_binding_connector_check_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .redefinition_checks
        .iter()
        .zip(&observations.redefinition_checks)
    {
        compare_redefinition_check_observation(expectation, observation)?;
    }
    for (expectation, observation) in expectations
        .specialization_checks
        .iter()
        .zip(&observations.specialization_checks)
    {
        compare_specialization_check_observation(expectation, observation)?;
    }
    Ok(())
}

fn compare_binding_connector_check_observation(
    expectation: &BindingConnectorCheckExpectation,
    observation: &BindingConnectorCheckObservation,
) -> Result<(), String> {
    let expected = match &expectation.outcome {
        BindingConnectorCheckOutcome::Satisfied => BindingConnectorValidationOutcome::Satisfied,
        BindingConnectorCheckOutcome::Violated => BindingConnectorValidationOutcome::Violated,
        BindingConnectorCheckOutcome::Unresolved => BindingConnectorValidationOutcome::Unresolved,
        BindingConnectorCheckOutcome::Unsupported(prerequisite) => {
            BindingConnectorValidationOutcome::Unsupported {
                prerequisite: *prerequisite,
            }
        }
    };
    match observation {
        BindingConnectorCheckObservation::Outcome(actual) if actual == &expected => Ok(()),
        BindingConnectorCheckObservation::Incomplete => Err(format!(
            "semantic BindingConnector check expectation for {:?} observed an incomplete publication",
            expectation.rule
        )),
        BindingConnectorCheckObservation::Outcome(_) => Err(format!(
            "semantic BindingConnector check expectation for {:?} did not match its typed outcome",
            expectation.rule
        )),
    }
}

fn compare_redefinition_check_observation(
    expectation: &RedefinitionCheckExpectation,
    observation: &RedefinitionCheckObservation,
) -> Result<(), String> {
    let expected = match expectation.outcome {
        RedefinitionCheckExpectationOutcome::Satisfied => RedefinitionCheckOutcome::Satisfied,
        RedefinitionCheckExpectationOutcome::Violated => RedefinitionCheckOutcome::Violated,
        RedefinitionCheckExpectationOutcome::Unresolved => RedefinitionCheckOutcome::Unresolved,
        RedefinitionCheckExpectationOutcome::Unsupported(prerequisite) => {
            RedefinitionCheckOutcome::Unsupported { prerequisite }
        }
    };
    match observation {
        RedefinitionCheckObservation::Outcome(actual) if actual == &expected => Ok(()),
        RedefinitionCheckObservation::Incomplete => Err(format!(
            "semantic redefinition check expectation for {:?} observed an incomplete publication",
            expectation.rule
        )),
        RedefinitionCheckObservation::Outcome(_) => Err(format!(
            "semantic redefinition check expectation for {:?} did not match its typed outcome",
            expectation.rule
        )),
    }
}

fn compare_specialization_check_observation(
    expectation: &SpecializationCheckExpectation,
    observation: &SpecializationCheckObservation,
) -> Result<(), String> {
    let expected = match expectation.outcome {
        SpecializationCheckExpectationOutcome::Satisfied => SpecializationCheckOutcome::Satisfied,
        SpecializationCheckExpectationOutcome::Violated => SpecializationCheckOutcome::Violated,
        SpecializationCheckExpectationOutcome::Unresolved => SpecializationCheckOutcome::Unresolved,
        SpecializationCheckExpectationOutcome::Unsupported(prerequisite) => {
            SpecializationCheckOutcome::Unsupported { prerequisite }
        }
    };
    match observation {
        SpecializationCheckObservation::Outcome(actual) if actual == &expected => Ok(()),
        SpecializationCheckObservation::Incomplete => Err(format!(
            "semantic specialization check expectation for {:?} observed an incomplete publication",
            expectation.rule
        )),
        SpecializationCheckObservation::Outcome(_) => Err(format!(
            "semantic specialization check expectation for {:?} did not match its typed outcome",
            expectation.rule
        )),
    }
}

fn compare_element_derived_owner_observation(
    expectation: &ElementDerivedOwnerExpectation,
    observation: &ElementDerivedOwnerObservation,
) -> Result<(), String> {
    match (expectation.outcome, observation) {
        (ElementDerivedOwnerOutcome::Incomplete, ElementDerivedOwnerObservation::Incomplete) => {}
        (ElementDerivedOwnerOutcome::Absent, ElementDerivedOwnerObservation::Absent { .. }) => {}
        (
            ElementDerivedOwnerOutcome::Resolved,
            ElementDerivedOwnerObservation::Owner {
                actual,
                expected: Some(expected),
                ..
            },
        ) if actual == expected => {}
        _ => {
            return Err(format!(
                "semantic element owner expectation for {} did not match its typed outcome",
                expectation.source
            ));
        }
    }
    Ok(())
}

fn compare_element_derived_documentation_observation(
    expectation: &ElementDerivedDocumentationExpectation,
    observation: &ElementDerivedDocumentationObservation,
) -> Result<(), String> {
    match (expectation.outcome, observation) {
        (
            ElementDerivedDocumentationOutcome::Incomplete,
            ElementDerivedDocumentationObservation::Incomplete,
        ) => Ok(()),
        (
            ElementDerivedDocumentationOutcome::Absent,
            ElementDerivedDocumentationObservation::Values { values, .. },
        ) if values.is_empty() => Ok(()),
        (
            ElementDerivedDocumentationOutcome::Resolved,
            ElementDerivedDocumentationObservation::Values {
                values,
                texts,
                expected: Some(expected),
                ..
            },
        ) if values.iter().zip(texts.iter()).any(|(actual, text)| {
            actual.form == expected.form
                && actual.locale.as_deref() == expected.locale.as_deref()
                && actual.language.as_deref() == expected.language.as_deref()
                && text.as_str() == expected.text
        }) =>
        {
            Ok(())
        }
        _ => Err(format!(
            "semantic element documentation expectation for {} did not match its typed outcome",
            expectation.source
        )),
    }
}

fn compare_namespace_derived_element_observation(
    expectation: &NamespaceDerivedElementExpectation,
    observation: &NamespaceDerivedElementObservation,
) -> Result<(), String> {
    match (expectation.outcome, observation) {
        (
            NamespaceDerivedElementOutcome::Incomplete,
            NamespaceDerivedElementObservation::Incomplete,
        )
        | (
            NamespaceDerivedElementOutcome::Unsupported,
            NamespaceDerivedElementObservation::Unsupported,
        ) => Ok(()),
        (
            NamespaceDerivedElementOutcome::Absent,
            NamespaceDerivedElementObservation::Values { values, .. },
        ) if values.is_empty() => Ok(()),
        (
            NamespaceDerivedElementOutcome::Resolved,
            NamespaceDerivedElementObservation::Values {
                values,
                expected: Some(expected),
            },
        ) if values.iter().any(|actual| actual == expected) => Ok(()),
        _ => Err(format!(
            "semantic Namespace element expectation for {} did not match its typed outcome",
            expectation.source
        )),
    }
}

fn compare_type_derived_element_observation(
    expectation: &TypeDerivedElementExpectation,
    observation: &TypeDerivedElementObservation,
) -> Result<(), String> {
    match (expectation.outcome, observation) {
        (TypeDerivedElementOutcome::Incomplete, TypeDerivedElementObservation::Incomplete)
        | (TypeDerivedElementOutcome::Unsupported, TypeDerivedElementObservation::Unsupported) => {
            Ok(())
        }
        (
            TypeDerivedElementOutcome::Absent,
            TypeDerivedElementObservation::Values { values, .. },
        ) if values.is_empty() => Ok(()),
        (
            TypeDerivedElementOutcome::Resolved,
            TypeDerivedElementObservation::Values {
                values,
                expected: Some(expected),
            },
        ) if values.iter().any(|actual| actual == expected) => Ok(()),
        _ => Err(format!(
            "semantic Type element expectation for {} did not match its typed outcome",
            expectation.source
        )),
    }
}

fn compare_type_derived_fact_observation(
    expectation: &TypeDerivedFactExpectation,
    observation: &TypeDerivedFactObservation,
) -> Result<(), String> {
    match (expectation.outcome, observation) {
        (TypeDerivedElementOutcome::Incomplete, TypeDerivedFactObservation::Incomplete) => Ok(()),
        (
            TypeDerivedElementOutcome::Unsupported,
            TypeDerivedFactObservation::Outcome {
                value: TypeDerivedFactOutcome::Unsupported { .. },
                ..
            },
        ) => Ok(()),
        (
            TypeDerivedElementOutcome::Absent,
            TypeDerivedFactObservation::Outcome {
                value: TypeDerivedFactOutcome::Values(values),
                ..
            },
        ) if values.is_empty() => Ok(()),
        (
            TypeDerivedElementOutcome::Resolved,
            TypeDerivedFactObservation::Outcome {
                value: TypeDerivedFactOutcome::Values(values),
                expected: Some(expected),
            },
        ) if values.iter().any(|value| match value {
            TypeDerivedFactValue::Feature(actual) => actual == expected,
            TypeDerivedFactValue::FeatureMembership { member } => member == expected,
            TypeDerivedFactValue::Conjugator { original_type } => original_type == expected,
            TypeDerivedFactValue::Multiplicity(_) => false,
        }) =>
        {
            Ok(())
        }
        (
            TypeDerivedElementOutcome::Resolved,
            TypeDerivedFactObservation::Outcome {
                value: TypeDerivedFactOutcome::Values(values),
                expected: None,
            },
        ) if expectation.collection == TypeDerivedFactCollection::Multiplicity
            && values
                .iter()
                .any(|value| matches!(value, TypeDerivedFactValue::Multiplicity(_))) =>
        {
            Ok(())
        }
        _ => Err(format!(
            "semantic Type fact expectation for {} did not match its typed outcome",
            expectation.source
        )),
    }
}

fn compare_action_derived_fact_observation(
    expectation: &ActionDerivedFactExpectation,
    observation: &ActionDerivedFactObservation,
) -> Result<(), String> {
    match (expectation.outcome, observation) {
        (TypeDerivedElementOutcome::Incomplete, ActionDerivedFactObservation::Incomplete) => Ok(()),
        (
            TypeDerivedElementOutcome::Unsupported,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Unsupported { .. },
                ..
            },
        ) => Ok(()),
        (
            TypeDerivedElementOutcome::Absent,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Values(values),
                ..
            },
        ) if values.is_empty() => Ok(()),
        (
            TypeDerivedElementOutcome::Absent,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Arguments(values),
                ..
            },
        ) if values.is_empty() => Ok(()),
        (
            TypeDerivedElementOutcome::Absent,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Parameters(values),
                ..
            },
        ) if values.is_empty() => Ok(()),
        (
            TypeDerivedElementOutcome::Absent,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::OwnedMembershipMembers(values),
                ..
            },
        ) if values.is_empty() => Ok(()),
        (
            TypeDerivedElementOutcome::Resolved,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Arguments(values),
                expected: None,
            },
        ) if expectation
            .position
            .is_some_and(|position| values.iter().any(|actual| actual.position == position)) =>
        {
            Ok(())
        }
        (
            TypeDerivedElementOutcome::Resolved,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Parameters(values),
                expected: None,
            },
        ) if expectation
            .position
            .is_some_and(|position| values.iter().any(|actual| actual.position == position)) =>
        {
            Ok(())
        }
        (
            TypeDerivedElementOutcome::Resolved,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Values(values),
                expected: Some(expected),
            },
        ) if values.iter().any(|actual| actual == expected) => Ok(()),
        (
            TypeDerivedElementOutcome::Resolved,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::OwnedMembershipMembers(values),
                expected: Some(expected),
            },
        ) if values.iter().any(|actual| actual.member == *expected) => Ok(()),
        (
            TypeDerivedElementOutcome::Resolved,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Values(values),
                expected: None,
            },
        ) if expectation.position.is_none() && !values.is_empty() => Ok(()),
        (
            TypeDerivedElementOutcome::Resolved,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Parameters(values),
                expected: None,
            },
        ) if expectation.position.is_none() && !values.is_empty() => Ok(()),
        (
            TypeDerivedElementOutcome::Resolved,
            ActionDerivedFactObservation::Outcome {
                value: ActionDerivedFactOutcome::Arguments(values),
                expected: None,
            },
        ) if expectation.position.is_none() && !values.is_empty() => Ok(()),
        _ => Err(format!(
            "semantic Action fact expectation for {} did not match its typed outcome",
            expectation.source
        )),
    }
}

fn compare_definition_usage_derived_observation(
    expectation: &DefinitionUsageDerivedExpectation,
    observation: &DefinitionUsageDerivedObservation,
) -> Result<(), String> {
    match (&expectation.outcome, observation) {
        (
            DefinitionUsageDerivedExpectationOutcome::Incomplete,
            DefinitionUsageDerivedObservation::Incomplete,
        ) => Ok(()),
        (
            DefinitionUsageDerivedExpectationOutcome::Unsupported(prerequisite),
            DefinitionUsageDerivedObservation::Outcome {
                value: DefinitionUsageDerivedOutcome::Unsupported { prerequisite: actual },
                ..
            },
        ) if actual == prerequisite => Ok(()),
        (
            DefinitionUsageDerivedExpectationOutcome::Absent,
            DefinitionUsageDerivedObservation::Outcome {
                value: DefinitionUsageDerivedOutcome::Elements(values),
                ..
            },
        ) if values.is_empty() => Ok(()),
        (
            DefinitionUsageDerivedExpectationOutcome::Absent,
            DefinitionUsageDerivedObservation::Outcome {
                value: DefinitionUsageDerivedOutcome::Memberships(values),
                ..
            },
        ) if values.is_empty() => Ok(()),
        (
            DefinitionUsageDerivedExpectationOutcome::Resolved,
            DefinitionUsageDerivedObservation::Outcome {
                value: DefinitionUsageDerivedOutcome::Elements(values),
                expected: Some(expected),
                ..
            },
        ) if values.iter().any(|actual| actual == expected) => Ok(()),
        (
            DefinitionUsageDerivedExpectationOutcome::Resolved,
            DefinitionUsageDerivedObservation::Outcome {
                value: DefinitionUsageDerivedOutcome::Memberships(_),
                expected: Some(expected),
                membership_members,
            },
        ) if membership_members.iter().any(|actual| actual == expected) => Ok(()),
        (
            DefinitionUsageDerivedExpectationOutcome::True,
            DefinitionUsageDerivedObservation::Outcome {
                value: DefinitionUsageDerivedOutcome::Boolean(true),
                ..
            },
        ) => Ok(()),
        (
            DefinitionUsageDerivedExpectationOutcome::False,
            DefinitionUsageDerivedObservation::Outcome {
                value: DefinitionUsageDerivedOutcome::Boolean(false),
                ..
            },
        ) => Ok(()),
        _ => Err(format!(
            "semantic Definition/Usage derivation expectation for {} did not match its typed outcome",
            expectation.source
        )),
    }
}

fn compare_requirement_derived_fact_observation(
    expectation: &RequirementDerivedFactExpectation,
    observation: &RequirementDerivedFactObservation,
) -> Result<(), String> {
    match (&expectation.outcome, observation) {
        (
            RequirementDerivedFactExpectationOutcome::Incomplete,
            RequirementDerivedFactObservation::Incomplete,
        ) => Ok(()),
        (
            RequirementDerivedFactExpectationOutcome::Unsupported(prerequisite),
            RequirementDerivedFactObservation::Outcome {
                value:
                    RequirementDerivedFactOutcome::Unsupported {
                        prerequisite: actual,
                    },
                ..
            },
        ) if prerequisite == actual => Ok(()),
        (
            RequirementDerivedFactExpectationOutcome::Absent,
            RequirementDerivedFactObservation::Outcome {
                value: RequirementDerivedFactOutcome::Elements(values),
                ..
            },
        ) if values.is_empty() => Ok(()),
        (
            RequirementDerivedFactExpectationOutcome::Resolved,
            RequirementDerivedFactObservation::Outcome {
                value: RequirementDerivedFactOutcome::Elements(values),
                expected: Some(expected),
            },
        ) if values.iter().any(|actual| actual == expected) => Ok(()),
        (
            RequirementDerivedFactExpectationOutcome::Text,
            RequirementDerivedFactObservation::Outcome {
                value: RequirementDerivedFactOutcome::Text(values),
                ..
            },
        ) if expectation
            .text
            .as_deref()
            .is_some_and(|text| values.iter().any(|actual| actual.as_ref() == text)) =>
        {
            Ok(())
        }
        _ => Err(format!(
            "semantic Requirements fact expectation for {} did not match its typed outcome",
            expectation.source
        )),
    }
}

fn compare_semantic_relationship_observation(
    outcome: SemanticRelationshipOutcome,
    source: &str,
    kind: SemanticRelationshipKind,
    observation: &SemanticRelationshipObservation,
) -> Result<(), String> {
    match (outcome, observation) {
        (SemanticRelationshipOutcome::Incomplete, SemanticRelationshipObservation::Incomplete) => {}
        (SemanticRelationshipOutcome::Absent, SemanticRelationshipObservation::Absent { .. }) => {}
        (
            SemanticRelationshipOutcome::Resolved,
            SemanticRelationshipObservation::Relationship {
                target: RelationshipTarget::Resolved(actual_target),
                expected_target: Some(expected_target),
                ..
            },
        ) => {
            if actual_target != expected_target {
                return Err(format!(
                        "semantic relationship expectation for {} resolved to a different target identity",
                        source
                    ));
            }
        }
        (
            SemanticRelationshipOutcome::Unresolved,
            SemanticRelationshipObservation::Relationship {
                target: RelationshipTarget::Unresolved,
                ..
            },
        )
        | (
            SemanticRelationshipOutcome::Ambiguous,
            SemanticRelationshipObservation::Relationship {
                target: RelationshipTarget::Ambiguous(_),
                ..
            },
        )
        | (
            SemanticRelationshipOutcome::Unsupported,
            SemanticRelationshipObservation::Relationship {
                target: RelationshipTarget::Unsupported,
                ..
            },
        ) => {}
        _ => {
            return Err(format!(
                "semantic relationship expectation for {} ({:?}) did not match its typed outcome",
                source, kind
            ));
        }
    }
    Ok(())
}

impl FixtureReport {
    fn from_meta(
        path: &Path,
        meta: &FixtureMeta,
        state: ExpectationState,
        registry: &IssueRegistry,
        diagnostics: Vec<ObservedDiagnostic>,
    ) -> Self {
        let (rule_ids, source_expectation, rule_family, expectation, blocked_by) =
            if let Some(normative) = &meta.normative_expectation {
                (
                    normative.rule_ids.clone(),
                    Some(normative.source_expectation),
                    Some(normative.rule_family),
                    Some(normative.expectation),
                    normative
                        .blocked_by
                        .as_deref()
                        .and_then(|id| registry.blocker(id)),
                )
            } else {
                (meta.legacy_rule_ids.clone(), None, None, None, None)
            };
        let by_construction_evidence = meta.normative_expectation.as_ref().and_then(|normative| {
            (normative.expectation == ExpectationKind::ByConstruction).then(|| {
                if normative.evidence.is_some() {
                    ByConstructionEvidenceStatus::Executable
                } else if normative.blocked_by.as_deref().is_some_and(|id| {
                    registry
                        .blocker(id)
                        .is_some_and(|blocker| blocker.kind == IssueKind::AbstractSyntaxCoverageGap)
                }) {
                    ByConstructionEvidenceStatus::AbstractSyntaxCoverageGap
                } else {
                    ByConstructionEvidenceStatus::Missing
                }
            })
        });
        Self {
            path: normalized_report_path(path),
            rule_ids,
            source_expectation,
            rule_family,
            expectation,
            state,
            blocked_by,
            by_construction_evidence,
            diagnostics,
        }
    }
}

impl ExpectationState {
    fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Unclassified => "unclassified",
            Self::Passed => "passed",
            Self::Blocked => "blocked",
            Self::Stale => "stale",
            Self::Failed => "failed",
        }
    }
}

impl IssueKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::ParserGap => "parser_gap",
            Self::LoweringGap => "lowering_gap",
            Self::SemanticNotImplemented => "semantic_not_implemented",
            Self::DiagnosticNotImplemented => "diagnostic_not_implemented",
            Self::SemanticQueryGap => "semantic_query_gap",
            Self::LibraryGap => "library_gap",
            Self::AbstractSyntaxCoverageGap => "abstract_syntax_coverage_gap",
            Self::NormativeSpecificationGap => "normative_specification_gap",
        }
    }
}

impl SnapshotReport {
    fn new(
        mut fixtures: Vec<FixtureReport>,
        processing_failures: &[(PathBuf, String)],
        stale_generated_snapshots: &[PathBuf],
        manifest_audit: Option<ManifestAuditOutcome>,
    ) -> Self {
        fixtures.sort_by(|left, right| left.path.cmp(&right.path));
        let mut expectations = BTreeMap::from([
            (ExpectationState::NotApplicable.as_str().to_string(), 0),
            (ExpectationState::Unclassified.as_str().to_string(), 0),
            (ExpectationState::Passed.as_str().to_string(), 0),
            (ExpectationState::Blocked.as_str().to_string(), 0),
            (ExpectationState::Stale.as_str().to_string(), 0),
            (ExpectationState::Failed.as_str().to_string(), 0),
        ]);
        let mut coverage_rule_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut normative_coverage = BTreeMap::new();
        type IssuePaths =
            BTreeMap<(String, String), (ReportBlocker, BTreeSet<String>, BTreeSet<String>)>;
        type ObservedDiagnostics = BTreeMap<
            String,
            (
                DiagnosticAggregate,
                BTreeSet<String>,
                BTreeMap<String, BTreeSet<String>>,
            ),
        >;
        let mut issue_paths: IssuePaths = BTreeMap::new();
        let mut observed_diagnostics: ObservedDiagnostics = BTreeMap::new();
        for fixture in &fixtures {
            *expectations
                .entry(fixture.state.as_str().to_string())
                .or_insert(0) += 1;
            if let Some(blocker) = &fixture.blocked_by {
                let entry = issue_paths
                    .entry((blocker.kind.as_str().to_string(), blocker.id.clone()))
                    .or_insert_with(|| (blocker.clone(), BTreeSet::new(), BTreeSet::new()));
                entry.1.insert(fixture.path.clone());
                entry.2.extend(fixture.rule_ids.iter().cloned());
            }
            if let Some(family) = fixture.rule_family {
                let family = family.as_str().to_string();
                let coverage = normative_coverage
                    .entry(family.clone())
                    .or_insert_with(NormativeCoverage::default);
                coverage.fixture_count += 1;
                match fixture.expectation {
                    Some(ExpectationKind::Diagnostics) => coverage.diagnostics_fixture_count += 1,
                    Some(ExpectationKind::Semantics) => coverage.semantics_fixture_count += 1,
                    Some(ExpectationKind::ByConstruction) => {
                        coverage.by_construction_fixture_count += 1;
                        match fixture.by_construction_evidence {
                            Some(ByConstructionEvidenceStatus::Executable) => {
                                coverage.by_construction_executable_evidence_fixture_count += 1
                            }
                            Some(ByConstructionEvidenceStatus::AbstractSyntaxCoverageGap) => {
                                coverage.by_construction_abstract_coverage_gap_fixture_count += 1
                            }
                            Some(ByConstructionEvidenceStatus::Missing) | None => {
                                coverage.by_construction_missing_evidence_fixture_count += 1
                            }
                        }
                    }
                    None => {}
                }
                match fixture.state {
                    ExpectationState::Passed => coverage.passed_fixture_count += 1,
                    ExpectationState::Blocked => coverage.blocked_fixture_count += 1,
                    ExpectationState::Stale => coverage.stale_fixture_count += 1,
                    ExpectationState::Failed => coverage.failed_fixture_count += 1,
                    ExpectationState::NotApplicable => coverage.not_applicable_fixture_count += 1,
                    ExpectationState::Unclassified => {}
                }
                coverage_rule_ids
                    .entry(family)
                    .or_default()
                    .extend(fixture.rule_ids.iter().cloned());
            }
            for diagnostic in &fixture.diagnostics {
                let (aggregate, paths, origin_paths) = observed_diagnostics
                    .entry(diagnostic.category.clone())
                    .or_insert_with(|| {
                        (
                            DiagnosticAggregate::default(),
                            BTreeSet::new(),
                            BTreeMap::new(),
                        )
                    });
                aggregate.occurrences += 1;
                let origin = aggregate
                    .origins
                    .entry(diagnostic.origin.clone())
                    .or_default();
                origin.occurrences += 1;
                *origin
                    .severities
                    .entry(diagnostic.severity.clone())
                    .or_insert(0) += 1;
                paths.insert(fixture.path.clone());
                origin_paths
                    .entry(diagnostic.origin.clone())
                    .or_default()
                    .insert(fixture.path.clone());
            }
        }
        for (family, rule_ids) in coverage_rule_ids {
            normative_coverage
                .get_mut(&family)
                .unwrap()
                .unique_rule_count = rule_ids.len();
        }
        for family in ["derive", "check", "validate"] {
            normative_coverage.entry(family.to_string()).or_default();
        }
        let outstanding_issues = issue_paths
            .into_values()
            .map(|(blocker, paths, rules)| IssueImpact {
                id: blocker.id,
                kind: blocker.kind,
                owner: blocker.owner,
                summary: blocker.summary,
                affected_fixture_count: paths.len(),
                affected_rule_count: rules.len(),
                fixtures: paths.into_iter().collect(),
            })
            .collect();
        let observed_diagnostics = observed_diagnostics
            .into_iter()
            .map(|(category, (mut aggregate, paths, origin_paths))| {
                aggregate.affected_fixture_count = paths.len();
                for (origin, paths) in origin_paths {
                    if let Some(origin_aggregate) = aggregate.origins.get_mut(&origin) {
                        origin_aggregate.affected_fixture_count = paths.len();
                    }
                }
                (category, aggregate)
            })
            .collect();
        let successfully_evaluated_fixtures = fixtures.len();
        let unclassified_fixtures = fixtures
            .iter()
            .filter(|fixture| fixture.state == ExpectationState::Unclassified)
            .count();
        Self {
            schema_version: 2,
            fixtures,
            aggregate: AggregateReport {
                selected_fixtures: successfully_evaluated_fixtures + processing_failures.len(),
                successfully_evaluated_fixtures,
                processing_failures: processing_failures.len(),
                unclassified_fixtures,
                stale_generated_snapshots: stale_generated_snapshots.len(),
                expectations,
                normative_coverage,
                outstanding_issues,
                observed_diagnostics,
            },
            processing_failures: {
                let mut failures: Vec<_> = processing_failures
                    .iter()
                    .map(|(path, error)| ProcessingFailure {
                        path: normalized_report_path(path),
                        error: error.clone(),
                    })
                    .collect();
                failures.sort_by(|left, right| left.path.cmp(&right.path));
                failures
            },
            stale_generated_snapshots: {
                let mut paths: Vec<_> = stale_generated_snapshots
                    .iter()
                    .map(|path| normalized_report_path(path))
                    .collect();
                paths.sort();
                paths
            },
            manifest_audit,
        }
    }
}

fn emit_report(report: &SnapshotReport, format: ReportFormat) -> Result<(), String> {
    match format {
        ReportFormat::Text => {
            print!("{}", render_report_text(report));
            Ok(())
        }
        ReportFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(report)
                    .map_err(|error| format!("report JSON serialization failed: {error}"))?
            );
            Ok(())
        }
    }
}

fn render_report_text(report: &SnapshotReport) -> String {
    let mut output = String::new();
    for fixture in &report.fixtures {
        let rules = if fixture.rule_ids.is_empty() {
            "-".to_string()
        } else {
            fixture.rule_ids.join(",")
        };
        let blocker = fixture
            .blocked_by
            .as_ref()
            .map(|blocker| blocker.id.as_str())
            .unwrap_or("-");
        writeln!(
            output,
            "{} {} rules={} blocker={}",
            fixture.state.as_str(),
            fixture.path,
            rules,
            blocker
        )
        .expect("writing to String cannot fail");
    }
    writeln!(output, "aggregate expectations:").expect("writing to String cannot fail");
    writeln!(
        output,
        "  selected={} evaluated={} processing_failures={} unclassified={} stale_generated={}",
        report.aggregate.selected_fixtures,
        report.aggregate.successfully_evaluated_fixtures,
        report.aggregate.processing_failures,
        report.aggregate.unclassified_fixtures,
        report.aggregate.stale_generated_snapshots
    )
    .expect("writing to String cannot fail");
    for (state, count) in &report.aggregate.expectations {
        writeln!(output, "  {state}: {count}").expect("writing to String cannot fail");
    }
    writeln!(output, "aggregate normative coverage:").expect("writing to String cannot fail");
    for (family, coverage) in &report.aggregate.normative_coverage {
        writeln!(output, "  {family}: fixtures={} rules={} diagnostics={} semantics={} by_construction={} executable_evidence={} abstract_coverage_gap={} missing_evidence={} passed={} blocked={} stale={} failed={} not_applicable={}", coverage.fixture_count, coverage.unique_rule_count, coverage.diagnostics_fixture_count, coverage.semantics_fixture_count, coverage.by_construction_fixture_count, coverage.by_construction_executable_evidence_fixture_count, coverage.by_construction_abstract_coverage_gap_fixture_count, coverage.by_construction_missing_evidence_fixture_count, coverage.passed_fixture_count, coverage.blocked_fixture_count, coverage.stale_fixture_count, coverage.failed_fixture_count, coverage.not_applicable_fixture_count).expect("writing to String cannot fail");
    }
    writeln!(output, "aggregate outstanding issues:").expect("writing to String cannot fail");
    for issue in &report.aggregate.outstanding_issues {
        writeln!(
            output,
            "  {} {} fixtures={} rules={} {}",
            issue.kind.as_str(),
            issue.id,
            issue.affected_fixture_count,
            issue.affected_rule_count,
            issue.summary
        )
        .expect("writing to String cannot fail");
        for fixture in &issue.fixtures {
            writeln!(output, "    fixture: {fixture}").expect("writing to String cannot fail");
        }
    }
    writeln!(output, "aggregate observed diagnostics:").expect("writing to String cannot fail");
    for (category, aggregate) in &report.aggregate.observed_diagnostics {
        writeln!(
            output,
            "  {category}: occurrences={} affected_fixtures={} origins={:?}",
            aggregate.occurrences, aggregate.affected_fixture_count, aggregate.origins
        )
        .expect("writing to String cannot fail");
    }
    if !report.processing_failures.is_empty() {
        writeln!(output, "processing failures:").expect("writing to String cannot fail");
        for failure in &report.processing_failures {
            writeln!(output, "  {}: {}", failure.path, failure.error)
                .expect("writing to String cannot fail");
        }
    }
    if !report.stale_generated_snapshots.is_empty() {
        writeln!(output, "stale generated snapshots:").expect("writing to String cannot fail");
        for path in &report.stale_generated_snapshots {
            writeln!(output, "  {path}").expect("writing to String cannot fail");
        }
    }
    if let Some(manifest_audit) = &report.manifest_audit {
        writeln!(output, "manifest audit:").expect("writing to String cannot fail");
        let ManifestAuditOutcome::Complete { audit } = manifest_audit else {
            let ManifestAuditOutcome::Failed { error } = manifest_audit else {
                unreachable!("manifest audit outcome is exhaustive")
            };
            writeln!(output, "  failed: {error}").expect("writing to String cannot fail");
            return output;
        };
        writeln!(
            output,
            "  manifest rule occurrences: {}",
            audit.manifest_rule_occurrences
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "  manifest unique rule IDs: {}",
            audit.manifest_unique_rule_ids
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "  fixture rule occurrences: {}",
            audit.fixture_rule_occurrences
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "  fixture unique rule IDs: {}",
            audit.fixture_unique_rule_ids
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "  selected fixture rule occurrences: {}",
            audit.selected_fixture_rule_occurrences
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "  selected fixture unique rule IDs: {}",
            audit.selected_fixture_unique_rule_ids
        )
        .expect("writing to String cannot fail");
        for (family, count) in &audit.manifest_rule_occurrences_by_family {
            writeln!(output, "  manifest {family} occurrences: {count}")
                .expect("writing to String cannot fail");
        }
        for (family, count) in &audit.missing_rule_ids_by_family {
            writeln!(output, "  missing {family} evidence: {count}")
                .expect("writing to String cannot fail");
        }
        render_manifest_audit_errors(&mut output, audit);
    }
    output
}

fn emit_check_summary(report: &SnapshotReport) {
    print!("{}", render_check_summary(report));
}

fn render_check_summary(report: &SnapshotReport) -> String {
    let mut output = String::new();
    for fixture in &report.fixtures {
        if fixture.state == ExpectationState::Blocked {
            if let Some(blocker) = &fixture.blocked_by {
                writeln!(
                    output,
                    "BLOCKED {} issue={} kind={} expectation={}",
                    fixture.path,
                    blocker.id,
                    blocker.kind.as_str(),
                    fixture
                        .expectation
                        .map(ExpectationKind::as_str)
                        .unwrap_or("-")
                )
                .expect("writing to String cannot fail");
            }
        }
    }
    writeln!(
        output,
        "snapshot summary: selected={} evaluated={} processing_failures={} stale_generated={}",
        report.aggregate.selected_fixtures,
        report.aggregate.successfully_evaluated_fixtures,
        report.aggregate.processing_failures,
        report.aggregate.stale_generated_snapshots
    )
    .expect("writing to String cannot fail");
    for (state, count) in &report.aggregate.expectations {
        writeln!(output, "  {state}: {count}").expect("writing to String cannot fail");
    }
    output
}

fn emit_failures(processing: &[(PathBuf, String)], expectations: &[(PathBuf, String)]) {
    if !processing.is_empty() {
        eprintln!("snapshot processing errors:");
        for (path, error) in processing {
            eprintln!("  {}: {error}", normalized_report_path(path));
        }
    }
    if !expectations.is_empty() {
        eprintln!("snapshot expectation failures:");
        for (path, error) in expectations {
            eprintln!("  {}: {error}", normalized_report_path(path));
        }
    }
}

fn normalized_report_path(path: &Path) -> String {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let canonical_root = fs::canonicalize(repository_root).ok();
    let canonical_path = fs::canonicalize(path).ok();
    match (canonical_root, canonical_path) {
        (Some(root), Some(path)) => path
            .strip_prefix(root)
            .map(|relative| relative.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| outside_repository_report_path(&path)),
        (_, Some(path)) => outside_repository_report_path(&path),
        _ => outside_repository_report_path(path),
    }
}

/// Reports never disclose an absolute host path. Snapshot fixtures are repository-local in normal
/// operation; this explicit marker preserves that invariant for malformed external selections.
fn outside_repository_report_path(path: &Path) -> String {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    format!("<outside-repository>/{name}")
}

fn render_manifest_audit_errors(output: &mut String, audit: &ManifestAuditReport) {
    for rule_id in &audit.missing_rule_ids {
        writeln!(output, "  missing evidence: {rule_id}").expect("writing to String cannot fail");
    }
    for (rule_id, paths) in &audit.duplicate_primary_rule_ids {
        writeln!(
            output,
            "  duplicate primary fixture evidence: {rule_id} ({})",
            paths.join(", ")
        )
        .expect("writing to String cannot fail");
    }
    for (rule_id, paths) in &audit.orphan_secondary_rule_ids {
        writeln!(
            output,
            "  secondary fixture evidence without primary: {rule_id} ({})",
            paths.join(", ")
        )
        .expect("writing to String cannot fail");
    }
    for (rule_id, paths) in &audit.unknown_rule_ids {
        writeln!(
            output,
            "  unknown fixture rule: {rule_id} ({})",
            paths.join(", ")
        )
        .expect("writing to String cannot fail");
    }
    for (rule_id, mismatches) in &audit.family_mismatches {
        for mismatch in mismatches {
            writeln!(output, "  rule family mismatch: {rule_id}: {mismatch}")
                .expect("writing to String cannot fail");
        }
    }
    for (rule_id, mismatches) in &audit.clause_mismatches {
        for mismatch in mismatches {
            writeln!(output, "  clause mismatch: {rule_id}: {mismatch}")
                .expect("writing to String cannot fail");
        }
    }
    for (rule_id, mismatches) in &audit.constraint_mismatches {
        for mismatch in mismatches {
            writeln!(output, "  constraint name mismatch: {rule_id}: {mismatch}")
                .expect("writing to String cannot fail");
        }
    }
    for (rule_id, mismatches) in &audit.specification_mismatches {
        for mismatch in mismatches {
            writeln!(output, "  specification mismatch: {rule_id}: {mismatch}")
                .expect("writing to String cannot fail");
        }
    }
    for (rule_id, mismatches) in &audit.formal_document_mismatches {
        for mismatch in mismatches {
            writeln!(output, "  formal document mismatch: {rule_id}: {mismatch}")
                .expect("writing to String cannot fail");
        }
    }
}

fn load_repository_sources(
    paths: &[String],
    fixture_path: &Path,
) -> Result<Vec<SourceDocument>, String> {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut documents = Vec::with_capacity(paths.len());
    for relative in paths {
        let relative_path = Path::new(relative);
        if relative_path.is_absolute()
            || relative_path.components().any(|component| {
                matches!(
                    component,
                    std::path::Component::ParentDir
                        | std::path::Component::RootDir
                        | std::path::Component::Prefix(_)
                )
            })
            || !relative.starts_with("examples/")
            || relative_path
                .extension()
                .is_none_or(|extension| extension != "sysml")
        {
            return Err(format!(
                "{}: repositorySources entry must be a repository-relative examples/*.sysml path: {relative:?}",
                fixture_path.display()
            ));
        }
        let text = fs::read_to_string(repository_root.join(relative_path)).map_err(|error| {
            format!(
                "{}: could not read repository source {relative:?}: {error}",
                fixture_path.display()
            )
        })?;
        documents.push(SourceDocument {
            name: relative.clone(),
            text,
        });
    }
    Ok(documents)
}

fn execute_generation(
    publication: Arc<PublishedModel>,
    request: &GenerationRequest,
    fixture_path: &Path,
) -> Result<GeneratedArtifacts, String> {
    let plugin_path = generator_plugin_path(&request.plugin);
    let module = fs::read(&plugin_path).map_err(|error| {
        format!(
            "{}: failed to read generator plugin `{}` at {}: {error}; run scripts/build-generator-plugins.sh",
            fixture_path.display(),
            generator_plugin_label(&request.plugin),
            plugin_path.display()
        )
    })?;
    let model_digest = publication.publication().model_digest();
    let model = Arc::new(
        GeneratorModelView::new(
            Arc::clone(&publication),
            model_digest.to_string(),
            env!("CARGO_PKG_VERSION"),
            QueryLimits::default(),
        )
        .map_err(|error| {
            format!(
                "{}: generator model unavailable: {error}",
                fixture_path.display()
            )
        })?,
    );
    debug_assert_eq!(
        model.publication_completeness(),
        publication.publication().completeness(),
        "snapshot evidence must record the exact publication completeness"
    );
    let args = generation_arguments(request, &publication, &model, fixture_path)?;
    let runtime = GeneratorRuntime::new().map_err(|error| {
        format!(
            "{}: generator runtime failed: {error}",
            fixture_path.display()
        )
    })?;
    let execution = runtime
        .execute(
            &module,
            model,
            &args,
            RuntimeLimits::default(),
            ArtifactLimits::default(),
            CancellationHandle::new(),
        )
        .map_err(|error| format!("{}: generation failed: {error}", fixture_path.display()))?;
    if !execution.diagnostics.is_empty() {
        return Err(format!(
            "{}: snapshot generator emitted diagnostics: {:?}",
            fixture_path.display(),
            execution.diagnostics
        ));
    }
    let mut artifacts = GeneratedArtifacts::default();
    for (path, bytes) in execution.artifacts.entries() {
        let contents = String::from_utf8(bytes.to_vec()).map_err(|_| {
            format!(
                "{}: generated artifact `{path}` is not UTF-8",
                fixture_path.display()
            )
        })?;
        artifacts.insert_utf8(path.to_string(), contents)?;
    }
    Ok(artifacts)
}

fn generator_plugin_label(plugin: &GeneratorPlugin) -> String {
    match plugin {
        GeneratorPlugin::Conformance(name) => format!("conformance:{name}"),
        GeneratorPlugin::RepositoryDiagram => "repository:diagram".to_string(),
    }
}

fn generator_plugin_path(plugin: &GeneratorPlugin) -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    match plugin {
        GeneratorPlugin::Conformance(name) => root
            .join("generator-tests/plugins/target/wasm32-unknown-unknown/release")
            .join(format!("spec42_conformance_{name}.wasm")),
        GeneratorPlugin::RepositoryDiagram => root
            .join("generator-plugins/target/wasm32-unknown-unknown/release")
            .join("spec42_diagram_generator.wasm"),
    }
}

fn generation_arguments(
    request: &GenerationRequest,
    publication: &PublishedModel,
    model: &GeneratorModelView,
    fixture_path: &Path,
) -> Result<Vec<String>, String> {
    let Some(selection) = &request.diagram_selection else {
        return Ok(Vec::new());
    };
    let expected_kind = diagram_element_kind(&selection.kind).ok_or_else(|| {
        format!(
            "{}: unknown diagram view kind {:?}",
            fixture_path.display(),
            selection.kind
        )
    })?;
    let document = SourceService::new()
        .admit_memory(
            "snapshot",
            &selection.document,
            String::new(),
            SourceKind::Workspace,
        )
        .map_err(|error| {
            format!(
                "{}: invalid diagram selection document {:?}: {error}",
                fixture_path.display(),
                selection.document
            )
        })?;
    let reference = QualifiedElementReference {
        document: Some(document.uri().as_str().into()),
        qualified_name: selection.qualified_name.clone().into(),
        expected_kind: Some(expected_kind),
    };
    let target = match publication
        .inspection()
        .resolve_qualified_reference(&reference)
    {
        QualifiedReferenceOutcome::Resolved(target)
        | QualifiedReferenceOutcome::Recovered(target)
        | QualifiedReferenceOutcome::UnsupportedWith(target) => target,
        outcome => {
            return Err(format!(
                "{}: diagram view reference {:?} in {:?} did not resolve: {outcome:?}",
                fixture_path.display(),
                selection.qualified_name,
                selection.document
            ))
        }
    };
    let catalog = model.diagram_views().map_err(|error| {
        format!(
            "{}: diagram view catalog failed: {error}",
            fixture_path.display()
        )
    })?;
    let matches = catalog
        .iter()
        .filter(|view| {
            matches!(
                &view.reference,
                DiagramSemanticReference::Qualified { document, qualified_name, .. }
                    if Some(document.as_ref()) == publication.document_identity(target.location.document)
                        && Some(qualified_name.as_ref())
                            == publication.qualified_name(target.identity)
            ) && diagram_kind_id(view.kind) == selection.kind
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [view] => Ok(vec![view.handle.clone()]),
        [] => Err(format!(
            "{}: selected diagram view kind {:?} with qualified reference {:?} in {:?} is not in the active publication; authored catalog entries: {}",
            fixture_path.display(),
            selection.kind,
            publication.qualified_name(target.identity).unwrap_or_default(),
            target.location.document,
            catalog
                .iter()
                .map(|view| format!("{}={:?}", diagram_kind_id(view.kind), view.reference))
                .collect::<Vec<_>>()
                .join(", ")
        )),
        _ => Err(format!(
            "{}: selected diagram identity is not unique in the active publication",
            fixture_path.display()
        )),
    }
}

fn diagram_element_kind(kind: &str) -> Option<ElementKind> {
    match kind {
        "general-view"
        | "interconnection-view"
        | "action-flow-view"
        | "state-transition-view"
        | "sequence-view"
        | "browser-view"
        | "grid-view"
        | "geometry-view" => Some(ElementKind::ViewUsage),
        _ => None,
    }
}

fn diagram_kind_id(kind: generator_api::DiagramViewKind) -> &'static str {
    match kind {
        generator_api::DiagramViewKind::GeneralView => "general-view",
        generator_api::DiagramViewKind::InterconnectionView => "interconnection-view",
        generator_api::DiagramViewKind::ActionFlowView => "action-flow-view",
        generator_api::DiagramViewKind::StateTransitionView => "state-transition-view",
        generator_api::DiagramViewKind::SequenceView => "sequence-view",
        generator_api::DiagramViewKind::BrowserView => "browser-view",
        generator_api::DiagramViewKind::GridView => "grid-view",
        generator_api::DiagramViewKind::GeometryView => "geometry-view",
    }
}

struct OwnedSections {
    smg: String,
    types: String,
    diagnostics: String,
    navigation: String,
    editor_queries: String,
    hover_reports: String,
    hover_markdown: String,
    qualified_references: String,
}

fn render_owned_sections(
    model: &PublishedModel,
    documents: &[SourceDocument],
    source_documents: &[QuerySourceDocument],
    hover_workspace: &InMemoryWorkspace,
    probes: &EditorProbes,
    qualified_reference_probes: &[QualifiedReferenceProbe],
) -> Result<OwnedSections, String> {
    // Both strings are complete owner-defined projections. The SMG includes publication phase,
    // completeness, evaluation state, and all owned facts; diagnostics includes canonical order.
    let smg = render_semantic_model(model)?;
    let diagnostics = render_diagnostics(model, documents, source_documents)?;
    let mut types = String::new();
    model
        .debug()
        .write_types_sexpr(&mut types)
        .map_err(|error| format!("type rendering failed: {error}"))?;
    let mut navigation = String::new();
    model
        .debug()
        .write_navigation_sexpr(&mut navigation)
        .map_err(|error| format!("navigation rendering failed: {error}"))?;
    let mut editor_queries = String::new();
    model
        .debug()
        .write_editor_queries_sexpr(&probes.queries, &mut editor_queries)
        .map_err(|error| format!("editor-query rendering failed: {error}"))?;
    let (hover_reports, hover_markdown) = render_hover_probes(hover_workspace, probes)?;
    let mut qualified_references = String::new();
    model
        .debug()
        .write_qualified_reference_queries_sexpr(
            qualified_reference_probes,
            &mut qualified_references,
        )
        .map_err(|error| format!("qualified-reference rendering failed: {error}"))?;
    Ok(OwnedSections {
        smg,
        types,
        diagnostics,
        navigation,
        editor_queries,
        hover_reports,
        hover_markdown,
        qualified_references,
    })
}

fn render_hover_probes(
    workspace: &InMemoryWorkspace,
    probes: &EditorProbes,
) -> Result<(String, String), String> {
    if probes.hover.is_empty() {
        return Ok((String::new(), String::new()));
    }
    let mut reports = String::from("(hover-reports\n");
    let mut markdown = String::new();
    for index in probes.hover.iter().copied() {
        let probe = probes
            .queries
            .get(index)
            .ok_or_else(|| format!("hover probe index {index} is out of bounds"))?;
        let path = probe
            .document
            .strip_prefix("memory://snapshot/")
            .unwrap_or(&probe.document);
        write!(
            reports,
            "  (probe (document {:?}) (position {} {})",
            probe.document, probe.position.line, probe.position.character
        )
        .map_err(|error| error.to_string())?;
        writeln!(
            markdown,
            "## {path}:{}:{}",
            probe.position.line, probe.position.character
        )
        .map_err(|error| error.to_string())?;
        markdown.push_str("~~~markdown\n");
        match hover_report(workspace, path, probe.position) {
            Some(report) => {
                reports.push_str(" (status available)\n");
                for line in render_hover_sexpr(&report).lines() {
                    writeln!(reports, "    {line}").map_err(|error| error.to_string())?;
                }
                reports.push_str("  )\n");
                markdown.push_str(&render_hover_markdown(&report));
                markdown.push('\n');
            }
            None => {
                reports.push_str(" (status none))\n");
            }
        }
        markdown.push_str("~~~\n");
    }
    reports.push_str(")\n");
    Ok((reports, markdown))
}

/// Rejects an owned section whose S-expression does not close.
///
/// These sections are a contract, not decoration: a reader that parses them has to be able to.
/// Three separate producers had drifted out of balance without any check noticing, because a
/// snapshot only ever had to match its own previous text. Parentheses inside quoted strings are
/// content, not structure, so the scan tracks quoting.
fn ensure_balanced(name: &str, text: &str) -> Result<(), String> {
    let mut depth = 0i64;
    let mut quoted = false;
    let mut escaped = false;
    for character in text.chars() {
        if quoted {
            match character {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => quoted = false,
                _ => {}
            }
            continue;
        }
        match character {
            '"' => quoted = true,
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(format!("{name} section closes more elements than it opens"));
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return Err(format!("{name} section leaves {depth} element(s) open"));
    }
    Ok(())
}

fn ensure_sections_balanced(sections: &OwnedSections) -> Result<(), String> {
    ensure_balanced("SMG", &sections.smg)?;
    ensure_balanced("TYPES", &sections.types)?;
    ensure_balanced("DIAGNOSTICS", &sections.diagnostics)?;
    ensure_balanced("NAVIGATION", &sections.navigation)?;
    ensure_balanced("EDITOR RESULTS", &sections.editor_queries).and_then(|()| {
        ensure_balanced("HOVER RESULTS", &sections.hover_reports)?;
        ensure_balanced(
            "QUALIFIED REFERENCE RESULTS",
            &sections.qualified_references,
        )
    })
}

fn render_semantic_model(model: &PublishedModel) -> Result<String, String> {
    let mut output = String::new();
    model
        .debug()
        .write_semantic_sexpr(&mut output)
        .map_err(|error| format!("semantic-model rendering failed: {error}"))?;
    Ok(output)
}

fn render_diagnostics(
    model: &PublishedModel,
    _documents: &[SourceDocument],
    _source_documents: &[QuerySourceDocument],
) -> Result<String, String> {
    let mut rendered = String::new();
    model
        .debug()
        .write_diagnostics_sexpr(&mut rendered)
        .map_err(|error| format!("diagnostic rendering failed: {error}"))?;
    Ok(rendered)
}

fn parse_source_documents(
    fixture: &str,
    fallback_name: &str,
) -> Result<Vec<SourceDocument>, String> {
    let source = raw_section(fixture, "SOURCE")
        .ok_or_else(|| format!("{fallback_name}: missing # SOURCE section"))?;
    let mut named = Vec::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("## ") {
        cursor = &cursor[index + 3..];
        let Some((name, rest)) = cursor.split_once('\n') else {
            return Err(format!("{fallback_name}: malformed named SOURCE document"));
        };
        let Some((text, after)) = fenced_block(rest) else {
            return Err(format!(
                "{fallback_name}: malformed SOURCE fence for {name}"
            ));
        };
        named.push(SourceDocument {
            name: name.trim().to_string(),
            text,
        });
        cursor = after;
    }
    if !named.is_empty() {
        return Ok(named);
    }
    fenced_block(source)
        .map(|(text, _)| {
            vec![SourceDocument {
                name: fallback_name.to_string(),
                text,
            }]
        })
        .ok_or_else(|| format!("{fallback_name}: malformed SOURCE fence"))
}

/// Reads execution-affecting META keys. Descriptive keys remain open-ended, but malformed lines,
/// duplicate execution keys, and incomplete generator declarations are rejected.
fn parse_fixture_meta(fixture: &str, fallback_name: &str) -> Result<FixtureMeta, String> {
    let Some(section) = raw_section(fixture, "META") else {
        return Ok(FixtureMeta {
            libraries: LibrarySelection::None,
            repository_sources: Vec::new(),
            generation: None,
            standard_library_documents: BTreeSet::new(),
            normative_expectation: None,
            legacy_rule_ids: Vec::new(),
        });
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!("{fallback_name}: malformed META fence"));
    };
    let mut selection = LibrarySelection::None;
    let mut fixture_type = None;
    let mut repository_sources = Vec::new();
    let mut plugin = None;
    let mut view_kind = None;
    let mut view_document = None;
    let mut view_qualified_name = None;
    let mut source_expectation = None;
    let mut rule_family = None;
    let mut expectation = None;
    let mut rule_ids = Vec::new();
    let mut coverage_role = None;
    let mut legacy_rule_ids = Vec::new();
    let mut blocked_by = None;
    let mut evidence = None;
    let mut specification_id = None;
    let mut standard_library_documents = BTreeSet::new();
    let mut seen = HashSet::new();
    for (line_index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!(
                "{fallback_name}: META line {} must be key=value",
                line_index + 1
            ));
        };
        let key = key.trim();
        let value = value.trim();
        if matches!(
            key,
            "libraries"
                | "repositorySources"
                | "type"
                | "plugin"
                | "viewKind"
                | "viewDocument"
                | "viewQualifiedName"
                | "source_expectation"
                | "rule_family"
                | "expectation"
                | "coverage_role"
                | "blocked_by"
                | "evidence_reference"
                | "specification_id"
        ) && !seen.insert(key)
        {
            return Err(format!("{fallback_name}: duplicate META key {key:?}"));
        }
        match key {
            "libraries" => selection = match value {
                "none" => LibrarySelection::None,
                "standard" => LibrarySelection::Standard,
                other => return Err(format!(
                    "{fallback_name}: unknown META libraries value {other:?} (expected \"none\" or \"standard\")"
                )),
            },
            "repositorySources" => {
                repository_sources = value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .collect();
                if repository_sources.is_empty() {
                    return Err(format!("{fallback_name}: META repositorySources must not be empty"));
                }
            }
            "type" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META type must not be empty"));
                }
                fixture_type = Some(value.to_string());
            }
            "plugin" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META plugin must not be empty"));
                }
                plugin = Some(value.to_string());
            }
            "viewKind" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META viewKind must not be empty"));
                }
                view_kind = Some(value.to_string());
            }
            "viewDocument" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META viewDocument must not be empty"));
                }
                view_document = Some(value.to_string());
            }
            "viewQualifiedName" => {
                if value.is_empty() {
                    return Err(format!(
                        "{fallback_name}: META viewQualifiedName must not be empty"
                    ));
                }
                view_qualified_name = Some(value.to_string());
            }
            "source_expectation" => {
                source_expectation = Some(SourceExpectation::parse(value, fallback_name)?);
            }
            "rule_family" => {
                rule_family = Some(RuleFamily::parse(value, fallback_name)?);
            }
            "expectation" => {
                expectation = Some(ExpectationKind::parse(value, fallback_name)?);
            }
            "coverage_role" => {
                coverage_role = Some(CoverageRole::parse(value, fallback_name)?);
            }
            "rule_id" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META rule_id must not be empty"));
                }
                if rule_ids.iter().any(|id| id == value) {
                    return Err(format!(
                        "{fallback_name}: duplicate META rule_id {value:?}"
                    ));
                }
                rule_ids.push(value.to_string());
            }
            // Kept only while the checked-in corpus is migrated to `rule_id`. It remains a
            // descriptive legacy key and cannot select typed blocker behavior.
            "validation_rule" => {
                if !value.is_empty() && !legacy_rule_ids.iter().any(|id| id == value) {
                    legacy_rule_ids.push(value.to_string());
                }
            }
            "blocked_by" => {
                if !valid_issue_id(value) {
                    return Err(format!(
                        "{fallback_name}: META blocked_by must be a stable lowercase issue id"
                    ));
                }
                blocked_by = Some(value.to_string());
            }
            "evidence_reference" => {
                evidence = Some(ByConstructionEvidence::parse(value, fallback_name)?);
            }
            "specification_id" => {
                specification_id = Some(SpecificationId::parse(value).ok_or_else(|| {
                    format!(
                        "{fallback_name}: unknown META specification_id value {value:?} (expected kerml-1.0 or sysml-2.0)"
                    )
                })?);
            }
            "standard_library_document" => {
                if value.is_empty() {
                    return Err(format!(
                        "{fallback_name}: META standard_library_document must name a SOURCE document"
                    ));
                }
                if !standard_library_documents.insert(value.to_string()) {
                    return Err(format!(
                        "{fallback_name}: duplicate META standard_library_document {value:?}"
                    ));
                }
            }
            _ => {}
        }
    }
    let generation = match (fixture_type.as_deref(), plugin) {
        (Some("generate"), Some(plugin)) => {
            let plugin = parse_generator_plugin(&plugin, fallback_name)?;
            let diagram_selection = match (view_kind, view_document, view_qualified_name) {
                (Some(kind), Some(document), Some(qualified_name)) => Some(DiagramSelection {
                    kind,
                    document,
                    qualified_name,
                }),
                (None, None, None) => None,
                _ => {
                    return Err(format!(
                        "{fallback_name}: META viewKind, viewDocument and viewQualifiedName must be specified together"
                    ))
                }
            };
            if diagram_selection.is_some() && plugin != GeneratorPlugin::RepositoryDiagram {
                return Err(format!(
                    "{fallback_name}: typed view selection is only valid with plugin=repository:diagram"
                ));
            }
            Some(GenerationRequest {
                plugin,
                diagram_selection,
            })
        }
        (Some("generate"), None) => {
            return Err(format!(
                "{fallback_name}: META type=generate requires a plugin"
            ))
        }
        (_, Some(_)) => {
            return Err(format!(
                "{fallback_name}: META plugin is only valid with type=generate"
            ))
        }
        _ if view_kind.is_some() || view_document.is_some() || view_qualified_name.is_some() => {
            return Err(format!(
                "{fallback_name}: view selection is only valid with type=generate"
            ))
        }
        _ => None,
    };
    if !standard_library_documents.is_empty() && selection == LibrarySelection::Standard {
        return Err(format!(
            "{fallback_name}: META standard_library_document cannot be combined with libraries=standard"
        ));
    }
    let has_normative_metadata = source_expectation.is_some()
        || rule_family.is_some()
        || expectation.is_some()
        || coverage_role.is_some()
        || !rule_ids.is_empty()
        || blocked_by.is_some()
        || evidence.is_some()
        || specification_id.is_some();
    let normative_expectation = if has_normative_metadata {
        let source_expectation = source_expectation.ok_or_else(|| {
            format!("{fallback_name}: META source_expectation is required for a normative rule")
        })?;
        let rule_family = rule_family.ok_or_else(|| {
            format!("{fallback_name}: META rule_family is required for a normative rule")
        })?;
        let expectation = expectation.ok_or_else(|| {
            format!("{fallback_name}: META expectation is required for a normative rule")
        })?;
        if rule_ids.is_empty() {
            return Err(format!(
                "{fallback_name}: META rule_id is required for a normative rule"
            ));
        }
        if !matches!(
            (rule_family, expectation),
            (
                RuleFamily::Derive | RuleFamily::Check,
                ExpectationKind::Semantics | ExpectationKind::ByConstruction
            ) | (
                RuleFamily::Validate,
                ExpectationKind::Diagnostics | ExpectationKind::ByConstruction
            )
        ) {
            return Err(format!(
                "{fallback_name}: META rule_family={rule_family:?} is incompatible with expectation={expectation:?}"
            ));
        }
        if evidence.is_some() && expectation != ExpectationKind::ByConstruction {
            return Err(format!(
                "{fallback_name}: META evidence_reference is only valid with expectation=by_construction"
            ));
        }
        Some(NormativeExpectation {
            source_expectation,
            rule_family,
            expectation,
            rule_ids,
            coverage_role: coverage_role.unwrap_or(CoverageRole::Primary),
            blocked_by,
            evidence,
            specification_id,
        })
    } else {
        None
    };
    Ok(FixtureMeta {
        libraries: selection,
        repository_sources,
        generation,
        standard_library_documents,
        normative_expectation,
        legacy_rule_ids,
    })
}

fn parse_generator_plugin(value: &str, fallback_name: &str) -> Result<GeneratorPlugin, String> {
    if value == "repository:diagram" {
        return Ok(GeneratorPlugin::RepositoryDiagram);
    }
    let name = value.strip_prefix("conformance:").unwrap_or(value);
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(format!(
            "{fallback_name}: unknown or unsafe META plugin {value:?}"
        ));
    }
    Ok(GeneratorPlugin::Conformance(name.to_string()))
}

fn validate_artifact_path(path: &str) -> Result<(), String> {
    let candidate = Path::new(path);
    if path.is_empty()
        || candidate.is_absolute()
        || candidate.components().any(|part| {
            matches!(
                part,
                std::path::Component::ParentDir | std::path::Component::CurDir
            )
        })
    {
        return Err(format!("invalid generated artifact path {path:?}"));
    }
    Ok(())
}

fn artifact_fence_language(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("csv") => "csv",
        Some("json") => "json",
        _ => "text",
    }
}

fn render_generated_artifacts(artifacts: &GeneratedArtifacts) -> String {
    let mut output = String::new();
    for (path, contents) in &artifacts.files {
        output.push_str("## ");
        output.push_str(path);
        output.push('\n');
        output.push_str("~~~");
        output.push_str(artifact_fence_language(path));
        output.push('\n');
        output.push_str(contents);
        output.push_str("\n~~~\n");
    }
    output
}

#[cfg(test)]
fn parse_generated_artifacts(
    fixture: &str,
    fallback_name: &str,
) -> Result<Option<GeneratedArtifacts>, String> {
    let Some(section) = raw_section(fixture, "GENERATED") else {
        return Ok(None);
    };
    let mut artifacts = GeneratedArtifacts::default();
    let mut cursor = section;
    while let Some(index) = cursor.find("## ") {
        cursor = &cursor[index + 3..];
        let Some((path, rest)) = cursor.split_once('\n') else {
            return Err(format!(
                "{fallback_name}: malformed GENERATED artifact name"
            ));
        };
        let Some((contents, after)) = fenced_block(rest) else {
            return Err(format!(
                "{fallback_name}: malformed GENERATED fence for {path}"
            ));
        };
        artifacts.insert_utf8(path.trim(), contents)?;
        cursor = after;
    }
    if !section.trim().is_empty() && artifacts.files.is_empty() {
        return Err(format!(
            "{fallback_name}: GENERATED section must contain named artifacts"
        ));
    }
    Ok(Some(artifacts))
}

fn replace_or_insert_generated_section(fixture: &str, artifacts: &GeneratedArtifacts) -> String {
    let body = render_generated_artifacts(artifacts);
    if let Some(updated) = replace_raw_section(fixture, "GENERATED", &body) {
        return updated;
    }
    let mut updated = fixture.trim_end_matches('\n').to_string();
    updated.push_str("\n# GENERATED\n");
    updated.push_str(&body);
    updated
}

fn parse_editor_probes(
    fixture: &str,
    documents: &[SourceDocument],
    fallback_name: &str,
) -> Result<EditorProbes, String> {
    let Some(section) = raw_section(fixture, "EDITOR QUERIES") else {
        return Ok(EditorProbes {
            queries: Vec::new(),
            hover: BTreeSet::new(),
        });
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!("{fallback_name}: malformed EDITOR QUERIES fence"));
    };
    let mut probes = Vec::new();
    let mut hover = BTreeSet::new();
    for (line_index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        if fields.next() != Some("probe") {
            return Err(format!(
                "{fallback_name}: EDITOR QUERIES line {} must start with `probe`",
                line_index + 1
            ));
        }
        let document = fields
            .next()
            .ok_or_else(|| format!("{fallback_name}: missing probe document"))?;
        if !documents.iter().any(|candidate| candidate.name == document) {
            return Err(format!(
                "{fallback_name}: unknown probe document {document:?}"
            ));
        }
        let line = fields
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("{fallback_name}: invalid probe line"))?;
        let character = fields
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("{fallback_name}: invalid probe character"))?;
        let mut qualifier = None;
        let mut rename_to = None;
        let probe_index = probes.len();
        for option in fields {
            if let Some(value) = option.strip_prefix("qualifier=") {
                qualifier = Some(value.to_string());
            } else if let Some(value) = option.strip_prefix("rename=") {
                rename_to = Some(value.to_string());
            } else if option == "hover" {
                if !hover.insert(probe_index) {
                    return Err(format!(
                        "{fallback_name}: duplicate editor probe option {option:?}"
                    ));
                }
            } else {
                return Err(format!(
                    "{fallback_name}: unknown editor probe option {option:?}"
                ));
            }
        }
        probes.push(EditorProbe {
            document: format!("memory://snapshot/{document}"),
            position: TextPosition { line, character },
            qualifier,
            rename_to,
        });
    }
    Ok(EditorProbes {
        queries: probes,
        hover,
    })
}

fn parse_qualified_reference_probes(
    fixture: &str,
    documents: &[SourceDocument],
    fallback_name: &str,
) -> Result<Vec<QualifiedReferenceProbe>, String> {
    let Some(section) = raw_section(fixture, "QUALIFIED REFERENCE QUERIES") else {
        return Ok(Vec::new());
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!(
            "{fallback_name}: malformed QUALIFIED REFERENCE QUERIES fence"
        ));
    };
    let mut probes = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        if fields.next() != Some("resolve") {
            return Err(format!(
                "{fallback_name}: QUALIFIED REFERENCE QUERIES line {} must start with `resolve`",
                line_index + 1
            ));
        }
        let document_name = fields
            .next()
            .ok_or_else(|| format!("{fallback_name}: missing reference document"))?;
        let document = if document_name == "*" {
            None
        } else {
            if !documents
                .iter()
                .any(|candidate| candidate.name == document_name)
            {
                return Err(format!(
                    "{fallback_name}: unknown reference document {document_name:?}"
                ));
            }
            let source = SourceService::new()
                .admit_memory(
                    "snapshot",
                    document_name,
                    String::new(),
                    SourceKind::Workspace,
                )
                .map_err(|error| format!("{fallback_name}: invalid reference document: {error}"))?;
            Some(source.uri().as_str().to_string())
        };
        let qualified_name = fields
            .next()
            .ok_or_else(|| format!("{fallback_name}: missing qualified name"))?
            .to_string();
        let expected_kind = match fields.next() {
            None | Some("*") => None,
            Some(kind) => Some(
                ElementKind::ALL
                    .iter()
                    .copied()
                    .find(|candidate| candidate.as_str() == kind)
                    .ok_or_else(|| {
                        format!("{fallback_name}: unknown expected element kind {kind:?}")
                    })?,
            ),
        };
        if fields.next().is_some() {
            return Err(format!(
                "{fallback_name}: too many qualified-reference fields on line {}",
                line_index + 1
            ));
        }
        probes.push(QualifiedReferenceProbe {
            document,
            qualified_name,
            expected_kind,
        });
    }
    Ok(probes)
}

fn raw_section<'a>(fixture: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("# {name}\n");
    let start = fixture.find(&marker)? + marker.len();
    let rest = &fixture[start..];
    let end = rest.find("\n# ").unwrap_or(rest.len());
    Some(&rest[..end])
}

fn replace_or_insert_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    if let Some(updated) = replace_section(fixture, name, replacement) {
        return Some(updated);
    }
    let insertion = fixture.find("\n# ").unwrap_or(fixture.len());
    let section = format!("\n# {name}\n~~~sexpr\n{replacement}\n~~~");
    let mut updated = String::with_capacity(fixture.len() + section.len());
    updated.push_str(&fixture[..insertion]);
    updated.push_str(&section);
    updated.push_str(&fixture[insertion..]);
    Some(updated)
}

fn replace_or_insert_raw_section(fixture: &str, name: &str, replacement: &str) -> String {
    if let Some(updated) = replace_raw_section(fixture, name, replacement) {
        return updated;
    }
    let mut updated = fixture.trim_end_matches('\n').to_string();
    write!(updated, "\n# {name}\n{replacement}").expect("writing to String cannot fail");
    updated
}

/// Canonical top-level Markdown order. SOURCE is authored; the other sections are owned by this
/// runner. Canonicalization drops sections outside this ownership contract.
const SECTION_ORDER: &[&str] = &[
    "META",
    "SOURCE",
    "EXPECTED DIAGNOSTICS",
    "EXPECTED SEMANTICS",
    "EDITOR QUERIES",
    "QUALIFIED REFERENCE QUERIES",
    "DIAGNOSTICS",
    "SMG",
    "TYPES",
    "NAVIGATION",
    "EDITOR RESULTS",
    "HOVER RESULTS",
    "HOVER MARKDOWN",
    "QUALIFIED REFERENCE RESULTS",
    "GENERATED",
];

fn canonicalize_sections(fixture: &str) -> String {
    let mut sections = Vec::<(&str, &str, usize)>::new();
    let mut marker = None;
    for (offset, line) in fixture.split_inclusive('\n').scan(0usize, |offset, line| {
        let start = *offset;
        *offset += line.len();
        Some((start, line))
    }) {
        let name = line
            .strip_prefix("# ")
            .and_then(|line| line.strip_suffix('\n'));
        if let Some(name) = name {
            if let Some((previous_name, previous_start)) = marker.take() {
                sections.push((
                    previous_name,
                    &fixture[previous_start..offset],
                    previous_start,
                ));
            }
            marker = Some((name, offset));
        }
    }
    if let Some((previous_name, previous_start)) = marker {
        sections.push((previous_name, &fixture[previous_start..], previous_start));
    }
    if sections.len() < 2 {
        return fixture.to_string();
    }
    let prefix_end = sections[0].2;
    let prefix = &fixture[..prefix_end];
    sections.retain(|(name, _, _)| SECTION_ORDER.contains(name));
    sections.sort_by_key(|(name, _, original)| {
        (
            SECTION_ORDER
                .iter()
                .position(|candidate| candidate == name)
                .unwrap_or(SECTION_ORDER.len()),
            *original,
        )
    });
    let mut output = String::with_capacity(fixture.len());
    output.push_str(prefix);
    for (_, body, _) in sections {
        output.push_str(body.trim_end_matches('\n'));
        output.push('\n');
    }
    output
}

fn replace_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    let marker = format!("# {name}\n");
    let section_start = fixture.find(&marker)? + marker.len();
    let section_end = fixture[section_start..]
        .find("\n# ")
        .map_or(fixture.len(), |index| section_start + index);
    let section = &fixture[section_start..section_end];
    fenced_block(section)?;
    let mut updated = String::with_capacity(fixture.len() + replacement.len() + 14);
    updated.push_str(&fixture[..section_start]);
    updated.push_str("~~~sexpr\n");
    updated.push_str(replacement.trim_end_matches('\n'));
    updated.push_str("\n~~~");
    updated.push_str(&fixture[section_end..]);
    Some(updated)
}

fn replace_raw_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    let marker = format!("# {name}\n");
    let section_start = fixture.find(&marker)? + marker.len();
    let section_end = fixture[section_start..]
        .find("\n# ")
        .map_or(fixture.len(), |index| section_start + index);
    let mut updated = String::with_capacity(fixture.len() + replacement.len());
    updated.push_str(&fixture[..section_start]);
    updated.push_str(replacement.trim_end_matches('\n'));
    updated.push('\n');
    updated.push_str(&fixture[section_end..]);
    Some(updated)
}

fn fenced_block(input: &str) -> Option<(String, &str)> {
    let start = input.find("~~~")?;
    let after_open = &input[start + 3..];
    let (_, body) = after_open.split_once('\n')?;
    if let Some(after_close) = body.strip_prefix("~~~") {
        return Some((String::new(), after_close));
    }
    let end = body.find("\n~~~")?;
    Some((body[..end].to_string(), &body[end + 4..]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use spec42_constraint_manifest::{SpecificationManifest, SCHEMA_VERSION};

    #[test]
    fn snapshot_replacement_is_atomic_per_file() {
        let directory = tempfile::tempdir().unwrap();
        let fixture = directory.path().join("fixture.md");
        fs::write(&fixture, "before").unwrap();

        replace_snapshot_files(vec![(fixture.clone(), b"after".to_vec())]).unwrap();

        assert_eq!(fs::read_to_string(fixture).unwrap(), "after");
    }

    #[test]
    fn snapshot_staging_failure_changes_no_targets() {
        let directory = tempfile::tempdir().unwrap();
        let first = directory.path().join("first.md");
        let unavailable = directory.path().join("missing/second.md");
        fs::write(&first, "before").unwrap();

        let error = replace_snapshot_files(vec![
            (first.clone(), b"after".to_vec()),
            (unavailable, b"never".to_vec()),
        ])
        .unwrap_err();

        assert!(error.contains("staging file creation failed"));
        assert_eq!(fs::read_to_string(first).unwrap(), "before");
        assert!(
            fs::read_dir(directory.path()).unwrap().all(|entry| {
                !entry
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".spec42-snapshot-")
            }),
            "failed staging must clean up temporary files"
        );
    }

    #[test]
    fn cli_does_not_allow_a_strategy_override() {
        assert!(
            Cli::try_parse_from(["spec42-snapshot", "check", "--strategy", "parallel"]).is_err()
        );
    }

    #[test]
    fn report_contract_does_not_reintroduce_legacy_or_message_based_classification() {
        let source = include_str!("main.rs");
        let forbidden = [
            ["skip", "_validation"].concat(),
            ["diagnostic", ".message"].concat(),
            ["message", ".contains("].concat(),
            ["code", ".starts_with("].concat(),
            ["code", ".contains("].concat(),
        ];
        for pattern in forbidden {
            assert!(
                !source.contains(&pattern),
                "report behavior must use closed typed contracts, not {pattern:?}"
            );
        }
    }

    #[test]
    fn work_results_are_sorted_for_deterministic_reporting() {
        let mut results = vec![
            FixtureWorkResult {
                path: PathBuf::from("z.md"),
                result: Err("z failure".to_string()),
            },
            FixtureWorkResult {
                path: PathBuf::from("a.md"),
                result: Err("a failure".to_string()),
            },
        ];
        sort_work_results(&mut results);
        assert_eq!(
            results
                .iter()
                .map(|result| result.path.as_path())
                .collect::<Vec<_>>(),
            vec![Path::new("a.md"), Path::new("z.md")]
        );
    }

    /// Every owned section is compared, not only the first: the editor-query section carries the
    /// inspection output, which is the one most likely to depend on construction order.

    #[test]
    fn parses_single_and_multi_source_documents() {
        let single = "# SOURCE\n~~~sysml\npackage A {}\n~~~\n";
        assert_eq!(
            parse_source_documents(single, "single.md").unwrap()[0].text,
            "package A {}"
        );
        let multi = "# SOURCE\n## A.sysml\n~~~sysml\npackage A {}\n~~~\n## B.sysml\n~~~sysml\npackage B {}\n~~~\n";
        let documents = parse_source_documents(multi, "multi.md").unwrap();
        assert_eq!(documents.len(), 2);
        assert_eq!(documents[1].name, "B.sysml");
    }

    #[test]
    fn replaces_existing_section_without_touching_neighbors() {
        let fixture = "# SOURCE\n~~~sysml\npackage A {}\n~~~\n# SMG\n~~~sexpr\nold\n~~~\n# DIAGNOSTICS\n~~~sexpr\nkeep\n~~~\n";
        let updated = replace_section(fixture, "SMG", "new").unwrap();
        assert!(updated.contains("# SMG\n~~~sexpr\nnew\n~~~"));
        assert!(updated.contains("# DIAGNOSTICS\n~~~sexpr\nkeep\n~~~"));
    }

    #[test]
    fn inserting_owned_sections_is_idempotent() {
        let fixture = "# META\n~~~ini\ntype=file\n~~~\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n";
        let first = replace_or_insert_section(fixture, "SMG", "model").unwrap();
        let first = replace_or_insert_section(&first, "DIAGNOSTICS", "diagnostics").unwrap();
        let first = replace_or_insert_section(&first, "NAVIGATION", "navigation").unwrap();
        let second = replace_or_insert_section(&first, "SMG", "model").unwrap();
        let second = replace_or_insert_section(&second, "DIAGNOSTICS", "diagnostics").unwrap();
        let second = replace_or_insert_section(&second, "NAVIGATION", "navigation").unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn update_barrier_preserves_authored_semantic_and_diagnostic_expectations() {
        let fixture = "# META\n~~~ini\nsource_expectation=accepted\nrule_family=check\nexpectation=semantics\nrule_id=sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization\n~~~\n# SOURCE\n~~~sysml\npackage Model {}\n~~~\n# EXPECTED DIAGNOSTICS\n~~~sexpr\n(authored-diagnostics)\n~~~\n# EXPECTED SEMANTICS\n~~~sexpr\n(authored-semantics)\n~~~\n";
        let updated = replace_or_insert_section(fixture, "SMG", "derived-model").unwrap();
        let updated =
            replace_or_insert_section(&updated, "DIAGNOSTICS", "derived-diagnostics").unwrap();
        let canonical = canonicalize_sections(&updated);
        assert!(canonical.contains("# EXPECTED DIAGNOSTICS\n~~~sexpr\n(authored-diagnostics)"));
        assert!(canonical.contains("# EXPECTED SEMANTICS\n~~~sexpr\n(authored-semantics)"));
    }

    #[test]
    fn canonicalizes_shuffled_top_level_sections() {
        let fixture = "# SMG\nold\n# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics)\n~~~\n# NAVIGATION\nnav\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# META\nmeta\n# DIAGNOSTICS\ndiag\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics)\n~~~\n# DIAGNOSTICS\ndiag\n# SMG\nold\n# NAVIGATION\nnav\n"
        );
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }

    #[test]
    fn normalizes_out_of_contract_sections_and_is_idempotent() {
        let fixture = "# META\nmeta\n# SOURCE\nsource\n# EXTRA\nextra\n# DIAGNOSTICS\ndiag\n# NOTES\nnotes\n# FORMAT\nformat\n# SMG\nsmg\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\nsource\n# DIAGNOSTICS\ndiag\n# SMG\nsmg\n"
        );
        assert!(!canonical.contains("# EXTRA\n"));
        assert!(!canonical.contains("# NOTES\n"));
        assert!(!canonical.contains("# FORMAT\n"));
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }

    #[test]
    fn parses_generate_metadata_and_rejects_incomplete_or_conflicting_metadata() {
        let fixture = "# META\n~~~ini\ndescription=Requirements CSV\ntype=generate\nlibraries=standard\nplugin=requirements_csv\n~~~\n";
        assert_eq!(
            parse_fixture_meta(fixture, "fixture.md").unwrap(),
            FixtureMeta {
                libraries: LibrarySelection::Standard,
                repository_sources: Vec::new(),
                generation: Some(GenerationRequest {
                    plugin: GeneratorPlugin::Conformance("requirements_csv".to_string()),
                    diagram_selection: None,
                }),
                standard_library_documents: BTreeSet::new(),
                normative_expectation: None,
                legacy_rule_ids: Vec::new(),
            }
        );

        for (meta, expected) in [
            ("type=generate", "requires a plugin"),
            ("type=file\nplugin=x", "only valid with type=generate"),
            ("type=generate\ntype=file\nplugin=x", "duplicate META key"),
            ("type=generate\nplugin=x\nplugin=y", "duplicate META key"),
            ("type=generate\nnot metadata", "must be key=value"),
        ] {
            let fixture = format!("# META\n~~~ini\n{meta}\n~~~\n");
            let error = parse_fixture_meta(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn untagged_expected_diagnostics_are_authored_exact_assertions() {
        let fixture = "# EXPECTED DIAGNOSTICS\n~~~sexpr\n(diagnostics expected)\n~~~\n";
        assert_eq!(
            check_untagged_diagnostics(fixture, "(diagnostics expected)\n", "fixture.md")
                .unwrap()
                .state,
            ExpectationState::Passed
        );
        let failed =
            check_untagged_diagnostics(fixture, "(diagnostics actual)", "fixture.md").unwrap();
        assert_eq!(failed.state, ExpectationState::Failed);
        assert!(failed.failure.unwrap().contains("do not match"));
    }

    #[test]
    fn parses_typed_normative_metadata_and_rejects_partial_or_duplicate_contracts() {
        let fixture = "# META\n~~~ini\nsource_expectation=accepted\nrule_family=validate\nexpectation=diagnostics\nrule_id=kerml-1.0:8.3:validateThing\nrule_id=sysml-2.0:8.3:validateOther\ncoverage_role=secondary\nblocked_by=diagnostic-gap-1\n~~~\n";
        let meta = parse_fixture_meta(fixture, "fixture.md").unwrap();
        assert_eq!(
            meta.normative_expectation,
            Some(NormativeExpectation {
                source_expectation: SourceExpectation::Accepted,
                rule_family: RuleFamily::Validate,
                expectation: ExpectationKind::Diagnostics,
                rule_ids: vec![
                    "kerml-1.0:8.3:validateThing".to_string(),
                    "sysml-2.0:8.3:validateOther".to_string(),
                ],
                coverage_role: CoverageRole::Secondary,
                blocked_by: Some("diagnostic-gap-1".to_string()),
                evidence: None,
                specification_id: None,
            })
        );
        for (meta, expected) in [
            ("rule_id=kerml-1", "source_expectation is required"),
            (
                "source_expectation=accepted\nrule_family=validate\nexpectation=diagnostics",
                "rule_id is required",
            ),
            (
                "source_expectation=accepted\nrule_family=validate\nexpectation=diagnostics\nrule_id=a\nrule_id=a",
                "duplicate META rule_id",
            ),
            (
                "source_expectation=accepted\nrule_family=validate\nexpectation=diagnostics\nrule_id=a\nevidence_reference=unsupported:a.rs",
                "is not supported",
            ),
            (
                "source_expectation=accepted\nrule_family=derive\nexpectation=diagnostics\nrule_id=a",
                "incompatible",
            ),
            (
                "source_expectation=accepted\nrule_family=validate\nexpectation=semantics\nrule_id=a",
                "incompatible",
            ),
            (
                "source_expectation=accepted\nrule_family=validate\nexpectation=diagnostics\nrule_id=a\ncoverage_role=tertiary",
                "unknown META coverage_role",
            ),
            (
                "source_expectation=accepted\nrule_family=validate\nexpectation=diagnostics\nrule_id=a\nevidence_reference=file:tests/example.md",
                "only valid",
            ),
        ] {
            let fixture = format!("# META\n~~~ini\n{meta}\n~~~\n");
            let error = parse_fixture_meta(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_authored_semantic_relationships_strictly() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (relationship\n    (kind specialization)\n    (source \"Model::Component\")\n    (target \"Parts::Part\")\n    (provenance implied)\n    (outcome resolved))\n  (relationship\n    (kind specialization)\n    (source \"Model::Equivalent\")\n    (provenance implied)\n    (outcome absent)))\n~~~\n";
        assert_eq!(
            parse_expected_semantics(fixture, "fixture.md").unwrap(),
            Some(SemanticExpectations {
                relationships: vec![
                    RelationshipExpectation {
                        kind: SemanticRelationshipKind::Specialization,
                        source: "Model::Component".to_string(),
                        target: Some("Parts::Part".to_string()),
                        provenance: Some(RelationshipProvenance::Implied),
                        outcome: SemanticRelationshipOutcome::Resolved,
                    },
                    RelationshipExpectation {
                        kind: SemanticRelationshipKind::Specialization,
                        source: "Model::Equivalent".to_string(),
                        target: None,
                        provenance: Some(RelationshipProvenance::Implied),
                        outcome: SemanticRelationshipOutcome::Absent,
                    },
                ],
                feature_derived_relationships: Vec::new(),
                type_derived_relationships: Vec::new(),
                type_derived_elements: Vec::new(),
                type_derived_facts: Vec::new(),
                action_derived_facts: Vec::new(),
                definition_usage_derived: Vec::new(),
                requirement_derived_facts: Vec::new(),
                element_derived_owners: Vec::new(),
                element_derived_documentation: Vec::new(),
                namespace_derived_elements: Vec::new(),
                namespace_import_derived_elements: Vec::new(),
                binding_connector_checks: Vec::new(),
                redefinition_checks: Vec::new(),
                specialization_checks: Vec::new(),
            })
        );
        for (body, expected) in [
            (
                "(fixture-semantics (relationship (kind specialization) (source A) (provenance implied) (outcome resolved)))",
                "requires target and provenance",
            ),
            (
                "(fixture-semantics (relationship (kind specialization) (source A) (outcome absent)))",
                "absent semantic relationship requires provenance",
            ),
            (
                "(fixture-semantics (relationship (kind other) (source A) (provenance implied) (outcome unresolved)))",
                "unknown semantic relationship kind",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_manifest_scoped_binding_connector_check_expectations_strictly() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (binding-connector-check\n    (rule_id \"kerml-1.0:8.3.4.8.5:checkFeatureReferenceExpressionBindingConnector\")\n    (outcome unsupported)\n    (prerequisite feature_reference_expression_target_and_result))\n  (binding-connector-check\n    (rule_id \"kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultDefaultValueBindingConnector\")\n    (outcome satisfied)))\n~~~\n";
        let parsed = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .expect("semantic expectations");
        assert_eq!(
            parsed.binding_connector_checks,
            vec![
                BindingConnectorCheckExpectation {
                    rule: BindingConnectorCheckKind::FeatureReferenceExpression,
                    outcome: BindingConnectorCheckOutcome::Unsupported(
                        BindingConnectorValidationPrerequisite::FeatureReferenceExpressionTargetAndResult,
                    ),
                },
                BindingConnectorCheckExpectation {
                    rule: BindingConnectorCheckKind::ConstructorExpressionResultDefaultValueTbd,
                    outcome: BindingConnectorCheckOutcome::Satisfied,
                },
            ]
        );

        let invalid = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics (binding-connector-check (rule_id \"kerml-1.0:8.3.4.8.5:checkFeatureReferenceExpressionBindingConnector\") (outcome unsupported)))\n~~~\n";
        assert!(parse_expected_semantics(invalid, "fixture.md")
            .unwrap_err()
            .contains("requires prerequisite"));
    }

    #[test]
    fn parses_manifest_scoped_redefinition_check_expectations_strictly() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (redefinition-check\n    (rule_id \"kerml-1.0:8.3.3.3.4:checkFeatureEndRedefinition\")\n    (outcome unsupported)\n    (prerequisite end_feature_position_and_inherited_ends))\n  (redefinition-check\n    (rule_id \"sysml-2.0:8.3.26.6:checkRenderingUsageRedefinition\")\n    (outcome unsupported)\n    (prerequisite view_rendering_membership)))\n~~~\n";
        let parsed = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .expect("semantic expectations");
        assert_eq!(
            parsed.redefinition_checks,
            vec![
                RedefinitionCheckExpectation {
                    rule: RedefinitionCheckKind::FeatureEnd,
                    outcome: RedefinitionCheckExpectationOutcome::Unsupported(
                        RedefinitionCheckPrerequisite::EndFeaturePositionAndInheritedEnds,
                    ),
                },
                RedefinitionCheckExpectation {
                    rule: RedefinitionCheckKind::RenderingUsage,
                    outcome: RedefinitionCheckExpectationOutcome::Unsupported(
                        RedefinitionCheckPrerequisite::ViewRenderingMembership,
                    ),
                },
            ]
        );

        let invalid = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics (redefinition-check (rule_id \"kerml-1.0:8.3.3.3.4:checkFeatureEndRedefinition\") (outcome unsupported)))\n~~~\n";
        assert!(parse_expected_semantics(invalid, "fixture.md")
            .unwrap_err()
            .contains("requires prerequisite"));
    }

    #[test]
    fn parses_manifest_scoped_specialization_check_expectations_strictly() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (specialization-check\n    (rule_id \"kerml-1.0:8.3.4.8.8:checkInvocationExpressionSpecialization\")\n    (outcome unsupported)\n    (prerequisite invocation_instantiated_type))\n  (specialization-check\n    (rule_id \"sysml-2.0:8.3.6.4:checkUsageVariationUsageSpecialization\")\n    (outcome satisfied)))\n~~~\n";
        let parsed = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .expect("semantic expectations");
        assert_eq!(
            parsed.specialization_checks,
            vec![
                SpecializationCheckExpectation {
                    rule: SpecializationCheckKind::InvocationExpression,
                    outcome: SpecializationCheckExpectationOutcome::Unsupported(
                        SpecializationCheckPrerequisite::InvocationInstantiatedType,
                    ),
                },
                SpecializationCheckExpectation {
                    rule: SpecializationCheckKind::UsageVariationUsage,
                    outcome: SpecializationCheckExpectationOutcome::Satisfied,
                },
            ]
        );
    }

    #[test]
    fn parses_closed_feature_derived_relationship_collection_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (derived-relationship-collection\n    (rule_id \"kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping\")\n    (source \"Model::Vehicle::mass\")\n    (kind feature_typing)\n    (target \"Model::Mass\")\n    (provenance authored)\n    (outcome resolved))\n  (derived-relationship-collection\n    (rule_id \"kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining\")\n    (source \"Model::Vehicle::path\")\n    (kind feature_chaining)\n    (provenance authored)\n    (outcome unresolved)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .unwrap();
        assert!(expectations.relationships.is_empty());
        assert_eq!(
            expectations.feature_derived_relationships,
            vec![
                FeatureDerivedRelationshipExpectation {
                    collection: FeatureDerivedRelationshipCollection::OwnedTyping,
                    source: "Model::Vehicle::mass".to_string(),
                    kind: SemanticRelationshipKind::FeatureTyping,
                    target: Some("Model::Mass".to_string()),
                    provenance: Some(RelationshipProvenance::Authored),
                    outcome: SemanticRelationshipOutcome::Resolved,
                },
                FeatureDerivedRelationshipExpectation {
                    collection: FeatureDerivedRelationshipCollection::OwnedFeatureChaining,
                    source: "Model::Vehicle::path".to_string(),
                    kind: SemanticRelationshipKind::FeatureChaining,
                    target: None,
                    provenance: Some(RelationshipProvenance::Authored),
                    outcome: SemanticRelationshipOutcome::Unresolved,
                },
            ]
        );
        for (body, expected) in [
            (
                "(fixture-semantics (derived-relationship-collection (rule_id kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping) (source Model::Feature) (kind feature_typing) (target Model::Type) (provenance authored) (outcome resolved) (collection owned_typing)))",
                "unknown semantic derived relationship collection field",
            ),
            (
                "(fixture-semantics (derived-relationship-collection (rule_id kerml-1.0:8.3.3.3.4:deriveFeatureType) (source Model::Feature) (kind feature_typing) (target Model::Type) (provenance authored) (outcome resolved)))",
                "does not own an exact Feature relationship query",
            ),
            (
                "(fixture-semantics (derived-relationship-collection (rule_id kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping) (source Model::Feature) (kind feature_typing) (provenance authored) (outcome resolved)))",
                "resolved semantic derived relationship collection requires target and provenance",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_closed_type_derived_relationship_collection_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (type-derived-relationship-collection\n    (rule_id \"kerml-1.0:8.3.3.1.10:deriveTypeUnioningType\")\n    (source \"Model::Derived\")\n    (kind unioning)\n    (target \"Model::Base\")\n    (provenance authored)\n    (outcome resolved)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .unwrap();
        assert_eq!(
            expectations.type_derived_relationships,
            vec![TypeDerivedRelationshipExpectation {
                collection: TypeDerivedRelationshipCollection::UnioningType,
                source: "Model::Derived".to_string(),
                kind: SemanticRelationshipKind::Unioning,
                target: Some("Model::Base".to_string()),
                provenance: Some(RelationshipProvenance::Authored),
                outcome: SemanticRelationshipOutcome::Resolved,
            }]
        );
        let invalid = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics (type-derived-relationship-collection (rule_id kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity) (source Model::Derived) (kind unioning) (target Model::Base) (provenance authored) (outcome resolved)))\n~~~\n";
        assert!(parse_expected_semantics(invalid, "fixture.md").is_err());
    }

    #[test]
    fn parses_closed_type_derived_element_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (type-derived-element\n    (rule_id \"kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeature\")\n    (source \"Model::Container\")\n    (target \"Model::Container::owned\")\n    (outcome resolved))\n  (type-derived-element\n    (rule_id \"kerml-1.0:8.3.3.1.10:deriveTypeOwnedEndFeature\")\n    (source \"Model::Container\")\n    (target \"Model::Container::endpoint\")\n    (outcome resolved)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .unwrap();
        assert_eq!(
            expectations.type_derived_elements,
            vec![
                TypeDerivedElementExpectation {
                    collection: TypeDerivedElementCollection::OwnedFeature,
                    source: "Model::Container".to_string(),
                    target: Some("Model::Container::owned".to_string()),
                    outcome: TypeDerivedElementOutcome::Resolved,
                },
                TypeDerivedElementExpectation {
                    collection: TypeDerivedElementCollection::OwnedEndFeature,
                    source: "Model::Container".to_string(),
                    target: Some("Model::Container::endpoint".to_string()),
                    outcome: TypeDerivedElementOutcome::Resolved,
                },
            ]
        );
        for (body, expected) in [
            (
                "(fixture-semantics (type-derived-element (rule_id kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeature) (source Model::Container) (outcome resolved)))",
                "requires target",
            ),
            (
                "(fixture-semantics (type-derived-element (rule_id kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeature) (source Model::Container) (target Model::Container::owned) (outcome absent)))",
                "must not declare target",
            ),
            (
                "(fixture-semantics (type-derived-element (rule_id kerml-1.0:8.3.3.1.10:deriveTypeFeature) (source Model::Container) (outcome absent)))",
                "does not own an exact Type element query",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_closed_type_derived_fact_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (type-derived-fact\n    (rule_id \"kerml-1.0:8.3.3.1.10:deriveTypeFeature\")\n    (source \"Model::Container\")\n    (target \"Model::Container::owned\")\n    (outcome resolved))\n  (type-derived-fact\n    (rule_id \"kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity\")\n    (source \"Model::Sized\")\n    (outcome resolved)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .unwrap();
        assert_eq!(
            expectations.type_derived_facts,
            vec![
                TypeDerivedFactExpectation {
                    collection: TypeDerivedFactCollection::Feature,
                    source: "Model::Container".to_string(),
                    target: Some("Model::Container::owned".to_string()),
                    outcome: TypeDerivedElementOutcome::Resolved,
                },
                TypeDerivedFactExpectation {
                    collection: TypeDerivedFactCollection::Multiplicity,
                    source: "Model::Sized".to_string(),
                    target: None,
                    outcome: TypeDerivedElementOutcome::Resolved,
                },
            ]
        );
        for (body, expected) in [
            (
                "(fixture-semantics (type-derived-fact (rule_id kerml-1.0:8.3.3.1.10:deriveTypeFeature) (source Model::Container) (outcome resolved)))",
                "requires target except for multiplicity",
            ),
            (
                "(fixture-semantics (type-derived-fact (rule_id kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity) (source Model::Sized) (target Model::Multiplicity) (outcome absent)))",
                "must not declare target",
            ),
            (
                "(fixture-semantics (type-derived-fact (rule_id kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeature) (source Model::Container) (target Model::Container::owned) (outcome resolved)))",
                "does not own an exact Type fact query",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_closed_action_derived_fact_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (action-derived-fact\n    (rule_id \"sysml-2.0:8.3.17.3:deriveActionDefinitionAction\")\n    (source \"Actions::Procedure\")\n    (target \"Actions::Procedure::step\")\n    (outcome resolved))\n  (action-derived-fact\n    (rule_id \"sysml-2.0:8.3.17.5:deriveAssignmentActionUsageValueExpression\")\n    (source \"Actions::Procedure\")\n    (position 2)\n    (outcome resolved)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .expect("semantic expectations");
        assert_eq!(
            expectations.action_derived_facts,
            vec![
                ActionDerivedFactExpectation {
                    collection: ActionDerivedFactCollection::ActionDefinitionAction,
                    source: "Actions::Procedure".to_string(),
                    target: Some("Actions::Procedure::step".to_string()),
                    position: None,
                    outcome: TypeDerivedElementOutcome::Resolved,
                },
                ActionDerivedFactExpectation {
                    collection: ActionDerivedFactCollection::AssignmentValueExpression,
                    source: "Actions::Procedure".to_string(),
                    target: None,
                    position: Some(2),
                    outcome: TypeDerivedElementOutcome::Resolved,
                },
            ]
        );
        for (body, expected) in [
            (
                "(fixture-semantics (action-derived-fact (rule_id sysml-2.0:8.3.17.3:deriveActionDefinitionAction) (source Actions::Procedure) (target Actions::Procedure::step) (outcome absent)))",
                "must not declare target",
            ),
            (
                "(fixture-semantics (action-derived-fact (rule_id kerml-1.0:8.3.3.1.10:deriveTypeFeature) (source Actions::Procedure) (outcome resolved)))",
                "does not own an exact Action fact query",
            ),
            (
                "(fixture-semantics (action-derived-fact (rule_id sysml-2.0:8.3.17.5:deriveAssignmentActionUsageValueExpression) (source Actions::Procedure) (position 0) (outcome resolved)))",
                "position must be a positive integer",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_manifest_scoped_definition_usage_derivations_strictly() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (definition-usage-derived\n    (rule_id \"sysml-2.0:8.3.6.2:deriveDefinitionOwnedPart\")\n    (source \"Model::Vehicle\")\n    (target \"Model::Vehicle::wheel\")\n    (outcome resolved))\n  (definition-usage-derived\n    (rule_id \"sysml-2.0:8.3.6.4:deriveUsageIsReference\")\n    (source \"Model::vehicle\")\n    (outcome false))\n  (definition-usage-derived\n    (rule_id \"sysml-2.0:8.3.6.4:deriveUsageMayTimeVary\")\n    (source \"Model::vehicle\")\n    (outcome false)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .expect("semantic expectations");
        assert_eq!(
            expectations.definition_usage_derived,
            vec![
                DefinitionUsageDerivedExpectation {
                    kind: DefinitionUsageDerivedKind::DefinitionOwnedPart,
                    source: "Model::Vehicle".to_string(),
                    target: Some("Model::Vehicle::wheel".to_string()),
                    outcome: DefinitionUsageDerivedExpectationOutcome::Resolved,
                },
                DefinitionUsageDerivedExpectation {
                    kind: DefinitionUsageDerivedKind::UsageIsReference,
                    source: "Model::vehicle".to_string(),
                    target: None,
                    outcome: DefinitionUsageDerivedExpectationOutcome::False,
                },
                DefinitionUsageDerivedExpectation {
                    kind: DefinitionUsageDerivedKind::UsageMayTimeVary,
                    source: "Model::vehicle".to_string(),
                    target: None,
                    outcome: DefinitionUsageDerivedExpectationOutcome::False,
                },
            ]
        );
        let invalid = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics (definition-usage-derived (rule_id \"sysml-2.0:8.3.6.2:deriveDefinitionOwnedPart\") (source Model::Vehicle) (outcome unsupported)))\n~~~\n";
        assert!(parse_expected_semantics(invalid, "fixture.md")
            .unwrap_err()
            .contains("requires prerequisite"));
    }

    #[test]
    fn parses_closed_element_owner_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (element-owner\n    (rule_id \"kerml-1.0:8.3.2.1.2:deriveElementOwner\")\n    (source \"Model::Vehicle::mass\")\n    (owner \"Model::Vehicle\")\n    (outcome resolved))\n  (element-owner\n    (rule_id \"kerml-1.0:8.3.2.1.2:deriveElementOwner\")\n    (source \"Model\")\n    (outcome absent)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .unwrap();
        assert_eq!(
            expectations.element_derived_owners,
            vec![
                ElementDerivedOwnerExpectation {
                    kind: ElementDerivedOwnerKind::Owner,
                    source: "Model::Vehicle::mass".to_string(),
                    owner: Some("Model::Vehicle".to_string()),
                    outcome: ElementDerivedOwnerOutcome::Resolved,
                },
                ElementDerivedOwnerExpectation {
                    kind: ElementDerivedOwnerKind::Owner,
                    source: "Model".to_string(),
                    owner: None,
                    outcome: ElementDerivedOwnerOutcome::Absent,
                },
            ]
        );
        for (body, expected) in [
            (
                "(fixture-semantics (element-owner (rule_id kerml-1.0:8.3.2.1.2:deriveElementOwner) (source Model::Vehicle::mass) (outcome resolved)))",
                "requires owner",
            ),
            (
                "(fixture-semantics (element-owner (rule_id kerml-1.0:8.3.2.1.2:deriveElementOwner) (source Model) (owner Model::Vehicle) (outcome absent)))",
                "must not declare owner",
            ),
            (
                "(fixture-semantics (element-owner (rule_id kerml-1.0:8.3.2.1.2:deriveElementName) (source Model) (outcome absent)))",
                "does not own an exact Element owner query",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_closed_element_documentation_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (element-documentation\n    (rule_id \"kerml-1.0:8.3.2.1.2:deriveElementDocumentation\")\n    (source \"Model::Vehicle\")\n    (form documentation)\n    (locale none)\n    (language none)\n    (text \" vehicle documentation \")\n    (outcome resolved))\n  (element-documentation\n    (rule_id \"kerml-1.0:8.3.2.1.2:deriveElementTextualRepresentation\")\n    (source \"Model::Vehicle\")\n    (form textual_representation)\n    (locale none)\n    (language \"Alf\")\n    (text \" vehicle implementation \")\n    (outcome resolved)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .unwrap();
        assert_eq!(
            expectations.element_derived_documentation,
            vec![
                ElementDerivedDocumentationExpectation {
                    collection: ElementDerivedDocumentationCollection::Documentation,
                    source: "Model::Vehicle".to_string(),
                    expected: Some(ExpectedDocumentation {
                        form: AnnotationForm::Documentation,
                        locale: None,
                        language: None,
                        text: " vehicle documentation ".to_string(),
                    }),
                    outcome: ElementDerivedDocumentationOutcome::Resolved,
                },
                ElementDerivedDocumentationExpectation {
                    collection: ElementDerivedDocumentationCollection::TextualRepresentation,
                    source: "Model::Vehicle".to_string(),
                    expected: Some(ExpectedDocumentation {
                        form: AnnotationForm::TextualRepresentation,
                        locale: None,
                        language: Some("Alf".to_string()),
                        text: " vehicle implementation ".to_string(),
                    }),
                    outcome: ElementDerivedDocumentationOutcome::Resolved,
                },
            ]
        );
        for (body, expected) in [
            (
                "(fixture-semantics (element-documentation (rule_id kerml-1.0:8.3.2.1.2:deriveElementDocumentation) (source Model::Vehicle) (form documentation) (locale none) (language none) (outcome resolved)))",
                "requires text",
            ),
            (
                "(fixture-semantics (element-documentation (rule_id kerml-1.0:8.3.2.1.2:deriveElementDocumentation) (source Model::Vehicle) (form textual_representation) (locale none) (language none) (text x) (outcome resolved)))",
                "does not match its manifest-owned collection",
            ),
            (
                "(fixture-semantics (element-documentation (rule_id kerml-1.0:8.3.2.1.2:deriveElementName) (source Model::Vehicle) (outcome absent)))",
                "does not own an exact Element documentation query",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_closed_namespace_derived_element_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (namespace-derived-element\n    (rule_id \"kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedMember\")\n    (source \"Model\")\n    (target \"Model::Owned\")\n    (outcome resolved))\n  (namespace-derived-element\n    (rule_id \"kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedImport\")\n    (source \"Model\")\n    (outcome absent)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .unwrap();
        assert_eq!(
            expectations.namespace_derived_elements,
            vec![
                NamespaceDerivedElementExpectation {
                    collection: NamespaceDerivedElementCollection::OwnedMember,
                    source: "Model".to_string(),
                    target: Some("Model::Owned".to_string()),
                    outcome: NamespaceDerivedElementOutcome::Resolved,
                },
                NamespaceDerivedElementExpectation {
                    collection: NamespaceDerivedElementCollection::OwnedImport,
                    source: "Model".to_string(),
                    target: None,
                    outcome: NamespaceDerivedElementOutcome::Absent,
                },
            ]
        );
        for (body, expected) in [
            (
                "(fixture-semantics (namespace-derived-element (rule_id kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedMember) (source Model) (outcome resolved)))",
                "requires target",
            ),
            (
                "(fixture-semantics (namespace-derived-element (rule_id kerml-1.0:8.3.2.4.5:deriveNamespaceOwnedImport) (source Model) (target Model::Owned) (outcome absent)))",
                "must not declare target",
            ),
            (
                "(fixture-semantics (namespace-derived-element (rule_id kerml-1.0:8.3.2.4.5:deriveNamespaceMembers) (source Model) (outcome absent)))",
                "does not own an exact Namespace element query",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_closed_namespace_import_derived_element_assertions() {
        let fixture = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (namespace-import-derived-element\n    (rule_id \"kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement\")\n    (owner \"Model\")\n    (target \"Library\")\n    (provenance authored)\n    (outcome resolved)))\n~~~\n";
        let expectations = parse_expected_semantics(fixture, "fixture.md")
            .unwrap()
            .unwrap();
        assert_eq!(
            expectations.namespace_import_derived_elements,
            vec![NamespaceImportDerivedElementExpectation {
                kind: NamespaceImportDerivedElementKind::ImportedElement,
                owner: "Model".to_string(),
                target: Some("Library".to_string()),
                provenance: Some(RelationshipProvenance::Authored),
                outcome: SemanticRelationshipOutcome::Resolved,
            }]
        );
        for (body, expected) in [
            (
                "(fixture-semantics (namespace-import-derived-element (rule_id kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement) (owner Model) (provenance authored) (outcome resolved)))",
                "requires target",
            ),
            (
                "(fixture-semantics (namespace-import-derived-element (rule_id kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement) (owner Model) (target Library) (outcome absent)))",
                "requires provenance",
            ),
            (
                "(fixture-semantics (namespace-import-derived-element (rule_id kerml-1.0:8.3.2.4.4:deriveMembershipImportImportedElement) (owner Model) (outcome absent)))",
                "does not own an exact NamespaceImport element query",
            ),
        ] {
            let fixture = format!("# EXPECTED SEMANTICS\n~~~sexpr\n{body}\n~~~\n");
            let error = parse_expected_semantics(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn feature_derived_semantics_use_the_query_facade_with_publication_parity() {
        let fixture = "# META\n~~~ini\nsource_expectation=accepted\nrule_family=derive\nexpectation=semantics\nrule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining\nlibraries=none\n~~~\n# SOURCE\n~~~kerml\npackage Model { classifier Vehicle { feature base; feature derived : Vehicle redefines base chains base; feature unresolved chains Missing; } }\n~~~\n# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics\n  (derived-relationship-collection (rule_id \"kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining\") (source \"Model::Vehicle::derived\") (kind feature_chaining) (target \"Model::Vehicle::base\") (provenance authored) (outcome resolved))\n  (derived-relationship-collection (rule_id \"kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining\") (source \"Model::Vehicle::unresolved\") (kind feature_chaining) (provenance authored) (outcome unresolved))\n  (derived-relationship-collection (rule_id \"kerml-1.0:8.3.3.3.4:deriveFeatureOwnedRedefinition\") (source \"Model::Vehicle::derived\") (kind redefinition) (target \"Model::Vehicle::base\") (provenance authored) (outcome resolved))\n  (derived-relationship-collection (rule_id \"kerml-1.0:8.3.3.3.4:deriveFeatureOwnedSubsetting\") (source \"Model::Vehicle::derived\") (kind redefinition) (target \"Model::Vehicle::base\") (provenance authored) (outcome resolved))\n  (derived-relationship-collection (rule_id \"kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping\") (source \"Model::Vehicle::derived\") (kind feature_typing) (target \"Model::Vehicle\") (provenance authored) (outcome resolved))\n  (derived-relationship-collection (rule_id \"kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTypeFeaturing\") (source \"Model::Vehicle::derived\") (kind type_featuring) (target \"Model::Vehicle\") (provenance implied) (outcome resolved)))\n~~~\n";
        let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("specifications/constraint_manifest.toml");
        let manifest = ConstraintManifest::load_toml(&manifest_path).unwrap();
        let output = regenerate_snapshot(
            fixture,
            Path::new("feature_derived_semantics.md"),
            &LibraryCorpus::new(PathBuf::from("tests/snapshots")),
            &IssueRegistry {
                issues: BTreeMap::new(),
            },
            Some(&manifest),
        )
        .unwrap();
        assert_eq!(output.report.state, ExpectationState::Passed);
        assert!(output.expectation_failure.is_none());
        // Regeneration only owns derived sections. This expectation was queried for canonical,
        // sequential, and parallel publications but remains authored byte-for-byte.
        assert_eq!(
            raw_section(&output.text, "EXPECTED SEMANTICS")
                .unwrap()
                .trim_end_matches('\n'),
            raw_section(fixture, "EXPECTED SEMANTICS")
                .unwrap()
                .trim_end_matches('\n')
        );
    }

    #[test]
    fn semantic_expectation_outcomes_remain_explicit_and_closed() {
        for outcome in ["unresolved", "ambiguous", "unsupported"] {
            let fixture = format!(
                "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics (relationship (kind specialization) (source Model::Component) (provenance implied) (outcome {outcome})))\n~~~\n"
            );
            assert_eq!(
                parse_expected_semantics(&fixture, "fixture.md")
                    .unwrap()
                    .unwrap()
                    .relationships[0]
                    .outcome,
                SemanticRelationshipOutcome::parse(outcome, "fixture.md").unwrap()
            );
        }
        let incomplete = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics (relationship (kind specialization) (source Model::Component) (outcome incomplete)))\n~~~\n";
        assert_eq!(
            parse_expected_semantics(incomplete, "fixture.md")
                .unwrap()
                .unwrap()
                .relationships[0]
                .outcome,
            SemanticRelationshipOutcome::Incomplete
        );
        let invalid = "# EXPECTED SEMANTICS\n~~~sexpr\n(fixture-semantics (relationship (kind specialization) (source Model::Component) (provenance implied) (outcome recovered)))\n~~~\n";
        assert!(parse_expected_semantics(invalid, "fixture.md").is_err());
    }

    #[test]
    fn semantic_expectations_block_mismatches_and_self_expire() {
        let mut issues = BTreeMap::new();
        issues.insert(
            "semantic-gap-1".to_string(),
            IssueEntry {
                id: "semantic-gap-1".to_string(),
                kind: IssueKind::SemanticNotImplemented,
                owner: IssueOwner::Spec42Semantic,
                summary: "The semantic relationship is not published yet.".to_string(),
                tracking: None,
            },
        );
        let registry = IssueRegistry { issues };
        let blocked = check_semantic_expectation(
            Some("semantic-gap-1"),
            &registry,
            true,
            Some("target is unresolved"),
            "fixture.md",
        )
        .unwrap();
        assert_eq!(blocked.state, ExpectationState::Blocked);
        let stale =
            check_semantic_expectation(Some("semantic-gap-1"), &registry, true, None, "fixture.md")
                .unwrap();
        assert_eq!(stale.state, ExpectationState::Stale);
        let missing =
            check_semantic_expectation(None, &registry, false, None, "fixture.md").unwrap();
        assert_eq!(missing.state, ExpectationState::Failed);
        assert!(missing
            .failure
            .unwrap()
            .contains("requires an EXPECTED SEMANTICS section"));
    }

    #[test]
    fn supplemental_diagnostics_are_joint_with_semantic_blocker_lifecycle() {
        let fixture = "# EXPECTED DIAGNOSTICS\n~~~sexpr\n(expected)\n~~~\n";
        let passed = check_supplemental_diagnostic_expectation(
            fixture,
            "(expected)",
            CheckedExpectation {
                state: ExpectationState::Passed,
                failure: None,
            },
            "fixture.md",
        )
        .unwrap();
        assert_eq!(passed.state, ExpectationState::Passed);

        let diagnostic_failure = check_supplemental_diagnostic_expectation(
            fixture,
            "(actual)",
            CheckedExpectation {
                state: ExpectationState::Passed,
                failure: None,
            },
            "fixture.md",
        )
        .unwrap();
        assert_eq!(diagnostic_failure.state, ExpectationState::Failed);
        assert!(diagnostic_failure
            .failure
            .unwrap()
            .contains("EXPECTED DIAGNOSTICS do not match"));

        let stale = check_supplemental_diagnostic_expectation(
            fixture,
            "(expected)",
            CheckedExpectation {
                state: ExpectationState::Stale,
                failure: Some("primary semantic expectation passes".to_string()),
            },
            "fixture.md",
        )
        .unwrap();
        assert_eq!(stale.state, ExpectationState::Stale);

        let still_blocked = check_supplemental_diagnostic_expectation(
            fixture,
            "(actual)",
            CheckedExpectation {
                state: ExpectationState::Stale,
                failure: Some("primary semantic expectation passes".to_string()),
            },
            "fixture.md",
        )
        .unwrap();
        assert_eq!(still_blocked.state, ExpectationState::Blocked);
    }

    #[test]
    fn typed_blockers_are_closed_registry_references_and_self_expire() {
        let mut issues = BTreeMap::new();
        issues.insert(
            "diagnostic-gap-1".to_string(),
            IssueEntry {
                id: "diagnostic-gap-1".to_string(),
                kind: IssueKind::DiagnosticNotImplemented,
                owner: IssueOwner::Spec42Diagnostics,
                summary: "Diagnostic contract is not yet implemented.".to_string(),
                tracking: None,
            },
        );
        let registry = IssueRegistry { issues };
        let expectation = NormativeExpectation {
            source_expectation: SourceExpectation::Accepted,
            rule_family: RuleFamily::Validate,
            expectation: ExpectationKind::Diagnostics,
            rule_ids: vec!["kerml-1".to_string()],
            coverage_role: CoverageRole::Primary,
            blocked_by: Some("diagnostic-gap-1".to_string()),
            evidence: None,
            specification_id: None,
        };
        let fixture = "# EXPECTED DIAGNOSTICS\n~~~sexpr\n(expected)\n~~~\n";
        assert_eq!(
            check_normative_expectation(
                fixture,
                "(actual)",
                &expectation,
                &registry,
                false,
                None,
                "fixture.md",
                Path::new("fixture.md"),
            )
            .unwrap()
            .state,
            ExpectationState::Blocked
        );
        let stale = check_normative_expectation(
            fixture,
            "(expected)",
            &expectation,
            &registry,
            false,
            None,
            "fixture.md",
            Path::new("fixture.md"),
        )
        .unwrap();
        assert_eq!(stale.state, ExpectationState::Stale);
        assert!(stale.failure.unwrap().contains("remove META blocked_by"));
    }

    #[test]
    fn source_intent_and_blocker_ownership_are_explicit_contracts() {
        let parser_issue = IssueEntry {
            id: "parser-gap-1".to_string(),
            kind: IssueKind::ParserGap,
            owner: IssueOwner::SysmlV2Parser,
            summary: "Parser drops valid syntax.".to_string(),
            tracking: None,
        };
        validate_issue_owner(&parser_issue).unwrap();
        let registry = IssueRegistry {
            issues: BTreeMap::from([(parser_issue.id.clone(), parser_issue)]),
        };
        let expectation = NormativeExpectation {
            source_expectation: SourceExpectation::Accepted,
            rule_family: RuleFamily::Validate,
            expectation: ExpectationKind::Diagnostics,
            rule_ids: vec!["kerml-1.0:8.3.2.1.2:validateThing".to_string()],
            coverage_role: CoverageRole::Primary,
            blocked_by: Some("parser-gap-1".to_string()),
            evidence: None,
            specification_id: None,
        };
        validate_blocker_contract(&expectation, &registry, "fixture.md", false).unwrap();
        let malformed = vec![ObservedDiagnostic {
            category: "malformed_syntax".to_string(),
            origin: "parser".to_string(),
            severity: "error".to_string(),
        }];
        assert!(validate_source_intent(
            &FixtureMeta {
                libraries: LibrarySelection::None,
                repository_sources: Vec::new(),
                generation: None,
                standard_library_documents: BTreeSet::new(),
                normative_expectation: Some(NormativeExpectation {
                    blocked_by: None,
                    ..expectation.clone()
                }),
                legacy_rule_ids: Vec::new(),
            },
            &registry,
            &malformed,
            "fixture.md",
        )
        .unwrap()
        .is_some());
        let invalid = NormativeExpectation {
            source_expectation: SourceExpectation::Malformed,
            ..expectation
        };
        assert!(validate_blocker_contract(&invalid, &registry, "fixture.md", false).is_err());
    }

    #[test]
    fn diagnostic_owner_can_block_only_a_supplemental_semantic_diagnostic() {
        let issue = IssueEntry {
            id: "diagnostic-gap-1".to_string(),
            kind: IssueKind::DiagnosticNotImplemented,
            owner: IssueOwner::Spec42Diagnostics,
            summary: "Diagnostic projection is not published.".to_string(),
            tracking: None,
        };
        let registry = IssueRegistry {
            issues: BTreeMap::from([(issue.id.clone(), issue)]),
        };
        let expectation = NormativeExpectation {
            source_expectation: SourceExpectation::Accepted,
            rule_family: RuleFamily::Check,
            expectation: ExpectationKind::Semantics,
            rule_ids: vec!["sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization".to_string()],
            coverage_role: CoverageRole::Primary,
            blocked_by: Some("diagnostic-gap-1".to_string()),
            evidence: None,
            specification_id: None,
        };
        assert!(validate_blocker_contract(&expectation, &registry, "fixture.md", false).is_err());
        validate_blocker_contract(&expectation, &registry, "fixture.md", true).unwrap();
    }

    #[test]
    fn source_role_metadata_is_explicit_and_closed() {
        let fixture = "# META\n~~~ini\nlibraries=none\nstandard_library_document=parts-a.sysml\nstandard_library_document=parts-b.sysml\n~~~\n";
        let meta = parse_fixture_meta(fixture, "fixture.md").unwrap();
        assert_eq!(
            meta.standard_library_documents,
            BTreeSet::from(["parts-a.sysml".to_string(), "parts-b.sysml".to_string()])
        );
        let conflicting =
            "# META\n~~~ini\nlibraries=standard\nstandard_library_document=parts.sysml\n~~~\n";
        assert!(parse_fixture_meta(conflicting, "fixture.md")
            .unwrap_err()
            .contains("cannot be combined"));
        let duplicate = "# META\n~~~ini\nlibraries=none\nstandard_library_document=parts.sysml\nstandard_library_document=parts.sysml\n~~~\n";
        assert!(parse_fixture_meta(duplicate, "fixture.md")
            .unwrap_err()
            .contains("duplicate META standard_library_document"));
    }

    #[test]
    fn by_construction_requires_typed_evidence_unless_abstract_syntax_is_blocked() {
        let mut issues = BTreeMap::new();
        issues.insert(
            "abstract-gap-1".to_string(),
            IssueEntry {
                id: "abstract-gap-1".to_string(),
                kind: IssueKind::AbstractSyntaxCoverageGap,
                owner: IssueOwner::Spec42Snapshot,
                summary: "The invalid abstract syntax cannot be authored.".to_string(),
                tracking: None,
            },
        );
        let registry = IssueRegistry { issues };
        let blocked = NormativeExpectation {
            source_expectation: SourceExpectation::Accepted,
            rule_family: RuleFamily::Validate,
            expectation: ExpectationKind::ByConstruction,
            rule_ids: vec!["kerml-1.0:8.3.2.1.2:validateThing".to_string()],
            coverage_role: CoverageRole::Primary,
            blocked_by: Some("abstract-gap-1".to_string()),
            evidence: None,
            specification_id: None,
        };
        assert_eq!(
            check_by_construction_expectation(
                &blocked,
                &registry,
                "fixture.md",
                Path::new("fixture.md")
            )
            .unwrap()
            .state,
            ExpectationState::Blocked
        );

        let passed = NormativeExpectation {
            blocked_by: None,
            evidence: Some(ByConstructionEvidence::File(PathBuf::from(
                "tools/snapshot_tool/src/main.rs",
            ))),
            ..blocked
        };
        assert_eq!(
            check_by_construction_expectation(
                &passed,
                &registry,
                "fixture.md",
                Path::new("fixture.md")
            )
            .unwrap()
            .state,
            ExpectationState::Passed
        );
        let absent_evidence = NormativeExpectation {
            evidence: None,
            ..passed.clone()
        };
        assert_eq!(
            check_by_construction_expectation(
                &absent_evidence,
                &registry,
                "fixture.md",
                Path::new("fixture.md")
            )
            .unwrap()
            .state,
            ExpectationState::Failed
        );
        assert!(
            ByConstructionEvidence::parse("test:../../outside.rs", "fixture.md")
                .unwrap_err()
                .contains("repository-relative")
        );
        let fixture = "# EXPECTED DIAGNOSTICS\n~~~sexpr\n(expected)\n~~~\n";
        let supplemental_failure = check_normative_expectation(
            fixture,
            "(actual)",
            &passed,
            &registry,
            false,
            None,
            "fixture.md",
            Path::new("fixture.md"),
        )
        .unwrap();
        assert_eq!(supplemental_failure.state, ExpectationState::Failed);
    }

    #[test]
    fn manifest_audit_counts_occurrences_and_reports_strict_coverage_debt() {
        let directory = tempfile::tempdir().unwrap();
        let first = directory.path().join("first.md");
        let second = directory.path().join("second.md");
        fs::write(
            &first,
            "# META\n~~~ini\nsource_expectation=accepted\nrule_family=validate\nexpectation=diagnostics\nrule_id=kerml-1.0:8.3.2.1.2:validateThing\nspecification_id=sysml-2.0\n~~~\n",
        )
        .unwrap();
        fs::write(
            &second,
            "# META\n~~~ini\nsource_expectation=accepted\nrule_family=validate\nexpectation=diagnostics\nrule_id=kerml-1.0:8.3.2.1.2:validateThing\nrule_id=kerml-1.0:8.3.2.9.9:checkOther\n~~~\n",
        )
        .unwrap();
        let manifest = ConstraintManifest {
            schema_version: SCHEMA_VERSION,
            specifications: vec![SpecificationManifest {
                name: "KerML".to_string(),
                version: "1.0".to_string(),
                formal_document_id: "formal/26-03-02".to_string(),
                xmi_file_id: "ptc/test".to_string(),
                xmi_sha256: "xmi".to_string(),
                pdf_sha256: "pdf".to_string(),
                constraints: vec![
                    ConstraintManifestEntry {
                        package: "Root".to_string(),
                        metaclass: "Thing".to_string(),
                        constraint: "validateThing".to_string(),
                        family: ConstraintFamily::Derive,
                        clause: "8.3.2.1.2".to_string(),
                        specializes_from_library: None,
                        conditional_specializes_from_library: None,
                        redefines_from_library: None,
                        feature_derived_relationship: None,
                        type_derived_relationship: None,
                        type_derived_element: None,
                        type_derived_fact: None,
                        type_featuring_check: None,
                        redefinition_check: None,
                        specialization_check: None,
                        element_derived_owner: None,
                        element_derived_documentation: None,
                        namespace_derived_element: None,
                        namespace_import_derived_element: None,
                        binding_connector_check: None,
                        definition_usage_derived: None,
                        action_derived_fact: None,
                        requirement_derived_fact: None,
                        rule_id: "kerml-1.0:8.3.2.1.2:validateThing".to_string(),
                    },
                    ConstraintManifestEntry {
                        package: "Root".to_string(),
                        metaclass: "Other".to_string(),
                        constraint: "checkOther".to_string(),
                        family: ConstraintFamily::Check,
                        clause: "8.3.2.2.2".to_string(),
                        specializes_from_library: None,
                        conditional_specializes_from_library: None,
                        redefines_from_library: None,
                        feature_derived_relationship: None,
                        type_derived_relationship: None,
                        type_derived_element: None,
                        type_derived_fact: None,
                        type_featuring_check: None,
                        redefinition_check: None,
                        specialization_check: None,
                        element_derived_owner: None,
                        element_derived_documentation: None,
                        namespace_derived_element: None,
                        namespace_import_derived_element: None,
                        binding_connector_check: None,
                        definition_usage_derived: None,
                        action_derived_fact: None,
                        requirement_derived_fact: None,
                        rule_id: "kerml-1.0:8.3.2.2.2:checkOther".to_string(),
                    },
                ],
            }],
        };
        let audit = audit_manifest_coverage(
            &manifest,
            &[first.clone(), second.clone()],
            std::slice::from_ref(&first),
        )
        .unwrap();
        assert_eq!(audit.manifest_rule_occurrences, 2);
        assert_eq!(audit.manifest_unique_rule_ids, 2);
        assert_eq!(audit.fixture_rule_occurrences, 3);
        assert_eq!(audit.fixture_unique_rule_ids, 2);
        assert_eq!(audit.selected_fixture_rule_occurrences, 1);
        assert_eq!(audit.selected_fixture_unique_rule_ids, 1);
        assert!(audit
            .duplicate_primary_rule_ids
            .contains_key("kerml-1.0:8.3.2.1.2:validateThing"));
        assert!(audit
            .family_mismatches
            .contains_key("kerml-1.0:8.3.2.1.2:validateThing"));
        assert!(audit
            .specification_mismatches
            .contains_key("kerml-1.0:8.3.2.1.2:validateThing"));
        assert!(audit
            .formal_document_mismatches
            .contains_key("kerml-1.0:8.3.2.1.2:validateThing"));
        assert!(audit
            .unknown_rule_ids
            .contains_key("kerml-1.0:8.3.2.9.9:checkOther"));
        assert!(audit
            .missing_rule_ids
            .contains(&"kerml-1.0:8.3.2.2.2:checkOther".to_string()));
        assert!(!audit.is_clean());
    }

    #[test]
    fn manifest_audit_accepts_secondary_cases_only_with_primary_evidence() {
        let directory = tempfile::tempdir().unwrap();
        let primary = directory.path().join("primary.md");
        let secondary = directory.path().join("secondary.md");
        let rule_id = "sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization";
        fs::write(
            &primary,
            format!("# META\n~~~ini\nsource_expectation=accepted\nrule_family=check\nexpectation=semantics\nrule_id={rule_id}\n~~~\n"),
        )
        .unwrap();
        fs::write(
            &secondary,
            format!("# META\n~~~ini\nsource_expectation=accepted\nrule_family=check\nexpectation=semantics\nrule_id={rule_id}\ncoverage_role=secondary\n~~~\n"),
        )
        .unwrap();
        let manifest = ConstraintManifest {
            schema_version: SCHEMA_VERSION,
            specifications: vec![SpecificationManifest {
                name: "SysML".to_string(),
                version: "2.0".to_string(),
                formal_document_id: "formal/26-03-02".to_string(),
                xmi_file_id: "ptc/test".to_string(),
                xmi_sha256: "xmi".to_string(),
                pdf_sha256: "pdf".to_string(),
                constraints: vec![ConstraintManifestEntry {
                    package: "Root".to_string(),
                    metaclass: "PartDefinition".to_string(),
                    constraint: "checkPartDefinitionSpecialization".to_string(),
                    family: ConstraintFamily::Check,
                    clause: "8.3.11.2".to_string(),
                    specializes_from_library: None,
                    conditional_specializes_from_library: None,
                    redefines_from_library: None,
                    feature_derived_relationship: None,
                    type_derived_relationship: None,
                    type_derived_element: None,
                    type_derived_fact: None,
                    type_featuring_check: None,
                    redefinition_check: None,
                    specialization_check: None,
                    element_derived_owner: None,
                    element_derived_documentation: None,
                    namespace_derived_element: None,
                    namespace_import_derived_element: None,
                    binding_connector_check: None,
                    definition_usage_derived: None,
                    action_derived_fact: None,
                    requirement_derived_fact: None,
                    rule_id: rule_id.to_string(),
                }],
            }],
        };
        let audit = audit_manifest_coverage(
            &manifest,
            &[primary.clone(), secondary.clone()],
            &[primary, secondary],
        )
        .unwrap();
        assert_eq!(audit.fixture_rule_occurrences, 2);
        assert_eq!(audit.fixture_unique_rule_ids, 1);
        assert!(audit.duplicate_primary_rule_ids.is_empty());
        assert!(audit.orphan_secondary_rule_ids.is_empty());
        assert!(audit.is_clean());
    }

    #[test]
    fn manifest_audit_rejects_secondary_evidence_without_a_primary_fixture() {
        let directory = tempfile::tempdir().unwrap();
        let secondary = directory.path().join("secondary.md");
        let rule_id = "sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization";
        fs::write(
            &secondary,
            format!("# META\n~~~ini\nsource_expectation=accepted\nrule_family=check\nexpectation=semantics\nrule_id={rule_id}\ncoverage_role=secondary\n~~~\n"),
        )
        .unwrap();
        let manifest = ConstraintManifest {
            schema_version: SCHEMA_VERSION,
            specifications: vec![SpecificationManifest {
                name: "SysML".to_string(),
                version: "2.0".to_string(),
                formal_document_id: "formal/test".to_string(),
                xmi_file_id: "ptc/test".to_string(),
                xmi_sha256: "xmi".to_string(),
                pdf_sha256: "pdf".to_string(),
                constraints: vec![ConstraintManifestEntry {
                    package: "Root".to_string(),
                    metaclass: "PartDefinition".to_string(),
                    constraint: "checkPartDefinitionSpecialization".to_string(),
                    family: ConstraintFamily::Check,
                    clause: "8.3.11.2".to_string(),
                    specializes_from_library: None,
                    conditional_specializes_from_library: None,
                    redefines_from_library: None,
                    feature_derived_relationship: None,
                    type_derived_relationship: None,
                    type_derived_element: None,
                    type_derived_fact: None,
                    type_featuring_check: None,
                    redefinition_check: None,
                    specialization_check: None,
                    element_derived_owner: None,
                    element_derived_documentation: None,
                    namespace_derived_element: None,
                    namespace_import_derived_element: None,
                    binding_connector_check: None,
                    definition_usage_derived: None,
                    action_derived_fact: None,
                    requirement_derived_fact: None,
                    rule_id: rule_id.to_string(),
                }],
            }],
        };
        let audit = audit_manifest_coverage(
            &manifest,
            std::slice::from_ref(&secondary),
            std::slice::from_ref(&secondary),
        )
        .unwrap();
        assert!(audit.orphan_secondary_rule_ids.contains_key(rule_id));
        assert!(audit.missing_rule_ids.contains(&rule_id.to_string()));
        assert!(!audit.is_clean());
    }

    #[test]
    fn issue_registry_rejects_invalid_schema_and_unreferenced_entries() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        fs::write(
            root.join("issues.toml"),
            "schema_version = 1\n\n[[issue]]\nid = \"parser-gap-1\"\nkind = \"parser_gap\"\nowner = \"sysml-v2-parser\"\nsummary = \"Parser loses a required construct.\"\n",
        )
        .unwrap();
        let fixture = root.join("fixture.md");
        fs::write(&fixture, "# SOURCE\n~~~sysml\npackage A {}\n~~~\n").unwrap();
        let registry = IssueRegistry::load(root).unwrap();
        assert!(validate_registry_references(&registry, &[fixture])
            .unwrap_err()
            .contains("not referenced"));
        fs::write(root.join("issues.toml"), "schema_version = 2\n").unwrap();
        assert!(IssueRegistry::load(root)
            .unwrap_err()
            .contains("unsupported issue registry schema_version"));
    }

    #[test]
    fn registry_rejects_unclassified_validation_fixture() {
        let directory = tempfile::tempdir().unwrap();
        let validation = directory.path().join("tests/snapshots/validation");
        fs::create_dir_all(&validation).unwrap();
        let fixture = validation.join("fixture.md");
        fs::write(&fixture, "# SOURCE\n~~~sysml\npackage A {}\n~~~\n").unwrap();
        let registry = IssueRegistry {
            issues: BTreeMap::new(),
        };
        assert!(validate_registry_references(&registry, &[fixture])
            .unwrap_err()
            .contains("unclassified validation fixture"));
    }

    #[test]
    fn normative_validation_scope_excludes_legacy_sysml_validation_snapshots() {
        assert!(is_validation_fixture(Path::new(
            "tests/snapshots/validation/normative.md"
        )));
        assert!(!is_validation_fixture(Path::new(
            "tests/snapshots/sysml/validation/10a_analysis.md"
        )));
    }

    #[test]
    fn report_aggregates_are_deterministic_and_typed() {
        let report = SnapshotReport::new(
            vec![
                FixtureReport {
                    path: "z.md".to_string(),
                    rule_ids: vec!["rule-z".to_string()],
                    source_expectation: Some(SourceExpectation::Accepted),
                    rule_family: Some(RuleFamily::Validate),
                    expectation: Some(ExpectationKind::Diagnostics),
                    state: ExpectationState::Blocked,
                    blocked_by: Some(ReportBlocker {
                        id: "parser-gap-1".to_string(),
                        kind: IssueKind::ParserGap,
                        owner: IssueOwner::SysmlV2Parser,
                        summary: "Parser gap".to_string(),
                    }),
                    by_construction_evidence: None,
                    diagnostics: vec![ObservedDiagnostic {
                        category: "validation".to_string(),
                        origin: "semantic".to_string(),
                        severity: "error".to_string(),
                    }],
                },
                FixtureReport {
                    path: "a.md".to_string(),
                    rule_ids: Vec::new(),
                    source_expectation: None,
                    rule_family: None,
                    expectation: None,
                    state: ExpectationState::Passed,
                    blocked_by: None,
                    by_construction_evidence: None,
                    diagnostics: Vec::new(),
                },
            ],
            &[],
            &[],
            None,
        );
        assert_eq!(report.fixtures[0].path, "a.md");
        assert_eq!(report.aggregate.expectations.get("blocked"), Some(&1));
        assert_eq!(report.aggregate.expectations.get("passed"), Some(&1));
        assert_eq!(report.aggregate.expectations.get("failed"), Some(&0));
        assert_eq!(report.aggregate.expectations.get("stale"), Some(&0));
        assert_eq!(
            report.aggregate.expectations.get("not_applicable"),
            Some(&0)
        );
        assert_eq!(report.aggregate.outstanding_issues.len(), 1);
        assert_eq!(
            report.aggregate.outstanding_issues[0].affected_fixture_count,
            1
        );
        assert_eq!(
            report.aggregate.observed_diagnostics.get("validation"),
            Some(&DiagnosticAggregate {
                occurrences: 1,
                affected_fixture_count: 1,
                origins: BTreeMap::from([(
                    "semantic".to_string(),
                    DiagnosticOriginAggregate {
                        occurrences: 1,
                        affected_fixture_count: 1,
                        severities: BTreeMap::from([("error".to_string(), 1)]),
                    },
                )]),
            })
        );
        let text = render_report_text(&report);
        assert_eq!(text, render_report_text(&report));
        assert!(text.starts_with("passed a.md rules=- blocker=-\nblocked z.md"));
        assert!(text.contains("unclassified=0"));
        assert!(text.contains("stale=0 failed=0 not_applicable=0"));
        assert!(text.contains("fixture: z.md"));
        let summary = render_check_summary(&report);
        assert!(summary.starts_with("BLOCKED z.md issue=parser-gap-1 kind=parser_gap"));
        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("\"parser_gap\""));
        assert!(json.contains("\"blocked\""));
        let json_value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(json_value["fixtures"][0]["path"], "a.md");
        assert_eq!(json_value["aggregate"]["expectations"]["stale"], 0);
        assert_eq!(
            json_value["aggregate"]["outstanding_issues"][0]["fixtures"][0],
            "z.md"
        );
        assert_eq!(
            json_value["aggregate"]["observed_diagnostics"]["validation"]["origins"]["semantic"]
                ["severities"]["error"],
            1
        );
    }

    #[test]
    fn report_paths_never_expose_absolute_host_paths() {
        assert_eq!(
            normalized_report_path(Path::new("/private/tmp/external-fixture.md")),
            "<outside-repository>/external-fixture.md"
        );
    }

    #[test]
    fn report_distinguishes_executable_and_abstract_by_construction_evidence() {
        let report = SnapshotReport::new(
            vec![
                FixtureReport {
                    path: "executable.md".to_string(),
                    rule_ids: vec!["kerml-1.0:8.3.2.1.2:deriveThing".to_string()],
                    source_expectation: Some(SourceExpectation::Accepted),
                    rule_family: Some(RuleFamily::Derive),
                    expectation: Some(ExpectationKind::ByConstruction),
                    state: ExpectationState::Passed,
                    blocked_by: None,
                    by_construction_evidence: Some(ByConstructionEvidenceStatus::Executable),
                    diagnostics: Vec::new(),
                },
                FixtureReport {
                    path: "abstract.md".to_string(),
                    rule_ids: vec!["kerml-1.0:8.3.2.1.3:deriveOther".to_string()],
                    source_expectation: Some(SourceExpectation::Accepted),
                    rule_family: Some(RuleFamily::Derive),
                    expectation: Some(ExpectationKind::ByConstruction),
                    state: ExpectationState::Blocked,
                    blocked_by: None,
                    by_construction_evidence: Some(
                        ByConstructionEvidenceStatus::AbstractSyntaxCoverageGap,
                    ),
                    diagnostics: Vec::new(),
                },
            ],
            &[],
            &[],
            None,
        );
        let coverage = &report.aggregate.normative_coverage["derive"];
        assert_eq!(coverage.by_construction_fixture_count, 2);
        assert_eq!(
            coverage.by_construction_executable_evidence_fixture_count,
            1
        );
        assert_eq!(
            coverage.by_construction_abstract_coverage_gap_fixture_count,
            1
        );
        assert_eq!(coverage.by_construction_missing_evidence_fixture_count, 0);
        let json = serde_json::to_value(&report).unwrap();
        assert_eq!(
            json["fixtures"][0]["by_construction_evidence"],
            "abstract_syntax_coverage_gap"
        );
    }

    #[test]
    fn report_exit_semantics_are_independent_of_generated_snapshot_staleness() {
        assert!(report_exit_result(false, ManifestAuditHealth::Clean).is_ok());
        assert!(report_exit_result(false, ManifestAuditHealth::CoverageDebt)
            .unwrap_err()
            .contains("manifest coverage debt"));
        assert!(report_exit_result(false, ManifestAuditHealth::Failed)
            .unwrap_err()
            .contains("manifest audit failure"));
        assert!(report_exit_result(true, ManifestAuditHealth::Clean)
            .unwrap_err()
            .contains("failed expectations"));
    }

    #[test]
    fn report_preserves_fixture_results_when_manifest_audit_fails() {
        let report = SnapshotReport::new(
            Vec::new(),
            &[],
            &[],
            Some(ManifestAuditOutcome::Failed {
                error: "manifest unavailable".to_string(),
            }),
        );
        let text = render_report_text(&report);
        assert!(text.contains("manifest audit:\n  failed: manifest unavailable"));
        let json = serde_json::to_value(&report).unwrap();
        assert_eq!(json["manifest_audit"]["state"], "failed");
        assert_eq!(json["manifest_audit"]["error"], "manifest unavailable");
    }

    #[test]
    fn parses_closed_typed_diagram_selection() {
        let diagram = "# META\n~~~ini\ntype=generate\nplugin=repository:diagram\nviewKind=general-view\nviewDocument=model.sysml\nviewQualifiedName=Example::selected\n~~~\n";
        assert_eq!(
            parse_fixture_meta(diagram, "fixture.md")
                .unwrap()
                .generation,
            Some(GenerationRequest {
                plugin: GeneratorPlugin::RepositoryDiagram,
                diagram_selection: Some(DiagramSelection {
                    kind: "general-view".to_string(),
                    document: "model.sysml".to_string(),
                    qualified_name: "Example::selected".to_string(),
                }),
            })
        );
    }

    #[test]
    fn parses_qualified_reference_probe_mechanics() {
        let fixture = "# SOURCE\n## model.sysml\n~~~sysml\npackage Example {}\n~~~\n# QUALIFIED REFERENCE QUERIES\n~~~text\nresolve model.sysml Example::selected ViewUsage\nresolve * StandardViewDefinitions::GeneralView *\n~~~\n";
        assert_eq!(
            parse_qualified_reference_probes(
                fixture,
                &[SourceDocument {
                    name: "model.sysml".to_string(),
                    text: "package Example {}".to_string(),
                }],
                "fixture.md",
            )
            .unwrap(),
            vec![
                QualifiedReferenceProbe {
                    document: Some("memory://snapshot/model.sysml".to_string()),
                    qualified_name: "Example::selected".to_string(),
                    expected_kind: Some(ElementKind::ViewUsage),
                },
                QualifiedReferenceProbe {
                    document: None,
                    qualified_name: "StandardViewDefinitions::GeneralView".to_string(),
                    expected_kind: None,
                },
            ]
        );
    }

    #[test]
    fn parses_hover_as_an_editor_probe_projection() {
        let fixture = "# SOURCE\n## model.sysml\n~~~sysml\npackage Example {}\n~~~\n# EDITOR QUERIES\n~~~text\nprobe model.sysml 0 8 hover\nprobe model.sysml 0 16 rename=Renamed\n~~~\n";
        let parsed = parse_editor_probes(
            fixture,
            &[SourceDocument {
                name: "model.sysml".to_string(),
                text: "package Example {}".to_string(),
            }],
            "fixture.md",
        )
        .unwrap();

        assert_eq!(parsed.queries.len(), 2);
        assert_eq!(parsed.hover, BTreeSet::from([0]));
        assert_eq!(parsed.queries[1].rename_to.as_deref(), Some("Renamed"));
    }

    #[test]
    fn canonicalizes_hover_projection_sections_after_editor_results() {
        let fixture = "# HOVER MARKDOWN\nrendered\n# SOURCE\nsource\n# HOVER RESULTS\nreport\n# META\nmeta\n# EDITOR RESULTS\neditor\n";
        let canonical = canonicalize_sections(fixture);
        let editor = canonical.find("# EDITOR RESULTS").unwrap();
        let report = canonical.find("# HOVER RESULTS").unwrap();
        let markdown = canonical.find("# HOVER MARKDOWN").unwrap();

        assert!(editor < report && report < markdown);
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }

    #[test]
    fn rejects_invalid_generator_selection_metadata() {
        for (meta, expected) in [
            (
                "type=generate\nplugin=repository:diagram\nviewKind=general-view",
                "must be specified together",
            ),
            (
                "type=generate\nplugin=requirements_csv\nviewKind=general-view\nviewDocument=model.sysml\nviewQualifiedName=Example::selected",
                "only valid with plugin=repository:diagram",
            ),
            ("type=generate\nplugin=../../escape", "unknown or unsafe"),
            (
                "type=file\nviewKind=general-view\nviewDocument=model.sysml\nviewQualifiedName=Example::selected",
                "only valid with type=generate",
            ),
        ] {
            let fixture = format!("# META\n~~~ini\n{meta}\n~~~\n");
            let error = parse_fixture_meta(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn repository_plugin_paths_are_closed() {
        assert!(generator_plugin_path(&GeneratorPlugin::RepositoryDiagram)
            .ends_with("generator-plugins/target/wasm32-unknown-unknown/release/spec42_diagram_generator.wasm"));
        assert!(generator_plugin_path(&GeneratorPlugin::Conformance("example".to_string()))
            .ends_with("generator-tests/plugins/target/wasm32-unknown-unknown/release/spec42_conformance_example.wasm"));
    }

    #[test]
    fn generated_artifacts_are_sorted_and_use_path_specific_fences() {
        let mut artifacts = GeneratedArtifacts::default();
        artifacts
            .insert_utf8("z/report.json", "{\"ok\":true}\n".to_string())
            .unwrap();
        artifacts
            .insert_utf8("requirements.csv", "name\nSafeStop\n".to_string())
            .unwrap();
        let rendered = render_generated_artifacts(&artifacts);
        assert_eq!(
            rendered,
            "## requirements.csv\n~~~csv\nname\nSafeStop\n\n~~~\n## z/report.json\n~~~json\n{\"ok\":true}\n\n~~~\n"
        );
        let fixture = format!("# GENERATED\n{rendered}");
        assert_eq!(
            parse_generated_artifacts(&fixture, "fixture.md").unwrap(),
            Some(artifacts)
        );
    }

    #[test]
    fn generated_section_is_inserted_last_and_replaced_as_a_complete_artifact_set() {
        let fixture = "# GENERATED\n## stale.txt\n~~~text\nstale\n~~~\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# META\n~~~ini\ntype=generate\nplugin=x\n~~~\n";
        let mut artifacts = GeneratedArtifacts::default();
        artifacts
            .insert_utf8("fresh.csv", "name\nA\n".to_string())
            .unwrap();
        let updated = replace_or_insert_generated_section(fixture, &artifacts);
        let canonical = canonicalize_sections(&updated);
        assert!(!canonical.contains("stale.txt"));
        assert!(canonical.ends_with("# GENERATED\n## fresh.csv\n~~~csv\nname\nA\n\n~~~\n"));
        assert_eq!(canonicalize_sections(&canonical), canonical);

        let without_generated = "# META\nmeta\n# SOURCE\nsource\n";
        let inserted = canonicalize_sections(&replace_or_insert_generated_section(
            without_generated,
            &artifacts,
        ));
        assert!(inserted.ends_with("# GENERATED\n## fresh.csv\n~~~csv\nname\nA\n\n~~~\n"));
    }

    #[test]
    fn generated_artifact_paths_are_safe_and_unique() {
        let mut artifacts = GeneratedArtifacts::default();
        artifacts
            .insert_utf8("ok/report.csv", String::new())
            .unwrap();
        assert!(artifacts
            .insert_utf8("ok/report.csv", String::new())
            .unwrap_err()
            .contains("duplicate"));
        for path in [
            "",
            "/absolute.csv",
            "../escape.csv",
            "a/../escape.csv",
            "./same.csv",
        ] {
            assert!(
                GeneratedArtifacts::default()
                    .insert_utf8(path, String::new())
                    .is_err(),
                "accepted {path:?}"
            );
        }
    }
}
