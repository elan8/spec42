//! Host semantic model projection (no LSP transport types).
//!
//! This is a 1:1 representation of the semantic graph in a serializable form.
//! Every field present in [`SemanticNode`] and [`SemanticEdge`] is preserved here;
//! nothing is dropped or summarised. Consumers such as babel42 receive the full
//! semantic model and can filter or present it as needed.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sysml_model::{
    ConnectStatementDetail, ElementKind, FlowStatementDetail, RelationshipKind, TextRange,
};

/// Normative metaclass selected for an addressable projected relationship.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum HostRelationshipMetaclass {
    Membership,
    FeatureTyping,
    /// Definition-level `specializes` / subclassification.
    Subclassification,
    /// Retained as a compatibility alias for older consumers; maps like Subclassification.
    Specialization,
    Subsetting,
    Redefinition,
    /// KerML 8.3.4.4 `ReferenceSubsetting`: `references` / `::>`, a `Subsetting` specialization.
    ReferenceSubsetting,
    /// KerML 8.3.4.5 `CrossSubsetting`: `crosses` / `=>`, a `Subsetting` specialization.
    CrossSubsetting,
    Annotation,
    /// Package `import Namespace::*;` / import-all form.
    NamespaceImport,
    /// Package `import Namespace::Member;` member import.
    MembershipImport,
    /// `alias Name for Target;` membership.
    AliasMembership,
    /// `satisfy` requirement-verification relationship.
    Satisfy,
    /// Requirement/case `subject` relationship.
    Subject,
    #[default]
    Relationship,
}

/// The KerML membership form used to establish containment.  This is kept
/// separate from the graph-resolution `RelationshipKind`: a parent/child
/// relation is a model element in its own right, not merely a display tree.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum HostMembershipKind {
    OwningMembership,
    FeatureMembership,
    Import,
    Alias,
    VariantMembership,
    ActorMembership,
    SubjectMembership,
    StakeholderMembership,
    ObjectiveMembership,
    ParameterMembership,
    ViewRenderingMembership,
    TransitionFeatureMembership,
}

/// Typed, API-oriented identity and ownership facts for a semantic element.
/// These facts deliberately duplicate no display-only `attributes` entries.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostElementFacts {
    pub declared_name: Option<String>,
    pub effective_name: String,
    pub owner_id: Option<String>,
    pub owning_membership_id: Option<String>,
    pub is_library_element: bool,
    /// Documentation comment text lifted from the legacy `doc` attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    /// Declared short name lifted from the legacy `shortName` attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declared_short_name: Option<String>,
    /// Normative Systems Modeling API `@type` for this element when it differs from
    /// the raw [`ElementKind`] spelling (for example `ReferenceUsage` for `ref`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,
    /// Explicit declaration modifiers when the element is a feature or definition
    /// that retained typed parser properties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_properties: Option<HostFeatureProperties>,
    /// Semantic ID of this element's own addressable `Expression` content (in
    /// [`HostSemanticProjection::expressions`]), when the element itself directly *is* an
    /// expression (for example a `TransitionGuard`) rather than merely having one attached via
    /// a `FeatureValue`. Distinct from [`HostFeatureValue::expression_id`], which represents
    /// "this feature's value is X"; this represents "this element's substance is X".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_expression_id: Option<String>,
}

/// Explicit feature/definition modifiers projected from declared semantic facts.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostFeatureProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(default)]
    pub is_abstract: bool,
    #[serde(default)]
    pub is_variation: bool,
    #[serde(default)]
    pub is_individual: bool,
    #[serde(default)]
    pub is_derived: bool,
    #[serde(default)]
    pub is_constant: bool,
    #[serde(default)]
    pub is_end: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_composite: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_reference: Option<bool>,
    #[serde(default)]
    pub is_conjugated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_ordered: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    #[serde(default)]
    pub is_portion: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portion_kind: Option<String>,
}

/// A node in the semantic model — maps 1:1 to [`sysml_model::SemanticNode`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostSemanticModelNode {
    /// Opaque identity of the semantic element in this immutable projection.
    /// It is independent from the element's display and qualified names.
    #[serde(default)]
    pub semantic_id: String,
    /// Document URI the node was declared in.
    pub uri: String,
    /// Fully-qualified name (unique workspace-wide, may include `#kind` disambiguator).
    pub qualified_name: String,
    /// Simple (unqualified) name as written in the source.
    pub name: String,
    /// SysML element kind (typed enum, serialises as its canonical lowercase string).
    pub element_kind: ElementKind,
    /// Source range of the element declaration.
    pub range: TextRange,
    /// Qualified name of the parent element, if any.
    pub parent: Option<String>,
    /// Element-specific attributes extracted during graph construction.
    /// Keys and value shapes are kind-dependent (e.g. `"typeRef"`, `"multiplicity"`,
    /// `"redefines"`, `"evaluatedValue"`, …).
    #[serde(default)]
    pub attributes: HashMap<String, Value>,
    /// Typed identity and containment data consumed by API projections.
    #[serde(default)]
    pub facts: HostElementFacts,
}

/// A directed relationship between two nodes — maps 1:1 to [`sysml_model::SemanticEdge`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostSemanticModelRelationship {
    /// Opaque identity of this addressable relationship element.
    #[serde(default)]
    pub semantic_id: String,
    /// Opaque semantic ID of the relationship source.
    #[serde(default)]
    pub source_id: String,
    /// Opaque semantic ID of the relationship target.
    #[serde(default)]
    pub target_id: String,
    /// Opaque semantic ID of the relationship owner, when present.
    #[serde(default)]
    pub owner_id: Option<String>,
    /// All related elements of this relationship. For binary relationships this
    /// contains the source and target IDs in declaration order.
    #[serde(default)]
    pub related_element_ids: Vec<String>,
    /// Source range of the relationship declaration when the semantic builder
    /// retained one. Absence means the graph fact is resolved but has no
    /// declaration range (for example a derived relationship).
    #[serde(default)]
    pub range: Option<TextRange>,
    /// Whether this relationship was implied rather than explicitly declared.
    #[serde(default)]
    pub is_implied: bool,
    /// Concrete relationship metaclass; `kind` remains the graph-resolution fact.
    #[serde(default)]
    pub metaclass: HostRelationshipMetaclass,
    /// Present exactly for `Membership` metaclasses.
    #[serde(default)]
    pub membership_kind: Option<HostMembershipKind>,
    /// Explicit visibility (`public` / `private` / `protected`) when projected for
    /// import/alias memberships.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// Qualified name of the source node.
    pub source: String,
    /// Qualified name of the target node.
    pub target: String,
    /// Relationship kind (typed enum, serialises as its canonical lowercase string).
    pub kind: RelationshipKind,
    /// Present when this `Connection` edge was resolved from an explicit `connect` statement.
    #[serde(default)]
    pub connect: Option<ConnectStatementDetail>,
    /// Present when this `Flow`/`SuccessionFlow` edge was resolved from a `flow` usage.
    #[serde(default)]
    pub flow: Option<FlowStatementDetail>,
}

/// The complete semantic projection of a workspace — all nodes and addressable
/// relationships from workspace (non-library) documents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HostSemanticProjection {
    pub nodes: Vec<HostSemanticModelNode>,
    pub relationships: Vec<HostSemanticModelRelationship>,
    #[serde(default)]
    pub multiplicities: Vec<HostMultiplicity>,
    #[serde(default)]
    pub expressions: Vec<HostExpression>,
    #[serde(default)]
    pub feature_values: Vec<HostFeatureValue>,
    #[serde(default)]
    pub connector_ends: Vec<HostConnectorEnd>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostMultiplicity {
    pub semantic_id: String,
    pub owner_id: String,
    pub lower_bound_id: Option<String>,
    pub upper_bound_id: Option<String>,
    pub range: TextRange,
    pub is_implied: bool,
    pub is_ordered: bool,
    pub is_unique: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostExpression {
    pub semantic_id: String,
    pub kind: String,
    pub range: TextRange,
    #[serde(default)]
    pub literal: Option<serde_json::Value>,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub operand_ids: Vec<String>,
    #[serde(default)]
    pub arguments: Vec<HostExpressionArgument>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostExpressionArgument {
    pub name: Option<String>,
    pub value_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostFeatureValue {
    pub semantic_id: String,
    pub owner_id: String,
    pub expression_id: String,
    pub kind: String,
    pub range: TextRange,
    pub is_implied: bool,
}

/// One end of a `Connection`/`Interface` connector, addressable and ordered independently of
/// the owning relationship's `source_id`/`target_id` pair -- supports the n-ary
/// `connect (a, b, c, ...)` form, not just the binary `from ... to ...` case.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostConnectorEnd {
    pub semantic_id: String,
    /// Semantic ID of the owning `Connection`/`Interface` relationship.
    pub owner_id: String,
    /// Declared position among the connector's ends (`0` for the first/`from` end, `1` for the
    /// second/`to` end, `2..` for additional n-ary ends). Explicit rather than relying on `Vec`
    /// order, which has no documented meaning once serialized/diffed independently.
    pub end_index: u32,
    /// Semantic ID the end's expression resolves to (e.g. `sensorA.cmd`), when resolution
    /// succeeded.
    pub target_feature_id: Option<String>,
    pub range: TextRange,
    pub is_implied: bool,
}
