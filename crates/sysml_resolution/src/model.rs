//! Direct parser-to-semantic canonicalization storage.
//!
//! Private parser-owned semantic construction.
//!
//! This module deliberately exposes no storage, graph adapter, or independently publishable
//! authored model. The publication owner consumes the typed coordinator outcome below.

use std::{
    collections::{hash_map::RandomState, BTreeMap},
    hash::BuildHasher,
    sync::Arc,
    time::{Duration, Instant},
};

use hashbrown::HashTable;

use crate::evaluation::EvaluationPolicy;
use source_identity::SourceRole;
use sysml_v2_parser_next::{
    ast::{
        ActionBranchBody, ActionDef, ActionDefBody, ActionDefBodyElement,
        ActionUsage as ParserActionUsage, ActionUsageBody, ActionUsageBodyElement, ActorUsage,
        AliasBody, AliasDef, Allocate, AllocationDef, AnalysisCaseDef,
        AnalysisCaseUsage as ParserAnalysisCaseUsage, AnnotatingMember, AssertConstraintMember,
        AssignStmt, AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage,
        BinaryOperator, Bind, BindingConnectorUsage, CalcDef, CalcDefBody, CalcDefBodyElement,
        CalcUsage as ParserCalcUsage, CaseDef, CaseReturnDecl, CaseUsage as ParserCaseUsage,
        ClassDef, CommentAnnotation, ConcernUsage as ParserConcernUsage, ConnectStmt,
        ConnectionDef, ConnectionDefBody, ConnectionDefBodyElement, ConnectionEnd,
        ConnectionUsageMember as ParserConnectionUsage, ConstraintDef, ConstraintDefBody,
        ConstraintDefBodyElement, ConstraintUsage as ParserConstraintUsage, DefaultReferenceUsage,
        DefinitionBody, DefinitionBodyElement, DefinitionPrefix, Dependency, DoAction, DocComment,
        EndDecl, EndIdentity, EntryAction, EnumDef, EnumerationBody,
        EnumerationUsage as ParserEnumerationUsage, ExitAction, Expression, ExtendedDefinition,
        FeatureValue, FeatureValueKind as ParserFeatureValueKind, FinalState, FirstMergeBody,
        FirstMergeBodyElement, FirstStmt, FlowDef, FlowUsage, ForLoop, FrameMember, IfStmt, Import,
        ImportShape, InOut, InOutDecl, IncludeUseCase, InterfaceDef, InterfaceDefBody,
        InterfaceDefBodyElement, InterfaceUsage as ParserInterfaceUsage, InterfaceUsageBodyElement,
        ItemDef, ItemUsage as ParserItemUsage, KermlBindingMember, KermlClassifierDecl,
        KermlClassifierKeyword, KermlConnectorEnd, KermlConnectorMember, KermlEndMember,
        KermlFeatureKind, KermlFeatureMember, KermlInvariantMember, KermlSuccessionMember,
        LibraryPackage, Membership, MembershipKind as ParserMembershipKind, MetadataAnnotation,
        MetadataDef, MetadataUsage as ParserMetadataUsage, Multiplicity, NamespaceDecl, Node,
        OccurrenceBodyElement, OccurrenceDef, OccurrencePortionKind as ParserOccurrencePortionKind,
        OccurrenceUsage as ParserOccurrenceUsage, OccurrenceUsageBody, Package, PackageBody,
        PackageBodyElement, PartDef, PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody,
        PartUsageBodyElement, Perform as ParserPerform, PerformBody, PerformBodyElement,
        PerformInOutBinding, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement,
        PortUsage as ParserPortUsage, PurposeMember, QualifiedIdentification, QualifiedReferenceId,
        RefDecl, RelationshipBodyElement, RenderingDef, RenderingDefBody, RenderingDefBodyElement,
        RenderingUsage as ParserRenderingUsage, RenderingUsageBody, RenderingUsageBodyElement,
        RequireConstraint, RequirementActorDecl, RequirementDef, RequirementDefBody,
        RequirementDefBodyElement, RequirementUsage as ParserRequirementUsage, ReturnDecl,
        RootElement, Satisfy, SatisfyViewMember, SendPayload, Span, StakeholderMember, StateDef,
        StateDefBody, StateDefBodyElement, StateUsage as ParserStateUsage, SubjectDecl,
        SubsettingKind, SubsettingRelationship, TerminateStmt, TextualRepresentation, ThenAction,
        ThenStmt, ThenTarget, Transition, TransitionAccept, TransitionEffect, TypedParameterMember,
        UnaryOperator, UseCaseDef, UseCaseDefBody, UseCaseDefBodyElement,
        UseCaseUsage as ParserUseCaseUsage, VariantTypedUsage, VariantUsage, VerificationCaseDef,
        VerificationCaseUsage as ParserVerificationCaseUsage, VerifyRequirementMember, ViewBody,
        ViewBodyElement, ViewDef, ViewDefBody, ViewDefBodyElement, ViewUsage as ParserViewUsage,
        ViewpointDef, ViewpointUsage as ParserViewpointUsage, Visibility as ParserVisibility,
    },
    ParseError, ParsedDocument,
};

macro_rules! semantic_id {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        struct $name(u32);

        impl $name {
            fn from_index(index: usize) -> Result<Self, ConstructionError> {
                Ok(Self(
                    u32::try_from(index).map_err(|_| ConstructionError::Capacity)?,
                ))
            }

            fn index(self) -> usize {
                self.0 as usize
            }
        }
    };
}

semantic_id!(DocumentId);
semantic_id!(DeclarationId);
semantic_id!(SymbolId);
semantic_id!(SymbolPathId);
semantic_id!(AuthoredReferenceId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConstructionError {
    Capacity,
    InvalidIdentity,
    DuplicateDocumentIdentity,
    InvalidParserReference,
    InvalidMembership,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DeclarationKind {
    Namespace,
    Package,
    LibraryPackage,
    PartDefinition,
    PartUsage,
    AttributeDefinition,
    AttributeUsage,
    /// `enum def` (BNF EnumerationDefinition): a type whose owned members are enumeration
    /// literals. Mirrors PartDefinition/AttributeDefinition lowering.
    EnumerationDefinition,
    /// A package/definition/usage-level `enum` feature member (BNF EnumerationUsage), e.g.
    /// `enum color : ColorKind;`. Distinct from `EnumerationLiteral`, which is a value owned
    /// directly by an `enum def` body.
    EnumerationUsage,
    /// One `enum <name>;` (or bare `<name>;`) value owned by an `enum def` body (BNF
    /// EnumeratedValue). Each literal gets its own declaration/qualified name, analogous to how
    /// attribute/part usages become owned members.
    EnumerationLiteral,
    /// `requirement def` (BNF RequirementDefinition): a type whose owned members are
    /// attribute/requirement usages, mirroring PartDefinition lowering. Requirement-specific
    /// semantics (subject binding, assumption/constraint facts) are out of scope here; only
    /// ownership, specialization, and owned-member structure are lowered.
    RequirementDefinition,
    /// A package/definition/usage-level `requirement` feature member (BNF RequirementUsage), e.g.
    /// `requirement r : SomeReq;`. Mirrors PartUsage lowering.
    RequirementUsage,
    /// `port def` (BNF PortDefinition): a type whose owned members are attribute/enum/nested-port
    /// usages, mirroring PartDefinition lowering. Port-specific semantics (interface/flow
    /// binding, conformance, connector-end validation) are out of scope here; only ownership,
    /// specialization, and owned-member structure are lowered.
    PortDefinition,
    /// A package/definition/usage-level `port` feature member (BNF PortUsage), e.g.
    /// `port source : ~InputPort;`. Mirrors PartUsage lowering. Its `:`/`:>` typing target may be
    /// conjugated (a leading `~`, e.g. `~InputPort`); the conjugation polarity is carried as an
    /// explicit `RelationshipFlags::conjugated` fact on the FeatureTyping/Subclassification
    /// reference rather than folded into the reference target itself.
    PortUsage,
    /// `item def` (BNF ItemDefinition): a type whose owned members are attribute/enum/nested-item
    /// usages, mirroring PartDefinition lowering. Item-specific semantics beyond ownership,
    /// specialization, and owned-member structure are out of scope here.
    ItemDefinition,
    /// A package/definition/usage-level `item` feature member (BNF ItemUsage), e.g.
    /// `item i : SomeItem;`. Mirrors PartUsage lowering.
    ItemUsage,
    /// `action def` (BNF ActionDefinition): a type whose owned members are attribute/item/nested
    /// action usages, mirroring PartDefinition lowering. Behavioral/control-flow semantics
    /// (parameters, succession, decision/merge/fork/join, accept/send, perform) are out of scope
    /// here; only ownership, specialization, and owned-declaration structure are lowered.
    ActionDefinition,
    /// A package/definition/usage-level `action` feature member (BNF ActionUsage), e.g.
    /// `action validateRoute;` or `action a : SomeAction;`. Mirrors PartUsage lowering. Like
    /// `PartUsage`, `ActionUsage`'s typing is a structured `TypingRelationship` (not a bare
    /// `QualifiedReferenceId`).
    ActionUsage,
    /// An anonymous succession feature synthesized for a `first X then Y;` control-flow
    /// statement (BNF `FirstStmt`) found in an action def/usage body. Owned by the enclosing
    /// action def/usage declaration (mirroring `EndDecl`'s nested `ConnectionUsage` children),
    /// so its `first`/`then` `Succession` end references resolve against the owning action's own
    /// scope -- where the sibling actions the statement connects are actually declared -- rather
    /// than the action's enclosing scope (the shape `ConnectorEnd`'s inline `connect from ... to
    /// ...;` uses, since a connector's endpoints are ordinarily declared alongside the connector
    /// itself, not nested inside it).
    Succession,
    /// `state def` (BNF StateDefinition): a type whose owned members are attribute/item/action/
    /// nested state usages, mirroring ActionDefinition lowering. State-machine-specific semantics
    /// (entry/do/exit action bindings, transitions, exclusive/parallel substates, history) are out
    /// of scope here; only ownership, specialization, and owned-declaration structure are lowered.
    StateDefinition,
    /// A package/definition/usage-level `state` feature member (BNF StateUsage), e.g.
    /// `state s;` or `state s : SomeState;`. Mirrors ActionUsage lowering. `StateUsage`'s typing
    /// is a structured `TypingRelationship` (not a bare `QualifiedReferenceId`).
    StateUsage,
    /// `metadata def` (BNF MetadataDefinition): a type whose owned members are attribute/nested
    /// usages, mirroring ItemDefinition lowering: ownership, membership, an optional `:>`
    /// specialization relationship, and owned-member structure through the shared
    /// `lower_attribute_body`. Metadata-specific semantics (annotation application, `about`
    /// targets) are out of scope here.
    MetadataDefinition,
    /// A package/definition/usage-level `metadata` feature member (BNF MetadataUsage), e.g.
    /// `metadata m : SomeMetadata;`. Mirrors ItemUsage lowering: its `:` typing target is a bare
    /// `QualifiedReferenceId`. The `about` clause (targets this usage annotates) is out of scope
    /// here -- a distinct annotation-application fact family, not the declaration/typing shape
    /// covered by this slice.
    MetadataUsage,
    /// `connection def` (BNF ConnectionDefinition): a type whose owned members are attribute/
    /// item/port usages, nested `end`/`connect` connector structure, mirroring PortDefinition
    /// lowering. Connector-end referential/multiplicity validation is out of scope here; only
    /// ownership, specialization, owned-member structure, and connector-end reference facts are
    /// lowered.
    ConnectionDefinition,
    /// A package/definition/usage-level `connection` feature member (BNF ConnectionUsage), e.g.
    /// `connection : ConnDef connect a::portA to b::portB;`. Mirrors PortUsage lowering; its `:`
    /// typing target is a bare `QualifiedReferenceId` (like MetadataUsage), not a structured
    /// `TypingRelationship`.
    ConnectionUsage,
    /// `occurrence def` (BNF OccurrenceDefinition): a type whose owned members are attribute/
    /// item/part/nested-occurrence usages, mirroring PartDefinition lowering. Occurrence is the
    /// base kind that `part`/`item`/`state`/`action`/`connection`/`event` etc. specialize from in
    /// the standard library. Occurrence-specific semantics (individual/portion-of-life,
    /// time-slicing, snapshot facts, `exhibit`/`succession`/`satisfy`/`allocate`/connector-end
    /// body constructs) are out of scope here; only ownership, specialization, and owned-member
    /// structure are lowered.
    OccurrenceDefinition,
    /// A package/definition/usage-level `occurrence` feature member (BNF OccurrenceUsage), e.g.
    /// `occurrence o;` or `occurrence o : SomeOccurrence;`. Mirrors PortUsage lowering; its `:`
    /// typing target is a bare `QualifiedReferenceId` (like MetadataUsage/ItemUsage), not a
    /// structured `TypingRelationship`, but does carry an independent conjugation flag
    /// (`type_is_conjugated`) analogous to PortUsage's conjugated typing target.
    OccurrenceUsage,
    /// `analysis def` (BNF AnalysisCaseDefinition): a type whose owned members are attribute
    /// usages and nested behavior/case structure, mirroring RequirementDefinition lowering.
    /// Analysis-case-specific semantics (subject binding, objective, result parameter binding to
    /// a calc/action) are out of scope here; only ownership, specialization, and owned-member
    /// structure are lowered. `analysis` usage lowering follows below, in
    /// `DeclarationKind::AnalysisCaseUsage` (planning/UPSTREAM_PARSER_GAPS.md #5 was resolved upstream in
    /// `0757de13`: `AnalysisCaseUsage` now carries `subsets`/`redefines` fields with full parity
    /// to `RequirementUsage`).
    AnalysisCaseDefinition,
    /// A package/definition/usage-level `analysis` feature member (BNF AnalysisCaseUsage), e.g.
    /// `analysis fuelEconomyAnalysis_1 : FuelEconomyAnalysis_1 { ... }`. Mirrors
    /// `RequirementUsage` lowering: ownership, membership, a `:` typing target (bare
    /// `QualifiedReferenceId`), and `subsets`/`redefines` subsetting relationships resolving
    /// through the same ancestor-closure fixed point. Analysis-case-specific semantics (subject
    /// binding, objective, result parameter binding) are out of scope here, sharing
    /// `UnsupportedFamily::AnalysisCaseDefinitionMember` with the `def` form's body walker.
    AnalysisCaseUsage,
    /// `interface def` (BNF InterfaceDefinition): a type whose owned members are attribute/item/
    /// port/flow usages, nested `end`/`connect` connector structure, mirroring ConnectionDefinition
    /// lowering (`InterfaceDefBody`/`InterfaceDefBodyElement` share the same `end`/`connect`
    /// connector-end shape as `ConnectionDefBody`/`ConnectionDefBodyElement`, so `end` declarations
    /// and `connect` statements reuse the same `ReferenceKind::ConnectorEnd` machinery). Verified
    /// `InterfaceDef`'s `specializes: Option<Node<TypingRelationship>>` field carries full parity
    /// with `ConnectionDef`/`ActionDef`/`OccurrenceDef`. `interface` usage lowering
    /// (`DeclarationKind::InterfaceUsage`) is deferred: `ast::InterfaceUsage`'s three variants
    /// (`TypedConnect`/`Connection`/`Declaration`) carry only a bare `interface_type:
    /// Option<QualifiedReferenceId>` with no `subsets`/`redefines` fields at all, unlike the
    /// structurally analogous `ConnectionUsageMember` (see planning/UPSTREAM_PARSER_GAPS.md #6). Interface-
    /// specific semantics beyond declaration/typing/ends (flow/protocol constraints) are out of
    /// scope here.
    InterfaceDefinition,
    /// `view def` (BNF ViewDefinition, Clause 8.2.2.26): a type whose owned members participate
    /// in the same Subclassification/FeatureTyping `DeclarationDomain::Type` lexical/ancestor
    /// fixed point as `OccurrenceDefinition`/`ConnectionDefinition`. Verified `ViewDef`'s
    /// `specializes: Option<Node<TypingRelationship>>` field carries full parity with
    /// `ConnectionDef`/`ActionDef`/`OccurrenceDef`. View-specific semantics -- `render`
    /// (`ViewRenderingUsage`), viewpoint `satisfy` binding, and `expose`/`filter` view
    /// composition -- are out of scope for this slice and fall through to
    /// `UnsupportedFamily::ViewDefinitionMember`. `view` usage lowering
    /// (`DeclarationKind::ViewUsage`) is deferred: `ast::ViewUsage` has no `subsets:
    /// Option<Node<SubsettingRelationship>>` field at all (only `type_name`/`redefines`), unlike
    /// the structurally analogous `OccurrenceUsage`/`StateUsage`/`PortUsage`, so a bare `view v
    /// :> Base { ... }` subsetting clause parses successfully but is silently dropped before it
    /// reaches the typed AST (see planning/UPSTREAM_PARSER_GAPS.md #8; confirmed real usage in
    /// `test/snapshots/sysml/validation/11b_safety_and_security_feature_views.md`'s
    /// `view vehicleMandatorySafetyFeatureView :> vehicleSafetyFeatureView { ... }`).
    ViewDefinition,
    /// `case def` (BNF CaseDefinition): a type whose owned members are attribute usages and
    /// nested case structure, mirroring `AnalysisCaseDefinition` lowering (shares the same
    /// `UseCaseDefBody`/`UseCaseDefBodyElement` shape). Case-specific semantics (subject binding,
    /// objective, first-succession/return structure) are out of scope here; only ownership,
    /// specialization, and owned-member structure are lowered. `case` usage lowering follows
    /// below, in `DeclarationKind::CaseUsage` (planning/UPSTREAM_PARSER_GAPS.md #5 was resolved upstream
    /// in `0757de13`: `CaseUsage` now carries `subsets`/`redefines` fields with full parity to
    /// `RequirementUsage`).
    CaseDefinition,
    /// A package/definition/usage-level `case` feature member (BNF CaseUsage), mirroring
    /// `AnalysisCaseUsage` lowering (shares the same field shape: `type_name`/`subsets`/
    /// `redefines`/body). Case-specific semantics (subject binding, objective, first-succession/
    /// return structure) are out of scope here, sharing `UnsupportedFamily::CaseDefinitionMember`
    /// with the `def` form's body walker.
    CaseUsage,
    /// `verification def` (BNF VerificationCaseDefinition): a type whose owned members are
    /// attribute usages and nested case structure, mirroring `CaseDefinition`/
    /// `AnalysisCaseDefinition` lowering. Verification-specific semantics are out of scope here;
    /// only ownership, specialization, and owned-member structure are lowered. `verification`
    /// usage lowering (`DeclarationKind::VerificationCaseUsage`): unlike `CaseUsage`/
    /// `AnalysisCaseUsage`, `VerificationCaseUsage` still has no `subsets`/`redefines` field at
    /// all (not just a stale "dropped" bug) -- only `name`/`type_name`/`is_abstract`/body are
    /// lowered; a header-level `:>`/`:>>` clause still fails to parse into this node at all
    /// (falls to raw-text recovery instead, per planning/UPSTREAM_PARSER_GAPS.md's `AllocationUsage`/
    /// `FlowUsage`/`ViewpointUsage` gap class).
    VerificationCaseDefinition,
    /// `use case def` (BNF UseCaseDefinition): a type whose owned members are attribute usages
    /// and nested case structure, mirroring `CaseDefinition`/`AnalysisCaseDefinition` lowering.
    /// Use-case-specific semantics (actor/include structure) are out of scope here; only
    /// ownership, specialization, and owned-member structure are lowered. `use case` usage
    /// lowering (`DeclarationKind::UseCaseUsage`): like `VerificationCaseUsage`, `UseCaseUsage`
    /// still has no `subsets`/`redefines`/multiplicity field at all -- only `name`/`type_name`/
    /// `is_abstract`/body are lowered.
    UseCaseDefinition,
    /// `viewpoint def` (BNF ViewpointDefinition, Clause 8.2.2.27): a type whose owned members
    /// share `RequirementDefBody` with `RequirementDefinition`, mirroring `lower_requirement_def`
    /// lowering: ownership, membership, an optional `:>` specialization relationship
    /// participating in the shared `DeclarationDomain::Type` fixed point, and owned
    /// attribute/nested-requirement members via the same shared body walker `requirement def`
    /// uses. Verified `ViewpointDef`'s `specializes: Option<Node<TypingRelationship>>` field
    /// carries full parity with `RequirementDef`/`ConnectionDef`. Stakeholder/concern-binding
    /// semantics (the viewpoint-specific surface, e.g. `stakeholder`/`concern` clauses) are out
    /// of scope for this slice and fall through to `RequirementDefinitionMember` diagnostics
    /// alongside the other unmodeled `RequirementDefBody` members. `viewpoint` usage lowering
    /// (`DeclarationKind::ViewpointUsage`): only `name`/`type_name`/body are lowered. A
    /// header-level `:>`/`:>>` clause now parses into `ast::ViewpointUsage::subsets`/`redefines`,
    /// but nothing reads it yet (planning/UPSTREAM_PARSER_GAPS.md, "Typed upstream, not yet
    /// lowered here").
    ViewpointDefinition,
    /// `rendering def` (BNF RenderingDefinition, Clause 8.2.2.26): a type whose owned members
    /// share `RenderingDefBody`/`RenderingDefBodyElement` with `ViewDefBody`/`ViewDefBodyElement`
    /// (same shape: `Filter`/`ViewRendering`/`Other`/`Doc`/`Error`), mirroring `lower_view_def`
    /// lowering: ownership, membership, an optional `:>` specialization relationship
    /// participating in the shared `DeclarationDomain::Type` fixed point. Verified
    /// `RenderingDef`'s `specializes: Option<Node<TypingRelationship>>` field carries full parity
    /// with `ViewDef`/`ConnectionDef`. Render-specific body semantics (`filter`/`render` members)
    /// are out of scope for this slice and fall through to a dedicated
    /// `RenderingDefinitionMember` diagnostic. `rendering` usage lowering
    /// (`DeclarationKind::RenderingUsage`): `ast::RenderingUsage` now carries full
    /// `subsets`/`redefines`/`ordered`/`nonunique`/`value` field parity with `ViewUsage`
    /// (planning/UPSTREAM_PARSER_GAPS.md #26, resolved upstream in `cb026cd`) and is lowered the same way.
    RenderingDefinition,
    /// `allocation def` (BNF AllocationDefinition): a type whose owned members share
    /// `DefinitionBody`/`OccurrenceBodyElement` with `OccurrenceDefinition`, mirroring
    /// `lower_occurrence_def` lowering: ownership, membership, an optional `:>` specialization
    /// relationship participating in the shared `DeclarationDomain::Type` fixed point, and owned
    /// attribute/part/item/nested-occurrence members plus `end` connector-end structure (reusing
    /// `ReferenceKind::ConnectorEnd`/`lower_end_decl`, the same machinery `connection def` uses)
    /// via the shared `lower_occurrence_body_element` walker. Verified `AllocationDef`'s
    /// `specializes: Option<Node<TypingRelationship>>` field carries full parity with
    /// `OccurrenceDef`/`ConnectionDef`. Allocation-specific semantics (the `allocate ... to ...`
    /// binding itself) are out of scope here and stay `unsupported_occurrence_definition_member`
    /// (the shared family `lower_occurrence_body_element` already uses). `allocation` usage
    /// lowering is deferred entirely: `AllocationUsage` was not verified for field parity and is
    /// not attempted here.
    AllocationDefinition,
    /// `flow def` (BNF FlowDefinition, Clause 8.2.2.16): a type whose owned members share
    /// `DefinitionBody`/`OccurrenceBodyElement` with `OccurrenceDefinition`/`AllocationDefinition`,
    /// mirroring `lower_allocation_def`/`lower_occurrence_def` lowering: ownership, membership, an
    /// optional `:>` specialization relationship participating in the shared
    /// `DeclarationDomain::Type` fixed point, and owned attribute/part/item/nested-occurrence
    /// members plus `end` connector-end structure via the same shared
    /// `lower_occurrence_body_element` walker. Verified `FlowDef`'s `specializes: Option<Node<
    /// TypingRelationship>>` field carries full parity with `OccurrenceDef`/`ConnectionDef`.
    /// Flow-payload (`ref :>> payload : Type;`) and succession-flow semantics are out of scope
    /// here and stay `unsupported_occurrence_definition_member`. `flow` usage lowering is
    /// deferred entirely: `FlowUsage` was not verified for field parity and is not attempted
    /// here.
    FlowDefinition,
    /// A package/definition/usage-level `view` feature member (BNF ViewUsage), mirroring
    /// `lower_analysis_case_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #8): `ViewUsage` previously had no `subsets` field to lower this
    /// relationship from. View-specific body members remain out of scope, sharing
    /// `UnsupportedFamily::ViewDefinitionMember` with the `def` form's body walker.
    ViewUsage,
    /// A package/definition/usage-level `rendering` feature member (BNF RenderingUsage),
    /// mirroring `lower_view_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships (`ast::RenderingUsage` now carries full
    /// field parity, planning/UPSTREAM_PARSER_GAPS.md #26, resolved upstream in `cb026cd`). The body
    /// (`RenderingUsageBody`) recurses into nested `view`/`rendering` usage members via
    /// `lower_view_usage`/`lower_rendering_usage` themselves; anything else stays
    /// `UnsupportedFamily::PackageMember`.
    RenderingUsage,
    /// A package/definition/usage-level `use case` feature member (BNF UseCaseUsage), mirroring
    /// `lower_case_usage`: ownership, membership, a `:` typing target, and owned-member structure
    /// via the shared `lower_case_family_def_body` walker. `ast::UseCaseUsage` has no
    /// `subsets`/`redefines`/multiplicity field at all (unlike `CaseUsage`/`AnalysisCaseUsage`),
    /// so only the plain `use case <name>[: <Type>] { ... }` header shape reaches this lowering;
    /// a `[mult] nonunique :> <target>` header (the Systems Library's own base-feature
    /// declaration idiom) still fails to parse into `UseCaseUsage` at all.
    UseCaseUsage,
    /// A package/definition/usage-level `verification` feature member (BNF
    /// VerificationCaseUsage), mirroring `UseCaseUsage`'s lowering (shares the same field
    /// shape/limitation: no `subsets`/`redefines`/multiplicity field on `ast::VerificationCaseUsage`).
    VerificationCaseUsage,
    /// A package/definition/usage-level `viewpoint` feature member (BNF ViewpointUsage),
    /// mirroring `lower_viewpoint_def`: ownership, membership, an optional `:` typing target, and
    /// owned-member structure via the shared `lower_requirement_shaped_body` walker
    /// `viewpoint def`/`requirement def` use. A header-level `:>`/`:>>` clause parses into
    /// `ast::ViewpointUsage::subsets`/`redefines`, but is not lowered yet
    /// (planning/UPSTREAM_PARSER_GAPS.md, "Typed upstream, not yet lowered here").
    ViewpointUsage,
    /// A package/definition/usage-level `interface` feature member (BNF InterfaceUsage),
    /// mirroring `lower_interface_def`: ownership, membership, an optional `:` typing target,
    /// `subsets`/`redefines` subsetting relationships, and connector-end structure (`connect`/
    /// `end`) via the same `ReferenceKind::ConnectorEnd` machinery `interface def` uses. Resolved
    /// upstream in `0757de13` (planning/UPSTREAM_PARSER_GAPS.md #6): all three `InterfaceUsage` variants
    /// now carry `subsets`/`redefines` fields with full parity to `ConnectionUsageMember`.
    /// Interface-specific semantics beyond declaration/typing/ends are out of scope, sharing
    /// `UnsupportedFamily::InterfaceDefinitionMember` with the `def` form's body walker.
    InterfaceUsage,
    /// `constraint def` (BNF ConstraintDefinition): a type whose owned members participate in the
    /// same Subclassification/FeatureTyping `DeclarationDomain::Type` fixed point as
    /// `ViewDefinition`/`OccurrenceDefinition`. `ConstraintDef` has full field parity with
    /// `ViewDef`/`ActionDef` (`specializes: Option<Node<TypingRelationship>>`). Constraint-body
    /// expression semantics are out of scope and fall through to
    /// `UnsupportedFamily::ConstraintDefinitionMember`.
    ConstraintDefinition,
    /// A package/definition/usage-level `constraint` feature member (BNF ConstraintUsage),
    /// mirroring `lower_analysis_case_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #4): `ConstraintUsage` previously had no `subsets`/`redefines`
    /// fields at all.
    ConstraintUsage,
    /// An `assert constraint <name>? { ... }` member.
    ///
    /// OMG `AssertConstraintUsage`, a concrete metaclass in its own right
    /// (`AssertConstraintUsage <: Invariant, ConstraintUsage`) rather than a plain constraint.
    AssertConstraintUsage,
    /// An `assume constraint <name>? { ... }` member of a requirement body.
    ///
    /// OMG `ConstraintUsage` owned by a `RequirementConstraintMembership` whose `kind` is
    /// `assumption`; the keyword is the only thing that distinguishes it from the `require` form,
    /// so it needs its own declaration kind for that role to be derivable.
    AssumeConstraintUsage,
    /// A `require constraint <name>? { ... }` member of a requirement body.
    ///
    /// As above, with `RequirementConstraintMembership.kind` = `requirement`.
    RequireConstraintUsage,
    /// `concern def` (BNF ConcernDefinition, Clause 8.2.2.11): a type whose owned members share
    /// `RequirementDefBody`/`RequirementDefBodyElement` with `RequirementDefinition`, mirroring
    /// `lower_viewpoint_def`. The parser models both `concern def` and `concern` under a single
    /// `ast::requirement::ConcernUsage` struct discriminated by `is_definition`, rather than a
    /// distinct `ConcernDef` type -- see that struct's doc comment. Genuinely new: previously
    /// blocked entirely (planning/UPSTREAM_PARSER_GAPS.md #9: no `specializes`/`subsets`/`redefines` field
    /// at all). Stakeholder/subject-binding semantics are out of scope, sharing
    /// `UnsupportedFamily::RequirementDefinitionMember` with `requirement def`/`viewpoint def`.
    ConcernDefinition,
    /// A package/definition/usage-level `concern` feature member (BNF ConcernUsage), mirroring
    /// `lower_requirement_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #9).
    ConcernUsage,
    /// `calc def` (BNF CalculationDefinition, Clause 8.2.2.14): a type whose owned members
    /// participate in the shared Subclassification/FeatureTyping `DeclarationDomain::Type` fixed
    /// point, mirroring `lower_view_def`/`lower_action_def`. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #3): `CalcDef` previously dropped its parsed `:>` specialization
    /// clause; it now carries `specializes: Option<Node<TypingRelationship>>` with full parity to
    /// `ActionDef`/`ViewDef`. Genuinely new: `calc def`/`calc usage` lowering was never attempted
    /// before this gap was resolved. Calculation-expression body content, `in`/`out`/`return`
    /// parameters, and nested `calc` structure are out of scope and fall through to
    /// `UnsupportedFamily::CalcDefinitionMember`.
    CalcDefinition,
    /// A package/definition/usage-level `calc` feature member (BNF CalculationUsage), mirroring
    /// `lower_analysis_case_usage`: ownership, membership, a `:` typing target, and `redefines`
    /// (a bare `Vec<QualifiedReferenceId>`, not a `SubsettingRelationship` node the way other
    /// usage kinds' `redefines` field is shaped -- `CalcUsage` has no `subsets` field at all).
    /// Direction (`in`/`out`/`inout`)/value-binding/body content are out of scope, sharing
    /// `UnsupportedFamily::CalcDefinitionMember` with the `def` form.
    CalcUsage,
    /// KerML `class def` (BNF ClassDefinition): a type whose owned members participate in the
    /// shared Subclassification/FeatureTyping `DeclarationDomain::Type` fixed point, mirroring
    /// `lower_item_def`. Resolved upstream in `0757de13` (planning/UPSTREAM_PARSER_GAPS.md #2): `ClassDef`
    /// previously had unparsed `:>` specialization inside the body; it now carries a typed
    /// `specializes: Option<Node<TypingRelationship>>` plus a plain `AttributeBody`, exactly the
    /// same shape `ItemDef` has. There is no separate KerML "class usage" form in the grammar --
    /// only the def-level construct is lowered here. Class-specific semantics beyond ownership,
    /// specialization, and owned-member structure are out of scope.
    ClassDefinition,
    Import,
    Alias,
    /// An anonymous entry-action-binding feature synthesized for a state def/usage's `entry
    /// action <path> ...;` body element (BNF `EntryAction.action_reference`), owned by the
    /// enclosing state declaration, mirroring `Succession`'s nested-declaration shape so the
    /// bound action reference resolves against the state's own scope (where sibling actions are
    /// declared), not the state's enclosing scope.
    EntryActionBinding,
    /// Same as `EntryActionBinding`, for a `do action <path> ...;` body element
    /// (`DoAction.action_reference`).
    DoActionBinding,
    /// Same as `EntryActionBinding`, for an `exit action <path> ...;` body element
    /// (`ExitAction.action_reference`).
    ExitActionBinding,
    /// An anonymous initial-state-binding feature synthesized for a state def/usage's `then
    /// <target>;` body element (BNF `ThenStmt.state_reference`), owned by the enclosing state
    /// declaration, mirroring `EntryActionBinding`'s nested-declaration shape.
    InitialState,
    /// A named final pseudo-state declared by a state def/usage's `final <name>;`/`final state
    /// <name>;` body element (BNF `FinalState`, `ast::FinalState`), owned by the enclosing state
    /// declaration. Unlike `InitialState` (which references an existing sibling state), `final
    /// <name>;` *declares* a brand-new nested state feature -- there is no separate reference to
    /// resolve, so this mirrors `lower_state_usage`'s plain named-declaration shape rather than
    /// `EntryActionBinding`'s reference-binding shape. `FinalState.state_name` is a plain `String`
    /// (not a structured typing/reference), so no other relationship is lowered.
    FinalState,
    /// A directed `in`/`out`/`inout` parameter declaration (BNF `InOutDecl`, `ast::InOutDecl`)
    /// found in a `calc def`/`constraint def`/`action def` body, e.g. `in partMasses :
    /// MassValue[0..*];`. Mirrors `ItemUsage`/`MetadataUsage` lowering: ownership, membership,
    /// and (when present) a `FeatureTyping` reference to the declared type. The `in`/`out`/
    /// `inout` direction itself is not modeled as a distinct declaration kind's own field --
    /// it is carried as an explicit `RelationshipFlags::direction` fact on the pushed
    /// `FeatureTyping` reference, mirroring how `PortUsage`'s conjugation polarity rides the
    /// `conjugated` flag on the same reference rather than becoming a new relationship kind.
    /// When the parameter has no type (`type_name` is `None`, e.g. a bare `in seq[1..*] nonunique
    /// ordered;` untyped/collection-modified form, or the leading `in :>> target = ...`
    /// redefinition form), no `FeatureTyping` reference is pushed and the direction fact is not
    /// recorded -- only the declaration/membership shell is lowered for that shape. A `redefines`
    /// (`ast::InOutDecl::redefines`, BNF `:>`/`:>>`) clause -- e.g. `in value[1] :> seq;`
    /// subsetting another parameter -- is lowered via the shared `lower_subsetting_relationship`
    /// helper, exactly as `AttributeUsage`/`ItemUsage` already do, independent of whether a type
    /// is present. Multiplicity (`[0..*]`/`[1..*]`) and collection modifiers (`nonunique`/
    /// `ordered`) are not modeled anywhere else in this codebase yet (attribute/part usages with
    /// array types don't carry a multiplicity fact either), so they are left unrepresented here too.
    ParameterUsage,
    /// A `subject` declaration (BNF `SubjectDecl`, `ast::SubjectDecl`) found in a requirement/
    /// concern/case-family def or usage body, e.g. `subject vehicle : Vehicle;` inside
    /// `requirement vehicleSpecification`. Structurally a plain typed feature declaration --
    /// name plus an optional `FeatureTyping` reference to the declared type -- mirroring
    /// `lower_parameter_declaration`'s shape but without a direction fact. Per
    /// planning/RESOLUTION_LAYER_DESIGN.md §5.4, `Subject` is a derived case-level relationship projected
    /// from this ordinary `FeatureTyping` fact by a later query-layer owner, not a distinct
    /// authored reference kind here; multiplicity and the bare `subject = expr;`/`subject;`
    /// shorthand forms are left unlowered, matching `ParameterUsage`'s scope.
    SubjectUsage,
    /// An explicit `perform action <name> : <Type>;` performance usage (BNF `Perform`,
    /// `ast::Perform`) found in a part def/usage or action def/usage body, e.g. `perform action
    /// generateTorque: GenerateTorque;` inside `part def Engine`. Mirrors `lower_action_usage`'s
    /// shape: ownership, membership, an optional `FeatureTyping`/`Subclassification` reference to
    /// the performed action type, and `subsets`/`redefines` specialization. The shorthand `perform
    /// <path>;` reference form (`Perform::action_reference`, no declaration label) and body
    /// content beyond nested `part`/`item` usages are out of scope for this slice.
    PerformActionUsage,
    /// An anonymous `transition` feature synthesized for a state def/usage's `transition ...;`
    /// body element (BNF `Transition`, `ast::Transition`), owned by the enclosing state
    /// declaration, mirroring `Succession`'s nested-declaration shape (this task picks up the
    /// full construct explicitly deferred by `4762b875`). `source`/`target` are lowered as
    /// `ReferenceKind::TransitionSource`/`TransitionTarget` references (mirroring
    /// `lower_succession_end`), a supported `guard` boolean expression is lowered/evaluated
    /// through the exact same `ExpressionOperand`/`classify_constraint_expression` machinery a
    /// `constraint`/`calc` body uses, an `accept` shorthand trigger (`TransitionAccept::
    /// Shorthand`) that is a simple/qualified name is lowered as a `TransitionTrigger`
    /// reference, and a `do` effect that is either `TransitionEffect::Perform`'s typed
    /// `type_name` or a simple/qualified-name `TransitionEffect::Expression` is lowered as a
    /// `TransitionEffect` reference (mirroring `EntryActionBinding`'s action-reference shape).
    /// A typed `accept` payload declaration (`TransitionAccept::Payload`), a time trigger
    /// (`TransitionAccept::TimeTrigger`), and the richer `Accept`/`Send`/`Assign` effect shapes
    /// are out of scope for this slice (not a parser gap -- the typed AST is adequate, this is a
    /// deliberate scope boundary) and fall through to the existing
    /// `unsupported_state_definition_member` diagnostic.
    Transition,
    /// An anonymous feature synthesized for a `satisfy <requirement> by <element>;` body element
    /// (BNF `Satisfy`, its bare shorthand form) found in a package/part def/part usage/occurrence
    /// body. Owned by the enclosing declaration, mirroring `Succession`/`Transition`'s
    /// nested-declaration shape, so the `SatisfySource`/`SatisfyTarget` references it carries are
    /// distinguishable per-statement (multiple `satisfy` statements can share one owner) even
    /// though the statement introduces no name of its own.
    Satisfy,
    /// An anonymous feature synthesized for an `allocate <source> to <target>;` body element
    /// (BNF `Allocate`, `ast::Allocate`) found in a part def/part usage/occurrence body -- the
    /// shorthand allocation *statement* form (asserting an allocation relationship between two
    /// existing declarations), genuinely distinct from `AllocationDefinition`/`AllocationUsage`
    /// (the declaration forms lowered in `04274711`). Owned by the enclosing declaration,
    /// mirroring `Satisfy`'s nested-declaration shape, so the `AllocateSource`/`AllocateTarget`
    /// references it carries are distinguishable per-statement (multiple `allocate` statements
    /// can share one owner) even though the statement introduces no name of its own.
    Allocate,
    /// An anonymous feature synthesized for a `bind <source> = <target>;` body element (BNF
    /// `Bind`, `ast::Bind`, found in part def/part usage/action def/action usage bodies) or a
    /// package-level `binding ... left = right;` element (BNF `BindingConnectorUsage`,
    /// `ast::BindingConnectorUsage`) -- both assert a binding-connector relationship between two
    /// existing declarations, mirroring `Allocate`/`Satisfy`'s "statement, not a new named usage"
    /// shape. Owned by the enclosing declaration, mirroring `Allocate`'s nested-declaration shape,
    /// so the `BindSource`/`BindTarget` references it carries are distinguishable per-statement
    /// (multiple `bind`/`binding` statements can share one owner) even though the statement
    /// introduces no name of its own (the optional `binding <name>` prefix on `Bind`, and the
    /// optional name on `BindingConnectorUsage`, are both left out of scope -- see their
    /// `ReferenceKind` doc comments).
    Bind,
    /// A `ref <name>: <Type>;` non-owning referential feature (BNF `ReferenceUsage`,
    /// `ast::RefDecl`), e.g. `ref self: Part :>> Item::self;` (SysML Systems Library
    /// `Parts.sysml`). Distinct from `PartUsage`/`AttributeUsage`/etc. even though it shares their
    /// `FeatureMembership` ownership/typing/redefines/subsets shape: unlike those keyword-typed
    /// usages, `ref`'s own keyword carries no type-family information at all -- its declared type
    /// comes entirely from the `:`/`:>>`/`:>` clauses, and the same `ref` syntax is reused verbatim
    /// across part/action/state/connection/interface/package bodies, so it is not a specialization
    /// of any one existing `*Usage` kind. A `ref { ... }` body holds the general usage-member set
    /// (`RefBody = Body<PartUsageBodyElement>`) and is dispatched through the shared
    /// `lower_part_usage_body_element` walker; members that walker does not model are reported
    /// under `UnsupportedFamily::ReferenceUsageMember`.
    ReferenceUsage,
    /// An anonymous feature synthesized for a `decide <expr>;`/`decide <expr> { ... }` decision
    /// control node (BNF `DecisionStmt`, `ast::DecisionStmt`) found either as a standalone action
    /// def/usage body element or as a `then decide <expr>;` continuation (`ThenTarget::Decide`).
    /// Owned by the enclosing action def/usage declaration, mirroring `Succession`/`Transition`'s
    /// nested-declaration shape: the required `decide` operand is lowered as a
    /// `ReferenceKind::DecisionInput` reference exactly like `lower_succession_end` resolves a
    /// `FirstStmt` end, and a braced body's nested members (in/out parameters, nested action
    /// usages, further `then <target>;` continuations) recurse through the same dispatch as an
    /// ordinary action def body. This is the priority construct for this slice -- the `if <guard>
    /// then <target>;` branches a decision fans out to are ordinary sibling `IfStmt`/`ThenAction`
    /// body elements (not nested inside the decision node's own body), already reusing the
    /// existing `classify_constraint_expression`/lexical-lookup machinery via `lower_then_action`.
    Decide,
    /// An anonymous feature synthesized for a `merge <expr>;`/`merge <expr> { ... }` merge
    /// control node (BNF `MergeStmt`, `ast::MergeStmt`), same shape and scope as `Decide`: the
    /// required `merge` operand is a `ReferenceKind::MergeInput` reference.
    Merge,
    /// An anonymous feature synthesized for a `fork <expr>;`/`fork <expr> { ... }` fork control
    /// node (BNF `ForkStmt`, `ast::ForkStmt`), same shape and scope as `Decide`: the required
    /// `fork` operand is a `ReferenceKind::ForkInput` reference. A braced body's `in`/`out`
    /// parameter declarations (the fork's output flows) lower through the same
    /// `lower_parameter_declaration` as an ordinary action def body.
    Fork,
    /// An anonymous feature synthesized for a `join <expr>;`/`join <expr> { ... }` join control
    /// node (BNF `JoinStmt`, `ast::JoinStmt`), same shape and scope as `Decide`: the required
    /// `join` operand is a `ReferenceKind::JoinInput` reference.
    Join,
    /// An anonymous feature synthesized for a bare `then <target>;` continuation statement (BNF
    /// `ThenAction`, `ThenTarget::Feature`) found in an action def/usage body, mirroring
    /// `Succession`'s nested-declaration shape: the reference must be sourced at a declaration
    /// owned by the enclosing action (not the action itself) so the shared `DeclarationDomain::
    /// Any` lexical lookup searches the action's own children (its sibling control-flow nodes),
    /// exactly like every other paired/single-operand control-flow reference kind.
    ThenContinuation,
    /// An anonymous feature synthesized for a standalone `flow <source> to <target>;` statement
    /// (BNF `FlowUsage`'s bare from/to shorthand, `ast::FlowUsage` with `name`/`type_name`/
    /// `payload` all absent), found inside an action def/usage body. Mirrors `Bind`/`Allocate`'s
    /// anonymous nested-declaration shape: `from`/`to` are lowered as authored `FlowSource`/
    /// `FlowTarget` references sourced at this new declaration (not at `owner` directly), so
    /// multiple `flow ...;` statements in the same body stay distinguishable. Deliberately narrow:
    /// a named/typed flow usage or def (`flow f : T { ... }`) and the `of <payload>` clause remain
    /// out of scope -- only the bare two-operand statement form's `from`/`to` references are
    /// resolved here.
    Flow,
    /// A `stakeholder` member found in a requirement/viewpoint def body (BNF `StakeholderMember`,
    /// `ast::requirement::StakeholderMember`), e.g. `stakeholder driver : Driver;` inside
    /// `requirement def SafetyRequirement`. The typed AST folds three distinct textual shapes into
    /// one struct: a plain typed declaration (`declaration_name`/`type_name`, mirroring
    /// `SubjectUsage`'s shape exactly -- ownership, membership, an optional `FeatureTyping`
    /// reference, no direction fact), a bare concern reference (`stakeholder Concern;`, `target`
    /// set, `declaration_name` empty), and a `:>>` redefinition (`stakeholder :>> name;`,
    /// `is_redefinition` true, `target` set). `intern_declared_name` already folds an empty
    /// `declaration_name` to an anonymous declaration, exactly like `SubjectUsage`'s own bare
    /// `subject;` shorthand. The `target` reference (plain or redefinition) is lowered as an
    /// authored `ReferenceKind::StakeholderTarget`/`ReferenceKind::Redefinition` reference sourced
    /// at this same declaration.
    StakeholderUsage,
    /// A typed `actor` parameter declaration found in a requirement def body (BNF
    /// `RequirementActorDecl`, `ast::requirement::RequirementActorDecl`), e.g. `actor pilot :
    /// Operator;` inside `requirement def FlightRequirement`. Structurally identical to
    /// `SubjectUsage` (a plain typed feature declaration: ownership, membership, a `FeatureTyping`
    /// reference to the declared type, no direction fact) except `type_name` is unconditional here
    /// (never optional), unlike `SubjectDecl::type_name`. Distinct from `UseCaseDefBodyElement::
    /// ActorUsage` (`ast::requirement::ActorUsage`), a different AST shape found only in case-family
    /// bodies, which is out of scope for this slice.
    RequirementActor,
    /// An `actor` member found in a use-case-family (`use case`/`analysis`/`case`/`verification`)
    /// def or usage body (BNF `ActorUsage`, `ast::requirement::ActorUsage`), e.g. `actor driver :
    /// Person;` inside `use case def DriveVehicle`. Distinct from `RequirementActor`
    /// (`RequirementActorDecl`), a different AST shape found only in requirement/concern bodies:
    /// mirrors it structurally (ownership, membership, an unconditional `FeatureTyping` reference
    /// to the declared type) but reads visibility off `ActorUsage::membership` (kind always
    /// `ActorMembership`) rather than `RequirementActorDecl`'s own membership. The optional
    /// trailing multiplicity (`actor passengers : Person[0..4];`) is not modeled as a distinct
    /// fact, mirroring `lower_subject_decl`'s own out-of-scope multiplicity.
    CaseActor,
    /// A named `frame` member found in a requirement def body (BNF `FrameMember`,
    /// `ast::requirement::FrameMember`), e.g. `frame concernFraming { stakeholder ...; }` -- a
    /// purely syntactic named grouping around further requirement-frame body content (subject/
    /// actor/stakeholder/etc.), not a fact-bearing construct of its own. Lowered as an anonymous-
    /// named owned feature (ownership, membership, no reference of its own) whose body is
    /// dispatched back through the same shared `RequirementDefBody`/`RequirementDefBodyElement`
    /// walker (`lower_requirement_shaped_body`) used by `requirement def`/`requirement` usage/
    /// `viewpoint def` bodies, sharing the caller's `UnsupportedFamily` so an unrecognized member
    /// nested inside a frame reports under the same diagnostic family as one outside it.
    Frame,
    /// An anonymous feature synthesized for a requirement/objective-body `verify <requirement>;`
    /// shorthand body element (BNF `VerifyRequirementMember`, `ast::requirement::
    /// VerifyRequirementMember`, `explicit_requirement_keyword == false`), e.g. `verify
    /// speedRequirement;` inside a `verification def`'s `objective { ... }`. Owned by the enclosing
    /// declaration, mirroring `Satisfy`'s nested-declaration shape, so the `VerifyRequirementTarget`
    /// reference it carries is distinguishable per-statement. The shorthand `:>>` redefinition
    /// (`VerifyRequirementMember::redefines`) is lowered as an authored `ReferenceKind::Redefinition`
    /// reference sourced at the same declaration, mirroring `AttributeUsage::redefines`'s existing
    /// bare-`QualifiedReferenceId` handling. The fuller `verify requirement <name> : <Type> { ... }`
    /// form (`explicit_requirement_keyword == true`, which defines a new anonymous requirement usage
    /// inline rather than referencing an existing one -- a meaningfully different construct, not
    /// merely an unresolved reference, mirroring `Satisfy::inline_requirement`'s own scope boundary)
    /// is out of scope and left as an explicit unsupported-member diagnostic.
    VerifyRequirement,
    // --- Bodied KerML classifier declarations (`KermlClassifierDecl`) ---------------------
    //
    // One variant per metaclass the declaration's keyword denotes. KerML makes these distinct
    // concrete metaclasses in a subtype lattice -- `Predicate <: Function <: Behavior <: Class
    // <: Classifier <: Type`, `Structure <: Class`, `Interaction <: Association, Behavior`,
    // `Multiplicity <: Feature` -- so a single bucket would erase real metaclass identity.
    // `ast::KermlClassifierDecl.keyword` already carries the spelling; the lowering reads it.
    //
    // All of them share one lowering shape (ownership, an optional `specializes` relationship,
    // and owned members through the shared `lower_calc_def_body` walker). Header
    // `type_relationships` (`disjoint from`/`unions`/`intersects`) remain out of scope.
    /// `type UnionType unions A, B;` -- the general type declaration. KerML `Type`.
    KermlType,
    /// `classifier C { ... }`. KerML `Classifier`.
    KermlClassifier,
    /// `class C { ... }` in the KerML fallback grammar. KerML `Class`.
    KermlClass,
    /// `struct S { ... }`. KerML `Structure`.
    KermlStructure,
    /// `assoc A { ... }`. KerML `Association`.
    KermlAssociation,
    /// `assoc struct L { ... }`. KerML `AssociationStructure`.
    KermlAssociationStructure,
    /// `datatype D { ... }`. KerML `DataType`.
    KermlDataType,
    /// `metaclass M { ... }`. KerML `Metaclass`.
    KermlMetaclass,
    /// `behavior B { ... }`. KerML `Behavior`.
    KermlBehavior,
    /// `function isZero specializes DataFunctions::isZero { ... }`. KerML `Function`.
    KermlFunction,
    /// `predicate P { ... }`. KerML `Predicate`.
    KermlPredicate,
    /// `interaction I { ... }`. KerML `Interaction`.
    KermlInteraction,
    /// `multiplicity m [0..1] { ... }` and the bare forward declaration `multiplicity m [0..1];`.
    /// KerML `Multiplicity`.
    ///
    /// Both spellings reach `ast::KermlClassifierDecl` (the bare form as a `;` body), so both
    /// lower as resolvable declarations.
    KermlMultiplicity,

    // --- KerML feature members (`KermlFeatureMember`) ------------------------------------
    //
    // Likewise one variant per metaclass the kind keyword denotes: `BooleanExpression <:
    // Expression <: Step <: Feature`. `ast::KermlFeatureMember.kind` carries the spelling.
    //
    // Shared lowering shape: ownership, membership, an optional `:` typing target
    // (`FeatureTyping`), `subsets`/`redefines` relationships, and owned members through
    // `lower_calc_def_body`. `references`/`chains`/`inverse_of`/`type_relationships` are still
    // not modeled as distinct facts.
    /// `feature x : Integer;`, `derived var feature annotatedElement : Element[1..*] ordered
    /// redefines annotatedElement;`. KerML `Feature`.
    KermlFeature,
    /// `step performances : Performance[0..*] subsets occurrences { ... }`. KerML `Step`.
    KermlStep,
    /// `expr evaluations : Evaluation[0..*] nonunique subsets performances { ... }`. KerML
    /// `Expression`.
    KermlExpression,
    /// `bool earlierFirstIncomingTransferSort : IncomingTransferSort { ... }`. KerML
    /// `BooleanExpression`.
    KermlBooleanExpression,
    /// A keyword-less `<name> = <expr>;` / `<name> : <Type>;` binding (`DefaultReferenceUsage`,
    /// BNF §8.2.2.6 / Spec §7.6.4), e.g. `baseType = Atom meta KerML::Classifier;` (KerML
    /// `metaclass` body) or the anonymous leading-redefinition form `:>> dimension =
    /// size(components);`. Mirrors `ReferenceUsage`/`KermlFeature` lowering: ownership,
    /// membership, an optional `:` typing target (`FeatureTyping`), `subsets`/`redefines`
    /// relationships, and an optional `=` value expression resolved through the shared
    /// `classify_calc_expression`/`lower_calc_expression` pipeline. Multiplicity and the
    /// `has_feature_keyword`/`body` shapes are not modeled as distinct facts here (multiplicity
    /// is unmodeled elsewhere in this codebase too, see `ParameterUsage`).
    DefaultReferenceUsage,
    /// A KerML connector member (`KermlConnectorMember`), e.g. `connector fixWheel :
    /// BikeWheelFixed from [1] rollsOn to [1] holdsWheel;` (KerML Spec Annex A-3-3). Mirrors
    /// `lower_connection_def`'s ownership/typing/end shape: ownership, membership, an optional
    /// `:` typing target (`FeatureTyping`), and `from`/`to` ends resolved as
    /// `ReferenceKind::ConnectorEnd` references through the same shared lexical lookup
    /// `connection def`/`interface def` use. `is_all`, end multiplicities, and each end's
    /// `references` chain are not modeled as distinct facts here.
    KermlConnector,
    /// A KerML binding connector member (`KermlBindingMember`), e.g. `binding [1] startShot =
    /// [1] endShot;` (KerML Spec §8.2.4). Structurally the keyword-full sibling of
    /// `BindingConnectorUsage`/`Bind` -- mirrors `lower_binding_connector_usage`'s two-reference
    /// shape (`ReferenceKind::BindSource`/`BindTarget`) applied to each end's `target`. The
    /// declared name/multiplicity and each end's `references` chain are not modeled as distinct
    /// facts here.
    KermlBinding,
    /// A KerML invariant member (`KermlInvariantMember`), e.g. `inv unitBound { -1.0 <= that &
    /// that <= 1.0 }` or the anonymous `inv { isClosed == true }` (KerML Spec §8.2.7). Its body
    /// shares the `CalcDefBody` grammar (not `ConstraintDefBody`), so its boolean expression is
    /// classified/lowered through the existing `classify_calc_expression`/`lower_calc_expression`
    /// pipeline via the shared `lower_calc_def_body` walker, mirroring `AssertConstraintMember`'s
    /// "anonymous nested declaration" pattern. `is_negated` is not modeled as a distinct fact
    /// here (see `AssertConstraintMember`'s own `is_negated` scope boundary).
    KermlInvariant,
    /// A KerML end member with an owned cross feature (`KermlEndMember`), e.g. `end happensDuring
    /// [1..*] subsets timeCoincidentOccurrences feature thatOccurrence: Occurrence redefines
    /// longerOccurrence;` (KerML Spec Annex A-3, association-end form). Distinct from a plain
    /// `end feature ...` member, which lowers directly as `DeclarationKind::KermlFeature` with
    /// `is_end` unmodeled -- here the end itself is named/constrained and owns a nested
    /// `KermlFeatureMember`. Mirrors `lower_kerml_connector_member`'s ownership/membership shape:
    /// ownership, membership, an optional `subsets` relationship on the end itself (through the
    /// existing `lower_subsetting_relationship`), and the owned nested feature lowered through
    /// the existing `lower_kerml_feature_member` (itself owned by this end declaration, not the
    /// enclosing `assoc`/type). The end's own multiplicity is not modeled as a distinct fact here.
    KermlEnd,
    /// An anonymous feature synthesized for an `assign <target> := <value>;` reassignment
    /// statement (BNF `AssignStmt`, `ast::AssignStmt`, `is_then` covering both the plain and
    /// `then assign ...;` spellings) found in an action def/usage body, mirroring `Bind`'s
    /// "statement, not a new named usage" nested-declaration shape: owned by the enclosing action
    /// def/usage declaration, so the `AssignTarget` reference and the value expression's own
    /// operand references are distinguishable per-statement even though the statement introduces
    /// no name of its own. The `lhs` target is lowered as a `ReferenceKind::AssignTarget`
    /// reference through the same `DeclarationDomain::Any` lexical lookup `Succession`/
    /// `ThenTarget` use (an existing sibling feature, not just a Type); the `rhs` value is lowered
    /// through the shared `lower_value_assignment`-style `classify_constraint_expression`/
    /// `lower_constraint_expression` pipeline, publishing its own evaluation fact exactly like an
    /// attribute default value.
    Assign,
    /// An anonymous feature synthesized for a `while <condition> { ... }` loop control node (BNF
    /// `WhileStmt`, `ast::WhileStmt`) found in an action def/usage body. Owned by the enclosing
    /// action def/usage declaration, mirroring `Decide`/`Merge`'s nested-declaration shape: the
    /// required boolean `condition` is lowered through the same `classify_constraint_expression`/
    /// `lower_constraint_expression` machinery already used for `decide`'s branch guards/
    /// transition guards/filter conditions (not `lower_succession_end`'s narrow feature-reference
    /// shape, since a loop condition is a genuine boolean expression, not a control-node
    /// reference). The body's nested statements recurse through the same `lower_action_def_body`
    /// dispatch this variant is itself reached from (bodies are typed `ActionDefBody` regardless
    /// of whether the enclosing action is a def or a usage), owned by this `While` declaration so
    /// nested action usages/parameters stay scoped to the loop, mirroring `Decide`/`Fork`'s own
    /// braced-body scope shift.
    While,
    /// An anonymous feature synthesized for a bare `loop { ... }` control node (BNF `LoopStmt`,
    /// `ast::LoopStmt` -- a `while` with no condition), same shape and scope as `While` minus the
    /// condition: only the body recurses.
    Loop,
    /// An anonymous feature synthesized for an `if <condition> { ... } (else { ... })?` control
    /// node (BNF `IfStmt`, `ast::IfStmt`) found in an action def/usage body, same condition
    /// handling as `While`. Both `then_body` and `else_body` (when present) recurse through the
    /// same `lower_action_def_body` dispatch, owned by this one `If` declaration -- branch bodies
    /// are not distinguished from one another as separate declaration scopes, mirroring how
    /// `Decide`'s own braced body is a single undifferentiated scope.
    If,
    /// An anonymous feature synthesized for a `for <var> in <range> { ... }` loop control node
    /// (BNF `ForLoop`, `ast::ForLoop`) found in an action def/usage body. Owned by the enclosing
    /// action def/usage declaration, mirroring `While`/`Decide`'s nested-declaration shape: the
    /// `range` collection expression is lowered through the same `classify_constraint_expression`/
    /// `lower_constraint_expression` machinery as `While`'s condition (sourced at this `ForLoop`
    /// declaration, not the loop variable, since the range is evaluated once per loop, not once
    /// per iteration binding). The body recurses through `lower_action_def_body`, owned by this
    /// `ForLoop` declaration so the loop variable declared alongside it is a visible sibling
    /// through the shared `DeclarationDomain::Any` lexical lookup every reference inside the body
    /// already uses.
    ForLoop,
    /// The loop variable declared by a `for <var> in <range> { ... }` statement (`ForLoop.var`,
    /// a bare `String` -- the parser records no type or multiplicity for it), e.g. `for i in
    /// 1..10 { ... }`'s `i`. Lowered as a named feature owned by the enclosing `DeclarationKind::
    /// ForLoop` declaration (a sibling of the loop body's own members, not the body's owner
    /// itself), introducing a binding with no reference of its own -- mirroring `InOutDecl`'s
    /// "the name introduces a binding, it does not reference one" scope boundary. No type
    /// inference from `range`'s element type is performed; this is reference-resolution scope
    /// only, not execution semantics.
    ForLoopVariable,
    /// A package/part-def-body-level `dependency` relationship declaration (BNF Dependency,
    /// `requirement.rs` struct `Dependency`): `dependency` (Identification `from`)? client(s)
    /// `to` supplier(s) RelationshipBody. Unlike a usage/definition, `Dependency` has no
    /// `membership: Membership` field of its own (no visibility prefix support), so it is always
    /// lowered with `MembershipKind::Feature`/`Visibility::Default`, mirroring `lower_satisfy`'s
    /// anonymous-relationship-declaration shape. Each client and supplier is an independent
    /// authored reference (`ReferenceKind::DependencyClient`/`DependencySupplier`) resolved
    /// through the same `DeclarationDomain::Any` lexical lookup as other authored references;
    /// only reference resolution is modeled, not dependency-specific semantics (e.g. no
    /// standalone "Dependency" relationship classification beyond the two reference kinds).
    /// `body_elements` (doc/comment/metadata only, BNF `RelationshipBody`) are walked through the
    /// shared `lower_relationship_body_elements` helper used by `Import`/`AliasDef`.
    Dependency,
    /// `#<keyword>+ def <Name> ...` (BNF ExtendedDefinition, `structure.rs` struct
    /// `ExtendedDefinition`, planning/UPSTREAM_PARSER_GAPS.md gap #12's short form), e.g. `#scenario def
    /// DeviceFailure { ... }`. Gap #12 tracked only the parser production landing upstream; this
    /// is the first `sysml_resolution` lowering attempt. `body: PackageBody` is the exact same
    /// shape an ordinary `package { ... }` body uses, so its owned members are lowered through
    /// the shared `lower_package_body` walker (reused verbatim, mirroring `lower_package`/
    /// `lower_namespace`). An optional `:>` `specializes` clause is lowered like
    /// `AllocationDefinition`/`ViewDefinition`. The `#`-prefix keyword tags themselves
    /// (`prefix_keywords`, a metadata-annotation-shaped list) and the `abstract`/`variation`
    /// `definition_prefix` flag are out of scope, matching every other definition kind's
    /// established "ownership, specialization, and owned-member structure only" scope boundary.
    ExtendedDefinition,
    /// `individual def` (BNF IndividualDef, `structure.rs` struct `IndividualDef`): a type whose
    /// owned members participate in the shared Subclassification/FeatureTyping
    /// `DeclarationDomain::Type` fixed point, mirroring `lower_item_def`/`lower_class_def` --
    /// `IndividualDef`'s `body: AttributeBody` is the exact same shape, so owned members are
    /// lowered through the existing `lower_attribute_body`. Distinct from the `individual`
    /// usage-side prefix (`individual occurrence def`/`individual item ...`, already handled
    /// elsewhere per planning/UPSTREAM_PARSER_GAPS.md gap #7); this is the standalone `individual def
    /// <Name> [:> <Type>] { ... }` definition form.
    IndividualDefinition,
    /// An anonymous connector feature synthesized for a keyword-less bare `connect <from> to
    /// <to> [:> ...] [:>> ...] { ... }` member (BNF `Connect`, distinct from `ConnectStmt`; see
    /// `lower_bare_connect`), mirroring `ForLoop`/`EntryActionBinding`'s nested-declaration
    /// shape: sourced as a child of the enclosing scope so its `from`/`to` connector-end
    /// references resolve against that scope's own siblings through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point (which starts one level *above* the
    /// reference's source declaration, so a bare top-level `connect a to b;` needs this
    /// intermediate nesting to see `a`/`b` as siblings of the enclosing package/definition, not
    /// as siblings of itself).
    BareConnect,
    /// An anonymous feature synthesized for a `perform` usage's `in`/`out <target> = <value>;`
    /// body element (BNF `PerformInOutBinding`, `ast::structure::PerformInOutBinding`, found only
    /// inside `PerformBody` -- the shorthand parameter-argument-binding form used when invoking a
    /// nested `perform <action>;`, e.g. `perform action dynamics : StraightLineDynamics { in
    /// power = vehiclePower; }`), mirroring `Bind`/`Allocate`'s "statement, not a new named usage"
    /// shape: `target` is an *authored reference* to the invoked action's own declared parameter
    /// (not a new declared name, unlike `InOutDecl`), so it resolves through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point as `BindTarget`, and `value` is a bound
    /// expression resolved through `lower_constraint_expression` exactly like `Assign`'s RHS.
    /// Owned by the enclosing `perform`'s own declaration, so multiple `in`/`out` bindings on one
    /// `perform` are distinguishable per-statement even though the statement introduces no name of
    /// its own. The `direction` (`in`/`out`/`inout`) flag is out of scope, matching `Bind`'s own
    /// "binding keyword prefix ignored" precedent.
    PerformParameterBinding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MembershipKind {
    Owning,
    Feature,
    Import,
    Alias,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Visibility {
    Default,
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ReferenceKind {
    NamespaceImport,
    MembershipImport,
    FilterImport,
    FeatureTyping,
    Subclassification,
    Subsetting,
    Redefinition,
    References,
    Crosses,
    Intersects,
    /// The authored target of an `alias X for Y;` member (`AliasDef::target`), resolved through
    /// the same lexical lookup fixed point as every other authored reference kind. Named
    /// `AliasBinding` to match planning/RESOLUTION_LAYER_DESIGN.md's "alias binding" vocabulary (section
    /// 10.1) rather than inventing new terminology.
    AliasBinding,
    /// The authored target of a connector end (`ConnectStmt`'s `from`/`to`/extra ends, or a bare
    /// `EndDecl`'s `::>`/`references` target), resolved through the same lexical lookup fixed
    /// point as `AliasBinding`: a connector end can reference any feature (not just a Type), so
    /// it is not restricted to the Subclassification/FeatureTyping `Type` domain. Connector-end
    /// referential/multiplicity constraints (matching end types/multiplicities) are explicitly
    /// out of scope; this only resolves the authored reference itself.
    ConnectorEnd,
    /// The authored source/target of a `first X then Y;` control-flow succession statement
    /// inside an action def/usage body (`FirstStmt.first`/`FirstStmt.then`), resolved through
    /// the same `DeclarationDomain::Any` lexical lookup as `ConnectorEnd`: a succession end can
    /// reference any owned action feature, not just a Type. Mirrors `lower_connect_stmt`/
    /// `lower_connector_end`'s two-references-from-owner shape: both the `first` and `then`
    /// targets are authored `Succession` references sourced at the enclosing action def/usage
    /// declaration, not at each other. Only a simple/qualified name (`Expression::FeatureRef`)
    /// is resolved; any other expression shape (and the bare `start`/`done` pseudo-action
    /// markers, which are ordinary identifier references that legitimately fail to resolve
    /// because no such declaration is synthesized) is out of scope. `first start;`'s absent
    /// `then` and a bare `then Y;` continuation statement (BNF `ThenAction`, a distinct AST
    /// shape referencing an implicit predecessor) are likewise out of scope for this slice.
    Succession,
    /// The authored target of a state def/usage's `entry action <path> ...;` body element
    /// (`EntryAction.action_reference`), resolved through the same `DeclarationDomain::Any`
    /// lexical lookup as `Succession`: the bound action can be any owned action feature, not
    /// just a Type. Sourced at an anonymous `DeclarationKind::EntryActionBinding` feature owned
    /// by the enclosing state declaration, mirroring `Succession`'s nested-declaration shape.
    EntryActionBinding,
    /// Same as `EntryActionBinding`, for a `do action <path> ...;` body element
    /// (`DoAction.action_reference`), sourced at an anonymous `DeclarationKind::DoActionBinding`.
    DoActionBinding,
    /// Same as `EntryActionBinding`, for an `exit action <path> ...;` body element
    /// (`ExitAction.action_reference`), sourced at an anonymous
    /// `DeclarationKind::ExitActionBinding`.
    ExitActionBinding,
    /// The authored target of a state def/usage's `then <target>;` initial-state body element
    /// (`ThenStmt.state_reference`, BNF's bare initial-state marker, distinct from a full
    /// `transition ... then ...;` construct), resolved through the same `DeclarationDomain::Any`
    /// lexical lookup as `Succession`. Sourced at an anonymous `DeclarationKind::InitialState`
    /// feature owned by the enclosing state declaration.
    InitialState,
    /// A feature-reference leaf (`Expression::FeatureRef`/`Expression::FeatureChainRef`) found
    /// while walking a supported constraint/calc boolean-comparison expression tree (slice 1 of
    /// the constraint/calc expression fact family; see `lower_constraint_expression_operand`),
    /// resolved through the same `DeclarationDomain::Any` lexical lookup fixed point as
    /// `ConnectorEnd`/`Succession`: an expression operand can reference any owned feature, not
    /// just a Type. Sourced directly at the enclosing `constraint`/`calc` declaration (unlike
    /// `Succession`, no anonymous nested-declaration scope shift is needed here, since a
    /// constraint/calc's expression operands are looked up in the constraint/calc's own
    /// enclosing scope, not in a nested sibling scope). Evaluation of the expression (computing
    /// an actual truth value) is explicitly out of scope for this slice; only the operand
    /// references themselves are resolved.
    ExpressionOperand,
    /// The authored `source` operand of a `transition ...;` body element (BNF `Transition.
    /// source`), resolved through the same `DeclarationDomain::Any` lexical lookup as
    /// `Succession`: a transition end can reference any owned sibling state feature, not just a
    /// Type. Sourced at an anonymous `DeclarationKind::Transition` feature owned by the
    /// enclosing state declaration, mirroring `Succession`'s nested-declaration shape. Only a
    /// simple/qualified name (`Expression::FeatureRef`) is resolved, exactly like
    /// `lower_succession_end`.
    TransitionSource,
    /// The authored `target` operand of a `transition ...;` body element (BNF `Transition.
    /// target`), same shape and scope as `TransitionSource`.
    TransitionTarget,
    /// The authored shorthand `accept` trigger of a `transition ...;` body element
    /// (`TransitionAccept::Shorthand`'s expression, when it is a simple/qualified name), resolved
    /// through the same `DeclarationDomain::Any` lexical lookup as `TransitionSource`. The typed
    /// `TransitionAccept::Payload` form (an inline declared parameter, not a reference) and the
    /// `TransitionAccept::TimeTrigger` form are out of scope.
    TransitionTrigger,
    /// The authored `do` effect target of a `transition ...;` body element -- either
    /// `TransitionEffect::Perform`'s typed `type_name` (a structured `QualifiedReferenceId`,
    /// resolved the same way `EntryActionBinding` resolves `EntryAction.action_reference`) or a
    /// simple/qualified-name `TransitionEffect::Expression` (resolved the same way
    /// `TransitionSource` resolves `Transition.source`). The richer `Accept`/`Send`/`Assign`
    /// effect shapes are out of scope.
    TransitionEffect,
    /// The authored target metadata definition of an `@Name{...}`/`@Name;` metadata annotation
    /// (BNF MetadataUsage's `@`-prefixed body-element form, `ast::MetadataAnnotation`) applied to
    /// the element that owns it (picks up the annotation-application slice explicitly deferred by
    /// `1b93b225`, which lowered `metadata def`/`metadata` declarations but left `@Name{...}`
    /// application unwired). Resolved through the same Subclassification/FeatureTyping
    /// `DeclarationDomain::Type` lexical lookup fixed point as `FeatureTyping`/`Subclassification`
    /// (a metadata annotation's target must be a type, specifically a metadata def, exactly like
    /// `metadata m : Safety;`'s own `FeatureTyping`), but kept a distinct `ReferenceKind` so the
    /// annotation relationship never collapses into ordinary typing/specialization in query
    /// output. Sourced directly at the declaration `MetadataAnnotation` decorates (no anonymous
    /// nested-declaration scope shift is needed -- unlike `Succession`/`Transition`, the
    /// annotation is not itself a feature, just a fact about its owner). The `about` clause
    /// (explicit annotation targets other than the owner) and the annotation body's nested
    /// feature-value overrides (`isMandatory = true;`) are deliberately out of scope for this
    /// slice: `about` needs its own multi-target fact shape, and the body's overrides need
    /// value-assignment machinery this repository has not built yet (see `lower_attribute_body`
    /// scope notes) -- only the annotation-target reference itself is resolved here.
    MetadataAnnotation,
    /// The metadata-def operand of an `@Name` metadata-classification test (`Expression::
    /// Classification`'s `metaclass`) found while walking a package-level `filter <expr>;`
    /// statement's condition (BNF `ElementFilterMember`, `ast::FilterMember`, distinct from the
    /// bracketed `import X::** [ ... ]` `FilterPackageMember` form that produces `FilterImport`).
    /// `@Safety` tests whether an imported member carries the `Safety` metadata annotation, so
    /// `Safety` names a type -- specifically a metadata def -- exactly like `MetadataAnnotation`'s
    /// own `@Name{...}` target, and is resolved through the same Subclassification/FeatureTyping
    /// `DeclarationDomain::Type` lexical lookup fixed point, kept as its own `ReferenceKind` so a
    /// filter's classification test never collapses into an ordinary `@Name{...}` annotation
    /// application in query output. Sourced directly at the enclosing package declaration (the
    /// filter statement's owner), mirroring `ExpressionOperand`'s "no anonymous nested-declaration
    /// scope shift" shape. See `lower_filter_expression`.
    FilterMetadataTest,
    /// The authored `source` operand of a `satisfy <requirement> by <element>;` body element's
    /// bare shorthand form (`Satisfy.source`, BNF `Satisfy` minus its optional
    /// `InlineSatisfyRequirement` prefix), resolved through the same `DeclarationDomain::Any`
    /// lexical lookup as `Succession`/`TransitionSource`: the satisfied requirement can be any
    /// owned feature, not just a Type. Sourced at an anonymous `DeclarationKind::Satisfy` feature
    /// owned by the enclosing package/part/occurrence declaration, mirroring `Transition`'s
    /// nested-declaration shape. Only a simple/qualified name (`Expression::FeatureRef`) is
    /// resolved; a dotted feature-chain (`Expression::MemberAccess`, e.g.
    /// `satisfy r by vehicle.engine;`'s target) or any other expression shape is out of scope, as
    /// is the fuller `satisfy requirement <name> : <Type> by <expr>;` form
    /// (`Satisfy.inline_requirement`, which introduces a new anonymous requirement usage inline
    /// rather than referencing an existing one) and the braced satisfy body's nested constraint
    /// members (`Satisfy.body_elements`).
    SatisfySource,
    /// The authored `target` operand (the `by <element>` clause) of a `satisfy <requirement> by
    /// <element>;` body element's bare shorthand form (`Satisfy.target`), same shape and scope as
    /// `SatisfySource`.
    SatisfyTarget,
    /// The authored viewpoint reference of a view-body `satisfy <viewpoint>;` statement
    /// (`SatisfyViewMember.viewpoint_ref`, BNF `ViewBodyElement::Satisfy`) -- a genuinely distinct
    /// AST shape from `Satisfy`/`SatisfySource`/`SatisfyTarget`: a single `QualifiedReferenceId`
    /// naming a viewpoint (not a requirement/satisfying-element pair), so it resolves through the
    /// same Subclassification/FeatureTyping `DeclarationDomain::Type` lexical lookup fixed point
    /// as `FeatureTyping` rather than `DeclarationDomain::Any`. Sourced directly at the enclosing
    /// `view` usage declaration (no anonymous nested-declaration scope shift is needed, since the
    /// viewpoint reference is not itself paired with a second operand the way `Satisfy` is).
    SatisfyViewpoint,
    /// The authored `source` operand of an `allocate <source> to <target>;` body element (BNF
    /// `Allocate`, `ast::Allocate.source`) -- the shorthand allocation *statement* form, distinct
    /// from `AllocationDefinition`/`AllocationUsage`'s declaration-side `ConnectorEnd` machinery
    /// (an `allocate` statement asserts a relationship between two already-declared elements,
    /// it does not introduce ends of a new allocation usage). Resolved through the same
    /// `DeclarationDomain::Any` lexical lookup as `SatisfySource`: the allocated source can be
    /// any owned feature, not just a Type. Sourced at an anonymous `DeclarationKind::Allocate`
    /// feature owned by the enclosing part def/part usage/occurrence declaration, mirroring
    /// `Satisfy`'s nested-declaration shape. Only a simple/qualified name
    /// (`Expression::FeatureRef`) is resolved; a dotted feature-chain
    /// (`Expression::MemberAccess`) or any other expression shape is out of scope.
    AllocateSource,
    /// The authored `target` operand (the `to <target>` clause) of an `allocate <source> to
    /// <target>;` body element (`Allocate.target`), same shape and scope as `AllocateSource`.
    AllocateTarget,
    /// The authored target of a `variant <name>;` member (`VariantUsage.reference`, BNF
    /// `VariantUsageElement`'s untyped reference form) inside a `variation part`/`variation part
    /// def` body, resolved through the same `DeclarationDomain::Any` lexical lookup as
    /// `Succession`/`SatisfySource`: the referenced variant can be any owned sibling feature, not
    /// just a Type (e.g. `part manualTransmission;` declared as a sibling of the enclosing
    /// `vehicleFamily` part, referenced from `variant manualTransmission;` nested inside
    /// `variation part transmission { ... }`). Sourced directly at the enclosing `variation`
    /// declaration itself (no anonymous nested-declaration scope shift, unlike `Succession`/
    /// `Satisfy`), since each `variant` member carries only a single operand -- mirroring
    /// `SatisfyViewpoint`'s single-operand shape rather than `Succession`'s paired-ends shape.
    /// Multiple `variant` members owned by the same variation declaration become multiple
    /// `Variant` references from that one source, distinguished by ordinal like any other
    /// multi-target reference family (e.g. `Subclassification`'s multiple `:>` targets). The typed
    /// inline form (`variant part name : Type { ... }`, `VariantUsage.typed`) introduces a new
    /// usage rather than referencing an existing one -- out of scope, like `Satisfy`'s
    /// `inline_requirement` form -- and left as an explicit unsupported-member diagnostic; so is
    /// any optional nested body on the untyped reference form (`VariantUsage.body`).
    Variant,
    /// The authored `target` of an `include <includedUseCase>;` body element inside a `use case
    /// def`/`use case` usage body (BNF `UseCaseDefBodyElement::IncludeUseCase`/
    /// `ThenIncludeUseCase`, `ast::IncludeUseCase.target`) -- the referenced use case is an
    /// ordinary owned feature (a use case usage), not necessarily a type, so it resolves through
    /// the same `DeclarationDomain::Any` lexical lookup fixed point as `Succession`/
    /// `SatisfySource` rather than the Subclassification/FeatureTyping `Type` domain. Sourced
    /// directly at the enclosing use case declaration (no anonymous nested-declaration scope
    /// shift), mirroring `SatisfyViewpoint`/`Variant`'s single-operand shape rather than
    /// `Succession`'s paired-ends shape, since `include` carries only one target reference.
    /// Optional multiplicity (`IncludeUseCase.multiplicity`) and a nested body
    /// (`IncludeUseCase.body`, always `Semicolon` in practice) are out of scope for this slice.
    IncludeUseCase,
    /// The authored `source` operand (`left`) of a `bind <source> = <target>;` body element (BNF
    /// `Bind`, `ast::Bind.left`) or a package-level `binding` statement (BNF
    /// `BindingConnectorUsage`, `ast::BindingConnectorUsage.left`) -- both the shorthand
    /// binding-connector *statement* form, asserting a relationship between two already-declared
    /// elements without introducing a new named binding-connector usage. Resolved through the
    /// same `DeclarationDomain::Any` lexical lookup as `AllocateSource`: the bound source can be
    /// any owned feature, not just a Type. Sourced at an anonymous `DeclarationKind::Bind` feature
    /// owned by the enclosing declaration, mirroring `Allocate`'s nested-declaration shape.
    /// `Bind.left` is a structured `Expression`, resolved only when it is a simple/qualified name
    /// (`Expression::FeatureRef`) exactly like `AllocateSource`; `BindingConnectorUsage.left` is
    /// already a `QualifiedReferenceId`, resolved directly like `AliasBinding`. Left/right
    /// multiplicities, the optional `binding <name>`/`: Type` prefix on either AST shape, and
    /// `Bind`'s optional braced body (`Bind.body_elements`) are out of scope -- only the two
    /// operand references themselves are resolved.
    BindSource,
    /// The authored `target` operand (`right`) of a `bind <source> = <target>;` body element or a
    /// package-level `binding` statement (`Bind.right` / `BindingConnectorUsage.right`), same
    /// shape and scope as `BindSource`.
    BindTarget,
    /// A dotted feature-chain access (`Expression::MemberAccess`, e.g. `t.bead`, `f.a`, chained
    /// `a.b.c`), found wherever `ConnectorEnd`/`Succession`/`TransitionSource`/`TransitionTarget`/
    /// `TransitionTrigger`/`TransitionEffect`/`SatisfySource`/`SatisfyTarget`/`AllocateSource`/
    /// `AllocateTarget`/`BindSource`/`BindTarget`/`ExpressionOperand` previously fell through to
    /// an unsupported-member diagnostic on this exact expression shape. Resolved as a two-step
    /// chase through the same `DeclarationDomain::Any` lexical lookup fixed point every other
    /// operand kind uses for its own first (root) segment, continued through each subsequent
    /// dotted segment by looking the segment up as a member OWNED (directly or through
    /// inheritance) by the *type* of the previously resolved segment -- never as a member of the
    /// previous segment's own declaration -- reusing the ancestor-closure/usage-typing-extended
    /// `inherited_names` index built for `Subsetting`/`Redefinition` (see
    /// `extend_inherited_names_with_usage_typing` in resolver.rs). If the root segment fails to
    /// resolve to exactly one declaration, or any subsequent segment is not found on the current
    /// segment's resolved type, the whole chain publishes an explicit `Unresolved`/`Ambiguous`
    /// outcome -- it never fabricates a partial result. This unifies every deferred dotted-path
    /// call site under one `ReferenceKind` and one resolution algorithm rather than growing a
    /// distinct kind per call site, at the cost of the diagnostics/query surface reporting these
    /// specific references as `MemberAccessOperand` rather than under their originating relation
    /// (e.g. a dotted connector end is `MemberAccessOperand`, not `ConnectorEnd`); each call site's
    /// own doc comment cross-references this trade-off.
    MemberAccessOperand,
    /// The callee of an `Expression::Invocation` (e.g. `sum` in `sum(partMasses)`) or the
    /// `type_name` of an `Expression::Constructor` (e.g. `PusherOutput` in `new
    /// PusherOutput(pusherForce)`) -- both name the function/operation/type being invoked with a
    /// parenthesized argument list, so they share this one `ReferenceKind` rather than growing a
    /// separate kind per AST shape. Resolved through the same `DeclarationDomain::Any` lexical
    /// lookup fixed point as `ExpressionOperand`/`ConnectorEnd`: a callee can name any owned
    /// feature (a `calc`/function) or a type (a constructor), not just a `Type`. Sourced directly
    /// at the enclosing declaration whose expression contains the invocation (no anonymous
    /// nested-declaration scope shift), mirroring `ExpressionOperand`'s shape. Each argument
    /// expression is recursively resolved through the same operand-resolution dispatch (`lower_
    /// constraint_expression`/`lower_calc_expression`/`lower_filter_expression`/`lower_satisfy_
    /// operand`) the invocation itself was reached through, so an argument can itself be a
    /// literal, `FeatureRef`, dotted `MemberAccess` chain, nested `BinaryOp`, or nested
    /// `Invocation`/`Constructor` -- each pushing whatever `ReferenceKind` its own shape resolves
    /// to (typically `ExpressionOperand`/`MemberAccessOperand`/another `InvocationCallee`).
    /// Evaluating the invocation itself (computing a function's result or a constructed value) is
    /// explicitly out of scope: see `EvalNode::Invocation`, which always folds to
    /// `EvaluatedValue::NonConstant`. A callee that is not a simple/qualified name, dotted chain,
    /// or (for `Constructor`) a type name -- e.g. `(a + b)(x)`, an invocation whose callee is
    /// itself an invocation result -- is out of scope and left unresolved by `lower_invocation_
    /// callee`.
    InvocationCallee,
    /// The required `decide` operand of a `decide <expr>;` control node (BNF `DecisionStmt.
    /// decide`), resolved through the same `DeclarationDomain::Any` lexical lookup as
    /// `Succession`/`TransitionSource`: the decision input can be any owned sibling action
    /// feature, not just a Type. Sourced at an anonymous `DeclarationKind::Decide` feature owned
    /// by the enclosing action def/usage declaration, mirroring `Transition`'s nested-declaration
    /// shape. Only a simple/qualified name (`Expression::FeatureRef`) or a dotted feature-chain
    /// (`Expression::MemberAccess`, via `MemberAccessOperand`) is resolved.
    DecisionInput,
    /// The required `merge` operand of a `merge <expr>;` control node (BNF `MergeStmt.merge`),
    /// same shape and scope as `DecisionInput`, sourced at an anonymous
    /// `DeclarationKind::Merge` feature.
    MergeInput,
    /// The required `fork` operand of a `fork <expr>;` control node (BNF `ForkStmt.fork`), same
    /// shape and scope as `DecisionInput`, sourced at an anonymous `DeclarationKind::Fork`
    /// feature.
    ForkInput,
    /// The required `join` operand of a `join <expr>;` control node (BNF `JoinStmt.join`), same
    /// shape and scope as `DecisionInput`, sourced at an anonymous `DeclarationKind::Join`
    /// feature.
    JoinInput,
    /// The target of a bare `then <target>;` continuation statement (BNF `ThenAction`,
    /// `ThenTarget::Feature`) found in an action def/usage body -- a reference to an
    /// already-declared sibling control-flow node (an action, decide/merge/fork/join node, or the
    /// `done` pseudo-action marker), distinct from `Succession`'s paired `first`/`then` ends since
    /// a `then <target>;` on its own references an implicit predecessor rather than declaring both
    /// ends. Resolved through the same `DeclarationDomain::Any` lexical lookup as `Succession`,
    /// sourced at an anonymous `DeclarationKind::ThenContinuation` feature owned by the enclosing
    /// action def/usage declaration (mirroring `Succession`'s own nested-declaration scope shift
    /// -- sourcing directly at the action itself would search the action's own siblings rather
    /// than its children, where the referenced sibling control-flow nodes actually live) via the
    /// same `lower_succession_end` dispatch every other paired-operand kind uses. The `done`
    /// marker itself is an ordinary identifier reference that legitimately fails to resolve
    /// because no such declaration is synthesized, exactly like `Succession`'s `start`/`done`
    /// scope note.
    ThenTarget,
    /// The authored `via <port>` clause found on an `accept`/`send` trigger or standalone
    /// control-node statement (`TransitionAccept::{Shorthand,Payload,TimeTrigger}`'s shared `via`
    /// operand, or `ast::ActionUsage.via`), resolved through the same `DeclarationDomain::Any`
    /// lexical lookup as `TransitionTrigger`/`ExpressionOperand`: the targeted port/receiver can
    /// be any owned feature, not just a Type. Sourced directly at the declaration the accept/send
    /// belongs to (no anonymous nested-declaration scope shift, mirroring `ExpressionOperand`).
    AcceptVia,
    /// The authored `to <target>` clause of a standalone `send`-suffixed action usage
    /// (`ast::ActionUsage.to`, e.g. `action snd2 send via this to aa.target;`), same shape and
    /// scope as `AcceptVia`. The `then send <expr> to <target>;` shorthand form is a distinct AST
    /// shape (`ThenTarget` has no `Send` variant at all -- see planning/UPSTREAM_PARSER_GAPS.md) and is not
    /// covered by this kind.
    SendTarget,
    /// The optional typed-payload type reference of a `TransitionAccept::Payload` trigger
    /// (`PayloadClause.type_name`, e.g. `accept sig : SomeSignal`), resolved through the same
    /// Subclassification/FeatureTyping `DeclarationDomain::Type` lexical lookup fixed point as
    /// `FeatureTyping`: the payload names a type. Sourced directly at the declaration the accept
    /// belongs to, same scope as `AcceptVia`.
    AcceptPayloadType,
    /// The optional target of a `terminate <target>;` body element (BNF `TerminateStmt`,
    /// `ast::TerminateStmt.target`), resolved through the same `DeclarationDomain::Any` lexical
    /// lookup as `Succession`/`SatisfySource`: the terminated node/action can be any owned
    /// feature, not just a Type. Sourced directly at the enclosing action def/usage declaration
    /// (no anonymous nested-declaration scope shift is needed -- unlike `Succession`, the target
    /// is looked up in the terminate statement's own enclosing scope, where sibling action names
    /// like `terminate c1;`'s `c1` are actually declared). The bare `terminate;` form (no target)
    /// has nothing to resolve.
    TerminateTarget,
    /// The authored `source` operand (`from`) of a standalone `flow <source> to <target>;`
    /// statement (BNF `FlowUsage.from`), resolved through the same `DeclarationDomain::Any`
    /// lexical lookup as `AllocateSource`/`BindSource`: the flow source can be any owned feature,
    /// including a dotted feature-chain (`aa.target`), not just a Type. Sourced at an anonymous
    /// `DeclarationKind::Flow` feature owned by the enclosing action def/usage declaration,
    /// mirroring `Allocate`/`Bind`'s nested-declaration shape.
    FlowSource,
    /// The authored `target` operand (`to`) of a standalone `flow <source> to <target>;`
    /// statement (`FlowUsage.to`), same shape and scope as `FlowSource`.
    FlowTarget,
    /// The `type_name` of an `Expression::TypeCheck` (`expr istype Type`/`expr hastype Type`/
    /// `expr as Type`, e.g. `x istype ScalarValues::Integer`), which names a type exactly like
    /// `AcceptPayloadType`/`FilterMetadataTest`, so it joins the same Subclassification/
    /// FeatureTyping `DeclarationDomain::Type` lexical lookup fixed point rather than a separate
    /// one, kept as a distinct `ReferenceKind` purely so a type-check test stays distinct from
    /// ordinary typing/annotation relationships in query output. The operand (`x`, optional in the
    /// parser's `TypeCheck` shape though never actually absent for `istype`/`hastype`/`as`) recurses
    /// back into the same operand-resolution dispatch (`lower_constraint_expression`/
    /// `lower_calc_expression`/`lower_filter_expression`) the type-check itself was reached
    /// through, pushing whatever `ReferenceKind` its own shape resolves to (typically
    /// `ExpressionOperand`/`MemberAccessOperand`). Evaluating the test itself (computing a concrete
    /// boolean from runtime classification) is out of scope: `istype`/`hastype` genuinely cannot be
    /// evaluated to a constant without runtime type information this static resolver does not have,
    /// so `EvalNode::Invocation` (reused rather than adding a distinct variant, exactly like
    /// `Expression::Tuple`) always folds to `EvaluatedValue::NonConstant`.
    TypeCheckTarget,
    /// The `metaclass` of an `Expression::MetaCast` (KerML reflective meta cast, `expr meta
    /// Metaclass`, e.g. `Atom meta KerML::Classifier`), which names a type -- specifically a
    /// metaclass -- exactly like `TypeCheckTarget`/`AcceptPayloadType`, so it joins the same
    /// Subclassification/FeatureTyping `DeclarationDomain::Type` lexical lookup fixed point rather
    /// than a separate one, kept as a distinct `ReferenceKind` purely so a meta-cast target stays
    /// distinct from ordinary typing/annotation relationships in query output. The `base` operand
    /// (`Atom`) recurses back into the same operand-resolution dispatch (`lower_constraint_expression`/
    /// `lower_calc_expression`/`lower_filter_expression`) the meta cast itself was reached through,
    /// pushing whatever `ReferenceKind` its own shape resolves to (typically `ExpressionOperand`/
    /// `MemberAccessOperand`). Evaluating the cast itself (computing the reflective metaobject) is
    /// out of scope -- it denotes a metaclass relationship, not a computable scalar value -- so
    /// `EvalNode::Invocation` (reused rather than adding a distinct variant, exactly like
    /// `Expression::TypeCheck`/`Expression::Tuple`) always folds to `EvaluatedValue::NonConstant`.
    MetaCastTarget,
    /// The `target` concern reference of a `stakeholder` member found in a requirement/viewpoint
    /// def body (`StakeholderMember.target`, BNF `StakeholderMember`'s bare `stakeholder Concern;`
    /// reference form, `is_redefinition == false`), resolved through the same `DeclarationDomain::
    /// Any` lexical lookup as `SatisfySource`/`Variant`: the referenced concern can be a `concern`
    /// usage (a feature, not a Type) as easily as a `concern def`, so it is not restricted to the
    /// Subclassification/FeatureTyping `Type` domain. Sourced at the `DeclarationKind::
    /// StakeholderUsage` declaration itself (no anonymous nested-declaration scope shift, mirroring
    /// `SatisfyViewpoint`/`Variant`'s single-operand shape), since a `stakeholder` member carries
    /// only one operand. The `:>>` redefinition spelling (`is_redefinition == true`) reuses the
    /// existing generic `ReferenceKind::Redefinition` instead of this kind.
    StakeholderTarget,
    /// The `target` concern reference of a viewpoint `purpose` member (`PurposeMember.target`, BNF
    /// `PurposeMember`), same shape and scope as `StakeholderTarget`: a concern reference resolved
    /// through `DeclarationDomain::Any`, sourced directly at the enclosing requirement/viewpoint
    /// declaration (no anonymous nested-declaration scope shift -- a `purpose` member carries only
    /// one operand and introduces no name of its own, mirroring `SatisfyViewpoint`).
    PurposeTarget,
    /// The shorthand `target` of a requirement/objective-body `verify <requirement>;` body element
    /// (`VerifyRequirementMember.target`, BNF `VerifyRequirementMember`'s bare shorthand form,
    /// `explicit_requirement_keyword == false`), resolved through the same `DeclarationDomain::Any`
    /// lexical lookup as `SatisfySource`/`StakeholderTarget`: the verified requirement can be any
    /// owned feature, not just a Type. Sourced at an anonymous `DeclarationKind::VerifyRequirement`
    /// feature owned by the enclosing declaration, mirroring `Satisfy`'s nested-declaration shape.
    /// The `:>>` redefinition spelling (`VerifyRequirementMember.redefines`) reuses the existing
    /// generic `ReferenceKind::Redefinition` instead of this kind.
    VerifyRequirementTarget,
    /// The `lhs` target of an `assign <target> := <value>;` reassignment statement (BNF
    /// `AssignStmt.lhs`), resolved through the same `DeclarationDomain::Any` lexical lookup as
    /// `Succession`/`ThenTarget`: the reassigned target can be any owned sibling feature, not just
    /// a Type. Sourced at an anonymous `DeclarationKind::Assign` feature owned by the enclosing
    /// action def/usage declaration, mirroring `Bind`'s nested-declaration shape, through the same
    /// `lower_succession_end` `FeatureRef`/`MemberAccess` dispatch every other paired/single-
    /// operand control-flow reference kind uses.
    AssignTarget,
    /// One `client` operand of a `dependency` relationship declaration (`Dependency.clients`),
    /// resolved through the same `DeclarationDomain::Any` lexical lookup as `SatisfySource`:
    /// each client can be any owned feature/type, not just a Type. Sourced at the anonymous
    /// `DeclarationKind::Dependency` declaration.
    DependencyClient,
    /// One `supplier` operand of a `dependency` relationship declaration
    /// (`Dependency.suppliers`), same shape/scope as `DependencyClient`.
    DependencySupplier,
    /// The `target` operand of a `perform` usage's `in`/`out <target> = <value>;` body element
    /// (BNF `PerformInOutBinding.target`), resolved through the same `DeclarationDomain::Any`
    /// lexical lookup as `BindTarget`: the bound parameter can be any owned feature (typically a
    /// parameter of the invoked action), not just a Type. Sourced at an anonymous
    /// `DeclarationKind::PerformParameterBinding` feature owned by the enclosing `perform`
    /// declaration, mirroring `Bind`'s nested-declaration shape. `target` is already a structured
    /// `QualifiedReferenceId` (not an `Expression`), resolved directly like `AliasBinding`. The
    /// bound `= <value>` expression itself (`PerformInOutBinding.value`) is lowered through the
    /// ordinary `lower_constraint_expression` machinery, exactly like `Assign`'s RHS, which
    /// publishes its own `ExpressionOperand` reference(s) rather than a dedicated kind here.
    PerformParameterTarget,
    /// The optional payload type of an anonymous, unnamed `flow of <payload> from <a> to <b>;`
    /// body element (BNF `FlowUsage.payload`'s `type_name`, `ast::PayloadFeature.type_name`),
    /// resolved through the same Subclassification/FeatureTyping `DeclarationDomain::Type`
    /// lexical lookup fixed point as `AcceptPayloadType`: the payload names a type. Sourced at the
    /// synthesized `DeclarationKind::Flow` declaration `lower_flow_usage` creates, alongside its
    /// `FlowSource`/`FlowTarget` references. The payload's own optional declared name (`of qty :
    /// Payload`) is not a reference target, mirroring `AcceptPayloadType`'s own scope boundary.
    FlowPayloadType,
}

/// The computed or explicit outcome of evaluating one supported constraint/calc expression
/// (slice 2 of the constraint/calc expression fact family; slice 1, `4ca42166`, only resolved
/// operand references and never evaluated anything). Only expressions within slice 1's supported
/// syntactic shapes (literal leaves, a comparison `BinaryOp` of two literals, `Parenthesized`
/// wrapping a supported shape) reach this pass at all -- a shape slice 1 leaves unsupported
/// publishes no evaluation fact, per `classify_constraint_expression`/`classify_calc_expression`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EvaluatedValue {
    /// A genuinely computed constant `bool` result: a literal boolean leaf, or a comparison of
    /// two literal operands.
    Boolean(bool),
    /// A genuinely computed constant integer result: a literal integer leaf.
    Integer(i64),
    /// A genuinely computed constant real result: a literal real leaf.
    Real(f64),
    /// A genuinely computed constant string result: a literal string leaf (`Expression::
    /// LiteralString`). Only equality comparison (`==`/`!=`, see `fold_literal_comparison`) is
    /// folded for strings; `Lt`/`Le`/`Gt`/`Ge` are out of scope (no lexicographic ordering).
    String(String),
    /// A genuinely computed constant quantity-with-unit result: a literal leaf carrying both a
    /// numeric magnitude and an authored unit token, e.g. `0[kg]`, `27316/100[K]`'s numerator
    /// literal `27316[K]`... (`Expression::LiteralWithUnit`, see `quantity_unit_text`). The unit is
    /// stored as the raw authored token text (`kg`, `SI::s`, `m/s^2`), never as a resolved
    /// declaration reference: the parser hands the bracketed text to this layer as an opaque
    /// string (`Expression::Unit(String)`), not a `QualifiedReferenceId` that lexical lookup could
    /// resolve (units may contain operators like `/`/`^`, so they are deliberately not qualified
    /// references upstream either). The magnitude is boxed so it stays exactly the `Boolean`/
    /// `Integer`/`Real`/`String` variant the wrapped literal would have folded to on its own --
    /// this is a widen, not a new numeric type, matching the minimal-but-honest posture of not
    /// fabricating a richer "physical quantity" concept `planning/RESOLUTION_LAYER_DESIGN.md` never
    /// anticipates. `fold_literal_comparison`/`fold_arithmetic`/`fold_unary` do not special-case
    /// this variant: their generic numeric-widening fallback (`as_f64`) does not match it, so any
    /// operation involving a `Quantity` conservatively folds to `NonConstant` rather than silently
    /// comparing/arithmetic-ing across mismatched or unmodeled units.
    Quantity(Box<EvaluatedValue>, String),
    /// Evaluation did not run for this expression. Reserved for a future evaluation-policy gate;
    /// the current pass attempts evaluation for every slice-1-supported expression whenever
    /// resolution itself converges (see `SemanticModelStorage::resolve`), so no fact currently
    /// publishes this variant, but consumers must not assume every supported expression yields a
    /// value.
    #[allow(dead_code)]
    NotEvaluated,
    /// The expression tree references at least one operand
    /// (`ReferenceKind::ExpressionOperand`) that resolution left unresolved, ambiguous,
    /// unsupported, or non-converged. What such an operand would evaluate to is unknown, so the
    /// expression cannot be folded.
    UnresolvedOperand,
    /// The expression is a slice-1-supported syntactic shape, but at least one leaf is a
    /// *resolved* feature reference to a declaration with no known constant value of its own
    /// (no evaluation fact at all, e.g. `attribute x : ScalarValues::Integer;` with no default
    /// value). Constant propagation (slice 3, `see EvalNode`) looks up whether a resolved operand
    /// reference's target itself published a concrete constant; when it did not, the expression
    /// is conservatively not a constant.
    NonConstant,
    /// Constant propagation (slice 3 of the constraint/calc expression fact family) detected a
    /// genuine cross-declaration dependency cycle while resolving what a resolved operand
    /// reference's target evaluates to (declaration A's value depends on B's, which depends on
    /// A's, directly or transitively). Mirrors the house convention for cycle-safe fixed-point
    /// iteration established for specialization-ancestor cycles (`69a897b9`) and alias-binding
    /// cycles (`422e2216`): an explicit typed non-converged outcome, never a fabricated value, an
    /// infinite loop, or a panic.
    NonConverged,
    /// Slice 4 (arithmetic calc-body evaluation): a constant `Div`/`Mod` whose divisor evaluated
    /// to zero (integer `0` or real `0.0`). Rust's `i64`/`i32` division panics on a zero divisor
    /// and `f64` division silently yields `inf`/`NaN`; both are explicitly intercepted before the
    /// arithmetic operation runs (see `fold_arithmetic`) so this typed outcome publishes instead
    /// of a panic or a fabricated "valid" `Real` infinity.
    DivisionByZero,
    /// Slice 4: an arithmetic `BinaryOp` operand folded to `Boolean` where a numeric (Integer/
    /// Real) operand is structurally expected. The grammar makes this effectively unreachable
    /// today (a `Boolean` leaf only arises from a literal or a propagated constant, and nothing
    /// currently propagates a `Boolean` into an arithmetic position from real corpus input), but
    /// `fold_arithmetic` handles it defensively rather than panicking, mirroring
    /// `fold_literal_comparison`'s own defensive `NonConstant` fallback for a mistyped pairing.
    TypeMismatch,
}

/// A construction-time-classified mirror of a supported constraint/calc expression tree, built by
/// `classify_constraint_expression`/`classify_calc_expression` in lockstep with
/// `lower_constraint_expression`/`lower_calc_expression`'s own left-to-right traversal: each
/// `Operand` leaf's ordinal exactly matches the `ordinal` `push_reference` assigns the
/// `ReferenceKind::ExpressionOperand` reference pushed for the same leaf (both walk literal /
/// feature-ref / parenthesized / comparison shapes identically), so `compute_evaluation` (slice 3)
/// can re-walk this tree at resolution time and pair each `Operand(n)` with the n-th
/// `ExpressionOperand` reference sourced at the same declaration.
#[derive(Debug, Clone, PartialEq)]
enum EvalNode {
    Literal(EvaluatedValue),
    /// The n-th (0-based) `ExpressionOperand` reference sourced at the owning declaration, in
    /// left-to-right expression order.
    Operand(u32),
    Comparison(BinaryOperator, Box<EvalNode>, Box<EvalNode>),
    /// Slice 4 (`classify_calc_node`): an arithmetic `BinaryOp` (`Add`/`Sub`/`Mul`/`Div`/`Mod`)
    /// found in a calc-body expression. Widened by the arithmetic/logical-combinator slice:
    /// `classify_constraint_node` now also builds this variant for an arithmetic sub-expression
    /// nested inside (or alongside) a comparison/logical combinator in a constraint body (e.g.
    /// `chassisMass + engine.mass` as a comparison operand), so a constraint body's `EvalNode` tree
    /// may now contain both `Comparison`/`Logical` and `Arithmetic` nodes, unlike slices 1-3.
    Arithmetic(BinaryOperator, Box<EvalNode>, Box<EvalNode>),
    /// A logical `BinaryOp` (`and`/`or`/`xor`/`implies`, `is_logical_operator`) combining two
    /// constraint-body sub-expressions, each of which folds to a `Boolean` (typically a
    /// `Comparison`). Only `classify_constraint_node` builds this variant -- calc bodies stay
    /// comparison/logical-free, unchanged.
    Logical(BinaryOperator, Box<EvalNode>, Box<EvalNode>),
    /// An `Expression::Invocation`/`Expression::Constructor` node (reference-resolution slice; see
    /// `ReferenceKind::InvocationCallee`). Carries each argument's own classified `EvalNode` purely
    /// so `ordinal` keeps advancing past every nested `Operand` leaf in exact lockstep with `lower_
    /// constraint_expression`/`lower_calc_expression`'s traversal (an operand lexically following
    /// the invocation in the same expression must still get the right ordinal) -- the argument
    /// values themselves are never read by `fold_eval_node_pending`, which always folds this node
    /// straight to `EvaluatedValue::NonConstant` regardless of its children's folded values,
    /// exactly as evaluating an invocation's function/constructor semantics is out of scope.
    ///
    /// Also reused, unchanged, for `Expression::Tuple` (`(a, b, c)`, e.g.
    /// `quantityPowerFactors = (lengthPF, massPF, durationPF)`): a tuple has no callee to resolve,
    /// but is otherwise identical in shape -- a plain `Vec<Node<Expression>>` of elements, each
    /// recursed back into the same classifier/lowerer, with the tuple as a whole never evaluated as
    /// a single scalar (folds to `NonConstant`, like an invocation). Reusing this variant rather
    /// than adding a distinct `Tuple` one keeps the "one EvalNode shape per fold behavior" property:
    /// nothing downstream (`fold_eval_node_pending`, `eval_node_is_pure_literal`) needs to
    /// distinguish "called with args" from "grouped as a sequence" since both fold identically.
    Invocation(Vec<EvalNode>),
    /// A unary prefix `UnaryOp` (`-x` negation or `not x` logical negation, see
    /// `UnaryOperator::Minus`/`UnaryOperator::Not`) wrapping a single classified operand, built by
    /// both `classify_constraint_node` and `classify_calc_node` using the exact same recursive
    /// classification call already used for every other single-child shape (`Parenthesized`).
    /// `Plus`/`BitNot`/`Other` unary operators are deliberately out of scope (unsupported), mirroring
    /// `is_arithmetic_operator`'s narrow-slice precedent: only `-`/`not` are folded (`fold_unary`).
    Unary(UnaryOperator, Box<EvalNode>),
}

/// The classification `classify_constraint_expression`/`classify_calc_expression` assign to one
/// expression node before resolution's fixed point runs. `Literal` expressions need no resolved
/// state at all (their value is already known); `HasOperand` expressions carry an `EvalNode` tree
/// that `compute_evaluation` re-folds once operand references are resolved (and, per slice 3,
/// once each operand's own target declaration's constant value -- if any -- is known); the tree
/// settles to `UnresolvedOperand`/`NonConstant`/`NonConverged` or a genuine folded constant.
/// `Unsupported` expressions (any shape `lower_constraint_expression`/`lower_calc_expression` does
/// not recognize) publish no evaluation fact at all.
#[derive(Debug, Clone, PartialEq)]
enum ExpressionEvalShape {
    /// The authored expression is itself a literal value -- nothing was folded.
    Literal(EvaluatedValue),
    /// A tree with no operand leaf, folded at construction (`2 + 3`). Kept apart from `Literal`
    /// because the published contract distinguishes a value that was *written* from one that was
    /// *computed*, and only the root node tells them apart.
    ConstantFolded(EvaluatedValue),
    HasOperand(EvalNode),
    Unsupported,
}

/// Folds a comparison of two already-literal operands to a `Boolean` outcome. Integer/Real
/// operands compare numerically (mixed Integer/Real is widened to `f64`); Boolean operands
/// support only `Eq`/`Ne`, mirroring `is_comparison_operator`'s scope. Any other literal-type
/// pairing (e.g. comparing a Boolean to an Integer) is conservatively `NonConstant`: SysML typing
/// would reject it, but this slice does not perform type checking, so it never fabricates a
/// truth value for a shape it cannot type.
fn fold_literal_comparison(
    op: BinaryOperator,
    left: EvaluatedValue,
    right: EvaluatedValue,
) -> EvaluatedValue {
    fn as_f64(value: EvaluatedValue) -> Option<f64> {
        match value {
            EvaluatedValue::Integer(value) => Some(value as f64),
            EvaluatedValue::Real(value) => Some(value),
            _ => None,
        }
    }
    let result = match (left, right) {
        // KerML's strict-identity `===`/`!==` (`StrictEq`/`StrictNe`) fold identically to
        // `==`/`!=` for the already-literal-scalar operands this fold ever sees -- there is no
        // separate "same object identity, different value" case to distinguish once both sides
        // are already-folded constants, so treating them as ordinary equality/inequality is exact,
        // not an approximation.
        (EvaluatedValue::Boolean(left), EvaluatedValue::Boolean(right)) => match op {
            BinaryOperator::Eq | BinaryOperator::StrictEq => Some(left == right),
            BinaryOperator::Ne | BinaryOperator::StrictNe => Some(left != right),
            _ => None,
        },
        (EvaluatedValue::String(left), EvaluatedValue::String(right)) => match op {
            BinaryOperator::Eq | BinaryOperator::StrictEq => Some(left == right),
            BinaryOperator::Ne | BinaryOperator::StrictNe => Some(left != right),
            _ => None,
        },
        (left, right) => match (as_f64(left), as_f64(right)) {
            (Some(left), Some(right)) => Some(match op {
                BinaryOperator::Eq | BinaryOperator::StrictEq => left == right,
                BinaryOperator::Ne | BinaryOperator::StrictNe => left != right,
                BinaryOperator::Lt => left < right,
                BinaryOperator::Le => left <= right,
                BinaryOperator::Gt => left > right,
                BinaryOperator::Ge => left >= right,
                _ => return EvaluatedValue::NonConstant,
            }),
            _ => None,
        },
    };
    result.map_or(EvaluatedValue::NonConstant, EvaluatedValue::Boolean)
}

/// Folds an arithmetic operation of two already-constant operands (slice 4). Numeric promotion
/// rule: `Integer op Integer` stays `Integer` via checked arithmetic (overflow is reported as
/// `NonConstant` rather than panicking or silently wrapping -- the same conservative "cannot fold
/// this" posture the house convention already uses for a mistyped comparison pairing); any pairing
/// involving a `Real` operand (`Real op Real` or mixed `Integer op Real`/`Real op Integer`)
/// promotes the `Integer` side to `f64` and produces a `Real`, matching Rust's/IEEE754's usual
/// widen-to-float promotion and the same `as f64` widening `fold_literal_comparison` already uses
/// for mixed-numeric comparisons -- i.e. no separate reference implementation was consulted, since
/// this repository already committed to that promotion rule for comparisons and arithmetic should
/// not diverge from it. `Div`/`Mod` by a constant zero divisor (integer or real) is intercepted
/// explicitly *before* the operation runs and reports `DivisionByZero`, never a panic (`i64 / 0`
/// panics) or a silently "valid" `f64` infinity/NaN. A `Boolean` operand in an arithmetic position
/// reports `TypeMismatch` (defensive; unreachable via today's supported shapes). `NonConverged`/
/// `UnresolvedOperand`/`NonConstant` operands propagate through unchanged, in the same priority
/// order `fold_literal_comparison`/`fold_eval_node_pending`'s comparison arm already uses.
fn fold_arithmetic(
    op: BinaryOperator,
    left: EvaluatedValue,
    right: EvaluatedValue,
) -> EvaluatedValue {
    match (left, right) {
        (EvaluatedValue::NonConverged, _) | (_, EvaluatedValue::NonConverged) => {
            EvaluatedValue::NonConverged
        }
        (EvaluatedValue::UnresolvedOperand, _) | (_, EvaluatedValue::UnresolvedOperand) => {
            EvaluatedValue::UnresolvedOperand
        }
        (EvaluatedValue::NonConstant, _) | (_, EvaluatedValue::NonConstant) => {
            EvaluatedValue::NonConstant
        }
        (EvaluatedValue::Boolean(_), _) | (_, EvaluatedValue::Boolean(_)) => {
            EvaluatedValue::TypeMismatch
        }
        (EvaluatedValue::Integer(left), EvaluatedValue::Integer(right))
            if matches!(op, BinaryOperator::Pow | BinaryOperator::Exp) =>
        {
            // A negative integer exponent produces a fractional result, so promote to `Real`
            // via `powf` (e.g. `2 ^ -1` folds to `Real(0.5)`) rather than folding to `NonConstant`.
            // A non-negative exponent that does not fit `u32` (checked_pow's exponent type) cannot
            // be computed as an exact integer either way, so it conservatively falls to
            // `NonConstant` rather than silently promoting to a lossy `Real`.
            if right < 0 {
                EvaluatedValue::Real((left as f64).powf(right as f64))
            } else {
                match u32::try_from(right) {
                    Ok(exponent) => left
                        .checked_pow(exponent)
                        .map_or(EvaluatedValue::NonConstant, EvaluatedValue::Integer),
                    Err(_) => EvaluatedValue::NonConstant,
                }
            }
        }
        (EvaluatedValue::Integer(left), EvaluatedValue::Integer(right)) => {
            let result = match op {
                BinaryOperator::Add => left.checked_add(right),
                BinaryOperator::Sub => left.checked_sub(right),
                BinaryOperator::Mul => left.checked_mul(right),
                BinaryOperator::Div => {
                    if right == 0 {
                        return EvaluatedValue::DivisionByZero;
                    }
                    left.checked_div(right)
                }
                BinaryOperator::Mod => {
                    if right == 0 {
                        return EvaluatedValue::DivisionByZero;
                    }
                    left.checked_rem(right)
                }
                _ => return EvaluatedValue::NonConstant,
            };
            result.map_or(EvaluatedValue::NonConstant, EvaluatedValue::Integer)
        }
        (left, right) => {
            fn as_f64(value: EvaluatedValue) -> Option<f64> {
                match value {
                    EvaluatedValue::Integer(value) => Some(value as f64),
                    EvaluatedValue::Real(value) => Some(value),
                    _ => None,
                }
            }
            let (Some(left), Some(right)) = (as_f64(left), as_f64(right)) else {
                return EvaluatedValue::NonConstant;
            };
            match op {
                BinaryOperator::Add => EvaluatedValue::Real(left + right),
                BinaryOperator::Sub => EvaluatedValue::Real(left - right),
                BinaryOperator::Mul => EvaluatedValue::Real(left * right),
                BinaryOperator::Div => {
                    if right == 0.0 {
                        EvaluatedValue::DivisionByZero
                    } else {
                        EvaluatedValue::Real(left / right)
                    }
                }
                BinaryOperator::Mod => {
                    if right == 0.0 {
                        EvaluatedValue::DivisionByZero
                    } else {
                        EvaluatedValue::Real(left % right)
                    }
                }
                BinaryOperator::Pow | BinaryOperator::Exp => EvaluatedValue::Real(left.powf(right)),
                _ => EvaluatedValue::NonConstant,
            }
        }
    }
}

/// Folds an `and`/`or` combination of two already-folded operands, mirroring
/// `fold_literal_comparison`'s priority order (`NonConverged` > `UnresolvedOperand` >
/// `NonConstant` > a genuine value): only a `Boolean`/`Boolean` pairing produces a result;
/// anything else (e.g. an `Integer` operand, which the grammar should never actually produce here
/// since a logical combinator's operands are themselves boolean comparisons) is conservatively
/// `NonConstant`, the same defensive fallback `fold_literal_comparison` uses for a mistyped
/// pairing.
fn fold_logical(op: BinaryOperator, left: EvaluatedValue, right: EvaluatedValue) -> EvaluatedValue {
    match (left, right) {
        (EvaluatedValue::NonConverged, _) | (_, EvaluatedValue::NonConverged) => {
            EvaluatedValue::NonConverged
        }
        (EvaluatedValue::UnresolvedOperand, _) | (_, EvaluatedValue::UnresolvedOperand) => {
            EvaluatedValue::UnresolvedOperand
        }
        (EvaluatedValue::NonConstant, _) | (_, EvaluatedValue::NonConstant) => {
            EvaluatedValue::NonConstant
        }
        (EvaluatedValue::Boolean(left), EvaluatedValue::Boolean(right)) => {
            EvaluatedValue::Boolean(match op {
                // `BitAnd`/`BitOr` are KerML's single-`&`/single-`|` spellings of boolean
                // conjunction/disjunction in a constraint/invariant boolean expression (see
                // `is_logical_operator`'s doc comment) -- not bitwise operators here, so they fold
                // identically to `And`/`Or`.
                BinaryOperator::And | BinaryOperator::BitAnd => left && right,
                BinaryOperator::Or | BinaryOperator::BitOr => left || right,
                // `Xor`/`Implies` share `And`/`Or`'s Boolean/Boolean truth-table shape exactly --
                // no new failure state, just a different two-operand boolean combination.
                BinaryOperator::Xor => left != right,
                BinaryOperator::Implies => !left || right,
                _ => return EvaluatedValue::NonConstant,
            })
        }
        _ => EvaluatedValue::NonConstant,
    }
}

/// Folds a unary `-`/`not` operation on an already-folded operand. `Minus` negates a constant
/// `Integer`/`Real` (`Integer` negation uses `checked_neg`, mirroring `fold_arithmetic`'s
/// conservative "cannot fold" `NonConstant` posture for `i64::MIN`'s unrepresentable negation rather
/// than panicking or silently wrapping); `Not` negates a constant `Boolean`. Any other operator/
/// operand-type pairing (unreachable via `classify_constraint_node`/`classify_calc_node`, which only
/// ever build this node for `Minus`/`Not`) conservatively falls to `NonConstant`, mirroring
/// `fold_literal_comparison`/`fold_arithmetic`/`fold_logical`'s own defensive fallback for a
/// mistyped pairing. `NonConverged`/`UnresolvedOperand`/`NonConstant` operands propagate through
/// unchanged, in the same priority order the binary folds already use.
fn fold_unary(op: &UnaryOperator, value: EvaluatedValue) -> EvaluatedValue {
    match value {
        EvaluatedValue::NonConverged => EvaluatedValue::NonConverged,
        EvaluatedValue::UnresolvedOperand => EvaluatedValue::UnresolvedOperand,
        EvaluatedValue::NonConstant => EvaluatedValue::NonConstant,
        EvaluatedValue::Integer(value) => match op {
            UnaryOperator::Minus => value
                .checked_neg()
                .map_or(EvaluatedValue::NonConstant, EvaluatedValue::Integer),
            _ => EvaluatedValue::NonConstant,
        },
        EvaluatedValue::Real(value) => match op {
            UnaryOperator::Minus => EvaluatedValue::Real(-value),
            _ => EvaluatedValue::NonConstant,
        },
        EvaluatedValue::Boolean(value) => match op {
            UnaryOperator::Not => EvaluatedValue::Boolean(!value),
            _ => EvaluatedValue::NonConstant,
        },
        _ => EvaluatedValue::NonConstant,
    }
}

fn literal_expression_value(node: &Expression) -> Option<EvaluatedValue> {
    match node {
        Expression::LiteralBoolean(value) => Some(EvaluatedValue::Boolean(*value)),
        Expression::LiteralInteger(value) => Some(EvaluatedValue::Integer(*value)),
        Expression::LiteralReal(text) => text.parse::<f64>().ok().map(EvaluatedValue::Real),
        Expression::LiteralString(value) => Some(EvaluatedValue::String(value.clone())),
        Expression::LiteralWithUnit { value, unit } => {
            let magnitude = literal_expression_value(&value.value)?;
            let unit_text = quantity_unit_text(&unit.value)?;
            Some(EvaluatedValue::Quantity(Box::new(magnitude), unit_text))
        }
        _ => None,
    }
}

/// Extracts the raw unit token text authored inside `[...]` for a `value [unit]` quantity literal
/// (`Expression::LiteralWithUnit`). The parser wraps the token as `Expression::Bracket(Box<
/// Expression::Unit(String)>)` (see `sysml_v2_parser_next::ast::core::Expression::Unit`'s doc
/// comment: "Units may contain operators such as `/` and `^`, so they are not qualified
/// references"), i.e. the unit is captured as free text, never as a `QualifiedReferenceId` that
/// could participate in lexical name resolution -- `kg`/`SI::s`/`m/s^2` are all just opaque
/// strings at this layer, not resolved declaration references. `None` defensively covers any
/// other shape (unreachable via `literal_with_unit` in the parser today, but this stays a private
/// helper feeding an `Option`-returning classifier, so a mismatched shape falls through to
/// `Unsupported`/`NonConstant` rather than panicking).
fn quantity_unit_text(node: &Expression) -> Option<String> {
    match node {
        Expression::Bracket(inner) => quantity_unit_text(&inner.value),
        Expression::Unit(text) => Some(text.clone()),
        _ => None,
    }
}

/// Recursively builds the `EvalNode` mirror for a constraint-body expression, threading an
/// operand-ordinal counter that increments exactly where `lower_constraint_expression` would push
/// an `ExpressionOperand` reference, so the two traversals stay index-aligned. Returns `None` for
/// any shape `lower_constraint_expression` does not recognize (`Unsupported`).
fn classify_constraint_node(node: &Expression, ordinal: &mut u32) -> Option<EvalNode> {
    match node {
        Expression::LiteralInteger(_)
        | Expression::LiteralReal(_)
        | Expression::LiteralBoolean(_)
        | Expression::LiteralString(_)
        | Expression::LiteralWithUnit { .. } => {
            literal_expression_value(node).map(EvalNode::Literal)
        }
        Expression::FeatureRef(_) | Expression::FeatureChainRef(_) => {
            let leaf = EvalNode::Operand(*ordinal);
            *ordinal += 1;
            Some(leaf)
        }
        Expression::Parenthesized(inner) => classify_constraint_node(&inner.value, ordinal),
        Expression::BinaryOp { op, left, right } if is_comparison_operator(op) => {
            let left = classify_constraint_node(&left.value, ordinal)?;
            let right = classify_constraint_node(&right.value, ordinal)?;
            Some(EvalNode::Comparison(
                op.clone(),
                Box::new(left),
                Box::new(right),
            ))
        }
        Expression::BinaryOp { op, left, right } if is_arithmetic_operator(op) => {
            let left = classify_constraint_node(&left.value, ordinal)?;
            let right = classify_constraint_node(&right.value, ordinal)?;
            Some(EvalNode::Arithmetic(
                op.clone(),
                Box::new(left),
                Box::new(right),
            ))
        }
        Expression::BinaryOp { op, left, right } if is_logical_operator(op) => {
            let left = classify_constraint_node(&left.value, ordinal)?;
            let right = classify_constraint_node(&right.value, ordinal)?;
            Some(EvalNode::Logical(
                op.clone(),
                Box::new(left),
                Box::new(right),
            ))
        }
        Expression::Invocation { args, .. } => {
            let mut children = Vec::with_capacity(args.len());
            for arg in args {
                children.push(classify_constraint_node(&arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Constructor { args, .. } => {
            let mut children = Vec::with_capacity(args.len());
            for arg in args {
                children.push(classify_constraint_node(&arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::CollectionOp { base, args, .. } => {
            let mut children = Vec::with_capacity(args.len() + 1);
            children.push(classify_constraint_node(&base.value, ordinal)?);
            for arg in args {
                children.push(classify_constraint_node(&arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Tuple(items) => {
            let mut children = Vec::with_capacity(items.len());
            for item in items {
                children.push(classify_constraint_node(&item.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
            let operand = classify_constraint_node(&operand.value, ordinal)?;
            Some(EvalNode::Unary(op.clone(), Box::new(operand)))
        }
        Expression::TypeCheck { operand, .. } => {
            let mut children = Vec::with_capacity(1);
            if let Some(operand) = operand {
                children.push(classify_constraint_node(&operand.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::MetaCast { base, .. } => {
            let base = classify_constraint_node(&base.value, ordinal)?;
            Some(EvalNode::Invocation(vec![base]))
        }
        _ => None,
    }
}

/// Recursively builds the `EvalNode` mirror for a calc-body expression, mirroring
/// `lower_calc_expression`'s supported shapes: no comparison-operator support (calc bodies stay
/// comparison-free, per slice 1), plus slice 4's arithmetic `BinaryOp` (`Add`/`Sub`/`Mul`/`Div`/
/// `Mod`) support.
fn classify_calc_node(node: &Expression, ordinal: &mut u32) -> Option<EvalNode> {
    match node {
        Expression::LiteralInteger(_)
        | Expression::LiteralReal(_)
        | Expression::LiteralBoolean(_)
        | Expression::LiteralString(_)
        | Expression::LiteralWithUnit { .. } => {
            literal_expression_value(node).map(EvalNode::Literal)
        }
        Expression::FeatureRef(_) | Expression::FeatureChainRef(_) => {
            let leaf = EvalNode::Operand(*ordinal);
            *ordinal += 1;
            Some(leaf)
        }
        Expression::Parenthesized(inner) => classify_calc_node(&inner.value, ordinal),
        Expression::BinaryOp { op, left, right } if is_arithmetic_operator(op) => {
            let left = classify_calc_node(&left.value, ordinal)?;
            let right = classify_calc_node(&right.value, ordinal)?;
            Some(EvalNode::Arithmetic(
                op.clone(),
                Box::new(left),
                Box::new(right),
            ))
        }
        Expression::Invocation { args, .. } => {
            let mut children = Vec::with_capacity(args.len());
            for arg in args {
                children.push(classify_calc_node(&arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Constructor { args, .. } => {
            let mut children = Vec::with_capacity(args.len());
            for arg in args {
                children.push(classify_calc_node(&arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::CollectionOp { base, args, .. } => {
            let mut children = Vec::with_capacity(args.len() + 1);
            children.push(classify_calc_node(&base.value, ordinal)?);
            for arg in args {
                children.push(classify_calc_node(&arg.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::Tuple(items) => {
            let mut children = Vec::with_capacity(items.len());
            for item in items {
                children.push(classify_calc_node(&item.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
            let operand = classify_calc_node(&operand.value, ordinal)?;
            Some(EvalNode::Unary(op.clone(), Box::new(operand)))
        }
        Expression::TypeCheck { operand, .. } => {
            let mut children = Vec::with_capacity(1);
            if let Some(operand) = operand {
                children.push(classify_calc_node(&operand.value, ordinal)?);
            }
            Some(EvalNode::Invocation(children))
        }
        Expression::MetaCast { base, .. } => {
            let base = classify_calc_node(&base.value, ordinal)?;
            Some(EvalNode::Invocation(vec![base]))
        }
        _ => None,
    }
}

/// Whether an `EvalNode` tree contains no `Operand` leaf at all (i.e. is a pure literal, needing
/// no resolved state whatsoever to fold).
fn eval_node_is_pure_literal(node: &EvalNode) -> bool {
    match node {
        EvalNode::Literal(_) => true,
        EvalNode::Operand(_) => false,
        EvalNode::Comparison(_, left, right)
        | EvalNode::Arithmetic(_, left, right)
        | EvalNode::Logical(_, left, right) => {
            eval_node_is_pure_literal(left) && eval_node_is_pure_literal(right)
        }
        EvalNode::Unary(_, operand) => eval_node_is_pure_literal(operand),
        // Always `NonConstant` once folded (see `EvalNode::Invocation`'s doc comment), never a
        // "genuinely known" literal, regardless of how many/which literal arguments it carries.
        EvalNode::Invocation(_) => false,
    }
}

/// Folds an `EvalNode` tree to a concrete `EvaluatedValue`, resolving each `Operand(n)` leaf via
/// `resolve_operand`. Used both for construction-time pure-literal folding (an empty/unreachable
/// resolver) and for `compute_evaluation`'s resolution-time constant-propagation fold (slice 3).
fn fold_eval_node(
    node: &EvalNode,
    resolve_operand: &mut impl FnMut(u32) -> EvaluatedValue,
) -> EvaluatedValue {
    fold_eval_node_pending(node, &mut |ordinal| Some(resolve_operand(ordinal)))
        .expect("resolve_operand never returns None")
}

/// Same fold as `fold_eval_node`, but `resolve_operand` may report an operand as not-yet-settled
/// (`None`) -- used by `compute_evaluation`'s bounded constant-propagation fixed point (slice 3):
/// a `None` anywhere in the tree means the whole expression cannot be folded *this pass*, without
/// asserting anything about its eventual outcome (unlike a settled `EvaluatedValue`, which is
/// final once produced).
fn fold_eval_node_pending(
    node: &EvalNode,
    resolve_operand: &mut impl FnMut(u32) -> Option<EvaluatedValue>,
) -> Option<EvaluatedValue> {
    match node {
        EvalNode::Literal(value) => Some(value.clone()),
        EvalNode::Operand(ordinal) => resolve_operand(*ordinal),
        EvalNode::Comparison(op, left, right) => {
            let left = fold_eval_node_pending(left, resolve_operand)?;
            let right = fold_eval_node_pending(right, resolve_operand)?;
            Some(match (left, right) {
                (EvaluatedValue::NonConverged, _) | (_, EvaluatedValue::NonConverged) => {
                    EvaluatedValue::NonConverged
                }
                (EvaluatedValue::UnresolvedOperand, _) | (_, EvaluatedValue::UnresolvedOperand) => {
                    EvaluatedValue::UnresolvedOperand
                }
                (EvaluatedValue::NonConstant, _) | (_, EvaluatedValue::NonConstant) => {
                    EvaluatedValue::NonConstant
                }
                (left, right) => fold_literal_comparison(op.clone(), left, right),
            })
        }
        EvalNode::Arithmetic(op, left, right) => {
            let left = fold_eval_node_pending(left, resolve_operand)?;
            let right = fold_eval_node_pending(right, resolve_operand)?;
            Some(fold_arithmetic(op.clone(), left, right))
        }
        EvalNode::Logical(op, left, right) => {
            let left = fold_eval_node_pending(left, resolve_operand)?;
            let right = fold_eval_node_pending(right, resolve_operand)?;
            Some(fold_logical(op.clone(), left, right))
        }
        EvalNode::Invocation(_) => Some(EvaluatedValue::NonConstant),
        EvalNode::Unary(op, operand) => {
            let operand = fold_eval_node_pending(operand, resolve_operand)?;
            Some(fold_unary(op, operand))
        }
    }
}

/// True when a state def/usage's `entry`/`do`/`exit` action body (BNF `StateDefBody`, shared by
/// `EntryAction`/`DoAction`/`ExitAction.body`) carries actual owned members, as opposed to a bare
/// `;` terminator or an empty `{ }` -- both of which are legal no-op markers with nothing to
/// represent when the action also has no bound `action_reference` (see
/// `lower_state_entry_action`'s doc comment). Used to distinguish that genuinely-empty case from
/// an inline `entry { <members> }` anonymous action body, which does carry content this typed AST
/// shape has no field for and so stays an explicit unsupported diagnostic.
fn state_action_body_has_content(body: &StateDefBody) -> bool {
    matches!(body, StateDefBody::Brace { elements, .. } if !elements.is_empty())
}

/// Classifies a constraint-body expression exactly along `lower_constraint_expression`'s
/// supported-shape boundary, without pushing any reference or diagnostic (a pure, side-effect-free
/// mirror used only to decide whether/how to publish an evaluation fact). See `EvaluatedValue`.
fn classify_constraint_expression(node: &Expression) -> ExpressionEvalShape {
    let mut ordinal = 0u32;
    match classify_constraint_node(node, &mut ordinal) {
        None => ExpressionEvalShape::Unsupported,
        Some(tree) if eval_node_is_pure_literal(&tree) => {
            let value = fold_eval_node(&tree, &mut |_| {
                unreachable!("eval_node_is_pure_literal guarantees no Operand leaf is folded")
            });
            if matches!(tree, EvalNode::Literal(_)) {
                ExpressionEvalShape::Literal(value)
            } else {
                ExpressionEvalShape::ConstantFolded(value)
            }
        }
        Some(tree) => ExpressionEvalShape::HasOperand(tree),
    }
}

/// Classifies a calc-body expression exactly along `lower_calc_expression`'s supported-shape
/// boundary (the same leaf/reference/parenthesized shapes as `classify_constraint_expression`,
/// minus comparison-operator support, plus slice 4's arithmetic `BinaryOp` support -- see
/// `classify_calc_node`).
fn classify_calc_expression(node: &Expression) -> ExpressionEvalShape {
    let mut ordinal = 0u32;
    match classify_calc_node(node, &mut ordinal) {
        None => ExpressionEvalShape::Unsupported,
        Some(tree) if eval_node_is_pure_literal(&tree) => {
            let value = fold_eval_node(&tree, &mut |_| {
                unreachable!("eval_node_is_pure_literal guarantees no Operand leaf is folded")
            });
            if matches!(tree, EvalNode::Literal(_)) {
                ExpressionEvalShape::Literal(value)
            } else {
                ExpressionEvalShape::ConstantFolded(value)
            }
        }
        Some(tree) => ExpressionEvalShape::HasOperand(tree),
    }
}

/// Whether a `BinaryOperator` is one of the eight boolean comparison operators
/// (`lower_constraint_expression`'s supported `BinaryOp` shape): `==`, `!=`, `<`, `<=`, `>`, `>=`,
/// and KerML's strict-identity `===`/`!==` (`StrictEq`/`StrictNe`). The latter two were originally
/// deliberately excluded from this predicate, but real-corpus evidence (exhaustive
/// `unsupported_calc_definition_member` audit, e.g. Kernel Function Library `BaseFunctions.kerml`'s
/// `function '!=='{ ... return : Boolean[1] = not (x === y); }`) shows they appear alongside the
/// other six comparisons in ordinary reference-resolution contexts identically -- this pipeline only
/// ever recurses into a comparison's operands to resolve references, it does not evaluate strict vs.
/// non-strict identity semantics differently, so there is no reason to keep them unsupported.
fn is_comparison_operator(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Eq
            | BinaryOperator::Ne
            | BinaryOperator::StrictEq
            | BinaryOperator::StrictNe
            | BinaryOperator::Lt
            | BinaryOperator::Le
            | BinaryOperator::Gt
            | BinaryOperator::Ge
    )
}

/// Whether a `BinaryOperator` is one of the boolean-combination operators
/// `classify_constraint_node`/`lower_constraint_expression`'s logical `BinaryOp` shape supports:
/// `and`, `or`, `xor`, `implies`, KerML's single-ampersand `&` spelling of conjunction
/// (`BinaryOperator::BitAnd`, `BinaryOperator::from_token("&")`), and its single-pipe `|` spelling
/// of disjunction (`BinaryOperator::BitOr`, `BinaryOperator::from_token("|")`). Per the KerML
/// textual notation (§8.2.7 invariants), `&`/`|` are the ordinary boolean-and/-or connectives in a
/// constraint/invariant boolean expression -- e.g. `sysml.library/trig_functions.md`'s `-1.0 <=
/// that & that <= 1.0` and `sysml.library/state_performances.md`'s `accT == accableT &
/// incomingTransferSort(...)` for `&`, and `sysml.library/state_performances.md`'s `accableT ==
/// accT | incomingTransferSort(accT, accableT)` for `|` -- not a bitwise/set operator; the parser's
/// own `BinaryOperator::from_token` has no distinct "boolean and"/"boolean or" token for `&`/`|`,
/// only the shared `BitAnd`/`BitOr` classification, so this predicate (and `fold_logical`, which
/// treats them identically to `And`/`Or`) is where that context-dependent meaning is recovered.
/// `xor`/`implies` share `and`/`or`'s exact Boolean/Boolean two-operand truth-table shape
/// (`fold_logical`), so widening this predicate is the whole of their support -- no new `EvalNode`
/// variant or failure state needed.
fn is_logical_operator(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::And
            | BinaryOperator::Or
            | BinaryOperator::Xor
            | BinaryOperator::Implies
            | BinaryOperator::BitAnd
            | BinaryOperator::BitOr
    )
}

/// Whether a `UnaryOperator` is one of the two unary operators `classify_constraint_node`/
/// `classify_calc_node`/`lower_constraint_expression`/`lower_calc_expression` support: `-` (negation,
/// `fold_unary`) and `not` (logical negation). `+` (`Plus`, a structural no-op the grammar rarely if
/// ever needs folded) and `~` (`BitNot`, no corresponding `EvaluatedValue` bit type) are deliberately
/// out of scope, mirroring `is_arithmetic_operator`'s precedent of excluding operators that would not
/// make any additional real-corpus expression foldable.
fn is_unary_operator(op: &UnaryOperator) -> bool {
    matches!(op, UnaryOperator::Minus | UnaryOperator::Not)
}

/// Whether a `BinaryOperator` is one of the arithmetic operators
/// (`lower_calc_expression`'s supported `BinaryOp` shape): `+`, `-`, `*`, `/`, `%`, and the two
/// exponentiation spellings `^`/`Pow` and `**`/`Exp`. Slice 4 (`bd50fccd`) originally excluded
/// `Exp`/`Pow`, reasoning every real-corpus `**` occurrence combined it with unary negation or
/// fractional/negative exponents -- shapes outside that slice's scope regardless. Unary negation
/// landed separately (`438b8572`), and fractional/negative exponents on a `Real` base are just
/// `f64::powf`, already covered by this function's own `Real`-promotion path (see
/// `fold_arithmetic`), so neither reason still blocks folding the operator itself: e.g.
/// `10c_fuel_economy_analysis.md`'s `231.0 * 'in'^3` now folds one level further into the `Pow`.
fn is_arithmetic_operator(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Add
            | BinaryOperator::Sub
            | BinaryOperator::Mul
            | BinaryOperator::Div
            | BinaryOperator::Mod
            | BinaryOperator::Pow
            | BinaryOperator::Exp
    )
}

/// Whether a `BinaryOperator` is the KerML range-construction operator `..` (`Range`, e.g.
/// `(1..size(x))->forAll {...}`, Kernel Function Library `SequenceFunctions.kerml`) or the
/// null-coalescing operator `??` (`NullCoalesce`, e.g. `collection->reduce '+' ?? zero`, Kernel
/// Function Library `DataFunctions.kerml`/`NumericalFunctions.kerml`). Neither is arithmetic,
/// comparison, or logical in the sense `is_arithmetic_operator`/`is_comparison_operator`/
/// `is_logical_operator` model, but both are ordinary two-operand expression shapes for this
/// pipeline's reference-resolution-only purposes (no evaluation folding is attempted for either),
/// so they share this narrow, purpose-specific predicate rather than being folded into one of the
/// other three.
fn is_range_or_coalesce_operator(op: &BinaryOperator) -> bool {
    matches!(op, BinaryOperator::Range | BinaryOperator::NullCoalesce)
}

/// Flattens a dotted `Expression::MemberAccess` chain (`a.b.c`, parsed as nested
/// `MemberAccess(MemberAccess(FeatureRef(a), b), c)`) into its ordered list of qualified-reference
/// segments: the innermost `FeatureRef`/`FeatureChainRef`'s own (possibly multi-segment) path,
/// followed by each subsequent `member` segment outward. Returns `None` when the chain's root is
/// anything other than a `FeatureRef`/`FeatureChainRef` (an index expression, invocation, literal,
/// etc.), since this pipeline has no lexical-lookup starting point for those shapes -- the caller
/// falls through to its existing unsupported-member diagnostic in that case. A bare
/// `FeatureRef`/`FeatureChainRef` (no `MemberAccess` wrapper at all) flattens to a single-entry
/// list, letting callers route both shapes through the same chain-resolution path uniformly.
///
/// `Expression::Parenthesized` and `Expression::TypeCheck` (the `as`/`istype`/`hastype` cast
/// family) are transparent wrappers for this purpose: `(vehicles as VehiclePart).m` (real corpus
/// usage, `sysml/examples/calculation_test.md`) is `MemberAccess(Parenthesized(TypeCheck{operand:
/// FeatureRef(vehicles), type_name: VehiclePart}), m)`, and its member `m` resolves relative to
/// `vehicles`' own lexical lookup exactly like an uncast `vehicles.m` would -- this pipeline does
/// not model the cast's type-narrowing effect on member lookup (no call site needs it yet), it
/// just stops treating the cast as an opaque root the way it previously stopped the whole chain
/// from resolving at all. A `TypeCheck` with no operand (bare `istype`/`hastype` with an implicit
/// subject) still returns `None`, matching Gap 41's `that`-self-reference boundary: an implicit
/// `that` operand is indistinguishable from a genuinely absent one without the same upstream
/// parser fix that gap already documents.
fn flatten_member_access_chain(node: &Node<Expression>) -> Option<Vec<QualifiedReferenceId>> {
    match &node.value {
        Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => Some(vec![*target]),
        Expression::MemberAccess { base, member, .. } => {
            let mut chain = flatten_member_access_chain(base)?;
            chain.push(*member);
            Some(chain)
        }
        Expression::Parenthesized(inner) => flatten_member_access_chain(inner),
        Expression::TypeCheck {
            operand: Some(operand),
            ..
        } => flatten_member_access_chain(operand),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthoredImportShape {
    Membership,
    Namespace,
    Filter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AuthoredImportFacts {
    shape: AuthoredImportShape,
    recursive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct RelationshipFlags {
    conjugated: bool,
    implied: bool,
    recursive: bool,
    wildcard: bool,
    direction: Option<ParameterDirection>,
    /// Mirrors the `variation` keyword prefix (BNF `BasicDefinitionPrefix`, `DefinitionPrefix::
    /// Variation`) on the owning `part`/`part def` declaration whose `FeatureTyping`/
    /// `Subclassification` reference carries this flag -- the same convention `conjugated` uses
    /// for a port's typing target polarity, rather than inventing a new relationship kind. Set on
    /// `lower_part_usage`'s own typing reference (e.g. `variation part transmission :
    /// Transmission;`); the sibling `abstract` prefix is deliberately left unrepresented, as
    /// before this slice.
    variation: bool,
}

/// The `in`/`out`/`inout` direction prefix on a directed parameter declaration (BNF `InOutDecl`),
/// carried as a fact on the declaration's `FeatureTyping` reference (see
/// `DeclarationKind::ParameterUsage`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParameterDirection {
    In,
    Out,
    InOut,
}

/// One authored multiplicity bound (`[lower..upper]`).
///
/// The parser stores each bound as a full `Expression` (`ast::Multiplicity`), and a bound written
/// as `*` -- or omitted entirely, as in `[1..*]`'s upper and both of `[*]`'s -- is `None` on that
/// side. A missing bound in an authored multiplicity is therefore the unbounded bound, which is
/// why absence and `Unbounded` are the same fact here; a declaration with no `[...]` at all
/// carries no `MultiplicityRecord` in the first place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MultiplicityBound {
    /// No bound authored on this side: unbounded.
    Unbounded,
    /// A bound that folds to a literal integer from literals alone (`[3]`, `[0..4]`).
    Literal(i64),
    /// A bound authored as a non-literal expression (`[1..n]`, `[a#(0)]`). Published as an explicit
    /// non-literal fact -- its effective value needs operand resolution, which this fact family
    /// deliberately does not perform, and it is never recovered by re-reading authored text.
    Expression,
}

/// The authored multiplicity of one declaration (BNF `MultiplicityBounds`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct MultiplicityRecord {
    lower: MultiplicityBound,
    upper: MultiplicityBound,
    span: Span,
}

/// The `snapshot`/`timeslice` portion prefix on an occurrence usage (`ast::OccurrencePortionKind`).
///
/// Note the bare `portion` keyword is a separate, unrelated modifier. In KerML scope it is
/// reachable as `ast::KermlFeatureMember::is_portion` and lowers to `DeclarationModifiers::portion`;
/// only the two portion *kinds* below are reachable here, and only on `OccurrenceUsage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PortionKind {
    Snapshot,
    Timeslice,
}

/// The closed set of declaration modifier prefixes the pinned parser can express.
///
/// Deliberately not a `Vec<String>` of labels: each flag is a typed fact with exactly one parser
/// field behind it. Modifiers the parser cannot represent at all -- SysML `readonly`, SysML
/// `variable`, `unique`, and the bare `portion` prefix -- are absent from this set by construction
/// rather than defaulted to `false`; see `planning/UPSTREAM_PARSER_GAPS.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct DeclarationModifiers {
    /// `abstract` (`ast::DefinitionPrefix::Abstract`, or a bare `is_abstract` field).
    is_abstract: bool,
    /// `variation` (`ast::DefinitionPrefix::Variation`, or a bare `is_variation` field).
    variation: bool,
    /// `individual`.
    individual: bool,
    /// `derived`.
    derived: bool,
    /// `end` used as a feature prefix (distinct from the `end name : T;` declaration form, which
    /// is its own `DeclarationKind`).
    end: bool,
    /// `ref` used as a feature prefix (distinct from the `ref name : T;` declaration form).
    reference: bool,
    /// `constant`.
    constant: bool,
    /// `event`.
    event: bool,
    /// `standard`, on a library package.
    standard: bool,
    /// `all`, the sufficiency prefix.
    all: bool,
    /// KerML `composite`.
    composite: bool,
    /// KerML `portion` (reachable only through `KermlFeatureMember`).
    portion: bool,
    /// KerML `var`.
    var: bool,
    /// KerML `member`.
    member: bool,
    /// `ordered`, the collection modifier.
    ordered: bool,
    /// `nonunique`, the collection modifier.
    nonunique: bool,
}

/// The authored presentation-adjacent facts of one declaration, recorded at the point its typed
/// parser node is lowered.
///
/// Held in a dense side table parallel to `SemanticModelStorage::declarations` rather than widened
/// into `Declaration`, which stays the compact identity/ownership record every resolution index is
/// built over.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct DeclarationFacts {
    /// The `<shortName>` identification prefix, where the owning parser node has the field.
    short_name: Option<SymbolId>,
    modifiers: DeclarationModifiers,
    portion_kind: Option<PortionKind>,
    direction: Option<ParameterDirection>,
    multiplicity: Option<MultiplicityRecord>,
}

impl DeclarationFacts {
    /// Facts for a declaration with no authored modifier, multiplicity, direction, or short name.
    ///
    /// Used both by synthesized anonymous scopes -- `BareConnect`, `Transition`, the state action
    /// bindings and other declarations the lowering mints to give nested references a lexical
    /// scope, which have no authored declaration syntax at all -- and by authored declaration
    /// forms whose parser node carries none of these fields.
    fn none() -> Self {
        Self::default()
    }
}

/// Which annotation production a `DocumentationRecord` came from.
///
/// The parser discards `/** ... */` and `//` as lexer trivia, so only the three keyworded forms
/// survive into the AST at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnnotationForm {
    /// `doc /* ... */` (`ast::DocComment`).
    Documentation,
    /// `comment /* ... */` (`ast::CommentAnnotation`).
    Comment,
    /// `rep <language> "..." /* ... */` (`ast::TextualRepresentation`).
    TextualRepresentation,
}

/// One documentation/comment/textual-representation annotation bound to its annotated declaration.
///
/// The parser models these as *sibling* body elements with no parent link, so lowering binds the
/// annotations at the head of a declaration's own body to that declaration. A declaration may
/// carry several, which is why this is a table rather than a field on `DeclarationFacts`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DocumentationRecord {
    declaration: DeclarationId,
    form: AnnotationForm,
    locale: Option<SymbolId>,
    /// The `rep` language string; always `None` for the other two forms.
    language: Option<SymbolId>,
    text: SymbolId,
    span: Span,
}

/// Whether a feature value binds (`=`) or assigns (`:=`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeatureValueKind {
    Bind,
    Assign,
}

/// The authored feature value of one declaration (`ast::FeatureValue`).
///
/// Keeps all five authored spellings distinguishable: `= e`, `:= e`, `default = e`, `default := e`,
/// and the operator-less `default e`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FeatureValueRecord {
    declaration: DeclarationId,
    kind: FeatureValueKind,
    is_default: bool,
    has_operator: bool,
    span: Span,
}

/// Builds the multiplicity fact for a declaration whose parser node carries a `multiplicity` field.
///
/// A declaration with no `[...]` written yields `None`; that is genuinely "no multiplicity
/// authored", distinct from `[*]`, which yields a record with both bounds `Unbounded`.
fn multiplicity_facts(multiplicity: Option<&Node<Multiplicity>>) -> Option<MultiplicityRecord> {
    let multiplicity = multiplicity?;
    Some(MultiplicityRecord {
        lower: multiplicity_bound(multiplicity.value.lower.as_deref()),
        upper: multiplicity_bound(multiplicity.value.upper.as_deref()),
        span: multiplicity.value.span.clone(),
    })
}

fn multiplicity_bound(expression: Option<&Node<Expression>>) -> MultiplicityBound {
    let Some(expression) = expression else {
        return MultiplicityBound::Unbounded;
    };
    match literal_bound_value(&expression.value) {
        Some(value) => MultiplicityBound::Literal(value),
        None => MultiplicityBound::Expression,
    }
}

/// Folds a multiplicity bound expression to a literal integer, or reports that it is not one.
///
/// Deliberately narrow: only an integer literal (optionally parenthesised) is a literal bound.
/// Everything else -- a feature reference, an arithmetic expression, an index -- is published as
/// `MultiplicityBound::Expression` rather than guessed at, because folding it needs operand
/// resolution this fact family does not perform.
fn literal_bound_value(expression: &Expression) -> Option<i64> {
    match expression {
        Expression::LiteralInteger(value) => Some(*value),
        Expression::Parenthesized(inner) => literal_bound_value(&inner.value),
        _ => None,
    }
}

/// Splits a `definition_prefix`/`usage_prefix` field into its two independent modifier facts.
fn definition_prefix_modifiers(prefix: Option<&DefinitionPrefix>) -> (bool, bool) {
    match prefix {
        Some(DefinitionPrefix::Abstract) => (true, false),
        Some(DefinitionPrefix::Variation) => (false, true),
        None => (false, false),
    }
}

fn portion_kind_fact(kind: Option<&ParserOccurrencePortionKind>) -> Option<PortionKind> {
    match kind? {
        ParserOccurrencePortionKind::Snapshot => Some(PortionKind::Snapshot),
        ParserOccurrencePortionKind::Timeslice => Some(PortionKind::Timeslice),
    }
}

/// Maps a bodied KerML classifier's keyword to the metaclass it denotes.
///
/// `assoc` and `association` are the short and spelled-out spellings of one keyword (see
/// `KermlClassifierKeyword`'s own doc comments), so both denote KerML `Association`; every other
/// keyword denotes a distinct metaclass.
fn kerml_classifier_kind(keyword: &KermlClassifierKeyword) -> DeclarationKind {
    match keyword {
        KermlClassifierKeyword::Type => DeclarationKind::KermlType,
        KermlClassifierKeyword::Classifier => DeclarationKind::KermlClassifier,
        KermlClassifierKeyword::Class => DeclarationKind::KermlClass,
        KermlClassifierKeyword::Struct => DeclarationKind::KermlStructure,
        KermlClassifierKeyword::Assoc | KermlClassifierKeyword::Association => {
            DeclarationKind::KermlAssociation
        }
        KermlClassifierKeyword::AssocStruct => DeclarationKind::KermlAssociationStructure,
        KermlClassifierKeyword::Datatype => DeclarationKind::KermlDataType,
        KermlClassifierKeyword::Metaclass => DeclarationKind::KermlMetaclass,
        KermlClassifierKeyword::Behavior => DeclarationKind::KermlBehavior,
        KermlClassifierKeyword::Function => DeclarationKind::KermlFunction,
        KermlClassifierKeyword::Predicate => DeclarationKind::KermlPredicate,
        KermlClassifierKeyword::Interaction => DeclarationKind::KermlInteraction,
        KermlClassifierKeyword::Multiplicity => DeclarationKind::KermlMultiplicity,
    }
}

/// Maps a KerML feature member's kind keyword to the metaclass it denotes.
fn kerml_feature_kind(kind: &KermlFeatureKind) -> DeclarationKind {
    match kind {
        KermlFeatureKind::Feature => DeclarationKind::KermlFeature,
        KermlFeatureKind::Step => DeclarationKind::KermlStep,
        KermlFeatureKind::Expr => DeclarationKind::KermlExpression,
        KermlFeatureKind::Bool => DeclarationKind::KermlBooleanExpression,
    }
}

fn direction_fact(direction: Option<&InOut>) -> Option<ParameterDirection> {
    match direction? {
        InOut::In => Some(ParameterDirection::In),
        InOut::Out => Some(ParameterDirection::Out),
        InOut::InOut => Some(ParameterDirection::InOut),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParserReferenceId {
    document: DocumentId,
    local: QualifiedReferenceId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnsupportedFamily {
    PackageMember,
    PartDefinitionMember,
    PartUsageMember,
    AttributeMember,
    RequirementDefinitionMember,
    PortDefinitionMember,
    /// No remaining `PortBodyElement` fallback variant constructs this family (every variant is
    /// now dispatched to a lowering function), but the diagnostic code itself remains part of
    /// `writer.rs`'s exhaustive mapping for schema stability.
    #[allow(dead_code)]
    PortUsageMember,
    ActionDefinitionMember,
    ActionUsageMember,
    /// Shared by `state def` and `state` usage bodies (both use `StateDefBody`/
    /// `StateDefBodyElement` in the typed AST -- there is no separate `StateUsageBody`).
    StateDefinitionMember,
    /// Shared by `connection def` and `connection` usage bodies (both use `ConnectionDefBody`/
    /// `ConnectionDefBodyElement` in the typed AST -- there is no separate `ConnectionUsageBody`).
    ConnectionDefinitionMember,
    /// Shared by `occurrence def` and `occurrence` usage bodies (both share `OccurrenceBodyElement`
    /// -- `OccurrenceDef.body` wraps it in the generic `DefinitionBody`/`DefinitionBodyElement`,
    /// while `OccurrenceUsage.body` (`OccurrenceUsageBody`) holds it directly). Occurrence-specific
    /// semantics -- individual/portion-of-life, time-slicing, snapshot facts, `exhibit`/
    /// `succession`/`satisfy`/`allocate`/connector-end body constructs -- are the out-of-scope
    /// surface for this slice.
    OccurrenceDefinitionMember,
    /// `analysis def` body members not modeled by this slice (subject/actor/objective/first-
    /// succession/return/nested case structure); shares `UseCaseDefBody`/`UseCaseDefBodyElement`
    /// with `case`/`verification` case bodies in the typed AST, but this family name is scoped to
    /// `analysis def` specifically since `analysis` usage lowering is deferred.
    AnalysisCaseDefinitionMember,
    /// `constraint def`/`constraint` usage body members not modeled by this slice (constraint
    /// expression content, nested constraint members). Shared by both forms since
    /// `ConstraintDefBody`/`ConstraintDefBodyElement` is the same typed AST shape for both.
    ConstraintDefinitionMember,
    /// `calc def`/`calc` usage body members not modeled by this slice (calculation expression
    /// content, in/out/return parameters, nested calc structure). Shared by both forms since
    /// `CalcDefBody`/`CalcDefBodyElement` is the same typed AST shape for both.
    CalcDefinitionMember,
    /// `interface def` body members not modeled by this slice; shares the same `end`/`connect`/
    /// attribute/item/port/flow member set as `ConnectionDefinitionMember`, kept as its own family
    /// name so interface def diagnostics stay distinct from connection def ones at the same span
    /// shape.
    InterfaceDefinitionMember,
    /// `view def` body members not modeled by this slice: `render` (`ViewRenderingUsage`),
    /// `filter` (view composition), `expose`/`satisfy` are `view` usage-body-only constructs
    /// (`ViewBodyElement`, not `ViewDefBodyElement`) and don't appear here at all.
    ViewDefinitionMember,
    /// `rendering def` body members not modeled by this slice: shares the same `filter`/`render`
    /// member set as `ViewDefinitionMember` (`RenderingDefBody`/`RenderingDefBodyElement` mirror
    /// `ViewDefBody`/`ViewDefBodyElement` exactly), kept as its own family name so `rendering def`
    /// diagnostics stay distinct from `view def` ones at the same span shape.
    RenderingDefinitionMember,
    /// `case def` body members not modeled by this slice (subject/actor/objective/first-
    /// succession/return/nested case structure); shares `UseCaseDefBody`/`UseCaseDefBodyElement`
    /// with `analysis`/`verification`/`use case` case bodies in the typed AST, but this family
    /// name is scoped to `case def` specifically since `case` usage lowering is deferred.
    CaseDefinitionMember,
    /// `verification def` body members not modeled by this slice; shares the same
    /// `UseCaseDefBody`/`UseCaseDefBodyElement` shape as `CaseDefinitionMember`, kept as its own
    /// family name so verification def diagnostics stay distinct from case/analysis/use-case def
    /// ones at the same span shape.
    VerificationCaseDefinitionMember,
    /// `use case def` body members not modeled by this slice; shares the same `UseCaseDefBody`/
    /// `UseCaseDefBodyElement` shape as `CaseDefinitionMember`, kept as its own family name so use
    /// case def diagnostics stay distinct from case/analysis/verification def ones at the same
    /// span shape.
    UseCaseDefinitionMember,
    /// Members inside a `ref { ... }` body that this slice does not model. A `ref` body is the
    /// general usage-member set (`RefBody = Body<PartUsageBodyElement>`, `UsageBody =
    /// DefinitionBody` per SysML 8.2.2.6.2), so it is dispatched through the same
    /// `lower_part_usage_body_element` walker every other usage body uses; this family keeps its
    /// unsupported members distinguishable from a `part` usage body's. The `ref` declaration
    /// itself (name, typing, redefines, subsets) is lowered via `DeclarationKind::ReferenceUsage`
    /// regardless of what its body holds.
    ReferenceUsageMember,
    /// Members of a KerML `RelationshipBody` (`import`/`dependency`/`alias`/plain `connect`
    /// bodies) that this slice does not model. The body's annotating members are recorded against
    /// the relationship's own declaration; this family covers unmodeled facts of an owned
    /// `feature` member (BNF `RelationshipBody`'s `ownedRelatedElement`).
    RelationshipBodyMember,
    ParserUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnsupportedRecord {
    document: DocumentId,
    family: UnsupportedFamily,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecoveryRecord {
    document: DocumentId,
    span: Span,
}

#[derive(Debug)]
struct CanonicalDocument {
    identity: Box<str>,
    /// The role this source plays in the build, carried through from the admitted
    /// [`OwnedSourceRecord`]. Library sources participate in one semantic system with workspace
    /// sources, so this is never an admission filter; it is what lets owner-defined projections
    /// report the authored workspace without also reporting the whole standard library.
    role: SourceRole,
    parsed: Arc<ParsedDocument>,
    parse_errors: Box<[ParseError]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Declaration {
    document: DocumentId,
    owner: Option<DeclarationId>,
    name: Option<SymbolId>,
    anonymous_ordinal: Option<u32>,
    kind: DeclarationKind,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MembershipRecord {
    member: DeclarationId,
    kind: MembershipKind,
    visibility: Visibility,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthoredReference {
    source: DeclarationId,
    kind: ReferenceKind,
    target: ParserReferenceId,
    path: SymbolPathId,
    ordinal: u32,
    import: Option<AuthoredImportFacts>,
    flags: RelationshipFlags,
    span: Span,
}

struct PendingReference {
    source: DeclarationId,
    kind: ReferenceKind,
    document: DocumentId,
    local: QualifiedReferenceId,
    flags: RelationshipFlags,
    span: Span,
    import: Option<AuthoredImportFacts>,
}

/// A construction-time-classified evaluation candidate: the declaration a supported constraint/
/// calc expression belongs to, plus its `ExpressionEvalShape`. Only `Literal`/`HasOperand` shapes
/// are ever stored (see `SemanticModelBuilder::push_evaluation_fact`); `Unsupported` publishes no
/// fact, keeping the evaluation pass strictly within slice 1's supported syntactic scope.
#[derive(Debug, Clone)]
struct PendingEvaluationFact {
    declaration: DeclarationId,
    shape: ExpressionEvalShape,
}

#[derive(Debug)]
struct SemanticModelStorage {
    documents: Box<[CanonicalDocument]>,
    declarations: Box<[Declaration]>,
    /// Parallel to `declarations`, one entry per `DeclarationId`.
    declaration_facts: Box<[DeclarationFacts]>,
    memberships: Box<[MembershipRecord]>,
    references: Box<[AuthoredReference]>,
    documentation: Box<[DocumentationRecord]>,
    feature_values: Box<[FeatureValueRecord]>,
    unsupported: Box<[UnsupportedRecord]>,
    recovery: Box<[RecoveryRecord]>,
    symbols: SymbolTable,
    paths: SymbolPathArena,
    evaluation_facts: Box<[PendingEvaluationFact]>,
}

impl SemanticModelStorage {
    fn document(&self, id: DocumentId) -> Option<&CanonicalDocument> {
        self.documents.get(id.index())
    }

    fn declaration(&self, id: DeclarationId) -> Option<&Declaration> {
        self.declarations.get(id.index())
    }

    fn declaration_facts(&self, id: DeclarationId) -> Option<&DeclarationFacts> {
        self.declaration_facts.get(id.index())
    }

    fn symbol(&self, id: SymbolId) -> Option<&str> {
        self.symbols.get(id)
    }
}

#[derive(Debug, Default)]
struct SemanticModelBuilder {
    documents: Vec<CanonicalDocument>,
    document_index: HashTable<DocumentId>,
    document_hash_builder: RandomState,
    declarations: Vec<Declaration>,
    declaration_facts: Vec<DeclarationFacts>,
    memberships: Vec<MembershipRecord>,
    references: Vec<AuthoredReference>,
    documentation: Vec<DocumentationRecord>,
    feature_values: Vec<FeatureValueRecord>,
    unsupported: Vec<UnsupportedRecord>,
    recovery: Vec<RecoveryRecord>,
    evaluation_facts: Vec<PendingEvaluationFact>,
    symbols: SymbolTableBuilder,
    paths: SymbolPathArenaBuilder,
    path_scratch: Vec<SymbolId>,
    next_anonymous_ordinals: BTreeMap<(DocumentId, Option<DeclarationId>, DeclarationKind), u32>,
    next_reference_ordinals: BTreeMap<(DeclarationId, ReferenceKind), u32>,
}

impl SemanticModelBuilder {
    fn admit_document(
        &mut self,
        identity: impl Into<Box<str>>,
        role: SourceRole,
        parsed: Arc<ParsedDocument>,
        parse_errors: Vec<ParseError>,
    ) -> Result<DocumentId, ConstructionError> {
        let identity = identity.into();
        let hash = self.document_hash_builder.hash_one(identity.as_ref());
        if self
            .document_index
            .find(hash, |candidate| {
                self.documents[candidate.index()].identity.as_ref() == identity.as_ref()
            })
            .is_some()
        {
            return Err(ConstructionError::DuplicateDocumentIdentity);
        }
        let id = DocumentId::from_index(self.documents.len())?;
        self.documents
            .try_reserve(1)
            .map_err(|_| ConstructionError::Capacity)?;
        let documents = &self.documents;
        let hash_builder = &self.document_hash_builder;
        self.document_index
            .try_reserve(1, |candidate| {
                hash_builder.hash_one(documents[candidate.index()].identity.as_ref())
            })
            .map_err(|_| ConstructionError::Capacity)?;
        self.documents.push(CanonicalDocument {
            identity,
            role,
            parsed,
            parse_errors: parse_errors.into_boxed_slice(),
        });
        let documents = &self.documents;
        let hash_builder = &self.document_hash_builder;
        self.document_index.insert_unique(hash, id, |candidate| {
            hash_builder.hash_one(documents[candidate.index()].identity.as_ref())
        });
        Ok(id)
    }

    fn intern_name(&mut self, value: &str) -> Result<SymbolId, ConstructionError> {
        self.symbols.intern(value)
    }

    fn intern_declared_name(&mut self, value: &str) -> Result<Option<SymbolId>, ConstructionError> {
        (!value.is_empty())
            .then(|| self.intern_name(value))
            .transpose()
    }

    #[cfg(test)]
    fn push_declaration(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        name: Option<SymbolId>,
    ) -> Result<DeclarationId, ConstructionError> {
        self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Package,
            name,
            Span::dummy(),
            DeclarationFacts::none(),
        )
    }

    /// Mints one declaration identity and records its authored facts in the same call.
    ///
    /// `facts` is a required parameter rather than an optional follow-up so that every present and
    /// future lowering site has to make an explicit decision about the declaration's modifiers,
    /// multiplicity, direction, and short name. A site with nothing to record passes
    /// `DeclarationFacts::none()`; a site that simply forgets does not compile.
    fn push_typed_declaration(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        kind: DeclarationKind,
        name: Option<SymbolId>,
        span: Span,
        facts: DeclarationFacts,
    ) -> Result<DeclarationId, ConstructionError> {
        if document.index() >= self.documents.len()
            || owner.is_some_and(|id| id.index() >= self.declarations.len())
            || name.is_some_and(|id| id.index() >= self.symbols.len())
            || facts
                .short_name
                .is_some_and(|id| id.index() >= self.symbols.len())
        {
            return Err(ConstructionError::InvalidIdentity);
        }
        let id = DeclarationId::from_index(self.declarations.len())?;
        let anonymous_ordinal = if name.is_none() {
            let ordinal = self
                .next_anonymous_ordinals
                .entry((document, owner, kind))
                .or_insert(0);
            let value = *ordinal;
            *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
            Some(value)
        } else {
            None
        };
        self.declarations.push(Declaration {
            document,
            owner,
            name,
            anonymous_ordinal,
            kind,
            span,
        });
        self.declaration_facts.push(facts);
        debug_assert_eq!(self.declarations.len(), self.declaration_facts.len());
        Ok(id)
    }

    /// Records one `doc`/`comment`/`rep` annotation against the declaration it annotates.
    ///
    /// The parser attaches these as sibling body elements with no parent link, so the binding is
    /// made by the lowering walk rather than read off the annotated node.
    fn push_documentation(
        &mut self,
        declaration: DeclarationId,
        form: AnnotationForm,
        locale: Option<SymbolId>,
        language: Option<SymbolId>,
        text: SymbolId,
        span: Span,
    ) -> Result<(), ConstructionError> {
        if declaration.index() >= self.declarations.len()
            || text.index() >= self.symbols.len()
            || locale.is_some_and(|id| id.index() >= self.symbols.len())
            || language.is_some_and(|id| id.index() >= self.symbols.len())
        {
            return Err(ConstructionError::InvalidIdentity);
        }
        self.documentation.push(DocumentationRecord {
            declaration,
            form,
            locale,
            language,
            text,
            span,
        });
        Ok(())
    }

    /// Interns an optional authored `<shortName>` prefix, treating an empty spelling as absent.
    fn intern_short_name(
        &mut self,
        short_name: Option<&String>,
    ) -> Result<Option<SymbolId>, ConstructionError> {
        match short_name {
            Some(value) => self.intern_declared_name(value),
            None => Ok(None),
        }
    }

    /// Records a `doc /* ... */` body element against the declaration owning that body.
    ///
    /// `declaration` is `None` only for a `doc` written at document-root scope, whose annotated
    /// element is the file root -- an element this model deliberately does not mint a declaration
    /// for -- so there is nothing to bind it to.
    fn record_root_doc_comment(
        &mut self,
        declaration: Option<DeclarationId>,
        node: &Node<DocComment>,
    ) -> Result<(), ConstructionError> {
        match declaration {
            Some(declaration) => self.record_doc_comment(declaration, node),
            None => Ok(()),
        }
    }

    /// Root-scope counterpart of `record_comment_annotation`; see `record_root_doc_comment`.
    fn record_root_comment_annotation(
        &mut self,
        declaration: Option<DeclarationId>,
        node: &Node<CommentAnnotation>,
    ) -> Result<(), ConstructionError> {
        match declaration {
            Some(declaration) => self.record_comment_annotation(declaration, node),
            None => Ok(()),
        }
    }

    /// Root-scope counterpart of `record_textual_representation`; see `record_root_doc_comment`.
    fn record_root_textual_representation(
        &mut self,
        declaration: Option<DeclarationId>,
        node: &Node<TextualRepresentation>,
    ) -> Result<(), ConstructionError> {
        match declaration {
            Some(declaration) => self.record_textual_representation(declaration, node),
            None => Ok(()),
        }
    }

    /// Lowers the grammar's whole `AnnotatingElement` production (`ast::AnnotatingMember`:
    /// `doc`, `comment`, `rep`, and the `@Name` metadata spelling), which upstream dispatches as
    /// one member in every scope that accepts all four alternatives. One production, one lowering:
    /// the alternatives keep the same per-form owners (`record_doc_comment`,
    /// `record_comment_annotation`, `record_textual_representation`,
    /// `lower_metadata_annotation`) they have wherever a scope still spells them out separately.
    ///
    /// `annotated` is `None` only where the construct owning the body mints no declaration of its
    /// own -- a `connect a to b { ... }` statement lowers its ends directly against the enclosing
    /// declaration -- so there is no element the annotation belongs to and attributing it to the
    /// enclosing type would misreport it.
    fn lower_annotating_member(
        &mut self,
        document: DocumentId,
        annotated: Option<DeclarationId>,
        member: &AnnotatingMember,
    ) -> Result<(), ConstructionError> {
        match member {
            AnnotatingMember::Doc(node) => self.record_root_doc_comment(annotated, node),
            AnnotatingMember::Comment(node) => self.record_root_comment_annotation(annotated, node),
            AnnotatingMember::TextualRep(node) => {
                self.record_root_textual_representation(annotated, node)
            }
            AnnotatingMember::MetadataAnnotation(node) => match annotated {
                Some(annotated) => self.lower_metadata_annotation(document, annotated, node),
                None => Ok(()),
            },
        }
    }

    /// Records a `doc /* ... */` annotation against the declaration whose body it heads.
    fn record_doc_comment(
        &mut self,
        declaration: DeclarationId,
        node: &Node<DocComment>,
    ) -> Result<(), ConstructionError> {
        let locale = self.intern_short_name(node.value.locale.as_ref())?;
        let text = self.intern_name(&node.value.text)?;
        self.push_documentation(
            declaration,
            AnnotationForm::Documentation,
            locale,
            None,
            text,
            node.span.clone(),
        )
    }

    /// Records a `comment /* ... */` annotation against the declaration whose body it heads.
    fn record_comment_annotation(
        &mut self,
        declaration: DeclarationId,
        node: &Node<CommentAnnotation>,
    ) -> Result<(), ConstructionError> {
        let locale = self.intern_short_name(node.value.locale.as_ref())?;
        let text = self.intern_name(&node.value.text)?;
        self.push_documentation(
            declaration,
            AnnotationForm::Comment,
            locale,
            None,
            text,
            node.span.clone(),
        )
    }

    /// Records a `rep <language> "..." /* ... */` annotation against the declaration whose body it
    /// heads.
    fn record_textual_representation(
        &mut self,
        declaration: DeclarationId,
        node: &Node<TextualRepresentation>,
    ) -> Result<(), ConstructionError> {
        let language = self.intern_declared_name(&node.value.language)?;
        let text = self.intern_name(&node.value.text)?;
        self.push_documentation(
            declaration,
            AnnotationForm::TextualRepresentation,
            None,
            language,
            text,
            node.span.clone(),
        )
    }

    /// Records the authored spelling of a `FeatureValue` clause.
    fn record_feature_value(
        &mut self,
        declaration: DeclarationId,
        value: &Node<FeatureValue>,
    ) -> Result<(), ConstructionError> {
        let kind = match value.value.kind {
            ParserFeatureValueKind::Bind => FeatureValueKind::Bind,
            ParserFeatureValueKind::Assign => FeatureValueKind::Assign,
        };
        self.push_feature_value(
            declaration,
            kind,
            value.value.is_default,
            value.value.has_operator,
            value.value.span.clone(),
        )
    }

    /// Records the authored feature value spelling of one declaration.
    ///
    /// The value *expression* itself keeps travelling the existing operand-reference and
    /// evaluation-classification path; this fact records only which of the five authored spellings
    /// (`=`, `:=`, `default =`, `default :=`, bare `default`) was written, which no other fact
    /// preserves.
    fn push_feature_value(
        &mut self,
        declaration: DeclarationId,
        kind: FeatureValueKind,
        is_default: bool,
        has_operator: bool,
        span: Span,
    ) -> Result<(), ConstructionError> {
        if declaration.index() >= self.declarations.len() {
            return Err(ConstructionError::InvalidIdentity);
        }
        self.feature_values.push(FeatureValueRecord {
            declaration,
            kind,
            is_default,
            has_operator,
            span,
        });
        Ok(())
    }

    fn push_membership(
        &mut self,
        member: DeclarationId,
        kind: MembershipKind,
        visibility: Visibility,
        span: Span,
    ) -> Result<(), ConstructionError> {
        if member.index() >= self.declarations.len() {
            return Err(ConstructionError::InvalidIdentity);
        }
        self.memberships.push(MembershipRecord {
            member,
            kind,
            visibility,
            span,
        });
        Ok(())
    }

    fn push_reference(
        &mut self,
        pending: PendingReference,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        let PendingReference {
            source,
            kind,
            document,
            local,
            flags,
            span,
            import,
        } = pending;
        if source.index() >= self.declarations.len() || document.index() >= self.documents.len() {
            return Err(ConstructionError::InvalidParserReference);
        }
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let reference = parsed
            .qualified_reference(local)
            .ok_or(ConstructionError::InvalidParserReference)?;
        let mut segments = std::mem::take(&mut self.path_scratch);
        segments.clear();
        segments
            .try_reserve(reference.segments.len())
            .map_err(|_| ConstructionError::Capacity)?;
        let path = (|| {
            for index in 0..reference.segments.len() {
                let decoded = reference
                    .segment_decoded_text(index)
                    .ok_or(ConstructionError::InvalidParserReference)?;
                segments.push(self.intern_name(decoded.as_ref())?);
            }
            self.paths.push(&segments, reference.metadata.is_absolute)
        })();
        segments.clear();
        self.path_scratch = segments;
        let path = path?;
        let ordinal = self
            .next_reference_ordinals
            .entry((source, kind))
            .or_insert(0);
        let authored_ordinal = *ordinal;
        *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
        let id = AuthoredReferenceId::from_index(self.references.len())?;
        self.references.push(AuthoredReference {
            source,
            kind,
            target: ParserReferenceId { document, local },
            path,
            ordinal: authored_ordinal,
            import,
            flags,
            span,
        });
        Ok(id)
    }

    /// Pushes one `ReferenceKind::MemberAccessOperand` reference for a flattened dotted
    /// feature-chain (`flatten_member_access_chain`'s output): `chain` is the ordered list of
    /// parser `QualifiedReferenceId`s from the root segment outward (a bare `FeatureRef`/
    /// `FeatureChainRef` flattens to a one-entry chain). Builds one combined `SymbolPathId` by
    /// concatenating every chain entry's own segments in order -- mirroring `push_reference`'s
    /// single-reference path construction, but across multiple parser references -- so
    /// `resolve_member_access_reference` in resolver.rs can walk the whole dotted path as one
    /// path with a root-lookup first segment followed by type-directed member segments. Always
    /// non-rooted (`::`-absolute chains do not occur in dotted member-access position), matching
    /// `ConnectorEnd`/`ExpressionOperand`'s existing `DeclarationDomain::Any` shape.
    fn push_member_access_reference(
        &mut self,
        source: DeclarationId,
        document: DocumentId,
        chain: &[QualifiedReferenceId],
        span: Span,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        if chain.is_empty()
            || source.index() >= self.declarations.len()
            || document.index() >= self.documents.len()
        {
            return Err(ConstructionError::InvalidParserReference);
        }
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let mut segments = std::mem::take(&mut self.path_scratch);
        segments.clear();
        let path = (|| {
            for local in chain {
                let reference = parsed
                    .qualified_reference(*local)
                    .ok_or(ConstructionError::InvalidParserReference)?;
                segments
                    .try_reserve(reference.segments.len())
                    .map_err(|_| ConstructionError::Capacity)?;
                for index in 0..reference.segments.len() {
                    let decoded = reference
                        .segment_decoded_text(index)
                        .ok_or(ConstructionError::InvalidParserReference)?;
                    segments.push(self.intern_name(decoded.as_ref())?);
                }
            }
            self.paths.push(&segments, false)
        })();
        segments.clear();
        self.path_scratch = segments;
        let path = path?;
        let kind = ReferenceKind::MemberAccessOperand;
        let ordinal = self
            .next_reference_ordinals
            .entry((source, kind))
            .or_insert(0);
        let authored_ordinal = *ordinal;
        *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
        let id = AuthoredReferenceId::from_index(self.references.len())?;
        self.references.push(AuthoredReference {
            source,
            kind,
            target: ParserReferenceId {
                document,
                local: *chain
                    .last()
                    .ok_or(ConstructionError::InvalidParserReference)?,
            },
            path,
            ordinal: authored_ordinal,
            import: None,
            flags: RelationshipFlags::default(),
            span,
        });
        Ok(id)
    }

    /// Resolves the callee of an `Expression::Invocation` (e.g. `sum` in `sum(partMasses)`) as an
    /// authored `ReferenceKind::InvocationCallee` reference sourced at `declaration`. A simple/
    /// qualified name (`FeatureRef`/`FeatureChainRef`) resolves through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point every other operand kind uses; a dotted
    /// chain (`MemberAccess`, e.g. a callee like `SysML::sum`) resolves through the same
    /// `flatten_member_access_chain`/`push_member_access_reference` path `ExpressionOperand`'s own
    /// `MemberAccess` arm uses (publishing `ReferenceKind::MemberAccessOperand`, not
    /// `InvocationCallee`, matching that shared path's existing "one kind per algorithm" trade-off
    /// -- see `ReferenceKind::MemberAccessOperand`'s doc comment). Any other callee shape (e.g. an
    /// invocation whose callee is itself computed, `(a + b)(x)`) is left unresolved: this narrow
    /// helper has no `UnsupportedFamily` to publish a diagnostic against (the invocation itself is
    /// a supported shape; only this specific callee sub-shape is not), so it silently resolves
    /// nothing for that callee rather than fabricating a reference.
    fn lower_invocation_callee(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        callee: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &callee.value {
            Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                self.push_invocation_callee_reference(document, declaration, *target)
            }
            Expression::MemberAccess { .. } => {
                if let Some(chain) = flatten_member_access_chain(callee) {
                    self.push_member_access_reference(
                        declaration,
                        document,
                        &chain,
                        callee.span.clone(),
                    )?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Pushes one `ReferenceKind::InvocationCallee` reference for a callee/`Constructor` type name
    /// that is already a parser `QualifiedReferenceId` (an `Expression::Invocation`'s `FeatureRef`/
    /// `FeatureChainRef` callee via `lower_invocation_callee`, or an `Expression::Constructor`'s
    /// `type_name` directly).
    fn push_invocation_callee_reference(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        target: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::InvocationCallee,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Pushes one `ReferenceKind::MetaCastTarget` reference for an `Expression::MetaCast`'s
    /// `metaclass` (e.g. `KerML::Classifier` in `Atom meta KerML::Classifier`), mirroring
    /// `push_type_check_target_reference`'s shape but its own `ReferenceKind` since a meta-cast
    /// target joins the `DeclarationDomain::Type` fixed point rather than `TypeCheckTarget`
    /// directly (kept distinct purely for query-output clarity).
    fn push_meta_cast_target_reference(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        target: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::MetaCastTarget,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Pushes one `ReferenceKind::TypeCheckTarget` reference for an `Expression::TypeCheck`'s
    /// `type_name` (e.g. `Type` in `x istype Type`), mirroring `push_invocation_callee_reference`'s
    /// shape but its own `ReferenceKind` since a type-check target joins the `DeclarationDomain::
    /// Type` fixed point rather than `InvocationCallee`'s `Any` domain.
    fn push_type_check_target_reference(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        target: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::TypeCheckTarget,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    fn push_unsupported(&mut self, document: DocumentId, family: UnsupportedFamily, span: Span) {
        self.unsupported.push(UnsupportedRecord {
            document,
            family,
            span,
        });
    }

    fn push_recovery(&mut self, document: DocumentId, span: Span) {
        self.recovery.push(RecoveryRecord { document, span });
    }

    /// Records one evaluation candidate for a slice-1-supported constraint/calc expression,
    /// classified by `classify_constraint_expression`/`classify_calc_expression` at the point the
    /// expression is lowered. `Unsupported` is deliberately dropped here rather than stored: an
    /// expression shape slice 1 does not recognize must publish no evaluation fact at all (mirrors
    /// its existing `unsupported_constraint_definition_member`/`unsupported_calc_definition_member`
    /// diagnostic boundary).
    fn push_evaluation_fact(&mut self, declaration: DeclarationId, shape: ExpressionEvalShape) {
        if matches!(shape, ExpressionEvalShape::Unsupported) {
            return;
        }
        self.evaluation_facts
            .push(PendingEvaluationFact { declaration, shape });
    }

    fn freeze(self) -> SemanticModelStorage {
        SemanticModelStorage {
            documents: self.documents.into_boxed_slice(),
            declarations: self.declarations.into_boxed_slice(),
            declaration_facts: self.declaration_facts.into_boxed_slice(),
            memberships: self.memberships.into_boxed_slice(),
            references: self.references.into_boxed_slice(),
            documentation: self.documentation.into_boxed_slice(),
            feature_values: self.feature_values.into_boxed_slice(),
            unsupported: self.unsupported.into_boxed_slice(),
            recovery: self.recovery.into_boxed_slice(),
            symbols: self.symbols.freeze(),
            paths: self.paths.freeze(),
            evaluation_facts: self.evaluation_facts.into_boxed_slice(),
        }
    }

    fn canonicalize_document(&mut self, document: DocumentId) -> Result<(), ConstructionError> {
        let parsed = Arc::clone(
            &self
                .documents
                .get(document.index())
                .ok_or(ConstructionError::InvalidIdentity)?
                .parsed,
        );
        for element in &parsed.root.elements {
            self.lower_root_element(document, element)?;
        }
        Ok(())
    }

    fn lower_root_element(
        &mut self,
        document: DocumentId,
        element: &Node<RootElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            RootElement::Package(node) => self.lower_package(document, None, node),
            RootElement::LibraryPackage(node) => self.lower_library_package(document, None, node),
            RootElement::Namespace(node) => self.lower_namespace(document, None, node),
            RootElement::Import(node) => self.lower_import(document, None, node),
            RootElement::Member(node) => self.lower_package_element(document, None, node),
        }
    }

    fn lower_package(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<Package>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(&node.identification)?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Package,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    fn lower_library_package(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<LibraryPackage>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(&node.identification)?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::LibraryPackage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    standard: node.is_standard,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    fn lower_namespace(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<NamespaceDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(&node.identification)?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Namespace,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    fn simple_name(
        &mut self,
        identification: &QualifiedIdentification,
    ) -> Result<Option<SymbolId>, ConstructionError> {
        identification
            .simple_name()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()
    }

    fn lower_package_body(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        body: &PackageBody,
    ) -> Result<(), ConstructionError> {
        if let PackageBody::Brace { elements, .. } = body {
            for element in elements {
                self.lower_package_element(document, owner, element)?;
            }
        }
        Ok(())
    }

    fn lower_package_element(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        element: &Node<PackageBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            PackageBodyElement::Error(node) => {
                self.push_recovery(document, node.span.clone());
            }
            PackageBodyElement::Unsupported(node) => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                );
            }
            PackageBodyElement::Doc(node) => {
                self.record_root_doc_comment(owner, node)?;
            }
            PackageBodyElement::Comment(node) => {
                self.record_root_comment_annotation(owner, node)?;
            }
            PackageBodyElement::TextualRep(node) => {
                self.record_root_textual_representation(owner, node)?;
            }
            PackageBodyElement::Filter(node) => match owner {
                Some(declaration) => {
                    self.lower_filter_expression(document, declaration, &node.value.condition)?
                }
                None => {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::PackageMember,
                        node.span.clone(),
                    );
                }
            },
            PackageBodyElement::Package(node) => self.lower_package(document, owner, node)?,
            PackageBodyElement::LibraryPackage(node) => {
                self.lower_library_package(document, owner, node)?
            }
            PackageBodyElement::Import(node) => self.lower_import(document, owner, node)?,
            PackageBodyElement::PartDef(node) => self.lower_part_def(document, owner, node)?,
            PackageBodyElement::PartUsage(node) => self.lower_part_usage(document, owner, node)?,
            PackageBodyElement::AttributeUsage(node) => {
                self.lower_attribute_usage(document, owner, node)?
            }
            PackageBodyElement::PortDef(node) => self.lower_port_def(document, owner, node)?,
            PackageBodyElement::InterfaceDef(node) => {
                self.lower_interface_def(document, owner, node)?
            }
            PackageBodyElement::AliasDef(node) => self.lower_alias_def(document, owner, node)?,
            PackageBodyElement::AttributeDef(node) => {
                self.lower_attribute_def(document, owner, node)?
            }
            PackageBodyElement::EnumDef(node) => self.lower_enum_def(document, owner, node)?,
            PackageBodyElement::EnumerationUsage(node) => {
                self.lower_enum_usage(document, owner, node)?
            }
            PackageBodyElement::ActionDef(node) => self.lower_action_def(document, owner, node)?,
            PackageBodyElement::ActionUsage(node) => {
                self.lower_action_usage(document, owner, node)?
            }
            PackageBodyElement::RequirementDef(node) => {
                self.lower_requirement_def(document, owner, node)?
            }
            PackageBodyElement::RequirementUsage(node) => {
                self.lower_requirement_usage(document, owner, node)?
            }
            PackageBodyElement::Satisfy(node) => match owner {
                Some(owner) => {
                    self.lower_satisfy(document, owner, UnsupportedFamily::PackageMember, node)?
                }
                None => self.push_unsupported(
                    document,
                    UnsupportedFamily::PackageMember,
                    node.span.clone(),
                ),
            },
            PackageBodyElement::UseCaseDef(node) => {
                self.lower_use_case_def(document, owner, node)?
            }
            PackageBodyElement::Actor(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::StateDef(node) => self.lower_state_def(document, owner, node)?,
            PackageBodyElement::StateUsage(node) => {
                self.lower_state_usage(document, owner, node)?
            }
            PackageBodyElement::ItemDef(node) => self.lower_item_def(document, owner, node)?,
            PackageBodyElement::MetadataDef(node) => {
                self.lower_metadata_def(document, owner, node)?
            }
            PackageBodyElement::IndividualDef(node) => {
                self.lower_individual_def(document, owner, node)?
            }
            PackageBodyElement::ConstraintDef(node) => {
                self.lower_constraint_def(document, owner, node)?
            }
            PackageBodyElement::ConstraintUsage(node) => {
                self.lower_constraint_usage(document, owner, node)?
            }
            PackageBodyElement::CalcDef(node) => self.lower_calc_def(document, owner, node)?,
            PackageBodyElement::ViewDef(node) => self.lower_view_def(document, owner, node)?,
            PackageBodyElement::ViewpointDef(node) => {
                self.lower_viewpoint_def(document, owner, node)?
            }
            PackageBodyElement::RenderingDef(node) => {
                self.lower_rendering_def(document, owner, node)?
            }
            PackageBodyElement::ViewUsage(node) => self.lower_view_usage(document, owner, node)?,
            PackageBodyElement::ViewpointUsage(node) => {
                self.lower_viewpoint_usage(document, owner, node)?
            }
            PackageBodyElement::RenderingUsage(node) => {
                self.lower_rendering_usage(document, owner, node)?
            }
            PackageBodyElement::ConnectionDef(node) => {
                self.lower_connection_def(document, owner, node)?
            }
            PackageBodyElement::OccurrenceDef(node) => {
                self.lower_occurrence_def(document, owner, node)?
            }
            PackageBodyElement::OccurrenceUsage(node) => {
                self.lower_occurrence_usage(document, owner, node)?
            }
            PackageBodyElement::Dependency(node) => self.lower_dependency(document, owner, node)?,
            PackageBodyElement::AllocationDef(node) => {
                self.lower_allocation_def(document, owner, node)?
            }
            PackageBodyElement::AllocationUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::FlowDef(node) => self.lower_flow_def(document, owner, node)?,
            PackageBodyElement::FlowUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ConcernUsage(node) => {
                self.lower_concern_usage(document, owner, node)?
            }
            PackageBodyElement::CaseDef(node) => self.lower_case_def(document, owner, node)?,
            PackageBodyElement::CaseUsage(node) => self.lower_case_usage(document, owner, node)?,
            PackageBodyElement::AnalysisCaseDef(node) => {
                self.lower_analysis_case_def(document, owner, node)?
            }
            PackageBodyElement::AnalysisCaseUsage(node) => {
                self.lower_analysis_case_usage(document, owner, node)?
            }
            PackageBodyElement::VerificationCaseDef(node) => {
                self.lower_verification_case_def(document, owner, node)?
            }
            PackageBodyElement::VerificationCaseUsage(node) => {
                self.lower_verification_case_usage(document, owner, node)?
            }
            PackageBodyElement::UseCaseUsage(node) => {
                self.lower_use_case_usage(document, owner, node)?
            }
            PackageBodyElement::FeatureDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ClassifierDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::KermlSemanticDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::KermlFeatureDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::KermlClassifier(node) => {
                self.lower_kerml_classifier_decl(document, owner, node)?
            }
            PackageBodyElement::KermlInvariant(node) => {
                self.lower_kerml_invariant_member(document, owner, node)?
            }
            PackageBodyElement::KermlConnector(node) => match owner {
                Some(owner) => self.lower_kerml_connector_member(document, owner, node)?,
                // A connector at the root of a document has no type to be featured by, so there
                // is no owner to source its ends at; the `connect` statement arm above defers the
                // same shape for the same reason.
                None => self.push_unsupported(
                    document,
                    UnsupportedFamily::PackageMember,
                    node.span.clone(),
                ),
            },
            PackageBodyElement::KermlRelationship(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::KermlFeatureMember(node) => self.lower_kerml_feature_member(
                document,
                owner,
                UnsupportedFamily::PackageMember,
                node,
            )?,
            PackageBodyElement::ExtendedLibraryDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ItemUsage(node) => self.lower_item_usage(document, owner, node)?,
            PackageBodyElement::MetadataUsage(node) => {
                self.lower_metadata_usage(document, owner, node)?
            }
            PackageBodyElement::PortUsage(node) => self.lower_port_usage(document, owner, node)?,
            PackageBodyElement::ConnectionUsage(node) => {
                self.lower_connection_usage(document, owner, node)?
            }
            PackageBodyElement::InterfaceUsage(node) => {
                self.lower_interface_usage(document, owner, node)?
            }
            PackageBodyElement::Ref(node) => self.lower_ref_decl(document, owner, node)?,
            PackageBodyElement::MetadataKeywordUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::Connect(node) => {
                if let Some(owner) = owner {
                    self.lower_bare_connect(document, owner, node)?;
                } else {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::PackageMember,
                        node.span.clone(),
                    );
                }
            }
            PackageBodyElement::DefaultReferenceUsage(node) => self.lower_default_reference_usage(
                document,
                owner,
                UnsupportedFamily::PackageMember,
                node,
            )?,
            PackageBodyElement::AssertConstraint(node) => match owner {
                Some(declaration) => self.lower_assert_constraint_member(
                    document,
                    declaration,
                    UnsupportedFamily::PackageMember,
                    node,
                )?,
                None => self.push_unsupported(
                    document,
                    UnsupportedFamily::PackageMember,
                    node.span.clone(),
                ),
            },
            PackageBodyElement::KermlBareDeclaration(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::MetadataAnnotation(node) => match owner {
                Some(owner) => self.lower_metadata_annotation(document, owner, node)?,
                None => self.push_unsupported(
                    document,
                    UnsupportedFamily::PackageMember,
                    node.span.clone(),
                ),
            },
            PackageBodyElement::PerformUsage(node) => self.lower_perform(document, owner, node)?,
            PackageBodyElement::BindingConnectorUsage(node) => match owner {
                Some(owner) => self.lower_binding_connector_usage(document, owner, node)?,
                None => self.push_unsupported(
                    document,
                    UnsupportedFamily::PackageMember,
                    node.span.clone(),
                ),
            },
            PackageBodyElement::ClassDef(node) => self.lower_class_def(document, owner, node)?,
            PackageBodyElement::Succession(node) => match owner {
                Some(owner) => {
                    self.lower_first_stmt(document, owner, UnsupportedFamily::PackageMember, node)?
                }
                None => self.push_unsupported(
                    document,
                    UnsupportedFamily::PackageMember,
                    node.span.clone(),
                ),
            },
            PackageBodyElement::ExhibitState(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::IncludeUseCase(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ExtendedDefinition(node) => {
                self.lower_extended_definition(document, owner, node)?
            }
        }
        Ok(())
    }

    fn lower_import(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<Import>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Import,
            None,
            node.span.clone(),
            // An import declares no name, modifier, multiplicity, or direction of its own; its
            // recursive/wildcard/filter facts belong to the authored import reference below.
            DeclarationFacts::none(),
        )?;
        let membership = &node.value.membership;
        self.push_membership(
            declaration,
            MembershipKind::Import,
            self.member_visibility(membership, ParserMembershipKind::Import)?,
            membership.span.clone(),
        )?;
        let (kind, flags) = match &node.value.target.shape {
            ImportShape::Membership { recursive_suffix } => (
                ReferenceKind::MembershipImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    ..RelationshipFlags::default()
                },
            ),
            ImportShape::Namespace {
                recursive_suffix, ..
            } => (
                ReferenceKind::NamespaceImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    wildcard: true,
                    ..RelationshipFlags::default()
                },
            ),
            ImportShape::Filter {
                recursive_suffix, ..
            } => (
                ReferenceKind::FilterImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    ..RelationshipFlags::default()
                },
            ),
        };
        let import = Some(AuthoredImportFacts {
            shape: match &node.value.target.shape {
                ImportShape::Membership { .. } => AuthoredImportShape::Membership,
                ImportShape::Namespace { .. } => AuthoredImportShape::Namespace,
                ImportShape::Filter { .. } => AuthoredImportShape::Filter,
            },
            recursive: flags.recursive,
        });
        self.push_reference(PendingReference {
            source: declaration,
            kind,
            document,
            local: node.value.target.reference,
            flags,
            span: node.value.target.span.clone(),
            import,
        })?;
        if let Some(elements) = &node.value.body_elements {
            self.lower_relationship_body_elements(document, Some(declaration), elements)?;
        }
        Ok(())
    }

    fn lower_part_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PartDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let (is_abstract, variation) =
            definition_prefix_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let PartDefBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                match &element.value {
                    PartDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PartDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    PartDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PartDefBodyElement::PartUsage(part) => {
                        self.lower_part_usage(document, Some(declaration), part)?;
                    }
                    PartDefBodyElement::PartDef(part) => {
                        self.lower_part_def(document, Some(declaration), part)?;
                    }
                    PartDefBodyElement::Import(import) => {
                        self.lower_import(document, Some(declaration), import)?;
                    }
                    PartDefBodyElement::EnumDef(enum_def) => {
                        self.lower_enum_def(document, Some(declaration), enum_def)?;
                    }
                    PartDefBodyElement::EnumerationUsage(enum_usage) => {
                        self.lower_enum_usage(document, Some(declaration), enum_usage)?;
                    }
                    PartDefBodyElement::RequirementDef(requirement_def) => {
                        self.lower_requirement_def(document, Some(declaration), requirement_def)?;
                    }
                    PartDefBodyElement::AnalysisCaseDef(analysis_case_def) => {
                        self.lower_analysis_case_def(
                            document,
                            Some(declaration),
                            analysis_case_def,
                        )?;
                    }
                    PartDefBodyElement::CaseDef(case_def) => {
                        self.lower_case_def(document, Some(declaration), case_def)?;
                    }
                    PartDefBodyElement::CaseUsage(case_usage) => {
                        self.lower_case_usage(document, Some(declaration), case_usage)?;
                    }
                    PartDefBodyElement::AnalysisCaseUsage(analysis_case_usage) => {
                        self.lower_analysis_case_usage(
                            document,
                            Some(declaration),
                            analysis_case_usage,
                        )?;
                    }
                    PartDefBodyElement::VerificationCaseDef(verification_case_def) => {
                        self.lower_verification_case_def(
                            document,
                            Some(declaration),
                            verification_case_def,
                        )?;
                    }
                    PartDefBodyElement::UseCaseDef(use_case_def) => {
                        self.lower_use_case_def(document, Some(declaration), use_case_def)?;
                    }
                    PartDefBodyElement::RequirementUsage(requirement_usage) => {
                        self.lower_requirement_usage(
                            document,
                            Some(declaration),
                            requirement_usage,
                        )?;
                    }
                    PartDefBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    PartDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PartDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    PartDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PartDefBodyElement::MetadataDef(metadata_def) => {
                        self.lower_metadata_def(document, Some(declaration), metadata_def)?;
                    }
                    PartDefBodyElement::MetadataUsage(metadata_usage) => {
                        self.lower_metadata_usage(document, Some(declaration), metadata_usage)?;
                    }
                    PartDefBodyElement::ActionDef(action_def) => {
                        self.lower_action_def(document, Some(declaration), action_def)?;
                    }
                    PartDefBodyElement::ActionUsage(action_usage) => {
                        self.lower_action_usage(document, Some(declaration), action_usage)?;
                    }
                    PartDefBodyElement::StateDef(state_def) => {
                        self.lower_state_def(document, Some(declaration), state_def)?;
                    }
                    PartDefBodyElement::StateUsage(state_usage) => {
                        self.lower_state_usage(document, Some(declaration), state_usage)?;
                    }
                    PartDefBodyElement::ConnectionDef(connection_def) => {
                        self.lower_connection_def(document, Some(declaration), connection_def)?;
                    }
                    PartDefBodyElement::InterfaceDef(interface_def) => {
                        self.lower_interface_def(document, Some(declaration), interface_def)?;
                    }
                    PartDefBodyElement::ViewDef(view_def) => {
                        self.lower_view_def(document, Some(declaration), view_def)?;
                    }
                    PartDefBodyElement::ViewpointDef(viewpoint_def) => {
                        self.lower_viewpoint_def(document, Some(declaration), viewpoint_def)?;
                    }
                    PartDefBodyElement::RenderingDef(rendering_def) => {
                        self.lower_rendering_def(document, Some(declaration), rendering_def)?;
                    }
                    PartDefBodyElement::AllocationDef(allocation_def) => {
                        self.lower_allocation_def(document, Some(declaration), allocation_def)?;
                    }
                    PartDefBodyElement::FlowDef(flow_def) => {
                        self.lower_flow_def(document, Some(declaration), flow_def)?;
                    }
                    PartDefBodyElement::Connection(connection_usage) => {
                        self.lower_connection_usage(document, Some(declaration), connection_usage)?;
                    }
                    PartDefBodyElement::OccurrenceDef(occurrence_def) => {
                        self.lower_occurrence_def(document, Some(declaration), occurrence_def)?;
                    }
                    PartDefBodyElement::OccurrenceUsage(occurrence_usage) => {
                        self.lower_occurrence_usage(document, Some(declaration), occurrence_usage)?;
                    }
                    PartDefBodyElement::InterfaceUsage(interface_usage) => {
                        self.lower_interface_usage(document, Some(declaration), interface_usage)?;
                    }
                    PartDefBodyElement::ViewUsage(view_usage) => {
                        self.lower_view_usage(document, Some(declaration), view_usage)?;
                    }
                    PartDefBodyElement::RenderingUsage(node) => {
                        self.lower_rendering_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::UseCaseUsage(node) => {
                        self.lower_use_case_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::VerificationCaseUsage(node) => {
                        self.lower_verification_case_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::ConstraintDef(constraint_def) => {
                        self.lower_constraint_def(document, Some(declaration), constraint_def)?;
                    }
                    PartDefBodyElement::ConstraintUsage(constraint_usage) => {
                        self.lower_constraint_usage(document, Some(declaration), constraint_usage)?;
                    }
                    PartDefBodyElement::CalcDef(calc_def) => {
                        self.lower_calc_def(document, Some(declaration), calc_def)?;
                    }
                    PartDefBodyElement::CalcUsage(calc_usage) => {
                        self.lower_calc_usage(document, Some(declaration), calc_usage)?;
                    }
                    PartDefBodyElement::AliasDef(alias_def) => {
                        self.lower_alias_def(document, Some(declaration), alias_def)?;
                    }
                    PartDefBodyElement::Perform(perform) => {
                        self.lower_perform(document, Some(declaration), perform)?;
                    }
                    PartDefBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    PartDefBodyElement::Comment(node) => {
                        self.record_comment_annotation(declaration, node)?;
                    }
                    PartDefBodyElement::MetadataAnnotation(node) => {
                        self.lower_metadata_annotation(document, declaration, node)?;
                    }
                    PartDefBodyElement::Satisfy(node) => {
                        self.lower_satisfy(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::Allocate(node) => {
                        self.lower_allocate(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::Bind(node) => {
                        self.lower_bind(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::FirstStmt(first_stmt) => {
                        self.lower_first_stmt(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            first_stmt,
                        )?;
                    }
                    PartDefBodyElement::VariantUsage(node) => {
                        self.lower_variant_usage(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::AssertConstraint(node) => self
                        .lower_assert_constraint_member(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?,
                    PartDefBodyElement::Ref(node) => {
                        self.lower_ref_decl(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::DefaultReferenceUsage(node) => {
                        self.lower_default_reference_usage(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::Dependency(node) => {
                        self.lower_dependency(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::Connect(node) => {
                        self.lower_bare_connect(document, declaration, node)?;
                    }
                    PartDefBodyElement::ViewpointUsage(node) => {
                        self.lower_viewpoint_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::KermlClassifier(node) => {
                        self.lower_kerml_classifier_decl(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::Annotation(_)
                    | PartDefBodyElement::MetadataKeywordUsage(_)
                    | PartDefBodyElement::FlowUsage(_)
                    | PartDefBodyElement::ExhibitState(_)
                    | PartDefBodyElement::AllocationUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::PartDefinitionMember,
                        element.span.clone(),
                    ),
                    PartDefBodyElement::UnsupportedMember(node) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ParserUnsupported,
                        node.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    fn lower_part_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PartUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let (is_abstract, variation) =
            definition_prefix_modifiers(node.value.usage_prefix.as_ref());
        let facts = DeclarationFacts {
            short_name,
            modifiers: DeclarationModifiers {
                is_abstract,
                variation,
                individual: node.value.is_individual,
                derived: node.value.is_derived,
                reference: node.value.is_reference,
                constant: node.value.is_constant,
                ordered: node.value.ordered,
                // `ast::PartUsage` has no `nonunique` field, unlike its sibling usages; see
                // planning/UPSTREAM_PARSER_GAPS.md.
                ..DeclarationModifiers::default()
            },
            direction: direction_fact(node.value.direction.as_ref()),
            multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
            ..DeclarationFacts::none()
        };
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartUsage,
            name,
            node.span.clone(),
            facts,
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship_impl(
                document,
                declaration,
                relationship,
                matches!(node.value.usage_prefix, Some(DefinitionPrefix::Variation)),
            )?;
        }
        if let Some((relationship, _)) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let PartUsageBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                self.lower_part_usage_body_element(
                    document,
                    declaration,
                    UnsupportedFamily::PartUsageMember,
                    element,
                )?;
            }
        }
        Ok(())
    }

    /// Lowers one `PartUsageBodyElement` found inside a `part` usage/def body, and reused
    /// verbatim by `Bind.body_elements` (BNF `Bind`'s optional braced body, `ast::structure::
    /// Bind.body_elements: Vec<Node<PartUsageBodyElement>>` -- the same part-usage member set
    /// `PartUsageBody` uses, per its own doc comment) rather than the blanket "every element is
    /// unsupported" fallback `lower_bind` used before this dispatcher was factored out, and by
    /// `ref { ... }` bodies, which hold this same member set upstream (`RefBody =
    /// Body<PartUsageBodyElement>`). See `lower_part_usage`'s own doc comment for the per-arm
    /// recognized/unsupported shape.
    ///
    /// `family` names the owning body in the unsupported facts this dispatch produces, so a `ref`
    /// body's unmodeled members stay distinguishable from a `part` usage body's.
    fn lower_part_usage_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        element: &Node<PartUsageBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            PartUsageBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            PartUsageBodyElement::AttributeUsage(attribute) => {
                self.lower_attribute_usage(document, Some(owner), attribute)?;
            }
            PartUsageBodyElement::PartUsage(part) => {
                self.lower_part_usage(document, Some(owner), part)?;
            }
            PartUsageBodyElement::Import(import) => {
                self.lower_import(document, Some(owner), import)?;
            }
            PartUsageBodyElement::EnumDef(enum_def) => {
                self.lower_enum_def(document, Some(owner), enum_def)?;
            }
            PartUsageBodyElement::EnumerationUsage(enum_usage) => {
                self.lower_enum_usage(document, Some(owner), enum_usage)?;
            }
            PartUsageBodyElement::RequirementDef(requirement_def) => {
                self.lower_requirement_def(document, Some(owner), requirement_def)?;
            }
            PartUsageBodyElement::AnalysisCaseDef(analysis_case_def) => {
                self.lower_analysis_case_def(document, Some(owner), analysis_case_def)?;
            }
            PartUsageBodyElement::AnalysisCaseUsage(analysis_case_usage) => {
                self.lower_analysis_case_usage(document, Some(owner), analysis_case_usage)?;
            }
            PartUsageBodyElement::RequirementUsage(requirement_usage) => {
                self.lower_requirement_usage(document, Some(owner), requirement_usage)?;
            }
            PartUsageBodyElement::PortDef(port_def) => {
                self.lower_port_def(document, Some(owner), port_def)?;
            }
            PartUsageBodyElement::PortUsage(port_usage) => {
                self.lower_port_usage(document, Some(owner), port_usage)?;
            }
            PartUsageBodyElement::ItemDef(item_def) => {
                self.lower_item_def(document, Some(owner), item_def)?;
            }
            PartUsageBodyElement::ItemUsage(item_usage) => {
                self.lower_item_usage(document, Some(owner), item_usage)?;
            }
            PartUsageBodyElement::MetadataDef(metadata_def) => {
                self.lower_metadata_def(document, Some(owner), metadata_def)?;
            }
            PartUsageBodyElement::MetadataUsage(metadata_usage) => {
                self.lower_metadata_usage(document, Some(owner), metadata_usage)?;
            }
            PartUsageBodyElement::ActionUsage(action_usage) => {
                self.lower_action_usage(document, Some(owner), action_usage)?;
            }
            PartUsageBodyElement::StateDef(state_def) => {
                self.lower_state_def(document, Some(owner), state_def)?;
            }
            PartUsageBodyElement::StateUsage(state_usage) => {
                self.lower_state_usage(document, Some(owner), state_usage)?;
            }
            PartUsageBodyElement::ConnectionDef(connection_def) => {
                self.lower_connection_def(document, Some(owner), connection_def)?;
            }
            PartUsageBodyElement::Connection(connection_usage) => {
                self.lower_connection_usage(document, Some(owner), connection_usage)?;
            }
            PartUsageBodyElement::OccurrenceDef(occurrence_def) => {
                self.lower_occurrence_def(document, Some(owner), occurrence_def)?;
            }
            PartUsageBodyElement::OccurrenceUsage(occurrence_usage) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence_usage)?;
            }
            PartUsageBodyElement::FlowDef(flow_def) => {
                self.lower_flow_def(document, Some(owner), flow_def)?;
            }
            PartUsageBodyElement::InterfaceUsage(interface_usage) => {
                self.lower_interface_usage(document, Some(owner), interface_usage)?;
            }
            PartUsageBodyElement::ConstraintDef(constraint_def) => {
                self.lower_constraint_def(document, Some(owner), constraint_def)?;
            }
            PartUsageBodyElement::ConstraintUsage(constraint_usage) => {
                self.lower_constraint_usage(document, Some(owner), constraint_usage)?;
            }
            PartUsageBodyElement::CalcDef(calc_def) => {
                self.lower_calc_def(document, Some(owner), calc_def)?;
            }
            PartUsageBodyElement::CalcUsage(calc_usage) => {
                self.lower_calc_usage(document, Some(owner), calc_usage)?;
            }
            PartUsageBodyElement::AliasDef(alias_def) => {
                self.lower_alias_def(document, Some(owner), alias_def)?;
            }
            PartUsageBodyElement::Perform(perform) => {
                self.lower_perform(document, Some(owner), perform)?;
            }
            PartUsageBodyElement::Annotating(member) => {
                self.lower_annotating_member(document, Some(owner), member)?;
            }
            PartUsageBodyElement::KermlClassifier(node) => {
                self.lower_kerml_classifier_decl(document, Some(owner), node)?;
            }
            PartUsageBodyElement::Satisfy(node) => {
                self.lower_satisfy(document, owner, family, node)?;
            }
            PartUsageBodyElement::VariantUsage(node) => {
                self.lower_variant_usage(document, owner, family, node)?;
            }
            PartUsageBodyElement::Allocate(node) => {
                self.lower_allocate(document, owner, family, node)?;
            }
            PartUsageBodyElement::Bind(node) => {
                self.lower_bind(document, owner, family, node)?;
            }
            PartUsageBodyElement::AssertConstraint(node) => {
                self.lower_assert_constraint_member(document, owner, family, node)?
            }
            PartUsageBodyElement::Ref(node) => {
                self.lower_ref_decl(document, Some(owner), node)?;
            }
            PartUsageBodyElement::DefaultReferenceUsage(node) => {
                self.lower_default_reference_usage(document, Some(owner), family, node)?;
            }
            PartUsageBodyElement::Connect(node) => {
                self.lower_bare_connect(document, owner, node)?;
            }
            PartUsageBodyElement::UseCaseUsage(node) => {
                self.lower_use_case_usage(document, Some(owner), node)?;
            }
            PartUsageBodyElement::VerificationCaseUsage(node) => {
                self.lower_verification_case_usage(document, Some(owner), node)?;
            }
            PartUsageBodyElement::Annotation(_)
            | PartUsageBodyElement::FlowUsage(_)
            | PartUsageBodyElement::SuccessionUsage(_)
            | PartUsageBodyElement::MetadataKeywordUsage(_)
            | PartUsageBodyElement::IncludeUseCase(_) => {
                self.push_unsupported(document, family, element.span.clone())
            }
        }
        Ok(())
    }

    /// Dispatches a shared KerML `RelationshipBody`-shaped element list (BNF `RelationshipBody :
    /// Relationship = ';' | '{' (ownedRelationship += OwnedAnnotation)* '}'`, `ast::
    /// RelationshipBodyElement`), used verbatim by `Import`/`Dependency`/plain `connect`
    /// statements/`alias ... for ...` bodies: recovery nodes, the whole annotating production
    /// bound to `annotated` (`lower_annotating_member`), and an owned KerML `feature` member
    /// (`dependency z to x, y { feature e; }`, the BNF's `ownedRelatedElement`), which is lowered
    /// by the same `lower_kerml_feature_member` owner every other KerML feature member uses.
    ///
    /// `annotated` is `None` only where the construct owning the body mints no declaration of its
    /// own -- a `connect a to b { ... }` statement lowers its ends directly against the enclosing
    /// declaration -- so there is no element the annotation belongs to and attributing it to the
    /// enclosing type would misreport it.
    fn lower_relationship_body_elements(
        &mut self,
        document: DocumentId,
        annotated: Option<DeclarationId>,
        elements: &[Node<RelationshipBodyElement>],
    ) -> Result<(), ConstructionError> {
        for element in elements {
            match &element.value {
                RelationshipBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                RelationshipBodyElement::Annotating(member) => {
                    self.lower_annotating_member(document, annotated, member)?;
                }
                RelationshipBodyElement::KermlFeature(node) => self.lower_kerml_feature_member(
                    document,
                    annotated,
                    UnsupportedFamily::RelationshipBodyMember,
                    node,
                )?,
            }
        }
        Ok(())
    }

    /// Lowers a `ref <name>: <Type>;` non-owning referential feature (BNF `ReferenceUsage`,
    /// `ast::RefDecl`), reused verbatim across part/attribute/action/state/connection/interface/
    /// package bodies. Mirrors `lower_part_usage`'s ownership/typing/redefines/subsets shape (`ref`
    /// is a `FeatureMembership` like any other usage; see `ast::connector::ref_decl`'s
    /// `Membership::feature` construction), since `RefDecl` carries the same structured
    /// `typing`/`redefines`/`subsets` clauses as `PartUsage`/`AttributeUsage`. Its body is the
    /// general usage-member set (`RefBody = Body<PartUsageBodyElement>`, `UsageBody =
    /// DefinitionBody` per SysML 8.2.2.6.2) whatever declaration owns it, so it walks through the
    /// shared `lower_part_usage_body_element` dispatcher under
    /// `UnsupportedFamily::ReferenceUsageMember`.
    fn lower_ref_decl(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<RefDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ReferenceUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    // The `ref` keyword is this declaration's own form, not a prefix modifier on
                    // some other usage, so `reference` stays false here.
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        for element in node.value.body.members() {
            self.lower_part_usage_body_element(
                document,
                declaration,
                UnsupportedFamily::ReferenceUsageMember,
                element,
            )?;
        }
        Ok(())
    }

    fn lower_attribute_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AttributeUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let (is_abstract, variation) =
            definition_prefix_modifiers(node.value.usage_prefix.as_ref());
        let facts = DeclarationFacts {
            short_name,
            modifiers: DeclarationModifiers {
                is_abstract,
                variation,
                derived: node.value.is_derived,
                end: node.value.is_end,
                reference: node.value.is_reference,
                constant: node.value.is_constant,
                ordered: node.value.ordered,
                nonunique: node.value.nonunique,
                ..DeclarationModifiers::default()
            },
            direction: direction_fact(node.value.direction.as_ref()),
            multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
            ..DeclarationFacts::none()
        };
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeUsage,
            name,
            node.span.clone(),
            facts,
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.crosses {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.intersects {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_default_value(document, declaration, node.value.value.as_ref())?;
        self.lower_attribute_body(document, declaration, &node.value.body)?;
        Ok(())
    }

    /// Value-assignment lowering for a `name = <Expression>;` shape, shared by every context that
    /// carries one: an attribute def/usage default value AND explicit redefinition value
    /// (`attribute mass = 5;` / `attribute :>> status = RequirementStatusKind::approved;`, both
    /// typed as `FeatureValue` -- the parser does not distinguish "default" from "redefinition
    /// value" syntactically, both are just an attribute's own optional `=` clause), a metadata
    /// annotation body override (`@Safety{isMandatory = true;}`, also `FeatureValue` on a nested
    /// `AttributeUsage`), and a parameter default value (`out v_out : SpeedValue = vel.v;`, also
    /// `FeatureValue`). All four contexts share the exact same typed-AST shape (a name, an `=`,
    /// and an `Expression`), so this one helper handles all of them: reuses the full
    /// `classify_constraint_expression`/`lower_constraint_expression` machinery (literal,
    /// feature-ref, member-access, arithmetic/comparison/logical `BinaryOp`, invocation) rather
    /// than a bespoke literal-only walk, so `attribute mass = length * width;`, `attribute :>>
    /// status = RequirementStatusKind::approved;`, and `attribute f = other.value;` all resolve
    /// their operands and, where possible, evaluate to a genuine constant via `compute_evaluation`
    /// -- a value that is resolved but not itself constant publishes `NonConstant`, never a
    /// fabricated value (see `EvaluatedValue`). `family` selects which diagnostic an unsupported
    /// expression shape falls through to.
    fn lower_value_assignment(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        family: UnsupportedFamily,
        value: Option<&Node<FeatureValue>>,
    ) -> Result<(), ConstructionError> {
        let Some(feature_value) = value else {
            return Ok(());
        };
        self.record_feature_value(declaration, feature_value)?;
        let expression = &feature_value.value.expression;
        self.push_evaluation_fact(
            declaration,
            classify_constraint_expression(&expression.value),
        );
        self.lower_constraint_expression(document, declaration, family, expression)
    }

    fn lower_attribute_default_value(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        value: Option<&Node<FeatureValue>>,
    ) -> Result<(), ConstructionError> {
        self.lower_value_assignment(
            document,
            declaration,
            UnsupportedFamily::AttributeMember,
            value,
        )
    }

    fn lower_attribute_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AttributeDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                // `ast::AttributeDef` carries no `multiplicity` field at all, unlike
                // `AttributeUsage`; see planning/UPSTREAM_PARSER_GAPS.md.
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_default_value(document, declaration, node.value.value.as_ref())?;
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    fn lower_attribute_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &AttributeBody,
    ) -> Result<(), ConstructionError> {
        let AttributeBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                AttributeBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                AttributeBodyElement::Doc(node) => {
                    self.record_doc_comment(owner, node)?;
                }
                AttributeBodyElement::AttributeDef(attribute) => {
                    self.lower_attribute_def(document, Some(owner), attribute)?;
                }
                AttributeBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                AttributeBodyElement::PartUsage(part) => {
                    self.lower_part_usage(document, Some(owner), part)?;
                }
                AttributeBodyElement::OccurrenceUsage(occurrence_usage) => {
                    self.lower_occurrence_usage(document, Some(owner), occurrence_usage)?;
                }
                AttributeBodyElement::ItemUsage(item_usage) => {
                    self.lower_item_usage(document, Some(owner), item_usage)?;
                }
                AttributeBodyElement::AssertConstraint(node) => self
                    .lower_assert_constraint_member(
                        document,
                        owner,
                        UnsupportedFamily::AttributeMember,
                        node,
                    )?,
                AttributeBodyElement::RefDecl(node) => {
                    self.lower_ref_decl(document, Some(owner), node)?;
                }
                AttributeBodyElement::Connect(node) => {
                    self.lower_bare_connect(document, owner, node)?;
                }
                AttributeBodyElement::Bind(node) => {
                    self.lower_bind(document, owner, UnsupportedFamily::AttributeMember, node)?;
                }
                AttributeBodyElement::Connection(node) => {
                    self.lower_connection_usage(document, Some(owner), node)?;
                }
                AttributeBodyElement::ConstraintUsage(node) => {
                    self.lower_constraint_usage(document, Some(owner), node)?;
                }
                AttributeBodyElement::CalcDef(node) => {
                    self.lower_calc_def(document, Some(owner), node)?;
                }
                AttributeBodyElement::CalcUsage(node) => {
                    self.lower_calc_usage(document, Some(owner), node)?;
                }
                AttributeBodyElement::ClassDef(node) => {
                    self.lower_class_def(document, Some(owner), node)?;
                }
                AttributeBodyElement::KermlClassifier(node) => {
                    self.lower_kerml_classifier_decl(document, Some(owner), node)?;
                }
                AttributeBodyElement::KermlConnector(node) => {
                    self.lower_kerml_connector_member(document, owner, node)?;
                }
                AttributeBodyElement::KermlFeature(node) => self.lower_kerml_feature_member(
                    document,
                    Some(owner),
                    UnsupportedFamily::AttributeMember,
                    node,
                )?,
                AttributeBodyElement::Invariant(node) => {
                    self.lower_kerml_invariant_member(document, Some(owner), node)?;
                }
                AttributeBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
                AttributeBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::AttributeMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers an `enum def` (BNF EnumerationDefinition), mirroring `lower_part_def`: ownership,
    /// membership, an optional `:>`/`:` specialization relationship (an enum def may specialize
    /// another enum def or an attribute def), and each owned enumeration literal as its own typed
    /// declaration.
    fn lower_enum_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<EnumDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::EnumerationDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let EnumerationBody::Brace { elements, .. } = &node.value.body {
            for value in elements {
                self.lower_enumerated_value(document, declaration, value)?;
            }
        }
        Ok(())
    }

    /// Lowers one `enum <name>;` value owned by an `enum def` body (BNF EnumeratedValue) into its
    /// own declaration. Any inline body / `= expr` initializer is discarded by the parser itself
    /// (only the name and its span survive), so there is no nested body to lower here.
    fn lower_enumerated_value(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<sysml_v2_parser_next::ast::EnumeratedValue>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::EnumerationLiteral,
            name,
            node.span.clone(),
            // `ast::EnumeratedValue` retains only a name and its span.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )
    }

    /// Lowers a package/definition/usage-level `enum` feature member (BNF EnumerationUsage), e.g.
    /// `enum color : ColorKind;`, mirroring `lower_attribute_usage`. `type_name` is a bare
    /// `QualifiedReferenceId` (not a `TypingRelationship` node), so its `FeatureTyping` reference
    /// is pushed directly rather than through `lower_typing_relationship`.
    fn lower_enum_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserEnumerationUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::EnumerationUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    end: node.value.is_end,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers an `item def` (BNF ItemDefinition), mirroring `lower_part_def`: ownership,
    /// membership, and an optional `:>` specialization relationship. `ItemDef`'s body is a plain
    /// `AttributeBody` (shared with `AttributeDef`/`AttributeUsage`), not a `PartDefBody`, so its
    /// owned members are lowered through the existing `lower_attribute_body` rather than a
    /// dedicated item-specific body walker.
    fn lower_item_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ItemDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ItemDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a KerML `class def` (BNF ClassDefinition), mirroring `lower_item_def`: ownership,
    /// membership, and an optional `:>` specialization relationship. `ClassDef`'s body is a plain
    /// `AttributeBody`, exactly the same shape `ItemDef` has, so owned members are lowered through
    /// the existing `lower_attribute_body`. There is no separate KerML "class usage" form in the
    /// grammar -- only this def-level construct exists.
    fn lower_class_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ClassDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ClassDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a bodied KerML classifier declaration (`KermlClassifierDecl`), mirroring
    /// `lower_class_def`: ownership, an optional `specializes` relationship, and owned-member
    /// structure. Its body shares the `CalcDefBody` grammar (parameters, `return` results,
    /// feature members, invariants, expressions, documentation), the same shape `calc def`
    /// bodies use, so it is walked through the existing `lower_calc_def_body` rather than
    /// `lower_attribute_body`. `is_abstract`/`is_all`/`multiplicity`/`type_relationships` and the
    /// specific `KermlClassifierKeyword` spelling are not modeled as distinct facts here (see
    /// `DeclarationKind::KermlClassifier`).
    fn lower_kerml_classifier_decl(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<KermlClassifierDecl>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            kerml_classifier_kind(&node.value.keyword),
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    all: node.value.is_all,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a bare/bodied KerML feature member (`KermlFeatureMember`, gap #14: previously an
    /// opaque `FeatureDecl { keyword, text }` raw-text fallback, now a fully typed shape),
    /// mirroring `lower_ref_decl`: ownership, membership, an optional `:` typing target, and
    /// `subsets`/`redefines` relationships. Its `= expr` value, when present, is classified and
    /// lowered through the same `classify_calc_expression`/`lower_calc_expression` pipeline
    /// `lower_parameter_declaration`/`lower_return_decl` use. Its body shares the `CalcDefBody`
    /// grammar, so owned members are walked through the existing `lower_calc_def_body`. See
    /// `DeclarationKind::KermlFeature` for the facts intentionally left unmodeled.
    fn lower_kerml_feature_member(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        family: UnsupportedFamily,
        node: &Node<KermlFeatureMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            kerml_feature_kind(&node.value.kind),
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    derived: node.value.is_derived,
                    end: node.value.is_end,
                    all: node.value.is_all,
                    composite: node.value.is_composite,
                    portion: node.value.is_portion,
                    var: node.value.is_var,
                    member: node.value.is_member,
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(declaration, classify_calc_expression(&expression.value));
            self.lower_calc_expression(document, declaration, family, &expression)?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a KerML connector member (`KermlConnectorMember`), e.g. `connector fixWheel :
    /// BikeWheelFixed from [1] rollsOn to [1] holdsWheel;` (KerML Spec Annex A-3-3, gap: this
    /// construct was previously entirely unlowered -- see `DeclarationKind::KermlConnector`).
    /// Mirrors `lower_connection_def`: ownership, membership, an optional `:` typing target, and
    /// `from`/`to` ends resolved through `lower_kerml_connector_end` (the same
    /// `ReferenceKind::ConnectorEnd` reference kind `connection def`/`interface def` use). `is_all`
    /// and body content beyond the shared `lower_calc_def_body` walk are not modeled as distinct
    /// facts here.
    fn lower_kerml_connector_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<KermlConnectorMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::KermlConnector,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    all: node.value.is_all,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.typing {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(end) = &node.value.from {
            self.lower_kerml_connector_end(
                document,
                declaration,
                ReferenceKind::ConnectorEnd,
                end,
            )?;
        }
        if let Some(end) = &node.value.to {
            self.lower_kerml_connector_end(
                document,
                declaration,
                ReferenceKind::ConnectorEnd,
                end,
            )?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a KerML binding connector member (`KermlBindingMember`), e.g. `binding [1]
    /// startShot = [1] endShot;` (KerML Spec §8.2.4, gap: previously entirely unlowered -- see
    /// `DeclarationKind::KermlBinding`). Structurally the keyword-full sibling of
    /// `BindingConnectorUsage`/`Bind` -- mirrors `lower_binding_connector_usage`'s two-reference
    /// shape, resolving `left`/`right` as `ReferenceKind::BindSource`/`BindTarget` references
    /// through `lower_kerml_connector_end`'s target rather than a bare `QualifiedReferenceId`
    /// (each end additionally carries an optional multiplicity/`references` chain, both out of
    /// scope here, same as `KermlConnectorMember`'s ends).
    fn lower_kerml_binding_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<KermlBindingMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::KermlBinding,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::BindSource,
            &node.value.left,
        )?;
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::BindTarget,
            &node.value.right,
        )?;
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers one `KermlConnectorEnd` -- the connector-end shape shared by KerML connector,
    /// binding and succession members and by a `flow`/`allocation` usage's `from`/`to` clauses --
    /// as an authored reference of `kind`, mirroring `lower_binding_connector_operand` but
    /// operating on `KermlConnectorEnd.target` rather than a bare `QualifiedReferenceId` directly.
    /// The target is already a structured `QualifiedReferenceId` (not a general `Expression`,
    /// unlike `ConnectionEnd`), so it resolves through the same `DeclarationDomain::Any` lexical
    /// lookup. The end's own `multiplicity` and `references` chain are not modeled as distinct
    /// facts here.
    fn lower_kerml_connector_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        kind: ReferenceKind,
        end: &Node<KermlConnectorEnd>,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(end.value.target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: owner,
            kind,
            document,
            local: end.value.target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers a KerML succession member (`KermlSuccessionMember`), e.g. `succession p_before_d
    /// first [1] paint then [1] dry;` (Kernel Semantic Library `ControlPerformances.kerml`, KerML
    /// Spec Annex A-3-6-Sequences). Structurally the keyword-full sibling of `KermlBindingMember`
    /// (same `KermlConnectorEnd`-shaped `first`/`then` operands, same absent `body`/`membership`
    /// shape difference from `KermlConnectorMember`) -- reuses `lower_kerml_connector_end`
    /// verbatim for both ends, tagged `ReferenceKind::Succession` (the same kind
    /// `lower_first_stmt`'s `FirstStmt` uses for its own `first`/`then` operands) rather than
    /// `BindSource`/`BindTarget`, since this is a succession relationship, not a binding. `is_all`
    /// (`all` sufficiency) and the succession's own `multiplicity` are not modeled as distinct
    /// facts here, mirroring `KermlConnectorMember`/`KermlBindingMember`'s own unmodeled
    /// end-level `multiplicity`/`references`.
    fn lower_kerml_succession_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<KermlSuccessionMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Succession,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    all: node.value.is_all,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::Succession,
            &node.value.first,
        )?;
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::Succession,
            &node.value.then,
        )?;
        Ok(())
    }

    /// Lowers a KerML kinded parameter member (`TypedParameterMember`), e.g. `in expr
    /// thenValue[0..1] { return : Anything[0..*] ordered nonunique; }` or `in bool onOccurrence =
    /// changeSignal.signalCondition;` (Kernel Function/Semantic Libraries, `CalcDefBodyElement`'s
    /// distinct kind-keyword sibling of `InOutDecl`). Mirrors `lower_parameter_declaration`'s
    /// shape exactly (declared name folding to `None` for the redefinition-only anonymous form,
    /// a direction-tagged `FeatureTyping` reference, a `redefines` relationship, and `= expr`
    /// value evaluation through `lower_calc_expression`), plus recurses into `body` -- the same
    /// `CalcDefBody` shape `lower_calc_def_body` already walks for nested parameters/`return`
    /// declarations -- which `InOutDecl` has no field for at all. `kind` (`expr`/`bool`/
    /// `feature`/`calc`/`step`), `is_abstract`, `multiplicity`, `ordered`, and `nonunique` are not
    /// modeled as distinct facts here, mirroring `lower_parameter_declaration`'s own unmodeled
    /// `InOutDecl.ordered`/`InOutDecl.nonunique`.
    fn lower_typed_parameter_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<TypedParameterMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ParameterUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(Some(&node.value.direction)),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            let direction = Some(match node.value.direction {
                InOut::In => ParameterDirection::In,
                InOut::Out => ParameterDirection::Out,
                InOut::InOut => ParameterDirection::InOut,
            });
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags {
                    direction,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(declaration, classify_calc_expression(&expression.value));
            self.lower_calc_expression(
                document,
                declaration,
                UnsupportedFamily::CalcDefinitionMember,
                &expression,
            )?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a KerML invariant member (`KermlInvariantMember`), e.g. `inv unitBound { -1.0 <=
    /// that & that <= 1.0 }` or the anonymous `inv { isClosed == true }` (KerML Spec §8.2.7, gap:
    /// previously entirely unlowered -- see `DeclarationKind::KermlInvariant`). Its body shares
    /// the `CalcDefBody` grammar (not `ConstraintDefBody`, unlike `AssertConstraintMember`), so it
    /// is walked through the existing `lower_calc_def_body` -- the same
    /// `classify_calc_expression`/`lower_calc_expression` pipeline already used for
    /// `KermlFeatureMember` values applies unchanged to its boolean expression(s). `is_negated` is
    /// not modeled as a distinct fact here (mirrors `AssertConstraintMember`'s own `is_negated`
    /// scope boundary).
    fn lower_kerml_invariant_member(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<KermlInvariantMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::KermlInvariant,
            name,
            node.span.clone(),
            // `ast::KermlInvariantMember` carries only `is_negated`, which is an invariant-polarity
            // fact rather than a declaration modifier, and is already lowered as such.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a KerML end member with an owned cross feature (`KermlEndMember`), e.g. `end
    /// happensDuring [1..*] subsets timeCoincidentOccurrences feature thatOccurrence: Occurrence
    /// redefines longerOccurrence;` (KerML Spec Annex A-3, association-end form, gap: previously
    /// entirely unlowered -- see `DeclarationKind::KermlEnd`). Mirrors
    /// `lower_kerml_connector_member`'s ownership/membership shape: ownership, membership, and an
    /// optional `subsets` relationship on the end itself resolved through the existing
    /// `lower_subsetting_relationship` (the same `SubsettingKind`-dispatched reference machinery
    /// `KermlFeatureMember`'s own `subsets`/`redefines` clauses use). The owned nested feature is
    /// lowered through the existing `lower_kerml_feature_member`, owned by this end declaration
    /// rather than the enclosing `assoc`/type, so its own `subsets`/`redefines`/typing/value/body
    /// resolve exactly as a bare `KermlFeatureMember` would. The end's own `multiplicity` is not
    /// modeled as a distinct fact here (matches `KermlConnectorMember`'s own end multiplicity
    /// scope boundary).
    fn lower_kerml_end_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<KermlEndMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::KermlEnd,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_kerml_feature_member(
            document,
            Some(declaration),
            UnsupportedFamily::CalcDefinitionMember,
            &node.value.feature,
        )
    }

    /// Lowers a keyword-less `<name> = <expr>;` / `<name> : <Type>;` binding
    /// (`ast::structure::DefaultReferenceUsage`, BNF §8.2.2.6 / Spec §7.6.4), e.g. `baseType =
    /// Atom meta KerML::Classifier;` (KerML `metaclass` body member), mirroring
    /// `lower_kerml_feature_member`/`lower_ref_decl`: ownership, membership, an optional `:`
    /// typing target, `subsets`/`redefines` relationships, and an optional `=` value expression
    /// resolved through the shared `classify_calc_expression`/`lower_calc_expression` pipeline
    /// (the same machinery `lower_kerml_feature_member`'s `value` clause uses, which already
    /// covers the `MetaCast` reflective-operator shape from `ea6eb632`). `family` selects which
    /// diagnostic an unsupported value-expression shape falls through to; multiplicity and the
    /// `has_feature_keyword`/`body` fields are intentionally left unmodeled (see
    /// `DeclarationKind::DefaultReferenceUsage`).
    fn lower_default_reference_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        family: UnsupportedFamily,
        node: &Node<DefaultReferenceUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::DefaultReferenceUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(declaration, classify_calc_expression(&expression.value));
            self.lower_calc_expression(document, declaration, family, &expression)?;
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `item` feature member (BNF ItemUsage), e.g.
    /// `item i : SomeItem;`, mirroring `lower_part_usage`. `type_name` is a bare
    /// `QualifiedReferenceId` (not a `TypingRelationship` node, like `ItemUsage::type_name`'s
    /// `lower_enum_usage` counterpart), so its `FeatureTyping` reference is pushed directly rather
    /// than through `lower_typing_relationship`. `ItemUsage`'s body is a plain `AttributeBody`
    /// (see `lower_item_def`), so owned members are lowered through `lower_attribute_body`.
    fn lower_item_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserItemUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ItemUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a directed `in`/`out`/`inout` parameter declaration (BNF `InOutDecl`) found in a
    /// `calc def`/`constraint def`/`action def` body, e.g. `in partMasses : MassValue[0..*];`,
    /// mirroring `lower_item_usage`: ownership, membership, and (when a type is present) a
    /// `FeatureTyping` reference to the declared type, carrying an explicit
    /// `RelationshipFlags::direction` fact mirroring the `conjugated` flag precedent set by
    /// `PortUsage`. Untyped parameters (`type_name` is `None`, e.g. a bare `in :>> target = expr;`
    /// redefinition form, or `in seq[1..*] nonunique ordered;` with only a multiplicity/collection
    /// modifiers) still get the declaration/membership shell lowered -- only the `FeatureTyping`
    /// reference (and hence the direction fact) is skipped when there is no type to reference. The
    /// `:>` subsets clause (`ast::InOutDecl::subsets`, e.g. `in value :> seq;`) and the `:>>`
    /// redefinition clause (`ast::InOutDecl::redefines`) are each lowered via
    /// `lower_subsetting_relationship` regardless of whether a type is present, reusing the exact
    /// same helper `AttributeUsage`/`ItemUsage` already call. The two spellings are separate
    /// authored clauses upstream -- `:>` was previously folded into `type_name`, which reported a
    /// subsetting as a typing. The declared name
    /// may be empty for the anonymous redefinition shape; `intern_declared_name` already treats an
    /// empty name as anonymous (see its callers for `EnumerationLiteral` etc.). Multiplicity and
    /// collection modifiers (`nonunique`/`ordered`) remain out of scope, matching every other
    /// declaration kind in this codebase.
    fn lower_parameter_declaration(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        family: UnsupportedFamily,
        node: &Node<InOutDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ParameterUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    reference: node.value.is_reference,
                    var: node.value.is_var,
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(Some(&node.value.direction)),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            let direction = Some(match node.value.direction {
                InOut::In => ParameterDirection::In,
                InOut::Out => ParameterDirection::Out,
                InOut::InOut => ParameterDirection::InOut,
            });
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags {
                    direction,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        // Widened value-assignment handling (see `lower_value_assignment`/`lower_return_decl`'s
        // own `= expr` handling, which this mirrors exactly): a parameter default value
        // (`out v_out : SpeedValue = vel.v;`) is a bare `Node<Expression>` on `InOutDecl::value`
        // -- the same shape `ReturnDecl::value` already has classify_calc_expression/
        // lower_calc_expression wiring for, so this reuses that identical pipeline rather than
        // introducing new logic. Previously deferred (`494b0ba6`) pending value-assignment
        // machinery existing at all.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(declaration, classify_calc_expression(&expression.value));
            self.lower_calc_expression(document, declaration, family, &expression)?;
        }
        Ok(())
    }

    /// Lowers a calc-body `return` declaration (BNF `ReturnDecl`, e.g. `return : Type = a + b;`
    /// or `return result : Type = a + b;`), reusing `lower_parameter_declaration`'s shape
    /// (ownership, membership, a `FeatureTyping` reference to the declared type) since a return
    /// declaration is itself a parameter-like feature -- SysML models a calc's `result` as an
    /// implicit output parameter. `name` is empty for the common anonymous `return : Type = expr;`
    /// form (validation `10c`/`10d`); `intern_declared_name` folds that to `None`, exactly like
    /// `lower_subject_decl`'s bare `subject;` form. Unlike `InOutDecl::type_name`, `ReturnDecl::
    /// type_name` is never optional (the grammar requires a type), so the `FeatureTyping`
    /// reference is unconditional here.
    ///
    /// When a `= expr` value is present, its expression is classified/lowered through the exact
    /// same `classify_calc_expression`/`lower_calc_expression` machinery slices 1-4 already built
    /// for a bare `CalcDefBodyElement::Expression` body -- this is the "distinct ReturnDecl shape"
    /// `bd50fccd` deferred: most real-corpus calc arithmetic (e.g. `return : Type = a + b * c;`)
    /// lives here, not in a bare `Expression` body-element, and it is the exact same `Expression`
    /// enum/`BinaryOp`/`FeatureRef` leaf shapes slices 1-4 already handle, so this is pure wiring
    /// into the same pipeline -- no new evaluation logic.
    ///
    /// `is_redefine` (`return :>> name = expr;`) and `is_subsetting` (`return name :> Type = expr;`)
    /// spelling variants are not modeled as distinct relationship kinds here (mirrors
    /// `lower_parameter_declaration`'s own `InOutDecl::redefines`-shaped field being out of
    /// scope); both spellings still get the same `FeatureTyping` reference and evaluation fact.
    fn lower_return_decl(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ReturnDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ParameterUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(declaration, classify_calc_expression(&expression.value));
            self.lower_calc_expression(
                document,
                declaration,
                UnsupportedFamily::CalcDefinitionMember,
                &expression,
            )?;
        }
        Ok(())
    }

    /// Lowers a case-family `return` declaration (BNF `CaseReturnDecl`) found in an analysis/
    /// verification/use-case/case def or usage body, e.g. `return calculatedFuelEconomy :
    /// DistancePerVolumeValue;`, `return part :>> selectedAlternative : Engine;`, `return
    /// simulatedRange = vehicle.vehicleBehavior.output.distance;`, or the bare shorthand `return
    /// :>> target;`. Mirrors `lower_parameter_declaration`'s shape (reusing the same
    /// `DeclarationKind::ParameterUsage` -- a case return is itself an output-parameter-like
    /// feature, same as a calc's own `ReturnDecl`): ownership, membership, a `FeatureTyping` (`:`)
    /// or `Subsetting` (`:>`, `is_subsetting`) reference to the declared type, an authored
    /// `Redefinition` reference for the `:>>`-shorthand `target` (mirrors `VerifyRequirementMember::
    /// redefines`'s identical bare-`QualifiedReferenceId` handling), and a bound `=`/`:=` value
    /// through the same `classify_calc_expression`/`lower_calc_expression` pipeline `lower_return_
    /// decl` uses. `declaration_name` is empty for the common anonymous `return : Type = expr;`
    /// form; `intern_declared_name` folds that to `None`. The `part`/`attribute` `feature_kind`
    /// prefix and `multiplicity` are not modeled as distinct facts, mirroring `lower_parameter_
    /// declaration`'s own out-of-scope fields.
    fn lower_case_return_decl(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<CaseReturnDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.declaration_name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ParameterUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: if node.value.is_subsetting {
                    ReferenceKind::Subsetting
                } else {
                    ReferenceKind::FeatureTyping
                },
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(target) = node.value.target {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::Redefinition,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(declaration, classify_calc_expression(&expression.value));
            self.lower_calc_expression(document, declaration, family, &expression)?;
        }
        Ok(())
    }

    /// Lowers a `subject` declaration (BNF `SubjectDecl`) found in a requirement/concern/case-
    /// family def or usage body, e.g. `subject vehicle : Vehicle;`, mirroring
    /// `lower_parameter_declaration`'s shape: ownership, membership, and (when a type is present)
    /// a `FeatureTyping` reference to the declared type. No direction fact applies here.
    /// Multiplicity, the bound `= expr` value, and the bare `subject = expr;`/`subject;`
    /// shorthand forms (`ast::SubjectRef`, handled separately) are out of scope.
    fn lower_subject_decl(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<SubjectDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::SubjectUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a `stakeholder` member found in a requirement/viewpoint def body (BNF
    /// `StakeholderMember`), mirroring `lower_subject_decl`'s typed-declaration shape (ownership,
    /// membership, an optional `FeatureTyping` reference) plus the concern-reference/redefinition
    /// operand: when `target` is present, it is lowered as an authored `ReferenceKind::
    /// Redefinition` reference (for the `:>>` spelling, `is_redefinition == true`) or
    /// `ReferenceKind::StakeholderTarget` reference (the bare `stakeholder Concern;` spelling)
    /// sourced at the same declaration. `declaration_name` may be empty for either reference form
    /// (`intern_declared_name` already folds that to an anonymous declaration, matching
    /// `SubjectUsage`'s own bare-form handling).
    fn lower_stakeholder_member(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<StakeholderMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.declaration_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StakeholderUsage,
            name,
            node.span.clone(),
            // `ast::StakeholderMember` carries no modifier, multiplicity, direction, or short name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(target) = node.value.target {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            let kind = if node.value.is_redefinition {
                ReferenceKind::Redefinition
            } else {
                ReferenceKind::StakeholderTarget
            };
            self.push_reference(PendingReference {
                source: declaration,
                kind,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a viewpoint `purpose` member (BNF `PurposeMember`), an always-present concern
    /// reference (`PurposeMember.target`, no plain-declaration/redefinition alternatives the way
    /// `StakeholderMember` has), resolved as an authored `ReferenceKind::PurposeTarget` reference
    /// sourced directly at the enclosing `owner` declaration, mirroring `SatisfyViewpoint`'s
    /// single-operand, no-nested-declaration shape.
    fn lower_purpose_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<PurposeMember>,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: owner,
            kind: ReferenceKind::PurposeTarget,
            document,
            local: node.value.target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers a typed `actor` parameter declaration found in a requirement def body (BNF
    /// `RequirementActorDecl`), mirroring `lower_subject_decl`'s shape (ownership, membership,
    /// a `FeatureTyping` reference to the declared type), except `type_name` is unconditional here
    /// (never optional, unlike `SubjectDecl::type_name`).
    fn lower_requirement_actor_decl(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<RequirementActorDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementActor,
            name,
            node.span.clone(),
            // `ast::RequirementActorDecl` has no `multiplicity` field, unlike its `ActorUsage`
            // sibling; see planning/UPSTREAM_PARSER_GAPS.md.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        let type_name = node.value.type_name;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(type_name)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::FeatureTyping,
            document,
            local: type_name,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers an `actor` member found in a use-case-family def/usage body (BNF `ActorUsage`,
    /// e.g. `actor driver : Person;`, `actor passengers : Person[0..4];`), mirroring
    /// `lower_requirement_actor_decl`'s shape (ownership, membership, a `FeatureTyping` reference
    /// to the declared type) but reading visibility off `ActorUsage::membership` (kind
    /// `ActorMembership`) instead. The bare untyped form (`actor environment;`) authors no type,
    /// so it contributes the declaration and its membership and no typing reference. The optional
    /// trailing multiplicity is not modeled as a distinct fact, mirroring `lower_subject_decl`'s
    /// own out-of-scope multiplicity.
    fn lower_actor_usage(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ActorUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::CaseActor,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::ActorMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a named `frame` member found in a requirement def body (BNF `FrameMember`) as a
    /// purely syntactic named grouping: ownership, membership, and its nested `RequirementDefBody`
    /// content dispatched back through the same shared `lower_requirement_shaped_body` walker used
    /// by `requirement def`/`requirement` usage/`viewpoint def` bodies, sharing the caller-supplied
    /// `unsupported` family so a member unrecognized inside a frame reports under the same
    /// diagnostic family as one outside it.
    fn lower_frame_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        unsupported: UnsupportedFamily,
        node: &Node<FrameMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Frame,
            name,
            node.span.clone(),
            // A `frame` member is a purely syntactic named grouping; `ast::FrameMember` carries no
            // modifier, multiplicity, direction, or short name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_requirement_shaped_body(document, declaration, &node.value.body, unsupported)
    }

    /// Lowers a requirement/objective-body `verify <requirement>;` shorthand body element (BNF
    /// `VerifyRequirementMember`, `explicit_requirement_keyword == false`) as an anonymous feature
    /// owned by the enclosing declaration, mirroring `Satisfy`'s nested-declaration shape: the
    /// shorthand `target` is lowered as an authored `ReferenceKind::VerifyRequirementTarget`
    /// reference, and an optional `:>>` `redefines` target is lowered as an authored
    /// `ReferenceKind::Redefinition` reference, both sourced at this new declaration. The fuller
    /// `verify requirement <name> : <Type> { ... }` form (`explicit_requirement_keyword == true`,
    /// which defines a new anonymous requirement usage inline rather than referencing an existing
    /// one) is out of scope and reported as an explicit `family` unsupported diagnostic.
    fn lower_verify_requirement_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<VerifyRequirementMember>,
    ) -> Result<(), ConstructionError> {
        if node.value.explicit_requirement_keyword {
            self.push_unsupported(document, family, node.span.clone());
            return Ok(());
        }
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::VerifyRequirement,
            None,
            node.span.clone(),
            // `ast::VerifyRequirementMember` carries only its redefinition target.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(target) = node.value.target {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::VerifyRequirementTarget,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(redefines) = node.value.redefines {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(redefines)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::Redefinition,
                document,
                local: redefines,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers an explicit `perform action <name> : <Type>;` performance usage (BNF `Perform`)
    /// found in a part def/usage or action def/usage body, mirroring `lower_action_usage`'s
    /// shape: ownership, membership, an optional `FeatureTyping`/`Subclassification` reference to
    /// the performed action type, and `subsets`/`redefines` specialization. Only nested `part`/
    /// `item` usages inside the perform's own body are lowered; the shorthand `perform <path>;`
    /// reference form (no declaration label) and other body content are out of scope.
    fn lower_perform(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserPerform>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.action_name)?;
        let (is_abstract, variation) =
            definition_prefix_modifiers(node.value.usage_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PerformActionUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_perform_body(document, declaration, &node.value.body)
    }

    /// Lowers the `PerformBody` owned by a `perform action` usage (BNF `PerformBodyElement`):
    /// `part`/`item`/`attribute` usages, `in`/`out` parameter-argument bindings, nested
    /// action-body content (an anonymous `perform action { ... }`'s own body, typed
    /// `ActionUsageBodyElement`, delegated to `lower_action_usage_body_element` rather than
    /// duplicating the control-flow dispatch here), and `variant` members (delegated to
    /// `lower_variant_usage`, the same function `PartUsageBodyElement::VariantUsage` uses) are all
    /// recognized.
    fn lower_perform_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &PerformBody,
    ) -> Result<(), ConstructionError> {
        let PerformBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                PerformBodyElement::PartUsage(part_usage) => {
                    self.lower_part_usage(document, Some(owner), part_usage)?;
                }
                PerformBodyElement::ItemUsage(item_usage) => {
                    self.lower_item_usage(document, Some(owner), item_usage)?;
                }
                PerformBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                PerformBodyElement::Doc(node) => {
                    self.record_doc_comment(owner, node)?;
                }
                PerformBodyElement::Action(element) => {
                    self.lower_action_usage_body_element(document, owner, element)?;
                }
                PerformBodyElement::InOut(node) => {
                    self.lower_perform_inout_binding(document, owner, node)?;
                }
                PerformBodyElement::Variant(node) => {
                    self.lower_variant_usage(
                        document,
                        owner,
                        UnsupportedFamily::ActionUsageMember,
                        node,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Lowers a `perform` usage's `in`/`out <target> = <value>;` parameter-argument binding (BNF
    /// `PerformInOutBinding`, `ast::structure::PerformInOutBinding`, found only inside
    /// `PerformBody`) as its own anonymous `DeclarationKind::PerformParameterBinding` feature
    /// owned by `owner` (the enclosing `perform`'s own declaration), mirroring `lower_bind`'s
    /// shape: `target` is an authored reference to the invoked action's own declared parameter
    /// (not a new declared name, unlike `InOutDecl`'s own `in`/`out` shorthand), resolved directly
    /// as a `PerformParameterTarget` reference since it is already a structured
    /// `QualifiedReferenceId`; `value` is lowered through the ordinary
    /// `lower_constraint_expression` machinery, exactly like `Assign`'s RHS. `direction`
    /// (`in`/`out`/`inout`) is out of scope, matching `Bind`'s own "binding keyword prefix
    /// ignored" precedent.
    fn lower_perform_inout_binding(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<PerformInOutBinding>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::PerformParameterBinding,
            None,
            node.span.clone(),
            DeclarationFacts {
                direction: direction_fact(Some(&node.value.direction)),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::PerformParameterTarget,
            document,
            local: node.value.target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        self.push_evaluation_fact(
            declaration,
            classify_constraint_expression(&node.value.value.value),
        );
        self.lower_constraint_expression(
            document,
            declaration,
            UnsupportedFamily::ActionUsageMember,
            &node.value.value,
        )
    }

    /// Lowers a `metadata def` (BNF MetadataDefinition), mirroring `lower_item_def`: ownership,
    /// membership, and an optional `:>` specialization relationship. `MetadataDef`'s body is a
    /// plain `AttributeBody` (shared with `AttributeDef`/`ItemDef`), so its owned members are
    /// lowered through the existing `lower_attribute_body`.
    fn lower_metadata_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<MetadataDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::MetadataDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `metadata` feature member (BNF MetadataUsage),
    /// e.g. `metadata m : SomeMetadata;`, mirroring `lower_item_usage`. `type_reference` is a
    /// bare `QualifiedReferenceId`, so its `FeatureTyping` reference is pushed directly rather
    /// than through `lower_typing_relationship`. `MetadataUsage`'s body is a plain
    /// `AttributeBody` (see `lower_metadata_def`), so owned members are lowered through
    /// `lower_attribute_body`. The `about` clause (annotation targets) is deliberately not
    /// lowered here -- it belongs to the separate annotation-application fact family, out of
    /// scope for this slice.
    fn lower_metadata_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserMetadataUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::MetadataUsage,
            name,
            node.span.clone(),
            // `ast::MetadataUsage` carries no modifier, multiplicity, direction, or short name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_reference) = node.value.type_reference {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_reference,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers an `@Name{...}`/`@Name;` metadata annotation body element (`ast::MetadataAnnotation`,
    /// see `ReferenceKind::MetadataAnnotation`), applied to `owner` -- the declaration that owns
    /// the body the annotation appears in (a part usage, action def, state def, ...). Only the
    /// annotation-target reference (`reference`, e.g. `Safety`) is resolved, sourced directly at
    /// `owner`; `about_targets` and the nested `body` (feature-value overrides) are out of scope,
    /// see the `ReferenceKind::MetadataAnnotation` doc comment.
    fn lower_metadata_annotation(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<MetadataAnnotation>,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.reference)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: owner,
            kind: ReferenceKind::MetadataAnnotation,
            document,
            local: node.value.reference,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        // Widened value-assignment handling (see `lower_value_assignment`): the annotation body's
        // nested feature-value overrides (`@Safety{isMandatory = true;}`'s `isMandatory = true;`)
        // were deliberately deferred by the annotation-application slice pending exactly this
        // machinery (see the `ReferenceKind::MetadataAnnotation` doc comment). Each override is
        // typed as an ordinary `AttributeUsage` (BNF-shared `AttributeBody`, exactly like `metadata
        // m : Safety { isMandatory = true; }`'s own body), but the `@Safety{...}` annotation form
        // has no named declaration of its own to own them (unlike a named `metadata m : Safety`
        // usage) -- an anonymous `MetadataUsage`-kind declaration nested under `owner` gives the
        // overrides a real owning scope without disturbing `owner`'s own member set or the
        // `MetadataAnnotation` reference above (still sourced directly at `owner`, unchanged).
        if matches!(&node.value.body, AttributeBody::Brace { elements, .. } if !elements.is_empty())
        {
            let annotation_scope = self.push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::MetadataUsage,
                None,
                node.span.clone(),
                // A synthesized scope for the annotation body's members; it has no authored
                // declaration syntax of its own.
                DeclarationFacts::none(),
            )?;
            self.push_membership(
                annotation_scope,
                MembershipKind::Feature,
                Visibility::Default,
                node.span.clone(),
            )?;
            self.lower_attribute_body(document, annotation_scope, &node.value.body)?;
        }
        Ok(())
    }

    /// Lowers an `action def` (BNF ActionDefinition), mirroring `lower_part_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned declarations.
    /// Behavioral/control-flow body elements (parameters, succession, decision/merge/fork/join,
    /// accept/send, perform, assign, loops) are explicitly out of scope; unrecognized body
    /// elements fall through to `unsupported_action_definition_member` via
    /// `lower_action_def_body`.
    fn lower_action_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ActionDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ActionDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_action_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the `ActionDefBody` shared by `action def` and by an `action` usage's own owned
    /// members (BNF `ActionDefBodyElement`): recognized owned members are nested action usages
    /// and `item` usages (BNF `StructureUsageMember` shape, see `crate::ast::ItemUsage`);
    /// everything else -- in/out parameters, `first`/`then` succession, decision/merge/fork/join,
    /// accept/send, perform, assign, loops -- falls through to
    /// `unsupported_action_definition_member`. This is the genuinely out-of-scope
    /// behavioral/control-flow surface for this slice.
    fn lower_action_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionDefBody,
    ) -> Result<(), ConstructionError> {
        let ActionDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            self.lower_action_def_body_element(document, owner, element)?;
        }
        Ok(())
    }

    /// Lowers one `ActionDefBodyElement`, wherever the grammar puts one: the members of an
    /// `ActionDefBody`, and the single brace-less member an `if` branch may be written as
    /// (`ActionBranchBody::Shorthand`). See `lower_action_def_body`'s doc comment for the per-arm
    /// recognized/unsupported shape.
    fn lower_action_def_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        element: &Node<ActionDefBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            ActionDefBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            ActionDefBodyElement::ActionUsage(action_usage) => {
                self.lower_action_usage(document, Some(owner), action_usage)?;
            }
            ActionDefBodyElement::ItemUsage(item_usage) => {
                self.lower_item_usage(document, Some(owner), item_usage)?;
            }
            ActionDefBodyElement::MetadataUsage(metadata_usage) => {
                self.lower_metadata_usage(document, Some(owner), metadata_usage)?;
            }
            ActionDefBodyElement::StateUsage(state_usage) => {
                self.lower_state_usage(document, Some(owner), state_usage)?;
            }
            ActionDefBodyElement::OccurrenceUsage(occurrence_usage) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence_usage)?;
            }
            ActionDefBodyElement::PartUsage(part_usage) => {
                self.lower_part_usage(document, Some(owner), part_usage)?;
            }
            ActionDefBodyElement::FirstStmt(first_stmt) => {
                self.lower_first_stmt(
                    document,
                    owner,
                    UnsupportedFamily::ActionDefinitionMember,
                    first_stmt,
                )?;
            }
            ActionDefBodyElement::InOutDecl(param) => {
                self.lower_parameter_declaration(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionDefinitionMember,
                    param,
                )?;
            }
            ActionDefBodyElement::Perform(perform) => {
                self.lower_perform(document, Some(owner), perform)?;
            }
            ActionDefBodyElement::Doc(node) => {
                self.record_doc_comment(owner, node)?;
            }
            ActionDefBodyElement::TextualRep(node) => {
                self.record_textual_representation(owner, node)?;
            }
            ActionDefBodyElement::MetadataAnnotation(node) => {
                self.lower_metadata_annotation(document, owner, node)?;
            }
            ActionDefBodyElement::Bind(node) => {
                self.lower_bind(
                    document,
                    owner,
                    UnsupportedFamily::ActionDefinitionMember,
                    node,
                )?;
            }
            ActionDefBodyElement::AssertConstraint(node) => self.lower_assert_constraint_member(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node,
            )?,
            ActionDefBodyElement::RefDecl(node) => {
                self.lower_ref_decl(document, Some(owner), node)?;
            }
            ActionDefBodyElement::MergeStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Merge,
                ReferenceKind::MergeInput,
                node.span.clone(),
                &node.value.merge,
                &node.value.body,
            )?,
            ActionDefBodyElement::DecisionStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Decide,
                ReferenceKind::DecisionInput,
                node.span.clone(),
                &node.value.decide,
                &node.value.body,
            )?,
            ActionDefBodyElement::JoinStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Join,
                ReferenceKind::JoinInput,
                node.span.clone(),
                &node.value.join,
                &node.value.body,
            )?,
            ActionDefBodyElement::ForkStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Fork,
                ReferenceKind::ForkInput,
                node.span.clone(),
                &node.value.fork,
                &node.value.body,
            )?,
            ActionDefBodyElement::ThenAction(node) => {
                self.lower_then_action(
                    document,
                    owner,
                    UnsupportedFamily::ActionDefinitionMember,
                    node,
                )?;
            }
            ActionDefBodyElement::FlowUsage(node) => self.lower_flow_usage(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node,
            )?,
            ActionDefBodyElement::TerminateStmt(node) => self.lower_terminate_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node,
            )?,
            ActionDefBodyElement::DefaultReferenceUsage(node) => {
                self.lower_default_reference_usage(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionDefinitionMember,
                    node,
                )?;
            }
            ActionDefBodyElement::WhileStmt(node) => self.lower_while_or_loop_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::While,
                node.span.clone(),
                Some(&node.value.condition),
                &node.value.body,
            )?,
            ActionDefBodyElement::LoopStmt(node) => self.lower_while_or_loop_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                DeclarationKind::Loop,
                node.span.clone(),
                None,
                &node.value.body,
            )?,
            ActionDefBodyElement::IfStmt(node) => self.lower_if_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionDefBodyElement::Assign(node) => self.lower_assign_stmt(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionDefBodyElement::ForLoop(node) => self.lower_for_loop(
                document,
                owner,
                UnsupportedFamily::ActionDefinitionMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionDefBodyElement::AttributeUsage(node) => {
                self.lower_attribute_usage(document, Some(owner), node)?;
            }
            ActionDefBodyElement::CalcUsage(node) => {
                self.lower_calc_usage(document, Some(owner), node)?;
            }
            ActionDefBodyElement::ActionDef(node) => {
                self.lower_action_def(document, Some(owner), node)?;
            }
            ActionDefBodyElement::Annotation(_) | ActionDefBodyElement::MetadataKeywordUsage(_) => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionDefinitionMember,
                    element.span.clone(),
                )
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `action` feature member (BNF ActionUsage), e.g.
    /// `action validateRoute;` or `action a : SomeAction;`, mirroring `lower_part_usage`.
    /// `ActionUsage`'s typing is a structured `TypingRelationship` (like `PartUsage.typing`), not
    /// a bare `QualifiedReferenceId`. Behavioral clauses (`accept`/`send`/`via`/`to`, parameters,
    /// abstract/variation/individual prefixes) are explicitly out of scope; owned members lower
    /// through the same `lower_action_def_body` as an `action def`'s body (BNF `ActionUsageBody`
    /// is a structurally near-identical production, differing only in the extra `VariantUsage`
    /// alternative, which is itself out of scope and so folds into the same unsupported family).
    fn lower_action_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserActionUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ActionUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    variation: node.value.is_variation,
                    individual: node.value.is_individual,
                    reference: node.value.is_reference,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_accept_send_clauses(document, declaration, node)?;
        // An action usage is one of the two constructs whose body may not be written at all --
        // `action a accept M via v;` ends at the statement after it -- so an absent body is a
        // distinct upstream state from `;`, and neither owns members.
        match &node.value.body {
            Some(body) => self.lower_action_usage_body(document, declaration, body),
            None => Ok(()),
        }
    }

    /// Lowers the accept/send-suffix facts an `ActionUsage` may carry (BNF `AcceptParameterPart`/
    /// `SenderReceiverPart`, GH-86): a standalone control-node statement's typed `accept name :
    /// Type` payload (`ActionUsage.accept`), a `send`-suffixed usage's optional payload
    /// (`ActionUsage.send`, either a typed-name clause like `accept`'s or a general expression
    /// e.g. `send new Publish(...)`), and the optional trailing `via <port>`/`to <target>` clauses
    /// shared by both forms. Only the payload TYPE reference (`AcceptPayloadType`) and the via/to
    /// operand references (`AcceptVia`/`SendTarget`) are resolved; the payload's own declared NAME
    /// is not a reference target (mirrors `InOutDecl`'s own scope boundary -- the name introduces a
    /// binding, it does not reference one). Sourced directly at `declaration` (the `ActionUsage`'s
    /// own declaration), not an anonymous nested one: unlike `Bind`/`Allocate`, each `ActionUsage`
    /// already has its own unique declaration to source these facts at.
    fn lower_accept_send_clauses(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        node: &Node<ParserActionUsage>,
    ) -> Result<(), ConstructionError> {
        let family = UnsupportedFamily::ActionUsageMember;
        if let Some(clause) = &node.value.accept {
            self.lower_payload_clause_type(document, declaration, clause)?;
        }
        if let Some(send) = &node.value.send {
            match send {
                SendPayload::Typed(clause) => {
                    self.lower_payload_clause_type(document, declaration, clause)?;
                }
                SendPayload::Expression(expr) => {
                    self.lower_constraint_expression(document, declaration, family, expr)?;
                }
            }
        }
        if let Some(via) = &node.value.via {
            self.lower_satisfy_operand(
                document,
                declaration,
                family,
                ReferenceKind::AcceptVia,
                via,
            )?;
        }
        if let Some(to) = &node.value.to {
            self.lower_satisfy_operand(
                document,
                declaration,
                family,
                ReferenceKind::SendTarget,
                to,
            )?;
        }
        Ok(())
    }

    /// Resolves a `PayloadClause`'s optional `: Type` suffix (`accept name : Type`/`send name :
    /// Type`) as an `AcceptPayloadType` reference, resolved through the same
    /// Subclassification/FeatureTyping `DeclarationDomain::Type` lexical lookup fixed point as
    /// `FeatureTyping` -- the payload names a type, exactly like an ordinary parameter's type
    /// annotation.
    fn lower_payload_clause_type(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        clause: &sysml_v2_parser_next::ast::PayloadClause,
    ) -> Result<(), ConstructionError> {
        let Some(type_name) = clause.type_name else {
            return Ok(());
        };
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(type_name)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::AcceptPayloadType,
            document,
            local: type_name,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers the `ActionUsageBody` owned by an `action` usage (BNF `ActionUsageBodyElement`):
    /// see `lower_action_def_body` for the shared recognized/unsupported shape. The one
    /// additional alternative here, `VariantUsage`, wraps the same `ast::VariantUsage` node
    /// `lower_variant_usage` already lowers for `PartUsageBodyElement::VariantUsage`/
    /// `PerformBodyElement::Variant`, so it dispatches there rather than staying unconditionally
    /// unsupported. Delegates each element to `lower_action_usage_body_element`, which
    /// `PerformBodyElement::Action` (BNF `PerformBodyElement`, the identical body-element shape an
    /// anonymous `perform action { ... }` owns) also calls directly, since a `perform action`'s
    /// own body is typed `ActionUsageBodyElement` too, not a distinct enum.
    fn lower_action_usage_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionUsageBody,
    ) -> Result<(), ConstructionError> {
        let ActionUsageBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            self.lower_action_usage_body_element(document, owner, element)?;
        }
        Ok(())
    }

    /// Lowers one `ActionUsageBodyElement` (see `lower_action_usage_body`'s doc comment for why
    /// this is a standalone per-element function rather than inlined into that loop).
    fn lower_action_usage_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        element: &Node<ActionUsageBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            ActionUsageBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            ActionUsageBodyElement::ActionUsage(action_usage) => {
                self.lower_action_usage(document, Some(owner), action_usage)?;
            }
            ActionUsageBodyElement::ItemUsage(item_usage) => {
                self.lower_item_usage(document, Some(owner), item_usage)?;
            }
            ActionUsageBodyElement::MetadataUsage(metadata_usage) => {
                self.lower_metadata_usage(document, Some(owner), metadata_usage)?;
            }
            ActionUsageBodyElement::StateUsage(state_usage) => {
                self.lower_state_usage(document, Some(owner), state_usage)?;
            }
            ActionUsageBodyElement::OccurrenceUsage(occurrence_usage) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence_usage)?;
            }
            ActionUsageBodyElement::PartUsage(part_usage) => {
                self.lower_part_usage(document, Some(owner), part_usage)?;
            }
            ActionUsageBodyElement::FirstStmt(first_stmt) => {
                self.lower_first_stmt(
                    document,
                    owner,
                    UnsupportedFamily::ActionUsageMember,
                    first_stmt,
                )?;
            }
            ActionUsageBodyElement::InOutDecl(param) => {
                self.lower_parameter_declaration(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionUsageMember,
                    param,
                )?;
            }
            ActionUsageBodyElement::Doc(node) => {
                self.record_doc_comment(owner, node)?;
            }
            ActionUsageBodyElement::TextualRep(node) => {
                self.record_textual_representation(owner, node)?;
            }
            ActionUsageBodyElement::MetadataAnnotation(node) => {
                self.lower_metadata_annotation(document, owner, node)?;
            }
            ActionUsageBodyElement::Bind(node) => {
                self.lower_bind(document, owner, UnsupportedFamily::ActionUsageMember, node)?;
            }
            ActionUsageBodyElement::AssertConstraint(node) => self.lower_assert_constraint_member(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node,
            )?,
            ActionUsageBodyElement::RefDecl(node) => {
                self.lower_ref_decl(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::MergeStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Merge,
                ReferenceKind::MergeInput,
                node.span.clone(),
                &node.value.merge,
                &node.value.body,
            )?,
            ActionUsageBodyElement::DecisionStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Decide,
                ReferenceKind::DecisionInput,
                node.span.clone(),
                &node.value.decide,
                &node.value.body,
            )?,
            ActionUsageBodyElement::JoinStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Join,
                ReferenceKind::JoinInput,
                node.span.clone(),
                &node.value.join,
                &node.value.body,
            )?,
            ActionUsageBodyElement::ForkStmt(node) => self.lower_first_merge_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Fork,
                ReferenceKind::ForkInput,
                node.span.clone(),
                &node.value.fork,
                &node.value.body,
            )?,
            ActionUsageBodyElement::ThenAction(node) => {
                self.lower_then_action(
                    document,
                    owner,
                    UnsupportedFamily::ActionUsageMember,
                    node,
                )?;
            }
            ActionUsageBodyElement::FlowUsage(node) => {
                self.lower_flow_usage(document, owner, UnsupportedFamily::ActionUsageMember, node)?
            }
            ActionUsageBodyElement::TerminateStmt(node) => self.lower_terminate_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node,
            )?,
            ActionUsageBodyElement::DefaultReferenceUsage(node) => {
                self.lower_default_reference_usage(
                    document,
                    Some(owner),
                    UnsupportedFamily::ActionUsageMember,
                    node,
                )?;
            }
            ActionUsageBodyElement::WhileStmt(node) => self.lower_while_or_loop_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::While,
                node.span.clone(),
                Some(&node.value.condition),
                &node.value.body,
            )?,
            ActionUsageBodyElement::LoopStmt(node) => self.lower_while_or_loop_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                DeclarationKind::Loop,
                node.span.clone(),
                None,
                &node.value.body,
            )?,
            ActionUsageBodyElement::IfStmt(node) => self.lower_if_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionUsageBodyElement::Assign(node) => self.lower_assign_stmt(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionUsageBodyElement::ForLoop(node) => self.lower_for_loop(
                document,
                owner,
                UnsupportedFamily::ActionUsageMember,
                node.span.clone(),
                &node.value,
            )?,
            ActionUsageBodyElement::VariantUsage(node) => {
                self.lower_variant_usage(
                    document,
                    owner,
                    UnsupportedFamily::ActionUsageMember,
                    node,
                )?;
            }
            ActionUsageBodyElement::AttributeUsage(node) => {
                self.lower_attribute_usage(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::CalcUsage(node) => {
                self.lower_calc_usage(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::ActionDef(node) => {
                self.lower_action_def(document, Some(owner), node)?;
            }
            ActionUsageBodyElement::Annotation(_)
            | ActionUsageBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                document,
                UnsupportedFamily::ActionUsageMember,
                element.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers a `first X then Y;` control-flow succession statement (BNF `FirstStmt`) found
    /// inside an action def/usage body as its own anonymous `DeclarationKind::Succession`
    /// feature owned by the enclosing action def/usage `owner` declaration, mirroring
    /// `lower_end_decl`'s nested-declaration shape: both ends are lowered as authored
    /// `Succession` references sourced at this new anonymous declaration (not at `owner`
    /// directly), so lexical lookup starts in `owner`'s own scope -- where `X`/`Y` are actually
    /// declared as sibling actions -- rather than `owner`'s enclosing scope. The `first` end is
    /// always lowered; the `then` end is `None` for the standalone initial-node marker
    /// `first start;` (§6 G13), which is left as-is (no reference to lower). The named/typed
    /// `succession` keyword prefix and any braced body content are out of scope.
    fn lower_first_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<FirstStmt>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Succession,
            None,
            node.span.clone(),
            DeclarationFacts {
                // The succession feature's own multiplicity (`succession [n] first ... then ...`).
                // The per-end `first_multiplicity`/`then_multiplicity` belong to the ends, which
                // are lowered as references rather than declarations, so they are not facts here.
                multiplicity: multiplicity_facts(node.value.succession_multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_succession_end(
            document,
            declaration,
            family,
            ReferenceKind::Succession,
            &node.value.first,
        )?;
        if let Some(then) = &node.value.then {
            self.lower_succession_end(
                document,
                declaration,
                family,
                ReferenceKind::Succession,
                then,
            )?;
        }
        Ok(())
    }

    /// Lowers one succession end (the `first` or `then` operand of a `FirstStmt`, or -- reused
    /// verbatim by `AssignTarget`/`Decide`/`Merge`/`Fork`/`Join` -- any other paired/single-operand
    /// control-flow reference): its path expression is a structured `Expression` (not a flattened
    /// string), so a simple/qualified name (`Expression::FeatureRef`) resolves as an authored
    /// reference through the same shared lexical lookup as `ConnectorEnd`. A dotted feature-chain
    /// path -- either nested `Expression::MemberAccess` nodes or a single `Expression::
    /// FeatureChainRef` (the shape the parser actually produces for a dotted path with no
    /// intervening non-name segments, e.g. `assign a.b := ...;`'s target, mirroring
    /// `lower_satisfy_operand`'s identical `MemberAccess`/`FeatureChainRef` pairing) -- resolves as
    /// a `MemberAccessOperand` reference through `flatten_member_access_chain`/
    /// `push_member_access_reference`. The bare `start`/`done` pseudo-action markers parse as an
    /// ordinary `FeatureRef` that legitimately fails to resolve because no such declaration is
    /// synthesized; any other expression shape is left as an explicit unsupported-member
    /// diagnostic.
    fn lower_succession_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        kind: ReferenceKind,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::FeatureRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: owner,
                    kind,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            Expression::MemberAccess { .. } | Expression::FeatureChainRef(_) => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(owner, document, &chain, node.span.clone())?;
                } else {
                    self.push_unsupported(document, family, node.span.clone());
                }
            }
            _ => self.push_unsupported(document, family, node.span.clone()),
        }
        Ok(())
    }

    /// Lowers a `decide`/`merge`/`fork`/`join` control node (BNF `DecisionStmt`/`MergeStmt`/
    /// `ForkStmt`/`JoinStmt`, which all share the identical `<keyword> <expr> <FirstMergeBody>`
    /// shape) as its own anonymous nested-declaration feature owned by `owner`, mirroring
    /// `lower_first_stmt`'s `Succession` shape: the required operand expression is lowered as a
    /// `kind` reference through `lower_succession_end`'s exact `FeatureRef`/`MemberAccess`
    /// dispatch, and a braced body's members recurse through `lower_first_merge_body`.
    #[allow(clippy::too_many_arguments)]
    fn lower_first_merge_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        decl_kind: DeclarationKind,
        ref_kind: ReferenceKind,
        span: Span,
        operand: &Node<Expression>,
        body: &FirstMergeBody,
    ) -> Result<(), ConstructionError> {
        // A synthesized control-flow scope with no authored declaration syntax of its own.
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            decl_kind,
            None,
            span.clone(),
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        self.lower_succession_end(document, declaration, family, ref_kind, operand)?;
        self.lower_first_merge_body(document, declaration, family, body)
    }

    /// Lowers a `decide`/`merge`/`fork`/`join` node's optional braced body (BNF
    /// `FirstMergeBody::Brace`): each retained member is an ordinary `ActionDefBodyElement`, so
    /// the common nested shapes actually authored -- `in`/`out` parameter declarations (a fork's
    /// output flows, e.g. `fork F { in a; out b1; out b2; }`), nested action usages, and further
    /// `then <target>;` continuations -- recurse through the same lowering functions an ordinary
    /// action def/usage body uses. Anything else falls through to the existing
    /// unsupported-member diagnostic, unchanged in kind from prior behavior.
    fn lower_first_merge_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        body: &FirstMergeBody,
    ) -> Result<(), ConstructionError> {
        let FirstMergeBody::Brace(brace) = body else {
            return Ok(());
        };
        for element in &brace.value.elements {
            match &element.value {
                FirstMergeBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                FirstMergeBodyElement::Unsupported(_) => {
                    self.push_unsupported(document, family, element.span.clone());
                }
                FirstMergeBodyElement::Member(member) => match &member.value {
                    ActionDefBodyElement::ActionUsage(action_usage) => {
                        self.lower_action_usage(document, Some(owner), action_usage)?;
                    }
                    ActionDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(document, Some(owner), family, param)?;
                    }
                    ActionDefBodyElement::ThenAction(then_action) => {
                        self.lower_then_action(document, owner, family, then_action)?;
                    }
                    ActionDefBodyElement::Doc(node) => {
                        self.record_doc_comment(owner, node)?;
                    }
                    ActionDefBodyElement::TextualRep(node) => {
                        self.record_textual_representation(owner, node)?;
                    }
                    ActionDefBodyElement::MergeStmt(node) => self.lower_first_merge_stmt(
                        document,
                        owner,
                        family,
                        DeclarationKind::Merge,
                        ReferenceKind::MergeInput,
                        node.span.clone(),
                        &node.value.merge,
                        &node.value.body,
                    )?,
                    ActionDefBodyElement::DecisionStmt(node) => self.lower_first_merge_stmt(
                        document,
                        owner,
                        family,
                        DeclarationKind::Decide,
                        ReferenceKind::DecisionInput,
                        node.span.clone(),
                        &node.value.decide,
                        &node.value.body,
                    )?,
                    ActionDefBodyElement::JoinStmt(node) => self.lower_first_merge_stmt(
                        document,
                        owner,
                        family,
                        DeclarationKind::Join,
                        ReferenceKind::JoinInput,
                        node.span.clone(),
                        &node.value.join,
                        &node.value.body,
                    )?,
                    ActionDefBodyElement::ForkStmt(node) => self.lower_first_merge_stmt(
                        document,
                        owner,
                        family,
                        DeclarationKind::Fork,
                        ReferenceKind::ForkInput,
                        node.span.clone(),
                        &node.value.fork,
                        &node.value.body,
                    )?,
                    _ => self.push_unsupported(document, family, element.span.clone()),
                },
            }
        }
        Ok(())
    }

    /// Lowers a `then <target>;` continuation statement (BNF `ThenAction`) found either as a
    /// direct action def/usage body element or nested inside a `decide`/`merge`/`fork`/`join`
    /// node's braced body: dispatches on `ThenTarget` to whichever existing lowering function
    /// already handles that target's own AST shape (an inline `action`/`perform` declaration, a
    /// nested `merge`/`fork`/`decide` control node, or a bare feature reference to an
    /// already-declared sibling node), so no new resolution machinery is written here. The
    /// `Accept` shorthand-trigger target is deliberately out of scope for this slice (mirroring
    /// `Transition`'s own `TransitionAccept::Payload`/`TimeTrigger` deferral) and falls through to
    /// the existing unsupported-member diagnostic.
    fn lower_then_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<ThenAction>,
    ) -> Result<(), ConstructionError> {
        match &node.value.target {
            // `then action ...;` and `then send ... to ...;` are the same `ActionUsage` shape
            // upstream -- the send form is the one carrying `send`/`via`/`to` clauses, which
            // `lower_action_usage` already reads off the usage itself.
            ThenTarget::Action(action_usage) | ThenTarget::Send(action_usage) => {
                self.lower_action_usage(document, Some(owner), action_usage)?;
            }
            ThenTarget::Perform(perform) => {
                self.lower_perform(document, Some(owner), perform)?;
            }
            ThenTarget::Merge(merge_stmt) => self.lower_first_merge_stmt(
                document,
                owner,
                family,
                DeclarationKind::Merge,
                ReferenceKind::MergeInput,
                merge_stmt.span.clone(),
                &merge_stmt.value.merge,
                &merge_stmt.value.body,
            )?,
            ThenTarget::Fork(fork_stmt) => self.lower_first_merge_stmt(
                document,
                owner,
                family,
                DeclarationKind::Fork,
                ReferenceKind::ForkInput,
                fork_stmt.span.clone(),
                &fork_stmt.value.fork,
                &fork_stmt.value.body,
            )?,
            ThenTarget::Decide(decision_stmt) => self.lower_first_merge_stmt(
                document,
                owner,
                family,
                DeclarationKind::Decide,
                ReferenceKind::DecisionInput,
                decision_stmt.span.clone(),
                &decision_stmt.value.decide,
                &decision_stmt.value.body,
            )?,
            ThenTarget::Feature(expression) => {
                let declaration = self.push_typed_declaration(
                    document,
                    Some(owner),
                    DeclarationKind::ThenContinuation,
                    None,
                    node.span.clone(),
                    // A synthesized scope for the `then <feature>` continuation target.
                    DeclarationFacts::none(),
                )?;
                self.push_membership(
                    declaration,
                    MembershipKind::Feature,
                    Visibility::Default,
                    node.span.clone(),
                )?;
                self.lower_succession_end(
                    document,
                    declaration,
                    family,
                    ReferenceKind::ThenTarget,
                    expression,
                )?;
            }
            ThenTarget::Accept(accept) => {
                self.lower_then_accept(document, owner, family, accept)?;
            }
        }
        Ok(())
    }

    /// Lowers an `assign <target> := <value>;` reassignment statement (BNF `AssignStmt`, `ast::
    /// AssignStmt`; `is_then` is not modeled as a distinct fact -- both the plain and `then assign
    /// ...;` spellings resolve identically) as its own anonymous `DeclarationKind::Assign` feature
    /// owned by `owner`, mirroring `lower_bind`'s "statement, not a new named usage" shape: `lhs`
    /// is lowered as an `AssignTarget` reference through the exact `lower_succession_end` `FeatureRef`/
    /// `MemberAccess` dispatch every other paired/single-operand control-flow reference kind uses,
    /// and `rhs` is lowered through the shared value-assignment pipeline (`classify_constraint_
    /// expression`/`lower_constraint_expression`), publishing its own evaluation fact exactly like
    /// an attribute default value.
    fn lower_assign_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        span: Span,
        node: &AssignStmt,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Assign,
            None,
            span.clone(),
            // A synthesized scope for the assignment's target/value references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        self.lower_succession_end(
            document,
            declaration,
            family,
            ReferenceKind::AssignTarget,
            &node.lhs,
        )?;
        self.push_evaluation_fact(declaration, classify_constraint_expression(&node.rhs.value));
        self.lower_constraint_expression(document, declaration, family, &node.rhs)
    }

    /// Lowers a `while <condition> { ... }` (BNF `WhileStmt`) or bare `loop { ... }` (BNF
    /// `LoopStmt`, no condition) control node as its own anonymous nested-declaration feature
    /// owned by `owner`, mirroring `lower_first_merge_stmt`'s shape: an optional boolean
    /// `condition` is lowered through the same `classify_constraint_expression`/
    /// `lower_constraint_expression` machinery already used for `decide`'s branch guards/
    /// transition guards/filter conditions (a loop condition is a genuine boolean expression, not
    /// a control-node reference, unlike `decide`/`merge`/`fork`/`join`'s own operand), and the
    /// body's nested statements recurse through `lower_action_def_body` -- the same dispatch this
    /// helper is itself reached from, since a nested body is always typed `ActionDefBody`
    /// regardless of whether the enclosing action is a def or a usage.
    #[allow(clippy::too_many_arguments)]
    fn lower_while_or_loop_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        decl_kind: DeclarationKind,
        span: Span,
        condition: Option<&Node<Expression>>,
        body: &ActionDefBody,
    ) -> Result<(), ConstructionError> {
        // A synthesized control-flow scope with no authored declaration syntax of its own.
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            decl_kind,
            None,
            span.clone(),
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        if let Some(condition) = condition {
            self.push_evaluation_fact(
                declaration,
                classify_constraint_expression(&condition.value),
            );
            self.lower_constraint_expression(document, declaration, family, condition)?;
        }
        self.lower_action_def_body(document, declaration, body)
    }

    /// Lowers an `if <condition> { ... } (else { ... })?` control node (BNF `IfStmt`) as its own
    /// anonymous nested-declaration feature owned by `owner`, same condition handling as
    /// `lower_while_or_loop_stmt`. Both `then_body` and `else_body` (when present) recurse through
    /// `lower_action_def_body`, owned by this one `If` declaration -- branch bodies are not
    /// distinguished from one another as separate declaration scopes, mirroring how `decide`'s own
    /// braced body is a single undifferentiated scope.
    fn lower_if_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        span: Span,
        node: &IfStmt,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::If,
            None,
            span.clone(),
            // A synthesized scope for the conditional's guard and branches.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        self.push_evaluation_fact(
            declaration,
            classify_constraint_expression(&node.condition.value),
        );
        self.lower_constraint_expression(document, declaration, family, &node.condition)?;
        self.lower_action_branch_body(document, declaration, &node.then_body)?;
        if let Some(else_body) = &node.else_body {
            self.lower_action_branch_body(document, declaration, else_body)?;
        }
        Ok(())
    }

    /// Lowers one branch of an `if` control node (`ast::ActionBranchBody`). The grammar offers two
    /// spellings -- a braced action body, or a single member written without braces (`if x then
    /// y;`) -- which are different authored syntax and so different states upstream. They own the
    /// same members either way, so both dispatch through the same walker; the branch keeps no
    /// declaration scope of its own (see `lower_if_stmt`).
    fn lower_action_branch_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionBranchBody,
    ) -> Result<(), ConstructionError> {
        match body {
            ActionBranchBody::Braced(body) => self.lower_action_def_body(document, owner, body),
            ActionBranchBody::Shorthand(element) => {
                self.lower_action_def_body_element(document, owner, element)
            }
        }
    }

    /// Lowers a `for <var> in <range> { ... }` loop control node (BNF `ForLoop`) as its own
    /// anonymous `DeclarationKind::ForLoop` nested-declaration feature owned by `owner`, mirroring
    /// `lower_while_or_loop_stmt`'s shape: the `range` collection expression is lowered through
    /// the same `classify_constraint_expression`/`lower_constraint_expression` machinery as
    /// `while`'s condition, sourced at this `ForLoop` declaration (the range is evaluated once per
    /// loop, not once per iteration binding). `var` (a bare, untyped `String` -- the parser
    /// records no type/multiplicity for it) is lowered as a named `DeclarationKind::
    /// ForLoopVariable` feature owned by this same `ForLoop` declaration -- introducing a binding,
    /// not a reference, mirroring `InOutDecl`'s own scope boundary -- so it is a visible sibling
    /// through the shared `DeclarationDomain::Any` lexical lookup the body's own statements use.
    /// The body then recurses through `lower_action_def_body`, owned by this `ForLoop`
    /// declaration. No type inference from `range`'s element type is performed; this is
    /// reference-resolution scope only, not iteration-execution semantics.
    fn lower_for_loop(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        span: Span,
        node: &ForLoop,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ForLoop,
            None,
            span.clone(),
            // A synthesized scope owning the loop variable and range expression.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span.clone(),
        )?;
        self.push_evaluation_fact(
            declaration,
            classify_constraint_expression(&node.range.value),
        );
        self.lower_constraint_expression(document, declaration, family, &node.range)?;
        let var_name = self.intern_declared_name(&node.var)?;
        let var_declaration = self.push_typed_declaration(
            document,
            Some(declaration),
            DeclarationKind::ForLoopVariable,
            var_name,
            span.clone(),
            // `ast::ForLoop::var` is a bare `String`; the parser records no type, multiplicity, or
            // modifier for the loop variable.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            var_declaration,
            MembershipKind::Feature,
            Visibility::Default,
            span,
        )?;
        self.lower_action_def_body(document, declaration, &node.body)
    }

    /// Lowers a `then accept ...;` shorthand trigger (BNF `ThenTarget::Accept`, `ast::
    /// TransitionAccept`) -- the inline accept-trigger form reused unchanged from `Transition`'s
    /// own `accept` clause, deliberately deferred by `39bd06fc`. Sourced directly at `owner` (the
    /// enclosing action def/usage declaration), not an anonymous nested declaration: an accept
    /// trigger's operand is looked up in the action's own enclosing scope (e.g. `accept sig after
    /// ...;`'s `sig`, `accept when b.f;`'s `b`), not among the action's own children the way a
    /// `Succession`/`Decide`/`Merge` end is. Dispatches on the three `TransitionAccept` shapes:
    /// `Shorthand`'s expression and `TimeTrigger`'s (`at`/`when`/`after`) expression both reuse
    /// `lower_constraint_expression` directly (picking up its `FeatureRef`/`MemberAccess`/
    /// `Invocation`/`Constructor` dispatch, e.g. `accept at new Time::Iso8601DateTime(...)`'s
    /// constructor callee/argument); `Payload`'s typed `: Type` suffix reuses
    /// `lower_payload_clause_type`. Either shape's optional trailing `via <port>` clause resolves
    /// as an `AcceptVia` reference through `lower_satisfy_operand`.
    fn lower_then_accept(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        accept: &Node<TransitionAccept>,
    ) -> Result<(), ConstructionError> {
        match &accept.value {
            TransitionAccept::Shorthand(expr, via) => {
                self.lower_constraint_expression(document, owner, family, expr)?;
                if let Some(via) = via {
                    self.lower_satisfy_operand(
                        document,
                        owner,
                        family,
                        ReferenceKind::AcceptVia,
                        via,
                    )?;
                }
            }
            TransitionAccept::TimeTrigger(_kind, expr) => {
                self.lower_constraint_expression(document, owner, family, expr)?;
            }
            TransitionAccept::Payload(clause, via) => {
                self.lower_payload_clause_type(document, owner, clause)?;
                if let Some(via) = via {
                    self.lower_satisfy_operand(
                        document,
                        owner,
                        family,
                        ReferenceKind::AcceptVia,
                        via,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Lowers a standalone, unnamed `flow (of <payload>)? <source> to <target>;` body element
    /// (BNF `FlowUsage`'s bare from/to shorthand, `ast::FlowUsage`) found inside an action
    /// def/usage body. Mirrors `lower_allocate`/`lower_bind`: an anonymous `DeclarationKind::Flow`
    /// feature owned by `owner`, with `from`/`to` lowered as authored `FlowSource`/`FlowTarget`
    /// references through `lower_kerml_connector_end` -- upstream types both ends as
    /// `KermlConnectorEnd`, the same connector-end shape the KerML connector, binding and
    /// succession members carry, so each end's `target` resolves through the shared
    /// `DeclarationDomain::Any` lexical lookup directly -- and the optional `of
    /// <payload>` clause's type resolved as a `FlowPayloadType` reference (mirroring
    /// `AcceptPayloadType`). A `: Type` clause on the flow itself (`type_name`) is a structurally
    /// distinct declaration form and stays unsupported, as does a *named* flow
    /// (`node.value.name.is_some()`, e.g. `flow generateToAmplify from a to b;`). The upstream
    /// misparse that made a name untrustworthy -- the canonical `flow from <a> to <b>;` shorthand
    /// consuming its own `from` keyword as the declared name -- is fixed, so an authored name is
    /// now a real one; lowering the named form is pending (planning/UPSTREAM_PARSER_GAPS.md,
    /// "Typed upstream, not yet lowered here").
    fn lower_flow_usage(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<FlowUsage>,
    ) -> Result<(), ConstructionError> {
        if node.value.name.is_some() || node.value.type_name.is_some() {
            self.push_unsupported(document, family, node.span.clone());
            return Ok(());
        }
        let (Some(from), Some(to)) = (&node.value.from, &node.value.to) else {
            self.push_unsupported(document, family, node.span.clone());
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Flow,
            None,
            node.span.clone(),
            // `ast::FlowUsage` carries no modifier, multiplicity, direction, or short name; its
            // payload/from/to facts are lowered as references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(payload) = &node.value.payload {
            if let Some(type_name) = payload.value.type_name {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(type_name)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::FlowPayloadType,
                    document,
                    local: type_name,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
        }
        self.lower_kerml_connector_end(document, declaration, ReferenceKind::FlowSource, from)?;
        self.lower_kerml_connector_end(document, declaration, ReferenceKind::FlowTarget, to)?;
        Ok(())
    }

    /// Lowers a `terminate <target>;`/bare `terminate;` body element (BNF `TerminateStmt`, `ast::
    /// TerminateStmt`) found inside an action def/usage body. The optional `target` is resolved as
    /// a `TerminateTarget` reference through the shared `lower_satisfy_operand`
    /// `DeclarationDomain::Any` lexical lookup, sourced directly at `owner` (the enclosing action
    /// def/usage declaration) -- unlike `Succession`/`Decide`, no anonymous nested-declaration
    /// scope shift is needed because the terminated node/action is looked up in the terminate
    /// statement's own enclosing scope, where sibling action names like `terminate c1;`'s `c1` are
    /// actually declared. The bare `terminate;` form (no target) has nothing to resolve and is a
    /// legitimate no-op, not an unsupported construct.
    fn lower_terminate_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<TerminateStmt>,
    ) -> Result<(), ConstructionError> {
        if let Some(target) = &node.value.target {
            self.lower_satisfy_operand(
                document,
                owner,
                family,
                ReferenceKind::TerminateTarget,
                target,
            )?;
        }
        Ok(())
    }

    /// Lowers a `constraint def`/`constraint` usage body's boolean expression (slice 1 of the
    /// constraint/calc expression fact family, widened by the arithmetic/logical-combinator slice
    /// to accept nested arithmetic and `and`/`or` combinators; see `ReferenceKind::
    /// ExpressionOperand`). Supports a literal, a feature/feature-chain reference (resolved as an
    /// `ExpressionOperand` reference sourced at `declaration`, exactly like `lower_succession_end`
    /// resolves `Expression::FeatureRef` through the shared `DeclarationDomain::Any` lexical lookup
    /// fixed point), a parenthesized wrapper (unwrapped and recursed into), a comparison `BinaryOp`
    /// (`Eq`/`Ne`/`Lt`/`Le`/`Gt`/`Ge` -- `StrictEq`/`StrictNe` KerML identity comparisons are
    /// deliberately excluded, left unsupported like every other operator), an arithmetic `BinaryOp`
    /// (`is_arithmetic_operator`, e.g. an operand like `chassisMass + engine.mass`), or a logical
    /// `BinaryOp` (`is_logical_operator`, `and`/`or`/`xor`/`implies`, combining multiple comparisons, e.g. `... and
    /// mass > 0[kg]`; `xor`/`implies` deliberately excluded) -- every one of these `BinaryOp` arms
    /// simply recurses into both operands identically, since reference resolution does not care
    /// which of the three operator families is used, only evaluation (`classify_constraint_node`)
    /// distinguishes them by building a different `EvalNode` shape. Evaluation itself is otherwise
    /// out of scope here. Also supports `Expression::Invocation`/`Expression::Constructor`
    /// (reference-resolution slice, see `ReferenceKind::InvocationCallee`/
    /// `lower_invocation_callee`): the callee/type name resolves as an `InvocationCallee` reference
    /// and each argument recurses back into this same function, but the invocation is never
    /// evaluated (`EvalNode::Invocation` always folds to `NonConstant`). Any other expression shape
    /// -- tuples, type-check/classification expressions, unary ops, etc. -- falls through to the
    /// existing unsupported-member diagnostic, unchanged from prior behavior.
    fn lower_constraint_expression(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::LiteralInteger(_)
            | Expression::LiteralReal(_)
            | Expression::LiteralBoolean(_)
            | Expression::LiteralString(_)
            | Expression::LiteralWithUnit { .. }
            | Expression::Null => Ok(()),
            Expression::Index { base, index } => {
                self.lower_constraint_expression(document, declaration, family, base)?;
                self.lower_constraint_expression(document, declaration, family, index)
            }
            Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ExpressionOperand,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
            Expression::MemberAccess { .. } => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(
                        declaration,
                        document,
                        &chain,
                        node.span.clone(),
                    )?;
                } else {
                    self.push_unsupported(document, family, node.span.clone());
                }
                Ok(())
            }
            Expression::Parenthesized(inner) => {
                self.lower_constraint_expression(document, declaration, family, inner)
            }
            Expression::BinaryOp { op, left, right }
                if is_comparison_operator(op)
                    || is_arithmetic_operator(op)
                    || is_logical_operator(op)
                    || is_range_or_coalesce_operator(op) =>
            {
                self.lower_constraint_expression(document, declaration, family, left)?;
                self.lower_constraint_expression(document, declaration, family, right)
            }
            Expression::Invocation { callee, args } => {
                self.lower_invocation_callee(document, declaration, callee)?;
                for arg in args {
                    self.lower_constraint_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::Constructor { type_name, args } => {
                self.push_invocation_callee_reference(document, declaration, *type_name)?;
                for arg in args {
                    self.lower_constraint_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::CollectionOp { base, args, .. } => {
                self.lower_constraint_expression(document, declaration, family, base)?;
                for arg in args {
                    self.lower_constraint_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::Tuple(items) => {
                for item in items {
                    self.lower_constraint_expression(document, declaration, family, item)?;
                }
                Ok(())
            }
            Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
                self.lower_constraint_expression(document, declaration, family, operand)
            }
            Expression::Conditional {
                test,
                then_expr,
                else_expr,
            } => {
                self.lower_constraint_expression(document, declaration, family, test)?;
                self.lower_constraint_expression(document, declaration, family, then_expr)?;
                self.lower_constraint_expression(document, declaration, family, else_expr)
            }
            Expression::TypeCheck {
                operand, type_name, ..
            } => {
                if let Some(operand) = operand {
                    self.lower_constraint_expression(document, declaration, family, operand)?;
                }
                self.push_type_check_target_reference(document, declaration, *type_name)
            }
            Expression::MetaCast { base, metaclass } => {
                self.lower_constraint_expression(document, declaration, family, base)?;
                self.push_meta_cast_target_reference(document, declaration, *metaclass)
            }
            _ => {
                self.push_unsupported(document, family, node.span.clone());
                Ok(())
            }
        }
    }

    /// Lowers a `calc def`/`calc` usage body's formula expression (slice 1 of the constraint/calc
    /// expression fact family, extended by slice 4 for arithmetic). Originally scoped to
    /// arithmetic-only `BinaryOp` support on the theory that calc bodies are typically
    /// arithmetic-result formulas rather than boolean comparisons -- but the exhaustive
    /// `unsupported_calc_definition_member` audit found this premise false for a large share of
    /// the real corpus (Kernel Function Library equality/comparison functions like
    /// `BaseFunctions.kerml`'s `return : Boolean[1] = not (x == y);`, KerML `inv { ... }`
    /// boolean-invariant bodies reusing this same `CalcDefBody`/`lower_calc_def_body` walker per
    /// `KermlInvariantMember`, etc.), so comparison/logical `BinaryOp` support now matches
    /// `lower_constraint_expression`'s `BinaryOp` arm exactly (`is_comparison_operator`/
    /// `is_logical_operator`, alongside `is_arithmetic_operator`'s `Add`/`Sub`/`Mul`/`Div`/`Mod`/
    /// `Exp`/`Pow`). This slice supports the minimal leaf shapes -- a literal, a feature/
    /// feature-chain reference (resolved as an `ExpressionOperand` reference exactly like
    /// `lower_constraint_expression`), a parenthesized wrapper -- plus every comparison/
    /// arithmetic/logical `BinaryOp` whose operands are recursed into just like
    /// `lower_constraint_expression`'s own `BinaryOp` arm. Also supports `Expression::Invocation`/
    /// `Expression::Constructor` (e.g. `sum(partMasses)`, `new PusherOutput(pusherForce)`),
    /// exactly like `lower_constraint_expression`'s own Invocation/Constructor arm: reference
    /// resolution only, never evaluation. Also supports a unary `-`/`not` `UnaryOp` (see
    /// `is_unary_operator`), recursing into its single operand exactly like `Parenthesized`, and a
    /// `Conditional` (`if <test> ? <then> else <else>`), recursing into all three sub-expressions
    /// reference-resolution-only exactly like `Tuple`'s multi-operand shape (no evaluation of
    /// which branch is taken). Every other expression shape stays unsupported, falling through to
    /// the existing `unsupported_calc_definition_member` diagnostic, unchanged from prior
    /// behavior.
    fn lower_calc_expression(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::LiteralInteger(_)
            | Expression::LiteralReal(_)
            | Expression::LiteralBoolean(_)
            | Expression::LiteralString(_)
            | Expression::LiteralWithUnit { .. }
            | Expression::Null => Ok(()),
            Expression::Index { base, index } => {
                self.lower_calc_expression(document, declaration, family, base)?;
                self.lower_calc_expression(document, declaration, family, index)
            }
            Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ExpressionOperand,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
            Expression::MemberAccess { .. } => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(
                        declaration,
                        document,
                        &chain,
                        node.span.clone(),
                    )?;
                } else {
                    self.push_unsupported(document, family, node.span.clone());
                }
                Ok(())
            }
            Expression::Parenthesized(inner) => {
                self.lower_calc_expression(document, declaration, family, inner)
            }
            Expression::BinaryOp { op, left, right }
                if is_arithmetic_operator(op)
                    || is_comparison_operator(op)
                    || is_logical_operator(op)
                    || is_range_or_coalesce_operator(op) =>
            {
                self.lower_calc_expression(document, declaration, family, left)?;
                self.lower_calc_expression(document, declaration, family, right)
            }
            Expression::Invocation { callee, args } => {
                self.lower_invocation_callee(document, declaration, callee)?;
                for arg in args {
                    self.lower_calc_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::Constructor { type_name, args } => {
                self.push_invocation_callee_reference(document, declaration, *type_name)?;
                for arg in args {
                    self.lower_calc_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::CollectionOp { base, args, .. } => {
                self.lower_calc_expression(document, declaration, family, base)?;
                for arg in args {
                    self.lower_calc_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::Tuple(items) => {
                for item in items {
                    self.lower_calc_expression(document, declaration, family, item)?;
                }
                Ok(())
            }
            Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
                self.lower_calc_expression(document, declaration, family, operand)
            }
            Expression::Conditional {
                test,
                then_expr,
                else_expr,
            } => {
                self.lower_calc_expression(document, declaration, family, test)?;
                self.lower_calc_expression(document, declaration, family, then_expr)?;
                self.lower_calc_expression(document, declaration, family, else_expr)
            }
            Expression::TypeCheck {
                operand, type_name, ..
            } => {
                if let Some(operand) = operand {
                    self.lower_calc_expression(document, declaration, family, operand)?;
                }
                self.push_type_check_target_reference(document, declaration, *type_name)
            }
            Expression::MetaCast { base, metaclass } => {
                self.lower_calc_expression(document, declaration, family, base)?;
                self.push_meta_cast_target_reference(document, declaration, *metaclass)
            }
            _ => {
                self.push_unsupported(document, family, node.span.clone());
                Ok(())
            }
        }
    }

    /// Lowers a package-level `filter <expr>;` statement's condition (BNF `ElementFilterMember`,
    /// `ast::FilterMember`, see `PackageBodyElement::Filter`), narrowing a recursive import to only
    /// members satisfying the expression. Reuses `lower_constraint_expression`'s operand-resolution
    /// shape as closely as the filter grammar allows: a literal (recognized, no reference), a
    /// feature/feature-chain reference (`Expression::FeatureRef`/`FeatureChainRef`, e.g.
    /// `Safety::isMandatory`, resolved as `ReferenceKind::ExpressionOperand` through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point), a parenthesized wrapper (unwrapped and
    /// recursed into), and a comparison `BinaryOp` (`is_comparison_operator`) whose operands are
    /// recursed into, exactly like `lower_constraint_expression`.
    ///
    /// Two shapes are specific to filter conditions and have no analog in
    /// `lower_constraint_expression`: an `@Name` metadata-classification test
    /// (`Expression::Classification`, e.g. `@Safety`) is resolved as a new
    /// `ReferenceKind::FilterMetadataTest` reference through the same `DeclarationDomain::Type`
    /// lexical lookup fixed point `MetadataAnnotation` uses (`Safety` must name a metadata def);
    /// and a logical `BinaryOp` (`and`/`or`/`xor`/`implies`, `is_logical_operator`) whose operands are recursed
    /// into, alongside comparison operators.
    ///
    /// `declaration` is the enclosing package's own declaration (the filter statement's owner,
    /// sourced directly, no anonymous nested-declaration scope shift -- mirroring
    /// `ExpressionOperand`'s shape). Evaluation of the filter (computing which imported members
    /// actually pass it) is explicitly out of scope for this slice; only the condition's own
    /// references are resolved. Any other expression shape falls through to
    /// `UnsupportedFamily::PackageMember`'s `unsupported_package_member` diagnostic, matching the
    /// unconditional `unsupported_package_member` this statement produced before this slice.
    fn lower_filter_expression(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::LiteralInteger(_)
            | Expression::LiteralReal(_)
            | Expression::LiteralBoolean(_)
            | Expression::LiteralString(_)
            | Expression::LiteralWithUnit { .. } => Ok(()),
            Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ExpressionOperand,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
            Expression::Classification { metaclass } => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*metaclass)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::FilterMetadataTest,
                    document,
                    local: *metaclass,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
            Expression::MemberAccess { .. } => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(
                        declaration,
                        document,
                        &chain,
                        node.span.clone(),
                    )?;
                } else {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::PackageMember,
                        node.span.clone(),
                    );
                }
                Ok(())
            }
            Expression::Parenthesized(inner) => {
                self.lower_filter_expression(document, declaration, inner)
            }
            Expression::BinaryOp { op, left, right }
                if is_comparison_operator(op) || is_logical_operator(op) =>
            {
                self.lower_filter_expression(document, declaration, left)?;
                self.lower_filter_expression(document, declaration, right)
            }
            Expression::Invocation { callee, args } => {
                self.lower_invocation_callee(document, declaration, callee)?;
                for arg in args {
                    self.lower_filter_expression(document, declaration, &arg.value)?;
                }
                Ok(())
            }
            Expression::Constructor { type_name, args } => {
                self.push_invocation_callee_reference(document, declaration, *type_name)?;
                for arg in args {
                    self.lower_filter_expression(document, declaration, &arg.value)?;
                }
                Ok(())
            }
            Expression::CollectionOp { base, args, .. } => {
                self.lower_filter_expression(document, declaration, base)?;
                for arg in args {
                    self.lower_filter_expression(document, declaration, &arg.value)?;
                }
                Ok(())
            }
            Expression::Tuple(items) => {
                for item in items {
                    self.lower_filter_expression(document, declaration, item)?;
                }
                Ok(())
            }
            Expression::TypeCheck {
                operand, type_name, ..
            } => {
                if let Some(operand) = operand {
                    self.lower_filter_expression(document, declaration, operand)?;
                }
                self.push_type_check_target_reference(document, declaration, *type_name)
            }
            Expression::MetaCast { base, metaclass } => {
                self.lower_filter_expression(document, declaration, base)?;
                self.push_meta_cast_target_reference(document, declaration, *metaclass)
            }
            Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
                self.lower_filter_expression(document, declaration, operand)
            }
            _ => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::PackageMember,
                    node.span.clone(),
                );
                Ok(())
            }
        }
    }

    /// Lowers a `state def` (BNF StateDefinition), mirroring `lower_action_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned declarations.
    /// State-machine-specific semantics (entry/do/exit action bindings, transitions, exclusive/
    /// parallel substates, history) are explicitly out of scope; unrecognized body elements fall
    /// through to `unsupported_state_definition_member` via `lower_state_def_body`.
    fn lower_state_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<StateDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_state_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the `StateDefBody` shared by `state def` and by a `state` usage's own owned
    /// members (BNF `StateDefBodyElement`): nested state/requirement usages, entry/do/exit action
    /// bindings, `then`/`final` state markers, `ref` bindings, and transitions are all lowered.
    /// `StateDefBodyElement` also carries `AttributeUsage`/`ActionUsage`/`AssertConstraint`/
    /// `SuccessionUsage` variants; the first three dispatch to their existing lowerings here, and
    /// a `succession` member stays on `unsupported_state_definition_member`.
    fn lower_state_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &StateDefBody,
    ) -> Result<(), ConstructionError> {
        let StateDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                StateDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                StateDefBodyElement::StateUsage(state_usage) => {
                    self.lower_state_usage(document, Some(owner), state_usage)?;
                }
                StateDefBodyElement::RequirementUsage(requirement_usage) => {
                    self.lower_requirement_usage(document, Some(owner), requirement_usage)?;
                }
                StateDefBodyElement::Doc(node) => {
                    self.record_doc_comment(owner, node)?;
                }
                StateDefBodyElement::Entry(entry) => {
                    self.lower_state_entry_action(document, owner, entry)?;
                }
                StateDefBodyElement::Do(action) => {
                    self.lower_state_do_action(document, owner, action)?;
                }
                StateDefBodyElement::Exit(exit) => {
                    self.lower_state_exit_action(document, owner, exit)?;
                }
                StateDefBodyElement::Then(then) => {
                    self.lower_state_then_stmt(document, owner, then)?;
                }
                StateDefBodyElement::Transition(transition) => {
                    self.lower_transition(document, owner, transition)?;
                }
                StateDefBodyElement::MetadataAnnotation(node) => {
                    self.lower_metadata_annotation(document, owner, node)?;
                }
                StateDefBodyElement::InOutDecl(param) => {
                    self.lower_parameter_declaration(
                        document,
                        Some(owner),
                        UnsupportedFamily::StateDefinitionMember,
                        param,
                    )?;
                }
                StateDefBodyElement::Ref(node) => {
                    self.lower_ref_decl(document, Some(owner), node)?;
                }
                StateDefBodyElement::FinalState(node) => {
                    self.lower_final_state(document, owner, node)?;
                }
                StateDefBodyElement::AttributeUsage(node) => {
                    self.lower_attribute_usage(document, Some(owner), node)?;
                }
                StateDefBodyElement::ActionUsage(node) => {
                    self.lower_action_usage(document, Some(owner), node)?;
                }
                StateDefBodyElement::AssertConstraint(node) => self
                    .lower_assert_constraint_member(
                        document,
                        owner,
                        UnsupportedFamily::StateDefinitionMember,
                        node,
                    )?,
                StateDefBodyElement::Annotation(_)
                | StateDefBodyElement::SuccessionUsage(_)
                | StateDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers a state def/usage's `entry action <path> ...;` body element (BNF `EntryAction`) as
    /// an anonymous `DeclarationKind::EntryActionBinding` feature owned by the enclosing state
    /// `owner` declaration, mirroring `lower_first_stmt`'s nested-declaration shape so the bound
    /// action reference resolves against the state's own scope (where sibling actions are
    /// declared), not the state's enclosing scope. `EntryAction.action_reference` is already a
    /// structured `QualifiedReferenceId` (not a flattened string), so it resolves through the
    /// same shared lexical lookup as `AliasBinding`/`Succession`. A plain `entry` with no bound
    /// action (`action_reference: None`) has no reference to lower: a bare `entry;`/empty `entry
    /// { }` (no owned members) is a legal no-op marker with genuinely nothing to represent, so it
    /// is silently recognized rather than reported (pervasive in the training/validation corpus,
    /// e.g. `24_state_actions.md`'s `entry; then off;`); an inline `entry { <members> }` body with
    /// actual owned content has no representation in this typed AST shape (no field carries it)
    /// and stays an explicit unsupported diagnostic.
    fn lower_state_entry_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<EntryAction>,
    ) -> Result<(), ConstructionError> {
        let Some(target) = node.value.action_reference else {
            if state_action_body_has_content(&node.value.body) {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::EntryActionBinding,
            None,
            node.span.clone(),
            // A synthesized scope for the state's entry-action binding reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.push_action_binding_reference(
            document,
            declaration,
            ReferenceKind::EntryActionBinding,
            target,
        )
    }

    /// Same as `lower_state_entry_action`, for a `do action <path> ...;` body element
    /// (`DoAction.action_reference`).
    fn lower_state_do_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<DoAction>,
    ) -> Result<(), ConstructionError> {
        let Some(target) = node.value.action_reference else {
            if state_action_body_has_content(&node.value.body) {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::DoActionBinding,
            None,
            node.span.clone(),
            // A synthesized scope for the state's do-action binding reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.push_action_binding_reference(
            document,
            declaration,
            ReferenceKind::DoActionBinding,
            target,
        )
    }

    /// Same as `lower_state_entry_action`, for an `exit action <path> ...;` body element
    /// (`ExitAction.action_reference`).
    fn lower_state_exit_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ExitAction>,
    ) -> Result<(), ConstructionError> {
        let Some(target) = node.value.action_reference else {
            if state_action_body_has_content(&node.value.body) {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ExitActionBinding,
            None,
            node.span.clone(),
            // A synthesized scope for the state's exit-action binding reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.push_action_binding_reference(
            document,
            declaration,
            ReferenceKind::ExitActionBinding,
            target,
        )
    }

    /// Lowers a state def/usage's `then <target>;` initial-state body element (BNF `ThenStmt`,
    /// the bare initial-state marker -- distinct from a full `transition ... then ...;`
    /// construct, which stays out of scope) as an anonymous `DeclarationKind::InitialState`
    /// feature owned by the enclosing state `owner` declaration, mirroring
    /// `lower_state_entry_action`. `ThenStmt.state_reference` is already a structured
    /// `QualifiedReferenceId`, so it always resolves through the same shared lexical lookup.
    fn lower_state_then_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ThenStmt>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::InitialState,
            None,
            node.span.clone(),
            // A synthesized scope for the `then <state>` initial-state reference.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.push_action_binding_reference(
            document,
            declaration,
            ReferenceKind::InitialState,
            node.value.state_reference,
        )
    }

    /// Lowers a state def/usage's `final <name>;`/`final state <name>;` body element (BNF
    /// `FinalState`) as a named `DeclarationKind::FinalState` feature owned by the enclosing state
    /// `owner` declaration, mirroring `lower_state_usage`'s plain named-declaration shape.
    /// `FinalState.state_name` is always a non-empty declared name per the grammar (`final` is
    /// always followed by a mandatory `name`), so this declares a genuine new nested state rather
    /// than referencing an existing one -- unlike `lower_state_then_stmt`'s `InitialState`, there
    /// is no target reference to resolve.
    fn lower_final_state(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<FinalState>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.state_name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::FinalState,
            name,
            node.span.clone(),
            // `ast::FinalState` carries only its state name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        Ok(())
    }

    /// Shared helper for `lower_state_entry_action`/`lower_state_do_action`/
    /// `lower_state_exit_action`/`lower_state_then_stmt`: pushes an authored reference of `kind`
    /// sourced at `declaration` for an already-structured `QualifiedReferenceId` target, mirroring
    /// `lower_alias_def`'s reference-push shape.
    fn push_action_binding_reference(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        kind: ReferenceKind,
        target: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers a `transition ...;` body element (BNF `Transition`, `ast::Transition`) found inside
    /// a state def/usage body as an anonymous `DeclarationKind::Transition` feature owned by the
    /// enclosing state `owner` declaration, mirroring `lower_first_stmt`/`lower_state_entry_
    /// action`'s nested-declaration shape so `source`/`target`/`guard`/`accept`/`effect` all
    /// resolve against the state's own scope (where sibling states/actions are declared), not
    /// the state's enclosing scope. Picks up the full construct explicitly deferred by
    /// `4762b875`; see `DeclarationKind::Transition`'s doc comment for the exact sub-piece scope
    /// boundary.
    fn lower_transition(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<Transition>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Transition,
            name,
            node.span.clone(),
            // `ast::Transition` carries no modifier, multiplicity, direction, or short name; its
            // source/target/trigger/guard/effect facts are lowered as references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(source) = &node.value.source {
            self.lower_transition_end(
                document,
                declaration,
                ReferenceKind::TransitionSource,
                source,
            )?;
        }
        self.lower_transition_end(
            document,
            declaration,
            ReferenceKind::TransitionTarget,
            &node.value.target,
        )?;
        if let Some(guard) = &node.value.guard {
            self.push_evaluation_fact(declaration, classify_constraint_expression(&guard.value));
            self.lower_constraint_expression(
                document,
                declaration,
                UnsupportedFamily::StateDefinitionMember,
                guard,
            )?;
        }
        match &node.value.accept {
            None => {}
            Some(TransitionAccept::Shorthand(expression, _via)) => {
                self.lower_transition_end(
                    document,
                    declaration,
                    ReferenceKind::TransitionTrigger,
                    expression,
                )?;
            }
            Some(TransitionAccept::TimeTrigger(_kind, expression)) => {
                // Mirrors `lower_then_accept`'s `TimeTrigger` arm: the `at`/`when`/`after`
                // trigger expression (e.g. `accept at vehicle.maintenanceTime`) is lowered
                // through the general constraint-expression dispatch (`FeatureRef`/
                // `MemberAccess`/`Invocation`/`Constructor`), the same as a `Transition`'s
                // `guard` clause, rather than `lower_transition_end`'s narrower reference-only
                // dispatch (which `Shorthand` above uses for a bare accepted-signal name). The
                // `TriggerKind` (`at`/`when`/`after`) distinction is not yet represented.
                self.lower_constraint_expression(
                    document,
                    declaration,
                    UnsupportedFamily::StateDefinitionMember,
                    expression,
                )?;
            }
            Some(TransitionAccept::Payload(_, _)) => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
        }
        match &node.value.effect {
            None => {}
            Some(TransitionEffect::Perform {
                type_name: Some(type_name),
                ..
            }) => {
                self.push_action_binding_reference(
                    document,
                    declaration,
                    ReferenceKind::TransitionEffect,
                    *type_name,
                )?;
            }
            Some(TransitionEffect::Expression(expression)) => {
                self.lower_transition_end(
                    document,
                    declaration,
                    ReferenceKind::TransitionEffect,
                    expression,
                )?;
            }
            Some(TransitionEffect::Perform {
                type_name: None, ..
            })
            | Some(TransitionEffect::Accept { .. })
            | Some(TransitionEffect::Send { .. })
            | Some(TransitionEffect::Assign { .. }) => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    node.span.clone(),
                );
            }
        }
        Ok(())
    }

    /// Lowers one `Transition` operand (`source`/`target`/shorthand `accept`/`Expression`
    /// effect): its path expression is a structured `Expression` (not a flattened string), so a
    /// simple/qualified name (`Expression::FeatureRef`) resolves as an authored reference of
    /// `kind` through the same shared `DeclarationDomain::Any` lexical lookup as
    /// `lower_succession_end`. Any other expression shape is left as an explicit unsupported-
    /// member diagnostic, mirroring `lower_succession_end`'s scope boundary.
    fn lower_transition_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        kind: ReferenceKind,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::FeatureRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: owner,
                    kind,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            Expression::MemberAccess { .. } => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(owner, document, &chain, node.span.clone())?;
                } else {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::StateDefinitionMember,
                        node.span.clone(),
                    );
                }
            }
            _ => self.push_unsupported(
                document,
                UnsupportedFamily::StateDefinitionMember,
                node.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers a `satisfy <requirement> by <element>;` body element (BNF `Satisfy`, `ast::
    /// Satisfy`) found inside a package/part def/part usage/occurrence body, as an anonymous
    /// `DeclarationKind::Satisfy` feature owned by the enclosing `owner` declaration, mirroring
    /// `lower_transition`'s nested-declaration shape. Only the bare shorthand form (`satisfy
    /// <ref> by <ref>;`, `Satisfy::inline_requirement == None`) is resolved: both `source` (the
    /// satisfied requirement) and `target` (the `by` satisfying element) are lowered as authored
    /// `SatisfySource`/`SatisfyTarget` references when they are a simple/qualified name
    /// (`Expression::FeatureRef`), resolved through the same `DeclarationDomain::Any` lexical
    /// lookup fixed point `Succession`/`TransitionSource` use. `is_negated` (`not satisfy ...`)
    /// does not change how the references resolve, so it is not modeled as a fact here. Out of
    /// scope, left as an explicit `family` unsupported diagnostic: the fuller `satisfy
    /// requirement <name> : <Type> by <expr>;` form (`inline_requirement`, which defines a new
    /// anonymous requirement usage inline rather than referencing an existing one -- a
    /// meaningfully different construct, not merely an unresolved reference), a dotted
    /// feature-chain operand (`Expression::MemberAccess`), any other expression shape, and the
    /// braced satisfy body's nested constraint members (`body_elements`).
    fn lower_satisfy(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Satisfy>,
    ) -> Result<(), ConstructionError> {
        if node.value.inline_requirement.is_some() {
            self.push_unsupported(document, family, node.span.clone());
            return Ok(());
        }
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Satisfy,
            None,
            node.span.clone(),
            // `ast::Satisfy` carries only `is_negated`, a satisfaction-polarity fact rather than a
            // declaration modifier.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::SatisfySource,
            &node.value.source,
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::SatisfyTarget,
            &node.value.target,
        )?;
        if let Some(elements) = &node.value.body_elements {
            for element in elements {
                self.push_unsupported(document, family, element.span.clone());
            }
        }
        Ok(())
    }

    /// Lowers one `Satisfy` operand (`source`/`target`), mirroring `lower_transition_end`: its
    /// path expression is a structured `Expression`, so a simple/qualified name
    /// (`Expression::FeatureRef`) resolves as an authored reference of `kind` through the shared
    /// `DeclarationDomain::Any` lexical lookup. A dotted feature-chain path
    /// (`Expression::MemberAccess`/`Expression::FeatureChainRef`, e.g. `f.a`) resolves as a
    /// `ReferenceKind::MemberAccessOperand` reference instead, through the same
    /// `flatten_member_access_chain`/`push_member_access_reference` path `lower_connector_end`
    /// uses -- this is also `Bind`'s (`lower_bind`) operand path, since it shares this helper, so
    /// `bind f.a = a.g;` resolves both dotted operands the same way `connect f.a to a.g;` does.
    /// Also supports `Expression::Invocation`/`Expression::Constructor` (reference resolution
    /// only, via `lower_invocation_callee`/`ReferenceKind::InvocationCallee`, recursing arguments
    /// back into this same function with `kind` unchanged). Any other expression shape falls
    /// through to an explicit `family` unsupported diagnostic.
    fn lower_satisfy_operand(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        kind: ReferenceKind,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::FeatureRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: owner,
                    kind,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            Expression::MemberAccess { .. } | Expression::FeatureChainRef(_) => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(owner, document, &chain, node.span.clone())?;
                } else {
                    self.push_unsupported(document, family, node.span.clone());
                }
            }
            Expression::Invocation { callee, args } => {
                self.lower_invocation_callee(document, owner, callee)?;
                for arg in args {
                    self.lower_satisfy_operand(document, owner, family, kind, &arg.value)?;
                }
            }
            Expression::Constructor { type_name, args } => {
                self.push_invocation_callee_reference(document, owner, *type_name)?;
                for arg in args {
                    self.lower_satisfy_operand(document, owner, family, kind, &arg.value)?;
                }
            }
            _ => self.push_unsupported(document, family, node.span.clone()),
        }
        Ok(())
    }

    /// Lowers an `allocate <source> to <target>;` body element (BNF `Allocate`, `ast::Allocate`)
    /// found inside a part def/part usage/occurrence body -- the shorthand allocation
    /// *statement* form, which asserts an allocation relationship between two already-declared
    /// elements without introducing a new named allocation usage (genuinely distinct from
    /// `AllocationDefinition`/`AllocationUsage`, the declaration forms lowered in `04274711`).
    /// Mirrors `lower_satisfy`: an anonymous `DeclarationKind::Allocate` feature owned by `owner`,
    /// with `source`/`target` lowered as authored `AllocateSource`/`AllocateTarget` references
    /// when they are a simple/qualified name (`Expression::FeatureRef`), resolved through the
    /// same `DeclarationDomain::Any` lexical lookup fixed point `Satisfy`/`Succession` use. Unlike
    /// `Satisfy`, `Allocate` has no `inline_requirement`/`body_elements` to gate on: its `body` is
    /// a bare `ConnectBody` (`;` or `{}`) with no structured content, so there is nothing further
    /// to lower or flag as unsupported.
    fn lower_allocate(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Allocate>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Allocate,
            None,
            node.span.clone(),
            // `ast::Allocate` carries only its source/target ends, lowered as references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::AllocateSource,
            &node.value.source,
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::AllocateTarget,
            &node.value.target,
        )?;
        Ok(())
    }

    /// Lowers a `bind <source> = <target>;` body element (BNF `Bind`, `ast::Bind`) found inside a
    /// part def/part usage/action def/action usage body -- the shorthand binding-connector
    /// *statement* form, which asserts a binding-connector relationship between two
    /// already-declared elements without introducing a new named binding-connector usage. Mirrors
    /// `lower_allocate`: an anonymous `DeclarationKind::Bind` feature owned by `owner`, with
    /// `left`/`right` lowered as authored `BindSource`/`BindTarget` references when they are a
    /// simple/qualified name (`Expression::FeatureRef`), resolved through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point `Satisfy`/`Allocate` use (reusing
    /// `lower_satisfy_operand` directly). The optional `binding <name>`/`: Type`/multiplicity
    /// prefix on either end is out of scope. `Bind.body_elements` (BNF `Bind`'s optional braced
    /// body) is typed `Vec<Node<PartUsageBodyElement>>` -- the same part-usage member set
    /// `PartUsageBody` uses (see its own doc comment) -- so each element dispatches through the
    /// shared `lower_part_usage_body_element`, owned by this `Bind`'s own anonymous declaration,
    /// rather than the blanket "every element unsupported" fallback used before that dispatcher
    /// was factored out of `lower_part_usage`.
    fn lower_bind(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Bind>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Bind,
            None,
            node.span.clone(),
            // `ast::Bind` carries only its two bound operands, lowered as references.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::BindSource,
            &node.value.left,
        )?;
        self.lower_satisfy_operand(
            document,
            declaration,
            family,
            ReferenceKind::BindTarget,
            &node.value.right,
        )?;
        for element in &node.value.body_elements {
            self.lower_part_usage_body_element(document, declaration, family, element)?;
        }
        Ok(())
    }

    /// Lowers a package-level `binding ... left = right;` element (BNF `BindingConnectorUsage`,
    /// `ast::BindingConnectorUsage`) -- the keyword-less sibling of `Bind` (see its doc comment),
    /// same binding-connector-statement semantics as `lower_bind` but with `left`/`right` already
    /// structured `QualifiedReferenceId`s rather than `Expression`s, so they resolve directly
    /// through the same `DeclarationDomain::Any` lexical lookup fixed point as `AliasBinding`
    /// (mirroring `lower_alias_def`'s single-reference shape, applied twice). The `all`/name/
    /// multiplicity prefix and any real content in the braced body are out of scope, matching
    /// `Bind`'s own scope boundary.
    fn lower_binding_connector_usage(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<BindingConnectorUsage>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Bind,
            None,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_binding_connector_operand(
            document,
            declaration,
            ReferenceKind::BindSource,
            node.value.left,
        )?;
        self.lower_binding_connector_operand(
            document,
            declaration,
            ReferenceKind::BindTarget,
            node.value.right,
        )?;
        Ok(())
    }

    /// Lowers one `BindingConnectorUsage` operand (`left`/`right`), mirroring `lower_alias_def`'s
    /// `AliasDef::target` handling: an already-structured `QualifiedReferenceId` resolves directly
    /// as an authored reference of `kind` through the shared `DeclarationDomain::Any` lexical
    /// lookup, with no expression-shape gating (`BindingConnectorUsage`'s ends are never a general
    /// `Expression`, unlike `Bind`'s).
    fn lower_binding_connector_operand(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        kind: ReferenceKind,
        target: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: owner,
            kind,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers a view-body `satisfy <viewpoint>;` statement (BNF `ViewBodyElement::Satisfy`,
    /// `ast::SatisfyViewMember`) -- a genuinely distinct AST shape from `Satisfy` (see
    /// `ReferenceKind::SatisfyViewpoint`'s doc comment): a single viewpoint reference, not a
    /// requirement/satisfying-element pair. Sourced directly at the enclosing `view` usage
    /// `declaration` (no anonymous nested-declaration scope shift, unlike `lower_satisfy`), since
    /// there is no second operand to keep distinguishable per-statement.
    fn lower_view_satisfy(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        node: &Node<SatisfyViewMember>,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.viewpoint_ref)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::SatisfyViewpoint,
            document,
            local: node.value.viewpoint_ref,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers an `include <includedUseCase>;` body element inside a `use case def`/`use case`
    /// usage body (BNF `UseCaseDefBodyElement::IncludeUseCase`, `ast::IncludeUseCase`) -- see
    /// `ReferenceKind::IncludeUseCase`'s doc comment: a single-operand reference to an existing
    /// use case, sourced directly at the enclosing use case declaration (no anonymous
    /// nested-declaration scope shift), mirroring `lower_view_satisfy`'s shape.
    fn lower_include_use_case(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        node: &Node<IncludeUseCase>,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::IncludeUseCase,
            document,
            local: node.value.target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers a package/definition/usage-level `state` feature member (BNF StateUsage), e.g.
    /// `state s;` or `state s : SomeState;`, mirroring `lower_action_usage`. `StateUsage`'s
    /// typing is a structured `TypingRelationship` (like `ActionUsage.typing`), not a bare
    /// `QualifiedReferenceId`. Behavioral clauses (`entry`/`do`/`exit`, transitions,
    /// abstract/reference/individual prefixes) are explicitly out of scope; owned members lower
    /// through the same `lower_state_def_body` as a `state def`'s body (both share
    /// `StateDefBody`/`StateDefBodyElement`).
    fn lower_state_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserStateUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    derived: node.value.is_derived,
                    reference: node.value.is_reference,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_state_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a `requirement def` (BNF RequirementDefinition), mirroring `lower_part_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/requirement members. Requirement-specific semantics (subject binding,
    /// assumption/constraint facts) are explicitly out of scope; unrecognized body elements fall
    /// through to `unsupported_requirement_definition_member`.
    fn lower_requirement_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<RequirementDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_requirement_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `requirement` feature member (BNF
    /// RequirementUsage), mirroring `lower_part_usage`: ownership, membership, an optional
    /// `:`/`:>` typing reference, `subsets`/`references` subsetting relationships, and owned
    /// attribute/requirement members.
    fn lower_requirement_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserRequirementUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    variation: node.value.is_variation,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                // `ast::RequirementUsage` has no `multiplicity` field; see
                // planning/UPSTREAM_PARSER_GAPS.md.
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_requirement_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the shared `RequirementDefBody` used by both `requirement def` and `requirement`
    /// usage bodies: recognized owned members are attribute def/usage and nested requirement
    /// usages; everything else falls through to `unsupported_requirement_definition_member` via
    /// the single `RequirementDefinitionMember` family (both def and usage bodies share the same
    /// grammar production, `RequirementBody`, so there is no def/usage-specific distinction to
    /// make here).
    fn lower_requirement_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &RequirementDefBody,
    ) -> Result<(), ConstructionError> {
        self.lower_requirement_shaped_body(
            document,
            owner,
            body,
            UnsupportedFamily::RequirementDefinitionMember,
        )
    }

    /// Shared body walker for grammar productions using `RequirementDefBody`/
    /// `RequirementDefBodyElement` (`requirement def`/`requirement` usage and `viewpoint def`),
    /// parameterized by the caller-supplied `unsupported` family so each kind's diagnostics stay
    /// distinct even though the typed AST body shape is identical.
    fn lower_requirement_shaped_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &RequirementDefBody,
        unsupported: UnsupportedFamily,
    ) -> Result<(), ConstructionError> {
        let RequirementDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                RequirementDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                RequirementDefBodyElement::AttributeDef(attribute) => {
                    self.lower_attribute_def(document, Some(owner), attribute)?;
                }
                RequirementDefBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                RequirementDefBodyElement::RequirementUsage(requirement) => {
                    self.lower_requirement_usage(document, Some(owner), requirement)?;
                }
                RequirementDefBodyElement::Import(import) => {
                    self.lower_import(document, Some(owner), import)?;
                }
                RequirementDefBodyElement::SubjectDecl(subject) => {
                    self.lower_subject_decl(document, Some(owner), subject)?;
                }
                RequirementDefBodyElement::Constraint(constraint) => {
                    self.lower_constraint_usage(document, Some(owner), constraint)?;
                }
                RequirementDefBodyElement::Doc(node) => {
                    self.record_doc_comment(owner, node)?;
                }
                RequirementDefBodyElement::TextualRep(node) => {
                    self.record_textual_representation(owner, node)?;
                }
                RequirementDefBodyElement::MetadataAnnotation(node) => {
                    self.lower_metadata_annotation(document, owner, node)?;
                }
                // `subject;` shorthand: an entirely empty AST node (`ast::requirement::SubjectRef`
                // has no fields at all) referencing the case-family subject already established
                // elsewhere -- nothing to lower, so it is recognized and silently ignored rather
                // than reported as an unsupported member, mirroring `Doc`/`TextualRep`'s inert
                // handling above.
                RequirementDefBodyElement::SubjectRef(_) => {}
                RequirementDefBodyElement::RequirementActorDecl(actor) => {
                    self.lower_requirement_actor_decl(document, Some(owner), actor)?;
                }
                RequirementDefBodyElement::Stakeholder(stakeholder) => {
                    self.lower_stakeholder_member(document, Some(owner), stakeholder)?;
                }
                RequirementDefBodyElement::Purpose(purpose) => {
                    self.lower_purpose_member(document, owner, purpose)?;
                }
                RequirementDefBodyElement::VerifyRequirement(verify) => {
                    self.lower_verify_requirement_member(document, owner, unsupported, verify)?;
                }
                RequirementDefBodyElement::Frame(frame) => {
                    self.lower_frame_member(document, owner, unsupported, frame)?;
                }
                RequirementDefBodyElement::VariantUsage(node) => {
                    self.lower_variant_usage(document, owner, unsupported, node)?;
                }
                RequirementDefBodyElement::RequireConstraint(node) => {
                    self.lower_require_constraint_member(document, owner, unsupported, node)?;
                }
                RequirementDefBodyElement::Annotation(_)
                | RequirementDefBodyElement::MetadataKeywordUsage(_) => {
                    self.push_unsupported(document, unsupported, element.span.clone())
                }
            }
        }
        Ok(())
    }

    /// Lowers a `viewpoint def` (BNF ViewpointDefinition), mirroring `lower_requirement_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/nested-requirement members via the shared `RequirementDefBody` walker.
    /// Stakeholder/concern-binding semantics are out of scope; unrecognized body elements fall
    /// through to `unsupported_requirement_definition_member` (the same family `requirement def`
    /// uses, since `ViewpointDef` shares its exact body shape). `viewpoint` usage lowering is
    /// deferred -- see `DeclarationKind::ViewpointDefinition`'s doc comment.
    fn lower_viewpoint_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ViewpointDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewpointDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_requirement_shaped_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::RequirementDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `viewpoint` feature member (BNF ViewpointUsage),
    /// mirroring `lower_viewpoint_def`: ownership, membership, a `:` typing target, and owned
    /// members via the same shared `lower_requirement_shaped_body` walker. Only `name`/`type_name`
    /// are lowered as facts: `ast::ViewpointUsage::subsets`/`redefines` now carry the header-level
    /// `:>`/`:>>` clause, but lowering them is pending (planning/UPSTREAM_PARSER_GAPS.md, "Typed
    /// upstream, not yet lowered here").
    fn lower_viewpoint_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserViewpointUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewpointUsage,
            name,
            node.span.clone(),
            // No modifier, multiplicity, or short-name field on `ast::ViewpointUsage`; its
            // `subsets`/`redefines` clauses are relationships, not declaration facts, and are
            // not lowered yet (see `lower_viewpoint_usage`'s doc comment).
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_requirement_shaped_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::RequirementDefinitionMember,
        )
    }

    /// Lowers a package-level `concern` member (BNF ConcernUsage), dispatching on
    /// `is_definition` to either `concern def` (`DeclarationKind::ConcernDefinition`, Owning
    /// membership, mirroring `lower_viewpoint_def`'s owned-type shape) or a bare `concern` usage
    /// (`DeclarationKind::ConcernUsage`, Feature membership, mirroring `lower_requirement_usage`).
    /// Both forms share the same parsed fields -- `parser::requirement::concern_usage` calls the
    /// same `feature_usage_header` for both textual forms, so there is no separate `specializes:
    /// Node<TypingRelationship>` for the `def` form the way `RequirementDef`/`ViewpointDef` have;
    /// `type_name`/`subsets`/`redefines` are lowered identically (`FeatureTyping`/`Subsetting`/
    /// `Redefinition`) regardless of `is_definition`. The parser folds both textual forms into
    /// this single struct (see `ast::requirement::ConcernUsage`'s doc comment) rather than a
    /// distinct `ConcernDef` type. Genuinely new: previously blocked entirely
    /// (planning/UPSTREAM_PARSER_GAPS.md #9), resolved upstream in `0757de13`. Stakeholder/subject-binding
    /// semantics are out of scope, sharing `UnsupportedFamily::RequirementDefinitionMember` with
    /// `requirement def`/`viewpoint def`.
    fn lower_concern_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserConcernUsage>,
    ) -> Result<(), ConstructionError> {
        // `parser::requirement::concern_usage` always constructs `Membership::feature(...)`
        // regardless of `is_definition` (there is no distinct owning-membership constructor call
        // for the `def` textual form the way other `*Def`/`*Usage` pairs have), so
        // `member_visibility` is always checked against `FeatureMembership` here even though the
        // `def` form maps to our own `MembershipKind::Owning`.
        let (kind, membership_kind) = if node.value.is_definition {
            (DeclarationKind::ConcernDefinition, MembershipKind::Owning)
        } else {
            (DeclarationKind::ConcernUsage, MembershipKind::Feature)
        };
        let name = self.intern_declared_name(&node.value.name)?;
        // `ast::ConcernUsage` carries no modifier, multiplicity, direction, or short name.
        let declaration = self.push_typed_declaration(
            document,
            owner,
            kind,
            name,
            node.span.clone(),
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            membership_kind,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_requirement_shaped_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::RequirementDefinitionMember,
        )
    }

    /// Lowers an `analysis def` (BNF AnalysisCaseDefinition), mirroring `lower_requirement_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/nested members via the shared `UseCaseDefBody`. Analysis-case-specific semantics
    /// (subject binding, objective, result parameter binding) are explicitly out of scope;
    /// unrecognized body elements (including nested `analysis` usages -- see
    /// planning/UPSTREAM_PARSER_GAPS.md #5) fall through to `unsupported_analysis_case_definition_member`.
    /// `analysis` usage lowering itself is deferred entirely (same doc entry): `AnalysisCaseUsage`
    /// silently drops parsed `:>`/`:>>` clauses, unlike `AnalysisCaseDef`.
    fn lower_analysis_case_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AnalysisCaseDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AnalysisCaseDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_analysis_case_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the `UseCaseDefBody` owned by an `analysis def`: recognized owned members are
    /// attribute def/usage; everything else (subject/actor/objective/succession/nested
    /// action/analysis/calc/requirement/part usages, etc.) falls through to
    /// `unsupported_analysis_case_definition_member`.
    fn lower_analysis_case_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &UseCaseDefBody,
    ) -> Result<(), ConstructionError> {
        self.lower_case_family_def_body(
            document,
            owner,
            body,
            UnsupportedFamily::AnalysisCaseDefinitionMember,
        )
    }

    /// Lowers a `case def` (BNF CaseDefinition), mirroring `lower_analysis_case_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned attribute/nested
    /// members via the shared `UseCaseDefBody`. Case-specific semantics (subject binding,
    /// objective, first-succession/return structure) are explicitly out of scope; unrecognized
    /// body elements fall through to `unsupported_case_definition_member`. `case` usage lowering
    /// is deferred entirely (planning/UPSTREAM_PARSER_GAPS.md #5): `CaseUsage` silently drops parsed
    /// `:>`/`:>>` clauses, unlike `CaseDef`.
    fn lower_case_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<CaseDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CaseDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::CaseDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `analysis` feature member (BNF
    /// AnalysisCaseUsage), mirroring `lower_requirement_usage`: ownership, membership, a `:`
    /// typing target (bare `QualifiedReferenceId`, pushed as a `FeatureTyping` reference), and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #5): `AnalysisCaseUsage` previously had no typed field to lower
    /// these relationships from.
    fn lower_analysis_case_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserAnalysisCaseUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AnalysisCaseUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::AnalysisCaseDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `case` feature member (BNF CaseUsage), mirroring
    /// `lower_analysis_case_usage` (shares the same field shape). Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #5): `CaseUsage` previously had no typed field to lower
    /// `subsets`/`redefines` from.
    fn lower_case_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserCaseUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CaseUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::CaseDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `use case` feature member (BNF UseCaseUsage),
    /// mirroring `lower_case_usage`. Unlike `CaseUsage`/`AnalysisCaseUsage`, `ast::UseCaseUsage`
    /// has no `subsets`/`redefines`/multiplicity field at all (see
    /// `DeclarationKind::UseCaseUsage`), so only `name`/`type_name` are lowered as facts; owned
    /// members are walked through the shared `lower_case_family_def_body`.
    fn lower_use_case_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserUseCaseUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::UseCaseUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                // `ast::UseCaseUsage` has no multiplicity/`nonunique` fields; see
                // planning/UPSTREAM_PARSER_GAPS.md Gap 53.
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::UseCaseDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `verification` feature member (BNF
    /// VerificationCaseUsage), mirroring `lower_use_case_usage` (shares the same field
    /// shape/limitation: no `subsets`/`redefines`/multiplicity field on
    /// `ast::VerificationCaseUsage`).
    fn lower_verification_case_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserVerificationCaseUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::VerificationCaseUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                // `ast::VerificationCaseUsage` has no multiplicity/`nonunique` fields; see
                // planning/UPSTREAM_PARSER_GAPS.md Gap 53.
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::VerificationCaseDefinitionMember,
        )
    }

    /// Lowers a `verification def` (BNF VerificationCaseDefinition), mirroring `lower_case_def`.
    /// Verification-specific semantics are explicitly out of scope; unrecognized body elements
    /// fall through to `unsupported_verification_case_definition_member`. `verification` usage
    /// lowering (`DeclarationKind::VerificationCaseUsage`, `lower_verification_case_usage`): see
    /// its own doc comment for the remaining `subsets`/`redefines`/multiplicity gap.
    fn lower_verification_case_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<VerificationCaseDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::VerificationCaseDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::VerificationCaseDefinitionMember,
        )
    }

    /// Lowers a `use case def` (BNF UseCaseDefinition), mirroring `lower_case_def`. Use-case-
    /// specific semantics (actor/include structure) are explicitly out of scope; unrecognized
    /// body elements fall through to `unsupported_use_case_definition_member`. `use case` usage
    /// lowering is deferred entirely (planning/UPSTREAM_PARSER_GAPS.md #5): `UseCaseUsage` silently drops
    /// parsed `:>`/`:>>` clauses, unlike `UseCaseDef`.
    fn lower_use_case_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<UseCaseDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::UseCaseDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::UseCaseDefinitionMember,
        )
    }

    /// Shared body walker for the case-family def kinds (`analysis def`/`case def`/
    /// `verification def`/`use case def`), all of which share the same `UseCaseDefBody`/
    /// `UseCaseDefBodyElement` shape in the typed AST. Recognized owned members are attribute
    /// def/usage; everything else (subject/actor/objective/succession/nested
    /// action/analysis/calc/requirement/part usages, etc.) falls through to the caller-supplied
    /// `unsupported` family so each def kind's diagnostics stay distinct.
    fn lower_case_family_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &UseCaseDefBody,
        unsupported: UnsupportedFamily,
    ) -> Result<(), ConstructionError> {
        let UseCaseDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                UseCaseDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                UseCaseDefBodyElement::AttributeDef(attribute) => {
                    self.lower_attribute_def(document, Some(owner), attribute)?;
                }
                UseCaseDefBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                UseCaseDefBodyElement::AnalysisCaseUsage(analysis_case_usage) => {
                    self.lower_analysis_case_usage(document, Some(owner), analysis_case_usage)?;
                }
                UseCaseDefBodyElement::ActionUsage(action_usage) => {
                    self.lower_action_usage(document, Some(owner), action_usage)?;
                }
                UseCaseDefBodyElement::CalcUsage(calc_usage) => {
                    self.lower_calc_usage(document, Some(owner), calc_usage)?;
                }
                UseCaseDefBodyElement::RequirementUsage(requirement_usage) => {
                    self.lower_requirement_usage(document, Some(owner), requirement_usage)?;
                }
                UseCaseDefBodyElement::PartUsage(part_usage) => {
                    self.lower_part_usage(document, Some(owner), part_usage)?;
                }
                UseCaseDefBodyElement::SubjectDecl(subject) => {
                    self.lower_subject_decl(document, Some(owner), subject)?;
                }
                UseCaseDefBodyElement::Ref(node) => {
                    self.lower_ref_decl(document, Some(owner), node)?;
                }
                UseCaseDefBodyElement::InOutDecl(param) => {
                    self.lower_parameter_declaration(document, Some(owner), unsupported, param)?;
                }
                UseCaseDefBodyElement::Doc(node) => {
                    self.record_doc_comment(owner, node)?;
                }
                UseCaseDefBodyElement::MetadataAnnotation(node) => {
                    self.lower_metadata_annotation(document, owner, node)?;
                }
                UseCaseDefBodyElement::AssertConstraint(node) => {
                    self.lower_assert_constraint_member(document, owner, unsupported, node)?
                }
                UseCaseDefBodyElement::IncludeUseCase(node) => {
                    self.lower_include_use_case(document, owner, node)?;
                }
                UseCaseDefBodyElement::ThenIncludeUseCase(node) => {
                    self.lower_include_use_case(document, owner, &node.value.include)?;
                }
                // `subject;` shorthand: see the identical-shape `RequirementDefBodyElement::
                // SubjectRef` handling in `lower_requirement_shaped_body` -- an entirely empty AST
                // node with nothing to lower, recognized and silently ignored.
                UseCaseDefBodyElement::SubjectRef(_) => {}
                UseCaseDefBodyElement::ActorUsage(node) => {
                    self.lower_actor_usage(document, owner, node)?;
                }
                // `objective { ... }`/`objective <name> [: Type] { ... }` wraps a fully typed
                // `RequirementUsage` (`Objective::requirement`) -- lower it through the exact same
                // `lower_requirement_usage` pipeline every other requirement-usage site uses.
                // `Objective::visibility` (an outer `private`/`protected`/`public` prefix consumed
                // separately by the parser, before the wrapped `RequirementUsage`'s own membership)
                // is not threaded through; the nested node's own membership visibility is used as
                // authored, mirroring other case-family wrapper nodes' out-of-scope facts.
                UseCaseDefBodyElement::Objective(node) => {
                    self.lower_requirement_usage(document, Some(owner), &node.value.requirement)?;
                }
                UseCaseDefBodyElement::CaseReturnDecl(node) => {
                    self.lower_case_return_decl(document, owner, unsupported, node)?;
                }
                UseCaseDefBodyElement::Assign(node) => {
                    self.lower_assign_stmt(
                        document,
                        owner,
                        unsupported,
                        node.span.clone(),
                        &node.value,
                    )?;
                }
                UseCaseDefBodyElement::ForLoop(node) => {
                    self.lower_for_loop(
                        document,
                        owner,
                        unsupported,
                        node.span.clone(),
                        &node.value,
                    )?;
                }
                UseCaseDefBodyElement::ThenAction(node) => {
                    self.lower_then_action(document, owner, unsupported, node)?;
                }
                UseCaseDefBodyElement::FlowUsage(node) => {
                    self.lower_flow_usage(document, owner, unsupported, node)?;
                }
                // Bare result expression in an analysis/case body (validation `10a`: `vehicle.
                // mass`) -- mirrors `CalcDefBodyElement::Expression`'s identical shape: the
                // expression is the enclosing case-family declaration's own evaluated result, not
                // a new nested declaration, so it is classified/lowered directly at `owner` through
                // the same `classify_calc_expression`/`lower_calc_expression` pipeline a calc def's
                // bare body expression uses.
                UseCaseDefBodyElement::Expression(expression) => {
                    self.push_evaluation_fact(owner, classify_calc_expression(&expression.value));
                    self.lower_calc_expression(document, owner, unsupported, expression)?;
                }
                UseCaseDefBodyElement::Annotation(_)
                | UseCaseDefBodyElement::MetadataKeywordUsage(_)
                | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
                | UseCaseDefBodyElement::FirstSuccession(_)
                | UseCaseDefBodyElement::ThenUseCaseUsage(_)
                | UseCaseDefBodyElement::ThenDone(_)
                | UseCaseDefBodyElement::RefRedefinition(_)
                | UseCaseDefBodyElement::ReturnRef(_) => {
                    self.push_unsupported(document, unsupported, element.span.clone())
                }
            }
        }
        Ok(())
    }

    /// Lowers a `port def` (BNF PortDefinition), mirroring `lower_part_def`:
    /// membership, an optional `:>` specialization relationship (participates in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point, see `DeclarationDomain::Type`
    /// in resolver.rs), and owned attribute/enum/nested-port members. Port-specific semantics
    /// (interface/flow binding, port conformance, connector-end validation) are explicitly out of
    /// scope; unrecognized body elements fall through to `unsupported_port_definition_member`.
    fn lower_port_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PortDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PortDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let PortDefBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                match &element.value {
                    PortDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PortDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    PortDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PortDefBodyElement::EnumerationUsage(enum_usage) => {
                        self.lower_enum_usage(document, Some(declaration), enum_usage)?;
                    }
                    PortDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PortDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    PortDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PortDefBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    PortDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PortDefinitionMember,
                            param,
                        )?;
                    }
                    PortDefBodyElement::Unsupported(node) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ParserUnsupported,
                        node.span.clone(),
                    ),
                    PortDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::PortDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `port` feature member (BNF PortUsage), mirroring
    /// `lower_part_usage`: ownership, membership, an optional `:`/`:>` typing/subclassification
    /// relationship (whose target may be conjugated, e.g. `port source : ~InputPort;` -- the
    /// polarity is carried as an explicit `RelationshipFlags::conjugated` fact via
    /// `lower_typing_relationship`, never folded into the reference target), `subsets`/
    /// `redefines`/`references`/`crosses`/`intersects` subsetting relationships, and owned
    /// attribute/nested-port members.
    fn lower_port_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserPortUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PortUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    derived: node.value.is_derived,
                    constant: node.value.is_constant,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some((relationship, _)) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.crosses {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.intersects {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let PortBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                match &element.value {
                    PortBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PortBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PortBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PortBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PortBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    PortBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PortDefinitionMember,
                            param,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Lowers a `connection def` (BNF ConnectionDefinition), mirroring `lower_port_def`:
    /// ownership, membership, an optional `:>` specialization relationship (participates in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point, see
    /// `DeclarationDomain::Type` in resolver.rs), and owned attribute/item/port/nested-connection
    /// members plus connector-end structure via `lower_connection_body`. Connector-end
    /// referential/multiplicity validation is explicitly out of scope; unrecognized body elements
    /// fall through to `unsupported_connection_definition_member`.
    fn lower_connection_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ConnectionDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConnectionDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_connection_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `connection` feature member (BNF ConnectionUsage),
    /// mirroring `lower_metadata_usage`: ownership, membership, an optional `:` typing reference
    /// (a bare `QualifiedReferenceId`, not a structured `TypingRelationship`),
    /// `subsets`/`redefines` subsetting relationships, an optional inline `connect from to to`
    /// clause (connector-end references), and owned attribute/item/port/nested-connection
    /// members via the same shared `lower_connection_body` as `connection def`.
    fn lower_connection_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserConnectionUsage>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConnectionUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_reference) = node.value.type_reference {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_reference,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(end) = &node.value.connect_from {
            self.lower_connector_end(document, declaration, end)?;
        }
        if let Some(end) = &node.value.connect_to {
            self.lower_connector_end(document, declaration, end)?;
        }
        for end in &node.value.connect_extra_ends {
            self.lower_connector_end(document, declaration, end)?;
        }
        self.lower_connection_body(document, declaration, &node.value.body)
    }

    /// Shared body walker for `connection def`/`connection` usage bodies (both use
    /// `ConnectionDefBody`/`ConnectionDefBodyElement` -- there is no separate
    /// `ConnectionUsageBody`), mirroring `lower_state_def_body`'s single-walker pattern. `end`
    /// declarations and `connect` statements carry the connector-end structure; everything else
    /// beyond attribute/item/port/nested-part-usage members falls through to
    /// `unsupported_connection_definition_member`.
    fn lower_connection_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &ConnectionDefBody,
    ) -> Result<(), ConstructionError> {
        if let ConnectionDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    ConnectionDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ConnectionDefBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    ConnectionDefBodyElement::EndDecl(end_decl) => {
                        self.lower_end_decl(document, declaration, end_decl)?;
                    }
                    ConnectionDefBodyElement::ConnectStmt(connect_stmt) => {
                        self.lower_connect_stmt(document, declaration, connect_stmt)?;
                    }
                    ConnectionDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    ConnectionDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    ConnectionDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    ConnectionDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    ConnectionDefBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    ConnectionDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    ConnectionDefBodyElement::PartUsage(part_usage) => {
                        self.lower_part_usage(document, Some(declaration), part_usage)?;
                    }
                    ConnectionDefBodyElement::OccurrenceUsage(occurrence_usage) => {
                        self.lower_occurrence_usage(document, Some(declaration), occurrence_usage)?;
                    }
                    ConnectionDefBodyElement::AssertConstraint(node) => self
                        .lower_assert_constraint_member(
                            document,
                            declaration,
                            UnsupportedFamily::ConnectionDefinitionMember,
                            node,
                        )?,
                    ConnectionDefBodyElement::RefDecl(node) => {
                        self.lower_ref_decl(document, Some(declaration), node)?;
                    }
                    ConnectionDefBodyElement::SuccessionUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ConnectionDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers an `end` declaration inside a connection/interface def body (BNF `EndDecl`) as its
    /// own nested declaration: a normal declared label (or an anonymous `#original`/`#derive`
    /// derivation role), an optional `:` typing relationship, and an optional `::>`/`references`
    /// reference-subsetting relationship as an authored `ConnectorEnd` reference (resolved
    /// through the same shared lexical lookup as `AliasBinding`, see `DeclarationDomain::Any` in
    /// resolver.rs). `redefines`/`crosses`/`nested_usage` -- connector-end referential
    /// constraints, not the plain reference shape this slice covers -- are explicitly out of
    /// scope and left unlowered.
    fn lower_end_decl(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<EndDecl>,
    ) -> Result<(), ConstructionError> {
        let name = match &node.value.identity {
            EndIdentity::Declaration(label) => self.intern_declared_name(&label.value)?,
            EndIdentity::Derivation(_) => None,
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ConnectionUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            for target in relationship.value.target.iter().copied() {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ConnectorEnd,
                    document,
                    local: target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
        }
        Ok(())
    }

    /// Lowers an inline `connect from to to (, extra)*` statement (BNF `ConnectStmt`) as
    /// `ConnectorEnd` references from the owning connection def/usage declaration to each end's
    /// target. Real annotation content in a braced body (`ConnectBody::Brace`) is out of scope --
    /// only the endpoint references themselves are lowered.
    fn lower_connect_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ConnectStmt>,
    ) -> Result<(), ConstructionError> {
        self.lower_connector_end(document, owner, &node.value.from)?;
        self.lower_connector_end(document, owner, &node.value.to)?;
        for end in &node.value.extra_ends {
            self.lower_connector_end(document, owner, end)?;
        }
        // A `connect` statement mints no declaration of its own, so a `doc` in its body has no
        // element to bind to; see `lower_relationship_body_elements`.
        self.lower_relationship_body_elements(document, None, &node.value.body_elements)?;
        Ok(())
    }

    /// Lowers one connector end (`ConnectionEnd`, used by both `ConnectStmt` and
    /// `ConnectionUsageMember`'s inline `connect` clause): its path expression is a structured
    /// `Expression` (not a flattened string), so a simple/qualified name (`Expression::FeatureRef`)
    /// resolves as an authored `ConnectorEnd` reference through the same shared lexical lookup as
    /// `AliasBinding`. A dotted feature-chain path (`Expression::MemberAccess`, e.g. `t.bead`)
    /// resolves as a `ReferenceKind::MemberAccessOperand` reference instead (see its doc comment
    /// for the algorithm), through `flatten_member_access_chain`/`push_member_access_reference`.
    /// Any other expression shape is left as an explicit `unsupported_connection_definition_member`
    /// diagnostic rather than a fabricated or partial resolution.
    fn lower_connector_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ConnectionEnd>,
    ) -> Result<(), ConstructionError> {
        match &node.value.expression.value {
            Expression::FeatureRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: owner,
                    kind: ReferenceKind::ConnectorEnd,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            Expression::MemberAccess { .. } | Expression::FeatureChainRef(_) => {
                if let Some(chain) = flatten_member_access_chain(&node.value.expression) {
                    self.push_member_access_reference(
                        owner,
                        document,
                        &chain,
                        node.value.expression.span.clone(),
                    )?;
                } else {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::ConnectionDefinitionMember,
                        node.span.clone(),
                    );
                }
            }
            _ => self.push_unsupported(
                document,
                UnsupportedFamily::ConnectionDefinitionMember,
                node.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers an `interface def` (BNF InterfaceDefinition), mirroring `lower_connection_def`:
    /// ownership, membership, an optional `:>` specialization relationship (participates in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point, see
    /// `DeclarationDomain::Type` in resolver.rs), and owned attribute/item/port/flow members plus
    /// connector-end structure via `lower_interface_body`, reusing the same `end`/`connect`
    /// `ReferenceKind::ConnectorEnd` machinery `lower_connection_def` uses (interface ends are
    /// semantically the same kind of fact). `interface` usage lowering is deferred -- see
    /// `DeclarationKind::InterfaceDefinition`'s doc comment and planning/UPSTREAM_PARSER_GAPS.md #6.
    fn lower_interface_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<InterfaceDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::InterfaceDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_interface_body(document, declaration, &node.value.body)
    }

    /// Body walker for `interface def` bodies (`InterfaceDefBody`/`InterfaceDefBodyElement`),
    /// mirroring `lower_connection_body`. `end` declarations and `connect` statements carry the
    /// connector-end structure through the same `lower_end_decl`/`lower_connect_stmt` helpers
    /// `connection def` uses; everything else beyond attribute/item/port members falls through to
    /// `unsupported_interface_definition_member`.
    fn lower_interface_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &InterfaceDefBody,
    ) -> Result<(), ConstructionError> {
        if let InterfaceDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    InterfaceDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    InterfaceDefBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    InterfaceDefBodyElement::EndDecl(end_decl) => {
                        self.lower_end_decl(document, declaration, end_decl)?;
                    }
                    InterfaceDefBodyElement::ConnectStmt(connect_stmt) => {
                        self.lower_connect_stmt(document, declaration, connect_stmt)?;
                    }
                    InterfaceDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    InterfaceDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    InterfaceDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    InterfaceDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    InterfaceDefBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    InterfaceDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    InterfaceDefBodyElement::RefDecl(node) => {
                        self.lower_ref_decl(document, Some(declaration), node)?;
                    }
                    InterfaceDefBodyElement::FlowUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::InterfaceDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `interface` feature member (BNF InterfaceUsage),
    /// mirroring `lower_connection_usage`: ownership, membership, an optional `:` typing target,
    /// `subsets`/`redefines` subsetting relationships, and connector-end structure (`connect`
    /// endpoints via `lower_interface_connector_expression`, reusing the same
    /// `ReferenceKind::ConnectorEnd` machinery `interface def`/`connection` usage use). Resolved
    /// upstream in `0757de13` (planning/UPSTREAM_PARSER_GAPS.md #6).
    fn lower_interface_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserInterfaceUsage>,
    ) -> Result<(), ConstructionError> {
        let (name, interface_type, subsets, redefines, ends, body_elements) = match &node.value {
            ParserInterfaceUsage::TypedConnect {
                name,
                interface_type,
                subsets,
                redefines,
                from,
                to,
                body_elements,
                ..
            } => (
                name.as_deref(),
                interface_type.as_ref(),
                subsets.as_ref(),
                redefines.as_ref(),
                vec![from, to],
                body_elements,
            ),
            ParserInterfaceUsage::Connection {
                subsets,
                redefines,
                from,
                to,
                body_elements,
                ..
            } => (
                None,
                None,
                subsets.as_ref(),
                redefines.as_ref(),
                vec![from, to],
                body_elements,
            ),
            ParserInterfaceUsage::Declaration {
                name,
                interface_type,
                subsets,
                redefines,
                body_elements,
                ..
            } => (
                name.as_deref(),
                interface_type.as_ref(),
                subsets.as_ref(),
                redefines.as_ref(),
                Vec::new(),
                body_elements,
            ),
        };
        let name = name
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::InterfaceUsage,
            name,
            node.span.clone(),
            // `ast::InterfaceUsage` is an enum of connect/declaration shapes carrying only name,
            // type, subsets/redefines, and ends -- no modifier, multiplicity, direction, or short
            // name on either variant.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_reference) = interface_type {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(*type_reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: *type_reference,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        for end in ends {
            self.lower_interface_connector_expression(document, declaration, end)?;
        }
        for element in body_elements {
            match &element.value {
                InterfaceUsageBodyElement::Doc(node) => {
                    self.record_doc_comment(declaration, node)?;
                }
                InterfaceUsageBodyElement::EndDecl(end_decl) => {
                    self.lower_end_decl(document, declaration, end_decl)?;
                }
                InterfaceUsageBodyElement::RefRedef { .. } => self.push_unsupported(
                    document,
                    UnsupportedFamily::InterfaceDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers one `from`/`to` interface-connect endpoint expression as a `ConnectorEnd`
    /// reference, mirroring `lower_connector_end` but operating directly on the bare
    /// `Node<Expression>` `InterfaceUsage::TypedConnect`/`Connection` carry (rather than the
    /// `Node<ConnectionEnd>` wrapper `connection` usage's `connect_from`/`connect_to` use).
    fn lower_interface_connector_expression(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::FeatureRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: owner,
                    kind: ReferenceKind::ConnectorEnd,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            Expression::MemberAccess { .. } | Expression::FeatureChainRef(_) => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(owner, document, &chain, node.span.clone())?;
                } else {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::InterfaceDefinitionMember,
                        node.span.clone(),
                    );
                }
            }
            _ => self.push_unsupported(
                document,
                UnsupportedFamily::InterfaceDefinitionMember,
                node.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers a `view def` (BNF ViewDefinition), mirroring `lower_interface_def`: ownership,
    /// membership, an optional `:>` specialization relationship (participates in the shared
    /// Subclassification/FeatureTyping `DeclarationDomain::Type` fixed point). View-specific
    /// body members (`render`, `filter`) are out of scope -- see `DeclarationKind::ViewDefinition`'s
    /// doc comment and planning/UPSTREAM_PARSER_GAPS.md #8.
    fn lower_view_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ViewDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_view_def_body(document, declaration, &node.value.body)
    }

    /// Body walker for `view def` bodies (`ViewDefBody`/`ViewDefBodyElement`). `filter`/`render`
    /// members are out of scope for this slice and fall through to
    /// `unsupported_view_definition_member`.
    fn lower_view_def_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &ViewDefBody,
    ) -> Result<(), ConstructionError> {
        if let ViewDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    ViewDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ViewDefBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    ViewDefBodyElement::MetadataAnnotation(node) => {
                        self.lower_metadata_annotation(document, declaration, node)?;
                    }
                    ViewDefBodyElement::Filter(filter) => {
                        self.lower_filter_expression(
                            document,
                            declaration,
                            &filter.value.condition,
                        )?;
                    }
                    ViewDefBodyElement::Unsupported(node) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ParserUnsupported,
                        node.span.clone(),
                    ),
                    ViewDefBodyElement::ViewRendering(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ViewDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `view` feature member (BNF ViewUsage), mirroring
    /// `lower_analysis_case_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #8): `ViewUsage` previously had no `subsets` field. Multiplicity
    /// and view-specific body members (`render`/`filter`) are out of scope for this slice.
    fn lower_view_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserViewUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_view_usage_body(document, declaration, &node.value.body)
    }

    /// Body walker for `view` usage bodies (`ViewBody`/`ViewBodyElement`), mirroring
    /// `lower_view_def_body`. `filter`/`render` members are out of scope for this slice and fall
    /// through to `unsupported_view_definition_member`.
    fn lower_view_usage_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &ViewBody,
    ) -> Result<(), ConstructionError> {
        if let ViewBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    ViewBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ViewBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    ViewBodyElement::Satisfy(node) => {
                        self.lower_view_satisfy(document, declaration, node)?;
                    }
                    ViewBodyElement::Filter(filter) => {
                        self.lower_filter_expression(
                            document,
                            declaration,
                            &filter.value.condition,
                        )?;
                    }
                    ViewBodyElement::ViewRendering(_) | ViewBodyElement::Expose(_) => self
                        .push_unsupported(
                            document,
                            UnsupportedFamily::ViewDefinitionMember,
                            element.span.clone(),
                        ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `rendering` feature member (BNF RenderingUsage),
    /// mirroring `lower_view_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. `ast::RenderingUsage` now carries full
    /// field parity with `ViewUsage` (planning/UPSTREAM_PARSER_GAPS.md #26, resolved upstream in
    /// `cb026cd`) -- `is_abstract`/`multiplicity`/`ordered`/`nonunique`/`value` are not modeled as
    /// distinct facts here (see `DeclarationKind::RenderingUsage`).
    fn lower_rendering_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserRenderingUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RenderingUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ordered: node.value.ordered,
                    nonunique: node.value.nonunique,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_rendering_usage_body(document, declaration, &node.value.body)
    }

    /// Body walker for `rendering` usage bodies (`RenderingUsageBody`/
    /// `RenderingUsageBodyElement`). Nested `view`/`rendering` usage members recurse through
    /// `lower_view_usage`/`lower_rendering_usage` themselves (the same shape a package-level
    /// `view`/`rendering` member uses); anything else falls through to
    /// `UnsupportedFamily::PackageMember`, matching the top-level dispatch this body's owner was
    /// itself lowered from.
    fn lower_rendering_usage_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &RenderingUsageBody,
    ) -> Result<(), ConstructionError> {
        if let RenderingUsageBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    RenderingUsageBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    RenderingUsageBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    RenderingUsageBodyElement::ViewUsage(node) => {
                        self.lower_view_usage(document, Some(declaration), node)?;
                    }
                    RenderingUsageBodyElement::Rendering(node) => {
                        self.lower_rendering_usage(document, Some(declaration), node)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Lowers a `rendering def` (BNF RenderingDefinition), mirroring `lower_view_def`: ownership,
    /// membership, an optional `:>` specialization relationship (participates in the shared
    /// `DeclarationDomain::Type` fixed point). Render-specific body members (`filter`/`render`)
    /// are out of scope -- see `DeclarationKind::RenderingDefinition`'s doc comment.
    /// Lowers a `constraint def` (BNF ConstraintDefinition), mirroring `lower_view_def`:
    /// ownership, membership, an optional `:>` specialization relationship participating in the
    /// shared `DeclarationDomain::Type` fixed point. Constraint-body expression content is out of
    /// scope for this slice and falls through to `UnsupportedFamily::ConstraintDefinitionMember`.
    fn lower_constraint_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ConstraintDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConstraintDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_constraint_def_body(document, declaration, &node.value.body)
    }

    /// Shared body walker for `constraint def`/`constraint` usage bodies (both use
    /// `ConstraintDefBody`/`ConstraintDefBodyElement` in the typed AST -- there is no separate
    /// `ConstraintUsageBody`), mirroring `lower_view_def_body`. Expression/nested-constraint
    /// content falls through to `unsupported_constraint_definition_member`.
    fn lower_constraint_def_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &ConstraintDefBody,
    ) -> Result<(), ConstructionError> {
        if let ConstraintDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    ConstraintDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ConstraintDefBodyElement::Constraint(constraint) => {
                        self.lower_constraint_usage(document, Some(declaration), constraint)?;
                    }
                    ConstraintDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ConstraintDefinitionMember,
                            param,
                        )?;
                    }
                    ConstraintDefBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    ConstraintDefBodyElement::Expression(expression) => {
                        self.push_evaluation_fact(
                            declaration,
                            classify_constraint_expression(&expression.value),
                        );
                        self.lower_constraint_expression(
                            document,
                            declaration,
                            UnsupportedFamily::ConstraintDefinitionMember,
                            expression,
                        )?
                    }
                    ConstraintDefBodyElement::MetadataAnnotation(node) => {
                        self.lower_metadata_annotation(document, declaration, node)?;
                    }
                    ConstraintDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    ConstraintDefBodyElement::FeatureDecl(node) => self
                        .lower_default_reference_usage(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ConstraintDefinitionMember,
                            node,
                        )?,
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `constraint` feature member (BNF
    /// ConstraintUsage), mirroring `lower_analysis_case_usage`: ownership, membership, a `:`
    /// typing target, and `subsets`/`redefines` subsetting relationships. Resolved upstream in
    /// `0757de13` (planning/UPSTREAM_PARSER_GAPS.md #4): `ConstraintUsage` previously had no
    /// `subsets`/`redefines` fields at all.
    fn lower_constraint_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserConstraintUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConstraintUsage,
            name,
            node.span.clone(),
            // `ast::ConstraintUsage` carries no modifier, multiplicity, direction, or short name;
            // see planning/UPSTREAM_PARSER_GAPS.md for its missing multiplicity field.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_constraint_def_body(document, declaration, &node.value.body)
    }

    /// Lowers `assert constraint { <boolExpr> }` / `assert constraint <name> : <ConstraintDef>
    /// { ... }` (BNF `AssertConstraintMember`, `AssertConstraintUsage`): semantically an inline,
    /// anonymous (or named) constraint usage introduced via `assert` rather than the bare
    /// `constraint` keyword. Mirrors `lower_first_stmt`'s "anonymous nested declaration" pattern
    /// (`Succession`) and reuses `lower_constraint_usage`'s typing + `lower_constraint_def_body`
    /// wiring verbatim -- `AssertConstraintMember.body` is the exact same `ConstraintDefBody`
    /// shape as `ConstraintDef`/`ConstraintUsage`, so the existing
    /// `lower_constraint_expression`/`classify_constraint_expression` evaluation machinery (Slice
    /// 1, `4ca42166`) applies unchanged.
    ///
    /// Deferred (falls through to `family`'s unsupported diagnostic):
    /// - `assert not constraint ...` (`is_negated`): negation is a distinct semantic (the
    ///   asserted truth value is the logical complement of the inner expression's evaluation) that
    ///   the current evaluation-fact shape has no representation for yet.
    /// - `assert <path> { ... }` shorthand (`target` set, no `constraint` keyword): references an
    ///   existing constraint by path rather than declaring one inline; out of scope for this
    ///   slice, which targets the `constraint`-keyword forms only.
    fn lower_assert_constraint_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<AssertConstraintMember>,
    ) -> Result<(), ConstructionError> {
        if node.value.is_negated || node.value.target.is_some() {
            self.push_unsupported(document, family, node.span.clone());
            return Ok(());
        }
        let name = node
            .value
            .declaration_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::AssertConstraintUsage,
            name,
            node.span.clone(),
            // `ast::AssertConstraintMember`'s remaining field is `is_negated`, an
            // assertion-polarity fact rather than a declaration modifier, and the negated form is
            // routed to the unsupported diagnostic above.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_constraint_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a requirement/objective/case-family-def-body `require`/`assume` constraint member
    /// (BNF `RequireConstraint`, `ast::RequireConstraint`): the `require constraint { ... }` /
    /// `assume constraint <name> { ... }` shape (`has_constraint_keyword == true`) declares an
    /// anonymous or named nested `ConstraintUsage` feature, structurally identical to
    /// `AssertConstraintMember`'s constraint-keyword form (`lower_assert_constraint_member`) minus
    /// the `is_negated`/shorthand-`target`/`type_name` operands `AssertConstraintMember` has and
    /// `RequireConstraint` does not. Its body *is* `ConstraintDefBody` upstream (the duplicate
    /// `RequireConstraintBody` name collapsed into the type it was always equal to), so it is
    /// dispatched through the existing `lower_constraint_def_body` walker unchanged.
    ///
    /// Deferred (falls through to `family`'s unsupported diagnostic): the `require <name>;` /
    /// `require <name> { ... }` shorthand (`has_constraint_keyword == false`), which references an
    /// *existing* constraint by name rather than declaring one. Upstream now carries that role on
    /// its own arena-backed `RequireConstraint::target`, so it can participate in the shared
    /// lexical-lookup reference machinery; wiring it is pending
    /// (planning/UPSTREAM_PARSER_GAPS.md, "Typed upstream, not yet lowered here"). Likewise
    /// `require constraint <name> : <Type>;` / `require
    /// constraint <name> :>> <target>;` (a `:`/`:>>` clause after the name) fails to parse as
    /// `RequireConstraint` at all upstream (no field for either), so those never reach this
    /// function in the first place.
    fn lower_require_constraint_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<RequireConstraint>,
    ) -> Result<(), ConstructionError> {
        if !node.value.has_constraint_keyword {
            self.push_unsupported(document, family, node.span.clone());
            return Ok(());
        }
        let name = node
            .value
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            if node.value.is_assume {
                DeclarationKind::AssumeConstraintUsage
            } else {
                DeclarationKind::RequireConstraintUsage
            },
            name,
            node.span.clone(),
            // `has_constraint_keyword` selects the authored form (checked above) rather than
            // modifying the declaration; `is_assume` rides the declaration kind, because it is
            // what makes `RequirementConstraintMembership.kind` derivable.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_constraint_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a `calc def` (BNF CalculationDefinition), mirroring `lower_action_def`: ownership,
    /// membership, an optional `:>` specialization relationship participating in the shared
    /// `DeclarationDomain::Type` fixed point. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #3): `CalcDef` previously dropped its parsed `:>` clause.
    /// Calculation-expression body content is out of scope and falls through to
    /// `UnsupportedFamily::CalcDefinitionMember`.
    fn lower_calc_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<CalcDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CalcDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Shared body walker for `calc def`/`calc` usage bodies (both use `CalcDefBody`/
    /// `CalcDefBodyElement` in the typed AST -- there is no separate `CalcUsageBody`), mirroring
    /// `lower_constraint_def_body`. Calculation-expression content, in/out/return parameters, and
    /// nested calc structure fall through to `unsupported_calc_definition_member`.
    fn lower_calc_def_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &CalcDefBody,
    ) -> Result<(), ConstructionError> {
        if let CalcDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    CalcDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    CalcDefBodyElement::CalcUsage(calc_usage) => {
                        self.lower_calc_usage(document, Some(declaration), calc_usage)?;
                    }
                    CalcDefBodyElement::CalcDef(calc_def) => {
                        self.lower_calc_def(document, Some(declaration), calc_def)?;
                    }
                    CalcDefBodyElement::PartUsage(part_usage) => {
                        self.lower_part_usage(document, Some(declaration), part_usage)?;
                    }
                    CalcDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(
                            document,
                            Some(declaration),
                            UnsupportedFamily::CalcDefinitionMember,
                            param,
                        )?;
                    }
                    CalcDefBodyElement::Doc(node) => {
                        self.record_doc_comment(declaration, node)?;
                    }
                    CalcDefBodyElement::Expression(expression) => {
                        self.push_evaluation_fact(
                            declaration,
                            classify_calc_expression(&expression.value),
                        );
                        self.lower_calc_expression(
                            document,
                            declaration,
                            UnsupportedFamily::CalcDefinitionMember,
                            expression,
                        )?
                    }
                    CalcDefBodyElement::ReturnDecl(return_decl) => {
                        self.lower_return_decl(document, Some(declaration), return_decl)?;
                    }
                    CalcDefBodyElement::MetadataAnnotation(node) => {
                        self.lower_metadata_annotation(document, declaration, node)?;
                    }
                    CalcDefBodyElement::AttributeUsage(nested) => {
                        self.lower_attribute_usage(document, Some(declaration), nested)?;
                    }
                    CalcDefBodyElement::KermlClassifier(nested) => {
                        self.lower_kerml_classifier_decl(document, Some(declaration), nested)?;
                    }
                    CalcDefBodyElement::KermlFeature(nested) => {
                        self.lower_kerml_feature_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::CalcDefinitionMember,
                            nested,
                        )?;
                    }
                    CalcDefBodyElement::DefaultReferenceUsage(node) => {
                        self.lower_default_reference_usage(
                            document,
                            Some(declaration),
                            UnsupportedFamily::CalcDefinitionMember,
                            node,
                        )?;
                    }
                    CalcDefBodyElement::Invariant(node) => {
                        self.lower_kerml_invariant_member(document, Some(declaration), node)?;
                    }
                    CalcDefBodyElement::Connector(node) => {
                        self.lower_kerml_connector_member(document, declaration, node)?;
                    }
                    CalcDefBodyElement::Binding(node) => {
                        self.lower_kerml_binding_member(document, declaration, node)?;
                    }
                    CalcDefBodyElement::EndMember(node) => {
                        self.lower_kerml_end_member(document, declaration, node)?;
                    }
                    CalcDefBodyElement::TypedParameter(node) => {
                        self.lower_typed_parameter_member(document, declaration, node)?;
                    }
                    CalcDefBodyElement::Succession(node) => {
                        self.lower_kerml_succession_member(document, declaration, node)?;
                    }
                    CalcDefBodyElement::Import(node) => {
                        self.lower_import(document, Some(declaration), node)?;
                    }
                    CalcDefBodyElement::Comment(node) => {
                        self.record_comment_annotation(declaration, node)?;
                    }
                    CalcDefBodyElement::AssertConstraint(node) => {
                        self.lower_assert_constraint_member(
                            document,
                            declaration,
                            UnsupportedFamily::CalcDefinitionMember,
                            node,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `calc` feature member (BNF CalculationUsage),
    /// mirroring `lower_analysis_case_usage`: ownership, membership, a `:` typing target, and
    /// `redefines` targets. Unlike other usage kinds, `CalcUsage::redefines` is a bare
    /// `Vec<QualifiedReferenceId>` rather than a `Node<SubsettingRelationship>` (and there is no
    /// `subsets` field at all), so each target is pushed as its own `Redefinition` reference
    /// using that target's own resolved span (via `qualified_reference`) rather than through
    /// `lower_subsetting_relationship`. `in`/`out`/`inout` direction, value binding, and
    /// calculation-expression body content are out of scope, sharing
    /// `UnsupportedFamily::CalcDefinitionMember` with the `def` form.
    fn lower_calc_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserCalcUsage>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CalcUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                direction: direction_fact(node.value.direction.as_ref()),
                // `ast::CalcUsage` has no `multiplicity` field; see planning/UPSTREAM_PARSER_GAPS.md.
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(targets) = &node.value.redefines {
            for target in targets.iter().copied() {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::Redefinition,
                    document,
                    local: target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    fn lower_rendering_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<RenderingDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RenderingDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        let RenderingDefBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                RenderingDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                RenderingDefBodyElement::Doc(node) => {
                    self.record_doc_comment(declaration, node)?;
                }
                RenderingDefBodyElement::Filter(filter) => {
                    self.lower_filter_expression(document, declaration, &filter.value.condition)?;
                }
                RenderingDefBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
                RenderingDefBodyElement::ViewRendering(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::RenderingDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers an `occurrence def` (BNF OccurrenceDefinition), mirroring `lower_port_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/item/part/nested-occurrence declarations. Occurrence-specific semantics
    /// (individual/portion-of-life, time-slicing, snapshot facts, `exhibit`/`succession`/
    /// `satisfy`/`allocate`/connector-end body constructs) are explicitly out of scope; unrecognized
    /// body elements fall through to `unsupported_occurrence_definition_member` via
    /// `lower_occurrence_body_element`. `OccurrenceDef.body` is the generic `DefinitionBody`
    /// (shared with e.g. `ItemDef`), which wraps the same `OccurrenceBodyElement` that
    /// `OccurrenceUsage.body` (`OccurrenceUsageBody`) holds directly -- both def and usage publish
    /// under one `UnsupportedFamily::OccurrenceDefinitionMember`.
    fn lower_occurrence_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<OccurrenceDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::OccurrenceDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        let DefinitionBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::Doc(node) => {
                    self.record_doc_comment(declaration, node)?;
                }
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers one `OccurrenceBodyElement`, shared by `occurrence def`'s body (wrapped in
    /// `DefinitionBodyElement::OccurrenceMember`), an `occurrence` usage's own owned members
    /// (`OccurrenceUsageBody` holds `OccurrenceBodyElement` directly), and `allocation def`/
    /// `flow def` bodies (also `DefinitionBodyElement::OccurrenceMember`): recognized owned
    /// members are attribute/part/item/nested-occurrence usages plus `end` declarations (lowered
    /// as connector-end references through the same `lower_end_decl`/`ReferenceKind::ConnectorEnd`
    /// machinery `connection def`/`interface def` use), plus `assert constraint` members
    /// (`lower_assert_constraint_member`); everything else -- flow usages, succession usages,
    /// `satisfy`, `allocate`, `exhibit` state usages -- falls through to
    /// `unsupported_occurrence_definition_member`.
    fn lower_occurrence_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        element: &Node<OccurrenceBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            OccurrenceBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            OccurrenceBodyElement::Doc(node) => {
                self.record_doc_comment(owner, node)?;
            }
            OccurrenceBodyElement::AttributeUsage(attribute) => {
                self.lower_attribute_usage(document, Some(owner), attribute)?;
            }
            OccurrenceBodyElement::PartUsage(part) => {
                self.lower_part_usage(document, Some(owner), part)?;
            }
            OccurrenceBodyElement::ItemUsage(item) => {
                self.lower_item_usage(document, Some(owner), item)?;
            }
            OccurrenceBodyElement::OccurrenceUsage(occurrence) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence)?;
            }
            OccurrenceBodyElement::EndDecl(end_decl) => {
                self.lower_end_decl(document, owner, end_decl)?;
            }
            OccurrenceBodyElement::StateUsage(state_usage) => {
                self.lower_state_usage(document, Some(owner), state_usage)?;
            }
            OccurrenceBodyElement::Satisfy(node) => {
                self.lower_satisfy(
                    document,
                    owner,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    node,
                )?;
            }
            OccurrenceBodyElement::Allocate(node) => {
                self.lower_allocate(
                    document,
                    owner,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    node,
                )?;
            }
            OccurrenceBodyElement::AssertConstraint(node) => self.lower_assert_constraint_member(
                document,
                owner,
                UnsupportedFamily::OccurrenceDefinitionMember,
                node,
            )?,
            OccurrenceBodyElement::Annotation(_)
            | OccurrenceBodyElement::FlowUsage(_)
            | OccurrenceBodyElement::SuccessionUsage(_) => self.push_unsupported(
                document,
                UnsupportedFamily::OccurrenceDefinitionMember,
                element.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `occurrence` feature member (BNF OccurrenceUsage),
    /// e.g. `occurrence o;` or `occurrence o : SomeOccurrence;`, mirroring `lower_port_usage`.
    /// `type_name` is a bare `QualifiedReferenceId` (like `ItemUsage`/`MetadataUsage`), not a
    /// structured `TypingRelationship`, but does carry an independent `type_is_conjugated` flag
    /// (mirrored as an explicit `RelationshipFlags::conjugated` fact on the pushed `FeatureTyping`
    /// reference, the same convention `lower_typing_relationship` uses for `PortUsage`). Individual/
    /// event/portion-of-life prefixes (`individual`/`then`/`event`/`ref`/`abstract`/`constant`,
    /// `portion_kind`) and the `event path` occurrence-reference shorthand are explicitly out of
    /// scope -- only the ordinary declaration/typing/subsetting shape is lowered. Owned members
    /// lower through the shared `lower_occurrence_body_element` (both def and usage share
    /// `OccurrenceBodyElement`).
    fn lower_occurrence_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserOccurrenceUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::OccurrenceUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    individual: node.value.is_individual,
                    reference: node.value.is_reference,
                    constant: node.value.is_constant,
                    event: node.value.is_event,
                    ..DeclarationModifiers::default()
                },
                portion_kind: portion_kind_fact(node.value.portion_kind.as_ref()),
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags {
                    conjugated: node.value.type_is_conjugated,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.crosses {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.intersects {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        let OccurrenceUsageBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            self.lower_occurrence_body_element(document, declaration, element)?;
        }
        Ok(())
    }

    /// Lowers an `allocation def` (BNF AllocationDefinition), mirroring `lower_occurrence_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/part/item/nested-occurrence declarations plus `end` connector-end structure via
    /// the shared `lower_occurrence_body_element` walker (`AllocationDef.body` is the same
    /// `DefinitionBody`/`OccurrenceBodyElement` shape `OccurrenceDef.body` uses). Allocation-
    /// specific semantics (the `allocate ... to ...` binding itself) are explicitly out of scope
    /// here -- see `DeclarationKind::AllocationDefinition`'s doc comment.
    fn lower_allocation_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AllocationDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AllocationDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        let DefinitionBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::Doc(node) => {
                    self.record_doc_comment(declaration, node)?;
                }
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers a `flow def` (BNF FlowDefinition), mirroring `lower_allocation_def`/
    /// `lower_occurrence_def`: ownership, membership, an optional `:>` specialization
    /// relationship, and owned attribute/part/item/nested-occurrence declarations plus `end`
    /// connector-end structure via the shared `lower_occurrence_body_element` walker
    /// (`FlowDef.body` is the same `DefinitionBody`/`OccurrenceBodyElement` shape
    /// `OccurrenceDef.body`/`AllocationDef.body` use). Flow-payload (`ref :>> payload : Type;`)
    /// and succession-flow semantics are explicitly out of scope here -- see
    /// `DeclarationKind::FlowDefinition`'s doc comment.
    fn lower_flow_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<FlowDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::FlowDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        let DefinitionBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::Doc(node) => {
                    self.record_doc_comment(declaration, node)?;
                }
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
            }
        }
        Ok(())
    }

    fn lower_subsetting_relationship(
        &mut self,
        document: DocumentId,
        source: DeclarationId,
        relationship: &Node<SubsettingRelationship>,
    ) -> Result<(), ConstructionError> {
        let kind = match relationship.value.kind {
            SubsettingKind::Subsets => ReferenceKind::Subsetting,
            SubsettingKind::References => ReferenceKind::References,
            SubsettingKind::Redefines => ReferenceKind::Redefinition,
            SubsettingKind::Crosses => ReferenceKind::Crosses,
            SubsettingKind::Intersects => ReferenceKind::Intersects,
        };
        for target in relationship.value.target.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source,
                kind,
                document,
                local: target,
                flags: RelationshipFlags {
                    implied: relationship.value.is_implied,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a package-level `alias X for Y;` member into a declaration plus an authored
    /// `AliasBinding` reference for `Y`, following the Subclassification/typing lowering pattern
    /// above: `target` is already a structured `QualifiedReferenceId` (not a flattened string), so
    /// it resolves through the same lexical lookup fixed point as every other authored reference.
    fn lower_alias_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AliasDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Alias,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Alias,
            self.member_visibility(&node.value.membership, ParserMembershipKind::Alias)?,
            node.value.membership.span.clone(),
        )?;
        let target = node.value.target;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::AliasBinding,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        if let AliasBody::Brace { elements, .. } = &node.value.body {
            self.lower_relationship_body_elements(document, Some(declaration), elements)?;
        }
        Ok(())
    }

    /// Lowers an `individual def` (BNF IndividualDef), mirroring `lower_item_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned members via the
    /// shared `lower_attribute_body` walker (`IndividualDef.body: AttributeBody` is the same
    /// shape `ItemDef`/`ClassDef` use).
    fn lower_individual_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<sysml_v2_parser_next::ast::IndividualDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::IndividualDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    // `individual def` is this declaration's own form; the `individual` prefix
                    // modifier belongs to the usages and definitions that carry `is_individual`.
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a keyword-less bare `connect <from> to <to> [:> <subsets>] [:>> <redefines>]
    /// { ... }` connector member (BNF `Connect`, `structure.rs` struct `Connect`, distinct from
    /// the `connect ... to ...;` sub-clause of an already-dispatched connector production modeled
    /// by `ConnectStmt`/`lower_connect_stmt`), e.g. a top-level `connect a to b;` package member.
    /// Sourced directly at the enclosing `owner` declaration (no separate declaration is
    /// synthesized), mirroring `lower_connect_stmt`'s anonymous shape: `from`/`to` resolve
    /// through the shared `lower_connector_end` walker, and an optional `:>`/`:>>` `subsets`/
    /// `redefines` clause resolves through the shared `lower_subsetting_relationship` helper.
    fn lower_bare_connect(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<sysml_v2_parser_next::ast::Connect>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::BareConnect,
            None,
            node.span.clone(),
            // A synthesized scope giving the bare `connect a to b;` ends a lexical owner.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_connector_end(document, declaration, &node.value.from)?;
        self.lower_connector_end(document, declaration, &node.value.to)?;
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        Ok(())
    }

    /// Lowers a `#<keyword>+ def <Name> ...` short-form definition (BNF ExtendedDefinition),
    /// mirroring `lower_package`: ownership, membership, an optional `:>` specialization
    /// relationship, and owned members through the same `lower_package_body` walker `body:
    /// PackageBody` shares with an ordinary `package { ... }`. `ExtendedDefinition` has no
    /// `Membership` node of its own (unlike `Package`, which also lowers with a synthesized
    /// `Owning`/`Default` membership for the same reason -- see `lower_package`), so membership is
    /// synthesized identically. The `#`-prefix keyword tags and `abstract`/`variation` prefix are
    /// out of scope; see `DeclarationKind::ExtendedDefinition`'s doc comment.
    fn lower_extended_definition(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ExtendedDefinition>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let (is_abstract, variation) =
            definition_prefix_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ExtendedDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    /// Lowers a `dependency` relationship declaration (BNF Dependency), mirroring `lower_satisfy`:
    /// an anonymous (or optionally named, via `Identification`) `DeclarationKind::Dependency`
    /// feature owned by the enclosing scope, with each `client`/`supplier` operand resolved as
    /// its own authored `ReferenceKind::DependencyClient`/`DependencySupplier` reference. Unlike
    /// `AliasDef`/`Import`, `Dependency` has no `membership: Membership` field of its own, so
    /// membership is always synthesized as `MembershipKind::Feature`/`Visibility::Default` at the
    /// declaration's own span (matching `lower_satisfy`'s anonymous-relationship shape).
    /// `body_elements` (doc/comment/metadata only) are walked through the same
    /// `lower_relationship_body_elements` helper `AliasDef`/`Import` use.
    fn lower_dependency(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<Dependency>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .as_ref()
            .and_then(|identification| identification.name.as_deref())
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(
            node.value
                .identification
                .as_ref()
                .and_then(|identification| identification.short_name.as_ref()),
        )?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Dependency,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        for target in node.value.clients.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::DependencyClient,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        for target in node.value.suppliers.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::DependencySupplier,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(elements) = &node.value.body_elements {
            self.lower_relationship_body_elements(document, Some(declaration), elements)?;
        }
        Ok(())
    }

    fn lower_typing_relationship(
        &mut self,
        document: DocumentId,
        source: DeclarationId,
        relationship: &Node<sysml_v2_parser_next::ast::TypingRelationship>,
    ) -> Result<(), ConstructionError> {
        self.lower_typing_relationship_impl(document, source, relationship, false)
    }

    /// Shared implementation behind `lower_typing_relationship`, with an extra `variation` flag
    /// only `lower_part_usage` sets (when its `usage_prefix` is `DefinitionPrefix::Variation`),
    /// mirroring the `conjugated` flag convention on a port's typing target: every other caller
    /// goes through the `lower_typing_relationship` wrapper above with `variation: false`.
    fn lower_typing_relationship_impl(
        &mut self,
        document: DocumentId,
        source: DeclarationId,
        relationship: &Node<sysml_v2_parser_next::ast::TypingRelationship>,
        variation: bool,
    ) -> Result<(), ConstructionError> {
        let kind = match relationship.value.kind {
            sysml_v2_parser_next::ast::TypingKind::Typing => ReferenceKind::FeatureTyping,
            sysml_v2_parser_next::ast::TypingKind::Subclassification => {
                ReferenceKind::Subclassification
            }
        };
        for target in relationship.value.target.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source,
                kind,
                document,
                local: target,
                flags: RelationshipFlags {
                    conjugated: relationship.value.is_conjugated,
                    implied: relationship.value.is_implied,
                    variation,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a `variant <name>;` member (BNF `VariantUsageElement`'s untyped reference form,
    /// `ast::VariantUsage`) found inside a `variation part`/`variation part def` body, mirroring
    /// `lower_view_satisfy`: the referenced sibling usage is a bare `QualifiedReferenceId` (not
    /// wrapped in an `Expression`), resolved as an authored `Variant` reference sourced directly
    /// at the enclosing variation `owner` declaration through the same `DeclarationDomain::Any`
    /// lexical lookup fixed point as `Succession`/`SatisfySource` -- no anonymous nested-
    /// declaration scope shift, since (unlike `Succession`/`Satisfy`) there is only one operand.
    /// The typed inline form (`VariantUsage.typed`, e.g. `variant part name : Type { ... }`)
    /// introduces a new usage rather than referencing an existing one -- out of scope, like
    /// `Satisfy.inline_requirement` -- for every `VariantTypedUsage` kind except `Perform`
    /// (`variant perform doX;` inside a `variation perform action ... { ... }` body): that kind
    /// wraps the exact same `ast::structure::Perform` node `lower_perform` already lowers for an
    /// ordinary `perform action ...;` usage, so it delegates there directly rather than staying
    /// unsupported -- there is no new lowering logic, just reuse. Every other `VariantTypedUsage`
    /// kind (`Part`/`Attribute`/`Item`/`Port`), the untyped form's optional nested body
    /// (`VariantUsage.body`), and the case where neither `reference` nor `typed` is present all
    /// fall through to an explicit unsupported-member diagnostic.
    fn lower_variant_usage(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<VariantUsage>,
    ) -> Result<(), ConstructionError> {
        if let Some(VariantTypedUsage::Perform(perform)) = &node.value.typed {
            if node.value.body.is_none() {
                return self.lower_perform(document, Some(owner), perform);
            }
        }
        if node.value.typed.is_some() || node.value.body.is_some() {
            self.push_unsupported(document, family, node.span.clone());
            return Ok(());
        }
        let Some(target) = node.value.reference else {
            self.push_unsupported(document, family, node.span.clone());
            return Ok(());
        };
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: owner,
            kind: ReferenceKind::Variant,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    fn member_visibility(
        &self,
        membership: &Membership,
        expected: ParserMembershipKind,
    ) -> Result<Visibility, ConstructionError> {
        if membership.kind != expected {
            return Err(ConstructionError::InvalidMembership);
        }
        Ok(membership
            .visibility
            .map(Self::visibility)
            .unwrap_or(Visibility::Default))
    }

    fn visibility(value: ParserVisibility) -> Visibility {
        match value {
            ParserVisibility::Public => Visibility::Public,
            ParserVisibility::Private => Visibility::Private,
            ParserVisibility::Protected => Visibility::Protected,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SymbolPath {
    start: u32,
    len: u32,
    rooted: bool,
}

#[derive(Debug)]
struct SymbolPathArena {
    paths: Box<[SymbolPath]>,
    segments: Box<[SymbolId]>,
}

impl SymbolPathArena {
    fn get(&self, id: SymbolPathId) -> Option<(&[SymbolId], bool)> {
        let path = self.paths.get(id.index())?;
        if path.len == 0 {
            return None;
        }
        let end = path.start.checked_add(path.len)?;
        let segments = self.segments.get(path.start as usize..end as usize)?;
        Some((segments, path.rooted))
    }
}

#[derive(Debug, Default)]
struct SymbolPathArenaBuilder {
    paths: Vec<SymbolPath>,
    segments: Vec<SymbolId>,
    index: HashTable<SymbolPathId>,
    hash_builder: RandomState,
}

impl SymbolPathArenaBuilder {
    fn push(
        &mut self,
        segments: &[SymbolId],
        rooted: bool,
    ) -> Result<SymbolPathId, ConstructionError> {
        if segments.is_empty() {
            return Err(ConstructionError::InvalidParserReference);
        }
        let hash = self.hash_builder.hash_one((rooted, segments));
        if let Some(existing) = self.index.find(hash, |candidate| {
            let path = self.paths[candidate.index()];
            let end = (path.start + path.len) as usize;
            path.rooted == rooted && self.segments[path.start as usize..end] == *segments
        }) {
            return Ok(*existing);
        }
        let id = SymbolPathId::from_index(self.paths.len())?;
        let start = u32::try_from(self.segments.len()).map_err(|_| ConstructionError::Capacity)?;
        let len = u32::try_from(segments.len()).map_err(|_| ConstructionError::Capacity)?;
        start.checked_add(len).ok_or(ConstructionError::Capacity)?;
        self.paths
            .try_reserve(1)
            .map_err(|_| ConstructionError::Capacity)?;
        self.segments
            .try_reserve(segments.len())
            .map_err(|_| ConstructionError::Capacity)?;
        let paths = &self.paths;
        let stored_segments = &self.segments;
        let hash_builder = &self.hash_builder;
        self.index
            .try_reserve(1, |candidate| {
                let path = paths[candidate.index()];
                let end = (path.start + path.len) as usize;
                hash_builder.hash_one((path.rooted, &stored_segments[path.start as usize..end]))
            })
            .map_err(|_| ConstructionError::Capacity)?;
        self.segments.extend_from_slice(segments);
        self.paths.push(SymbolPath { start, len, rooted });
        let paths = &self.paths;
        let stored_segments = &self.segments;
        let hash_builder = &self.hash_builder;
        self.index.insert_unique(hash, id, |candidate| {
            let path = paths[candidate.index()];
            let end = (path.start + path.len) as usize;
            hash_builder.hash_one((path.rooted, &stored_segments[path.start as usize..end]))
        });
        Ok(id)
    }

    fn freeze(self) -> SymbolPathArena {
        SymbolPathArena {
            paths: self.paths.into_boxed_slice(),
            segments: self.segments.into_boxed_slice(),
        }
    }
}

#[derive(Debug)]
struct SymbolTable {
    bytes: Box<str>,
    spans: Box<[(u32, u32)]>,
}

impl SymbolTable {
    fn get(&self, id: SymbolId) -> Option<&str> {
        let (start, len) = *self.spans.get(id.index())?;
        let end = start.checked_add(len)?;
        self.bytes.get(start as usize..end as usize)
    }

    fn find(&self, value: &str) -> Option<SymbolId> {
        self.spans.iter().enumerate().find_map(|(index, _)| {
            let id = SymbolId::from_index(index).ok()?;
            (self.get(id) == Some(value)).then_some(id)
        })
    }
}

#[derive(Debug, Default)]
struct SymbolTableBuilder {
    bytes: String,
    spans: Vec<(u32, u32)>,
    index: HashTable<SymbolId>,
    hash_builder: RandomState,
}

impl SymbolTableBuilder {
    fn len(&self) -> usize {
        self.spans.len()
    }

    fn get(&self, id: SymbolId) -> &str {
        let (start, len) = self.spans[id.index()];
        &self.bytes[start as usize..(start + len) as usize]
    }

    fn intern(&mut self, value: &str) -> Result<SymbolId, ConstructionError> {
        let hash = self.hash_builder.hash_one(value);
        if let Some(existing) = self.index.find(hash, |id| self.get(*id) == value) {
            return Ok(*existing);
        }

        let id = SymbolId::from_index(self.spans.len())?;
        let start = u32::try_from(self.bytes.len()).map_err(|_| ConstructionError::Capacity)?;
        let len = u32::try_from(value.len()).map_err(|_| ConstructionError::Capacity)?;
        start.checked_add(len).ok_or(ConstructionError::Capacity)?;
        self.bytes
            .try_reserve(value.len())
            .map_err(|_| ConstructionError::Capacity)?;
        self.spans
            .try_reserve(1)
            .map_err(|_| ConstructionError::Capacity)?;
        let bytes = &self.bytes;
        let spans = &self.spans;
        let hash_builder = &self.hash_builder;
        self.index
            .try_reserve(1, |candidate| {
                let (candidate_start, candidate_len) = spans[candidate.index()];
                hash_builder.hash_one(
                    &bytes[candidate_start as usize..(candidate_start + candidate_len) as usize],
                )
            })
            .map_err(|_| ConstructionError::Capacity)?;

        self.bytes.push_str(value);
        self.spans.push((start, len));
        let bytes = &self.bytes;
        let spans = &self.spans;
        let hash_builder = &self.hash_builder;
        self.index.insert_unique(hash, id, |candidate| {
            let (candidate_start, candidate_len) = spans[candidate.index()];
            hash_builder.hash_one(
                &bytes[candidate_start as usize..(candidate_start + candidate_len) as usize],
            )
        });
        Ok(id)
    }

    fn freeze(self) -> SymbolTable {
        SymbolTable {
            bytes: self.bytes.into_boxed_str(),
            spans: self.spans.into_boxed_slice(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OwnedSourceRecord {
    pub(crate) identity: Box<str>,
    pub(crate) role: SourceRole,
    pub(crate) content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuildSchedule {
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoordinatorError {
    DuplicateSourceIdentity,
    ConstructionFailed,
}

/// A library that has already been parsed and solved, ready to be reused by later publications.
///
/// Holds the parsed documents rather than their text, so a workspace build pays neither the
/// library's parse nor its solve. Lowering still runs: it is per-document and cheap, and rerunning
/// it keeps every dense identity assigned by exactly the same code path as an unseeded build.
#[derive(Debug)]
pub(crate) struct PreparedLibrary {
    pub(crate) documents: Vec<PreparedDocument>,
    pub(crate) settled: resolver::SettledLibrary,
}

#[derive(Debug)]
pub(crate) struct PreparedDocument {
    pub(crate) identity: Box<str>,
    pub(crate) role: SourceRole,
    pub(crate) parsed: Arc<ParsedDocument>,
    pub(crate) parse_errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SemanticModelBuildCoordinator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BuildPhaseDurations {
    pub(crate) parse: Duration,
    pub(crate) lowering: Duration,
    pub(crate) resolution: Duration,
}

impl SemanticModelBuildCoordinator {
    pub(crate) fn build_measured_with_library(
        mut sources: Vec<OwnedSourceRecord>,
        schedule: BuildSchedule,
        policy: EvaluationPolicy,
        library: Option<&PreparedLibrary>,
    ) -> Result<(resolver::ResolvedSemanticModel, BuildPhaseDurations), CoordinatorError> {
        // Library sources are ordered ahead of workspace sources so that the dense declaration
        // domain assigns them a contiguous prefix. Rendered output is sorted independently by
        // document identity, so this affects storage order only. Duplicate detection therefore
        // compares identities across the whole set, not within one role.
        sources.sort_unstable_by(|left, right| {
            source_admission_rank(left.role)
                .cmp(&source_admission_rank(right.role))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        let mut identities = sources
            .iter()
            .map(|source| source.identity.as_ref())
            .chain(
                library
                    .into_iter()
                    .flat_map(|library| library.documents.iter())
                    .map(|document| document.identity.as_ref()),
            )
            .collect::<Vec<_>>();
        identities.sort_unstable();
        if identities.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(CoordinatorError::DuplicateSourceIdentity);
        }

        let parse_started = Instant::now();
        let parsed: Vec<(Box<str>, SourceRole, sysml_v2_parser_next::ParseResult)> = match schedule
        {
            BuildSchedule::Sequential => sources
                .into_iter()
                .map(Self::parse_source)
                .collect::<Result<Vec<_>, _>>()?,
            BuildSchedule::Parallel => {
                use rayon::prelude::*;
                sources
                    .into_par_iter()
                    .map(Self::parse_source)
                    .collect::<Result<Vec<_>, _>>()?
            }
        };
        let parse = parse_started.elapsed();

        let lowering_started = Instant::now();
        let mut builder = SemanticModelBuilder::default();
        let mut documents = Vec::with_capacity(parsed.len());
        // The prepared library is admitted first and in its own recorded order, so its
        // declarations and references land on exactly the dense prefix its settled outcomes were
        // recorded against.
        for document in library.into_iter().flat_map(|library| &library.documents) {
            let admitted = builder
                .admit_document(
                    document.identity.clone(),
                    document.role,
                    Arc::clone(&document.parsed),
                    document.parse_errors.clone(),
                )
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push(admitted);
        }
        for (identity, role, parsed) in parsed {
            let document = builder
                .admit_document(identity, role, Arc::new(parsed.document), parsed.errors)
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push(document);
        }
        for document in documents {
            builder
                .canonicalize_document(document)
                .map_err(|_| CoordinatorError::ConstructionFailed)?;
        }
        let storage = builder.freeze();
        let lowering = lowering_started.elapsed();
        let resolution_started = Instant::now();
        let model = storage
            .resolve(policy, library.map(|library| &library.settled))
            .map_err(|_| CoordinatorError::ConstructionFailed)?;
        let resolution = resolution_started.elapsed();
        Ok((
            model,
            BuildPhaseDurations {
                parse,
                lowering,
                resolution,
            },
        ))
    }

    fn parse_source(
        source: OwnedSourceRecord,
    ) -> Result<(Box<str>, SourceRole, sysml_v2_parser_next::ParseResult), CoordinatorError> {
        Ok((
            source.identity,
            source.role,
            sysml_v2_parser_next::parse_for_editor_owned(source.content),
        ))
    }
}

/// Storage order for admitted sources: every library role precedes workspace sources.
///
/// This is a construction-order policy, not a semantic one. It exists so a library-only build and
/// a workspace-plus-library build assign the same declaration ids to the same library
/// declarations, which is the precondition for reusing a solved library stratum.
fn source_admission_rank(role: SourceRole) -> u8 {
    match role {
        SourceRole::StandardLibrary => 0,
        SourceRole::Library => 1,
        SourceRole::External => 2,
        SourceRole::Workspace => 3,
    }
}

mod element_kind;
mod evaluation;
pub(crate) mod resolver;

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_v2_parser_next::ast::{QualifiedReferenceArena, RootNamespace, SourceStorage};

    fn empty_document() -> Arc<ParsedDocument> {
        Arc::new(ParsedDocument {
            source: SourceStorage::default(),
            qualified_references: QualifiedReferenceArena::default(),
            root: RootNamespace {
                elements: Vec::new(),
            },
        })
    }

    #[test]
    fn canonicalization_assigns_dense_typed_slots_and_interns_names() {
        let mut builder = SemanticModelBuilder::default();
        let parsed = empty_document();
        let document = builder
            .admit_document("model", SourceRole::Workspace, parsed.clone(), Vec::new())
            .unwrap();
        let first_name = builder.intern_name("Vehicle").unwrap();
        let second_name = builder.intern_name("Vehicle").unwrap();
        assert_eq!(first_name, second_name);
        let root = builder
            .push_declaration(document, None, Some(first_name))
            .unwrap();
        let child = builder
            .push_declaration(document, Some(root), Some(second_name))
            .unwrap();

        let model = builder.freeze();
        assert_eq!(model.document(document).unwrap().identity.as_ref(), "model");
        assert!(Arc::ptr_eq(
            &model.document(document).unwrap().parsed,
            &parsed
        ));
        assert_eq!(model.declaration(root).unwrap().owner, None);
        assert_eq!(model.declaration(child).unwrap().owner, Some(root));
        assert_eq!(model.symbol(first_name), Some("Vehicle"));
        assert_eq!(model.symbols.spans.len(), 1);
    }

    #[test]
    fn symbol_interning_survives_hash_table_growth() {
        let mut symbols = SymbolTableBuilder::default();
        let vehicle = symbols.intern("Vehicle").unwrap();
        for index in 0..256 {
            symbols.intern(&format!("Name{index}")).unwrap();
        }

        assert_eq!(symbols.intern("Vehicle").unwrap(), vehicle);
        assert_eq!(symbols.len(), 257);
    }

    #[test]
    fn semantic_paths_are_interned_across_arena_growth() {
        let mut paths = SymbolPathArenaBuilder::default();
        let vehicle = paths.push(&[SymbolId(1), SymbolId(2)], false).unwrap();
        for index in 0..256 {
            paths
                .push(&[SymbolId(index), SymbolId(index + 1)], true)
                .unwrap();
        }

        assert_eq!(
            paths.push(&[SymbolId(1), SymbolId(2)], false).unwrap(),
            vehicle
        );
        assert_ne!(
            paths.push(&[SymbolId(1), SymbolId(2)], true).unwrap(),
            vehicle
        );
    }

    #[test]
    fn document_identity_index_rejects_duplicates_after_growth_without_mutation() {
        let parsed = empty_document();
        let mut builder = SemanticModelBuilder::default();
        for index in 0..256 {
            builder
                .admit_document(
                    format!("model-{index}"),
                    SourceRole::Workspace,
                    parsed.clone(),
                    Vec::new(),
                )
                .unwrap();
        }
        let before = builder.documents.len();

        assert_eq!(
            builder
                .admit_document("model-0", SourceRole::Workspace, parsed, Vec::new())
                .unwrap_err(),
            ConstructionError::DuplicateDocumentIdentity
        );
        assert_eq!(builder.documents.len(), before);
    }

    #[test]
    fn anonymous_ordinals_are_owner_local_and_ignore_named_declarations() {
        let parsed = empty_document();
        let mut builder = SemanticModelBuilder::default();
        let document = builder
            .admit_document("model", SourceRole::Workspace, parsed, Vec::new())
            .unwrap();
        let owner_name = builder.intern_name("Owner").unwrap();
        let owner = builder
            .push_typed_declaration(
                document,
                None,
                DeclarationKind::Package,
                Some(owner_name),
                Span::dummy(),
                DeclarationFacts::none(),
            )
            .unwrap();
        let first = builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::Import,
                None,
                Span::dummy(),
                DeclarationFacts::none(),
            )
            .unwrap();
        let named = builder.intern_name("Named").unwrap();
        builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::PartUsage,
                Some(named),
                Span::dummy(),
                DeclarationFacts::none(),
            )
            .unwrap();
        let second = builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::Import,
                None,
                Span::dummy(),
                DeclarationFacts::none(),
            )
            .unwrap();

        assert_eq!(
            builder.declarations[first.index()].anonymous_ordinal,
            Some(0)
        );
        assert_eq!(
            builder.declarations[second.index()].anonymous_ordinal,
            Some(1)
        );
    }

    fn build_semantic_sexpr(source: &str) -> String {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                source.to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published.debug().write_semantic_sexpr(&mut output).unwrap();
        output
    }

    #[test]
    fn enum_def_lowers_to_a_declaration_with_its_literal_as_an_owned_member() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tenum def StatusKind {\n\
             \t\tenum approved;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::StatusKind\"))) (kind enum-def)"),
            "expected an enum-def declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::StatusKind::approved\"))) (kind enum-literal)"
            ),
            "expected an owned enum-literal declaration with its own qualified name, got:\n{output}"
        );
    }

    #[test]
    fn attribute_typed_by_an_enum_def_resolves_its_feature_typing_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tenum def StatusKind {\n\
             \t\tenum approved;\n\
             \t}\n\
             \tattribute def Holder {\n\
             \t\tattribute status : StatusKind;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind featureTyping) (ordinal 0))\n      (authored-target \"StatusKind\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::StatusKind\"))))"
            ),
            "expected the attribute's featureTyping reference to StatusKind to resolve, got:\n{output}"
        );
    }

    #[test]
    fn enum_def_specializing_another_enum_def_resolves_its_subclassification_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tenum def Base {\n\
             \t\tenum on;\n\
             \t}\n\
             \tenum def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn requirement_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def MassRequirement {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::MassRequirement\"))) (kind requirement-def)"),
            "expected a requirement-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::MassRequirement::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the requirement def, got:\n{output}"
        );
    }

    #[test]
    fn requirement_def_specializing_another_requirement_def_resolves_its_subclassification_reference(
    ) {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def Base;\n\
             \trequirement def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn requirement_usage_typed_by_a_requirement_def_resolves_its_feature_typing_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def MassRequirement;\n\
             \tpart def Vehicle {\n\
             \t\trequirement massReq : MassRequirement;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind featureTyping) (ordinal 0))\n      (authored-target \"MassRequirement\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassRequirement\"))))"
            ),
            "expected the requirement usage's featureTyping reference to MassRequirement to resolve, got:\n{output}"
        );
    }

    #[test]
    fn port_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def InputPort {\n\
             \t\tattribute level : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::InputPort\"))) (kind port-def)"),
            "expected a port-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::InputPort::level\"))) (kind attribute)"),
            "expected an owned attribute declaration under the port def, got:\n{output}"
        );
    }

    #[test]
    fn port_def_specializing_another_port_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def Base;\n\
             \tport def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn connection_def_lowers_to_a_declaration() {
        // Bare `end name;` (no `:` type, `::>`/`references` target, or nested occurrence/item
        // usage) is not valid `EndDecl` grammar at all -- confirmed against the upstream parser's
        // `end_decl` (`src/parser/connector.rs`), which requires one of those three forms after
        // the name -- so a real end declaration must carry an explicit type here.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def P;\n\
             \tconnection def C {\n\
             \t\tend end1 : P;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::C\"))) (kind connection-def)"),
            "expected a connection-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::C::end1\"))) (kind connection)"),
            "expected an owned end declaration under the connection def, got:\n{output}"
        );
    }

    #[test]
    fn connection_def_specializing_another_connection_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconnection def Base;\n\
             \tconnection def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn connection_usage_connector_end_references_resolve_to_their_targets() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconnection def C;\n\
             \tpart d1;\n\
             \tpart d2;\n\
             \tconnection bus : C connect d1 to d2;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::bus\"))) (kind connection)"),
            "expected a connection usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::bus\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d1\")))"
            ),
            "expected bus's connector-end reference to d1 to resolve, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::bus\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d2\")))"
            ),
            "expected bus's connector-end reference to d2 to resolve, got:\n{output}"
        );
    }

    #[test]
    fn connector_end_dotted_member_access_resolves_through_its_bases_type() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def T {\n\
             \t\tpart bead;\n\
             \t}\n\
             \tconnection def C;\n\
             \tpart t : T;\n\
             \tpart d2;\n\
             \tconnection bus : C connect t.bead to d2;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"t::bead\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::T::bead\")))))"
            ),
            "expected t.bead to resolve to T's owned `bead` member, got:\n{output}"
        );
    }

    #[test]
    fn attribute_default_value_dotted_member_access_resolves_through_its_bases_type() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def F {\n\
             \t\tattribute a;\n\
             \t}\n\
             \tpart f : F;\n\
             \tattribute g = f.a;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"f::a\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::F::a\")))))"
            ),
            "expected f.a to resolve to F's owned `a` member, got:\n{output}"
        );
    }

    #[test]
    fn attribute_default_value_dotted_member_access_chain_resolves_through_multiple_hops() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def C3 {\n\
             \t\tattribute z;\n\
             \t}\n\
             \tpart def B3 {\n\
             \t\tpart c : C3;\n\
             \t}\n\
             \tpart def A3 {\n\
             \t\tpart b : B3;\n\
             \t}\n\
             \tpart a : A3;\n\
             \tattribute g = a.b.c.z;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"a::b::c::z\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C3::z\")))))"
            ),
            "expected the a.b.c.z chain to resolve through three hops to C3's owned `z` member, got:\n{output}"
        );
    }

    #[test]
    fn attribute_default_value_dotted_member_access_with_unresolvable_base_stays_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute g = nope.a;\n\
             }\n",
        );
        assert!(
            output.contains("(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"nope::a\")\n      (outcome (status unresolved))"),
            "expected an unresolvable base to leave the whole chain explicitly unresolved (never fabricated), got:\n{output}"
        );
    }

    #[test]
    fn attribute_default_value_dotted_member_access_with_missing_member_stays_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def F {\n\
             \t\tattribute a;\n\
             \t}\n\
             \tpart f : F;\n\
             \tattribute g = f.missing;\n\
             }\n",
        );
        assert!(
            output.contains("(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"f::missing\")\n      (outcome (status unresolved))"),
            "expected a member absent from f's type F to leave the chain explicitly unresolved (never fabricated), got:\n{output}"
        );
    }

    #[test]
    fn attribute_default_value_member_access_through_type_check_cast_resolves_through_the_operand()
    {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def F {\n\
             \t\tattribute a;\n\
             \t}\n\
             \tpart f : F;\n\
             \tattribute g = (f as F).a;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"f::a\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::F::a\")))))"
            ),
            "expected the TypeCheck cast wrapping f to be transparent, resolving (f as F).a exactly \
             like the uncast f.a case, got:\n{output}"
        );
    }

    #[test]
    fn attribute_default_value_member_access_through_parenthesized_base_resolves_through_the_operand(
    ) {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def F {\n\
             \t\tattribute a;\n\
             \t}\n\
             \tpart f : F;\n\
             \tattribute g = (f).a;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind memberAccessOperand) (ordinal 0))\n      (authored-target \"f::a\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::F::a\")))))"
            ),
            "expected the redundant parentheses around f to be transparent, resolving (f).a exactly \
             like the unparenthesized f.a case, got:\n{output}"
        );
    }

    #[test]
    fn interface_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def P;\n\
             \tinterface def I {\n\
             \t\tend end1 : P;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::I\"))) (kind interface-def)"),
            "expected an interface-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::I::end1\"))) (kind connection)"),
            "expected an owned end declaration under the interface def, got:\n{output}"
        );
    }

    #[test]
    fn interface_def_specializing_another_interface_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tinterface def Base;\n\
             \tinterface def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn interface_def_connect_stmt_connector_end_references_resolve_to_their_targets() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart d1;\n\
             \tpart d2;\n\
             \tinterface def I {\n\
             \t\tconnect d1 to d2;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::I\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d1\")))"
            ),
            "expected I's connector-end reference to d1 to resolve, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::I\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d2\")))"
            ),
            "expected I's connector-end reference to d2 to resolve, got:\n{output}"
        );
    }

    #[test]
    fn view_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tview def V;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::V\"))) (kind view-def)"),
            "expected a view-def declaration, got:\n{output}"
        );
    }

    #[test]
    fn view_def_specializing_another_view_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tview def Base;\n\
             \tview def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn constraint_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::C\"))) (kind constraint-def)"),
            "expected a constraint-def declaration, got:\n{output}"
        );
    }

    #[test]
    fn constraint_def_specializing_another_constraint_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def Base;\n\
             \tconstraint def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn constraint_usage_typed_by_a_constraint_def_resolves() {
        // planning/UPSTREAM_PARSER_GAPS.md #4 was resolved upstream in `0757de13`: `ConstraintUsage` now
        // carries `subsets`/`redefines` fields.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C;\n\
             \tconstraint c : C;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::c\"))) (kind constraint)"),
            "expected constraint c to lower to a declaration with kind constraint, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::c\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C\")))"
            ),
            "expected c's featureTyping of C to resolve, got:\n{output}"
        );
    }

    #[test]
    fn constraint_usage_subsetting_another_constraint_usage_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint baseConstraint;\n\
             \tconstraint derivedConstraint :> baseConstraint;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::derivedConstraint\"))) (kind constraint)"),
            "expected a constraint usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derivedConstraint\")))"
            ),
            "expected derivedConstraint's subsetting of baseConstraint to resolve, got:\n{output}"
        );
    }

    #[test]
    fn constraint_comparison_expression_resolves_both_operands() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute x : ScalarValues::Integer;\n\
             \tattribute y : ScalarValues::Integer;\n\
             \tconstraint def C { x > y }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"x\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::x\")))))"
            ),
            "expected x to resolve as an expressionOperand reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 1))\n      (authored-target \"y\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::y\")))))"
            ),
            "expected y to resolve as an expressionOperand reference, got:\n{output}"
        );
    }

    #[test]
    fn constraint_comparison_expression_leaves_undeclared_operand_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute x : ScalarValues::Integer;\n\
             \tconstraint def C { x > y }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"x\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::x\")))))"
            ),
            "expected x to resolve, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 1))\n      (authored-target \"y\")\n      (outcome (status unresolved))"
            ),
            "expected undeclared y to stay unresolved (not fabricated), got:\n{output}"
        );
    }

    #[test]
    fn constraint_literal_only_comparison_is_supported_with_no_operand_references() {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \tconstraint def C { 1 < 2 }\n\
                 }\n"
                .to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        assert!(
            !output.contains("unsupported_constraint_definition_member"),
            "did not expect an unsupported constraint-definition-member diagnostic for a \
             literal-only comparison, got:\n{output}"
        );
    }

    #[test]
    fn constraint_unsupported_expression_shape_still_falls_through_to_diagnostic() {
        // `Expression::Invocation` (e.g. `compute(x, y)`) is a supported shape as of this slice
        // (see `lower_invocation_callee`/`ReferenceKind::InvocationCallee`); `-`/`not` unary ops
        // are now supported too (`is_unary_operator`), so `~x` (`UnaryOperator::BitNot`, out of
        // scope, see `is_unary_operator`'s doc comment) exercises the still-unsupported path.
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \tconstraint def C { ~x }\n\
                 }\n"
                .to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        assert!(
            output.contains("unsupported_constraint_definition_member"),
            "expected a still-unsupported unary-op expression to surface as an unsupported \
             constraint-definition-member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn constraint_literal_comparison_evaluates_to_boolean_true() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { 1 < 2 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `1 < 2` to fold to a published Boolean(true) evaluation fact, got:\n{output}"
        );
        assert!(
            output.contains("(has-evaluation true)"),
            "expected has-evaluation to flip true once a fact publishes, got:\n{output}"
        );
    }

    #[test]
    fn constraint_literal_comparison_evaluates_to_boolean_false() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { 2 < 1 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean false)))"
            ),
            "expected `2 < 1` to fold to a published Boolean(false) evaluation fact, got:\n{output}"
        );
    }

    #[test]
    fn attribute_quantity_literal_default_value_evaluates_to_quantity_with_folded_magnitude() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass = 0[kg];\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::mass\"))) (state literal) (value (kind quantity) (magnitude (value \
                 (kind integer) (integer 0))) (unit \"kg\")))"
            ),
            "expected `attribute mass = 0[kg];` to fold its magnitude to Integer(0) while carrying \
             the authored unit token as a riding-along string fact, got:\n{output}"
        );
    }

    #[test]
    fn constraint_comparison_of_property_against_quantity_literal_resolves_both_operands() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass : ScalarValues::Integer;\n\
             \tconstraint def C { mass > 0[kg] }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"mass\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::mass\")))))"
            ),
            "expected `mass` to resolve as an expressionOperand reference, got:\n{output}"
        );
        assert!(
            !output.contains("unsupported_constraint_definition_member"),
            "expected `mass > 0[kg]` to be a supported shape (quantity-literal leaf), got:\n{output}"
        );
    }

    #[test]
    fn attribute_string_literal_default_value_evaluates_to_string() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute value = \"approved\";\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::value\"))) (state literal) (value (kind string) (value \"approved\")))"
            ),
            "expected `attribute value = \"approved\";` to fold to a published \
             EvaluatedValue::String(\"approved\") evaluation fact, got:\n{output}"
        );
    }

    #[test]
    fn constraint_string_equality_comparison_evaluates_to_boolean_true() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { \"a\" == \"a\" }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `\"a\" == \"a\"` to fold to a published Boolean(true) evaluation fact, \
             got:\n{output}"
        );
    }

    #[test]
    fn constraint_string_equality_comparison_evaluates_to_boolean_false() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { \"a\" == \"b\" }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean false)))"
            ),
            "expected `\"a\" == \"b\"` to fold to a published Boolean(false) evaluation fact, \
             got:\n{output}"
        );
    }

    #[test]
    fn assert_constraint_literal_comparison_evaluates_to_boolean_true() {
        // `assert constraint { <boolExpr> }` is semantically an anonymous constraint usage --
        // reuses the exact same `lower_constraint_expression`/`classify_constraint_expression`
        // evaluation machinery as `constraint def`/`constraint` (Slice 1, `4ca42166`).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P {\n\
             \t\tassert constraint { 1 < 2 }\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(state evaluated) (value (kind boolean) (boolean true)))"),
            "expected `assert constraint {{ 1 < 2 }}` to fold to a published Boolean(true) \
             evaluation fact, got:\n{output}"
        );
        assert!(
            output.contains("(has-evaluation true)"),
            "expected has-evaluation to flip true once a fact publishes, got:\n{output}"
        );
    }

    #[test]
    fn assert_constraint_operand_resolves_to_sibling_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P {\n\
             \t\tattribute x : ScalarValues::Integer;\n\
             \t\tattribute y : ScalarValues::Integer;\n\
             \t\tassert constraint { x > y }\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"x\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::x\")))))"
            ),
            "expected x to resolve to the sibling attribute declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 1))\n      (authored-target \"y\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::y\")))))"
            ),
            "expected y to resolve to the sibling attribute declaration, got:\n{output}"
        );
    }

    #[test]
    fn assert_constraint_typed_reference_form_resolves_its_type() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def MassConstraint;\n\
             \tpart def P {\n\
             \t\tassert constraint massConstraint : MassConstraint;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind featureTyping) (ordinal 0))\n      (authored-target \"MassConstraint\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassConstraint\")))))"
            ),
            "expected `assert constraint massConstraint : MassConstraint;` to resolve its type \
             reference through the shared FeatureTyping fixed point, got:\n{output}"
        );
    }

    #[test]
    fn constraint_resolved_feature_ref_operand_evaluates_to_non_constant() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute x : ScalarValues::Integer;\n\
             \tconstraint def C { x < 2 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state non-constant))"
            ),
            "expected a resolved but non-literal operand `x` to publish NonConstant rather than \
             a fabricated boolean, got:\n{output}"
        );
    }

    #[test]
    fn constraint_collection_op_arrow_invocation_resolves_base_and_argument_operands() {
        // `x->excludes(y)` (KerML `->` collection-operator invocation, e.g.
        // `derivedRequirements->excludes(originalRequirement)` in the Systems Library). The base
        // (`x`) and the argument (`y`) are both plain feature references and resolve exactly like
        // `Expression::Invocation`'s operands; the operator name (`excludes`) itself is a fixed
        // `CollectionOperator` enum value with no `QualifiedReferenceId` in the parser AST, so it
        // is never pushed as a reference.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P {\n\
             \t\tattribute x : ScalarValues::Integer;\n\
             \t\tattribute y : ScalarValues::Integer;\n\
             \t\tassert constraint { x->excludes(y) }\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"x\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::x\")))))"
            ),
            "expected `x` (the collection-op base) to resolve to the sibling attribute \
             declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 1))\n      (authored-target \"y\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::y\")))))"
            ),
            "expected `y` (the collection-op argument) to resolve to the sibling attribute \
             declaration, got:\n{output}"
        );
    }

    #[test]
    fn constraint_collection_op_arrow_invocation_evaluates_to_non_constant() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute x : ScalarValues::Integer;\n\
             \tattribute y : ScalarValues::Integer;\n\
             \tconstraint def C { x->excludes(y) }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state non-constant))"
            ),
            "expected `x->excludes(y)` to publish NonConstant, matching `Invocation`'s own \
             evaluation shape, got:\n{output}"
        );
    }

    #[test]
    fn constraint_undeclared_feature_ref_operand_evaluates_to_unresolved_operand() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { x < 2 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state unresolved-operand))"
            ),
            "expected an undeclared operand `x` to publish UnresolvedOperand rather than a \
             fabricated boolean, got:\n{output}"
        );
    }

    #[test]
    fn constraint_unsupported_arithmetic_shape_publishes_no_evaluation_fact() {
        // See `constraint_unsupported_expression_shape_still_falls_through_to_diagnostic`: an
        // invocation and `-`/`not` unary ops are now supported (reference-resolvable) shapes, so
        // this uses `~x` (`UnaryOperator::BitNot`), still genuinely unsupported, to exercise the
        // no-evaluation-fact path.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { ~x }\n\
             }\n",
        );
        assert!(
            !output.contains("(evaluated (declaration"),
            "expected an unsupported (non-comparison) expression shape to publish no evaluation \
             fact at all, got:\n{output}"
        );
        assert!(
            output.contains("(has-evaluation false)"),
            "expected has-evaluation to stay false when nothing evaluates, got:\n{output}"
        );
    }

    #[test]
    fn calc_literal_addition_evaluates_to_integer() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 2 + 3 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer 5)))"
            ),
            "expected `2 + 3` to fold to a published Integer(5) evaluation fact, got:\n{output}"
        );
    }

    #[test]
    fn calc_mixed_multiplication_evaluates_to_promoted_real() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 2.0 * 3 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind real) (real 6"
            ),
            "expected `2.0 * 3` to fold to a promoted Real(6.0) evaluation fact, got:\n{output}"
        );
    }

    #[test]
    fn calc_integer_division_by_zero_publishes_typed_division_by_zero_not_a_panic() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 10 / 0 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state division-by-zero))"
            ),
            "expected `10 / 0` to publish a typed DivisionByZero outcome rather than panicking, \
             got:\n{output}"
        );
    }

    #[test]
    fn calc_real_division_by_zero_publishes_typed_division_by_zero() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 10.0 / 0.0 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state division-by-zero))"
            ),
            "expected `10.0 / 0.0` to publish a typed DivisionByZero outcome rather than a \
             fabricated infinity, got:\n{output}"
        );
    }

    #[test]
    fn calc_propagates_constant_operands_through_referenced_attribute_default_values() {
        // `length` and `width` are both literal-default-valued attributes (slice 3); `Calc`
        // arithmetic-propagates through both, mirroring the constraint-body propagation tests.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute length = 4;\n\
             \tattribute width = 5;\n\
             \tcalc def Calc { length * width }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer 20)))"
            ),
            "expected `length * width` to propagate both attributes' literal defaults and fold to \
             Integer(20), got:\n{output}"
        );
    }

    #[test]
    fn calc_exponent_operator_integer_base_folds_to_integer() {
        // `**` (BinaryOperator::Exp) with a non-negative integer exponent stays `Integer` via
        // `checked_pow`, mirroring `fold_arithmetic`'s other checked-integer arms.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 2 ** 3 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer 8)))"
            ),
            "expected `2 ** 3` to fold to Integer(8), got:\n{output}"
        );
    }

    #[test]
    fn calc_exponent_operator_real_base_folds_to_real() {
        // `^` (BinaryOperator::Pow) with a `Real` base promotes to `Real` via `f64::powf`, the
        // same `Real`-involving promotion rule `fold_arithmetic` already uses for +/-/*//  /%.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 2.0 ^ 3 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind real) (real 8"
            ),
            "expected `2.0 ^ 3` to fold to a promoted Real(8.0), got:\n{output}"
        );
    }

    #[test]
    fn calc_exponent_operator_negative_integer_exponent_promotes_to_real() {
        // A negative integer exponent (`2 ^ -1`) cannot stay `Integer` (fractional result), so it
        // promotes to `Real` via `powf`, exactly like a `Real`-involving pairing.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 2 ^ -1 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind real) (real 0.5)))"
            ),
            "expected `2 ^ -1` to fold to Real(0.5) via the Real-promotion path, got:\n{output}"
        );
    }

    #[test]
    fn calc_exponent_operator_integer_overflow_folds_to_non_constant() {
        // A huge integer base/exponent pairing that overflows `checked_pow` conservatively folds
        // to `NonConstant`, never a panic, mirroring `fold_arithmetic`'s other checked-integer arms.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 99999999999 ** 99999999999 }\n\
             }\n",
        );
        assert!(
            output.contains("(state non-constant)"),
            "expected an overflowing `**` to publish a NonConstant evaluation fact, \
             got:\n{output}"
        );
    }

    #[test]
    fn constraint_arithmetic_mixed_with_comparison_folds_to_boolean() {
        // Mixing arithmetic into a constraint's comparison shape (`(a + b) > c`) is now supported:
        // `classify_constraint_node` recognizes an arithmetic `BinaryOp` operand nested inside a
        // comparison, reusing the same `EvalNode::Arithmetic` slice-4 already built for calc bodies.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { (1 + 2) > 0 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `(1 + 2) > 0` (arithmetic mixed with comparison) to fold to a published \
             Boolean(true) evaluation fact, got:\n{output}"
        );
    }

    #[test]
    fn constraint_arithmetic_operand_resolves_all_leaf_references() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute a : ScalarValues::Integer;\n\
             \tattribute b : ScalarValues::Integer;\n\
             \tattribute c : ScalarValues::Integer;\n\
             \tconstraint def C { (a + b) < c }\n\
             }\n",
        );
        for name in ["a", "b", "c"] {
            assert!(
                output.contains(&format!(
                    "(authored-target \"{name}\")\n      (outcome (status resolved) (target \
                     (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::{name}\")))))"
                )),
                "expected operand `{name}` in `(a + b) < c` to resolve to its sibling attribute \
                 declaration, got:\n{output}"
            );
        }
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state non-constant))"
            ),
            "expected `(a + b) < c` with no constant-valued operands to publish NonConstant \
             rather than a fabricated boolean, got:\n{output}"
        );
    }

    #[test]
    fn constraint_arithmetic_operand_constant_propagates_to_boolean() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass1 = 2;\n\
             \tattribute mass2 = 3;\n\
             \tattribute massLimit = 4;\n\
             \tconstraint def C { (mass1 + mass2) > massLimit }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `(mass1 + mass2) > massLimit` to constant-propagate through all three \
             attribute defaults and fold to Boolean(true) (2 + 3 = 5 > 4), got:\n{output}"
        );
    }

    #[test]
    fn constraint_logical_and_combines_two_comparisons_to_boolean() {
        // `and`/`or` combining multiple comparisons in a general constraint body (not just a
        // `filter <expr>;` condition, which already supported `and`/`or` for reference resolution
        // per `25c8bf52`) is the same "widen the recursive classifier" pattern applied to
        // evaluation: `EvalNode::Logical` folds two already-folded Boolean comparison operands.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass1 = 2;\n\
             \tattribute mass2 = 3;\n\
             \tattribute massLimit = 10;\n\
             \tattribute isActive = true;\n\
             \tconstraint def C { (mass1 + mass2) < massLimit and isActive }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `(mass1 + mass2) < massLimit and isActive` to fold to Boolean(true) \
             (2 + 3 = 5 < 10, and isActive is true), got:\n{output}"
        );
    }

    #[test]
    fn constraint_ampersand_folds_as_logical_and() {
        // KerML's single-`&` conjunction spelling (`BinaryOperator::BitAnd`, see
        // `is_logical_operator`'s doc comment) combines two comparisons exactly like `and`, e.g.
        // `sysml.library/trig_functions.md`'s `-1.0 <= that & that <= 1.0` shape.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass1 = 2;\n\
             \tattribute massLimit = 10;\n\
             \tconstraint def C { (mass1 < massLimit) & (massLimit > 0) }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `(mass1 < massLimit) & (massLimit > 0)` to fold to Boolean(true) via the same \
             `fold_logical` path as `and`, got:\n{output}"
        );
    }

    #[test]
    fn calc_unary_minus_negates_literal_integer() {
        // Unary negation (`UnaryOperator::Minus`) on a pure-literal calc body folds at
        // construction time (`eval_node_is_pure_literal`), exactly like a bare literal.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { -5 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer -5)))"
            ),
            "expected `-5` to fold to Integer(-5), got:\n{output}"
        );
    }

    #[test]
    fn constraint_unary_not_negates_literal_boolean() {
        // Unary logical negation (`UnaryOperator::Not`) on a literal boolean constraint body.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { not true }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean false)))"
            ),
            "expected `not true` to fold to Boolean(false), got:\n{output}"
        );
    }

    #[test]
    fn calc_unary_minus_resolves_feature_operand() {
        // `-x` with a resolvable feature operand: the operand reference resolves and, since it
        // has a known constant value, the whole expression folds through `fold_unary`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass = 5;\n\
             \tcalc def Calc { -mass }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer -5)))"
            ),
            "expected `-mass` (mass = 5) to resolve the operand reference and fold to \
             Integer(-5), got:\n{output}"
        );
    }

    #[test]
    fn constraint_logical_xor_combines_two_comparisons_to_boolean() {
        // `xor` shares `and`/`or`'s exact Boolean/Boolean truth-table shape (`is_logical_operator`
        // widened, `fold_logical`'s new `Xor` arm): true xor false = true.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass1 = 2;\n\
             \tattribute massLimit = 10;\n\
             \tattribute isActive = false;\n\
             \tconstraint def C { mass1 < massLimit xor isActive }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `mass1 < massLimit xor isActive` (true xor false) to fold to \
             Boolean(true), got:\n{output}"
        );
    }

    #[test]
    fn constraint_logical_implies_combines_two_comparisons_to_boolean() {
        // `implies`: false implies anything is true (`!left || right`).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass1 = 20;\n\
             \tattribute massLimit = 10;\n\
             \tattribute isActive = false;\n\
             \tconstraint def C { mass1 < massLimit implies isActive }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `mass1 < massLimit implies isActive` (false implies false) to fold to \
             Boolean(true), got:\n{output}"
        );
    }

    #[test]
    fn constraint_simple_comparison_only_regression_unaffected() {
        // Regression guard: a plain comparison-only constraint body (slices 1-3, no arithmetic or
        // logical widening involved) must fold exactly as before.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { 1 < 2 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected plain comparison-only `1 < 2` to still fold to Boolean(true), got:\n{output}"
        );
    }

    #[test]
    fn calc_arithmetic_only_regression_unaffected() {
        // Regression guard: calc-body arithmetic (slice 4) must stay comparison-free and fold
        // exactly as before -- unaffected by the constraint-side widening.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 2 + 3 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc\"))) (state evaluated) (value (kind integer) (integer 5)))"
            ),
            "expected plain arithmetic-only `2 + 3` calc body to still fold to Integer(5), \
             got:\n{output}"
        );
    }

    #[test]
    fn calc_anonymous_return_decl_arithmetic_evaluates_to_integer() {
        // Slice 5: most real-corpus calc arithmetic lives inside a `return : Type = expr;`
        // declaration, a distinct `CalcDefBodyElement::ReturnDecl` shape bd50fccd (slice 4)
        // deferred. This wires the return declaration's own expression through the exact same
        // classify_calc_expression/lower_calc_expression pipeline slices 1-4 already built.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { return : ScalarValues::Integer = 2 + 3; }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (path (named (kind package) (name \"Demo\")) (named (kind calc-def) (name \"Calc\")) (anonymous (kind parameter) (ordinal 0))))) (state evaluated) (value (kind integer) (integer 5)))"
            ),
            "expected `return : Type = 2 + 3;` to fold to a published Integer(5) evaluation fact \
             on the anonymous return declaration, got:\n{output}"
        );
    }

    #[test]
    fn calc_named_return_decl_lowers_declaration_and_evaluates_expression() {
        // `return name : Type = expr;` form: the name lowers as an owned declaration
        // (participating in the same lexical lookup as any other feature), AND the expression
        // evaluates through the shared pipeline.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { return result : ScalarValues::Integer = 4 * 5; }\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Calc::result\")"),
            "expected the named return declaration `result` to lower as its own owned \
             declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::Calc::result\"))) (state evaluated) (value (kind integer) (integer 20)))"
            ),
            "expected `return result : Type = 4 * 5;` to fold to a published Integer(20) \
             evaluation fact on the named return declaration, got:\n{output}"
        );
    }

    #[test]
    fn attribute_literal_default_value_publishes_its_own_evaluation_fact() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass = 5;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::mass\"))) (state literal) (value (kind integer) (integer 5)))"
            ),
            "expected a literal attribute default value to publish its own Integer(5) evaluation \
             fact, got:\n{output}"
        );
    }

    #[test]
    fn attribute_arithmetic_default_value_resolves_operands_and_evaluates() {
        // Widened value-assignment handling: `length * width` (arithmetic, not a bare literal)
        // now resolves both operand references and, since both are themselves constant-valued,
        // evaluates via the same classify_constraint_expression/EvalNode::Arithmetic machinery
        // slice 4/6ce84b06 built for constraint/calc bodies.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute length = 4;\n\
             \tattribute width = 5;\n\
             \tattribute area = length * width;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::area\"))) (state evaluated) (value (kind integer) (integer 20)))"
            ),
            "expected `attribute area = length * width;` to resolve both operands and fold to \
             Integer(20), got:\n{output}"
        );
        for name in ["length", "width"] {
            assert!(
                output.contains(&format!(
                    "(authored-target \"{name}\")\n      (outcome (status resolved) (target \
                     (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::{name}\")))))"
                )),
                "expected `area`'s arithmetic default value operand `{name}` to resolve to its \
                 sibling attribute declaration, got:\n{output}"
            );
        }
    }

    #[test]
    fn redefinition_value_with_a_qualified_reference_is_pushed_and_classified() {
        // The exact `enum_status_redefinition.md` shape (`attribute :>> status =
        // RequirementStatusKind::approved;`): the `= RequirementStatusKind::approved` value
        // portion publishes an `ExpressionOperand` reference (the shared lookup every
        // constraint/calc operand reference already uses) sourced at the redefining attribute's
        // own anonymous declaration, and -- since the multi-segment qualified-path lookup bug
        // fixed alongside this test -- now resolves to the enum literal, exactly as the same
        // qualified name would resolve if used as e.g. a `FeatureTyping` target.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tenum def RequirementStatusKind {\n\
             \t\tenum approved;\n\
             \t}\n\
             \trequirement def Base {\n\
             \t\tattribute status : RequirementStatusKind;\n\
             \t}\n\
             \trequirement def Derived :> Base {\n\
             \t\tattribute :>> status = RequirementStatusKind::approved;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(authored-target \"RequirementStatusKind::approved\")\n      (outcome (status \
                 resolved) (target (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::RequirementStatusKind::approved\")))))"
            ),
            "expected the redefinition value `RequirementStatusKind::approved` to resolve its \
             ExpressionOperand reference to the enum literal, got:\n{output}"
        );
    }

    #[test]
    fn multi_segment_qualified_expression_operand_resolves_through_nested_namespaces() {
        // Regression for the qualified-path `ExpressionOperand` lookup bug: `resolve_reference`'s
        // multi-segment segment loop was reading `exported_names` (the cross-file import
        // propagation index, which treats a member owned by a non-Package/LibraryPackage
        // namespace as private by KerML's default-visibility rule) instead of `direct_names` (the
        // unfiltered index every other same-scope qualified traversal -- e.g. usage-typing
        // redefinition targets -- reads from). A three-segment qualified name reaching through two
        // nested non-Package namespaces (`Outer::Inner::member`, `Inner` owned by `Outer`, not by
        // a package) now resolves, matching how `Outer::Inner` alone already resolved as e.g. a
        // `FeatureTyping` target.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Outer {\n\
             \t\tpart def Inner {\n\
             \t\t\tattribute member = 5;\n\
             \t\t}\n\
             \t}\n\
             \tattribute x = Outer::Inner::member;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(authored-target \"Outer::Inner::member\")\n      (outcome (status resolved) \
                 (target (node (document \"memory://test/enum.sysml\") (qualified-name \
                 \"Demo::Outer::Inner::member\")))))"
            ),
            "expected the three-segment qualified name `Outer::Inner::member` to resolve its \
             ExpressionOperand reference to the nested attribute, got:\n{output}"
        );
    }

    #[test]
    fn metadata_annotation_body_override_value_resolves() {
        // The metadata annotation body override deferred by `2680ca20` pending exactly this
        // value-assignment machinery: `isMandatory = true;` inside `@Safety{...}` now lowers
        // through the same shared pipeline as an attribute default value.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Safety {\n\
             \t\tattribute isMandatory : Boolean;\n\
             \t}\n\
             \tpart def Vehicle {\n\
             \t\tpart seatBelt[2] {@Safety{isMandatory = true;}}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (path (named (kind package) (name \"Demo\")) (named (kind part-def) (name \"Vehicle\")) (named (kind part) (name \"seatBelt\")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name \"isMandatory\"))))) (state literal) (value (kind \
                 boolean) (boolean true)))"
            ),
            "expected `isMandatory = true;` inside `@Safety{{...}}` to publish its own \
             Boolean(true) evaluation fact, got:\n{output}"
        );
    }

    #[test]
    fn parameter_default_value_with_member_access_resolves() {
        // The `out v_out : SpeedValue = vel.v;` shape deferred by `494b0ba6`: the parameter
        // default value now resolves its `vel.v` member-access operand through the exact same
        // pipeline `ReturnDecl::value` already used (bd50fccd precedent).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def Calc {\n\
             \t\tattribute vel;\n\
             \t\tout v_out = vel.v;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(memberAccessOperand (reference \"vel::v\"))"),
            "expected `out v_out = vel.v;`'s parameter default value to resolve `vel.v` as a \
             memberAccessOperand reference, got:\n{output}"
        );
    }

    #[test]
    fn non_constant_value_assignment_stays_non_constant_not_fabricated() {
        // A resolved-but-non-constant value assignment (`other`'s own default value is not
        // itself a known constant) must stay explicitly NonConstant, never fabricate a value.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute other : ScalarValues::Integer;\n\
             \tattribute mass = other;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::mass\"))) (state non-constant))"
            ),
            "expected `attribute mass = other;` (operand with no evaluation fact of its own) to \
             stay explicitly NonConstant, got:\n{output}"
        );
    }

    #[test]
    fn constraint_propagates_a_referenced_attributes_literal_default_value() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute mass = 5;\n\
             \tconstraint def C { mass > 3 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `mass > 3` to propagate through `attribute mass = 5;`'s own evaluated \
             constant and fold to Boolean(true), got:\n{output}"
        );
    }

    #[test]
    fn constraint_propagates_transitively_through_another_constraints_evaluated_value() {
        // Two-hop propagation through a non-attribute declaration: `B` folds to a literal
        // comparison, and `A` references `B` as a feature operand, so `A` should propagate `B`'s
        // own evaluated Boolean(true) rather than staying NonConstant.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def B { 1 < 2 }\n\
             \tconstraint def A { B == true }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::A\"))) (state evaluated) (value (kind boolean) (boolean true)))"
            ),
            "expected `A` to propagate `B`'s evaluated Boolean(true) and fold to Boolean(true), \
             got:\n{output}"
        );
    }

    #[test]
    fn constraints_referencing_each_others_evaluated_value_publish_non_converged() {
        // A genuine cross-declaration dependency cycle: `A`'s expression operand references `B`,
        // and `B`'s expression operand references `A`. Neither can ever settle to a concrete
        // constant; both must publish the explicit `NonConverged` outcome rather than hang,
        // panic, or fabricate a value.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def A { B == true }\n\
             \tconstraint def B { A == true }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::A\"))) (state cyclic))"
            ),
            "expected cyclic constraint A to publish NonConverged, got:\n{output}"
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::B\"))) (state cyclic))"
            ),
            "expected cyclic constraint B to publish NonConverged, got:\n{output}"
        );
    }

    #[test]
    fn constraint_operand_with_no_evaluated_value_at_all_still_stays_non_constant() {
        // `x` has no default value at all (no evaluation fact of its own), so `C` cannot
        // propagate any constant through it and must stay `NonConstant`, not fabricate a value.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute x : ScalarValues::Integer;\n\
             \tconstraint def C { x > 3 }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state non-constant))"
            ),
            "expected an operand with no evaluated value at all to keep the expression \
             NonConstant, got:\n{output}"
        );
    }

    #[test]
    fn concern_def_lowers_to_a_declaration() {
        // planning/UPSTREAM_PARSER_GAPS.md #9 was resolved upstream in `0757de13`: `ConcernUsage`
        // (which models both `concern def` and `concern` textual forms) now carries a
        // `type_name`/`subsets`/`redefines` field at all, previously entirely blocked.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconcern def C;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::C\"))) (kind concern-def)"),
            "expected a concern-def declaration, got:\n{output}"
        );
    }

    #[test]
    fn concern_usage_typed_by_a_concern_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconcern def C;\n\
             \tconcern c : C;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::c\"))) (kind concern)"),
            "expected concern c to lower to a declaration with kind concern, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::c\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C\")))"
            ),
            "expected c's featureTyping of C to resolve, got:\n{output}"
        );
    }

    #[test]
    fn concern_usage_subsetting_another_concern_usage_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconcern baseConcern;\n\
             \tconcern derivedConcern :> baseConcern;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::derivedConcern\"))) (kind concern)"),
            "expected a concern usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derivedConcern\")))"
            ),
            "expected derivedConcern's subsetting of baseConcern to resolve, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_lowers_to_a_declaration() {
        // planning/UPSTREAM_PARSER_GAPS.md #3 was resolved upstream in `0757de13`: `CalcDef` now carries a
        // `specializes` field. `calc def`/`calc` usage are only reachable inside a part body in
        // the typed AST (`calc_usage` is not dispatched at package level).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P {\n\
             \t\tcalc def Calc;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::P::Calc\"))) (kind calc-def)"),
            "expected a calc-def declaration, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_specializing_another_calc_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P {\n\
             \t\tcalc def Base;\n\
             \t\tcalc def Derived :> Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn calc_usage_typed_by_a_calc_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P {\n\
             \t\tcalc def Calc;\n\
             \t\tcalc c : Calc;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::P::c\"))) (kind calc)"),
            "expected calc c to lower to a declaration with kind calc, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::c\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::Calc\")))"
            ),
            "expected c's featureTyping of Calc to resolve, got:\n{output}"
        );
    }

    #[test]
    fn calc_usage_redefining_another_calc_usage_resolves() {
        // `CalcUsage::redefines` is a bare `Vec<QualifiedReferenceId>`, not a
        // `Node<SubsettingRelationship>` -- lowered as direct `Redefinition` references rather
        // than through `lower_subsetting_relationship`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P {\n\
             \t\tcalc def Calc;\n\
             \t\tcalc calcA : Calc;\n\
             \t\tcalc calcB : Calc :>> calcA;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::P::calcB\"))) (kind calc)"),
            "expected a calc usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind redefinition) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::calcB\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::calcA\")))"
            ),
            "expected calcB's redefinition of calcA to resolve, got:\n{output}"
        );
    }

    #[test]
    fn view_usage_typed_by_a_view_def_resolves() {
        // planning/UPSTREAM_PARSER_GAPS.md #8 was resolved upstream in `0757de13`: `ViewUsage` now carries
        // a `subsets` field, so `view` usage lowering is no longer deferred.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tview def V;\n\
             \tview v : V;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::V\"))) (kind view-def)"),
            "expected view def V to still lower to a declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::v\"))) (kind view)"),
            "expected view v to lower to a declaration with kind view, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::v\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::V\")))"
            ),
            "expected v's featureTyping of V to resolve, got:\n{output}"
        );
    }

    #[test]
    fn view_usage_subsetting_another_view_usage_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tview baseView;\n\
             \tview derivedView :> baseView;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::derivedView\"))) (kind view)"),
            "expected a view usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derivedView\")))"
            ),
            "expected derivedView's subsetting of baseView to resolve, got:\n{output}"
        );
    }

    #[test]
    fn rendering_usage_typed_and_subsetting_resolve() {
        // planning/UPSTREAM_PARSER_GAPS.md #26 was resolved upstream in `cb026cd`: `RenderingUsage` now
        // carries `subsets`/`redefines` fields (full parity with `ViewUsage`), so package-level
        // `rendering` usage lowering (previously unconditionally `unsupported_package_member`) is
        // no longer deferred.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trendering def R;\n\
             \trendering renderings : R;\n\
             \trendering asTree : R :> renderings;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::asTree\"))) (kind rendering)"),
            "expected a rendering usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::asTree\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::R\")))"
            ),
            "expected asTree's featureTyping of R to resolve, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::asTree\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::renderings\")))"
            ),
            "expected asTree's subsetting of renderings to resolve, got:\n{output}"
        );
    }

    #[test]
    fn use_case_usage_and_verification_case_usage_at_package_scope_resolve() {
        // `UseCaseUsage`/`VerificationCaseUsage` were previously unconditionally
        // `unsupported_package_member` at package scope even for the plain `use case <name> :
        // <Type> { ... }` header shape, which needs no multiplicity field (still missing
        // upstream, planning/UPSTREAM_PARSER_GAPS.md Gap 53).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tuse case def UC;\n\
             \tuse case uc : UC;\n\
             \tverification def V;\n\
             \tverification v : V;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::uc\"))) (kind use-case)"),
            "expected a use case usage declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::v\"))) (kind verification)"),
            "expected a verification case usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::uc\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::UC\")))"
            ),
            "expected uc's featureTyping of UC to resolve, got:\n{output}"
        );
    }

    #[test]
    fn viewpoint_usage_at_package_scope_resolves() {
        // `ViewpointUsage` was previously unconditionally `unsupported_package_member`. Only the
        // plain `viewpoint <name>[: <Type>]` header shape lowers: its `subsets`/`redefines`
        // clauses now parse but are not lowered yet (see `lower_viewpoint_usage`).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tviewpoint def VP;\n\
             \tviewpoint vp : VP;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::vp\"))) (kind viewpoint)"),
            "expected a viewpoint usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::vp\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::VP\")))"
            ),
            "expected vp's featureTyping of VP to resolve, got:\n{output}"
        );
    }

    #[test]
    fn interface_usage_declaration_typed_by_an_interface_def_resolves() {
        // planning/UPSTREAM_PARSER_GAPS.md #6 was resolved upstream in `0757de13`: all three
        // `InterfaceUsage` variants now carry `subsets`/`redefines` fields. Nested in a `part def`
        // body: `part/body.rs` tries `interface_usage` before `interface_def_required`, so a bare
        // `interface i : I;` (no `connect`) unambiguously parses as `InterfaceUsage::Declaration`
        // there, unlike at package level where `interface_def` (optional `def`) is tried first.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tinterface def I;\n\
             \tpart def P {\n\
             \t\tinterface i : I;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::I\"))) (kind interface-def)"),
            "expected interface def I to lower to a declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::P::i\"))) (kind interface)"),
            "expected interface i to lower to a declaration with kind interface, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::i\")))"
            ) && output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::I\")))"
            ),
            "expected i's featureTyping of I to resolve, got:\n{output}"
        );
    }

    #[test]
    fn interface_usage_subsetting_another_interface_usage_resolves() {
        // `interface_usage`'s `named_interface` capture requires a `:` typed form to consume the
        // name at all (a bare `name :> target` with no `: Type` never captures `name` -- see
        // `part::usage::interface_usage`'s doc comments), so both usages carry an explicit `: I`
        // typing target alongside the `:>` subsetting clause.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tinterface def I;\n\
             \tpart def P {\n\
             \t\tinterface baseInterface : I;\n\
             \t\tinterface derivedInterface : I :> baseInterface;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::P::derivedInterface\"))) (kind interface)"),
            "expected an interface usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::derivedInterface\")))"
            ),
            "expected derivedInterface's subsetting of baseInterface to resolve, got:\n{output}"
        );
    }

    #[test]
    fn occurrence_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \toccurrence def Occ {\n\
             \t\titem x;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Occ\"))) (kind occurrence-def)"),
            "expected an occurrence-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Occ::x\"))) (kind item)"),
            "expected an owned item usage under the occurrence def, got:\n{output}"
        );
    }

    #[test]
    fn occurrence_def_specializing_another_occurrence_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \toccurrence def Base;\n\
             \toccurrence def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn occurrence_usage_typed_by_an_occurrence_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \toccurrence def Occ;\n\
             \toccurrence o : Occ;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::o\"))) (kind occurrence)"),
            "expected an occurrence usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::o\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Occ\")))"
            ),
            "expected o's typing reference to Occ to resolve, got:\n{output}"
        );
    }

    #[test]
    fn occurrence_definition_member_body_construct_stays_explicitly_unsupported() {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \toccurrence def Occ {\n\
                 \t\tsuccession first x then y;\n\
                 \t}\n\
                 }\n"
                .to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        assert!(
            output.contains("unsupported_occurrence_definition_member"),
            "expected the succession usage to surface as an explicit unsupported diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn analysis_case_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tanalysis def FuelEconomyAnalysis {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::FuelEconomyAnalysis\"))) (kind analysis-def)"),
            "expected an analysis-def declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::FuelEconomyAnalysis::mass\"))) (kind attribute)"
            ),
            "expected an owned attribute declaration under the analysis def, got:\n{output}"
        );
    }

    #[test]
    fn analysis_case_def_specializing_another_analysis_case_def_resolves_its_specialization_reference(
    ) {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tanalysis def Base;\n\
             \tanalysis def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn analysis_case_usage_nested_in_an_analysis_def_body_lowers_to_a_declaration() {
        // planning/UPSTREAM_PARSER_GAPS.md #5 was resolved upstream in `0757de13`: `AnalysisCaseUsage` now
        // carries `subsets`/`redefines` fields with full parity to `RequirementUsage`, so a nested
        // `analysis` usage inside an `analysis def` body must lower as its own `analysis`
        // declaration with its `:` typing target resolved, not fall through to
        // `unsupported_analysis_case_definition_member`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tanalysis def Outer {\n\
             \t\tanalysis inner : Outer;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("Demo::Outer::inner"),
            "expected nested analysis usage declaration, got:\n{output}"
        );
        assert!(
            !output.contains("unsupported_analysis_case_definition_member"),
            "did not expect unsupported_analysis_case_definition_member, got:\n{output}"
        );
        assert!(
            output.contains("(kind analysis)"),
            "expected inner to lower with kind analysis, got:\n{output}"
        );
    }

    #[test]
    fn case_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcase def Investigation {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Investigation\"))) (kind case-def)"),
            "expected a case-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Investigation::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the case def, got:\n{output}"
        );
    }

    #[test]
    fn case_def_specializing_another_case_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcase def Base;\n\
             \tcase def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn case_usage_lowers_to_a_declaration_with_its_subsetting_resolved() {
        // planning/UPSTREAM_PARSER_GAPS.md #5 was resolved upstream in `0757de13`: `CaseUsage` now carries
        // `subsets`/`redefines` fields with full parity to `RequirementUsage`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcase baseCase;\n\
             \tcase derivedCase :> baseCase;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::derivedCase\"))) (kind case)"),
            "expected a case usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::derivedCase\")))"
            ),
            "expected derivedCase's subsetting of baseCase to resolve, got:\n{output}"
        );
    }

    #[test]
    fn case_definition_member_nested_action_usage_lowers_to_a_declaration() {
        // A nested `action` usage inside a `case def` body dispatches through the
        // `UseCaseDefBodyElement::ActionUsage` -> `lower_action_usage` wiring shared with
        // `use case def`/`verification def` bodies (they all use `UseCaseDefBody`).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcase def Outer {\n\
             \t\taction inner;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Outer::inner\"))) (kind action)"),
            "expected an owned action usage declaration under the case def, got:\n{output}"
        );
    }

    #[test]
    fn verification_case_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tverification def RangeVerification {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output
                .contains("(qualified-name \"Demo::RangeVerification\"))) (kind verification-def)"),
            "expected a verification-def declaration, got:\n{output}"
        );
        assert!(
            output
                .contains("(qualified-name \"Demo::RangeVerification::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the verification def, got:\n{output}"
        );
    }

    #[test]
    fn verification_case_def_specializing_another_verification_case_def_resolves_its_specialization_reference(
    ) {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tverification def Base;\n\
             \tverification def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn verification_case_definition_member_nested_action_usage_lowers_to_a_declaration() {
        // Same `UseCaseDefBodyElement::ActionUsage` -> `lower_action_usage` wiring as
        // `case_definition_member_nested_action_usage_lowers_to_a_declaration`, exercised
        // through the `verification def` body shape.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tverification def Outer {\n\
             \t\taction inner;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Outer::inner\"))) (kind action)"),
            "expected an owned action usage declaration under the verification def, got:\n{output}"
        );
    }

    #[test]
    fn use_case_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tuse case def PurchaseTicket {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::PurchaseTicket\"))) (kind use-case-def)"),
            "expected a use-case-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::PurchaseTicket::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the use case def, got:\n{output}"
        );
    }

    #[test]
    fn use_case_def_specializing_another_use_case_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tuse case def Base;\n\
             \tuse case def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn use_case_definition_member_nested_action_usage_lowers_to_a_declaration() {
        // Same `UseCaseDefBodyElement::ActionUsage` -> `lower_action_usage` wiring as
        // `case_definition_member_nested_action_usage_lowers_to_a_declaration`, exercised
        // through the `use case def` body shape.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tuse case def Outer {\n\
             \t\taction inner;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Outer::inner\"))) (kind action)"),
            "expected an owned action usage declaration under the use case def, got:\n{output}"
        );
    }

    #[test]
    fn conjugated_port_usage_typing_reference_resolves_and_carries_the_conjugated_flag() {
        // `port p : ~Base;` nested inside a `part def` body dispatches through the real
        // `PortUsage` grammar production (package-level bare `port name : Type;` instead folds
        // into `PortDef`, see `lower_port_def`'s doc comment) -- the `~` conjugation polarity
        // must survive as an explicit fact distinct from the (unconjugated) target declaration.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def Base;\n\
             \tpart def Holder {\n\
             \t\tport p : ~Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Holder::p\"))) (kind port)"),
            "expected a port usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (conjugated true) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::p\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected p's conjugated typing reference to Base to resolve with the conjugated flag, got:\n{output}"
        );
    }

    #[test]
    fn non_conjugated_port_usage_typing_reference_does_not_carry_the_conjugated_flag() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def Base;\n\
             \tpart def Holder {\n\
             \t\tport p : Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::p\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected p's unconjugated typing reference to Base to resolve without a conjugated flag, got:\n{output}"
        );
        assert!(
            !output.contains("(kind typing) (conjugated true)"),
            "did not expect the conjugated flag on an unconjugated port typing reference, got:\n{output}"
        );
    }

    #[test]
    fn item_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \titem def Widget {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget\"))) (kind item-def)"),
            "expected an item-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the item def, got:\n{output}"
        );
    }

    #[test]
    fn item_def_specializing_another_item_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \titem def Base;\n\
             \titem def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn item_usage_typed_by_an_item_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \titem def Base;\n\
             \tpart def Holder {\n\
             \t\titem w : Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Holder::w\"))) (kind item)"),
            "expected an item usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::w\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected w's typing reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn constraint_def_in_parameter_lowers_and_resolves_with_a_direction_fact() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute def MassValue;\n\
             \tconstraint def MassConstraint {\n\
             \t\tin partMasses : MassValue;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::MassConstraint::partMasses\"))) (kind parameter)"
            ),
            "expected a parameter declaration for partMasses, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (direction in) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassConstraint::partMasses\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassValue\")))"
            ),
            "expected partMasses's typing reference to MassValue to resolve with an `in` direction fact, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_out_parameter_lowers_and_resolves_with_a_direction_fact() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute def Real;\n\
             \tcalc def Sum {\n\
             \t\tout result : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Sum::result\"))) (kind parameter)"),
            "expected a parameter declaration for result, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (direction out) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Sum::result\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Real\")))"
            ),
            "expected result's typing reference to Real to resolve with an `out` direction fact, got:\n{output}"
        );
    }

    #[test]
    fn action_def_inout_parameter_lowers_and_resolves_with_a_direction_fact() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \titem def Image;\n\
             \taction def Focus {\n\
             \t\tinout image : Image;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Focus::image\"))) (kind parameter)"),
            "expected a parameter declaration for image, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (direction inout) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Focus::image\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Image\")))"
            ),
            "expected image's typing reference to Image to resolve with an `inout` direction fact, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_untyped_parameter_still_lowers_a_declaration_shell() {
        // `in seq[1..*];` (BNF `InOutDecl` with no `type_name`, only a multiplicity) must still
        // lower as a declaration -- no `FeatureTyping`/direction fact is pushed for it (there is
        // no type to reference), but the declaration/membership shell is not skipped. Mirrors
        // `sysml.library/interfaces.md`'s `excludingOnce` calc's `in seq[1..*] nonunique ordered;`
        // line minus the `nonunique`/`ordered` collection modifiers, which are lowered as their
        // own modifier facts and are not what this test pins.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def ExcludingOnce {\n\
             \t\tin seq[1..*];\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::ExcludingOnce::seq\"))) (kind parameter)"),
            "expected a parameter declaration for untyped seq, got:\n{output}"
        );
        assert!(
            !output.contains("(kind typing)"),
            "expected no FeatureTyping reference for untyped seq, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_parameter_subsets_clause_resolves_as_a_subsetting_relationship() {
        // `in value :> seq;` on a *named* `InOutDecl` is an authored subsetting clause, carried on
        // `ast::InOutDecl::subsets`. The parser previously folded the `:>` spelling into
        // `type_name`, which reported a subsetting as a typing; the two clauses are now separate
        // fields, so this lowers through `lower_subsetting_relationship` like every other
        // `:>` clause and no typing reference is invented for it.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def ExcludingOnce {\n\
             \t\tin seq;\n\
             \t\tin value :> seq;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::ExcludingOnce::value\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::ExcludingOnce::seq\")))"
            ),
            "expected value's `:>` clause to resolve as a subsetting relationship to seq, got:\n{output}"
        );
        assert!(
            !output.contains("(kind typing)"),
            "expected no FeatureTyping reference for the subsets clause, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_anonymous_redefinition_parameter_lowers_its_redefines_relationship() {
        // The leading `in :>> target = expr;` spelling is the one case that actually populates
        // `ast::InOutDecl::redefines` (a `Node<SubsettingRelationship>`), independent of whether a
        // type is present (`type_name` stays `None` here). `lower_parameter_declaration` now
        // lowers this via the same `lower_subsetting_relationship` helper `AttributeUsage`/
        // `ItemUsage` already call, so the redefinition target reference resolves.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Sum {\n\
             \t\tin target;\n\
             \t\tin :>> target = 1;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind redefinition) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Sum::\")))"
            ) || output.contains("(kind redefinition)"),
            "expected an anonymous parameter's redefinition reference to lower, got:\n{output}"
        );
        assert!(
            output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Sum::target\")))"
            ),
            "expected the redefinition reference to resolve to target, got:\n{output}"
        );
    }

    #[test]
    fn requirement_subject_declaration_lowers_and_resolves_its_typing() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Vehicle;\n\
             \trequirement vehicleSpecification {\n\
             \t\tsubject vehicle : Vehicle;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::vehicleSpecification::vehicle\"))) (kind subject)"
            ),
            "expected a subject declaration for vehicle, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::vehicleSpecification::vehicle\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Vehicle\")))"
            ),
            "expected vehicle's typing reference to Vehicle to resolve, got:\n{output}"
        );
    }

    #[test]
    fn use_case_subject_declaration_lowers_and_resolves_its_typing() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Vehicle;\n\
             \tcase def Inspect {\n\
             \t\tsubject vehicle : Vehicle;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Inspect::vehicle\"))) (kind subject)"),
            "expected a subject declaration for vehicle, got:\n{output}"
        );
    }

    #[test]
    fn requirement_actor_declaration_lowers_and_resolves_its_typing() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Operator;\n\
             \trequirement def FlightRequirement {\n\
             \t\tactor pilot : Operator;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::FlightRequirement::pilot\"))) (kind requirement-actor)"
            ),
            "expected a requirement-actor declaration for pilot, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::FlightRequirement::pilot\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Operator\")))"
            ),
            "expected pilot's typing reference to Operator to resolve, got:\n{output}"
        );
    }

    #[test]
    fn stakeholder_typed_declaration_lowers_and_resolves_its_typing() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Driver;\n\
             \trequirement def SafetyRequirement {\n\
             \t\tstakeholder driver : Driver;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::SafetyRequirement::driver\"))) (kind stakeholder)"
            ),
            "expected a stakeholder declaration for driver, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::SafetyRequirement::driver\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Driver\")))"
            ),
            "expected driver's typing reference to Driver to resolve, got:\n{output}"
        );
    }

    #[test]
    fn stakeholder_concern_reference_resolves_through_the_any_domain_lookup() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconcern modularity;\n\
             \tviewpoint def SystemView {\n\
             \t\tstakeholder modularity;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind stakeholderTarget)"),
            "expected a stakeholderTarget reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::modularity\")))"
            ),
            "expected the stakeholder concern reference to resolve to modularity, got:\n{output}"
        );
    }

    #[test]
    fn stakeholder_redefinition_resolves_through_the_redefinition_reference_kind() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconcern modularity;\n\
             \tviewpoint def SystemView {\n\
             \t\tstakeholder :>> modularity;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind redefinition) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Demo\")) (named (kind viewpoint-def) (name \"SystemView\")) (anonymous (kind stakeholder) (ordinal 0))))"
            ),
            "expected a redefinition reference sourced at the anonymous stakeholder declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::modularity\")))"
            ),
            "expected the stakeholder redefinition to resolve to modularity, got:\n{output}"
        );
    }

    #[test]
    fn purpose_member_resolves_its_concern_target() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconcern modularity;\n\
             \tviewpoint def SystemView {\n\
             \t\tpurpose modularity;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind purposeTarget)"),
            "expected a purposeTarget reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::modularity\")))"
            ),
            "expected the purpose reference to resolve to modularity, got:\n{output}"
        );
    }

    #[test]
    fn frame_member_recurses_into_its_nested_body_content() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Driver;\n\
             \trequirement def SafetyRequirement {\n\
             \t\tframe concernFraming {\n\
             \t\t\tstakeholder driver : Driver;\n\
             \t\t}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::SafetyRequirement::concernFraming\"))) (kind frame)"
            ),
            "expected a frame declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::SafetyRequirement::concernFraming::driver\"))) (kind stakeholder)"
            ),
            "expected the nested stakeholder to lower under the frame, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::SafetyRequirement::concernFraming::driver\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Driver\")))"
            ),
            "expected the nested stakeholder's typing reference to resolve, got:\n{output}"
        );
    }

    #[test]
    fn verify_requirement_shorthand_resolves_its_target() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement speedRequirement;\n\
             \trequirement def CheckSpeed {\n\
             \t\tverify speedRequirement;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind verify-requirement)"),
            "expected a verify-requirement declaration, got:\n{output}"
        );
        assert!(
            output.contains("(kind verifyRequirementTarget)"),
            "expected a verifyRequirementTarget reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::speedRequirement\")))"
            ),
            "expected the verify target to resolve to speedRequirement, got:\n{output}"
        );
    }

    #[test]
    fn subject_ref_shorthand_is_recognized_and_ignored() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tviewpoint def SystemView {\n\
             \t\tsubject;\n\
             \t}\n\
             }\n",
        );
        assert!(
            !output.contains("unsupported_requirement_definition_member"),
            "expected the bare `subject;` shorthand not to be reported as unsupported, got:\n{output}"
        );
    }

    #[test]
    fn perform_action_usage_inside_a_part_def_lowers_and_resolves_its_typing() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def GenerateTorque;\n\
             \tpart def Engine {\n\
             \t\tperform action generateTorque: GenerateTorque;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::Engine::generateTorque\"))) (kind perform-action)"
            ),
            "expected a perform-action declaration for generateTorque, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Engine::generateTorque\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::GenerateTorque\")))"
            ),
            "expected generateTorque's typing reference to GenerateTorque to resolve, got:\n{output}"
        );
    }

    #[test]
    fn perform_action_usage_inside_an_action_def_lowers_and_resolves_its_typing() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def Sub;\n\
             \taction def Main {\n\
             \t\tperform action step: Sub;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Main::step\"))) (kind perform-action)"),
            "expected a perform-action declaration for step, got:\n{output}"
        );
    }

    #[test]
    fn class_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tclass Widget {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget\"))) (kind class-def)"),
            "expected a class-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the class def, got:\n{output}"
        );
    }

    #[test]
    fn class_def_specializing_another_class_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tclass Base;\n\
             \tclass Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn kerml_classifier_decl_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tstruct Widget {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget\"))) (kind kerml-structure)"),
            "expected `struct Widget` to lower as KerML `Structure`, not a generic classifier \
             bucket, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the struct, got:\n{output}"
        );
    }

    /// Each KerML classifier keyword denotes its own concrete metaclass -- the spec makes them a
    /// subtype lattice (`Predicate <: Function <: Behavior <: Class <: Classifier <: Type`,
    /// `Structure <: Class`, `Interaction <: Association, Behavior`, `Multiplicity <: Feature`) --
    /// and `ast::KermlClassifierDecl.keyword` already carries the spelling, so none of them may
    /// collapse into a shared bucket. `assoc` and `association` are two spellings of one keyword
    /// and so are the one exception; `subclassifier` is not a classifier keyword at all -- it
    /// declares a subclassification *relationship* (`ast::KermlRelationshipDecl`), which this
    /// slice reports as an unsupported package member.
    #[test]
    fn each_kerml_classifier_keyword_lowers_to_its_own_metaclass() {
        for (source, kind) in [
            ("type K;", "kerml-type"),
            ("classifier K;", "kerml-classifier"),
            ("struct K;", "kerml-structure"),
            ("assoc K;", "kerml-association"),
            ("association K;", "kerml-association"),
            ("assoc struct K;", "kerml-association-structure"),
            ("datatype K;", "kerml-datatype"),
            ("metaclass K;", "kerml-metaclass"),
            ("behavior K;", "kerml-behavior"),
            ("function K;", "kerml-function"),
            ("predicate K;", "kerml-predicate"),
            ("interaction K;", "kerml-interaction"),
            ("multiplicity K [0..1];", "kerml-multiplicity"),
        ] {
            // Both spellings reach `KermlClassifierDecl`: the bare forward declaration as a `;`
            // body, and the bodied form as a brace body. Each must land on the same metaclass.
            let bodied = source.replace(';', " { }");
            for spelling in [source, bodied.as_str()] {
                let output = build_semantic_sexpr(&format!("package Demo {{\n\t{spelling}\n}}\n"));
                assert!(
                    output.contains(&format!("(qualified-name \"Demo::K\"))) (kind {kind})")),
                    "expected `{spelling}` to lower as {kind}, got:\n{output}"
                );
            }
        }

        // A plain `class K { }` is claimed by the dedicated `class_def` production, so it lowers
        // as `ClassDefinition`; `KermlClassifierKeyword::Class` is reached only for the shapes
        // `class_def` rejects (see that variant's own doc comment).
        let class_def = build_semantic_sexpr("package Demo {\n\tclass K { }\n}\n");
        assert!(
            class_def.contains("(qualified-name \"Demo::K\"))) (kind class-def)"),
            "expected plain `class` to keep using the dedicated class-def production, got:\n\
             {class_def}"
        );
    }

    /// The same, for the KerML feature kind keywords: `BooleanExpression <: Expression <: Step <:
    /// Feature` are four distinct metaclasses, carried by `ast::KermlFeatureMember.kind`.
    #[test]
    fn each_kerml_feature_keyword_lowers_to_its_own_metaclass() {
        for (source, kind) in [
            ("feature f : Real;", "kerml-feature"),
            ("step f : Real;", "kerml-step"),
            ("expr f : Real;", "kerml-expression"),
            ("bool f : Real;", "kerml-boolean-expression"),
        ] {
            let output = build_semantic_sexpr(&format!(
                "package Demo {{\n\tstruct S {{\n\t\tderived {source}\n\t}}\n}}\n"
            ));
            assert!(
                output.contains(&format!("(qualified-name \"Demo::S::f\"))) (kind {kind})")),
                "expected `{source}` to lower as {kind}, got:\n{output}"
            );
        }
    }

    #[test]
    fn kerml_classifier_decl_specializing_another_classifier_resolves_its_specialization_reference()
    {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tstruct Base;\n\
             \tstruct Derived specializes Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn kerml_classifier_decl_nested_inside_calc_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Outer {\n\
             \t\tstruct Inner;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Outer::Inner\"))) (kind kerml-structure)"),
            "expected a nested `struct` declaration inside the calc def, got:\n{output}"
        );
    }

    #[test]
    fn kerml_feature_member_lowers_to_a_declaration_with_typing() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tderived feature x : Integer;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::x\"))) (kind kerml-feature)"),
            "expected a kerml-feature declaration for x, got:\n{output}"
        );
        assert!(
            output.contains("(relationships (featureTyping (reference \"Integer\")))"),
            "expected x's FeatureTyping reference, got:\n{output}"
        );
    }

    #[test]
    fn kerml_feature_member_redefines_resolves_its_target() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tderived feature base : Integer;\n\
             \tderived feature x redefines base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::base\")))"
            ),
            "expected x's redefinition of base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn kerml_feature_member_nested_inside_classifier_decl_resolves_its_typing() {
        // `classifier { ... }` bodies share the `CalcDefBody` grammar (b7d6ac36), so a bare
        // `feature` member inside a `classifier` body dispatches through the same
        // `CalcDefBodyElement::KermlFeature` -> `lower_kerml_feature_member` path already used
        // for package-level and calc-def-nested feature members; it should resolve identically.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tclassifier Wheel {}\n\
             \tclassifier Bicycle {\n\
             \t\tfeature rollsOn : Wheel [2];\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Bicycle::rollsOn\"))) (kind kerml-feature)"),
            "expected a nested kerml-feature declaration for rollsOn, got:\n{output}"
        );
        assert!(
            output.contains("(relationships (featureTyping (reference \"Wheel\")))"),
            "expected rollsOn's FeatureTyping reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Wheel\"))))"
            ),
            "expected rollsOn's featureTyping reference to Wheel to resolve, got:\n{output}"
        );
    }

    #[test]
    fn kerml_connector_member_lowers_ends_and_typing() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tclassifier Bicycle {\n\
             \t\tfeature rollsOn : Wheel;\n\
             \t\tfeature holdsWheel : BikeFork;\n\
             \t\tconnector fixWheel : BikeWheelFixed from rollsOn to holdsWheel;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output
                .contains("(qualified-name \"Demo::Bicycle::fixWheel\"))) (kind kerml-connector)"),
            "expected a kerml-connector declaration for fixWheel, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Bicycle::fixWheel\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Bicycle::rollsOn\")))"
            ),
            "expected fixWheel's `from` end to resolve to rollsOn, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Bicycle::fixWheel\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Bicycle::holdsWheel\")))"
            ),
            "expected fixWheel's `to` end to resolve to holdsWheel, got:\n{output}"
        );
    }

    #[test]
    fn kerml_binding_member_lowers_left_and_right_ends() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tclassifier Bicycle {\n\
             \t\tfeature startShot : Integer;\n\
             \t\tfeature endShot : Integer;\n\
             \t\tbinding startShot = endShot;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind kerml-binding)"),
            "expected a kerml-binding declaration, got:\n{output}"
        );
        assert!(
            output.contains("(kind bindSource)") && output.contains("(kind bindTarget)"),
            "expected bindSource/bindTarget references for startShot/endShot, got:\n{output}"
        );
    }

    #[test]
    fn kerml_end_member_lowers_its_subsets_and_nested_feature() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tassoc HappensDuring {\n\
             \t\tfeature timeCoincidentOccurrences : Occurrence;\n\
             \t\tfeature longerOccurrence : Occurrence;\n\
             \t\tend happensDuring subsets timeCoincidentOccurrences feature thatOccurrence: \
             Occurrence redefines longerOccurrence;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::HappensDuring::happensDuring\"))) (kind kerml-end)"
            ),
            "expected a kerml-end declaration for happensDuring, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind subsetting) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::HappensDuring::happensDuring\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::HappensDuring::timeCoincidentOccurrences\")))"
            ),
            "expected happensDuring's end-level subsets to resolve to timeCoincidentOccurrences, got:\n{output}"
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::HappensDuring::happensDuring::thatOccurrence\"))) (kind kerml-feature)"
            ),
            "expected the nested feature thatOccurrence to be owned by the happensDuring end, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind redefinition) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::HappensDuring::happensDuring::thatOccurrence\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::HappensDuring::longerOccurrence\")))"
            ),
            "expected thatOccurrence's redefines to resolve to longerOccurrence, got:\n{output}"
        );
    }

    #[test]
    fn kerml_invariant_member_lowers_its_boolean_expression() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tclassifier Bicycle {\n\
             \t\tfeature isClosed : Boolean;\n\
             \t\tinv unitBound {\n\
             \t\t\tisClosed\n\
             \t\t}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output
                .contains("(qualified-name \"Demo::Bicycle::unitBound\"))) (kind kerml-invariant)"),
            "expected a kerml-invariant declaration for unitBound, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_nested_inside_calc_def_lowers_to_a_declaration() {
        // `CalcDefBodyElement::CalcDef`/`CalcUsage`/`PartUsage` dispatch into a `calc def`
        // body's own already-existing `lower_calc_def`/`lower_calc_usage`/`lower_part_usage`
        // functions, mirroring the same nesting already supported inside `part def` bodies.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Outer {\n\
             \t\tcalc def Inner;\n\
             \t\tcalc rollup : Inner;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Outer::Inner\"))) (kind calc-def)"),
            "expected a nested calc-def declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Outer::rollup\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Outer::Inner\")))"
            ),
            "expected the nested calc usage's typing reference to Inner to resolve, got:\n{output}"
        );
    }

    #[test]
    fn kerml_succession_member_lowers_first_and_then_ends() {
        // `CalcDefBodyElement::Succession` (`KermlSuccessionMember`) was previously
        // unconditionally unsupported despite `lower_kerml_connector_end` already existing to
        // lower its identical `KermlConnectorEnd`-shaped operands (see the exhaustive
        // `unsupported_calc_definition_member` audit's `a_3_6_sequences.md`/
        // `a_3_7_decisions_and_merges.md` KerML Spec Annex A fixtures).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tbehavior Manufacture {\n\
             \t\tstep paint : Paint;\n\
             \t\tstep dry : Dry;\n\
             \t\tsuccession p_before_d first paint then dry;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output
                .contains("(qualified-name \"Demo::Manufacture::p_before_d\"))) (kind succession)"),
            "expected a succession declaration for p_before_d, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind succession) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Manufacture::p_before_d\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Manufacture::paint\")))"
            ),
            "expected p_before_d's first end to resolve to paint, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind succession) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Manufacture::p_before_d\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Manufacture::dry\")))"
            ),
            "expected p_before_d's then end to resolve to dry, got:\n{output}"
        );
        assert!(
            !output.contains("unsupported_calc_definition_member"),
            "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_body_typed_parameter_member_lowers_typing_and_value() {
        // `CalcDefBodyElement::TypedParameter` (`TypedParameterMember`) was previously
        // unconditionally unsupported; mirrors `lower_parameter_declaration`'s shape (a
        // direction-tagged `FeatureTyping` reference plus `= expr` value evaluation), e.g. Kernel
        // Function Library `ControlFunctions.kerml`'s `in bool onOccurrence = ...;`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def C {\n\
             \t\tin a : Boolean;\n\
             \t\tin expr p : Boolean = a;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::C::p\"))) (kind parameter)"),
            "expected a parameter declaration for p, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind featureTyping) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C::p\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Boolean\")))"
            ) || output.contains("(authored-target \"Boolean\")\n      (outcome (status unresolved))"),
            "expected p's featureTyping reference to Boolean, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C::p\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C::a\")))"
            ),
            "expected p's value expression to resolve its operand a, got:\n{output}"
        );
        assert!(
            !output.contains("unsupported_calc_definition_member"),
            "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_body_typed_parameter_member_redefines_and_nested_body() {
        // The anonymous redefinition-only form (`in bool redefines onOccurrence { ... }`, Kernel
        // Semantic Library `Observation.kerml`) folds its empty `name` to `None` (mirroring
        // `lower_return_decl`'s identical `ReturnDecl.name` handling) and recurses into its own
        // nested `CalcDefBody` (a field `InOutDecl` has no equivalent for at all).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def C {\n\
             \t\tin a : Boolean;\n\
             \t\tin bool redefines a {\n\
             \t\t\treturn : Boolean;\n\
             \t\t}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind redefinition) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Demo\")) (named (kind calc-def) (name \"C\")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C::a\")))"
            ),
            "expected the anonymous parameter's redefines to resolve to a, got:\n{output}"
        );
        assert!(
            output.contains(
                "(path (named (kind package) (name \"Demo\")) (named (kind calc-def) (name \"C\")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference \"Boolean\"))))"
            ),
            "expected the nested anonymous return declaration (typed `: Boolean`, no direction/\
             redefines) to lower too, got:\n{output}"
        );
        assert!(
            !output.contains("unsupported_calc_definition_member"),
            "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_body_assert_constraint_member_lowers_to_a_declaration() {
        // `CalcDefBodyElement::AssertConstraint` was previously unconditionally unsupported
        // despite `lower_assert_constraint_member` already existing (wired for
        // `ConstraintDefBodyElement`/case-family bodies) -- pure mechanical dispatch wiring, same
        // shape as `9_6cbf` originally added it for. Real-corpus site: Kernel Semantic Library
        // `ScalarValues.kerml`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def C {\n\
             \t\tin a : Boolean;\n\
             \t\tassert constraint check : Boolean {\n\
             \t\t\ta\n\
             \t\t}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::C::check\"))) (kind assert-constraint)"),
            "expected an assert-constraint declaration for check, got:\n{output}"
        );
        assert!(
            !output.contains("unsupported_calc_definition_member"),
            "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_body_import_member_lowers_its_target() {
        // `CalcDefBodyElement::Import` was previously unconditionally unsupported despite
        // `lower_import` already accepting an `Option<DeclarationId>` owner -- pure mechanical
        // dispatch wiring. Real-corpus site: Kernel Function/Semantic Libraries' `private import
        // ...;`/`comment`-adjacent members inside a `calc def`/KerML classifier body.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpackage Other {\n\
             \t\tattribute def X;\n\
             \t}\n\
             \tcalc def C {\n\
             \t\tprivate import Other::*;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind import)"),
            "expected an import declaration owned by C, got:\n{output}"
        );
        assert!(
            !output.contains("unsupported_calc_definition_member"),
            "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_body_comment_member_is_ignored() {
        // `CalcDefBodyElement::Comment` mirrors `PartDefBodyElement::Comment`/
        // `PackageBodyElement::Comment`'s existing inert no-op treatment (like `Doc`) rather than
        // being unconditionally unsupported.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def C {\n\
             \t\tcomment /* a note about C */\n\
             \t}\n\
             }\n",
        );
        assert!(
            !output.contains("unsupported_calc_definition_member"),
            "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn lower_calc_expression_supports_comparison_logical_and_conditional_operators() {
        // `lower_calc_expression`'s `BinaryOp` arm was originally arithmetic-only; the exhaustive
        // `unsupported_calc_definition_member` audit found real-corpus calc-body formulas
        // routinely use comparison/logical operators too (e.g. Kernel Function Library
        // `BaseFunctions.kerml`'s `return : Boolean[1] = not (x == y);`), plus the `Conditional`
        // (`if <test> ? <then> else <else>`) expression shape, previously unhandled entirely.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def C {\n\
             \t\tin a : Boolean;\n\
             \t\tin b : Boolean;\n\
             \t\tin c : Boolean;\n\
             \t\treturn : Boolean = if (a == b and not c) ? a else b;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Demo\")) (named (kind calc-def) (name \"C\")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C::a\")))"
            ),
            "expected the return's conditional expression to resolve its operand a, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Demo\")) (named (kind calc-def) (name \"C\")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C::b\")))"
            ),
            "expected the return's conditional expression to resolve its operand b, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (source (node (document \"memory://test/enum.sysml\") (path (named (kind package) (name \"Demo\")) (named (kind calc-def) (name \"C\")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::C::c\")))"
            ),
            "expected the return's conditional expression to resolve its operand c, got:\n{output}"
        );
        assert!(
            !output.contains("unsupported_calc_definition_member"),
            "expected no unsupported_calc_definition_member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn constraint_usage_nested_inside_requirement_def_lowers_to_a_declaration() {
        // `RequirementDefBodyElement::Constraint` dispatches into the already-existing
        // `lower_constraint_usage`, mirroring the real Systems Library
        // `RequirementCheck`/`RequirementConstraintCheck` shape (redefining
        // `assumptions`/`constraints`).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def Base;\n\
             \trequirement def Outer {\n\
             \t\tconstraint assumptions : Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Outer::assumptions\"))) (kind constraint)"),
            "expected a nested constraint usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Outer::assumptions\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected assumptions' typing reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn alias_def_nested_inside_part_def_lowers_to_a_declaration() {
        // `PartDefBodyElement::AliasDef`/`PartUsageBodyElement::AliasDef` dispatch into the
        // already-existing `lower_alias_def` (previously only reachable at package scope).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P {\n\
             \t\tport porig;\n\
             \t\talias po for porig;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::P::po\"))) (kind alias)"),
            "expected a nested alias declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind aliasBinding) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::po\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::porig\")))"
            ),
            "expected po's alias binding to porig to resolve, got:\n{output}"
        );
    }

    #[test]
    fn dependency_lowers_clients_and_suppliers() {
        // `PackageBodyElement::Dependency` dispatches into the new `lower_dependency`: each
        // client/supplier is resolved as its own authored reference.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart a;\n\
             \tpart b;\n\
             \tdependency Use from a to b;\n\
             }\n",
        );
        assert!(
            output.contains("(kind dependency)"),
            "expected a dependency declaration, got:\n{output}"
        );
        assert!(
            output.contains("(kind dependencyClient)")
                && output.contains(
                    "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::a\")))"
                ),
            "expected a's dependencyClient reference to resolve, got:\n{output}"
        );
        assert!(
            output.contains("(kind dependencySupplier)")
                && output.contains(
                    "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::b\")))"
                ),
            "expected b's dependencySupplier reference to resolve, got:\n{output}"
        );
    }

    #[test]
    fn extended_definition_lowers_owned_members_and_specialization() {
        // `PackageBodyElement::ExtendedDefinition` dispatches into the new
        // `lower_extended_definition`, reusing `lower_package_body` for `#<keyword> def`'s
        // owned members and `lower_typing_relationship` for its `:>` clause.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Base;\n\
             \t#scenario def Failure :> Base {\n\
             \t\tattribute cause : Boolean;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Failure\"))) (kind extended-definition)"),
            "expected an extended-definition declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Failure::cause\"))) (kind attribute-def)"),
            "expected Failure's nested attribute usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Failure\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Failure's specialization reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn individual_def_lowers_to_a_declaration_with_specialization() {
        // `PackageBodyElement::IndividualDef` dispatches into the new `lower_individual_def`,
        // mirroring `lower_item_def`/`lower_class_def`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Base;\n\
             \tindividual def Widget :> Base {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget\"))) (kind individual-definition)"),
            "expected an individual-definition declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
            "expected Widget's nested attribute usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Widget\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Widget's specialization reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn bare_connect_at_package_scope_resolves_ends() {
        // `PackageBodyElement::Connect` (the keyword-less `Connect` struct, distinct from
        // `ConnectStmt`) dispatches into the new `lower_bare_connect`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart a;\n\
             \tpart b;\n\
             \tconnect a to b;\n\
             }\n",
        );
        assert!(
            output.contains("(kind connectorEnd)")
                && output.contains(
                    "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::a\")))"
                )
                && output.contains(
                    "(target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::b\")))"
                ),
            "expected both connector ends to resolve, got:\n{output}"
        );
    }

    #[test]
    fn item_usage_nested_inside_port_def_lowers_to_a_declaration() {
        // `PortDefBodyElement::ItemDef`/`ItemUsage` and `PortBodyElement::ItemUsage` dispatch
        // into the already-existing `lower_item_def`/`lower_item_usage`. A `port def` body's
        // item usage must carry an `in`/`out`/`inout` direction prefix (BNF `directed_item_usage`
        // -- unlike a plain `port` usage body's undirected `item_usage`, see
        // `PortBodyElement::ItemUsage`).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \titem def Widget;\n\
             \tport def P {\n\
             \t\tin item w : Widget;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::P::w\"))) (kind item)"),
            "expected a nested item usage declaration under the port def, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::P::w\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Widget\")))"
            ),
            "expected w's typing reference to Widget to resolve, got:\n{output}"
        );
    }

    #[test]
    fn metadata_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Safety {\n\
             \t\tattribute isMandatory : Boolean;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Safety\"))) (kind metadata-def)"),
            "expected a metadata-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Safety::isMandatory\"))) (kind attribute)"),
            "expected an owned attribute declaration under the metadata def, got:\n{output}"
        );
    }

    #[test]
    fn metadata_def_specializing_another_metadata_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Base;\n\
             \tmetadata def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn metadata_usage_typed_by_a_metadata_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Base;\n\
             \tpart def Holder {\n\
             \t\tmetadata m : Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Holder::m\"))) (kind metadata)"),
            "expected a metadata usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::m\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected m's typing reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn metadata_annotation_on_part_usage_resolves_the_annotation_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Safety {\n\
             \t\tattribute isMandatory : Boolean;\n\
             \t}\n\
             \tpart def Vehicle {\n\
             \t\tpart seatBelt[2] {@Safety{isMandatory = true;}}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind metadataAnnotation) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Vehicle::seatBelt\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Safety\")))"
            ),
            "expected seatBelt's @Safety metadata annotation reference to resolve, got:\n{output}"
        );
    }

    #[test]
    fn metadata_annotation_with_unresolvable_target_stays_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Vehicle {\n\
             \t\tpart seatBelt[2] {@NoSuchMetadata;}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind metadataAnnotation)") && output.contains("(status unresolved)"),
            "expected seatBelt's @NoSuchMetadata metadata annotation reference to stay explicitly unresolved, got:\n{output}"
        );
    }

    #[test]
    fn filter_metadata_test_resolves_the_metadata_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Safety;\n\
             \tpackage 'Safety Features' {\n\
             \t\tpublic import Demo::**;\n\
             \t\tfilter @Safety;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind filterMetadataTest) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Safety Features\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Safety\")))"
            ),
            "expected the filter's @Safety metadata-test reference to resolve, got:\n{output}"
        );
    }

    #[test]
    fn filter_and_expression_resolves_both_metadata_test_and_operand_references() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Safety {\n\
             \t\tattribute isMandatory : Boolean;\n\
             \t}\n\
             \tpackage 'Mandatory Safety Features' {\n\
             \t\tpublic import Demo::**;\n\
             \t\tfilter @Safety and Safety::isMandatory;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind filterMetadataTest) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Mandatory Safety Features\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Safety\")))"
            ),
            "expected the filter's @Safety metadata-test reference to resolve, got:\n{output}"
        );
        // `Safety::isMandatory`'s second segment is a `metadata def`-owned attribute with default
        // (non-package-owner) visibility, so it is not effective-public and the qualified lexical
        // lookup's second segment finds no exported candidate -- exactly the same shape
        // `40_filtering_example_1.md`'s real-corpus fixture exercises (see `Safety::isMandatory`'s
        // own `featureTyping` reference staying unresolved there too). The reference is still
        // authored, sourced, and explicitly unresolved rather than silently dropped or unsupported.
        assert!(
            output.contains("(kind expressionOperand)")
                && output.contains("(authored-target \"Safety::isMandatory\")")
                && output.contains("(status unresolved)"),
            "expected the filter's Safety::isMandatory operand reference to be resolved-attempted \
             and stay explicitly unresolved (not unsupported), got:\n{output}"
        );
    }

    #[test]
    fn filter_with_not_unary_operator_resolves_its_operand() {
        // `lower_filter_expression` previously had no `Expression::UnaryOp` arm at all (unlike
        // `lower_calc_expression`/`lower_constraint_expression`, which both already recurse
        // through `not`), so `not <operand>` inside a `filter` statement always fell to the
        // blanket `unsupported_package_member` diagnostic even though the operand itself is an
        // ordinary resolvable reference (`kerml/filtering.md`'s `filter (... and not
        // Type::isAbstract) or ...`).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Safety {\n\
             \t\tattribute isMandatory : Boolean;\n\
             \t}\n\
             \tpackage 'Not Mandatory' {\n\
             \t\tpublic import Demo::**;\n\
             \t\tfilter not Safety::isMandatory;\n\
             \t}\n\
             }\n",
        );
        assert!(
            !output.contains("unsupported_package_member"),
            "expected `not Safety::isMandatory` to no longer trip the blanket unsupported \
             diagnostic, got:\n{output}"
        );
        assert!(
            output.contains("(kind expressionOperand)")
                && output.contains("(authored-target \"Safety::isMandatory\")"),
            "expected the filter's `not`-wrapped operand reference to still be lowered and \
             attempted, got:\n{output}"
        );
    }

    #[test]
    fn filter_with_unresolvable_metadata_target_stays_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpackage 'Safety Features' {\n\
             \t\tpublic import Demo::**;\n\
             \t\tfilter @NoSuchMetadata;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind filterMetadataTest)") && output.contains("(status unresolved)"),
            "expected the filter's @NoSuchMetadata metadata-test reference to stay explicitly unresolved, got:\n{output}"
        );
    }

    #[test]
    fn action_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def ExecuteMission {\n\
             \t\taction validateRoute;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::ExecuteMission\"))) (kind action-def)"),
            "expected an action-def declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::ExecuteMission::validateRoute\"))) (kind action)"
            ),
            "expected an owned nested action usage declaration under the action def, got:\n{output}"
        );
    }

    #[test]
    fn action_def_specializing_another_action_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def Base;\n\
             \taction def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn action_usage_typed_by_an_action_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def Base;\n\
             \taction a : Base;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::a\"))) (kind action)"),
            "expected an action usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::a\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected a's typing reference to Base to resolve, got:\n{output}"
        );
    }

    /// `first X then Y;` inside an action def body now lowers as a resolved `succession`
    /// relationship (see `crate::tests::first_then_succession_inside_action_def_body_resolves_both_ends`
    /// in `lib.rs` for the full assertion); it no longer falls through to the generic
    /// unsupported-member diagnostic this test originally locked in per commit `f4ae83f7`.
    #[test]
    fn first_then_succession_inside_an_action_def_no_longer_surfaces_as_unsupported() {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \taction def ExecuteMission {\n\
                 \t\taction validateRoute;\n\
                 \t\taction startMission;\n\
                 \t\tfirst validateRoute then startMission;\n\
                 \t}\n\
                 }\n"
                .to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        assert!(
            !output.contains("unsupported_action_definition_member"),
            "did not expect an unsupported action-definition-member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn state_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tstate def SD {\n\
             \t\tstate s;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::SD\"))) (kind state-def)"),
            "expected a state-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::SD::s\"))) (kind state)"),
            "expected an owned nested state usage declaration under the state def, got:\n{output}"
        );
    }

    #[test]
    fn state_def_specializing_another_state_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tstate def Base;\n\
             \tstate def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn state_usage_typed_by_a_state_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tstate def Base;\n\
             \tstate s : Base;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::s\"))) (kind state)"),
            "expected a state usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::s\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected s's typing reference to Base to resolve, got:\n{output}"
        );
    }

    /// A `transition t first s1 then s2;` body element's `source`/`target` operands now resolve
    /// to their sibling state declarations (this task picks up the full `transition` construct
    /// explicitly deferred by `4762b875`), so it no longer surfaces as an explicit unsupported
    /// state-definition-member diagnostic. See `TransitionEffect`/`TransitionAccept`-specific
    /// unsupported sub-piece coverage in `lib.rs`'s `transition_*` tests for what remains
    /// deliberately out of scope (typed `accept` payload declarations, time triggers, and the
    /// richer `Accept`/`Send`/`Assign` effect shapes).
    #[test]
    fn transition_inside_a_state_def_resolves_source_and_target() {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \tstate def SD {\n\
                 \t\tstate s1;\n\
                 \t\tstate s2;\n\
                 \t\ttransition t first s1 then s2;\n\
                 \t}\n\
                 }\n"
                .to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        assert!(
            !output.contains("unsupported_state_definition_member"),
            "did not expect an unsupported state-definition-member diagnostic for a fully \
             resolvable transition, got:\n{output}"
        );
        let mut semantic = String::new();
        published
            .debug()
            .write_semantic_sexpr(&mut semantic)
            .unwrap();
        assert!(
            semantic.contains("(kind transitionSource)")
                && semantic.contains("(kind transitionTarget)"),
            "expected transitionSource/transitionTarget relationship kinds, got:\n{semantic}"
        );
    }

    #[test]
    fn satisfy_inside_a_part_usage_resolves_source_and_target() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def R;\n\
             \tpart def P;\n\
             \trequirement r : R;\n\
             \tpart p : P {\n\
             \t\tsatisfy r by p;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind satisfySource)") && output.contains("(kind satisfyTarget)"),
            "expected satisfySource/satisfyTarget relationship kinds, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind satisfy) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::p::satisfy\""
            ) || output.contains("(kind satisfy)"),
            "expected an owned satisfy declaration, got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)"),
            "expected both satisfy operands to resolve, got:\n{output}"
        );
    }

    #[test]
    fn satisfy_with_an_unresolvable_requirement_stays_explicitly_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def P;\n\
             \tpart p : P {\n\
             \t\tsatisfy missingReq by p;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind satisfySource)")
                && output.contains("(authored-target \"missingReq\")")
                && output.contains("(status unresolved)"),
            "expected the unresolvable satisfy source to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
        assert!(
            output.contains("(kind satisfyTarget)")
                && output.contains("(authored-target \"p\")\n      (outcome (status resolved)"),
            "expected the satisfy target to still resolve independently, got:\n{output}"
        );
    }

    #[test]
    fn satisfy_with_an_unresolvable_satisfying_element_stays_explicitly_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def R;\n\
             \trequirement r : R;\n\
             \tpart def P;\n\
             \tpart p : P {\n\
             \t\tsatisfy r by missingElement;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind satisfyTarget)")
                && output.contains("(authored-target \"missingElement\")")
                && output.contains("(status unresolved)"),
            "expected the unresolvable satisfying element to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
        assert!(
            output.contains("(kind satisfySource)")
                && output.contains("(authored-target \"r\")\n      (outcome (status resolved)"),
            "expected the satisfy source to still resolve independently, got:\n{output}"
        );
    }

    #[test]
    fn allocate_statement_inside_a_part_usage_resolves_source_and_target() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def A;\n\
             \tpart def B;\n\
             \tpart a : A;\n\
             \tpart b : B {\n\
             \t\tallocate a to b;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind allocateSource)") && output.contains("(kind allocateTarget)"),
            "expected allocateSource/allocateTarget relationship kinds, got:\n{output}"
        );
        assert!(
            output.contains("(kind allocate)"),
            "expected an owned allocate declaration, got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)"),
            "expected both allocate operands to resolve, got:\n{output}"
        );
    }

    #[test]
    fn allocate_statement_with_an_unresolvable_target_stays_explicitly_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def A;\n\
             \tpart a : A;\n\
             \tpart b : A {\n\
             \t\tallocate a to missingTarget;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind allocateTarget)")
                && output.contains("(authored-target \"missingTarget\")")
                && output.contains("(status unresolved)"),
            "expected the unresolvable allocate target to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
        assert!(
            output.contains("(kind allocateSource)")
                && output.contains("(authored-target \"a\")\n      (outcome (status resolved)"),
            "expected the allocate source to still resolve independently, got:\n{output}"
        );
    }

    #[test]
    fn bind_statement_inside_a_part_usage_resolves_source_and_target() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def A;\n\
             \tpart def B;\n\
             \tpart a : A;\n\
             \tpart b : B {\n\
             \t\tbind a = b;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind bindSource)") && output.contains("(kind bindTarget)"),
            "expected bindSource/bindTarget relationship kinds, got:\n{output}"
        );
        assert!(
            output.contains("(kind bind)"),
            "expected an owned bind declaration, got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)"),
            "expected both bind operands to resolve, got:\n{output}"
        );
    }

    #[test]
    fn bind_statement_with_an_unresolvable_target_stays_explicitly_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def A;\n\
             \tpart a : A;\n\
             \tpart b : A {\n\
             \t\tbind a = missingTarget;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind bindTarget)")
                && output.contains("(authored-target \"missingTarget\")")
                && output.contains("(status unresolved)"),
            "expected the unresolvable bind target to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
        assert!(
            output.contains("(kind bindSource)")
                && output.contains("(authored-target \"a\")\n      (outcome (status resolved)"),
            "expected the bind source to still resolve independently, got:\n{output}"
        );
    }

    #[test]
    fn bind_statement_with_dotted_feature_chain_operands_resolves_both_ends() {
        // Regression test: `lower_satisfy_operand` (shared by `lower_bind`, `lower_satisfy`,
        // `lower_allocate`, etc.) only matched `Expression::MemberAccess` in its dotted-chain arm,
        // not `Expression::FeatureChainRef` -- the shape the parser actually produces for a
        // dotted path like `f.a`/`a.g` (see `flatten_member_access_chain`, which has always
        // handled both). `lower_connector_end` (used by `connect`) already matched both variants,
        // so `connect f.a to a.g;` resolved while the very next line, `bind f.a = a.g;`, fell
        // through to an unsupported diagnostic on both operands -- exactly the shape from
        // `test/snapshots/sysml/examples/feature_path_test.md`. Fixed by adding
        // `Expression::FeatureChainRef(_)` to `lower_satisfy_operand`'s dotted-chain match arm,
        // mirroring `lower_connector_end`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def F { part a : A; }\n\
             \tpart def A { part g : F; }\n\
             \tpart def B {\n\
             \t\tpart f : F;\n\
             \t\tpart a : A;\n\
             \t}\n\
             \tpart b : B {\n\
             \t\tbind f.a = a.g;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind bind)"),
            "expected an owned bind declaration, got:\n{output}"
        );
        assert!(
            output.matches("(kind memberAccessOperand)").count() >= 2,
            "expected both dotted bind operands to lower as memberAccessOperand references, \
             got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)") && !output.contains("unsupported"),
            "expected both dotted bind operands (f.a, a.g) to resolve, got:\n{output}"
        );
    }

    #[test]
    fn connect_statement_with_dotted_feature_chain_operands_resolves_both_ends() {
        // Companion regression to the `bind` test above: `connect a.b to c.d;` with dotted
        // endpoints already resolved correctly (via `lower_connector_end`, which has always
        // matched both `Expression::MemberAccess` and `Expression::FeatureChainRef`) -- this
        // pins that behavior down explicitly so a future refactor of the shared
        // `flatten_member_access_chain`/`push_member_access_reference` path can't silently
        // regress the `connect` side while fixing/touching the `bind` side.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def A { part d : A; }\n\
             \tpart def B {\n\
             \t\tpart a : A;\n\
             \t\tpart c : A;\n\
             \t}\n\
             \tpart b : B {\n\
             \t\tconnect a.d to c.d;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.matches("(kind memberAccessOperand)").count() >= 2,
            "expected both dotted connect endpoints to lower as memberAccessOperand \
             references, got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)") && !output.contains("unsupported"),
            "expected both dotted connect endpoints (a.d, c.d) to resolve, got:\n{output}"
        );
    }

    #[test]
    fn variation_part_resolves_both_variant_members_to_sibling_declarations() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Transmission;\n\
             \tpart manualTransmission;\n\
             \tpart automaticTransmission;\n\
             \tpart vehicle {\n\
             \t\tvariation part transmission : Transmission {\n\
             \t\t\tvariant manualTransmission;\n\
             \t\t\tvariant automaticTransmission;\n\
             \t\t}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.matches("(kind variant)").count() >= 2,
            "expected two variant relationship kinds, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind variant) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::vehicle::transmission\""
            ),
            "expected both variant references to be sourced at the variation declaration \
             itself (no anonymous nested-declaration shift), got:\n{output}"
        );
        assert!(
            output.contains("(authored-target \"manualTransmission\")")
                && output.contains("(authored-target \"automaticTransmission\")"),
            "expected both variant targets to be authored, got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)"),
            "expected both variant members to resolve to their sibling declarations, \
             got:\n{output}"
        );
        assert!(
            output.contains("(variation true)"),
            "expected the variation part's own typing reference to carry the variation flag, \
             got:\n{output}"
        );
    }

    #[test]
    fn variant_with_an_unresolvable_target_stays_explicitly_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Transmission;\n\
             \tpart manualTransmission;\n\
             \tpart vehicle {\n\
             \t\tvariation part transmission : Transmission {\n\
             \t\t\tvariant manualTransmission;\n\
             \t\t\tvariant missingVariant;\n\
             \t\t}\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind variant)")
                && output.contains("(authored-target \"missingVariant\")")
                && output.contains("(status unresolved)"),
            "expected the unresolvable variant target to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
        assert!(
            output.contains(
                "(authored-target \"manualTransmission\")\n      (outcome (status resolved)"
            ),
            "expected the resolvable variant to still resolve independently, got:\n{output}"
        );
    }

    #[test]
    fn use_case_include_resolves_its_target_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tuse case def UsedUseCase;\n\
             \tuse case def MainUseCase {\n\
             \t\tinclude UsedUseCase;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind includeUseCase)"),
            "expected an includeUseCase relationship kind, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind includeUseCase) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MainUseCase\""
            ),
            "expected the includeUseCase reference to be sourced at the enclosing use case \
             declaration (no anonymous nested-declaration scope shift), got:\n{output}"
        );
        assert!(
            output.contains("(authored-target \"UsedUseCase\")"),
            "expected the include target to be authored, got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)"),
            "expected the include target to resolve, got:\n{output}"
        );
    }

    #[test]
    fn use_case_include_with_an_unresolvable_target_stays_explicitly_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tuse case def MainUseCase {\n\
             \t\tinclude missingUseCase;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind includeUseCase)")
                && output.contains("(authored-target \"missingUseCase\")")
                && output.contains("(status unresolved)"),
            "expected the unresolvable include target to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
    }

    #[test]
    fn ref_decl_resolves_its_typing_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Part;\n\
             \tpart def Holder {\n\
             \t\tref self: Part;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind ref)"),
            "expected a `ref` declaration, got:\n{output}"
        );
        assert!(
            output.contains("(kind featureTyping)"),
            "expected a featureTyping relationship kind for the ref's `:` clause, got:\n{output}"
        );
        assert!(
            output.contains("(authored-target \"Part\")"),
            "expected the ref's typing target to be authored, got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)"),
            "expected the ref's typing target to resolve, got:\n{output}"
        );
    }

    #[test]
    fn ref_decl_resolves_its_redefines_reference() {
        // `part def`/`part` usage bodies parse `ref` through the narrower `part_ref_usage`
        // production (`ast::part::usage::part_ref_usage`), which does not capture a trailing
        // `:>>` redefines target at all. `connection def`/`interface def` bodies instead parse
        // `ref` through `connector::ref_decl`, which captures the full `:`/`:>>`/`:>` clause set
        // -- use a `connection def` body here so the redefines clause actually round-trips.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Item {\n\
             \t\tref self: Item;\n\
             \t}\n\
             \tconnection def C {\n\
             \t\tref self: Item :>> Item::self;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind redefinition)"),
            "expected a redefinition relationship kind for the ref's `:>>` clause, got:\n{output}"
        );
        assert!(
            output.contains("(authored-target \"Item::self\")"),
            "expected the ref's redefines target to be authored, got:\n{output}"
        );
    }

    #[test]
    fn ref_decl_resolves_combined_redefines_and_subsets_references_independently() {
        // GH-51: a single `ref` can carry both an explicit `:>>` redefines clause and a `:>`
        // subsets clause at once, e.g. `ref requirement originalRequirement[1] :>>
        // originalRequirements :> participant { ... }` (Systems Library `Domain Libraries/
        // Requirement Derivation/DerivationConnections.sysml`). `lower_ref_decl` already checks
        // `node.value.redefines` and `node.value.subsets` as two independent `if let`s (not an
        // `if`/`else if`), so both references are expected to resolve independently -- this test
        // locks that in.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def Req {\n\
             \t\trequirement participant;\n\
             \t\trequirement original;\n\
             \t}\n\
             \tconnection def C :> Req {\n\
             \t\tref requirement r :>> original :> participant;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind redefinition)"),
            "expected a redefinition relationship kind for the ref's `:>>` clause, got:\n{output}"
        );
        assert!(
            output.contains("(kind subsetting)"),
            "expected a subsetting relationship kind for the ref's `:>` clause, got:\n{output}"
        );
        assert!(
            output.contains("(authored-target \"original\")"),
            "expected the ref's redefines target to be authored, got:\n{output}"
        );
        assert!(
            output.contains("(authored-target \"participant\")"),
            "expected the ref's subsets target to be authored, got:\n{output}"
        );
    }

    #[test]
    fn ref_decl_with_an_unresolvable_typing_target_stays_explicitly_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Holder {\n\
             \t\tref self: MissingType;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind featureTyping)")
                && output.contains("(authored-target \"MissingType\")")
                && output.contains("(status unresolved)"),
            "expected the unresolvable ref typing target to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
    }

    #[test]
    fn view_body_satisfy_resolves_its_viewpoint_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tviewpoint def V;\n\
             \tview def VD;\n\
             \tview v : VD {\n\
             \t\tsatisfy V;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind satisfyViewpoint)"),
            "expected a satisfyViewpoint relationship kind, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind satisfyViewpoint) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::v\""
            ),
            "expected the satisfyViewpoint reference to be sourced at the view declaration, \
             got:\n{output}"
        );
        assert!(
            !output.contains("(status unresolved)"),
            "expected the viewpoint reference to resolve, got:\n{output}"
        );
    }

    #[test]
    fn view_body_satisfy_with_an_unresolvable_viewpoint_stays_explicitly_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tview def VD;\n\
             \tview v : VD {\n\
             \t\tsatisfy MissingViewpoint;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind satisfyViewpoint)")
                && output.contains("(authored-target \"MissingViewpoint\")")
                && output.contains("(status unresolved)"),
            "expected the unresolvable viewpoint reference to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
    }

    #[test]
    fn viewpoint_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tviewpoint def V;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::V\"))) (kind viewpoint-def)"),
            "expected a viewpoint-def declaration, got:\n{output}"
        );
    }

    #[test]
    fn viewpoint_def_specializing_another_viewpoint_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tviewpoint def Base;\n\
             \tviewpoint def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn rendering_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trendering def R;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::R\"))) (kind rendering-def)"),
            "expected a rendering-def declaration, got:\n{output}"
        );
    }

    #[test]
    fn rendering_def_specializing_another_rendering_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trendering def Base;\n\
             \trendering def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn allocation_def_lowers_to_a_declaration_with_connector_end_references() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpart def Logical;\n\
             \tpart def Physical;\n\
             \tallocation def A {\n\
             \t\tend logical : Logical;\n\
             \t\tend physical : Physical;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::A\"))) (kind allocation-def)"),
            "expected an allocation-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::A::logical\"))) (kind connection)"),
            "expected an owned end declaration under the allocation def, got:\n{output}"
        );
    }

    #[test]
    fn allocation_def_specializing_another_allocation_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tallocation def Base;\n\
             \tallocation def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn flow_def_lowers_to_a_declaration_with_connector_end_references() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def SupplierPort;\n\
             \tport def ConsumerPort;\n\
             \tflow def F {\n\
             \t\tend supplierPort : SupplierPort;\n\
             \t\tend consumerPort : ConsumerPort;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::F\"))) (kind flow-def)"),
            "expected a flow-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::F::supplierPort\"))) (kind connection)"),
            "expected an owned end declaration under the flow def, got:\n{output}"
        );
    }

    #[test]
    fn flow_def_specializing_another_flow_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tflow def Base;\n\
             \tflow def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn value_assignment_tuple_resolves_every_element_reference() {
        // `Expression::Tuple` (`(a, b, c)`) reuses the Invocation-shaped reference-resolution
        // slice: no callee, but every element recurses back into `lower_constraint_expression`
        // exactly like an invocation argument, so all three feature references resolve.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute a : ScalarValues::Integer;\n\
             \tattribute b : ScalarValues::Integer;\n\
             \tattribute c : ScalarValues::Integer;\n\
             \tattribute tuple = (a, b, c);\n\
             }\n",
        );
        for (name, ordinal) in [("a", 0), ("b", 1), ("c", 2)] {
            assert!(
                output.contains(&format!(
                    "(kind expressionOperand) (ordinal {ordinal}))\n      (authored-target \
                     \"{name}\")\n      (outcome (status resolved) (target (node (document \
                     \"memory://test/enum.sysml\") (qualified-name \"Demo::{name}\")))))"
                )),
                "expected tuple element `{name}` to resolve as an expressionOperand reference, \
                 got:\n{output}"
            );
        }
    }

    #[test]
    fn value_assignment_tuple_with_unresolvable_element_leaves_only_that_element_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute a : ScalarValues::Integer;\n\
             \tattribute tuple = (a, missing);\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"a\")\n      \
                 (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::a\")))))"
            ),
            "expected resolvable tuple element `a` to resolve, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 1))\n      (authored-target \"missing\")\n      \
                 (outcome (status unresolved))"
            ),
            "expected undeclared tuple element `missing` to stay explicitly unresolved (not \
             fabricated), got:\n{output}"
        );
    }

    #[test]
    fn value_assignment_tuple_of_literals_evaluates_to_non_constant() {
        // A tuple never folds to a single scalar `EvaluatedValue` (see `EvalNode::Invocation`'s
        // doc comment, reused unchanged for `Expression::Tuple`): even an all-literal tuple
        // publishes `NonConstant`, matching the `Invocation`/`Constructor` precedent rather than
        // fabricating an unmodeled composite-value representation.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute tuple = (1, 2, 3);\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::tuple\"))) (state non-constant))"
            ),
            "expected an all-literal tuple to publish NonConstant rather than a fabricated \
             composite value, got:\n{output}"
        );
    }

    #[test]
    fn value_assignment_istype_resolves_operand_and_type_target() {
        // `Expression::TypeCheck` (`x istype T`) resolves the operand through the ordinary
        // ExpressionOperand recursion and the `T` target through the new TypeCheckTarget
        // reference, mirroring `AcceptPayloadType`/`FilterMetadataTest`'s Type-domain lookup.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute a : ScalarValues::Integer;\n\
             \tclass T;\n\
             \tattribute check = a istype T;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"a\")\n      \
                 (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::a\")))))"
            ),
            "expected `a` to resolve as an expressionOperand reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typeCheckTarget) (ordinal 0))\n      (authored-target \"T\")\n      \
                 (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::T\")))))"
            ),
            "expected `T` to resolve as a typeCheckTarget reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::check\"))) (state non-constant))"
            ),
            "expected `a istype T` to publish NonConstant (no runtime type info available), \
             got:\n{output}"
        );
    }

    #[test]
    fn value_assignment_hastype_resolves_operand_and_type_target() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute a : ScalarValues::Integer;\n\
             \tclass T;\n\
             \tattribute check = a hastype T;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"a\")\n      \
                 (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::a\")))))"
            ),
            "expected `a` to resolve as an expressionOperand reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typeCheckTarget) (ordinal 0))\n      (authored-target \"T\")\n      \
                 (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::T\")))))"
            ),
            "expected `T` to resolve as a typeCheckTarget reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::check\"))) (state non-constant))"
            ),
            "expected `a hastype T` to publish NonConstant (no runtime type info available), \
             got:\n{output}"
        );
    }

    #[test]
    fn value_assignment_istype_with_unresolvable_operand_and_type_stays_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute check = missingOperand istype MissingType;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \
                 \"missingOperand\")\n      (outcome (status unresolved))"
            ),
            "expected undeclared operand `missingOperand` to stay explicitly unresolved, \
             got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typeCheckTarget) (ordinal 0))\n      (authored-target \"MissingType\")\n      \
                 (outcome (status unresolved))"
            ),
            "expected undeclared type target `MissingType` to stay explicitly unresolved, \
             got:\n{output}"
        );
    }

    #[test]
    fn value_assignment_meta_cast_resolves_base_and_metaclass_target() {
        // `Expression::MetaCast` (`Base meta Ns::Metaclass`) resolves the base operand through
        // the ordinary ExpressionOperand recursion and the qualified `Ns::Metaclass` target
        // through the new MetaCastTarget reference, mirroring `TypeCheckTarget`'s Type-domain
        // lookup and supporting a multi-segment qualified reference exactly like other
        // Type-domain targets (e.g. `KerML::Classifier`).
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpackage Meta {\n\
             \t\tclass Classifier;\n\
             \t}\n\
             \tattribute a : ScalarValues::Integer;\n\
             \tattribute check = a meta Meta::Classifier;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \"a\")\n      \
                 (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::a\")))))"
            ),
            "expected `a` to resolve as an expressionOperand reference, got:\n{output}"
        );
        assert!(
            output.contains("(kind metaCastTarget)")
                && output.contains(
                    "(outcome (status resolved) (target (node (document \
                     \"memory://test/enum.sysml\") (qualified-name \"Demo::Meta::Classifier\")))))"
                ),
            "expected `Meta::Classifier` to resolve as a metaCastTarget reference, got:\n{output}"
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::check\"))) (state non-constant))"
            ),
            "expected `a meta Meta::Classifier` to publish NonConstant (denotes a metaclass \
             relationship, not a computable scalar value), got:\n{output}"
        );
    }

    #[test]
    fn default_reference_usage_meta_cast_value_lowers_and_resolves() {
        // Keyword-less `<name> = <expr>;` binding (`ast::structure::DefaultReferenceUsage`),
        // e.g. `baseType = Atom meta KerML::Classifier;` inside a KerML `metaclass` body
        // (`test/snapshots/kerml/a_2_atoms.md`). The declaration itself, and its `=` value's
        // `MetaCast` base/metaclass references, should both resolve, mirroring
        // `value_assignment_meta_cast_resolves_base_and_metaclass_target`.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tpackage KerML {\n\
             \t\tclass Classifier;\n\
             \t}\n\
             \tclass Atom;\n\
             \tmetaclass AtomMetadata {\n\
             \t\tbaseType = Atom meta KerML::Classifier;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind default-reference)")
                && output.contains("(qualified-name \"Demo::AtomMetadata::baseType\")"),
            "expected a DefaultReferenceUsage declaration for baseType, got:\n{output}"
        );
        assert!(
            output.contains("(kind expressionOperand) (ordinal 0))")
                && output.contains(
                    "(outcome (status resolved) (target (node (document \
                     \"memory://test/enum.sysml\") (qualified-name \"Demo::Atom\")))))"
                ),
            "expected `Atom` to resolve as the meta cast's base operand, got:\n{output}"
        );
        assert!(
            output.contains("(kind metaCastTarget)")
                && output.contains(
                    "(outcome (status resolved) (target (node (document \
                     \"memory://test/enum.sysml\") (qualified-name \"Demo::KerML::Classifier\")))))"
                ),
            "expected `KerML::Classifier` to resolve as the meta cast's metaclass target, got:\n{output}"
        );
    }

    #[test]
    fn default_reference_usage_typed_binding_resolves_its_typing() {
        // A typed keyword-less binding, `<name> : <Type> = <expr>;`, still routes through
        // `DefaultReferenceUsage` (no leading keyword) and should resolve its `FeatureTyping`
        // reference in addition to the declaration and value.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute def MassValue;\n\
             \tpart def Vehicle {\n\
             \t\tmass : MassValue = 10;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(kind default-reference)")
                && output.contains("(qualified-name \"Demo::Vehicle::mass\")"),
            "expected a DefaultReferenceUsage declaration for mass, got:\n{output}"
        );
        assert!(
            output.contains("(kind featureTyping)")
                && output.contains(
                    "(outcome (status resolved) (target (node (document \
                     \"memory://test/enum.sysml\") (qualified-name \"Demo::MassValue\")))))"
                ),
            "expected `mass`'s typing to resolve to Demo::MassValue, got:\n{output}"
        );
    }

    #[test]
    fn value_assignment_meta_cast_with_unresolvable_base_and_metaclass_stays_unresolved() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tattribute check = missingOperand meta Missing::Metaclass;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind expressionOperand) (ordinal 0))\n      (authored-target \
                 \"missingOperand\")\n      (outcome (status unresolved))"
            ),
            "expected undeclared base `missingOperand` to stay explicitly unresolved, \
             got:\n{output}"
        );
        assert!(
            output.contains("(kind metaCastTarget)") && output.contains("(status unresolved)"),
            "expected undeclared metaclass target `Missing::Metaclass` to stay explicitly \
             unresolved, got:\n{output}"
        );
    }

    #[test]
    fn foreign_typed_ids_are_rejected_before_mutation() {
        let mut builder = SemanticModelBuilder::default();
        let invalid_document = DocumentId(0);
        let name = builder.intern_name("Vehicle").unwrap();
        let error = builder
            .push_declaration(invalid_document, None, Some(name))
            .unwrap_err();
        assert_eq!(error, ConstructionError::InvalidIdentity);
        assert!(builder.declarations.is_empty());
    }
}
