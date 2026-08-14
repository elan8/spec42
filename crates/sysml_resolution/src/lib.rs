//! Opaque parser-owned semantic construction and batch resolution.
//!
//! Syntax documents, dense IDs, semantic storage, solver state, and indexes remain private. The
//! public contract accepts immutable source inputs and streams owner-defined canonical output.

use std::fmt;

use source_identity::{ContentDigest, RootDigest, SourceManifest, SourceManifestEntry, SourceRole};

mod element_kind;
mod model;

pub use element_kind::{ElementKind, MembershipRole, StateSubactionKind};

use model::resolver::ResolvedSemanticModel;
use model::{BuildSchedule, CoordinatorError, OwnedSourceRecord, SemanticModelBuildCoordinator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Workspace,
    StandardLibrary,
    Library,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceInput {
    identity: Box<str>,
    content: String,
    kind: SourceKind,
    content_digest: ContentDigest,
}

impl SourceInput {
    pub fn new(identity: impl Into<Box<str>>, content: String, kind: SourceKind) -> Self {
        let content_digest = ContentDigest::of_bytes(content.as_bytes());
        Self {
            identity: identity.into(),
            content,
            kind,
            content_digest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPosition {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextRange {
    pub start: TextPosition,
    pub end: TextPosition,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolIdentity(Box<str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OccurrenceRole {
    Declaration,
    Reference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub document: Box<str>,
    pub range: TextRange,
    pub role: OccurrenceRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationTarget {
    pub symbol: SymbolIdentity,
    pub name: Box<str>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryOutcome<T> {
    Resolved(T),
    Recovered(T),
    UnsupportedWith(T),
    Unresolved,
    Ambiguous(Box<[T]>),
    Unsupported,
    Recovery,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenameOutcome {
    Ready {
        symbol: SymbolIdentity,
        name: Box<str>,
        range: TextRange,
        occurrences: Box<[SourceLocation]>,
    },
    Unresolved,
    Ambiguous(Box<[NavigationTarget]>),
    InvalidName,
    Collision(Box<[NavigationTarget]>),
    Unsupported,
    Recovery,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleMember {
    pub symbol: SymbolIdentity,
    pub name: Box<str>,
    pub kind: ElementKind,
    /// The role this member plays in its owner, where the OMG carries that on the owning
    /// membership rather than on the element; `None` for an ordinary member.
    pub role: Option<MembershipRole>,
    pub qualified_name: Box<str>,
    pub container_name: Option<Box<str>>,
    pub declaring_document: Box<str>,
    pub declaration_range: TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationCompleteness {
    Complete,
    ParseRecovery,
    UnsupportedSyntax,
    NonConverged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstructionSchedule {
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationIdentity {
    source_digest: RootDigest,
    semantic_contract_version: Box<str>,
}

impl PublicationIdentity {
    pub fn source_digest(&self) -> &RootDigest {
        &self.source_digest
    }

    pub fn semantic_contract_version(&self) -> &str {
        &self.semantic_contract_version
    }
}

#[derive(Debug)]
pub struct BuildRequest {
    sources: Vec<SourceInput>,
    schedule: ConstructionSchedule,
    identity: PublicationIdentity,
}

impl BuildRequest {
    pub fn new(
        mut sources: Vec<SourceInput>,
        schedule: ConstructionSchedule,
        semantic_contract_version: impl Into<Box<str>>,
    ) -> Result<Self, BuildFailure> {
        sources.sort_unstable_by(|left, right| left.identity.cmp(&right.identity));
        if sources
            .windows(2)
            .any(|pair| pair[0].identity == pair[1].identity)
        {
            return Err(BuildFailure::DuplicateSourceIdentity);
        }
        let semantic_contract_version = semantic_contract_version.into();
        let entries = sources
            .iter()
            .map(|source| SourceManifestEntry {
                uri: source.identity.to_string(),
                path_hint: None,
                role: source_role(source.kind),
                content_digest: source.content_digest,
                byte_len: source.content.len() as u64,
                library_root_slot: None,
                relative_path: None,
            })
            .collect();
        let source_digest = SourceManifest::new(entries, Vec::new()).root_digest();
        Ok(Self {
            sources,
            schedule,
            identity: PublicationIdentity {
                source_digest,
                semantic_contract_version,
            },
        })
    }

    pub fn identity(&self) -> &PublicationIdentity {
        &self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildFailure {
    DuplicateSourceIdentity,
    ConstructionFailed,
}

impl fmt::Display for BuildFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for BuildFailure {}

/// An opaque, immutable resolved publication.
///
/// Publications are shared by reference; cloning the semantic owner is intentionally impossible.
///
/// ```compile_fail
/// fn requires_clone<T: Clone>() {}
/// requires_clone::<sysml_resolution::PublishedResolution>();
/// ```
///
/// Dense storage identities and indexes are private implementation details.
///
/// ```compile_fail
/// use sysml_resolution::{DeclarationId, ResolutionResults, SemanticModelStorage};
/// ```
#[derive(Debug)]
pub struct PublishedResolution {
    identity: PublicationIdentity,
    model: ResolvedSemanticModel,
}

// SAFETY: a publication is fully constructed before this type is created and exposes only shared
// queries. Its parser documents own immutable source/AST storage; the only interior mutation is
// `OnceLock`-backed source line indexing, whose implementation is thread-safe. The parser AST is a
// deeply recursive owned enum for which rustc's auto-trait solver overflows in downstream async
// hosts, so the publication boundary states the invariant explicitly.
unsafe impl Send for PublishedResolution {}
unsafe impl Sync for PublishedResolution {}

pub fn build(request: BuildRequest) -> Result<PublishedResolution, BuildFailure> {
    let schedule = match request.schedule {
        ConstructionSchedule::Sequential => BuildSchedule::Sequential,
        ConstructionSchedule::Parallel => BuildSchedule::Parallel,
    };
    let sources = request
        .sources
        .into_iter()
        .map(|source| OwnedSourceRecord {
            identity: source.identity,
            content: source.content,
        })
        .collect();
    let model =
        SemanticModelBuildCoordinator::build(sources, schedule).map_err(|error| match error {
            CoordinatorError::DuplicateSourceIdentity => BuildFailure::DuplicateSourceIdentity,
            CoordinatorError::ConstructionFailed => BuildFailure::ConstructionFailed,
        })?;
    Ok(PublishedResolution {
        identity: request.identity,
        model,
    })
}

impl PublishedResolution {
    pub fn identity(&self) -> &PublicationIdentity {
        &self.identity
    }

    pub fn debug(&self) -> DebugQueries<'_> {
        DebugQueries {
            identity: &self.identity,
            model: &self.model,
        }
    }

    pub fn completeness(&self) -> PublicationCompleteness {
        self.model.completeness()
    }

    pub fn target_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<NavigationTarget> {
        self.model.target_at(document, position)
    }

    pub fn references(
        &self,
        symbol: &SymbolIdentity,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        self.model.references(symbol, include_declaration)
    }

    pub fn prepare_rename(
        &self,
        document: &str,
        position: TextPosition,
        new_name: Option<&str>,
    ) -> RenameOutcome {
        self.model.prepare_rename(document, position, new_name)
    }

    pub fn visible_members(
        &self,
        document: &str,
        position: TextPosition,
        qualifier: Option<&str>,
    ) -> QueryOutcome<Box<[VisibleMember]>> {
        self.model.visible_members(document, position, qualifier)
    }
}

pub struct DebugQueries<'a> {
    identity: &'a PublicationIdentity,
    model: &'a ResolvedSemanticModel,
}

impl DebugQueries<'_> {
    pub fn write_semantic_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_semantic_sexpr(
            &self.identity.source_digest,
            &self.identity.semantic_contract_version,
            output,
        )
    }

    pub fn write_diagnostics_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_diagnostics_sexpr(output)
    }

    pub fn write_navigation_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_navigation_sexpr(output)
    }
}

fn source_role(kind: SourceKind) -> SourceRole {
    match kind {
        SourceKind::Workspace => SourceRole::Workspace,
        SourceKind::StandardLibrary => SourceRole::StandardLibrary,
        SourceKind::Library => SourceRole::Library,
        SourceKind::External => SourceRole::External,
    }
}

/// Raw semantic storage is deliberately inaccessible.
///
/// ```compile_fail
/// use sysml_resolution::{DeclarationId, ResolutionResults, SemanticModelStorage};
/// ```
///
/// ```compile_fail
/// fn require_clone<T: Clone>() {}
/// require_clone::<sysml_resolution::BuildRequest>();
/// require_clone::<sysml_resolution::PublishedResolution>();
/// ```
pub struct RawStorageIsNotPublic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_schedule_does_not_change_semantic_publication_identity() {
        let sequential =
            BuildRequest::new(Vec::new(), ConstructionSchedule::Sequential, "contract-v1").unwrap();
        let parallel =
            BuildRequest::new(Vec::new(), ConstructionSchedule::Parallel, "contract-v1").unwrap();

        assert_eq!(sequential.identity(), parallel.identity());
    }

    fn semantic_sexpr_for(source: &str) -> String {
        let request = BuildRequest::new(
            vec![SourceInput::new(
                "memory://test.sysml",
                source.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        let published = build(request).unwrap();
        let mut output = String::new();
        published.debug().write_semantic_sexpr(&mut output).unwrap();
        output
    }

    /// Like `semantic_sexpr_for`, but renders the per-document diagnostics sexpr (which carries
    /// the actual `unsupported_*_definition_member` diagnostic codes) instead of the semantic
    /// model sexpr (which only carries the coarser `(completeness unsupported-syntax)` summary
    /// flag) -- needed for tests asserting a *specific* diagnostic code is present, not merely
    /// that publication completeness is degraded.
    fn diagnostics_sexpr_for(source: &str) -> String {
        let request = BuildRequest::new(
            vec![SourceInput::new(
                "memory://test.sysml",
                source.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        let published = build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        output
    }

    /// A nested `part` usage inside an `attribute def` body (BNF `AttributeBodyElement::PartUsage`,
    /// shared with `item def`/`item` usage bodies per the OMG `14c-Language Extensions.sysml`
    /// FMEA library example) must lower as its own `part` declaration, not fall through to
    /// `unsupported_attribute_member`.
    #[test]
    fn nested_part_usage_inside_attribute_def_body_lowers_as_part() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute def Show { part frame : Frame; attribute def Frame; } }",
        );
        assert!(
            sexpr.contains("P::Show::frame"),
            "expected nested part declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_attribute_member"),
            "did not expect unsupported_attribute_member, got: {sexpr}"
        );
    }

    /// A nested `item` usage inside an `attribute def` body (BNF
    /// `AttributeBodyElement::ItemUsage`, resolved upstream in `0757de13` --
    /// planning/UPSTREAM_PARSER_GAPS.md #11) must lower as its own `item` declaration via the
    /// already-existing `lower_item_usage`, not fall through to `unsupported_attribute_member`.
    #[test]
    fn nested_item_usage_inside_attribute_def_body_lowers_as_item() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute def Show { item picture : Picture; attribute def Picture; } }",
        );
        assert!(
            sexpr.contains("P::Show::picture"),
            "expected nested item declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_attribute_member"),
            "did not expect unsupported_attribute_member, got: {sexpr}"
        );
    }

    /// A nested `occurrence` usage inside an `attribute def` body (BNF
    /// `AttributeBodyElement::OccurrenceUsage`, e.g. the FMEA library's `#prevention occurs;`-style
    /// members) must lower as its own `occurrence` declaration via the already-existing
    /// `lower_occurrence_usage`, not fall through to `unsupported_attribute_member`.
    #[test]
    fn nested_occurrence_usage_inside_attribute_def_body_lowers_as_occurrence() {
        let sexpr = semantic_sexpr_for("package P { attribute def Show { occurrence flash; } }");
        assert!(
            sexpr.contains("P::Show::flash"),
            "expected nested occurrence declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_attribute_member"),
            "did not expect unsupported_attribute_member, got: {sexpr}"
        );
    }

    /// A `first X then Y;` control-flow succession statement inside an `action def` body (BNF
    /// `ActionDefBodyElement::FirstStmt`) must resolve both ends as `succession` relationships
    /// against the two sibling owned action declarations, not fall through to
    /// `unsupported_action_definition_member`.
    #[test]
    fn first_then_succession_inside_action_def_body_resolves_both_ends() {
        let sexpr = semantic_sexpr_for(
            "package P { action def ExecuteMission { action validateRoute; action startMission; first validateRoute then startMission; } }",
        );
        assert!(
            sexpr.contains("(kind succession)"),
            "expected a succession relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        // Both ends resolve to their sibling declarations, not unresolved/unsupported.
        assert!(
            sexpr.matches("(kind succession)").count() >= 2,
            "expected a succession reference for both the `first` and `then` ends, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)")
                || !sexpr.contains("succession"),
            "did not expect an unresolved succession outcome for two declared siblings, got: {sexpr}"
        );
    }

    /// A `first X then Y;` succession whose `then` target is not declared anywhere in the model
    /// must stay an explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn first_then_succession_unresolvable_target_stays_unresolved() {
        let sexpr = semantic_sexpr_for(
            "package P { action def ExecuteMission { action validateRoute; first validateRoute then missingAction; } }",
        );
        assert!(
            sexpr.contains("(kind succession)"),
            "expected a succession reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable `then` target to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A `state def` body's `entry action X;` / `do action Y;` / `exit action Z;` bindings (BNF
    /// `EntryAction`/`DoAction`/`ExitAction.action_reference`) must each resolve to the enclosing
    /// package's action declarations (there is no `StateDefBodyElement::ActionUsage` shape --
    /// bound actions are ordinarily declared alongside the state def, not nested inside it,
    /// mirroring the real corpus fixture `24_state_actions.md`), not fall through to
    /// `unsupported_state_definition_member`.
    #[test]
    fn entry_do_exit_action_bindings_inside_state_def_body_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { action enter1; action running1; action leave1; state def S { entry action enter1; do action running1; exit action leave1; } }",
        );
        assert!(
            sexpr.contains("(kind entryActionBinding)"),
            "expected an entryActionBinding relationship kind, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind doActionBinding)"),
            "expected a doActionBinding relationship kind, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind exitActionBinding)"),
            "expected an exitActionBinding relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_state_definition_member"),
            "did not expect unsupported_state_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected all three action bindings to resolve to their sibling declarations, got: {sexpr}"
        );
    }

    /// An `entry action X;` binding whose target is not declared anywhere in the model must stay
    /// an explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn entry_action_binding_unresolvable_target_stays_unresolved() {
        let sexpr = semantic_sexpr_for("package P { state def S { entry action missingAction; } }");
        assert!(
            sexpr.contains("(kind entryActionBinding)"),
            "expected an entryActionBinding reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable entry action target to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A `state def` body's `then <target>;` initial-state marker (BNF `ThenStmt.state_reference`)
    /// must resolve to the sibling owned state declaration, not fall through to
    /// `unsupported_state_definition_member`.
    #[test]
    fn then_initial_state_inside_state_def_body_resolves() {
        let sexpr = semantic_sexpr_for("package P { state def S { state off; then off; } }");
        assert!(
            sexpr.contains("(kind initialState)"),
            "expected an initialState relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_state_definition_member"),
            "did not expect unsupported_state_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the `then` target to resolve to its sibling state declaration, got: {sexpr}"
        );
    }

    /// A `then <target>;` initial-state marker whose target is not declared anywhere in the model
    /// must stay an explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn then_initial_state_unresolvable_target_stays_unresolved() {
        let sexpr = semantic_sexpr_for("package P { state def S { then missingState; } }");
        assert!(
            sexpr.contains("(kind initialState)"),
            "expected an initialState reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable `then` target to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A nested `exhibit` state usage inside an `occurrence def`/usage body (BNF
    /// `OccurrenceBodyElement::StateUsage`, e.g. `exhibit vehicleStates.on;` from the OMG spec
    /// Annex's individuals/snapshots examples) must lower as its own `state` declaration via the
    /// already-existing `lower_state_usage`, not fall through to
    /// `unsupported_occurrence_definition_member`.
    #[test]
    fn nested_state_usage_inside_occurrence_def_body_lowers_as_state() {
        let sexpr =
            semantic_sexpr_for("package P { occurrence def O { exhibit vehicleStates.on; } }");
        assert!(
            sexpr.contains("(kind state)"),
            "expected nested state declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_occurrence_definition_member"),
            "did not expect unsupported_occurrence_definition_member, got: {sexpr}"
        );
    }

    /// A `transition ... first X then Y;` body element's `source`/`target` operands must each
    /// resolve to their sibling state declarations, not fall through to
    /// `unsupported_state_definition_member`.
    #[test]
    fn transition_source_and_target_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { state off; state on; transition first off then on; } }",
        );
        assert!(
            sexpr.contains("(kind transitionSource)"),
            "expected a transitionSource relationship kind, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind transitionTarget)"),
            "expected a transitionTarget relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected both transition ends to resolve to their sibling state declarations, got: {sexpr}"
        );
    }

    /// A transition whose `source`/`target` are not declared anywhere in the model must stay an
    /// explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn transition_source_and_target_unresolvable_stay_unresolved() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { transition first missingOff then missingOn; } }",
        );
        assert!(
            sexpr.contains("(kind transitionSource)") && sexpr.contains("(kind transitionTarget)"),
            "expected transitionSource/transitionTarget references to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable transition ends to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A transition `if <guard>;` boolean expression with literal comparison operands must
    /// evaluate to a constant `Boolean` through the exact same `classify_constraint_expression`/
    /// `EvalNode` machinery a `constraint`/`calc` body uses (see `9f63c5a4` and earlier
    /// expression/evaluation slices), not a separate transition-specific evaluator.
    #[test]
    fn transition_guard_with_literal_operands_evaluates() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { state off; state on; transition first off if 1 < 2 then on; } }",
        );
        assert!(
            sexpr.contains("(value (boolean true))") || sexpr.contains("(boolean true)"),
            "expected the literal guard `1 < 2` to fold to a constant true, got: {sexpr}"
        );
    }

    /// A transition guard referencing an operand with no known constant value must stay
    /// non-constant, not fabricate a truth value.
    #[test]
    fn transition_guard_with_unresolvable_operand_stays_non_constant() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { state off; state on; transition first off if missingFlag then on; } }",
        );
        assert!(
            sexpr.contains("(kind expressionOperand)"),
            "expected the guard's feature reference to be lowered as an expressionOperand, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(value (boolean true))") && !sexpr.contains("(value (boolean false))"),
            "did not expect an unresolvable guard operand to fold to a concrete boolean, got: {sexpr}"
        );
    }

    /// A transition's shorthand `accept <trigger>;` and `do action <effect>;` clauses must each
    /// resolve to their sibling declarations, not fall through to
    /// `unsupported_state_definition_member`.
    #[test]
    fn transition_trigger_and_effect_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { action doStuff; state def S { state off; state on; transition first off accept trigger1 do doStuff then on; } action trigger1; }",
        );
        assert!(
            sexpr.contains("(kind transitionTrigger)"),
            "expected a transitionTrigger relationship kind, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind transitionEffect)"),
            "expected a transitionEffect relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected both the trigger and effect to resolve to their sibling declarations, got: {sexpr}"
        );
    }

    /// A standalone `decide <expr>;` decision control node (BNF `DecisionStmt`) must lower as a
    /// `DeclarationKind::Decide` feature whose `decide` operand resolves as a `decisionInput`
    /// reference to its sibling action, not fall through to `unsupported_action_definition_member`.
    #[test]
    fn decide_stmt_input_resolves() {
        let sexpr = semantic_sexpr_for("package P { action def A { action x; decide x; } }");
        assert!(
            sexpr.contains("(kind decide)"),
            "expected a decide declaration, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind decisionInput)"),
            "expected a decisionInput relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the decide operand to resolve to its sibling action, got: {sexpr}"
        );
    }

    /// A `decide <expr>;` node whose operand is not declared anywhere in the model must stay an
    /// explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn decide_stmt_unresolvable_input_stays_unresolved() {
        let sexpr = semantic_sexpr_for("package P { action def A { decide missing; } }");
        assert!(
            sexpr.contains("(kind decisionInput)"),
            "expected a decisionInput reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable decide operand to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A `then decide <expr>;` continuation (`ThenTarget::Decide`) inside an action body must
    /// lower through the same `lower_first_merge_stmt` dispatch as a standalone `decide`
    /// statement, not fall through to `unsupported_action_definition_member`.
    #[test]
    fn then_decide_target_lowers_as_decide_declaration() {
        let sexpr = semantic_sexpr_for("package P { action def A { action x; then decide x; } }");
        assert!(
            sexpr.contains("(kind decide)") && sexpr.contains("(kind decisionInput)"),
            "expected a decide declaration reached via `then decide`, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// A standalone `merge <expr>;`/`fork <expr>;`/`join <expr>;` control node must each lower as
    /// their own anonymous declaration kind with a resolved input reference, mirroring `decide`.
    #[test]
    fn merge_fork_join_stmts_resolve() {
        let sexpr =
            semantic_sexpr_for("package P { action def A { action x; merge x; fork x; join x; } }");
        for kind in ["merge", "fork", "join"] {
            assert!(
                sexpr.contains(&format!("(kind {kind})")),
                "expected a {kind} declaration, got: {sexpr}"
            );
            assert!(
                sexpr.contains(&format!("(kind {kind}Input)")),
                "expected a {kind}Input relationship kind, got: {sexpr}"
            );
        }
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected every control node's input to resolve to the sibling action, got: {sexpr}"
        );
    }

    /// A bare `then <target>;` continuation (`ThenTarget::Feature`) referencing an
    /// already-declared sibling action must resolve as a `thenTarget` reference sourced at the
    /// enclosing action, not fall through to `unsupported_action_definition_member`.
    #[test]
    fn then_target_feature_resolves() {
        let sexpr = semantic_sexpr_for("package P { action def A { action x; then x; } }");
        assert!(
            sexpr.contains("(kind thenTarget)"),
            "expected a thenTarget relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the `then` target to resolve to its sibling action, got: {sexpr}"
        );
    }

    /// A `then accept <sig>;` shorthand trigger (`ThenTarget::Accept`, `TransitionAccept::
    /// Shorthand`) must resolve its expression operand through the same constraint-expression
    /// machinery as an ordinary `accept`, not fall through to `unsupported_action_definition_member`.
    #[test]
    fn then_accept_shorthand_resolves_its_payload() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute def Sig; action def A { then accept Sig; } }",
        );
        assert!(
            sexpr.contains("(kind expressionOperand)"),
            "expected an expressionOperand reference for the accept payload, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// A `then accept at <expr>;` time trigger whose expression is a `new Type(...)` invocation
    /// must resolve the invocation callee through the existing `Expression::Invocation`/
    /// `InvocationCallee` machinery (session `1c035232`), reused unchanged here.
    #[test]
    fn then_accept_at_time_trigger_resolves_invocation_callee() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute def Time; action def A { then accept at new Time(); } }",
        );
        assert!(
            sexpr.contains("(kind invocationCallee)"),
            "expected an invocationCallee reference for the `new Time()` constructor, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// A `then accept when <boolExpr>;` change trigger must resolve its dotted feature-chain
    /// operand as a `memberAccessOperand` reference, reusing the general `MemberAccess` machinery
    /// (session `64318c70`) directly rather than duplicating it.
    #[test]
    fn then_accept_when_resolves_member_access_operand() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action b { attribute f; } then accept when b.f; } }",
        );
        assert!(
            sexpr.contains("(kind memberAccessOperand)"),
            "expected a memberAccessOperand reference for `b.f`, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// A standalone `action <name> send via <source> to <target>;` action-usage shorthand (an
    /// `ActionUsage` with `send`/`via`/`to` all set on the usage itself, distinct from the
    /// `then send ...;` continuation form blocked by planning/UPSTREAM_PARSER_GAPS.md Gap 30) must resolve
    /// its `via`/`to` operands, mirroring satisfy/allocate/bind's two-operand pattern via
    /// `lower_satisfy_operand`.
    #[test]
    fn send_action_usage_via_and_to_targets_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action aa; action b; action snd2 send via aa to b; } }",
        );
        assert!(
            sexpr.contains("(kind sendTarget)"),
            "expected a sendTarget reference for the `to b` clause, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind acceptVia)"),
            "expected an acceptVia reference for the `via aa` clause, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected both the send usage's `via` and `to` targets to resolve, got: {sexpr}"
        );
    }

    /// A `Transition`'s own `accept at <expr>`/`accept when <expr>`/`accept after <expr>` time
    /// trigger (`TransitionAccept::TimeTrigger`) previously fell through to
    /// `unsupported_state_definition_member` unconditionally. It now mirrors
    /// `lower_then_accept`'s `TimeTrigger` arm, lowering the trigger expression through the
    /// general constraint-expression dispatch (picking up `MemberAccess` chains like
    /// `vehicle.maintenanceTime`, not just bare `FeatureRef` names).
    #[test]
    fn transition_time_trigger_resolves_member_access_operand() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Vehicle { attribute maintenanceTime; } state def S { in vehicle : Vehicle; state a; state b; accept at vehicle.maintenanceTime then b; } }",
        );
        assert!(
            sexpr.contains("(kind memberAccessOperand)"),
            "expected a memberAccessOperand reference for `vehicle.maintenanceTime`, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_state_definition_member"),
            "did not expect unsupported_state_definition_member, got: {sexpr}"
        );
    }

    /// `RequirementDefBodyElement::VariantUsage` (a bare `variant <name>;` member inside a
    /// `requirement def`/usage body, e.g. inside a `variation`-flavored requirement choice) was
    /// unconditionally unsupported even though `lower_variant_usage` is already shared by
    /// `part def`/`part usage` bodies for the identical AST node. Wires the existing lowering
    /// into the requirement-shaped body walker.
    #[test]
    fn requirement_def_variant_usage_resolves() {
        let sexpr = semantic_sexpr_for(
            "package P { requirement def R1; requirement def R2; requirement def choice { variant R1; variant R2; } }",
        );
        assert!(
            sexpr.contains("(kind variant)"),
            "expected a variant reference, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_requirement_definition_member"),
            "did not expect unsupported_requirement_definition_member, got: {sexpr}"
        );
    }

    /// `RequirementDefBodyElement::RequireConstraint` (`require constraint { ... }`/`assume
    /// constraint <name> { ... }`) was unconditionally unsupported even though its body is the
    /// exact same `ConstraintDefBody`-shaped `elements` list `lower_constraint_def_body` already
    /// walks for `Constraint`/`AssertConstraintMember`. Wires the anonymous and named forms into
    /// the requirement-shaped body walker (`lower_require_constraint_member`), covering both
    /// `require`/`assume` spellings.
    #[test]
    fn require_and_assume_constraint_members_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute massActual; attribute massReqd; requirement def R { require constraint { massActual <= massReqd } assume constraint fuelOk { massActual >= 0 } } }",
        );
        assert!(
            sexpr.matches("(kind constraint)").count() >= 2,
            "expected two constraint declarations (anonymous `require` + named `assume fuelOk`), got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_requirement_definition_member"),
            "did not expect unsupported_requirement_definition_member, got: {sexpr}"
        );
    }

    /// A bare `require;`-less-constraint shorthand (`has_constraint_keyword == false`, e.g.
    /// `require someExistingConstraint;`) references an existing constraint by a plain `String`
    /// name rather than declaring one -- the typed AST gives no `QualifiedReferenceId` to resolve
    /// through the shared lexical-lookup machinery (planning/UPSTREAM_PARSER_GAPS.md #44), so it must stay
    /// an explicit unsupported diagnostic rather than being silently dropped or guessed at.
    #[test]
    fn require_shorthand_reference_without_constraint_keyword_stays_unsupported() {
        let sexpr =
            diagnostics_sexpr_for("package P { constraint c; requirement def R { require c; } }");
        assert!(
            sexpr.contains("unsupported_requirement_definition_member"),
            "expected the constraint-keyword-less `require c;` shorthand to remain unsupported, got: {sexpr}"
        );
    }

    /// A state def/usage body's bare `entry;`/`do;`/`exit;` (no `action` reference, no body
    /// content) is a legal no-op marker -- pervasive in the training/validation corpus (e.g.
    /// `entry; then off;`) -- and must not be reported as `unsupported_state_definition_member`
    /// merely because it has no bound action reference to lower.
    #[test]
    fn bare_entry_do_exit_with_no_reference_or_body_is_not_unsupported() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { state off; entry; do; exit; then off; } }",
        );
        assert!(
            !sexpr.contains("unsupported_state_definition_member"),
            "did not expect unsupported_state_definition_member for bare entry/do/exit, got: {sexpr}"
        );
    }

    /// An inline `entry { <members> }` anonymous action body (non-empty brace, no `action`
    /// reference) genuinely has no representation in the `EntryAction` typed AST and must stay an
    /// explicit unsupported diagnostic, distinguishing it from the empty/semicolon no-op case
    /// above.
    #[test]
    fn entry_with_inline_body_content_and_no_reference_stays_unsupported() {
        let sexpr = diagnostics_sexpr_for("package P { state def S { entry { state inner; } } }");
        assert!(
            sexpr.contains("unsupported_state_definition_member"),
            "expected an inline non-empty entry body with no reference to remain unsupported, got: {sexpr}"
        );
    }

    /// A state def/usage body's `final <name>;` body element (BNF `FinalState`) declares a new
    /// named final pseudo-state, distinct from `then <target>;`'s reference-to-an-existing-state
    /// shape. Must lower as its own `DeclarationKind::FinalState` feature, not fall through to
    /// `unsupported_state_definition_member`.
    #[test]
    fn final_state_declares_named_pseudo_state() {
        let sexpr = semantic_sexpr_for("package P { state def S { final done; } }");
        assert!(
            sexpr.contains("(kind final-state)"),
            "expected a final-state declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_state_definition_member"),
            "did not expect unsupported_state_definition_member, got: {sexpr}"
        );
    }

    /// The `then send new S() to b;` continuation shorthand is a genuine parser gap (see
    /// planning/UPSTREAM_PARSER_GAPS.md Gap 30, `ThenTarget` has no `Send` variant): the parser itself
    /// cannot represent it as a distinguishable `ThenAction` target, so it falls to parser-level
    /// recovery rather than admitting a typed node `sysml_resolution` could silently mis-resolve.
    #[test]
    fn then_send_continuation_stays_unsupported_pending_gap_30() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute def S; action def A { action b; then send new S() to b; } }",
        );
        assert!(
            sexpr.contains("(completeness parse-recovery)"),
            "expected `then send ...;` to remain a parser-recovery gap (Gap 30), got: {sexpr}"
        );
    }

    /// A bare `flow <source> to <target>;` statement (distinct from a named/typed flow usage or
    /// def) must lower as its own `DeclarationKind::Flow` feature with `from`/`to` resolved as
    /// `memberAccessOperand` dotted feature-chain references, reusing
    /// `resolve_member_access_reference` for both ends.
    #[test]
    fn bare_flow_stmt_resolves_source_and_target() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action aa { out part target; } action snd { in receiver; } flow aa.target to snd.receiver; } }",
        );
        assert!(
            sexpr.contains("(kind flow)"),
            "expected a flow declaration, got: {sexpr}"
        );
        assert!(
            sexpr.matches("(kind memberAccessOperand)").count() >= 2,
            "expected both flow ends to resolve as memberAccessOperand references, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// A `TextualRepresentation` (`language "..." /* ... */`) nested inside an `action def`, an
    /// `action` usage, or a `requirement def` body is inert documentation content with no
    /// resolvable semantic fact, mirroring the existing package-body and `ref` usage-body
    /// treatment (`PackageBodyElement::TextualRep`/`RefBodyElement::TextualRep`, both silently
    /// ignored alongside `Doc`). It must not be reported as an unsupported member.
    #[test]
    fn textual_representation_inside_action_def_body_is_ignored() {
        let sexpr = semantic_sexpr_for(
            r#"package P { action def A { language "alf" /* c.x = newX; */ } }"#,
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member for a TextualRep member, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(completeness complete)"),
            "expected TextualRep to be fully ignored (no parse-recovery/unsupported-syntax fallout), got: {sexpr}"
        );
    }

    /// Same as `textual_representation_inside_action_def_body_is_ignored`, but nested inside an
    /// `action` usage body rather than an `action def` body.
    #[test]
    fn textual_representation_inside_action_usage_body_is_ignored() {
        let sexpr =
            semantic_sexpr_for(r#"package P { action a { language "alf" /* c.x = newX; */ } }"#);
        assert!(
            !sexpr.contains("unsupported_action_usage_member"),
            "did not expect unsupported_action_usage_member for a TextualRep member, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(completeness complete)"),
            "expected TextualRep to be fully ignored (no parse-recovery/unsupported-syntax fallout), got: {sexpr}"
        );
    }

    /// Same as `textual_representation_inside_action_def_body_is_ignored`, but nested inside a
    /// `requirement def` body.
    #[test]
    fn textual_representation_inside_requirement_def_body_is_ignored() {
        let sexpr = semantic_sexpr_for(
            r#"package P { requirement def R { language "alf" /* c.x = newX; */ } }"#,
        );
        assert!(
            !sexpr.contains("unsupported_requirement_definition_member"),
            "did not expect unsupported_requirement_definition_member for a TextualRep member, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(completeness complete)"),
            "expected TextualRep to be fully ignored (no parse-recovery/unsupported-syntax fallout), got: {sexpr}"
        );
    }

    /// `terminate <name>;` nested inside a `then action <name> { ... }` self-named action usage
    /// (the representative fixture shape, e.g. `then action c1 { terminate c1; }`) must resolve
    /// its target through the shared `DeclarationDomain::Any` lexical lookup, sourced directly at
    /// the enclosing action usage's own declaration (not an anonymous nested one): the terminate
    /// statement's own enclosing scope is the action usage's *parent*'s children, where its own
    /// self-name is declared -- a genuine self-termination idiom.
    #[test]
    fn terminate_stmt_with_target_resolves() {
        let sexpr =
            semantic_sexpr_for("package P { action def A { then action c1 { terminate c1; } } }");
        assert!(
            sexpr.contains("(kind terminateTarget)"),
            "expected a terminateTarget reference, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the terminate target to resolve to its enclosing self-named action, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// A bare `terminate;` (no target) has nothing to resolve and must not be flagged as
    /// unsupported -- it is a legitimate no-op self-termination form, not a parser gap.
    #[test]
    fn bare_terminate_stmt_is_not_unsupported() {
        let sexpr = semantic_sexpr_for("package P { action def A { terminate; } }");
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// An `assign <target> := <value>;` reassignment statement must lower as an anonymous
    /// `assign` declaration whose `lhs` resolves as an `assignTarget` reference to its sibling
    /// action and whose `rhs` value expression resolves/evaluates, not fall through to
    /// `unsupported_action_definition_member`.
    #[test]
    fn assign_stmt_target_and_value_resolve() {
        let sexpr = semantic_sexpr_for("package P { action def A { action x; assign x := 5; } }");
        assert!(
            sexpr.contains("(kind assign)"),
            "expected an assign declaration, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind assignTarget)"),
            "expected an assignTarget relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the assign target to resolve to its sibling action, got: {sexpr}"
        );
    }

    /// An `assign` statement whose value expression references an unresolvable operand must
    /// still publish the target/value references, staying explicitly unresolved rather than
    /// silently dropped.
    #[test]
    fn assign_stmt_unresolvable_target_stays_unresolved() {
        let sexpr = semantic_sexpr_for("package P { action def A { assign missing := 5; } }");
        assert!(
            sexpr.contains("(kind assignTarget)"),
            "expected an assignTarget reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable assign target to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A `while <condition> { ... }` loop must lower as an anonymous `while` declaration whose
    /// condition resolves its operand and whose nested body recurses back into the same action-
    /// body-element dispatch (a nested `action x;` usage must be reachable), not fall through to
    /// `unsupported_action_definition_member`.
    #[test]
    fn while_stmt_condition_and_body_resolve() {
        let sexpr =
            semantic_sexpr_for("package P { action def A { action x; while x { action y; } } }");
        assert!(
            sexpr.contains("(kind while)"),
            "expected a while declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(path (named (kind package) (name \"P\")) (named (kind action-def) (name \"A\")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name \"y\")))"),
            "expected the nested `action y;` body member to be lowered, got: {sexpr}"
        );
    }

    /// A bare `loop { ... }` (no condition) must lower as an anonymous `loop` declaration whose
    /// body recurses, not fall through to `unsupported_action_definition_member`.
    #[test]
    fn loop_stmt_body_resolves() {
        let sexpr = semantic_sexpr_for("package P { action def A { loop { action y; } } }");
        assert!(
            sexpr.contains("(kind loop)"),
            "expected a loop declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// An `if <condition> { ... } else { ... }` control node must lower as an anonymous `if`
    /// declaration whose condition resolves and whose then/else bodies both recurse, not fall
    /// through to `unsupported_action_definition_member`.
    #[test]
    fn if_stmt_condition_and_both_branches_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action x; if x { action y; } else { action z; } } }",
        );
        assert!(
            sexpr.contains("(kind if)"),
            "expected an if declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            sexpr.contains(
                "(path (named (kind package) (name \"P\")) (named (kind action-def) (name \"A\")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name \"y\")))"
            ) && sexpr.contains(
                "(path (named (kind package) (name \"P\")) (named (kind action-def) (name \"A\")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name \"z\")))"
            ),
            "expected both the then and else branch body members to be lowered, got: {sexpr}"
        );
    }

    /// A `for <var> in <range> { ... }` loop must lower as an anonymous `forLoop` declaration
    /// whose range expression resolves, whose loop variable is declared as a named
    /// `forLoopVariable` sibling, and whose body recurses, not fall through to
    /// `unsupported_action_definition_member`.
    #[test]
    fn for_loop_range_variable_and_body_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action items; for i in items { action y; } } }",
        );
        assert!(
            sexpr.contains("(kind for-loop)"),
            "expected a for-loop declaration, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind for-loop-variable)"),
            "expected a for-loop-variable declaration for `i`, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the for-loop range `items` to resolve to its sibling action, got: {sexpr}"
        );
    }

    /// `UseCaseDefBodyElement::ActorUsage` (`actor <name> : <Type>;`) was unconditionally
    /// unsupported despite being a fully typed node (name, mandatory `type_name`, membership).
    /// Wires it into the shared case-family body walker (`lower_actor_usage`), mirroring
    /// `lower_requirement_actor_decl`'s shape.
    #[test]
    fn case_family_actor_usage_resolves() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Person; use case def U { actor driver : Person; } }",
        );
        assert!(
            sexpr.contains("(kind case-actor)"),
            "expected a case-actor declaration for `driver`, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_use_case_definition_member"),
            "did not expect unsupported_use_case_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected `driver`'s `Person` type to resolve, got: {sexpr}"
        );
    }

    /// `UseCaseDefBodyElement::Objective` (`objective { ... }`/`objective <name> : <Type> { ... }`)
    /// wraps a fully typed `RequirementUsage` (`Objective::requirement`) but was unconditionally
    /// unsupported. Wires it through the existing `lower_requirement_usage` pipeline, the same as
    /// every other requirement-usage site.
    #[test]
    fn case_family_objective_lowers_as_requirement_usage() {
        let sexpr =
            semantic_sexpr_for("package P { analysis def A { objective obj { doc /* g */ } } }");
        assert!(
            sexpr.contains("(kind requirement)"),
            "expected the objective's wrapped RequirementUsage to lower as a requirement, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_analysis_case_definition_member"),
            "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
        );
    }

    /// `UseCaseDefBodyElement::CaseReturnDecl` (`return [part|attribute]? [:>>]? <name>?
    /// [:|:>] <Type> [= expr];`) is a fully typed node (declared name, redefinition target, typed
    /// or subsetting type reference, bound value) but was unconditionally unsupported. Wires it
    /// (`lower_case_return_decl`), mirroring `lower_parameter_declaration`'s shape: a `:>>`
    /// redefinition target lowers as an authored `Redefinition` reference, and a `:`-typed name
    /// lowers as a `FeatureTyping` reference.
    #[test]
    fn case_return_decl_resolves_redefinition_and_type() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Engine; part def selectedAlternative; analysis def A { return part :>> selectedAlternative : Engine; } }",
        );
        assert!(
            sexpr.contains("(kind redefinition)"),
            "expected a redefinition relationship for the `:>>` target, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_analysis_case_definition_member"),
            "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected both the redefinition target and the `Engine` type to resolve, got: {sexpr}"
        );
    }

    /// A bare `return <name> = <expr>;` (no type, no `part`/`attribute` keyword, no `:>>`) is the
    /// anonymous-declared-name shape of `CaseReturnDecl`; its value expression should be lowered
    /// through the same `classify_calc_expression`/`lower_calc_expression` pipeline `lower_return_
    /// decl` (a calc's own `return`) uses.
    #[test]
    fn case_return_decl_value_expression_resolves() {
        let sexpr = semantic_sexpr_for(
            "package P { analysis def A { attribute source; return computed = source; } }",
        );
        assert!(
            sexpr.contains("(kind parameter)"),
            "expected an anonymous parameter declaration for the bare return, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_analysis_case_definition_member"),
            "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected `source` to resolve as the return value's operand, got: {sexpr}"
        );
    }

    /// `UseCaseDefBodyElement::Expression` (a bare result expression directly in an analysis/case
    /// body, e.g. `vehicle.mass`) mirrors `CalcDefBodyElement::Expression`'s identical shape: it is
    /// the enclosing declaration's own evaluated result, not a new nested declaration.
    #[test]
    fn case_family_bare_expression_resolves_as_own_result() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Vehicle { attribute mass; } analysis def A { in vehicle : Vehicle; vehicle.mass } }",
        );
        assert!(
            sexpr.contains("(kind memberAccessOperand)"),
            "expected a memberAccessOperand reference for `vehicle.mass`, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_analysis_case_definition_member"),
            "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
        );
    }

    /// `UseCaseDefBodyElement::Assign`/`ForLoop`/`ThenAction`/`FlowUsage` all already had working
    /// `lower_*` functions shared with `ActionDefBodyElement`/`ActionUsageBodyElement`, but were
    /// never dispatched from the case-family body walker. Wires all four through the same shared
    /// functions.
    #[test]
    fn case_family_shares_action_body_statement_wiring() {
        let sexpr = semantic_sexpr_for(
            "package P { analysis def A { attribute x; for i in 1 { assign x := i; } } }",
        );
        assert!(
            sexpr.contains("(kind for-loop)"),
            "expected a for-loop declaration inside the analysis def body, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind assign)"),
            "expected an assign declaration nested inside the for-loop body, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_analysis_case_definition_member"),
            "did not expect unsupported_analysis_case_definition_member, got: {sexpr}"
        );
    }

    /// `PerformBodyElement::Action` (an anonymous `perform action { ... }`'s own body, e.g. the
    /// OMG spec Annex A vehicle model's `perform action startVehicle { action turnVehicleOn send
    /// ... via ...; }`) was unconditionally unsupported despite wrapping the exact same
    /// `ActionUsageBodyElement` shape `lower_action_usage_body` already dispatches -- wires it
    /// through the shared `lower_action_usage_body_element` dispatcher.
    #[test]
    fn perform_action_body_element_dispatches_nested_action_usage() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Driver { port p1; } part part0 { perform action startVehicle { action turnVehicleOn send ignitionCmd via driver.p1 { in ignitionCmd:IgnitionCmd; } } } }",
        );
        assert!(
            sexpr.contains("(kind action)"),
            "expected a nested action-usage declaration inside the perform body, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind memberAccessOperand)"),
            "expected the send-usage's dotted `via driver.p1` clause to resolve as memberAccessOperand, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_usage_member"),
            "did not expect unsupported_action_usage_member, got: {sexpr}"
        );
    }

    /// `PerformBodyElement::InOut` (BNF `PerformInOutBinding`, the `in`/`out <target> = <value>;`
    /// parameter-argument-binding shorthand used when invoking a nested `perform action`, e.g.
    /// `perform action dynamics : StraightLineDynamics { in power = vehiclePower; }`) was
    /// unconditionally unsupported -- wires it via `lower_perform_inout_binding`.
    #[test]
    fn perform_inout_binding_resolves_target_and_value() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { in power; perform action dynamics : A { in power = vehiclePower; } } action def Outer { attribute vehiclePower; } }",
        );
        assert!(
            sexpr.contains("(kind perform-parameter-binding)"),
            "expected an anonymous perform-parameter-binding declaration, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind performParameterTarget)"),
            "expected the `in power` target to resolve as performParameterTarget, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_usage_member"),
            "did not expect unsupported_action_usage_member, got: {sexpr}"
        );
    }

    /// `PerformBodyElement::AttributeUsage` (an `in`/`out attribute` usage directly inside a
    /// `perform` body, BNF §6 G6) was unconditionally unsupported despite being a fully typed
    /// `AttributeUsage` node -- wires it through the already-existing `lower_attribute_usage`.
    #[test]
    fn perform_body_attribute_usage_lowers() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Vehicle { attribute mass; } part v : Vehicle; action def A { perform action doIt { in attribute mass :> v.mass; } } }",
        );
        assert!(
            sexpr.contains("(kind attribute)"),
            "expected an attribute declaration inside the perform body, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_usage_member"),
            "did not expect unsupported_action_usage_member, got: {sexpr}"
        );
    }

    /// `lower_succession_end` (used for `AssignTarget` among others) handled `Expression::
    /// MemberAccess` but not the sibling `Expression::FeatureChainRef` shape the parser actually
    /// produces for a dotted assign target (e.g. `assign a.b := 1;`), mirroring the fix already
    /// applied to `lower_satisfy_operand`.
    #[test]
    fn assign_target_dotted_feature_chain_resolves() {
        let sexpr = semantic_sexpr_for(
            "package P { part def A { part def B { attribute count; } part b : B; } action def Act { part a : A; assign a.b.count := 1; } }",
        );
        assert!(
            sexpr.contains("(kind memberAccessOperand)"),
            "expected the dotted assign target to resolve as memberAccessOperand, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// `Expression::Index` (`base#(index)`, e.g. `assign x := seq#(i);`) had no arm in
    /// `lower_constraint_expression`, so both the base and index sub-expressions fell through to
    /// unsupported. Recurses into both, mirroring `Tuple`/`CollectionOp`.
    #[test]
    fn assign_value_index_expression_resolves() {
        let sexpr = semantic_sexpr_for(
            "package P { action def Act { attribute seq; attribute i; assign x := seq#(i); } }",
        );
        assert!(
            sexpr.matches("(kind expressionOperand)").count() >= 2,
            "expected both the index base and index expression to resolve as expressionOperand references, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// `Expression::Null` (KerML `null`) had no arm in `lower_constraint_expression`, so `assign x
    /// := null;` fell through to unsupported even though it needs no reference resolution at all,
    /// mirroring the existing literal arms.
    #[test]
    fn assign_value_null_literal_is_supported() {
        let sexpr = semantic_sexpr_for("package P { action def Act { assign x := null; } }");
        assert!(
            sexpr.contains("(kind assign)"),
            "expected an assign declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// `variant perform doX;` (BNF `VariantTypedUsage::Perform`, inside a `variation perform
    /// action ... { ... }` body) was unconditionally unsupported both because `PerformBodyElement::
    /// Variant` was never dispatched and because `lower_variant_usage` treated every typed variant
    /// as out of scope; `Perform` now delegates to the already-existing `lower_perform`.
    #[test]
    fn variant_perform_lowers_as_perform_action_usage() {
        let sexpr = semantic_sexpr_for(
            "package P { action def Act { action doX; variation perform action doXorY { variant perform doX; } } }",
        );
        assert!(
            sexpr.contains("(kind perform-action)"),
            "expected a perform-action declaration for the variant, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_usage_member"),
            "did not expect unsupported_action_usage_member, got: {sexpr}"
        );
    }

    /// `flow of <payload> from <a> to <b>;` (the payload-first anonymous flow shorthand, BNF §6
    /// G12) was unconditionally unsupported purely because `payload.is_some()` was treated the
    /// same as a genuinely out-of-scope named/typed flow -- widens `lower_flow_usage` to resolve
    /// the payload's type as a new `FlowPayloadType` reference alongside `FlowSource`/
    /// `FlowTarget`.
    #[test]
    fn flow_usage_with_payload_only_resolves() {
        let sexpr = semantic_sexpr_for(
            "package P { item def Exposure; action def Focus { out xrsl: Exposure; } action def Shoot { in xsf: Exposure; } action takePicture { action focus: Focus; action shoot: Shoot; flow of Exposure from focus.xrsl to shoot.xsf; } }",
        );
        assert!(
            sexpr.contains("(kind flow)"),
            "expected a flow declaration, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind flowPayloadType)"),
            "expected the payload type to resolve as flowPayloadType, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_usage_member"),
            "did not expect unsupported_action_usage_member, got: {sexpr}"
        );
    }

    /// `Bind.body_elements` (a `bind a = b { ... }` statement's optional braced body) is typed the
    /// same `Vec<Node<PartUsageBodyElement>>` `PartUsageBody` uses, but every element was
    /// unconditionally flagged unsupported rather than dispatched through the shared
    /// `lower_part_usage_body_element` -- confirmed against the Systems Library's `bind start =
    /// done { doc /* ... */ }` shape (`Systems Library/Actions.sysml`): a `doc` comment nested in a
    /// bind body must be recognized and bound to the owning `bind` declaration, not reported as an
    /// unsupported member.
    #[test]
    fn bind_body_doc_comment_is_recorded() {
        let sexpr = semantic_sexpr_for(
            r#"package P { action def Act { first start; then done; bind start = done { doc /* note */ } } }"#,
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member for a doc comment in a bind body, got: {sexpr}"
        );
        assert!(
            sexpr.contains(r#"(documentation (doc (text " note ")))"#),
            "expected the bind body's doc comment recorded against the bind declaration, got: {sexpr}"
        );
    }

    /// Same as `bind_body_doc_comment_is_recorded`, but for real (non-`doc`) content: a nested
    /// `part` usage inside a `bind ... { ... }` body must lower as its own `part` declaration
    /// through the shared `lower_part_usage_body_element`.
    #[test]
    fn bind_body_nested_part_usage_lowers() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Widget; action def Act { first start; then done; bind start = done { part w : Widget; } } }",
        );
        assert!(
            sexpr.contains("(kind part)"),
            "expected a nested part declaration inside the bind body, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    // --- Canonical declaration facts -------------------------------------------------------
    //
    // These cover the authored presentation-adjacent facts (multiplicity, collection and
    // declaration modifiers, direction, short name, documentation, and the authored feature-value
    // spelling) recorded at each `lower_*` site. Every fact below has exactly one typed parser
    // field behind it; none is recovered by re-reading authored text.

    #[test]
    fn declared_multiplicity_bounds_are_recorded_as_literals() {
        let sexpr = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel[0..4]; }");
        assert!(
            sexpr.contains("(multiplicity (lower 0) (upper 4))"),
            "expected literal multiplicity bounds, got: {sexpr}"
        );
    }

    #[test]
    fn a_bare_bound_sets_both_multiplicity_bounds() {
        let sexpr = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel[3]; }");
        assert!(
            sexpr.contains("(multiplicity (lower 3) (upper 3))"),
            "expected `[3]` to set both bounds to 3, got: {sexpr}"
        );
    }

    /// `[*]` writes neither bound and `[1..*]` writes only the lower one, so both render their
    /// missing side as `unbounded` -- but a declaration with no `[...]` at all publishes no
    /// multiplicity fact whatsoever, which is a different answer from `[*]`.
    #[test]
    fn unwritten_and_absent_multiplicity_bounds_stay_distinct() {
        let unbounded = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel[*]; }");
        assert!(
            unbounded.contains("(multiplicity (lower unbounded) (upper unbounded))"),
            "expected `[*]` to publish an unbounded multiplicity fact, got: {unbounded}"
        );

        let lower_only =
            semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel[1..*]; }");
        assert!(
            lower_only.contains("(multiplicity (lower 1) (upper unbounded))"),
            "expected `[1..*]` to keep its authored lower bound, got: {lower_only}"
        );

        let absent = semantic_sexpr_for("package P { part def Wheel; part wheels : Wheel; }");
        assert!(
            !absent.contains("(multiplicity"),
            "expected no multiplicity fact when none is authored, got: {absent}"
        );
    }

    /// A bound the parser records as a non-literal `Expression` is published as an explicit
    /// non-literal fact rather than folded, dropped, or re-read from source text.
    #[test]
    fn a_non_literal_multiplicity_bound_is_published_as_an_expression() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Wheel; attribute n : Integer; part wheels : Wheel[1..n]; }",
        );
        assert!(
            sexpr.contains("(multiplicity (lower 1) (upper expression))"),
            "expected the non-literal upper bound published as `expression`, got: {sexpr}"
        );
    }

    #[test]
    fn collection_modifiers_are_recorded() {
        let sexpr =
            semantic_sexpr_for("package P { attribute seq : Integer[1..*] ordered nonunique; }");
        assert!(
            sexpr.contains("(modifiers ordered nonunique)"),
            "expected both collection modifiers, got: {sexpr}"
        );
    }

    #[test]
    fn definition_prefix_modifiers_are_recorded() {
        let abstract_def = semantic_sexpr_for("package P { abstract part def Vehicle; }");
        assert!(
            abstract_def.contains("(modifiers abstract)"),
            "expected the abstract prefix recorded, got: {abstract_def}"
        );

        let variation_def = semantic_sexpr_for("package P { variation part def Engine; }");
        assert!(
            variation_def.contains("(modifiers variation)"),
            "expected the variation prefix recorded, got: {variation_def}"
        );
    }

    #[test]
    fn parameter_direction_is_recorded_as_a_declaration_fact() {
        let sexpr =
            semantic_sexpr_for("package P { calc def C { in x : Integer; return : Integer; } }");
        assert!(
            sexpr.contains("(facts (direction in))"),
            "expected the `in` direction recorded on the parameter declaration, got: {sexpr}"
        );
    }

    #[test]
    fn authored_short_names_are_recorded() {
        let sexpr = semantic_sexpr_for("package <pkg> P { part def <w> Wheel; }");
        assert!(
            sexpr.contains(r#"(short-name "pkg")"#),
            "expected the package short name recorded, got: {sexpr}"
        );
        assert!(
            sexpr.contains(r#"(short-name "w")"#),
            "expected the part def short name recorded, got: {sexpr}"
        );
    }

    /// A `doc` body element annotates the declaration owning that body, and the recorded text is
    /// the raw content between the comment delimiters -- the parser performs no leading-`*`
    /// stripping or dedent, so neither does this fact.
    #[test]
    fn doc_comments_bind_to_the_declaration_owning_their_body() {
        let sexpr = semantic_sexpr_for("package P { part def Wheel { doc /* a wheel */ } }");
        assert!(
            sexpr.contains(r#"(documentation (doc (text " a wheel ")))"#),
            "expected the doc comment bound to the part def, got: {sexpr}"
        );
    }

    #[test]
    fn comment_and_rep_annotations_are_recorded_as_distinct_forms() {
        let comment = semantic_sexpr_for(r#"package P { calc def C { comment /* note */ } }"#);
        assert!(
            comment.contains(r#"(comment (text " note "))"#),
            "expected the comment annotation recorded, got: {comment}"
        );

        // The corpus-proven spelling is the bare `language "..." /* ... */` form inside an action
        // def body; the `rep <name> language ...` spelling is not reachable in every scope.
        let rep = semantic_sexpr_for(r#"package P { action def A { language "Alf" /* body */ } }"#);
        assert!(
            rep.contains(r#"(rep (language "Alf") (text " body "))"#),
            "expected the textual representation recorded with its language, got: {rep}"
        );
    }

    /// All five authored value spellings stay distinguishable: `=`, `:=`, `default =`,
    /// `default :=`, and the operator-less bare `default`.
    #[test]
    fn authored_feature_value_spellings_stay_distinct() {
        let bind = semantic_sexpr_for("package P { attribute mass : Integer = 10; }");
        assert!(
            bind.contains("(feature-value (kind bind))"),
            "expected a plain `=` bind, got: {bind}"
        );

        let assign = semantic_sexpr_for("package P { attribute mass : Integer := 10; }");
        assert!(
            assign.contains("(feature-value (kind assign))"),
            "expected a `:=` assign, got: {assign}"
        );

        let default_bind =
            semantic_sexpr_for("package P { attribute mass : Integer default = 10; }");
        assert!(
            default_bind.contains("(feature-value (kind bind) (default true))"),
            "expected a `default =` bind, got: {default_bind}"
        );

        let bare_default = semantic_sexpr_for("package P { attribute mass : Integer default 10; }");
        assert!(
            bare_default.contains("(feature-value (kind bind) (default true) (operator false))"),
            "expected the operator-less bare `default` spelling, got: {bare_default}"
        );
    }

    /// A declaration whose parser node carries none of these facts -- and every synthesized
    /// anonymous scope the lowering mints, which has no authored declaration syntax at all --
    /// publishes no fact block, so absence stays absence rather than a defaulted answer.
    #[test]
    fn a_declaration_with_no_authored_facts_publishes_no_fact_block() {
        let sexpr = semantic_sexpr_for("package P { part def Wheel; }");
        assert!(
            !sexpr.contains("(facts "),
            "expected no fact block for a plain package and part def, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(documentation "),
            "expected no documentation block, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(feature-value "),
            "expected no feature-value block, got: {sexpr}"
        );
    }

    // --- Canonical element identity ---------------------------------------------------------

    fn publication_for(sources: &[(&str, &str)]) -> PublishedResolution {
        let request = BuildRequest::new(
            sources
                .iter()
                .map(|(identity, source)| {
                    SourceInput::new(*identity, source.to_string(), SourceKind::Workspace)
                })
                .collect(),
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        build(request).unwrap()
    }

    fn target_symbol(
        published: &PublishedResolution,
        document: &str,
        line: u32,
        character: u32,
    ) -> SymbolIdentity {
        match published.target_at(document, TextPosition { line, character }) {
            QueryOutcome::Resolved(target) => target.symbol,
            other => panic!("expected a resolved navigation target, got: {other:?}"),
        }
    }

    /// Anonymous ordinals are allocated per `(document, owner, kind)`, so an identity that named
    /// only the kind and ordinal could not tell two same-kind anonymous declarations under
    /// different owners apart. The identity spells out the owner chain for exactly this reason.
    #[test]
    fn anonymous_declarations_under_different_owners_get_distinct_identities() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action x; if x { action y; } else { action z; } } action def B { action x; if x { action y; } else { action z; } } }",
        );
        assert!(
            sexpr.contains(r#"(path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind if) (ordinal 0)))"#),
            "expected the if-scope under A to carry its owner in its identity, got: {sexpr}"
        );
        assert!(
            sexpr.contains(r#"(path (named (kind package) (name "P")) (named (kind action-def) (name "B")) (anonymous (kind if) (ordinal 0)))"#),
            "expected the if-scope under B to carry its owner in its identity, got: {sexpr}"
        );
    }

    /// A named declaration whose owner chain passes through an anonymous scope cannot be
    /// identified by a qualified name alone -- the anonymous owner contributes no name segment --
    /// so it renders the explicit path form instead.
    #[test]
    fn a_named_declaration_under_an_anonymous_owner_renders_an_explicit_path() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action x; if x { action y; } else { action z; } } }",
        );
        assert!(
            sexpr.contains(
                r#"(path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "y")))"#
            ),
            "expected the branch member to render an explicit path, got: {sexpr}"
        );
        assert!(
            !sexpr.contains(r#"(qualified-name "P::A::::y")"#),
            "expected no ambiguous empty-segment qualified name, got: {sexpr}"
        );
    }

    /// The identity is structural, so editing an unrelated document cannot change it. A dense
    /// storage ordinal would shift as soon as any earlier document gained a declaration.
    #[test]
    fn element_identity_survives_an_edit_to_an_unrelated_document() {
        let before = publication_for(&[
            ("memory://a.sysml", "package A { part def Wheel; }"),
            ("memory://b.sysml", "package B { part def Engine; }"),
        ]);
        let after = publication_for(&[
            (
                "memory://a.sysml",
                "package A { part def Wheel; part def Axle; part def Frame; }",
            ),
            ("memory://b.sysml", "package B { part def Engine; }"),
        ]);

        let engine_before = target_symbol(&before, "memory://b.sysml", 0, 21);
        let engine_after = target_symbol(&after, "memory://b.sysml", 0, 21);
        assert_eq!(
            engine_before, engine_after,
            "expected an unrelated document's edit to leave this element's identity unchanged"
        );
    }

    /// Two identically named siblings of the same kind are distinguished by an occurrence ordinal,
    /// so each remains addressable. The Pilot does the same: its `qualifiedName` derivation yields
    /// null for every same-named member after the first, and `path()` then falls through to a
    /// positional form.
    ///
    /// The first occurrence keeps the plain name, so authoring a duplicate later never disturbs
    /// the identity already published for the declaration that was there first.
    #[test]
    fn duplicate_sibling_names_stay_separately_addressable() {
        let published = publication_for(&[(
            "memory://dup.sysml",
            "package P { part def Failure; part def Failure; }",
        )]);

        let first = target_symbol(&published, "memory://dup.sysml", 0, 21);
        let second = target_symbol(&published, "memory://dup.sysml", 0, 39);
        assert_ne!(
            first, second,
            "expected identically named siblings to carry distinct identities"
        );

        for symbol in [&first, &second] {
            match published.references(symbol, true) {
                QueryOutcome::Resolved(locations) => assert_eq!(
                    locations.len(),
                    1,
                    "expected each sibling to resolve to its own declaration site"
                ),
                other => panic!("expected a resolved references outcome, got: {other:?}"),
            }
        }
    }

    /// A name shared by siblings of *different* kinds needs no occurrence ordinal -- the kind on
    /// every path segment already separates them. This is the sibling `sysml-compiler`'s tag byte:
    /// `metadata def X` and the `metadata X about ...` annotating it are distinct elements.
    #[test]
    fn same_name_different_kind_siblings_are_separated_by_kind() {
        let sexpr = semantic_sexpr_for(
            "package P { part def Vehicle; metadata def Safety; metadata Safety about Vehicle; }",
        );
        assert!(
            sexpr.contains(r#"(named (kind metadata-def) (name "Safety"))"#),
            "expected the metadata definition's kind in its identity, got: {sexpr}"
        );
        assert!(
            sexpr.contains(r#"(named (kind metadata) (name "Safety"))"#),
            "expected the metadata usage's kind in its identity, got: {sexpr}"
        );
    }
}
