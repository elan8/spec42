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
};

use hashbrown::HashTable;
use sysml_v2_parser_next::{
    ast::{
        ActionDef, ActionDefBody, ActionDefBodyElement, ActionUsage as ParserActionUsage,
        ActionUsageBody, ActionUsageBodyElement, AliasDef, Allocate, AllocationDef,
        AnalysisCaseDef, AnalysisCaseUsage as ParserAnalysisCaseUsage, AssertConstraintMember,
        AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, BinaryOperator, Bind,
        BindingConnectorUsage, CalcDef, CalcDefBody, CalcDefBodyElement,
        CalcUsage as ParserCalcUsage, CaseDef, CaseUsage as ParserCaseUsage, ClassDef,
        ConcernUsage as ParserConcernUsage, ConnectStmt, ConnectionDef, ConnectionDefBody,
        ConnectionDefBodyElement, ConnectionEnd, ConnectionUsageMember as ParserConnectionUsage,
        ConstraintDef, ConstraintDefBody, ConstraintDefBodyElement,
        ConstraintUsage as ParserConstraintUsage, DefinitionBody, DefinitionBodyElement,
        DefinitionPrefix, DoAction, EndDecl, EndIdentity, EntryAction, EnumDef, EnumerationBody,
        EnumerationUsage as ParserEnumerationUsage, ExitAction, Expression, FeatureValue,
        FirstStmt, FlowDef, Import, ImportShape, InOut, InOutDecl, IncludeUseCase, InterfaceDef,
        InterfaceDefBody, InterfaceDefBodyElement, InterfaceUsage as ParserInterfaceUsage,
        InterfaceUsageBodyElement, ItemDef, ItemUsage as ParserItemUsage, LibraryPackage,
        Membership, MembershipKind as ParserMembershipKind, MetadataAnnotation, MetadataDef,
        MetadataUsage as ParserMetadataUsage, NamespaceDecl, Node, OccurrenceBodyElement,
        OccurrenceDef, OccurrenceUsage as ParserOccurrenceUsage, OccurrenceUsageBody, Package,
        PackageBody, PackageBodyElement, PartDef, PartDefBody, PartDefBodyElement, PartUsage,
        PartUsageBody, PartUsageBodyElement, Perform as ParserPerform, PerformBody,
        PerformBodyElement, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement,
        PortUsage as ParserPortUsage, QualifiedIdentification, QualifiedReferenceId, RenderingDef,
        RenderingDefBody, RenderingDefBodyElement, RequirementDef, RequirementDefBody,
        RequirementDefBodyElement, RequirementUsage as ParserRequirementUsage, ReturnDecl,
        RootElement, Satisfy, SatisfyViewMember, Span, StateDef, StateDefBody, StateDefBodyElement,
        StateUsage as ParserStateUsage, SubjectDecl, SubsettingKind, SubsettingRelationship,
        ThenStmt, Transition, TransitionAccept, TransitionEffect, UseCaseDef, UseCaseDefBody,
        UseCaseDefBodyElement, VariantUsage, VerificationCaseDef, ViewBody, ViewBodyElement,
        ViewDef, ViewDefBody, ViewDefBodyElement, ViewUsage as ParserViewUsage, ViewpointDef,
        Visibility as ParserVisibility,
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
enum DeclarationKind {
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
    /// `DeclarationKind::AnalysisCaseUsage` (UPSTREAM_PARSER_GAPS.md #5 was resolved upstream in
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
    /// structurally analogous `ConnectionUsageMember` (see UPSTREAM_PARSER_GAPS.md #6). Interface-
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
    /// reaches the typed AST (see UPSTREAM_PARSER_GAPS.md #8; confirmed real usage in
    /// `test/snapshots/sysml/validation/11b_safety_and_security_feature_views.md`'s
    /// `view vehicleMandatorySafetyFeatureView :> vehicleSafetyFeatureView { ... }`).
    ViewDefinition,
    /// `case def` (BNF CaseDefinition): a type whose owned members are attribute usages and
    /// nested case structure, mirroring `AnalysisCaseDefinition` lowering (shares the same
    /// `UseCaseDefBody`/`UseCaseDefBodyElement` shape). Case-specific semantics (subject binding,
    /// objective, first-succession/return structure) are out of scope here; only ownership,
    /// specialization, and owned-member structure are lowered. `case` usage lowering follows
    /// below, in `DeclarationKind::CaseUsage` (UPSTREAM_PARSER_GAPS.md #5 was resolved upstream
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
    /// usage lowering is deferred (see UPSTREAM_PARSER_GAPS.md #5: `VerificationCaseUsage`
    /// silently drops parsed `:>`/`:>>` clauses, unlike `VerificationCaseDef`, which has full
    /// field parity).
    VerificationCaseDefinition,
    /// `use case def` (BNF UseCaseDefinition): a type whose owned members are attribute usages
    /// and nested case structure, mirroring `CaseDefinition`/`AnalysisCaseDefinition` lowering.
    /// Use-case-specific semantics (actor/include structure) are out of scope here; only
    /// ownership, specialization, and owned-member structure are lowered. `use case` usage
    /// lowering is deferred (see UPSTREAM_PARSER_GAPS.md #5: `UseCaseUsage` silently drops parsed
    /// `:>`/`:>>` clauses, unlike `UseCaseDef`, which has full field parity).
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
    /// alongside the other unmodeled `RequirementDefBody` members. `viewpoint` usage lowering is
    /// deferred: `ast::ViewpointUsage` has only `type_name` (bare `QualifiedReferenceId`), no
    /// `subsets`/`redefines` fields at all -- the same gap class as `ViewUsage`
    /// (UPSTREAM_PARSER_GAPS.md #8).
    ViewpointDefinition,
    /// `rendering def` (BNF RenderingDefinition, Clause 8.2.2.26): a type whose owned members
    /// share `RenderingDefBody`/`RenderingDefBodyElement` with `ViewDefBody`/`ViewDefBodyElement`
    /// (same shape: `Filter`/`ViewRendering`/`Other`/`Doc`/`Error`), mirroring `lower_view_def`
    /// lowering: ownership, membership, an optional `:>` specialization relationship
    /// participating in the shared `DeclarationDomain::Type` fixed point. Verified
    /// `RenderingDef`'s `specializes: Option<Node<TypingRelationship>>` field carries full parity
    /// with `ViewDef`/`ConnectionDef`. Render-specific body semantics (`filter`/`render` members)
    /// are out of scope for this slice and fall through to a dedicated
    /// `RenderingDefinitionMember` diagnostic. `rendering` usage lowering is deferred:
    /// `ast::RenderingUsage` has only `type_name` (bare `QualifiedReferenceId`), no
    /// `subsets`/`redefines` fields at all -- the same gap class as `ViewUsage`.
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
    /// (UPSTREAM_PARSER_GAPS.md #8): `ViewUsage` previously had no `subsets` field to lower this
    /// relationship from. View-specific body members remain out of scope, sharing
    /// `UnsupportedFamily::ViewDefinitionMember` with the `def` form's body walker.
    ViewUsage,
    /// A package/definition/usage-level `interface` feature member (BNF InterfaceUsage),
    /// mirroring `lower_interface_def`: ownership, membership, an optional `:` typing target,
    /// `subsets`/`redefines` subsetting relationships, and connector-end structure (`connect`/
    /// `end`) via the same `ReferenceKind::ConnectorEnd` machinery `interface def` uses. Resolved
    /// upstream in `0757de13` (UPSTREAM_PARSER_GAPS.md #6): all three `InterfaceUsage` variants
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
    /// (UPSTREAM_PARSER_GAPS.md #4): `ConstraintUsage` previously had no `subsets`/`redefines`
    /// fields at all.
    ConstraintUsage,
    /// `concern def` (BNF ConcernDefinition, Clause 8.2.2.11): a type whose owned members share
    /// `RequirementDefBody`/`RequirementDefBodyElement` with `RequirementDefinition`, mirroring
    /// `lower_viewpoint_def`. The parser models both `concern def` and `concern` under a single
    /// `ast::requirement::ConcernUsage` struct discriminated by `is_definition`, rather than a
    /// distinct `ConcernDef` type -- see that struct's doc comment. Genuinely new: previously
    /// blocked entirely (UPSTREAM_PARSER_GAPS.md #9: no `specializes`/`subsets`/`redefines` field
    /// at all). Stakeholder/subject-binding semantics are out of scope, sharing
    /// `UnsupportedFamily::RequirementDefinitionMember` with `requirement def`/`viewpoint def`.
    ConcernDefinition,
    /// A package/definition/usage-level `concern` feature member (BNF ConcernUsage), mirroring
    /// `lower_requirement_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (UPSTREAM_PARSER_GAPS.md #9).
    ConcernUsage,
    /// `calc def` (BNF CalculationDefinition, Clause 8.2.2.14): a type whose owned members
    /// participate in the shared Subclassification/FeatureTyping `DeclarationDomain::Type` fixed
    /// point, mirroring `lower_view_def`/`lower_action_def`. Resolved upstream in `0757de13`
    /// (UPSTREAM_PARSER_GAPS.md #3): `CalcDef` previously dropped its parsed `:>` specialization
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
    /// `lower_item_def`. Resolved upstream in `0757de13` (UPSTREAM_PARSER_GAPS.md #2): `ClassDef`
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
    /// A directed `in`/`out`/`inout` parameter declaration (BNF `InOutDecl`, `ast::InOutDecl`)
    /// found in a `calc def`/`constraint def`/`action def` body, e.g. `in partMasses :
    /// MassValue[0..*];`. Mirrors `ItemUsage`/`MetadataUsage` lowering: ownership, membership,
    /// and (when present) a `FeatureTyping` reference to the declared type. The `in`/`out`/
    /// `inout` direction itself is not modeled as a distinct declaration kind's own field --
    /// it is carried as an explicit `RelationshipFlags::direction` fact on the pushed
    /// `FeatureTyping` reference, mirroring how `PortUsage`'s conjugation polarity rides the
    /// `conjugated` flag on the same reference rather than becoming a new relationship kind.
    /// When the parameter has no type (`type_name` is `None`, e.g. a bare `in :>> target = ...`
    /// redefinition form), no `FeatureTyping` reference is pushed and the direction fact is not
    /// recorded -- only the declaration/membership shell is lowered for that shape. Multiplicity
    /// (`[0..*]`) is not modeled anywhere else in this codebase yet (attribute/part usages with
    /// array types don't carry a multiplicity fact either), so it is left unrepresented here too.
    ParameterUsage,
    /// A `subject` declaration (BNF `SubjectDecl`, `ast::SubjectDecl`) found in a requirement/
    /// concern/case-family def or usage body, e.g. `subject vehicle : Vehicle;` inside
    /// `requirement vehicleSpecification`. Structurally a plain typed feature declaration --
    /// name plus an optional `FeatureTyping` reference to the declared type -- mirroring
    /// `lower_parameter_declaration`'s shape but without a direction fact. Per
    /// RESOLUTION_LAYER_DESIGN.md §5.4, `Subject` is a derived case-level relationship projected
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
enum ReferenceKind {
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
    /// `AliasBinding` to match RESOLUTION_LAYER_DESIGN.md's "alias binding" vocabulary (section
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
}

/// The computed or explicit outcome of evaluating one supported constraint/calc expression
/// (slice 2 of the constraint/calc expression fact family; slice 1, `4ca42166`, only resolved
/// operand references and never evaluated anything). Only expressions within slice 1's supported
/// syntactic shapes (literal leaves, a comparison `BinaryOp` of two literals, `Parenthesized`
/// wrapping a supported shape) reach this pass at all -- a shape slice 1 leaves unsupported
/// publishes no evaluation fact, per `classify_constraint_expression`/`classify_calc_expression`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum EvaluatedValue {
    /// A genuinely computed constant `bool` result: a literal boolean leaf, or a comparison of
    /// two literal operands.
    Boolean(bool),
    /// A genuinely computed constant integer result: a literal integer leaf.
    Integer(i64),
    /// A genuinely computed constant real result: a literal real leaf.
    Real(f64),
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
    /// Slice 4: an arithmetic `BinaryOp` (`Add`/`Sub`/`Mul`/`Div`/`Mod`) found in a calc-body
    /// expression. Constraint bodies never build this variant -- `classify_constraint_node` has no
    /// arithmetic arm, only `classify_calc_node` does -- keeping constraint evaluation strictly
    /// comparison-only, unchanged from slices 1-3.
    Arithmetic(BinaryOperator, Box<EvalNode>, Box<EvalNode>),
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
    Literal(EvaluatedValue),
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
        (EvaluatedValue::Boolean(left), EvaluatedValue::Boolean(right)) => match op {
            BinaryOperator::Eq => Some(left == right),
            BinaryOperator::Ne => Some(left != right),
            _ => None,
        },
        (left, right) => match (as_f64(left), as_f64(right)) {
            (Some(left), Some(right)) => Some(match op {
                BinaryOperator::Eq => left == right,
                BinaryOperator::Ne => left != right,
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
                _ => EvaluatedValue::NonConstant,
            }
        }
    }
}

fn literal_expression_value(node: &Expression) -> Option<EvaluatedValue> {
    match node {
        Expression::LiteralBoolean(value) => Some(EvaluatedValue::Boolean(*value)),
        Expression::LiteralInteger(value) => Some(EvaluatedValue::Integer(*value)),
        Expression::LiteralReal(text) => text.parse::<f64>().ok().map(EvaluatedValue::Real),
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
        | Expression::LiteralBoolean(_) => literal_expression_value(node).map(EvalNode::Literal),
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
        | Expression::LiteralBoolean(_) => literal_expression_value(node).map(EvalNode::Literal),
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
        _ => None,
    }
}

/// Whether an `EvalNode` tree contains no `Operand` leaf at all (i.e. is a pure literal, needing
/// no resolved state whatsoever to fold).
fn eval_node_is_pure_literal(node: &EvalNode) -> bool {
    match node {
        EvalNode::Literal(_) => true,
        EvalNode::Operand(_) => false,
        EvalNode::Comparison(_, left, right) | EvalNode::Arithmetic(_, left, right) => {
            eval_node_is_pure_literal(left) && eval_node_is_pure_literal(right)
        }
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
        EvalNode::Literal(value) => Some(*value),
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
    }
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
            ExpressionEvalShape::Literal(value)
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
            ExpressionEvalShape::Literal(value)
        }
        Some(tree) => ExpressionEvalShape::HasOperand(tree),
    }
}

/// Whether a `BinaryOperator` is one of the six boolean comparison operators
/// (`lower_constraint_expression`'s supported `BinaryOp` shape): `==`, `!=`, `<`, `<=`, `>`, `>=`.
/// KerML's strict-identity `===`/`!==` (`StrictEq`/`StrictNe`) are deliberately excluded from this
/// narrow slice.
fn is_comparison_operator(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Eq
            | BinaryOperator::Ne
            | BinaryOperator::Lt
            | BinaryOperator::Le
            | BinaryOperator::Gt
            | BinaryOperator::Ge
    )
}

/// Whether a `BinaryOperator` is one of the two boolean-combination operators a `filter <expr>;`
/// statement's condition supports (`lower_filter_expression`'s `BinaryOp` shape): `and`, `or`.
/// `xor`/`implies` are deliberately excluded from this narrow slice, mirroring
/// `is_comparison_operator`'s exclusion of KerML strict-identity comparison.
fn is_logical_operator(op: &BinaryOperator) -> bool {
    matches!(op, BinaryOperator::And | BinaryOperator::Or)
}

/// Whether a `BinaryOperator` is one of the five basic arithmetic operators
/// (`lower_calc_expression`'s slice-4 supported `BinaryOp` shape): `+`, `-`, `*`, `/`, `%`.
/// `Exp`/`Pow` (KerML's two distinct exponentiation-like variants) are deliberately excluded from
/// this slice: real-corpus `**` usage (the only exponentiation spelling found in the corpus, e.g.
/// `turbojet_stage_analysis.md`'s `(p_2 / p_1)**((gamma - 1) / gamma)`,
/// `sys_ml_v2_spec_annex_a_simple_vehicle_model.md`'s `tpd_avg **(-3)`) is consistently mixed with
/// unary negation, nested division, and fractional/negative real exponents in the same expression
/// -- shapes well outside this slice's scope regardless of whether `Exp`/`Pow` themselves were
/// supported, so including just the operator would not make any additional real-corpus expression
/// foldable. `is_arithmetic_operator` excludes them; `classify_calc_node`/`lower_calc_expression`
/// therefore fall through to the existing `Unsupported`/`unsupported_calc_definition_member` path
/// for any `BinaryOp` using `Exp`/`Pow`, exactly like every other still-unsupported shape -- never
/// a panic.
fn is_arithmetic_operator(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Add
            | BinaryOperator::Sub
            | BinaryOperator::Mul
            | BinaryOperator::Div
            | BinaryOperator::Mod
    )
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
    memberships: Box<[MembershipRecord]>,
    references: Box<[AuthoredReference]>,
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
    memberships: Vec<MembershipRecord>,
    references: Vec<AuthoredReference>,
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
        )
    }

    fn push_typed_declaration(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        kind: DeclarationKind,
        name: Option<SymbolId>,
        span: Span,
    ) -> Result<DeclarationId, ConstructionError> {
        if document.index() >= self.documents.len()
            || owner.is_some_and(|id| id.index() >= self.declarations.len())
            || name.is_some_and(|id| id.index() >= self.symbols.len())
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
        Ok(id)
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
            memberships: self.memberships.into_boxed_slice(),
            references: self.references.into_boxed_slice(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Package,
            name,
            node.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::LibraryPackage,
            name,
            node.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Namespace,
            name,
            node.span.clone(),
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
        if let PackageBody::Brace { elements } = body {
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
            PackageBodyElement::Doc(_)
            | PackageBodyElement::Comment(_)
            | PackageBodyElement::TextualRep(_) => {}
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
            PackageBodyElement::IndividualDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
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
            PackageBodyElement::ViewpointUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::RenderingUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ConnectionDef(node) => {
                self.lower_connection_def(document, owner, node)?
            }
            PackageBodyElement::OccurrenceDef(node) => {
                self.lower_occurrence_def(document, owner, node)?
            }
            PackageBodyElement::OccurrenceUsage(node) => {
                self.lower_occurrence_usage(document, owner, node)?
            }
            PackageBodyElement::Dependency(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
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
            PackageBodyElement::VerificationCaseUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::UseCaseUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
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
            PackageBodyElement::Ref(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::MetadataKeywordUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::Connect(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::DefaultReferenceUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
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
            PackageBodyElement::ExtendedDefinition(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartDefinition,
            name,
            node.span.clone(),
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
        if let PartDefBody::Brace { elements } = &node.value.body {
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
                    PartDefBodyElement::Doc(_) | PartDefBodyElement::Comment(_) => {}
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
                    PartDefBodyElement::Annotation(_)
                    | PartDefBodyElement::MetadataKeywordUsage(_)
                    | PartDefBodyElement::Dependency(_)
                    | PartDefBodyElement::Other(_)
                    | PartDefBodyElement::DefaultReferenceUsage(_)
                    | PartDefBodyElement::Ref(_)
                    | PartDefBodyElement::Connect(_)
                    | PartDefBodyElement::FlowUsage(_)
                    | PartDefBodyElement::ExhibitState(_)
                    | PartDefBodyElement::AllocationUsage(_)
                    | PartDefBodyElement::ViewpointUsage(_)
                    | PartDefBodyElement::RenderingUsage(_)
                    | PartDefBodyElement::UseCaseUsage(_)
                    | PartDefBodyElement::VerificationCaseUsage(_) => self.push_unsupported(
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartUsage,
            name,
            node.span.clone(),
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
        if let PartUsageBody::Brace { elements } = &node.value.body {
            for element in elements {
                match &element.value {
                    PartUsageBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PartUsageBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PartUsageBodyElement::PartUsage(part) => {
                        self.lower_part_usage(document, Some(declaration), part)?;
                    }
                    PartUsageBodyElement::Import(import) => {
                        self.lower_import(document, Some(declaration), import)?;
                    }
                    PartUsageBodyElement::EnumDef(enum_def) => {
                        self.lower_enum_def(document, Some(declaration), enum_def)?;
                    }
                    PartUsageBodyElement::EnumerationUsage(enum_usage) => {
                        self.lower_enum_usage(document, Some(declaration), enum_usage)?;
                    }
                    PartUsageBodyElement::RequirementDef(requirement_def) => {
                        self.lower_requirement_def(document, Some(declaration), requirement_def)?;
                    }
                    PartUsageBodyElement::AnalysisCaseDef(analysis_case_def) => {
                        self.lower_analysis_case_def(
                            document,
                            Some(declaration),
                            analysis_case_def,
                        )?;
                    }
                    PartUsageBodyElement::AnalysisCaseUsage(analysis_case_usage) => {
                        self.lower_analysis_case_usage(
                            document,
                            Some(declaration),
                            analysis_case_usage,
                        )?;
                    }
                    PartUsageBodyElement::RequirementUsage(requirement_usage) => {
                        self.lower_requirement_usage(
                            document,
                            Some(declaration),
                            requirement_usage,
                        )?;
                    }
                    PartUsageBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    PartUsageBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PartUsageBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    PartUsageBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PartUsageBodyElement::MetadataDef(metadata_def) => {
                        self.lower_metadata_def(document, Some(declaration), metadata_def)?;
                    }
                    PartUsageBodyElement::MetadataUsage(metadata_usage) => {
                        self.lower_metadata_usage(document, Some(declaration), metadata_usage)?;
                    }
                    PartUsageBodyElement::ActionUsage(action_usage) => {
                        self.lower_action_usage(document, Some(declaration), action_usage)?;
                    }
                    PartUsageBodyElement::StateDef(state_def) => {
                        self.lower_state_def(document, Some(declaration), state_def)?;
                    }
                    PartUsageBodyElement::StateUsage(state_usage) => {
                        self.lower_state_usage(document, Some(declaration), state_usage)?;
                    }
                    PartUsageBodyElement::ConnectionDef(connection_def) => {
                        self.lower_connection_def(document, Some(declaration), connection_def)?;
                    }
                    PartUsageBodyElement::Connection(connection_usage) => {
                        self.lower_connection_usage(document, Some(declaration), connection_usage)?;
                    }
                    PartUsageBodyElement::OccurrenceDef(occurrence_def) => {
                        self.lower_occurrence_def(document, Some(declaration), occurrence_def)?;
                    }
                    PartUsageBodyElement::OccurrenceUsage(occurrence_usage) => {
                        self.lower_occurrence_usage(document, Some(declaration), occurrence_usage)?;
                    }
                    PartUsageBodyElement::FlowDef(flow_def) => {
                        self.lower_flow_def(document, Some(declaration), flow_def)?;
                    }
                    PartUsageBodyElement::InterfaceUsage(interface_usage) => {
                        self.lower_interface_usage(document, Some(declaration), interface_usage)?;
                    }
                    PartUsageBodyElement::ConstraintDef(constraint_def) => {
                        self.lower_constraint_def(document, Some(declaration), constraint_def)?;
                    }
                    PartUsageBodyElement::ConstraintUsage(constraint_usage) => {
                        self.lower_constraint_usage(document, Some(declaration), constraint_usage)?;
                    }
                    PartUsageBodyElement::CalcDef(calc_def) => {
                        self.lower_calc_def(document, Some(declaration), calc_def)?;
                    }
                    PartUsageBodyElement::CalcUsage(calc_usage) => {
                        self.lower_calc_usage(document, Some(declaration), calc_usage)?;
                    }
                    PartUsageBodyElement::AliasDef(alias_def) => {
                        self.lower_alias_def(document, Some(declaration), alias_def)?;
                    }
                    PartUsageBodyElement::Perform(perform) => {
                        self.lower_perform(document, Some(declaration), perform)?;
                    }
                    PartUsageBodyElement::Doc(_) => {}
                    PartUsageBodyElement::MetadataAnnotation(node) => {
                        self.lower_metadata_annotation(document, declaration, node)?;
                    }
                    PartUsageBodyElement::Satisfy(node) => {
                        self.lower_satisfy(
                            document,
                            declaration,
                            UnsupportedFamily::PartUsageMember,
                            node,
                        )?;
                    }
                    PartUsageBodyElement::VariantUsage(node) => {
                        self.lower_variant_usage(
                            document,
                            declaration,
                            UnsupportedFamily::PartUsageMember,
                            node,
                        )?;
                    }
                    PartUsageBodyElement::Allocate(node) => {
                        self.lower_allocate(
                            document,
                            declaration,
                            UnsupportedFamily::PartUsageMember,
                            node,
                        )?;
                    }
                    PartUsageBodyElement::Bind(node) => {
                        self.lower_bind(
                            document,
                            declaration,
                            UnsupportedFamily::PartUsageMember,
                            node,
                        )?;
                    }
                    PartUsageBodyElement::AssertConstraint(node) => self
                        .lower_assert_constraint_member(
                            document,
                            declaration,
                            UnsupportedFamily::PartUsageMember,
                            node,
                        )?,
                    PartUsageBodyElement::Annotation(_)
                    | PartUsageBodyElement::DefaultReferenceUsage(_)
                    | PartUsageBodyElement::Ref(_)
                    | PartUsageBodyElement::Connect(_)
                    | PartUsageBodyElement::FlowUsage(_)
                    | PartUsageBodyElement::SuccessionUsage(_)
                    | PartUsageBodyElement::MetadataKeywordUsage(_)
                    | PartUsageBodyElement::IncludeUseCase(_)
                    | PartUsageBodyElement::UseCaseUsage(_)
                    | PartUsageBodyElement::VerificationCaseUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::PartUsageMember,
                        element.span.clone(),
                    ),
                }
            }
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeUsage,
            name,
            node.span.clone(),
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
        self.lower_attribute_default_value(declaration, node.value.value.as_ref());
        self.lower_attribute_body(document, declaration, &node.value.body)?;
        Ok(())
    }

    /// Slice 3 of the constraint/calc expression fact family (prerequisite step, see
    /// `RESOLUTION_LAYER_DESIGN.md`): a minimal, literal-only mirror of slice 2's
    /// `classify_constraint_expression`/`classify_calc_expression` literal folding, scoped to a
    /// `FeatureValue`'s expression on an attribute def/usage's own `=`/`:=`/`default` clause
    /// (`attribute mass = 5;`). Publishes a `Literal` evaluation fact sourced directly at the
    /// attribute's own declaration -- exactly the shape constant propagation
    /// (`compute_evaluation` in resolver.rs) looks up when a constraint/calc expression's operand
    /// reference resolves to this declaration. Deliberately narrow: only a bare
    /// `LiteralInteger`/`LiteralReal`/`LiteralBoolean` expression (optionally parenthesized)
    /// publishes a fact; any other default-value expression shape (arithmetic, another feature
    /// reference, an invocation, etc.) is left entirely untouched -- no fact, no reference, no
    /// diagnostic -- since general default-value expression lowering (arithmetic, feature-ref
    /// operands with their own resolution) is out of scope here and deferred to a future slice
    /// (see `REMAINING_WORK_TO_PORT.md`'s "default value expressions on attributes/parameters").
    fn lower_attribute_default_value(
        &mut self,
        declaration: DeclarationId,
        value: Option<&Node<FeatureValue>>,
    ) {
        fn unwrap_literal(node: &Expression) -> Option<EvaluatedValue> {
            match node {
                Expression::Parenthesized(inner) => unwrap_literal(&inner.value),
                other => literal_expression_value(other),
            }
        }
        if let Some(feature_value) = value {
            if let Some(literal) = unwrap_literal(&feature_value.value.expression.value) {
                self.push_evaluation_fact(declaration, ExpressionEvalShape::Literal(literal));
            }
        }
    }

    fn lower_attribute_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AttributeDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeDefinition,
            name,
            node.span.clone(),
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
        self.lower_attribute_default_value(declaration, node.value.value.as_ref());
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    fn lower_attribute_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &AttributeBody,
    ) -> Result<(), ConstructionError> {
        let AttributeBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                AttributeBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                AttributeBodyElement::Doc(_) => {}
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
                AttributeBodyElement::Connect(_)
                | AttributeBodyElement::MetadataKeywordUsage(_)
                | AttributeBodyElement::RefDecl(_)
                | AttributeBodyElement::Other(_) => self.push_unsupported(
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::EnumerationDefinition,
            name,
            node.span.clone(),
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
        if let EnumerationBody::Brace { values } = &node.value.body {
            for value in values {
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ItemDefinition,
            name,
            node.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ClassDefinition,
            name,
            node.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ItemUsage,
            name,
            node.span.clone(),
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
    /// `PortUsage`. Anonymous redefinition-only parameters (`type_name` is `None`, e.g. a bare
    /// `in :>> target = expr;`) get only the declaration/membership shell -- no `FeatureTyping`
    /// reference (and hence no direction fact) is pushed for them, and their `redefines`/`value`
    /// clauses are left unlowered (out of scope for this slice, matching multiplicity). The
    /// declared name may be empty for that same anonymous shape; `intern_declared_name` already
    /// treats an empty name as anonymous (see its callers for `EnumerationLiteral` etc.).
    fn lower_parameter_declaration(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<InOutDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ParameterUsage,
            name,
            node.span.clone(),
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
        if let Some(expression) = &node.value.value {
            self.push_evaluation_fact(declaration, classify_calc_expression(&expression.value));
            self.lower_calc_expression(
                document,
                declaration,
                UnsupportedFamily::CalcDefinitionMember,
                expression,
            )?;
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PerformActionUsage,
            name,
            node.span.clone(),
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
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_perform_body(document, declaration, &node.value.body)
    }

    /// Lowers the `PerformBody` owned by a `perform action` usage (BNF `PerformBodyElement`):
    /// only nested `part`/`item` usages are recognized; in/out bindings, nested action-body
    /// content, and variant members are out of scope and fall through to the enclosing
    /// unsupported-member family via `push_unsupported`.
    fn lower_perform_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &PerformBody,
    ) -> Result<(), ConstructionError> {
        let PerformBody::Brace { elements } = body else {
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
                PerformBodyElement::Doc(_) => {}
                PerformBodyElement::InOut(_)
                | PerformBodyElement::Variant(_)
                | PerformBodyElement::Action(_)
                | PerformBodyElement::AttributeUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionUsageMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::MetadataDefinition,
            name,
            node.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ActionDefinition,
            name,
            node.span.clone(),
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
        let ActionDefBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
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
                    self.lower_parameter_declaration(document, Some(owner), param)?;
                }
                ActionDefBodyElement::Perform(perform) => {
                    self.lower_perform(document, Some(owner), perform)?;
                }
                ActionDefBodyElement::Doc(_) => {}
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
                ActionDefBodyElement::AssertConstraint(node) => self
                    .lower_assert_constraint_member(
                        document,
                        owner,
                        UnsupportedFamily::ActionDefinitionMember,
                        node,
                    )?,
                ActionDefBodyElement::Annotation(_)
                | ActionDefBodyElement::MetadataKeywordUsage(_)
                | ActionDefBodyElement::TextualRep(_)
                | ActionDefBodyElement::RefDecl(_)
                | ActionDefBodyElement::FlowUsage(_)
                | ActionDefBodyElement::MergeStmt(_)
                | ActionDefBodyElement::DecisionStmt(_)
                | ActionDefBodyElement::JoinStmt(_)
                | ActionDefBodyElement::ForkStmt(_)
                | ActionDefBodyElement::TerminateStmt(_)
                | ActionDefBodyElement::WhileStmt(_)
                | ActionDefBodyElement::LoopStmt(_)
                | ActionDefBodyElement::IfStmt(_)
                | ActionDefBodyElement::Assign(_)
                | ActionDefBodyElement::ForLoop(_)
                | ActionDefBodyElement::ThenAction(_)
                | ActionDefBodyElement::Decl(_)
                | ActionDefBodyElement::DefaultReferenceUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionDefinitionMember,
                    element.span.clone(),
                ),
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
        self.lower_action_usage_body(document, declaration, &node.value.body)
    }

    /// Lowers the `ActionUsageBody` owned by an `action` usage (BNF `ActionUsageBodyElement`):
    /// see `lower_action_def_body` for the shared recognized/unsupported shape. The one
    /// additional alternative here, `VariantUsage`, is out of scope and falls through to the same
    /// `unsupported_action_usage_member` family.
    fn lower_action_usage_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionUsageBody,
    ) -> Result<(), ConstructionError> {
        let ActionUsageBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
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
                    self.lower_parameter_declaration(document, Some(owner), param)?;
                }
                ActionUsageBodyElement::Doc(_) => {}
                ActionUsageBodyElement::MetadataAnnotation(node) => {
                    self.lower_metadata_annotation(document, owner, node)?;
                }
                ActionUsageBodyElement::Bind(node) => {
                    self.lower_bind(document, owner, UnsupportedFamily::ActionUsageMember, node)?;
                }
                ActionUsageBodyElement::AssertConstraint(node) => self
                    .lower_assert_constraint_member(
                        document,
                        owner,
                        UnsupportedFamily::ActionUsageMember,
                        node,
                    )?,
                ActionUsageBodyElement::Annotation(_)
                | ActionUsageBodyElement::MetadataKeywordUsage(_)
                | ActionUsageBodyElement::TextualRep(_)
                | ActionUsageBodyElement::RefDecl(_)
                | ActionUsageBodyElement::FlowUsage(_)
                | ActionUsageBodyElement::MergeStmt(_)
                | ActionUsageBodyElement::DecisionStmt(_)
                | ActionUsageBodyElement::JoinStmt(_)
                | ActionUsageBodyElement::ForkStmt(_)
                | ActionUsageBodyElement::TerminateStmt(_)
                | ActionUsageBodyElement::WhileStmt(_)
                | ActionUsageBodyElement::LoopStmt(_)
                | ActionUsageBodyElement::IfStmt(_)
                | ActionUsageBodyElement::Assign(_)
                | ActionUsageBodyElement::ForLoop(_)
                | ActionUsageBodyElement::ThenAction(_)
                | ActionUsageBodyElement::Decl(_)
                | ActionUsageBodyElement::DefaultReferenceUsage(_)
                | ActionUsageBodyElement::VariantUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionUsageMember,
                    element.span.clone(),
                ),
            }
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
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_succession_end(document, declaration, family, &node.value.first)?;
        if let Some(then) = &node.value.then {
            self.lower_succession_end(document, declaration, family, then)?;
        }
        Ok(())
    }

    /// Lowers one succession end (the `first` or `then` operand of a `FirstStmt`): its path
    /// expression is a structured `Expression` (not a flattened string), so a simple/qualified
    /// name (`Expression::FeatureRef`) resolves as an authored `Succession` reference through the
    /// same shared lexical lookup as `ConnectorEnd`. A dotted feature-chain path
    /// (`Expression::MemberAccess`) or any other expression shape -- including the bare `start`/
    /// `done` pseudo-action markers, which parse as an ordinary `FeatureRef` that legitimately
    /// fails to resolve because no such declaration is synthesized -- has no chained-feature-
    /// access resolution anywhere in this pipeline yet, so only the `FeatureRef` shape is
    /// resolved here; anything else is left as an explicit unsupported-member diagnostic.
    fn lower_succession_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
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
                    kind: ReferenceKind::Succession,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            _ => self.push_unsupported(document, family, node.span.clone()),
        }
        Ok(())
    }

    /// Lowers a `constraint def`/`constraint` usage body's boolean expression (slice 1 of the
    /// constraint/calc expression fact family; see `ReferenceKind::ExpressionOperand`). Supports
    /// only a narrow "boolean comparison" expression shape: a literal, a feature/feature-chain
    /// reference (resolved as an `ExpressionOperand` reference sourced at `declaration`, exactly
    /// like `lower_succession_end` resolves `Expression::FeatureRef` through the shared
    /// `DeclarationDomain::Any` lexical lookup fixed point), a parenthesized wrapper (unwrapped
    /// and recursed into), or a comparison `BinaryOp` (`Eq`/`Ne`/`Lt`/`Le`/`Gt`/`Ge` -- `StrictEq`/
    /// `StrictNe` KerML identity comparisons are deliberately excluded from this narrow slice, left
    /// unsupported like every other operator) whose operands are recursed into. Evaluation
    /// (computing an actual truth value) is out of scope. Any other expression shape -- arithmetic
    /// ops, invocations, tuples, type-check/classification expressions, unary ops, a dotted
    /// `MemberAccess` chain, etc. -- falls through to the existing unsupported-member diagnostic,
    /// unchanged from prior behavior.
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
            | Expression::LiteralBoolean(_) => Ok(()),
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
            Expression::Parenthesized(inner) => {
                self.lower_constraint_expression(document, declaration, family, inner)
            }
            Expression::BinaryOp { op, left, right } if is_comparison_operator(op) => {
                self.lower_constraint_expression(document, declaration, family, left)?;
                self.lower_constraint_expression(document, declaration, family, right)
            }
            _ => {
                self.push_unsupported(document, family, node.span.clone());
                Ok(())
            }
        }
    }

    /// Lowers a `calc def`/`calc` usage body's formula expression (slice 1 of the constraint/calc
    /// expression fact family, extended by slice 4 for arithmetic). Calc bodies are typically
    /// arithmetic-result formulas rather than boolean comparisons, so comparison-operator support
    /// (`lower_constraint_expression`'s `BinaryOp` arm) deliberately does not apply here; this slice
    /// supports the minimal leaf shapes -- a literal, a feature/feature-chain reference (resolved
    /// as an `ExpressionOperand` reference exactly like `lower_constraint_expression`), a
    /// parenthesized wrapper -- plus (slice 4) an arithmetic `BinaryOp` (`Add`/`Sub`/`Mul`/`Div`/
    /// `Mod`, see `is_arithmetic_operator`; `Exp`/`Pow` deliberately excluded, see
    /// `is_arithmetic_operator`'s doc comment) whose operands are recursed into just like
    /// `lower_constraint_expression`'s comparison arm. Invocations, unary ops, `Exp`/`Pow`, and
    /// every other expression shape stay unsupported, falling through to the existing
    /// `unsupported_calc_definition_member` diagnostic, unchanged from prior behavior.
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
            | Expression::LiteralBoolean(_) => Ok(()),
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
            Expression::Parenthesized(inner) => {
                self.lower_calc_expression(document, declaration, family, inner)
            }
            Expression::BinaryOp { op, left, right } if is_arithmetic_operator(op) => {
                self.lower_calc_expression(document, declaration, family, left)?;
                self.lower_calc_expression(document, declaration, family, right)
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
    /// and a logical `BinaryOp` (`and`/`or`, `is_logical_operator`) whose operands are recursed
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
            | Expression::LiteralBoolean(_) => Ok(()),
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
            Expression::Parenthesized(inner) => {
                self.lower_filter_expression(document, declaration, inner)
            }
            Expression::BinaryOp { op, left, right }
                if is_comparison_operator(op) || is_logical_operator(op) =>
            {
                self.lower_filter_expression(document, declaration, left)?;
                self.lower_filter_expression(document, declaration, right)
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateDefinition,
            name,
            node.span.clone(),
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
    /// members (BNF `StateDefBodyElement`): the only recognized owned member is a nested state
    /// usage; everything else -- entry/do/exit action bindings, `then`/`final` state markers,
    /// `ref` bindings, transitions -- falls through to `unsupported_state_definition_member`.
    /// This is the genuinely out-of-scope state-machine surface for this slice.
    fn lower_state_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &StateDefBody,
    ) -> Result<(), ConstructionError> {
        let StateDefBody::Brace { elements } = body else {
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
                StateDefBodyElement::Doc(_) => {}
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
                    self.lower_parameter_declaration(document, Some(owner), param)?;
                }
                StateDefBodyElement::Annotation(_)
                | StateDefBodyElement::MetadataKeywordUsage(_)
                | StateDefBodyElement::Other(_)
                | StateDefBodyElement::FinalState(_)
                | StateDefBodyElement::Ref(_) => self.push_unsupported(
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
    /// action (`action_reference: None`, e.g. a bare `entry;` or an inline `entry { ... }` body)
    /// has no reference to lower and falls through to the existing unsupported diagnostic; its
    /// own body content stays out of scope either way.
    fn lower_state_entry_action(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<EntryAction>,
    ) -> Result<(), ConstructionError> {
        let Some(target) = node.value.action_reference else {
            self.push_unsupported(
                document,
                UnsupportedFamily::StateDefinitionMember,
                node.span.clone(),
            );
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::EntryActionBinding,
            None,
            node.span.clone(),
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
            self.push_unsupported(
                document,
                UnsupportedFamily::StateDefinitionMember,
                node.span.clone(),
            );
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::DoActionBinding,
            None,
            node.span.clone(),
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
            self.push_unsupported(
                document,
                UnsupportedFamily::StateDefinitionMember,
                node.span.clone(),
            );
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ExitActionBinding,
            None,
            node.span.clone(),
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
            Some(TransitionAccept::Payload(_, _)) | Some(TransitionAccept::TimeTrigger(_, _)) => {
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
    /// `DeclarationDomain::Any` lexical lookup. Any other expression shape falls through to an
    /// explicit `family` unsupported diagnostic.
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
    /// prefix on either end and the whole construct itself, and any real content in the optional
    /// braced body (`Bind.body_elements`), are out of scope and flagged as unsupported per
    /// element, mirroring `lower_satisfy`'s `body_elements` handling.
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
            self.push_unsupported(document, family, element.span.clone());
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementDefinition,
            name,
            node.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementUsage,
            name,
            node.span.clone(),
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
        let RequirementDefBody::Brace { elements } = body else {
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
                RequirementDefBodyElement::Doc(_) => {}
                RequirementDefBodyElement::MetadataAnnotation(node) => {
                    self.lower_metadata_annotation(document, owner, node)?;
                }
                RequirementDefBodyElement::Other(_)
                | RequirementDefBodyElement::Annotation(_)
                | RequirementDefBodyElement::MetadataKeywordUsage(_)
                | RequirementDefBodyElement::SubjectRef(_)
                | RequirementDefBodyElement::RequirementActorDecl(_)
                | RequirementDefBodyElement::Stakeholder(_)
                | RequirementDefBodyElement::Purpose(_)
                | RequirementDefBodyElement::VariantUsage(_)
                | RequirementDefBodyElement::VerifyRequirement(_)
                | RequirementDefBodyElement::RequireConstraint(_)
                | RequirementDefBodyElement::Frame(_)
                | RequirementDefBodyElement::TextualRep(_) => {
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewpointDefinition,
            name,
            node.span.clone(),
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
    /// (UPSTREAM_PARSER_GAPS.md #9), resolved upstream in `0757de13`. Stakeholder/subject-binding
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
        let declaration =
            self.push_typed_declaration(document, owner, kind, name, node.span.clone())?;
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
    /// UPSTREAM_PARSER_GAPS.md #5) fall through to `unsupported_analysis_case_definition_member`.
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AnalysisCaseDefinition,
            name,
            node.span.clone(),
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
    /// is deferred entirely (UPSTREAM_PARSER_GAPS.md #5): `CaseUsage` silently drops parsed
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CaseDefinition,
            name,
            node.span.clone(),
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
    /// (UPSTREAM_PARSER_GAPS.md #5): `AnalysisCaseUsage` previously had no typed field to lower
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
    /// (UPSTREAM_PARSER_GAPS.md #5): `CaseUsage` previously had no typed field to lower
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

    /// Lowers a `verification def` (BNF VerificationCaseDefinition), mirroring `lower_case_def`.
    /// Verification-specific semantics are explicitly out of scope; unrecognized body elements
    /// fall through to `unsupported_verification_case_definition_member`. `verification` usage
    /// lowering is deferred entirely (UPSTREAM_PARSER_GAPS.md #5): `VerificationCaseUsage`
    /// silently drops parsed `:>`/`:>>` clauses, unlike `VerificationCaseDef`.
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::VerificationCaseDefinition,
            name,
            node.span.clone(),
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
    /// lowering is deferred entirely (UPSTREAM_PARSER_GAPS.md #5): `UseCaseUsage` silently drops
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::UseCaseDefinition,
            name,
            node.span.clone(),
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
        let UseCaseDefBody::Brace { elements } = body else {
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
                UseCaseDefBodyElement::Doc(_) => {}
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
                UseCaseDefBodyElement::Other(_)
                | UseCaseDefBodyElement::Annotation(_)
                | UseCaseDefBodyElement::MetadataKeywordUsage(_)
                | UseCaseDefBodyElement::SubjectRef(_)
                | UseCaseDefBodyElement::ActorUsage(_)
                | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
                | UseCaseDefBodyElement::Objective(_)
                | UseCaseDefBodyElement::FirstSuccession(_)
                | UseCaseDefBodyElement::ThenUseCaseUsage(_)
                | UseCaseDefBodyElement::ThenDone(_)
                | UseCaseDefBodyElement::RefRedefinition(_)
                | UseCaseDefBodyElement::ReturnRef(_)
                | UseCaseDefBodyElement::CaseReturnDecl(_)
                | UseCaseDefBodyElement::Assign(_)
                | UseCaseDefBodyElement::ForLoop(_)
                | UseCaseDefBodyElement::ThenAction(_)
                | UseCaseDefBodyElement::Expression(_)
                | UseCaseDefBodyElement::FlowUsage(_) => {
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PortDefinition,
            name,
            node.span.clone(),
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
        if let PortDefBody::Brace { elements } = &node.value.body {
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
                    PortDefBodyElement::Doc(_) => {}
                    PortDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(document, Some(declaration), param)?;
                    }
                    PortDefBodyElement::MetadataKeywordUsage(_) | PortDefBodyElement::Other(_) => {
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::PortDefinitionMember,
                            element.span.clone(),
                        )
                    }
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PortUsage,
            name,
            node.span.clone(),
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
        if let PortBody::Brace { elements } = &node.value.body {
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
                    PortBodyElement::Doc(_) => {}
                    PortBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(document, Some(declaration), param)?;
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConnectionDefinition,
            name,
            node.span.clone(),
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
        if let ConnectionDefBody::Brace { elements } = body {
            for element in elements {
                match &element.value {
                    ConnectionDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ConnectionDefBodyElement::Doc(_) => {}
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
                    ConnectionDefBodyElement::RefDecl(_)
                    | ConnectionDefBodyElement::SuccessionUsage(_) => self.push_unsupported(
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
        Ok(())
    }

    /// Lowers one connector end (`ConnectionEnd`, used by both `ConnectStmt` and
    /// `ConnectionUsageMember`'s inline `connect` clause): its path expression is a structured
    /// `Expression` (not a flattened string), so a simple/qualified name (`Expression::FeatureRef`)
    /// resolves as an authored `ConnectorEnd` reference through the same shared lexical lookup as
    /// `AliasBinding`. A dotted feature-chain path (`Expression::MemberAccess`, e.g. `a.portA`) or
    /// any other expression shape has no chained-feature-access resolution anywhere in this
    /// pipeline yet (a materially larger resolution problem than a single-segment reference), so
    /// it is left as an explicit `unsupported_connection_definition_member` diagnostic here rather
    /// than a fabricated or partial resolution.
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
    /// `DeclarationKind::InterfaceDefinition`'s doc comment and UPSTREAM_PARSER_GAPS.md #6.
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::InterfaceDefinition,
            name,
            node.span.clone(),
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
        if let InterfaceDefBody::Brace { elements } = body {
            for element in elements {
                match &element.value {
                    InterfaceDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    InterfaceDefBodyElement::Doc(_) => {}
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
                    InterfaceDefBodyElement::RefDecl(_) | InterfaceDefBodyElement::FlowUsage(_) => {
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::InterfaceDefinitionMember,
                            element.span.clone(),
                        )
                    }
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
    /// upstream in `0757de13` (UPSTREAM_PARSER_GAPS.md #6).
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
                InterfaceUsageBodyElement::Doc(_) => {}
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
    /// doc comment and UPSTREAM_PARSER_GAPS.md #8.
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewDefinition,
            name,
            node.span.clone(),
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
        if let ViewDefBody::Brace { elements } = body {
            for element in elements {
                match &element.value {
                    ViewDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ViewDefBodyElement::Doc(_) => {}
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
                    ViewDefBodyElement::ViewRendering(_) | ViewDefBodyElement::Other(_) => self
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

    /// Lowers a package/definition/usage-level `view` feature member (BNF ViewUsage), mirroring
    /// `lower_analysis_case_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (UPSTREAM_PARSER_GAPS.md #8): `ViewUsage` previously had no `subsets` field. Multiplicity
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
        if let ViewBody::Brace { elements } = body {
            for element in elements {
                match &element.value {
                    ViewBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ViewBodyElement::Doc(_) => {}
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
                    ViewBodyElement::ViewRendering(_)
                    | ViewBodyElement::Expose(_)
                    | ViewBodyElement::Other(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ViewDefinitionMember,
                        element.span.clone(),
                    ),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConstraintDefinition,
            name,
            node.span.clone(),
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
        if let ConstraintDefBody::Brace { elements } = body {
            for element in elements {
                match &element.value {
                    ConstraintDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ConstraintDefBodyElement::Constraint(constraint) => {
                        self.lower_constraint_usage(document, Some(declaration), constraint)?;
                    }
                    ConstraintDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(document, Some(declaration), param)?;
                    }
                    ConstraintDefBodyElement::Doc(_) => {}
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
                    ConstraintDefBodyElement::Other(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ConstraintDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `constraint` feature member (BNF
    /// ConstraintUsage), mirroring `lower_analysis_case_usage`: ownership, membership, a `:`
    /// typing target, and `subsets`/`redefines` subsetting relationships. Resolved upstream in
    /// `0757de13` (UPSTREAM_PARSER_GAPS.md #4): `ConstraintUsage` previously had no
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
            DeclarationKind::ConstraintUsage,
            name,
            node.span.clone(),
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

    /// Lowers a `calc def` (BNF CalculationDefinition), mirroring `lower_action_def`: ownership,
    /// membership, an optional `:>` specialization relationship participating in the shared
    /// `DeclarationDomain::Type` fixed point. Resolved upstream in `0757de13`
    /// (UPSTREAM_PARSER_GAPS.md #3): `CalcDef` previously dropped its parsed `:>` clause.
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CalcDefinition,
            name,
            node.span.clone(),
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
        if let CalcDefBody::Brace { elements } = body {
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
                        self.lower_parameter_declaration(document, Some(declaration), param)?;
                    }
                    CalcDefBodyElement::Doc(_) => {}
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
                    CalcDefBodyElement::Other(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::CalcDefinitionMember,
                        element.span.clone(),
                    ),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CalcUsage,
            name,
            node.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RenderingDefinition,
            name,
            node.span.clone(),
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
        let RenderingDefBody::Brace { elements } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                RenderingDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                RenderingDefBodyElement::Doc(_) => {}
                RenderingDefBodyElement::Filter(filter) => {
                    self.lower_filter_expression(document, declaration, &filter.value.condition)?;
                }
                RenderingDefBodyElement::ViewRendering(_) | RenderingDefBodyElement::Other(_) => {
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::RenderingDefinitionMember,
                        element.span.clone(),
                    )
                }
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::OccurrenceDefinition,
            name,
            node.span.clone(),
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
        let DefinitionBody::Brace { elements } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::Doc(_) => {}
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Other(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    element.span.clone(),
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
            OccurrenceBodyElement::Doc(_) => {}
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
            | OccurrenceBodyElement::Other(_)
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
        let OccurrenceUsageBody::Brace { elements } = &node.value.body else {
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AllocationDefinition,
            name,
            node.span.clone(),
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
        let DefinitionBody::Brace { elements } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::Doc(_) => {}
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Other(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    element.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::FlowDefinition,
            name,
            node.span.clone(),
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
        let DefinitionBody::Brace { elements } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::Doc(_) => {}
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Other(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    element.span.clone(),
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
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Alias,
            name,
            node.span.clone(),
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
    /// `Satisfy.inline_requirement` -- and falls through to an explicit unsupported-member
    /// diagnostic, as does the untyped form's optional nested body (`VariantUsage.body`) and the
    /// case where neither `reference` nor `typed` is present.
    fn lower_variant_usage(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<VariantUsage>,
    ) -> Result<(), ConstructionError> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SemanticModelBuildCoordinator;

impl SemanticModelBuildCoordinator {
    pub(crate) fn build(
        mut sources: Vec<OwnedSourceRecord>,
        schedule: BuildSchedule,
    ) -> Result<resolver::ResolvedSemanticModel, CoordinatorError> {
        sources.sort_unstable_by(|left, right| left.identity.cmp(&right.identity));
        if sources
            .windows(2)
            .any(|pair| pair[0].identity == pair[1].identity)
        {
            return Err(CoordinatorError::DuplicateSourceIdentity);
        }

        let parsed = match schedule {
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

        let mut builder = SemanticModelBuilder::default();
        let mut documents = Vec::with_capacity(parsed.len());
        for (identity, parsed) in parsed {
            let document = builder
                .admit_document(identity, Arc::new(parsed.document), parsed.errors)
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push(document);
        }
        for document in documents {
            builder
                .canonicalize_document(document)
                .map_err(|_| CoordinatorError::ConstructionFailed)?;
        }
        builder
            .freeze()
            .resolve()
            .map_err(|_| CoordinatorError::ConstructionFailed)
    }

    fn parse_source(
        source: OwnedSourceRecord,
    ) -> Result<(Box<str>, sysml_v2_parser_next::ParseResult), CoordinatorError> {
        Ok((
            source.identity,
            sysml_v2_parser_next::parse_for_editor_owned(source.content),
        ))
    }
}

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
            .admit_document("model", parsed.clone(), Vec::new())
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
                .admit_document(format!("model-{index}"), parsed.clone(), Vec::new())
                .unwrap();
        }
        let before = builder.documents.len();

        assert_eq!(
            builder
                .admit_document("model-0", parsed, Vec::new())
                .unwrap_err(),
            ConstructionError::DuplicateDocumentIdentity
        );
        assert_eq!(builder.documents.len(), before);
    }

    #[test]
    fn anonymous_ordinals_are_owner_local_and_ignore_named_declarations() {
        let parsed = empty_document();
        let mut builder = SemanticModelBuilder::default();
        let document = builder.admit_document("model", parsed, Vec::new()).unwrap();
        let owner_name = builder.intern_name("Owner").unwrap();
        let owner = builder
            .push_typed_declaration(
                document,
                None,
                DeclarationKind::Package,
                Some(owner_name),
                Span::dummy(),
            )
            .unwrap();
        let first = builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::Import,
                None,
                Span::dummy(),
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
            )
            .unwrap();
        let second = builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::Import,
                None,
                Span::dummy(),
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
        // UPSTREAM_PARSER_GAPS.md #4 was resolved upstream in `0757de13`: `ConstraintUsage` now
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
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \tconstraint def C { compute(x, y) }\n\
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
            "expected a function-call expression to still surface as an unsupported \
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
                 (qualified-name \"Demo::C\"))) (value (kind boolean) (boolean true)))"
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
                 (qualified-name \"Demo::C\"))) (value (kind boolean) (boolean false)))"
            ),
            "expected `2 < 1` to fold to a published Boolean(false) evaluation fact, got:\n{output}"
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
            output.contains("(value (kind boolean) (boolean true)))"),
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
                 (qualified-name \"Demo::C\"))) (value (kind non-constant)))"
            ),
            "expected a resolved but non-literal operand `x` to publish NonConstant rather than \
             a fabricated boolean, got:\n{output}"
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
                 (qualified-name \"Demo::C\"))) (value (kind unresolved-operand)))"
            ),
            "expected an undeclared operand `x` to publish UnresolvedOperand rather than a \
             fabricated boolean, got:\n{output}"
        );
    }

    #[test]
    fn constraint_unsupported_arithmetic_shape_publishes_no_evaluation_fact() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { compute(x, y) }\n\
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
                 (qualified-name \"Demo::Calc\"))) (value (kind integer) (integer 5)))"
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
                 (qualified-name \"Demo::Calc\"))) (value (kind real) (real 6"
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
                 (qualified-name \"Demo::Calc\"))) (value (kind division-by-zero)))"
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
                 (qualified-name \"Demo::Calc\"))) (value (kind division-by-zero)))"
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
                 (qualified-name \"Demo::Calc\"))) (value (kind integer) (integer 20)))"
            ),
            "expected `length * width` to propagate both attributes' literal defaults and fold to \
             Integer(20), got:\n{output}"
        );
    }

    #[test]
    fn calc_exponent_operator_stays_unsupported_not_a_panic() {
        // `**` (BinaryOperator::Pow) is deliberately excluded from slice 4's arithmetic scope
        // (see `is_arithmetic_operator`'s doc comment); it must fall through to the existing
        // unsupported-shape path, publishing no evaluation fact, not panic.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def Calc { 2 ** 3 }\n\
             }\n",
        );
        assert!(
            !output.contains("(evaluated (declaration"),
            "expected `2 ** 3` (Exp/Pow, out of slice-4 scope) to publish no evaluation fact, \
             got:\n{output}"
        );
    }

    #[test]
    fn constraint_arithmetic_mixed_with_comparison_stays_unsupported_not_a_panic() {
        // Mixing arithmetic into a constraint's comparison shape (`(a + b) > c`) is deliberately
        // out of scope: constraint bodies remain comparison-only (slice 1), and calc bodies never
        // get comparison support, so this combination is never recognized by either lowering path.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { (1 + 2) > 0 }\n\
             }\n",
        );
        assert!(
            !output.contains("(evaluated (declaration"),
            "expected `(1 + 2) > 0` (arithmetic mixed with comparison) to publish no evaluation \
             fact, got:\n{output}"
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
                 (anonymous (kind parameter) (ordinal 0))))) (value (kind integer) (integer 5)))"
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
                 (qualified-name \"Demo::Calc::result\"))) (value (kind integer) (integer 20)))"
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
                 (qualified-name \"Demo::mass\"))) (value (kind integer) (integer 5)))"
            ),
            "expected a literal attribute default value to publish its own Integer(5) evaluation \
             fact, got:\n{output}"
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
                 (qualified-name \"Demo::C\"))) (value (kind boolean) (boolean true)))"
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
                 (qualified-name \"Demo::A\"))) (value (kind boolean) (boolean true)))"
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
                 (qualified-name \"Demo::A\"))) (value (kind non-converged)))"
            ),
            "expected cyclic constraint A to publish NonConverged, got:\n{output}"
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::B\"))) (value (kind non-converged)))"
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
                 (qualified-name \"Demo::C\"))) (value (kind non-constant)))"
            ),
            "expected an operand with no evaluated value at all to keep the expression \
             NonConstant, got:\n{output}"
        );
    }

    #[test]
    fn concern_def_lowers_to_a_declaration() {
        // UPSTREAM_PARSER_GAPS.md #9 was resolved upstream in `0757de13`: `ConcernUsage`
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
        // UPSTREAM_PARSER_GAPS.md #3 was resolved upstream in `0757de13`: `CalcDef` now carries a
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
        // UPSTREAM_PARSER_GAPS.md #8 was resolved upstream in `0757de13`: `ViewUsage` now carries
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
    fn interface_usage_declaration_typed_by_an_interface_def_resolves() {
        // UPSTREAM_PARSER_GAPS.md #6 was resolved upstream in `0757de13`: all three
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
        // UPSTREAM_PARSER_GAPS.md #5 was resolved upstream in `0757de13`: `AnalysisCaseUsage` now
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
        // UPSTREAM_PARSER_GAPS.md #5 was resolved upstream in `0757de13`: `CaseUsage` now carries
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
