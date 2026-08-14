//! Opaque parser-owned semantic construction and batch resolution.
//!
//! Syntax documents, dense IDs, semantic storage, solver state, and indexes remain private. The
//! public contract accepts immutable source inputs and streams owner-defined canonical output.

use std::fmt;

use source_identity::{ContentDigest, RootDigest, SourceManifest, SourceManifestEntry, SourceRole};

mod model;

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
    /// UPSTREAM_PARSER_GAPS.md #11) must lower as its own `item` declaration via the
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
    /// `then send ...;` continuation form blocked by UPSTREAM_PARSER_GAPS.md Gap 30) must resolve
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

    /// The `then send new S() to b;` continuation shorthand is a genuine parser gap (see
    /// UPSTREAM_PARSER_GAPS.md Gap 30, `ThenTarget` has no `Send` variant): the parser itself
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
            sexpr.contains("(qualified-name \"P::A::::y\")"),
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
            sexpr.contains("(qualified-name \"P::A::::y\")")
                && sexpr.contains("(qualified-name \"P::A::::z\")"),
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
}
