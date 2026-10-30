//! The schema-11 JSON wire contract. The Rust writer validates against these types before
//! publication, and TypeScript bindings are generated from the same declarations.

use serde::Deserialize;
use ts_rs::TS;

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagramProduct {
    #[ts(type = "typeof import('./schemaVersion').DIAGRAM_PRODUCT_SCHEMA_VERSION")]
    pub schema_version: u32,
    pub model_digest: String,
    pub documents: Vec<DiagramDocument>,
    pub sources: Vec<DiagramSource>,
    pub references: Vec<SemanticReference>,
    pub selected_view: SelectedView,
    pub completeness: Completeness,
    pub projection: Projection,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagramDocument {
    pub uri: String,
    pub source_domain: SourceDomain,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct DiagramSource {
    pub document: usize,
    pub range: [u32; 4],
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum SourceDomain {
    Workspace,
    StandardLibrary,
    Library,
    External,
}

#[derive(Deserialize, TS)]
#[serde(
    tag = "kind",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub enum SemanticReference {
    QualifiedName {
        document: usize,
        qualified_name: String,
    },
    ToolingElementId {
        element_id: String,
        source_domain: SourceDomain,
    },
    SourceAnchor {
        source: usize,
        owner_qualified_name: Option<String>,
        metaclass: String,
        source_domain: SourceDomain,
    },
    Relationship {
        source: usize,
        relationship_kind: String,
        ordinal: u32,
    },
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelectedView {
    pub reference: usize,
    pub kind: ViewKind,
    pub name: String,
    pub source: usize,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum ViewKind {
    GeneralView,
    InterconnectionView,
    ActionFlowView,
    StateTransitionView,
    SequenceView,
    BrowserView,
    GridView,
    GeometryView,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct Completeness {
    pub status: CompletenessStatus,
    pub reasons: Vec<IncompleteReason>,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum CompletenessStatus {
    Complete,
    Incomplete,
}

#[derive(Deserialize, TS)]
#[serde(
    tag = "code",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub enum IncompleteReason {
    ParseRecovery,
    UnsupportedSyntax,
    NonConverged,
    ExposureUnresolved { exposure: usize },
    ExposureAmbiguous { exposure: usize },
    ExposureUnsupported { exposure: usize },
    RelationshipUnresolved { relationship_kind: String },
    RelationshipAmbiguous { relationship_kind: String },
    RelationshipUnsupported { relationship_kind: String },
    ViewFilterUnresolved,
    ViewFilterAmbiguous,
    ViewFilterUnsupported,
    GeometryFactsUnavailable,
    SequenceMessageEndpointOutsideLifeline,
    SequenceOrderingCycle { witness: Vec<SequenceCycleStep> },
    SequenceContentOmitted { elements: Vec<usize> },
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SequenceCycleStep {
    pub source: usize,
    pub target: usize,
    pub events: Vec<usize>,
    pub provenance: Provenance,
}

macro_rules! projection {
    ($( $variant:ident => ($kind:literal, $metadata:ty, $scene:ty) ),+ $(,)?) => {
        #[derive(Deserialize, TS)]
        #[serde(tag = "kind")]
        pub enum Projection {
            $(
                #[serde(rename = $kind)]
                $variant {
                    #[serde(rename = "exposedRoots")]
                    exposed_roots: Vec<usize>,
                    nodes: Vec<DiagramNode>,
                    relationships: Vec<DiagramRelationship>,
                    edges: Vec<DiagramEdge>,
                    metadata: $metadata,
                    scene: $scene,
                },
            )+
        }
    };
}

projection! {
    General => ("general-view", RootsMetadata, GeneralScene),
    Interconnection => ("interconnection-view", InterconnectionMetadata, InterconnectionScene),
    ActionFlow => ("action-flow-view", ActionFlowMetadata, ActionFlowScene),
    StateTransition => ("state-transition-view", StateTransitionMetadata, StateTransitionScene),
    Sequence => ("sequence-view", SequenceMetadata, SequenceScene),
    Browser => ("browser-view", BrowserMetadata, BrowserScene),
    Grid => ("grid-view", GridMetadata, GridScene),
    Geometry => ("geometry-view", GeometryMetadata, GeometryScene),
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagramNode {
    /// Opaque canonical contextual identity for transferring presentation state. It is not
    /// a semantic lookup handle or a product-local node index.
    pub presentation_key: String,
    pub reference: usize,
    pub metaclass: String,
    pub notation_role: NotationRole,
    pub name: Option<String>,
    pub typing: NodeTyping,
    pub owner: Option<usize>,
    pub source: usize,
    pub compartments: Vec<Compartment>,
    /// The authored multiplicity as written in a label (`"[6]"`, `"[0..*]"`), or `null` when none
    /// was authored or a bound is an expression.
    pub multiplicity: Option<String>,
    pub direction: Option<String>,
    pub conjugated: bool,
    /// `"entry"`, `"do"` or `"exit"` for the action a state owns in that role.
    pub state_subaction: Option<String>,
    /// On a conditional succession: its guard expression as written, or `"else"`.
    pub succession_guard: Option<String>,
    /// On a port: the direction its directed features give it as a whole. Derived.
    pub feature_direction: Option<String>,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum NotationRole {
    Definition,
    Usage,
    ReferenceUsage,
    Namespace,
    Annotation,
    Unsupported,
}

#[derive(Deserialize, TS)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum NodeTyping {
    Absent,
    Resolved { types: Vec<TypeReference> },
    Partial { types: Vec<TypeReference> },
    Ambiguous { candidates: Vec<usize> },
    Unresolved,
    Unsupported,
    Recovery,
    Incomplete,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct TypeReference {
    pub reference: usize,
    pub label: String,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct Compartment {
    pub kind: CompartmentKind,
    pub provenance: CompartmentProvenance,
    pub members: Vec<usize>,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum CompartmentKind {
    Attributes,
    Parts,
    Ports,
    Items,
    Constraints,
    Requirements,
    Actions,
    States,
    Calculations,
    Connections,
    Interfaces,
    Occurrences,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum CompartmentProvenance {
    Direct,
    Inherited,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagramRelationship {
    pub reference: usize,
    pub source: usize,
    pub kind: String,
    pub target: RelationshipTarget,
    pub provenance: Provenance,
    pub navigation: Option<usize>,
}

#[derive(Deserialize, TS)]
#[serde(untagged)]
pub enum RelationshipTarget {
    ResolvedNode {
        status: ResolvedStatus,
        node: usize,
    },
    ResolvedReference {
        status: ResolvedStatus,
        reference: usize,
    },
    Ambiguous {
        status: AmbiguousStatus,
        candidates: Vec<usize>,
    },
    Unresolved {
        status: UnresolvedStatus,
    },
    Unsupported {
        status: UnsupportedStatus,
    },
}

macro_rules! status {
    ($name:ident, $variant:ident, $wire:literal) => {
        #[derive(Deserialize, TS)]
        pub enum $name {
            #[serde(rename = $wire)]
            $variant,
        }
    };
}
status!(ResolvedStatus, Resolved, "resolved");
status!(AmbiguousStatus, Ambiguous, "ambiguous");
status!(UnresolvedStatus, Unresolved, "unresolved");
status!(UnsupportedStatus, Unsupported, "unsupported");

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagramEdge {
    pub reference: usize,
    pub source: usize,
    pub target: usize,
    pub origin: usize,
    pub kind: String,
    pub provenance: Provenance,
    pub navigation: Option<usize>,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum Provenance {
    Authored,
    Implied,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct RootsMetadata {
    pub roots: Vec<usize>,
}

/// A browser projects its exposed roots and the branches the client has opened; each row says how
/// it belongs to its parent and whether its own members are loaded, still to load, or absent.
#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct BrowserMetadata {
    pub roots: Vec<usize>,
    pub rows: Vec<BrowserRow>,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct BrowserRow {
    pub node: usize,
    /// The row's stable key: sent back with `spec42/diagram` to load this row's members.
    pub key: String,
    pub membership: Membership,
    pub members: BrowserMembers,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum Membership {
    Owned,
    Inherited,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum BrowserMembers {
    None,
    Loaded,
    Unloaded,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InterconnectionMetadata {
    pub parts: Vec<usize>,
    pub ports: Vec<usize>,
    pub connectors: Vec<usize>,
    /// Connectors grouped by the family of related port definitions their ends share, in
    /// family-name order; a connector in no family has no common port type.
    pub port_families: Vec<PortFamily>,
}

/// One port family: its naming (most general) types, every connector port type in it, and its
/// connector nodes.
#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct PortFamily {
    pub general: Vec<TypeReference>,
    pub types: Vec<TypeReference>,
    pub connectors: Vec<usize>,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActionFlowMetadata {
    pub actions: Vec<usize>,
    pub control_nodes: Vec<usize>,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StateTransitionMetadata {
    pub states: Vec<usize>,
    pub initial_nodes: Vec<usize>,
    pub final_nodes: Vec<usize>,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct SequenceMetadata {
    pub participants: Vec<usize>,
    pub messages: Vec<usize>,
}

/// How a Grid View arranges its exposed elements (SysML v2 9.2.20.2.5).
#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum GridLayout {
    /// Rows are the source ends of the relationships between exposed elements, columns their
    /// target ends, and each cell one relationship.
    RelationshipMatrix,
    /// One row for each exposed element; no relationship joins two of them.
    ElementTable,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct GridMetadata {
    pub layout: GridLayout,
    /// Node indexes, in exposure order.
    pub rows: Vec<usize>,
    /// Node indexes, in exposure order; empty for an element table.
    pub columns: Vec<usize>,
    pub cells: Vec<GridCell>,
}

/// One cell of a relationship matrix: the relationship edge from the `row` node to the `column`
/// node.
#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct GridCell {
    pub row: usize,
    pub column: usize,
    /// Index into the projection's edges.
    pub edge: usize,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct GeometryMetadata {
    pub elements: Vec<usize>,
    pub primitives: Vec<usize>,
}

macro_rules! marker_scene {
    ($( $scene:ident => ($kind:ident, $wire:literal) ),+ $(,)?) => {
        $(
            #[derive(Deserialize, TS)]
            #[serde(deny_unknown_fields)]
            pub struct $scene { pub kind: $kind }
            #[derive(Deserialize, TS)]
            pub enum $kind { #[serde(rename = $wire)] Value }
        )+
    };
}

marker_scene! {
    GeneralScene => (GeneralSceneKind, "general"),
    InterconnectionScene => (InterconnectionSceneKind, "interconnection"),
    ActionFlowScene => (ActionFlowSceneKind, "action-flow"),
    BrowserScene => (BrowserSceneKind, "browser"),
    GridScene => (GridSceneKind, "grid"),
    GeometryScene => (GeometrySceneKind, "geometry"),
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct SequenceScene {
    pub kind: SequenceSceneKind,
    pub lifelines: Vec<usize>,
    pub messages: Vec<SequenceMessage>,
}
status!(SequenceSceneKind, Sequence, "sequence");

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct SequenceMessage {
    pub node: usize,
    pub label: Option<String>,
    pub source: SequenceEndpoint,
    pub target: SequenceEndpoint,
    pub order: SequenceOrder,
    pub provenance: Provenance,
    pub navigation: usize,
}

#[derive(Deserialize, TS)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum SequenceEndpoint {
    Resolved { lifeline: usize },
    Ambiguous,
    Unresolved,
    Unsupported,
    OutsideLifeline,
}

#[derive(Deserialize, TS)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum SequenceOrder {
    Resolved { value: u32 },
    Cyclic,
    BlockedByCycle,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct StateTransitionScene {
    pub kind: StateTransitionSceneKind,
    pub frame: Option<StateFrame>,
    pub vertices: Vec<StateVertex>,
    pub transitions: Vec<StateTransition>,
}
status!(
    StateTransitionSceneKind,
    StateTransition,
    "state-transition"
);

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct StateFrame {
    pub id: String,
    pub label: String,
    pub navigation: usize,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct StateVertex {
    pub id: String,
    pub label: String,
    pub kind: StateVertexKind,
    pub navigation: usize,
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "kebab-case")]
pub enum StateVertexKind {
    Initial,
    State,
    Final,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct StateTransition {
    pub id: String,
    pub label: Option<String>,
    pub source: usize,
    pub target: usize,
    pub trigger: TransitionTrigger,
    pub guard: ProjectionFeature,
    pub effect: ProjectionFeature,
    pub provenance: Provenance,
    pub navigation: usize,
}

#[derive(Deserialize, TS)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum TransitionTrigger {
    Absent,
    Accept {
        label: String,
        target: Option<TransitionTarget>,
        navigation: usize,
    },
    Unsupported {
        code: String,
        message: String,
    },
    Unresolved,
    Ambiguous,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct TransitionTarget {
    pub id: String,
    pub label: String,
}

#[derive(Deserialize, TS)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum ProjectionFeature {
    Absent,
    Supported { label: String, navigation: usize },
    Unsupported { code: String, message: String },
    Unresolved,
    Ambiguous,
    Recovery,
}
