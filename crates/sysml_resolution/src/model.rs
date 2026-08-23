//! Direct parser-to-semantic canonicalization storage.
//!
//! Private parser-owned semantic construction.
//!
//! This module deliberately exposes no storage, graph adapter, or independently publishable
//! authored model. The publication owner consumes the typed coordinator outcome below.

#[cfg(test)]
use crate::lower::SemanticModelBuilder;

macro_rules! semantic_id {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(crate) struct $name(pub(crate) u32);

        impl $name {
            pub(crate) fn from_index(index: usize) -> Result<Self, ConstructionError> {
                Ok(Self(
                    u32::try_from(index).map_err(|_| ConstructionError::Capacity)?,
                ))
            }

            pub(crate) fn index(self) -> usize {
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
pub(crate) enum ConstructionError {
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
    /// An action usage with the typed parser's `accept` clause. The OMG metaclass distinction is
    /// semantic: ordinary action usages and accept actions share grammar infrastructure, but only
    /// the latter own the `isTriggerAction` specialization predicates.
    AcceptActionUsage,
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
    /// `tests/snapshots/sysml/validation/11b_safety_and_security_feature_views.md`'s
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
    /// usage lowering (`DeclarationKind::VerificationCaseUsage`): `name`/`type_name`/
    /// `is_abstract`/`multiplicity`/`subsets`/body are lowered. `VerificationCaseUsage` still has
    /// no `redefines` field, so a header-level `:>>` clause fails to parse into this node at all
    /// (falls to raw-text recovery instead, per planning/UPSTREAM_PARSER_GAPS.md's `AllocationUsage`/
    /// `FlowUsage` gap class).
    VerificationCaseDefinition,
    /// `use case def` (BNF UseCaseDefinition): a type whose owned members are attribute usages
    /// and nested case structure, mirroring `CaseDefinition`/`AnalysisCaseDefinition` lowering.
    /// Use-case-specific semantics (actor/include structure) are out of scope here; only
    /// ownership, specialization, and owned-member structure are lowered. `use case` usage
    /// lowering (`DeclarationKind::UseCaseUsage`): like `VerificationCaseUsage`, `UseCaseUsage`
    /// still has no `redefines` field -- `name`/`type_name`/`is_abstract`/`multiplicity`/
    /// `subsets`/body are lowered.
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
    /// `lower_case_usage`: ownership, membership, a `:` typing target, a `[mult]` multiplicity, a
    /// `:>` subsetting clause, and owned-member structure via the shared
    /// `lower_case_family_def_body` walker. `ast::UseCaseUsage` still has no `redefines` field,
    /// so a `:>>` header clause fails to parse into `UseCaseUsage` at all.
    UseCaseUsage,
    /// A package/definition/usage-level `verification` feature member (BNF
    /// VerificationCaseUsage), mirroring `UseCaseUsage`'s lowering (shares the same field
    /// shape/limitation: no `redefines` field on `ast::VerificationCaseUsage`).
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
    /// usage kinds' `redefines` field is shaped) and a `:>` `subsets` clause.
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
    /// `Subject` is a derived case-level relationship projected
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
    /// through the exact same `ExpressionOperand`/`classify_expression` machinery a
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
    /// existing `classify_expression`/lexical-lookup machinery via `lower_then_action`.
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
    /// `classify_expression`/`lower_calc_expression` pipeline. Multiplicity and the
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
    /// classified/lowered through the existing `classify_expression`/`lower_calc_expression`
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
    /// through the shared `lower_value_assignment`-style `classify_expression`/
    /// `lower_constraint_expression` pipeline, publishing its own evaluation fact exactly like an
    /// attribute default value.
    Assign,
    /// An anonymous feature synthesized for a `while <condition> { ... }` loop control node (BNF
    /// `WhileStmt`, `ast::WhileStmt`) found in an action def/usage body. Owned by the enclosing
    /// action def/usage declaration, mirroring `Decide`/`Merge`'s nested-declaration shape: the
    /// required boolean `condition` is lowered through the same `classify_expression`/
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
    /// `range` collection expression is lowered through the same `classify_expression`/
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
    /// Its `RelationshipBody` members (doc/comment/metadata only) are walked through the shared
    /// `lower_relationship_body_elements` helper used by `Import`/`AliasDef`.
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
    /// An anonymous declaration synthesized for a view body's `expose <target>;` member (BNF
    /// `Expose`, `ast::view::ExposeMember`). Structurally the view-scoped sibling of `Import`: the
    /// production carries the same `ImportTarget`, so the target is lowered as one authored
    /// reference through the same lexical lookup, and the optional `RelationshipBody` walks the
    /// shared `lower_relationship_body_elements` helper `Import`/`AliasDef` use.
    ///
    /// Kept apart from `Import` because the two mean different things: an import brings names into
    /// a scope, while an expose selects the elements a view shows. Collapsing them would make a
    /// view's exposed members indistinguishable from its imports in query output, and would give
    /// the expose target import-conformance rules that do not apply to it.
    Expose,
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
pub(crate) enum MembershipKind {
    Owning,
    Feature,
    Import,
    Alias,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Visibility {
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
    /// An authored KerML `featured by` target. This is a `TypeFeaturing` relationship from the
    /// feature declaration to a Type, and remains distinct from ordinary `FeatureTyping`.
    TypeFeaturing,
    /// An authored KerML `chains` target. Derived featuring facts consume this canonical
    /// relationship after resolution; no consumer reconstructs it from source text.
    FeatureChaining,
    Subclassification,
    Subsetting,
    Redefinition,
    References,
    Crosses,
    Intersects,
    /// One target of a KerML `unions` header clause (`ast::KermlTypeRelationship` with
    /// `KermlTypeRelationshipKeyword::Unions`). KerML `Unioning`, a direct kind of `Relationship`
    /// -- *not* of `Specialization` -- relating `typeUnioned` (the owning type) to `unioningType`
    /// (this target). One reference per authored target: `classifier U unions A, B;` publishes two,
    /// whose ordinals preserve authored order across repeated clauses.
    Unioning,
    /// One target of a KerML `intersects` header clause. KerML `Intersecting`, relating
    /// `typeIntersected` to `intersectingType`.
    ///
    /// Distinct from [`ReferenceKind::Intersects`], which is the *feature*-level `intersects`
    /// operand of a subsetting-family clause (`ast::SubsettingKind::Intersects`). The two are
    /// different productions with different owners and are kept apart rather than merged.
    Intersecting,
    /// One target of a KerML `differences` header clause. KerML `Differencing`, relating
    /// `typeDifferenced` to `differencingType`.
    ///
    /// Ordinal order is semantically load-bearing here in a way it is not for the other three: the
    /// owning type classifies what the *first* target classifies, excluding everything the rest
    /// classify, and a later `differences` clause continues that exclusion list.
    Differencing,
    /// One target of a KerML `disjoint from` header clause. KerML `Disjoining`, relating
    /// `typeDisjoined` to `disjoiningType`.
    Disjoining,
    /// The authored target of an `alias X for Y;` member (`AliasDef::target`), resolved through
    /// the same lexical lookup fixed point as every other authored reference kind. Named
    /// `AliasBinding` to use the semantic contract's "alias binding" vocabulary rather than
    /// inventing new terminology.
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
    /// just a Type. Sourced directly at the enclosing `constraint`/`calc` declaration. Operand
    /// lookup begins in that declaration's owned scope, so its `in`/`out`/`return` parameters
    /// participate before the ordinary enclosing lexical scopes. Evaluation of the expression
    /// (computing an actual truth value) is explicitly out of scope for this slice; only the
    /// operand references themselves are resolved.
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
    /// The satisfied requirement named by a satisfy usage's reference alternative
    /// (`SatisfiedRequirement::Reference`, BNF `SatisfyRequirementUsage`'s
    /// `OwnedReferenceSubsetting`), resolved through the same `DeclarationDomain::Any` lexical
    /// lookup as `Succession`/`TransitionSource`: the satisfied requirement can be any owned
    /// feature, not just a Type. Sourced at an anonymous `DeclarationKind::Satisfy` feature owned
    /// by the enclosing package/part/occurrence/requirement/view declaration, mirroring
    /// `Transition`'s nested-declaration shape. The production's other alternative,
    /// `'requirement' UsageDeclaration` (`SatisfiedRequirement::Declaration`, which declares a
    /// requirement inline rather than referencing an existing one), and the members of the
    /// `RequirementBody` the usage owns are out of scope.
    SatisfySource,
    /// The authored subject of a satisfy usage's `by` clause
    /// (`SatisfyRequirementUsage.subject`), same shape and scope as `SatisfySource`. Absent when
    /// the author wrote no `by` clause, which the production allows.
    SatisfyTarget,
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
    /// `Satisfy`), since each `variant` member carries only a single operand -- a single-operand
    /// shape rather than `Succession`'s paired-ends shape.
    /// Multiple `variant` members owned by the same variation declaration become multiple
    /// `Variant` references from that one source, distinguished by ordinal like any other
    /// multi-target reference family (e.g. `Subclassification`'s multiple `:>` targets). The typed
    /// inline form (`variant part name : Type { ... }`, `VariantUsage.typed`) introduces a new
    /// usage rather than referencing an existing one -- out of scope, like `Satisfy`'s
    /// `inline_requirement` form -- and left as an explicit unsupported-member diagnostic; so is
    /// any optional nested body on the untyped reference form (`VariantUsage.body`).
    Variant,
    /// The authored target of a view body's `expose <target>;` member (`ExposeMember.target`),
    /// resolved through the same `DeclarationDomain::Any` lexical lookup as `SatisfySource`: an
    /// exposed element can be any member, not just a Type. Sourced at the anonymous
    /// `DeclarationKind::Expose` declaration the member lowers to, so a view's several `expose`
    /// members stay distinguishable.
    ///
    /// Deliberately not a `NamespaceImport`/`MembershipImport`: those carry
    /// [`AuthoredImportFacts`] and are judged by import conformance, which says nothing about what
    /// a view shows.
    ViewExpose,
    /// The authored `target` of an `include <includedUseCase>;` body element inside a `use case
    /// def`/`use case` usage body (BNF `UseCaseDefBodyElement::IncludeUseCase`/
    /// `ThenIncludeUseCase`, `ast::IncludeUseCase.target`) -- the referenced use case is an
    /// ordinary owned feature (a use case usage), not necessarily a type, so it resolves through
    /// the same `DeclarationDomain::Any` lexical lookup fixed point as `Succession`/
    /// `SatisfySource` rather than the Subclassification/FeatureTyping `Type` domain. Sourced
    /// directly at the enclosing use case declaration (no anonymous nested-declaration scope
    /// shift), mirroring `Variant`'s single-operand shape rather than
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
    /// `Bind`'s braced body are out of scope -- only the two operand references themselves are
    /// resolved.
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
    /// `Variant`'s single-operand shape), since a `stakeholder` member carries
    /// only one operand. The `:>>` redefinition spelling (`is_redefinition == true`) reuses the
    /// existing generic `ReferenceKind::Redefinition` instead of this kind.
    StakeholderTarget,
    /// The `target` concern reference of a viewpoint `purpose` member (`PurposeMember.target`, BNF
    /// `PurposeMember`), same shape and scope as `StakeholderTarget`: a concern reference resolved
    /// through `DeclarationDomain::Any`, sourced directly at the enclosing requirement/viewpoint
    /// declaration (no anonymous nested-declaration scope shift -- a `purpose` member carries only
    /// one operand and introduces no name of its own, mirroring `Variant`).
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
/// publishes no evaluation fact, per `classify_expression`.
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
    /// fabricating a richer "physical quantity" concept the semantic model does not
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

pub(crate) mod element_kind;
pub(crate) mod evaluation;
pub(crate) mod query;
pub(crate) mod render;
pub(crate) mod resolver;

#[cfg(test)]
mod tests {
    use super::*;
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

    fn build_diagnostics_sexpr(source: &str) -> String {
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
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        output
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
        // reuses the exact same `lower_constraint_expression`/`classify_expression`
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

    /// An expression whose shape this engine does not evaluate says so.
    ///
    /// It previously published nothing, which made the declaration indistinguishable from one that
    /// authored no expression at all -- and a consumer asking "does this element have a value" got
    /// the same answer for "there is nothing here" and "there is something here I cannot fold".
    ///
    /// See `constraint_unsupported_expression_shape_still_falls_through_to_diagnostic`: an
    /// invocation and `-`/`not` unary ops are supported (reference-resolvable) shapes, so this uses
    /// `~x` (`UnaryOperator::BitNot`), still genuinely unsupported.
    #[test]
    fn constraint_unsupported_expression_shape_publishes_an_unsupported_evaluation_state() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconstraint def C { ~x }\n\
             }\n",
        );
        assert!(
            output.contains(
                "(evaluated (declaration (node (document \"memory://test/enum.sysml\") \
                 (qualified-name \"Demo::C\"))) (state unsupported))"
            ),
            "expected an unsupported expression shape to publish the explicit unsupported state, \
             got:\n{output}"
        );
        assert!(
            !output.contains("(value "),
            "an unsupported expression must carry no value, got:\n{output}"
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
    fn calc_anonymous_return_decl_arithmetic_evaluates_to_integer() {
        // Slice 5: most real-corpus calc arithmetic lives inside a `return : Type = expr;`
        // declaration, a distinct `CalcDefBodyElement::ReturnDecl` shape bd50fccd (slice 4)
        // deferred. This wires the return declaration's own expression through the exact same
        // classify_expression/lower_calc_expression pipeline slices 1-4 already built.
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
        // evaluates via the same classify_expression/EvalNode::Arithmetic machinery
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
    fn calc_def_body_kinded_parameter_is_recovered_by_the_pinned_parser() {
        // Regression pin, not desired behavior. Through `49bdf3f` a directed KerML-kinded
        // parameter in a calc-shaped body reached the AST as a `KermlFeature` and lowered under
        // the kind its keyword names (`expr` -> `kerml-expression`) with its direction as a
        // declaration fact. At the pinned `f52100fd` the new `in`/`out`/`inout` branch of
        // `parser/constraint.rs` commits to the `InOutDecl` parameter parser and no longer falls
        // back to the KerML feature route, so the member is dropped to parse recovery and nothing
        // is published for it. See planning/UPSTREAM_PARSER_GAPS.md gap 81. The regression is
        // scoped to the SysML `calc`/`constraint`-shaped bodies that route through that branch:
        // the same spelling in a KerML `function`/`behavior` body still parses and lowers, which
        // is why `tests/snapshots/sysml.library/control_functions.md` is unaffected.
        //
        // This pins the loss so it stays visible: the publication must say `parse-recovery`
        // rather than silently publishing a partial model that looks complete.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tcalc def C {\n\
             \t\tin a : Boolean;\n\
             \t\tin expr p : Boolean = a;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(completeness parse-recovery)"),
            "expected the kinded parameter to be reported as parse recovery, got:\n{output}"
        );
        assert!(
            !output.contains("(qualified-name \"Demo::C::p\")"),
            "expected no declaration for the recovered parameter p, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::C::a\")"),
            "expected the plain directed parameter a to still lower, got:\n{output}"
        );
    }

    #[test]
    fn calc_def_body_kinded_parameter_redefinition_is_recovered_by_the_pinned_parser() {
        // The redefinition-only spelling of the same production (`in bool redefines onOccurrence
        // { ... }`, the shape Kernel Semantic Library `Observation.kerml` authors in a KerML
        // function body) is lost the same way in a `calc def` body at the pinned revision,
        // including its nested body. See the sibling test above and
        // planning/UPSTREAM_PARSER_GAPS.md gap 81.
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
            output.contains("(completeness parse-recovery)"),
            "expected the kinded redefinition to be reported as parse recovery, got:\n{output}"
        );
        assert!(
            !output.contains("kerml-boolean-expression"),
            "expected no kerml-boolean-expression declaration for the recovered member, got:\n\
             {output}"
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
