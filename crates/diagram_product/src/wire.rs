//! The schema-5 JSON wire contract. The Rust writer validates against these types before
//! publication, and TypeScript bindings are generated from the same declarations.

use serde::Deserialize;
use ts_rs::TS;

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagramProduct {
    #[ts(type = "5")]
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
    SequenceOrderingCycle,
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
    Browser => ("browser-view", RootsMetadata, BrowserScene),
    Grid => ("grid-view", GridMetadata, GridScene),
    Geometry => ("geometry-view", GeometryMetadata, GeometryScene),
}

#[derive(Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagramNode {
    pub reference: usize,
    pub metaclass: String,
    pub notation_role: NotationRole,
    pub name: Option<String>,
    pub typing: NodeTyping,
    pub owner: Option<usize>,
    pub source: usize,
    pub compartments: Vec<Compartment>,
    pub direction: Option<String>,
    pub conjugated: bool,
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

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct InterconnectionMetadata {
    pub parts: Vec<usize>,
    pub ports: Vec<usize>,
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

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct GridMetadata {
    pub rows: Vec<usize>,
    pub columns: Vec<String>,
    pub cells: Vec<GridCell>,
}

#[derive(Deserialize, TS)]
#[serde(deny_unknown_fields)]
pub struct GridCell {
    pub row: usize,
    pub column: String,
    pub relationship: usize,
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
    pub label: String,
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
