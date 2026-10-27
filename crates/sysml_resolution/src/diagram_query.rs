use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{
    ElementDetails, ElementKind, ElementSearch, ElementSource, FeatureDirection,
    PublishedResolution, QueryAnswer, QueryOutcome, RelationshipOutcome, RelationshipProvenance,
    RelationshipTarget, SourceLocation, StateSubactionKind, SymbolEntry, SymbolId,
    ViewExposureObstacle, ViewSelectionObstacle, ViewSelectionOutcome,
};

pub use sysml_contract::{
    DiagramCompartmentKind, DiagramCompartmentProvenance, DiagramRelationshipKind,
    DiagramStateVertexKind, DiagramViewKind,
};

/// The standard library declaration name each view kind is defined by.
///
/// Deliberately not a method on [`DiagramViewKind`]: matching a projected catalog entry against a
/// library declaration is how *this* authority finds a view definition, not part of the vocabulary
/// a consumer is handed.
const fn definition_name(kind: DiagramViewKind) -> &'static str {
    match kind {
        DiagramViewKind::General => "GeneralView",
        DiagramViewKind::Interconnection => "InterconnectionView",
        DiagramViewKind::ActionFlow => "ActionFlowView",
        DiagramViewKind::StateTransition => "StateTransitionView",
        DiagramViewKind::Sequence => "SequenceView",
        DiagramViewKind::Browser => "BrowserView",
        DiagramViewKind::Grid => "GridView",
        DiagramViewKind::Geometry => "GeometryView",
    }
}

/// One authored standard view usage, as the catalog publishes it.
///
/// The display name is not carried. It is the view usage's authored name -- or, where the usage
/// is anonymous, its `::`-joined display path -- both of which the publication already stores.
/// A catalog of every authored view would allocate one copy per entry for text only a picker
/// renders; a consumer reads it with [`PublishedResolution::diagram_view_name`] instead, which
/// borrows and applies the same fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramViewCatalogEntry {
    pub kind: DiagramViewKind,
    pub semantic_id: SymbolId,
    pub reference: DiagramSemanticReference,
    pub source: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramSemanticReference {
    Qualified {
        document: Box<str>,
        qualified_name: Box<str>,
    },
    SourceAnchor {
        document: Box<str>,
        owner_qualified_name: Option<Box<str>>,
        kind: ElementKind,
        range: crate::TextRange,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagramIncompleteReason {
    ParseRecovery,
    UnsupportedSyntax,
    NonConverged,
    ExposureUnresolved {
        exposure: SymbolId,
    },
    ExposureAmbiguous {
        exposure: SymbolId,
    },
    ExposureUnsupported {
        exposure: SymbolId,
    },
    RelationshipUnresolved {
        relationship: DiagramRelationshipKind,
    },
    RelationshipAmbiguous {
        relationship: DiagramRelationshipKind,
    },
    RelationshipUnsupported {
        relationship: DiagramRelationshipKind,
    },
    ViewFilterUnresolved,
    ViewFilterAmbiguous,
    ViewFilterUnsupported,
    GeometryFactsUnavailable,
    SequenceMessageEndpointOutsideLifeline,
    SequenceOrderingCycle {
        witness: Box<[DiagramSequenceCycleStep]>,
    },
    SequenceContentOmitted {
        elements: Box<[DiagramOccurrenceIdentity]>,
    },
}

/// One edge of an actual closed precedence path, retained by the projection owner.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiagramSequenceCycleStep {
    pub source: DiagramOccurrenceIdentity,
    pub target: DiagramOccurrenceIdentity,
    pub events: Box<[DiagramOccurrenceIdentity]>,
    pub authored: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramElement {
    /// Identity of this element occurrence in the projected containment context.
    ///
    /// This is deliberately distinct from `semantic_id`: one inherited feature declaration may
    /// occur below several typed usages in the same diagram. Consumers use this identity for
    /// layout, expansion and containment, and use `semantic_id` for navigation and model queries.
    pub occurrence_id: DiagramOccurrenceIdentity,
    pub semantic_id: SymbolId,
    pub reference: DiagramSemanticReference,
    pub kind: ElementKind,
    /// KerML effectiveName for display, including an unnamed redefinition's inherited name.
    /// Authored identification and naming provenance remain owned by the publication.
    pub name: Option<Box<str>>,
    /// Authored FeatureTyping for compact labels; not the effective-type closure.
    pub typing: DiagramElementTyping,
    pub owner: Option<DiagramOccurrenceIdentity>,
    pub source: SourceLocation,
    pub compartments: Box<[DiagramCompartment]>,
    /// The authored multiplicity of this element's declaration (`joints : Motor[6]`), which its
    /// label shows. `Absent` when no `[...]` was written.
    pub multiplicity: crate::MultiplicityFacts,
    /// The authored direction (`in` / `out` / `inout`) of a port or of a directed feature such
    /// as a parameter of an action. `None` for an element with no authored direction; a
    /// renderer must not derive a side from this element's name or its owner's when it is
    /// `None`.
    pub direction: Option<FeatureDirection>,
    /// Whether a port's authored typing conjugates the definition it names (`port p : ~PD;`).
    /// `false` for a port with no conjugated typing and for every non-port element.
    pub conjugated: bool,
    /// Whether this is the entry, do or exit action of the state that owns it. The three are
    /// action usages that only their membership tells apart, so it is carried here; `None` for
    /// every other element.
    pub state_subaction: Option<StateSubactionKind>,
    /// On a conditional succession: when it is taken. `None` for every other element.
    pub succession_guard: Option<DiagramSuccessionGuard>,
    /// On a port: the direction its directed features give it as a whole, with a conjugated
    /// typing applied. Derived from the port's own features and those of its definitions, and
    /// kept apart from the authored `direction`. `None` when it has no directed feature.
    pub feature_direction: Option<FeatureDirection>,
    /// In a browser view: how this row is a member of its parent and whether its own members
    /// are in the projection. `None` in every other view kind.
    pub browser_row: Option<DiagramBrowserRow>,
}

/// How a browser row belongs to the row above it (SysML 9.2.20.2.2 presents membership, which
/// for a type includes the members it inherits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagramMembership {
    Owned,
    Inherited,
}

/// Whether a browser row's own members are in the projection. A browser loads its exposed roots
/// and their members; further branches load when the client asks for them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagramBrowserMembers {
    None,
    Loaded,
    Unloaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagramBrowserRow {
    pub membership: DiagramMembership,
    pub members: DiagramBrowserMembers,
}

/// When a conditional succession is taken (SysML 7.17.5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramSuccessionGuard {
    /// The guard expression as it is written, with its white space collapsed.
    Condition(Box<str>),
    /// `else`: taken when the guards of the conditional successions before it are all false.
    Else,
}

/// A stable, contextual identity for one occurrence in a diagram projection.
///
/// The path starts at an exposed semantic root and ends at the declaration presented at this
/// occurrence. Keeping the canonical semantic identities in the path makes the identity
/// deterministic without deriving semantics from names or rendered labels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagramOccurrenceIdentity {
    pub semantic_path: Box<[SymbolId]>,
    /// The length-prefixed concatenation of the path's boundary tokens.
    ///
    /// Diagram scene ids are published to generators and reports, so the key is the token text,
    /// not the handles. It is settled once when the occurrence is created -- where the
    /// publication is in hand -- rather than re-derived at each of the places that format a scene
    /// id. Ordered after `semantic_path` so the derived `Ord` still compares paths, which is the
    /// same order: a handle sorts as its token does.
    key: Box<str>,
}

impl DiagramOccurrenceIdentity {
    fn root(semantic_id: SymbolId, token: &str) -> Self {
        let mut key = String::new();
        push_key_segment(&mut key, token);
        Self {
            semantic_path: vec![semantic_id].into_boxed_slice(),
            key: key.into(),
        }
    }

    fn child(&self, semantic_id: SymbolId, token: &str) -> Self {
        let mut path = self.semantic_path.to_vec();
        path.push(semantic_id);
        let mut key = self.key.to_string();
        push_key_segment(&mut key, token);
        Self {
            semantic_path: path.into_boxed_slice(),
            key: key.into(),
        }
    }

    fn contains(&self, semantic_id: SymbolId) -> bool {
        self.semantic_path.contains(&semantic_id)
    }

    fn semantic_id(&self) -> SymbolId {
        *self
            .semantic_path
            .last()
            .expect("a diagram occurrence path is never empty")
    }

    /// The settled boundary form of this occurrence.
    pub fn stable_key(&self) -> &str {
        &self.key
    }
}

fn push_key_segment(key: &mut String, token: &str) {
    key.push_str(&token.len().to_string());
    key.push(':');
    key.push_str(token);
}

/// Authored FeatureTyping of a projected element, for compact node labels.
///
/// This is the declaration's [`crate::details::ElementDetails::typing`] family, not its
/// effective-type closure. Implied library types inherited through subsetting `Parts::parts`,
/// `Items::items`, and the rest of the kernel chain belong in inspectors and queries, not on
/// every diagram node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramElementTyping {
    Absent,
    Resolved(Box<[SymbolId]>),
    Partial(Box<[SymbolId]>),
    Ambiguous(Box<[SymbolId]>),
    Unresolved,
    Unsupported,
    Recovery,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramCompartment {
    pub kind: DiagramCompartmentKind,
    pub provenance: DiagramCompartmentProvenance,
    pub members: Box<[DiagramOccurrenceIdentity]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramRelationshipTarget {
    Resolved(DiagramRelationshipEndpoint),
    Ambiguous(Box<[DiagramRelationshipEndpoint]>),
    Unresolved,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramRelationshipEndpoint {
    pub semantic_id: SymbolId,
    pub occurrence: DiagramEndpointOccurrence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramEndpointOccurrence {
    Resolved(DiagramOccurrenceIdentity),
    Ambiguous(Box<[DiagramOccurrenceIdentity]>),
    OutsideProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramRelationship {
    /// Position of this relationship among the source element's published relationships.
    pub ordinal: u32,
    pub source: DiagramOccurrenceIdentity,
    pub source_semantic_id: SymbolId,
    pub kind: DiagramRelationshipKind,
    pub target: DiagramRelationshipTarget,
    pub provenance: RelationshipProvenance,
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramEdgeKind {
    Containment,
    Connector,
    Flow,
    Succession,
    Transition,
    InitialState,
    Relationship(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramEdge {
    /// Index into the projection's `elements` of the element this edge was composed for: the
    /// contained element for containment, the state for an initial-state edge, the transition or
    /// connector usage for a composed edge.
    pub origin: u32,
    pub source: DiagramOccurrenceIdentity,
    pub source_semantic_id: SymbolId,
    pub target: DiagramOccurrenceIdentity,
    pub target_semantic_id: SymbolId,
    pub kind: DiagramEdgeKind,
    pub provenance: RelationshipProvenance,
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramScene {
    General,
    Interconnection,
    ActionFlow,
    StateTransition(DiagramStateTransitionScene),
    Sequence(DiagramSequenceScene),
    Browser,
    Grid,
    Geometry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramSequenceScene {
    pub lifelines: Box<[DiagramOccurrenceIdentity]>,
    pub messages: Box<[DiagramSequenceMessage]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramSequenceMessage {
    /// Index into the projection's `elements` of the message usage.
    pub origin: u32,
    pub label: Option<Box<str>>,
    pub source: DiagramSequenceEndpoint,
    pub target: DiagramSequenceEndpoint,
    pub order: DiagramSequenceOrder,
    pub provenance: RelationshipProvenance,
    pub source_location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramSequenceEndpoint {
    Resolved(DiagramOccurrenceIdentity),
    Ambiguous,
    Unresolved,
    Unsupported,
    OutsideLifeline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramSequenceOrder {
    Resolved(u32),
    Cyclic,
    BlockedByCycle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramStateTransitionScene {
    pub machine: Option<SymbolId>,
    pub vertices: Box<[DiagramStateVertex]>,
    pub transitions: Box<[DiagramStateTransition]>,
}

/// One vertex of a state-transition scene.
///
/// The label is not carried: it is the element's authored name, which the publication already
/// stores. A consumer that renders one reads it with [`PublishedResolution::symbol_name`], which
/// borrows from the settled symbol blob, rather than being handed a copy per vertex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagramStateVertex {
    pub semantic_id: SymbolId,
    pub kind: DiagramStateVertexKind,
    pub source: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagramTransitionFeature {
    Absent,
    Resolved {
        label: Box<str>,
        target: SymbolId,
        source: SourceLocation,
    },
    Unresolved,
    Ambiguous,
    Unsupported,
}

/// Whether a state-transition edge is the machine's initial-state edge or an authored transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagramTransitionRole {
    Initial,
    Transition,
}

impl DiagramTransitionRole {
    /// The suffix a scene id carries for this role.
    pub const fn scene_suffix(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::Transition => "edge",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramStateTransition {
    /// Index into the projection's `elements` of the element the transition was composed for.
    pub origin: u32,
    pub role: DiagramTransitionRole,
    pub label: Option<Box<str>>,
    pub source: SymbolId,
    pub target: SymbolId,
    pub trigger: DiagramTransitionFeature,
    pub guard: DiagramTransitionFeature,
    pub effect: DiagramTransitionFeature,
    pub provenance: RelationshipProvenance,
    pub source_location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramViewProjection {
    pub view: DiagramViewCatalogEntry,
    pub exposed_roots: Box<[SymbolId]>,
    pub elements: Box<[DiagramElement]>,
    pub relationships: Box<[DiagramRelationship]>,
    pub edges: Box<[DiagramEdge]>,
    pub scene: DiagramScene,
    pub incomplete_reasons: Box<[DiagramIncompleteReason]>,
}

impl PublishedResolution {
    /// The occurrence identity of an exposed root, as the projection spells it.
    ///
    /// An occurrence carries its published scene key, which is derived from boundary tokens, so
    /// only the publication can mint one. A consumer that has a root handle and wants the
    /// occurrence the projection uses for it asks here instead of assembling the path itself.
    pub fn diagram_root_occurrence(&self, root: SymbolId) -> Option<DiagramOccurrenceIdentity> {
        let token = self.symbol_token(root)?;
        Some(DiagramOccurrenceIdentity::root(root, token.as_str()))
    }

    /// The display name of one catalogued diagram view, borrowed from this publication.
    ///
    /// The authored name where the view usage has one, and its `::`-joined display path where it
    /// is anonymous. The fallback is the authority's rule, not a consumer's, so it lives here
    /// rather than being re-decided at each edge that renders a view picker.
    pub fn diagram_view_name(&self, view: SymbolId) -> Option<&str> {
        self.symbol_name(view).or_else(|| self.qualified_name(view))
    }

    pub fn diagram_view_catalog(&self) -> QueryOutcome<Box<[DiagramViewCatalogEntry]>> {
        let mut definitions = BTreeMap::new();
        for kind in DiagramViewKind::ALL {
            let entries = resolved_values(self.search_elements(ElementSearch {
                kind: ElementKind::ViewDefinition,
                source: ElementSource::StandardLibrary,
            }));
            let matches = entries
                .iter()
                .filter(|entry| entry.name.as_deref() == Some(definition_name(kind)))
                .collect::<Vec<_>>();
            if matches.len() > 1 {
                return self
                    .model
                    .query_outcome(QueryAnswer::Ambiguous(Box::new([])));
            }
            if let Some(entry) = matches.first() {
                definitions.insert(entry.identity, kind);
            }
        }

        let mut catalog = Vec::new();
        for entry in resolved_values(self.search_elements(ElementSearch {
            kind: ElementKind::ViewUsage,
            source: ElementSource::Workspace,
        })) {
            let types = resolved_values(self.direct_types(entry.identity));
            let supertypes = |symbol| {
                resolved_values(
                    self.all_supertypes(symbol, crate::SpecializationScope::Subclassification),
                )
            };
            // Each authored type contributes the most specific standard view definitions it
            // conforms to. `StateTransitionView` and `ActionFlowView` specialize
            // `InterconnectionView`; a view typed by one of them is that view, not also an
            // interconnection view.
            let kinds = types
                .iter()
                .flat_map(|ty| {
                    let standard = std::iter::once(ty.symbol)
                        .chain(supertypes(ty.symbol).iter().copied())
                        .filter(|symbol| definitions.contains_key(symbol))
                        .collect::<BTreeSet<_>>();
                    standard
                        .iter()
                        .filter(|general| {
                            !standard.iter().any(|specific| {
                                specific != *general && supertypes(*specific).contains(general)
                            })
                        })
                        .map(|definition| definitions[definition])
                        .collect::<Vec<_>>()
                })
                .collect::<BTreeSet<_>>();
            for kind in kinds {
                catalog.push(DiagramViewCatalogEntry {
                    kind,
                    semantic_id: entry.identity,
                    reference: semantic_reference(
                        &entry,
                        self.qualified_name(entry.identity).unwrap_or_default(),
                        entry.owner.and_then(|owner| self.qualified_name(owner)),
                        self.document_identity(entry.location.document)
                            .unwrap_or_default(),
                    ),
                    source: entry.location,
                });
            }
        }
        // A handle sorts as its canonical identity does, so this is the identity order the
        // catalog has always published, without materialising either string.
        catalog.sort_by_key(|a| a.semantic_id);
        self.model
            .query_outcome(QueryAnswer::Resolved(catalog.into_boxed_slice()))
    }

    /// The projection of `view` as its first catalogued kind. A view typed by more than one
    /// standard view definition has one catalog entry per kind; `diagram_view_of_kind` projects a
    /// chosen one.
    pub fn diagram_view(&self, view: SymbolId) -> QueryOutcome<DiagramViewProjection> {
        self.project_diagram_view(view, None, &|_| false)
    }

    /// The projection of `view`, as the catalogued `kind` when one is given, with the browser
    /// branches `expanded` names loaded. `expanded` is asked about the stable key of each
    /// occurrence (see [`DiagramOccurrenceIdentity::stable_key`]); other view kinds ignore it.
    pub fn diagram_view_expanding(
        &self,
        view: SymbolId,
        kind: Option<DiagramViewKind>,
        expanded: &dyn Fn(&str) -> bool,
    ) -> QueryOutcome<DiagramViewProjection> {
        self.project_diagram_view(view, kind, expanded)
    }

    /// The projection of `view` as the catalogued `kind`: `Unresolved` if `view` is not
    /// catalogued with that kind.
    pub fn diagram_view_of_kind(
        &self,
        view: SymbolId,
        kind: DiagramViewKind,
    ) -> QueryOutcome<DiagramViewProjection> {
        self.project_diagram_view(view, Some(kind), &|_| false)
    }

    fn project_diagram_view(
        &self,
        view: SymbolId,
        kind: Option<DiagramViewKind>,
        expanded: &dyn Fn(&str) -> bool,
    ) -> QueryOutcome<DiagramViewProjection> {
        let catalog = match self.diagram_view_catalog().answer {
            QueryAnswer::Resolved(value) => value,
            QueryAnswer::Ambiguous(_) => {
                return self
                    .model
                    .query_outcome(QueryAnswer::Ambiguous(Box::new([])));
            }
            QueryAnswer::Unsupported => {
                return self.model.query_outcome(QueryAnswer::Unsupported);
            }
            QueryAnswer::Unresolved => return self.model.query_outcome(QueryAnswer::Unresolved),
            QueryAnswer::Recovery => return self.model.query_outcome(QueryAnswer::Recovery),
            QueryAnswer::Incomplete => return self.model.query_outcome(QueryAnswer::Incomplete),
        };
        let Some(view_entry) = catalog
            .iter()
            .find(|entry| entry.semantic_id == view && kind.is_none_or(|kind| entry.kind == kind))
            .cloned()
        else {
            return self.model.query_outcome(QueryAnswer::Unresolved);
        };

        let all = self.diagram_entries();
        let workspace = self
            .diagram_entries_for(ElementSource::Workspace)
            .into_keys()
            .collect::<BTreeSet<_>>();
        // Non-convergence is a property of the whole solve, so every view inherits it. Parser
        // recovery and unsupported syntax are attributed to their documents and reach a view only
        // through the documents it draws from (below), so an unrelated broken file does not mark
        // every view incomplete.
        let mut reasons = self
            .model
            .completeness()
            .obstacles()
            .filter(|obstacle| matches!(obstacle, crate::PublicationObstacle::NonConverged))
            .map(publication_obstacle_reason)
            .collect::<BTreeSet<_>>();
        let exposed = match self.view_exposed_elements(view).answer {
            QueryAnswer::Resolved(exposed) => exposed,
            QueryAnswer::Ambiguous(_) => {
                return self
                    .model
                    .query_outcome(QueryAnswer::Ambiguous(Box::new([])));
            }
            QueryAnswer::Unsupported => return self.model.query_outcome(QueryAnswer::Unsupported),
            QueryAnswer::Unresolved => return self.model.query_outcome(QueryAnswer::Unresolved),
            QueryAnswer::Recovery => return self.model.query_outcome(QueryAnswer::Recovery),
            QueryAnswer::Incomplete => return self.model.query_outcome(QueryAnswer::Incomplete),
        };
        let exposed_set = exposed.elements.iter().copied().collect::<BTreeSet<_>>();
        // A recursive expose also exposes everything nested in its target. Those elements are
        // drawn inside their exposed owner, so only an element with no exposed ancestor is a root.
        let roots = exposed_set
            .iter()
            .copied()
            .filter(|element| {
                let mut owner = all.get(element).and_then(|entry| entry.owner);
                while let Some(current) = owner {
                    if exposed_set.contains(&current) {
                        return false;
                    }
                    owner = all.get(&current).and_then(|entry| entry.owner);
                }
                true
            })
            .collect::<BTreeSet<_>>();
        for obstacle in exposed.obstacles {
            reasons.insert(match obstacle {
                ViewExposureObstacle::ExposureUnresolved { exposure } => {
                    DiagramIncompleteReason::ExposureUnresolved { exposure }
                }
                ViewExposureObstacle::ExposureAmbiguous { exposure, .. } => {
                    DiagramIncompleteReason::ExposureAmbiguous { exposure }
                }
                ViewExposureObstacle::ExposureUnsupported { exposure } => {
                    DiagramIncompleteReason::ExposureUnsupported { exposure }
                }
                ViewExposureObstacle::FilterUnresolved { .. } => {
                    DiagramIncompleteReason::ViewFilterUnresolved
                }
                ViewExposureObstacle::FilterAmbiguous { .. } => {
                    DiagramIncompleteReason::ViewFilterAmbiguous
                }
                ViewExposureObstacle::FilterUnsupported { .. } => {
                    DiagramIncompleteReason::ViewFilterUnsupported
                }
            });
        }

        let mut direct_children = BTreeMap::<SymbolId, Vec<SymbolId>>::new();
        for entry in all.values() {
            if let Some(owner) = &entry.owner {
                direct_children
                    .entry(*owner)
                    .or_default()
                    .push(entry.identity);
            }
        }
        for children in direct_children.values_mut() {
            children.sort();
            children.dedup();
        }
        let mut occurrences = BTreeMap::<
            DiagramOccurrenceIdentity,
            (SymbolId, Option<DiagramOccurrenceIdentity>),
        >::new();
        // Each connector with the resolved feature path of each of its ends.
        let mut connected_ports: Vec<(SymbolId, Vec<SymbolId>)> = Vec::new();
        if view_entry.kind == DiagramViewKind::Interconnection {
            // Reuse the canonical connector paths, including intermediate port hops.
            // Do not expand generic library collections such as Port::subports as if
            // they were contextual physical ports.
            for connector in self.model.all_connectors().iter() {
                for end in connector.ends.iter() {
                    use crate::connection_query::ConnectorEndpoint;
                    match &end.endpoint {
                        ConnectorEndpoint::Feature(RelationshipTarget::Resolved(target)) => {
                            connected_ports.push((connector.identity, vec![*target]));
                        }
                        ConnectorEndpoint::FeatureChain { path, .. } => {
                            let resolved = path
                                .iter()
                                .map_while(|target| match target {
                                    RelationshipTarget::Resolved(target) => Some(*target),
                                    _ => None,
                                })
                                .collect::<Vec<_>>();
                            if !resolved.is_empty() {
                                connected_ports.push((connector.identity, resolved));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        let mut queue = VecDeque::new();
        for root in &roots {
            // The scene id of an occurrence is published text, so the boundary token is taken
            // here, once, where the publication is in hand.
            let Some(token) = self.symbol_token(*root) else {
                continue;
            };
            let occurrence = DiagramOccurrenceIdentity::root(*root, token.as_str());
            occurrences.insert(occurrence.clone(), (*root, None));
            queue.push_back(occurrence);
        }
        let mut browser_members =
            BTreeMap::<DiagramOccurrenceIdentity, DiagramBrowserMembers>::new();
        let mut browser_membership =
            BTreeMap::<DiagramOccurrenceIdentity, DiagramMembership>::new();
        while let Some(owner_occurrence) = queue.pop_front() {
            // SysML 9.2.20.2.5: a Grid View arranges the exposed elements and their
            // relationships; their members are not rows or columns of their own.
            if view_entry.kind == DiagramViewKind::Grid {
                continue;
            }
            let owner = occurrences
                .get(&owner_occurrence)
                .expect("queued diagram occurrence must exist")
                .0;
            // Owned and inherited members: a usage also has the features of its types. A browser
            // presents that membership hierarchy, but only as far as it has been opened -- every
            // branch at once repeated each type's members under every usage, thousands of rows
            // for a modest package.
            let mut children = resolved_values(self.effective_features(owner)).to_vec();
            children.extend(
                direct_children
                    .get(&owner)
                    .into_iter()
                    .flatten()
                    .filter_map(|identity| all.get(identity).cloned()),
            );
            children.sort_by_key(|left| left.identity);
            children.dedup_by(|left, right| left.identity == right.identity);
            // An action flow names nodes its action inherits from the library, the `start`
            // and `done` of `Actions::Action`, as the ends of its successions. Such a feature
            // is a node of the flow where a succession of this owner begins or ends at it.
            let library_ends = if view_entry.kind == DiagramViewKind::ActionFlow {
                self.library_succession_ends(&children, &workspace)
            } else {
                BTreeSet::new()
            };
            let owned_connectors = children
                .iter()
                .map(|child| child.identity)
                .collect::<BTreeSet<_>>();
            // A browser loads the members of its exposed roots and of the branches the client
            // has opened; of any other row it only records that members are there to load.
            let load_members = view_entry.kind != DiagramViewKind::Browser
                || owner_occurrence.semantic_path.len() == 1
                || expanded(owner_occurrence.stable_key());
            let mut has_members = false;
            for child in &children {
                // An inherited library port is still a port of this contextual part. Omitting
                // it removes resolved connector endpoints (e.g. a purchased sensor's data). It
                // is connected in this occurrence only: an end path that ends at this occurrence
                // (`sensor.data`), or a bare end of a connector this occurrence owns. The same
                // port inherited by another part stays hidden there.
                let inherited_port = view_entry.kind == DiagramViewKind::Interconnection
                    && child.kind == ElementKind::PortUsage
                    && {
                        let mut path = owner_occurrence.semantic_path.to_vec();
                        path.push(child.identity);
                        connected_ports.iter().any(|(connector, end)| {
                            (1..=end.len()).any(|length| {
                                let hops = &end[..length];
                                hops.last() == Some(&child.identity)
                                    && if length == 1 {
                                        owned_connectors.contains(connector)
                                    } else {
                                        path.ends_with(hops)
                                    }
                            })
                        })
                    };
                if !workspace.contains(&child.identity)
                    && !library_ends.contains(&child.identity)
                    && !inherited_port
                {
                    continue;
                }
                // Lowering structure is not part of the browsed model: the ends a `flow` declares
                // for its endpoints, the anonymous features under them, and value expressions,
                // which the attribute that owns them already presents.
                if view_entry.kind == DiagramViewKind::Browser
                    && (child.kind == ElementKind::FlowEnd
                        || child.kind.conforms_to(ElementKind::Expression))
                {
                    continue;
                }
                // Nor is it a node of any other view: a usage's multiplicity (`joints : Motor[6]`)
                // and a KerML expression, with the result features beneath them, are parts of a
                // declaration that its own notation shows (SysML 8.2.3.6 `UsageDeclaration`). A
                // SysML usage that is an expression -- a calculation or a constraint -- is a node.
                if is_declaration_structure(child.kind) {
                    continue;
                }
                if !self.diagram_candidate_selected(view, child.identity, &mut reasons) {
                    continue;
                }
                has_members = true;
                if !load_members {
                    continue;
                }
                let Some(token) = self.symbol_token(child.identity) else {
                    continue;
                };
                let occurrence = owner_occurrence.child(child.identity, token.as_str());
                if view_entry.kind == DiagramViewKind::Browser {
                    browser_membership.insert(
                        occurrence.clone(),
                        if child.owner == Some(owner) {
                            DiagramMembership::Owned
                        } else {
                            DiagramMembership::Inherited
                        },
                    );
                }
                let inserted = occurrences
                    .insert(
                        occurrence.clone(),
                        (child.identity, Some(owner_occurrence.clone())),
                    )
                    .is_none();
                // Recursive types have an unbounded semantic instance tree. Present the cycle-closing
                // occurrence, but do not invent an arbitrary depth beyond the first repeated declaration.
                if inserted && !owner_occurrence.contains(child.identity) {
                    queue.push_back(occurrence);
                }
            }
            if view_entry.kind == DiagramViewKind::Browser {
                browser_members.insert(
                    owner_occurrence.clone(),
                    match (has_members, load_members) {
                        (false, _) => DiagramBrowserMembers::None,
                        (true, true) => DiagramBrowserMembers::Loaded,
                        (true, false) => DiagramBrowserMembers::Unloaded,
                    },
                );
            }
        }
        let projected_roots = occurrences
            .iter()
            .filter_map(|(_, (identity, owner))| owner.is_none().then_some(*identity))
            .collect::<BTreeSet<_>>();
        // Relationship usages may be owned outside the exposed subtrees (a deployment
        // allocation normally belongs to the enclosing assembly). Include the authored
        // relationship when its metaclass passes the filter and both of its settled ends
        // are already represented. Do not expose unrelated endpoints or expand its owner. A
        // Grid View's relationship matrix shows the same relationships as cells.
        if matches!(
            view_entry.kind,
            DiagramViewKind::General | DiagramViewKind::Grid
        ) {
            let represented = occurrences
                .values()
                .map(|(id, _)| *id)
                .collect::<BTreeSet<_>>();
            let already_present = represented.clone();
            for entry in all
                .values()
                .filter(|entry| workspace.contains(&entry.identity))
            {
                let Some((source_kind, target_kind, _)) = general_binary_relationship(entry.kind)
                else {
                    continue;
                };
                if already_present.contains(&entry.identity)
                    || !self.diagram_candidate_selected(view, entry.identity, &mut reasons)
                {
                    continue;
                }
                let Some(inspection) = usable_value(self.inspect(entry.identity)) else {
                    continue;
                };
                let end_present = |kind| {
                    inspection.relationships.iter().any(|relationship| {
                    relationship_kind_from_name(relationship.kind) == Some(kind)
                        && matches!(&relationship.target, RelationshipTarget::Resolved(target) if represented.contains(target))
                })
                };
                if !end_present(source_kind) || !end_present(target_kind) {
                    continue;
                }
                let Some(token) = self.symbol_token(entry.identity) else {
                    continue;
                };
                occurrences.insert(
                    DiagramOccurrenceIdentity::root(entry.identity, token.as_str()),
                    (entry.identity, None),
                );
            }
        }
        if view_entry.kind == DiagramViewKind::Interconnection {
            // Connections between separately exposed modules are commonly owned by
            // their enclosing assembly. Select their canonical endpoint paths, not
            // merely their terminal port declarations: two sensors may inherit the
            // same port while only one sensor is exposed.
            use crate::connection_query::ConnectorEndpoint;
            let represented_paths = occurrences
                .keys()
                .map(|occurrence| occurrence.semantic_path.as_ref())
                .collect::<Vec<_>>();
            let incident = self
                .model
                .all_connectors()
                .iter()
                .filter(|connector| {
                    workspace.contains(&connector.identity)
                        && !occurrences
                            .values()
                            .any(|(id, _)| *id == connector.identity)
                        && connector.ends.len() >= 2
                        && connector.ends.iter().all(|end| match &end.endpoint {
                            ConnectorEndpoint::Feature(RelationshipTarget::Resolved(target)) => {
                                represented_paths
                                    .iter()
                                    .any(|path| path.last() == Some(target))
                            }
                            ConnectorEndpoint::FeatureChain { path, .. } => {
                                let resolved = path
                                    .iter()
                                    .map(|segment| match segment {
                                        RelationshipTarget::Resolved(id) => Some(*id),
                                        _ => None,
                                    })
                                    .collect::<Option<Vec<_>>>();
                                resolved.is_some_and(|resolved| {
                                    represented_paths.iter().any(|shown| {
                                        shown.len() >= 2
                                            && (shown.ends_with(&resolved)
                                                || resolved.ends_with(shown))
                                    })
                                })
                            }
                            _ => false,
                        })
                })
                .map(|connector| connector.identity)
                .collect::<Vec<_>>();
            for identity in incident {
                if !self.diagram_candidate_selected(view, identity, &mut reasons) {
                    continue;
                }
                if let Some(token) = self.symbol_token(identity) {
                    occurrences.insert(
                        DiagramOccurrenceIdentity::root(identity, token.as_str()),
                        (identity, None),
                    );
                }
            }
        }
        let mut elements = occurrences
            .iter()
            .filter_map(|(occurrence_id, (identity, owner))| {
                all.get(identity).map(|entry| (occurrence_id, owner, entry))
            })
            .map(|(occurrence_id, owner, entry)| {
                // Direction and conjugation are settled facts about an element's own
                // declaration, not derived from its name or its owner's: `None` / `false` for a
                // declaration that carries no such fact, rather than a guess. A port and a
                // parameter of an action both have an authored direction; only a port's typing
                // can conjugate.
                let (direction, conjugated) = self
                    .model
                    .port_direction_and_conjugation(entry.identity)
                    .map(|(direction, conjugated)| {
                        (direction, conjugated && is_port_kind(entry.kind))
                    })
                    .unwrap_or_default();
                DiagramElement {
                    occurrence_id: occurrence_id.clone(),
                    semantic_id: entry.identity,
                    reference: semantic_reference(
                        entry,
                        self.qualified_name(entry.identity).unwrap_or_default(),
                        entry.owner.and_then(|owner| self.qualified_name(owner)),
                        self.document_identity(entry.location.document)
                            .unwrap_or_default(),
                    ),
                    kind: entry.kind,
                    // A declared name is its own effective name, so it survives an effective
                    // identification the publication cannot answer.
                    name: usable_value(self.effective_identification(entry.identity))
                        .and_then(|identification| match identification.name {
                            crate::EffectiveName::Resolved(name) => Some(name.into()),
                            crate::EffectiveName::Absent
                            | crate::EffectiveName::Unresolved
                            | crate::EffectiveName::NonConverged => None,
                        })
                        .or_else(|| entry.name.clone()),
                    typing: diagram_element_typing(self.element_details(entry.identity)),
                    owner: owner.clone(),
                    source: entry.location,
                    compartments: Box::default(),
                    multiplicity: self.model.symbol_multiplicity(entry.identity),
                    direction,
                    conjugated,
                    state_subaction: self.model.state_subaction_kind(entry.identity),
                    succession_guard: self.model.succession_guard(entry.identity),
                    feature_direction: if is_port_kind(entry.kind) {
                        self.model.port_feature_direction_of(entry.identity)
                    } else {
                        None
                    },
                    browser_row: (view_entry.kind == DiagramViewKind::Browser).then(|| {
                        DiagramBrowserRow {
                            membership: browser_membership
                                .get(occurrence_id)
                                .copied()
                                .unwrap_or(DiagramMembership::Owned),
                            members: browser_members
                                .get(occurrence_id)
                                .copied()
                                .unwrap_or(DiagramBrowserMembers::None),
                        }
                    }),
                }
            })
            .collect::<Vec<_>>();
        // A browser presents the model as it is written: siblings in declaration order, not
        // semantic-ID order. Owners are referenced by occurrence identity, so the order is free.
        if view_entry.kind == DiagramViewKind::Browser {
            elements.sort_by_key(|element| (element.source.document, element.source.range));
        }
        // SysML 8.2.3.11: an Interconnection View shows parts and ports as nodes and connection
        // usages as edges. Attributes, expressions, actions, states, and the rest are not peer
        // boxes here. Keep the connector usages (they are edge origins, filtered out of the node
        // metadata downstream) and every container on a kept element's occurrence path.
        if view_entry.kind == DiagramViewKind::Interconnection {
            let kept: BTreeSet<SymbolId> = elements
                .iter()
                .filter(|element| {
                    is_interconnection_node(element.kind) || is_connector_edge_element(element.kind)
                })
                .flat_map(|element| element.occurrence_id.semantic_path.iter().copied())
                .collect();
            elements.retain(|element| kept.contains(&element.semantic_id));
        }
        let element_kinds = elements
            .iter()
            .map(|element| (element.semantic_id, element.kind))
            .collect::<BTreeMap<_, _>>();
        for owner in &mut elements {
            let mut grouped = BTreeMap::<
                (DiagramCompartmentKind, DiagramCompartmentProvenance),
                Vec<DiagramOccurrenceIdentity>,
            >::new();
            for (child_occurrence, (child, projected_owner)) in &occurrences {
                if projected_owner.as_ref() != Some(&owner.occurrence_id) {
                    continue;
                }
                let Some(kind) = element_kinds
                    .get(child)
                    .and_then(|kind| compartment_kind(*kind))
                else {
                    continue;
                };
                let provenance = if all.get(child).and_then(|entry| entry.owner.as_ref())
                    == Some(&owner.semantic_id)
                {
                    DiagramCompartmentProvenance::Direct
                } else {
                    DiagramCompartmentProvenance::Inherited
                };
                grouped
                    .entry((kind, provenance))
                    .or_default()
                    .push(child_occurrence.clone());
            }
            owner.compartments = grouped
                .into_iter()
                .map(|((kind, provenance), members)| DiagramCompartment {
                    kind,
                    provenance,
                    members: members.into_boxed_slice(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
        }
        let mut relationships = Vec::new();
        // For a flow end that names a feature the view does not draw: the drawn element on the
        // authored path to it (`chain_carrier`).
        let mut flow_carriers: Vec<(
            DiagramOccurrenceIdentity,
            DiagramRelationshipKind,
            DiagramOccurrenceIdentity,
        )> = Vec::new();
        for element in &elements {
            if let Some(inspection) = usable_value(self.inspect(element.semantic_id)) {
                for (index, relationship) in inspection.relationships.iter().enumerate() {
                    // Every name the authority publishes is one of the canonical reference
                    // kinds, which is exactly the enum's variant set.
                    let kind = relationship_kind_from_name(relationship.kind).expect(
                        "a published relationship names one of the canonical reference kinds",
                    );
                    let target = match &relationship.target {
                        RelationshipTarget::Resolved(target) => {
                            let endpoint = contextual_endpoint(
                                &element.occurrence_id,
                                *target,
                                &elements,
                                relationship.authored.as_deref(),
                                |identity| self.symbol_name(identity),
                            );
                            // SysML 8.2.3.13: a flow between features of two ports is shown
                            // on the line between those ports. Its end is a feature the view
                            // does not draw, so remember the element of its path that it does.
                            if view_entry.kind == DiagramViewKind::Interconnection
                                && matches!(
                                    kind,
                                    DiagramRelationshipKind::FlowSource
                                        | DiagramRelationshipKind::FlowTarget
                                )
                                && endpoint.occurrence
                                    == DiagramEndpointOccurrence::OutsideProjection
                            {
                                if let Some(carrier) = chain_carrier(
                                    &element.occurrence_id,
                                    relationship.authored.as_deref(),
                                    &elements,
                                    |identity| self.symbol_name(identity),
                                ) {
                                    flow_carriers.push((
                                        element.occurrence_id.clone(),
                                        kind,
                                        carrier,
                                    ));
                                }
                            }
                            DiagramRelationshipTarget::Resolved(endpoint)
                        }
                        RelationshipTarget::Ambiguous(candidates) => {
                            if relationship_is_required(view_entry.kind, kind) {
                                reasons.insert(DiagramIncompleteReason::RelationshipAmbiguous {
                                    relationship: kind,
                                });
                            }
                            DiagramRelationshipTarget::Ambiguous(
                                candidates
                                    .iter()
                                    .map(|candidate| {
                                        contextual_endpoint(
                                            &element.occurrence_id,
                                            *candidate,
                                            &elements,
                                            relationship.authored.as_deref(),
                                            |identity| self.symbol_name(identity),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .into_boxed_slice(),
                            )
                        }
                        RelationshipTarget::Unresolved => {
                            if relationship_is_required(view_entry.kind, kind) {
                                reasons.insert(DiagramIncompleteReason::RelationshipUnresolved {
                                    relationship: kind,
                                });
                            }
                            DiagramRelationshipTarget::Unresolved
                        }
                        RelationshipTarget::Unsupported => {
                            if relationship_is_required(view_entry.kind, kind) {
                                reasons.insert(DiagramIncompleteReason::RelationshipUnsupported {
                                    relationship: kind,
                                });
                            }
                            DiagramRelationshipTarget::Unsupported
                        }
                    };
                    relationships.push(DiagramRelationship {
                        ordinal: index as u32,
                        source: element.occurrence_id.clone(),
                        source_semantic_id: element.semantic_id,
                        kind,
                        target,
                        provenance: relationship.provenance,
                        source_location: relationship.location,
                    });
                }
            }
        }
        relationships.sort_by_cached_key(|relationship| {
            format!(
                "{}#{}:{}",
                relationship.source.stable_key(),
                relationship.kind.name(),
                relationship.ordinal
            )
        });
        let mut edges =
            elements
                .iter()
                .enumerate()
                .filter_map(|(origin, element)| {
                    element.owner.as_ref().map(|owner| DiagramEdge {
                        origin: origin as u32,
                        source: owner.clone(),
                        source_semantic_id: owner.semantic_id(),
                        target: element.occurrence_id.clone(),
                        target_semantic_id: element.semantic_id,
                        kind: if matches!(
                            view_entry.kind,
                            DiagramViewKind::General | DiagramViewKind::Interconnection
                        ) && self.model.declaration_of(element.semantic_id).and_then(
                            |id| self.model.types.usage_is_reference(&self.model.storage, id),
                        ) == Some(true)
                        {
                            DiagramEdgeKind::Relationship("noncomposite-membership".into())
                        } else {
                            DiagramEdgeKind::Containment
                        },
                        provenance: if all.get(&element.semantic_id).and_then(|entry| entry.owner)
                            == Some(owner.semantic_id())
                        {
                            RelationshipProvenance::Authored
                        } else {
                            RelationshipProvenance::Implied
                        },
                        source_location: Some(element.source),
                    })
                })
                .collect::<Vec<_>>();
        for (origin, element) in elements.iter().enumerate() {
            let outgoing = relationships
                .iter()
                .filter(|relationship| relationship.source == element.occurrence_id)
                .collect::<Vec<_>>();
            if matches!(
                view_entry.kind,
                DiagramViewKind::General | DiagramViewKind::Grid
            ) {
                if let Some((source_kind, target_kind, kind)) =
                    general_binary_relationship(element.kind)
                {
                    // A dependency may have several clients and suppliers
                    // (`dependency from a, b to c`): one line for each pair whose ends are
                    // drawn.
                    let ends = |wanted: DiagramRelationshipKind| {
                        outgoing
                            .iter()
                            .filter(move |relationship| relationship.kind == wanted)
                            .filter_map(|relationship| resolved_target(&relationship.target))
                    };
                    let before = edges.len();
                    for source in ends(source_kind) {
                        for target in ends(target_kind) {
                            edges.push(edge_from_relationships(
                                origin as u32,
                                source,
                                target,
                                DiagramEdgeKind::Relationship(kind.into()),
                                &outgoing,
                            ));
                        }
                    }
                    if edges.len() > before {
                        continue;
                    }
                }
            }
            if let Some(edge) = composed_edge(
                origin as u32,
                &outgoing,
                DiagramRelationshipKind::TransitionSource,
                DiagramRelationshipKind::TransitionTarget,
                DiagramEdgeKind::Transition,
            ) {
                edges.push(edge);
                continue;
            }
            // An end of a transition written as a feature chain (`first on.ready`) is published
            // as a `memberAccessOperand`, not as `transitionSource` / `transitionTarget`. The
            // operands are the ends that are missing, in the order they are written.
            if element.kind == ElementKind::TransitionUsage {
                let named = |wanted: DiagramRelationshipKind| {
                    outgoing
                        .iter()
                        .find(|relationship| relationship.kind == wanted)
                        .and_then(|relationship| resolved_target(&relationship.target))
                };
                let mut operands = outgoing
                    .iter()
                    .filter(|relationship| {
                        relationship.kind == DiagramRelationshipKind::MemberAccessOperand
                    })
                    .filter_map(|relationship| resolved_target(&relationship.target));
                let source = named(DiagramRelationshipKind::TransitionSource);
                let target = named(DiagramRelationshipKind::TransitionTarget);
                let (source, target) = match (source, target) {
                    (None, Some(target)) => (operands.next(), Some(target)),
                    (Some(source), None) => (Some(source), operands.next()),
                    (None, None) => (operands.next(), operands.next()),
                    both => both,
                };
                if let (Some(source), Some(target), None) = (source, target, operands.next()) {
                    edges.push(edge_from_relationships(
                        origin as u32,
                        source,
                        target,
                        DiagramEdgeKind::Transition,
                        &outgoing,
                    ));
                    continue;
                }
            }
            if let Some(edge) = composed_edge(
                origin as u32,
                &outgoing,
                DiagramRelationshipKind::FlowSource,
                DiagramRelationshipKind::FlowTarget,
                DiagramEdgeKind::Flow,
            ) {
                edges.push(edge);
                continue;
            }
            // A flow whose ends are features of ports runs between those ports.
            if view_entry.kind == DiagramViewKind::Interconnection {
                let end = |wanted: DiagramRelationshipKind| {
                    outgoing
                        .iter()
                        .find(|relationship| relationship.kind == wanted)
                        .and_then(|relationship| resolved_target(&relationship.target))
                        .or_else(|| {
                            flow_carriers
                                .iter()
                                .find(|(source, kind, _)| {
                                    *source == element.occurrence_id && *kind == wanted
                                })
                                .map(|(_, _, carrier)| carrier)
                        })
                };
                if let (Some(source), Some(target)) = (
                    end(DiagramRelationshipKind::FlowSource),
                    end(DiagramRelationshipKind::FlowTarget),
                ) {
                    if source != target {
                        edges.push(edge_from_relationships(
                            origin as u32,
                            source,
                            target,
                            DiagramEdgeKind::Flow,
                            &outgoing,
                        ));
                        continue;
                    }
                }
            }
            // A `bind a = b.c` usage publishes `bindSource` plus one `memberAccessOperand` for the
            // dotted side. Those are the binding's two ends. Counting only the member-access
            // operand makes a delegation look like an unresolved `connectorEnd`.
            if view_entry.kind == DiagramViewKind::Interconnection
                && is_binding_connector(element.kind)
            {
                let mut ends = outgoing
                    .iter()
                    .filter(|relationship| {
                        matches!(
                            relationship.kind,
                            DiagramRelationshipKind::BindSource
                                | DiagramRelationshipKind::BindTarget
                                | DiagramRelationshipKind::MemberAccessOperand
                        )
                    })
                    .copied()
                    .collect::<Vec<_>>();
                // `relationships` above is sorted by "{source}#{kind_name}:{ordinal}" for
                // deterministic overall output, which reorders these two ends by kind name
                // ("bindTarget" < "memberAccessOperand") whenever the dotted side and the plain
                // side have different kinds -- e.g. `bind inner.nested = boundary;` would then
                // report `boundary` as the source. `ordinal` does not help: it is each
                // relationship's position among the *resolved* facts the authority publishes,
                // which for a dotted chain reflects when member-access resolution finished, not
                // when `left` was lowered relative to `right` -- empirically the plain `right`
                // operand's simple reference can resolve (and so get a lower ordinal) before the
                // dotted `left` operand's chain does. `left` is always textually to the left of
                // `right` in `bind left = right`, though, so sorting by each end's own authored
                // source position -- which does not depend on resolution timing -- recovers the
                // source/target roles reliably regardless of which kind each end ended up as.
                ends.sort_by_key(|relationship| {
                    relationship
                        .source_location
                        .map(|location| (location.document, location.range.start))
                });
                match ends.as_slice() {
                    [first, second] => {
                        match (
                            resolved_target(&first.target),
                            resolved_target(&second.target),
                        ) {
                            (Some(source), Some(target)) => {
                                edges.push(edge_from_relationships(
                                    origin as u32,
                                    source,
                                    target,
                                    DiagramEdgeKind::Connector,
                                    &ends,
                                ));
                            }
                            _ => {
                                for end in &ends {
                                    if resolved_target(&end.target).is_none() {
                                        reasons
                                            .insert(connector_end_incomplete_reason(&end.target));
                                    }
                                }
                            }
                        }
                    }
                    [] => {}
                    _ => {
                        reasons.insert(DiagramIncompleteReason::RelationshipUnresolved {
                            relationship: DiagramRelationshipKind::ConnectorEnd,
                        });
                    }
                }
            }
            // A `connect a to b` usage publishes its two ends as `connectorEnd` relationships; the
            // dotted `connect a.b to c.d` form publishes them as `memberAccessOperand` on the same
            // connection usage. Compose one Connector edge from whichever pair the usage carries so
            // a connection is an edge, not a peer box. An end that is unresolved, ambiguous, or
            // resolved outside the projection is a typed incomplete reason, never a guessed line.
            {
                // The dotted `connect a.b to c.d` ends and the typed incomplete reasons are an
                // Interconnection View concern; other views keep the prior plain-`connectorEnd`
                // behaviour untouched. Binding connectors are composed above from their own ends.
                let dotted_ends = view_entry.kind == DiagramViewKind::Interconnection
                    && is_connector_edge_element(element.kind)
                    && !is_binding_connector(element.kind);
                let ends = outgoing
                    .iter()
                    .filter(|relationship| {
                        relationship.kind == DiagramRelationshipKind::ConnectorEnd
                            || (dotted_ends
                                && relationship.kind
                                    == DiagramRelationshipKind::MemberAccessOperand)
                    })
                    .copied()
                    .collect::<Vec<_>>();
                match ends.as_slice() {
                    [first, second] => {
                        match (
                            resolved_target(&first.target),
                            resolved_target(&second.target),
                        ) {
                            (Some(source), Some(target)) => {
                                edges.push(edge_from_relationships(
                                    origin as u32,
                                    source,
                                    target,
                                    DiagramEdgeKind::Connector,
                                    &ends,
                                ));
                            }
                            _ if dotted_ends => {
                                for end in &ends {
                                    if resolved_target(&end.target).is_none() {
                                        reasons
                                            .insert(connector_end_incomplete_reason(&end.target));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    [] => {}
                    _ if dotted_ends => {
                        reasons.insert(DiagramIncompleteReason::RelationshipUnresolved {
                            relationship: DiagramRelationshipKind::ConnectorEnd,
                        });
                    }
                    _ => {}
                }
            }
            {
                let endpoints = outgoing
                    .iter()
                    .filter(|relationship| relationship.kind == DiagramRelationshipKind::Succession)
                    .filter_map(|relationship| resolved_target(&relationship.target))
                    .collect::<Vec<_>>();
                if let [source, target] = endpoints.as_slice() {
                    edges.push(edge_from_relationships(
                        origin as u32,
                        source,
                        target,
                        DiagramEdgeKind::Succession,
                        &outgoing,
                    ));
                }
            }
            // A `then` shorthand declares a succession from the source the lowering settled
            // for it (SysML 7.17.4) to the feature it names, or to the node written after it.
            if matches!(
                view_entry.kind,
                DiagramViewKind::ActionFlow | DiagramViewKind::Sequence
            ) {
                let source = self
                    .model
                    .then_source(element.semantic_id)
                    .map(|source| {
                        contextual_endpoint(&element.occurrence_id, source, &elements, None, |id| {
                            self.symbol_name(id)
                        })
                    })
                    .and_then(|endpoint| match endpoint.occurrence {
                        DiagramEndpointOccurrence::Resolved(source) => Some(source),
                        _ => None,
                    });
                let target = if element.kind == ElementKind::SuccessionAsUsage {
                    outgoing
                        .iter()
                        .find(|relationship| {
                            relationship.kind == DiagramRelationshipKind::ThenTarget
                        })
                        .and_then(|relationship| resolved_target(&relationship.target))
                        .cloned()
                } else {
                    Some(element.occurrence_id.clone())
                };
                if let (Some(source), Some(target)) = (source, target) {
                    edges.push(DiagramEdge {
                        origin: origin as u32,
                        source_semantic_id: source.semantic_id(),
                        source,
                        target_semantic_id: target.semantic_id(),
                        target,
                        kind: DiagramEdgeKind::Succession,
                        provenance: RelationshipProvenance::Authored,
                        source_location: Some(element.source),
                    });
                }
            }
            // SysML 8.2.3.6: the General View draws authored subclassification, subsetting,
            // redefinition, and feature typing between two elements it projects. Implied library
            // subsetting (`Parts::parts`, `Items::items`, the kernel chain) stays off the canvas --
            // the typed facts remain on `relationships` for inspectors -- and an end that is
            // unresolved, ambiguous, or outside the view yields no edge rather than a guessed line.
            if matches!(
                view_entry.kind,
                DiagramViewKind::General | DiagramViewKind::Grid
            ) {
                for relationship in &outgoing {
                    if relationship.provenance != RelationshipProvenance::Authored {
                        continue;
                    }
                    let Some(edge_kind) = general_specialization_edge_kind(relationship.kind)
                    else {
                        continue;
                    };
                    let Some(target) = resolved_target(&relationship.target) else {
                        continue;
                    };
                    edges.push(edge_from_relationships(
                        origin as u32,
                        &element.occurrence_id,
                        target,
                        DiagramEdgeKind::Relationship(edge_kind.into()),
                        std::slice::from_ref(relationship),
                    ));
                }
            }
            if let Some(initial) = outgoing
                .iter()
                .find(|relationship| relationship.kind == DiagramRelationshipKind::InitialState)
            {
                if let Some(target) = resolved_target(&initial.target) {
                    edges.push(DiagramEdge {
                        origin: origin as u32,
                        source: element.occurrence_id.clone(),
                        source_semantic_id: element.semantic_id,
                        target: target.clone(),
                        target_semantic_id: target.semantic_id(),
                        kind: DiagramEdgeKind::InitialState,
                        provenance: initial.provenance,
                        source_location: initial.source_location,
                    });
                }
            }
        }
        edges.sort_by_cached_key(|edge| {
            format!(
                "{}#{}",
                elements[edge.origin as usize].occurrence_id.stable_key(),
                edge_scene_suffix(&edge.kind)
            )
        });
        // The view's own declaration and every projected element name the documents this view
        // draws from; their recovery or unsupported constructs may have dropped content from it.
        let documents = std::iter::once(view_entry.source.document)
            .chain(elements.iter().map(|element| element.source.document))
            .collect::<BTreeSet<_>>();
        for document in documents {
            reasons.extend(
                self.model
                    .document_obstacles(document)
                    .map(publication_obstacle_reason),
            );
        }
        let scene = diagram_scene(
            view_entry.kind,
            &projected_roots,
            &elements,
            &relationships,
            &edges,
            &all,
            &mut reasons,
        );
        if view_entry.kind == DiagramViewKind::Geometry && !elements.is_empty() {
            reasons.insert(DiagramIncompleteReason::GeometryFactsUnavailable);
        }
        self.model
            .query_outcome(QueryAnswer::Resolved(DiagramViewProjection {
                view: view_entry,
                exposed_roots: projected_roots
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                elements: elements.into_boxed_slice(),
                relationships: relationships.into_boxed_slice(),
                edges: edges.into_boxed_slice(),
                scene,
                incomplete_reasons: reasons.into_iter().collect::<Vec<_>>().into_boxed_slice(),
            }))
    }

    fn diagram_candidate_selected(
        &self,
        view: SymbolId,
        candidate: SymbolId,
        reasons: &mut BTreeSet<DiagramIncompleteReason>,
    ) -> bool {
        match self.view_selection(view, candidate).answer {
            QueryAnswer::Resolved(selection) => match selection.outcome {
                ViewSelectionOutcome::Included => true,
                ViewSelectionOutcome::Excluded => false,
                ViewSelectionOutcome::Indeterminate(obstacles) => {
                    for obstacle in obstacles.iter() {
                        reasons.insert(match obstacle {
                            ViewSelectionObstacle::UnresolvedPredicate => {
                                DiagramIncompleteReason::ViewFilterUnresolved
                            }
                            ViewSelectionObstacle::AmbiguousPredicate(_) => {
                                DiagramIncompleteReason::ViewFilterAmbiguous
                            }
                            ViewSelectionObstacle::UnsupportedPredicate => {
                                DiagramIncompleteReason::ViewFilterUnsupported
                            }
                        });
                    }
                    false
                }
            },
            QueryAnswer::Ambiguous(_) => {
                reasons.insert(DiagramIncompleteReason::ViewFilterAmbiguous);
                false
            }
            QueryAnswer::Unresolved => {
                reasons.insert(DiagramIncompleteReason::ViewFilterUnresolved);
                false
            }
            QueryAnswer::Unsupported | QueryAnswer::Recovery | QueryAnswer::Incomplete => {
                reasons.insert(DiagramIncompleteReason::ViewFilterUnsupported);
                false
            }
        }
    }

    fn diagram_entries(&self) -> BTreeMap<SymbolId, SymbolEntry> {
        let mut entries = BTreeMap::new();
        for source in [
            ElementSource::Workspace,
            ElementSource::StandardLibrary,
            ElementSource::Library,
            ElementSource::External,
        ] {
            entries.extend(self.diagram_entries_for(source));
        }
        entries
    }

    /// The features outside the workspace that successions among `members` begin or end at:
    /// the resolved ends of each succession, the feature a `then` names, and the source the
    /// lowering settled for a `then`.
    fn library_succession_ends(
        &self,
        members: &[SymbolEntry],
        workspace: &BTreeSet<SymbolId>,
    ) -> BTreeSet<SymbolId> {
        let mut ends = BTreeSet::new();
        for member in members
            .iter()
            .filter(|member| workspace.contains(&member.identity))
        {
            ends.extend(self.model.then_source(member.identity));
            if member.kind != ElementKind::SuccessionAsUsage {
                continue;
            }
            let Some(inspection) = usable_value(self.inspect(member.identity)) else {
                continue;
            };
            for relationship in inspection.relationships.iter() {
                let is_end = matches!(
                    relationship_kind_from_name(relationship.kind),
                    Some(DiagramRelationshipKind::Succession | DiagramRelationshipKind::ThenTarget)
                );
                if let (true, RelationshipTarget::Resolved(target)) = (is_end, &relationship.target)
                {
                    ends.insert(*target);
                }
            }
        }
        ends.retain(|end| !workspace.contains(end));
        ends
    }

    fn diagram_entries_for(&self, source: ElementSource) -> BTreeMap<SymbolId, SymbolEntry> {
        let mut entries = BTreeMap::new();
        for &kind in ElementKind::ALL {
            for entry in resolved_values(self.search_elements(ElementSearch { kind, source })) {
                entries.insert(entry.identity, entry);
            }
        }
        entries
    }
}

/// KerML structure that lowering adds to a declaration and that no view draws as a node: a
/// multiplicity and its bounds, and an expression that is not a SysML usage.
fn is_declaration_structure(kind: ElementKind) -> bool {
    !kind.conforms_to(ElementKind::Usage)
        && (kind.conforms_to(ElementKind::Multiplicity)
            || kind.conforms_to(ElementKind::Expression))
}

fn relationship_is_required(view: DiagramViewKind, kind: DiagramRelationshipKind) -> bool {
    match view {
        DiagramViewKind::Interconnection => kind == DiagramRelationshipKind::ConnectorEnd,
        DiagramViewKind::ActionFlow => matches!(
            kind,
            DiagramRelationshipKind::FlowSource
                | DiagramRelationshipKind::FlowTarget
                | DiagramRelationshipKind::Succession
        ),
        DiagramViewKind::StateTransition => matches!(
            kind,
            DiagramRelationshipKind::InitialState
                | DiagramRelationshipKind::TransitionSource
                | DiagramRelationshipKind::TransitionTarget
                | DiagramRelationshipKind::TransitionTrigger
                | DiagramRelationshipKind::TransitionEffect
        ),
        // A message (`FlowConnectionUsage`) on an occurrence lifeline publishes its ends as
        // `flowSource` / `flowTarget`; `succession` between messages carries their order.
        DiagramViewKind::Sequence => matches!(
            kind,
            DiagramRelationshipKind::FlowSource
                | DiagramRelationshipKind::FlowTarget
                | DiagramRelationshipKind::Succession
        ),
        DiagramViewKind::General | DiagramViewKind::Grid => matches!(
            kind,
            DiagramRelationshipKind::AllocateSource
                | DiagramRelationshipKind::AllocateTarget
                | DiagramRelationshipKind::DependencyClient
                | DiagramRelationshipKind::DependencySupplier
                | DiagramRelationshipKind::BindSource
                | DiagramRelationshipKind::BindTarget
        ),
        DiagramViewKind::Browser | DiagramViewKind::Geometry => false,
    }
}

/// The boundary form of one element reference.
///
fn publication_obstacle_reason(obstacle: crate::PublicationObstacle) -> DiagramIncompleteReason {
    match obstacle {
        crate::PublicationObstacle::ParseRecovery => DiagramIncompleteReason::ParseRecovery,
        crate::PublicationObstacle::UnsupportedSyntax => DiagramIncompleteReason::UnsupportedSyntax,
        crate::PublicationObstacle::NonConverged => DiagramIncompleteReason::NonConverged,
    }
}

/// `document` is the identity the caller materialised from `entry.location.document`: the
/// generator protocol is a process boundary, so the URI is spelled out here rather than carried
/// as a publication-scoped handle no other process could resolve.
fn semantic_reference(
    entry: &SymbolEntry,
    qualified_name: &str,
    owner_qualified_name: Option<&str>,
    document: &str,
) -> DiagramSemanticReference {
    if entry.name.is_some() {
        DiagramSemanticReference::Qualified {
            document: document.into(),
            qualified_name: qualified_name.into(),
        }
    } else {
        DiagramSemanticReference::SourceAnchor {
            document: document.into(),
            owner_qualified_name: owner_qualified_name.map(Into::into),
            kind: entry.kind,
            range: entry.declaration_range,
        }
    }
}

/// The projected element that carries the feature an authored feature chain ends at, when that
/// feature is not projected itself: for `tank::outlet::fuel` the occurrence of `outlet` below
/// `tank`. It is the element whose path of names ends with the longest proper prefix of the
/// chain; among several, the one that shares most of its path with `source`. `None` when no
/// prefix names a projected element, or when the choice is not unique.
fn chain_carrier<'a>(
    source: &DiagramOccurrenceIdentity,
    authored: Option<&str>,
    elements: &[DiagramElement],
    symbol_name: impl Fn(SymbolId) -> Option<&'a str>,
) -> Option<DiagramOccurrenceIdentity> {
    let segments: Vec<&str> = authored?
        .split("::")
        .filter(|segment| !segment.is_empty() && *segment != "$")
        .collect();
    for length in (1..segments.len()).rev() {
        let prefix = &segments[..length];
        let mut candidates = elements
            .iter()
            .filter_map(|element| {
                let names = element
                    .occurrence_id
                    .semantic_path
                    .iter()
                    .filter_map(|identity| symbol_name(*identity))
                    .collect::<Vec<_>>();
                if !names.ends_with(prefix) {
                    return None;
                }
                let shared = source
                    .semantic_path
                    .iter()
                    .zip(element.occurrence_id.semantic_path.iter())
                    .take_while(|(left, right)| left == right)
                    .count();
                Some((shared, &element.occurrence_id))
            })
            .collect::<Vec<_>>();
        candidates.sort();
        match candidates.as_slice() {
            [] => continue,
            [.., (runner_up, _), (best, _)] if runner_up == best => return None,
            [.., (_, carrier)] => return Some((*carrier).clone()),
        }
    }
    None
}

/// How much of an authored feature-chain (`propulsion::propulsionUnit1::cmd`) appears, in
/// order, along one occurrence's name path. Shared-prefix scoring alone ties when several
/// usages of the same definition contribute the same terminal `semantic_id` (four
/// `PropulsionUnit::cmd` ports) and the connector itself does not sit under a distinguishing
/// ancestor; the authored intermediates break that tie.
fn authored_chain_score(authored: Option<&str>, names: &[&str]) -> usize {
    let Some(authored) = authored.filter(|text| !text.is_empty()) else {
        return 0;
    };
    let segments = authored
        .split("::")
        .filter(|segment| !segment.is_empty() && *segment != "$");
    let mut matched = 0;
    let mut names = names.iter();
    for segment in segments {
        loop {
            match names.next() {
                Some(name) if *name == segment => {
                    matched += 1;
                    break;
                }
                Some(_) => continue,
                None => return matched,
            }
        }
    }
    matched
}

fn contextual_endpoint<'a>(
    source: &DiagramOccurrenceIdentity,
    semantic_id: SymbolId,
    elements: &[DiagramElement],
    authored: Option<&str>,
    symbol_name: impl Fn(SymbolId) -> Option<&'a str>,
) -> DiagramRelationshipEndpoint {
    let mut candidates = elements
        .iter()
        .filter(|element| element.semantic_id == semantic_id)
        .map(|element| {
            let names = element
                .occurrence_id
                .semantic_path
                .iter()
                .filter_map(|identity| symbol_name(*identity))
                .collect::<Vec<_>>();
            let authored_match = authored_chain_score(authored, &names);
            let shared = source
                .semantic_path
                .iter()
                .zip(element.occurrence_id.semantic_path.iter())
                .take_while(|(left, right)| left == right)
                .count();
            (authored_match, shared, element.occurrence_id.clone())
        })
        .collect::<Vec<_>>();
    candidates.sort();
    let occurrence = match candidates
        .last()
        .map(|candidate| (candidate.0, candidate.1))
    {
        None => DiagramEndpointOccurrence::OutsideProjection,
        Some(best) => {
            let best = candidates
                .into_iter()
                .filter_map(|(authored_match, shared, occurrence)| {
                    (authored_match == best.0 && shared == best.1).then_some(occurrence)
                })
                .collect::<Vec<_>>();
            if let [occurrence] = best.as_slice() {
                DiagramEndpointOccurrence::Resolved(occurrence.clone())
            } else {
                DiagramEndpointOccurrence::Ambiguous(best.into_boxed_slice())
            }
        }
    };
    DiagramRelationshipEndpoint {
        semantic_id,
        occurrence,
    }
}

fn resolved_target(target: &DiagramRelationshipTarget) -> Option<&DiagramOccurrenceIdentity> {
    match target {
        DiagramRelationshipTarget::Resolved(DiagramRelationshipEndpoint {
            occurrence: DiagramEndpointOccurrence::Resolved(target),
            ..
        }) => Some(target),
        _ => None,
    }
}

fn compartment_kind(kind: ElementKind) -> Option<DiagramCompartmentKind> {
    use DiagramCompartmentKind as Compartment;
    Some(match kind {
        ElementKind::AttributeUsage
        | ElementKind::EnumerationUsage
        | ElementKind::ReferenceUsage => Compartment::Attributes,
        ElementKind::PartUsage => Compartment::Parts,
        ElementKind::PortUsage => Compartment::Ports,
        ElementKind::ItemUsage => Compartment::Items,
        ElementKind::ConstraintUsage
        | ElementKind::AssertConstraintUsage
        | ElementKind::Invariant => Compartment::Constraints,
        ElementKind::RequirementUsage
        | ElementKind::ConcernUsage
        | ElementKind::SatisfyRequirementUsage => Compartment::Requirements,
        ElementKind::ActionUsage
        | ElementKind::AcceptActionUsage
        | ElementKind::SendActionUsage
        | ElementKind::TerminateActionUsage
        | ElementKind::PerformActionUsage
        | ElementKind::AssignmentActionUsage
        | ElementKind::IfActionUsage
        | ElementKind::WhileLoopActionUsage
        | ElementKind::ForLoopActionUsage => Compartment::Actions,
        ElementKind::StateUsage | ElementKind::ExhibitStateUsage | ElementKind::FinalState => {
            Compartment::States
        }
        ElementKind::CalculationUsage => Compartment::Calculations,
        ElementKind::ConnectionUsage
        | ElementKind::FlowConnectionUsage
        | ElementKind::SuccessionFlowUsage
        | ElementKind::BindingConnectorAsUsage
        | ElementKind::Connector
        | ElementKind::BindingConnector => Compartment::Connections,
        ElementKind::InterfaceUsage => Compartment::Interfaces,
        ElementKind::OccurrenceUsage => Compartment::Occurrences,
        _ => return None,
    })
}

fn diagram_scene(
    kind: DiagramViewKind,
    roots: &BTreeSet<SymbolId>,
    elements: &[DiagramElement],
    relationships: &[DiagramRelationship],
    edges: &[DiagramEdge],
    entries: &BTreeMap<SymbolId, SymbolEntry>,
    reasons: &mut BTreeSet<DiagramIncompleteReason>,
) -> DiagramScene {
    match kind {
        DiagramViewKind::General => DiagramScene::General,
        DiagramViewKind::Interconnection => DiagramScene::Interconnection,
        DiagramViewKind::ActionFlow => DiagramScene::ActionFlow,
        DiagramViewKind::Sequence => DiagramScene::Sequence(sequence_scene(
            roots,
            elements,
            relationships,
            edges,
            reasons,
        )),
        DiagramViewKind::Browser => DiagramScene::Browser,
        DiagramViewKind::Grid => DiagramScene::Grid,
        DiagramViewKind::Geometry => DiagramScene::Geometry,
        DiagramViewKind::StateTransition => {
            let machine = roots.iter().next().cloned();
            let initial_sources = edges
                .iter()
                .filter(|edge| edge.kind == DiagramEdgeKind::InitialState)
                .map(|edge| edge.source_semantic_id)
                .collect::<BTreeSet<_>>();
            let vertices = elements
                .iter()
                .filter_map(|element| {
                    let kind = if initial_sources.contains(&element.semantic_id) {
                        DiagramStateVertexKind::Initial
                    } else {
                        match element.kind {
                            ElementKind::StateUsage | ElementKind::ExhibitStateUsage => {
                                DiagramStateVertexKind::State
                            }
                            ElementKind::FinalState => DiagramStateVertexKind::Final,
                            _ => return None,
                        }
                    };
                    Some(DiagramStateVertex {
                        semantic_id: element.semantic_id,
                        kind,
                        source: element.source,
                    })
                })
                .collect::<Vec<_>>();
            let transitions = edges
                .iter()
                .filter(|edge| {
                    matches!(
                        edge.kind,
                        DiagramEdgeKind::Transition | DiagramEdgeKind::InitialState
                    )
                })
                .filter_map(|edge| {
                    let origin = elements.get(edge.origin as usize)?;
                    let feature = |relationship_kind| {
                        transition_feature(origin, relationship_kind, relationships, entries)
                    };
                    Some(DiagramStateTransition {
                        origin: edge.origin,
                        role: if edge.kind == DiagramEdgeKind::InitialState {
                            DiagramTransitionRole::Initial
                        } else {
                            DiagramTransitionRole::Transition
                        },
                        label: origin.name.clone(),
                        source: edge.source_semantic_id,
                        target: edge.target_semantic_id,
                        trigger: if edge.kind == DiagramEdgeKind::InitialState {
                            DiagramTransitionFeature::Absent
                        } else {
                            feature(DiagramRelationshipKind::TransitionTrigger)
                        },
                        // A guard is an expression, not a reference: its text as written, with
                        // the transition as its navigation target (SysML 8.2.3.18).
                        guard: match &origin.succession_guard {
                            Some(DiagramSuccessionGuard::Condition(text)) => {
                                DiagramTransitionFeature::Resolved {
                                    label: text.clone(),
                                    target: origin.semantic_id,
                                    source: origin.source,
                                }
                            }
                            _ => DiagramTransitionFeature::Absent,
                        },
                        effect: match feature(DiagramRelationshipKind::TransitionEffect) {
                            // `do action a : A` binds the type through the effect action the
                            // transition owns, so the reference starts one level down.
                            DiagramTransitionFeature::Absent => {
                                transition_effect_of_owned_action(origin, relationships, entries)
                            }
                            effect => effect,
                        },
                        provenance: edge.provenance,
                        source_location: edge.source_location.unwrap_or(origin.source),
                    })
                })
                .collect::<Vec<_>>();
            DiagramScene::StateTransition(DiagramStateTransitionScene {
                machine,
                vertices: vertices.into_boxed_slice(),
                transitions: transitions.into_boxed_slice(),
            })
        }
    }
}

fn sequence_scene(
    roots: &BTreeSet<SymbolId>,
    elements: &[DiagramElement],
    relationships: &[DiagramRelationship],
    edges: &[DiagramEdge],
    reasons: &mut BTreeSet<DiagramIncompleteReason>,
) -> DiagramSequenceScene {
    let is_direct_member = |element: &&DiagramElement| {
        element
            .owner
            .as_ref()
            .is_some_and(|owner| roots.contains(&owner.semantic_id()))
    };
    // A sequence view owns a stronger order than semantic-ID order: lifelines stand in the
    // order the scenario declares its participants, so messages read the way they were written
    // rather than across an alphabetical column order.
    let mut lifeline_elements = elements
        .iter()
        .filter(is_direct_member)
        .filter(|element| {
            matches!(
                element.kind,
                ElementKind::PartUsage | ElementKind::PortUsage
            )
        })
        .collect::<Vec<_>>();
    lifeline_elements.sort_by_key(|element| (element.source.document, element.source.range));
    let lifelines = lifeline_elements
        .into_iter()
        .map(|element| element.occurrence_id.clone())
        .collect::<Vec<_>>();
    let lifeline_set = lifelines.iter().cloned().collect::<BTreeSet<_>>();
    let messages = elements
        .iter()
        .enumerate()
        .filter(|(_, element)| element.kind.conforms_to(ElementKind::FlowConnectionUsage))
        .filter(|(_, element)| is_direct_member(element))
        .map(|(index, element)| (index as u32, element))
        .collect::<Vec<_>>();

    // A flow the scene cannot place is omitted scenario content only when it sits in nested
    // behaviour (a sub-scenario or structured action). A flow inside a lifeline's own structure,
    // or inside any definition, is that participant's internals, not a message of this
    // interaction, so it is not reported.
    let by_occurrence = elements
        .iter()
        .map(|element| (&element.occurrence_id, element))
        .collect::<BTreeMap<_, _>>();
    let in_structure = |element: &DiagramElement| {
        let mut owner = element.owner.as_ref();
        while let Some(current) = owner {
            if roots.contains(&current.semantic_id()) {
                return false;
            }
            let Some(entry) = by_occurrence.get(current) else {
                return false;
            };
            if lifeline_set.contains(current)
                || [
                    ElementKind::Definition,
                    ElementKind::PartUsage,
                    ElementKind::PortUsage,
                    ElementKind::ItemUsage,
                ]
                .into_iter()
                .any(|kind| entry.kind.conforms_to(kind))
            {
                return true;
            }
            owner = entry.owner.as_ref();
        }
        false
    };
    let omitted = elements
        .iter()
        .filter(|element| element.kind.conforms_to(ElementKind::FlowConnectionUsage))
        .filter(|element| !is_direct_member(element))
        .filter(|element| !in_structure(element))
        .map(|element| element.occurrence_id.clone())
        .collect::<BTreeSet<_>>();
    if !omitted.is_empty() {
        reasons.insert(DiagramIncompleteReason::SequenceContentOmitted {
            elements: omitted.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        });
    }

    let owner_by_occurrence = elements
        .iter()
        .filter_map(|element| {
            element
                .owner
                .as_ref()
                .map(|owner| (element.occurrence_id.clone(), owner.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    let endpoint = |message: &DiagramElement, kind: DiagramRelationshipKind| {
        let Some(relationship) = relationships.iter().find(|relationship| {
            relationship.source == message.occurrence_id && relationship.kind == kind
        }) else {
            return DiagramSequenceEndpoint::Unresolved;
        };
        let occurrence = match &relationship.target {
            DiagramRelationshipTarget::Resolved(target) => match &target.occurrence {
                DiagramEndpointOccurrence::Resolved(occurrence) => occurrence,
                DiagramEndpointOccurrence::Ambiguous(_) => {
                    return DiagramSequenceEndpoint::Ambiguous;
                }
                DiagramEndpointOccurrence::OutsideProjection => {
                    return DiagramSequenceEndpoint::OutsideLifeline;
                }
            },
            DiagramRelationshipTarget::Ambiguous(_) => {
                return DiagramSequenceEndpoint::Ambiguous;
            }
            DiagramRelationshipTarget::Unresolved => {
                return DiagramSequenceEndpoint::Unresolved;
            }
            DiagramRelationshipTarget::Unsupported => {
                return DiagramSequenceEndpoint::Unsupported;
            }
        };
        let mut current = occurrence.clone();
        let mut seen = BTreeSet::new();
        loop {
            if lifeline_set.contains(&current) {
                return DiagramSequenceEndpoint::Resolved(current);
            }
            if !seen.insert(current.clone()) {
                return DiagramSequenceEndpoint::OutsideLifeline;
            }
            let Some(owner) = owner_by_occurrence.get(&current) else {
                return DiagramSequenceEndpoint::OutsideLifeline;
            };
            current = owner.clone();
        }
    };

    // Kahn's algorithm gives a true topological order. The element index is the deterministic
    // tie-break for unrelated messages and therefore preserves the projection's canonical order.
    let message_indexes = messages
        .iter()
        .map(|(index, _)| *index)
        .collect::<BTreeSet<_>>();
    let occurrence_to_index = messages
        .iter()
        .map(|(index, element)| (element.occurrence_id.clone(), *index))
        .collect::<BTreeMap<_, _>>();
    let mut precedence = edges
        .iter()
        .filter(|edge| edge.kind == DiagramEdgeKind::Succession)
        .filter_map(|edge| {
            Some((
                *occurrence_to_index.get(&edge.source)?,
                *occurrence_to_index.get(&edge.target)?,
            ))
        })
        .collect::<BTreeSet<_>>();
    // A message is ordered by its event occurrences, not by its name or declaration order.
    // Preserve the event graph through each transfer so causal paths may cross lifelines.
    let event_end = |message: &DiagramElement, kind| {
        relationships
            .iter()
            .find(|relationship| {
                relationship.source == message.occurrence_id && relationship.kind == kind
            })
            .and_then(|relationship| resolved_target(&relationship.target))
            .cloned()
    };
    // Only event occurrences order messages. A lifeline, or a port or part of one, is structure
    // that every message to it shares; treating it as an event would order repeated or reply
    // messages through it into a false cycle.
    let kind_by_occurrence = elements
        .iter()
        .map(|element| (&element.occurrence_id, element.kind))
        .collect::<BTreeMap<_, _>>();
    let is_event = |occurrence: &DiagramOccurrenceIdentity| {
        !lifeline_set.contains(occurrence)
            && !matches!(
                kind_by_occurrence.get(occurrence),
                Some(ElementKind::PartUsage | ElementKind::PortUsage | ElementKind::ItemUsage)
            )
    };
    let message_events = messages
        .iter()
        .map(|(index, message)| {
            (
                *index,
                event_end(message, DiagramRelationshipKind::FlowSource).filter(is_event),
                event_end(message, DiagramRelationshipKind::FlowTarget).filter(is_event),
            )
        })
        .collect::<Vec<_>>();
    let mut event_successors =
        BTreeMap::<DiagramOccurrenceIdentity, BTreeSet<DiagramOccurrenceIdentity>>::new();
    for edge in edges
        .iter()
        .filter(|edge| edge.kind == DiagramEdgeKind::Succession)
    {
        event_successors
            .entry(edge.source.clone())
            .or_default()
            .insert(edge.target.clone());
    }
    for (_, source, target) in &message_events {
        let (Some(source), Some(target)) = (source, target) else {
            continue;
        };
        event_successors
            .entry(source.clone())
            .or_default()
            .insert(target.clone());
    }
    // A row is a deterministic display position respecting known precedence, not a claim
    // that unrelated messages have an authored temporal order. A message precedes another when
    // one of its events (send or receive) reaches one of the other's events. An event the two
    // share carries no order between them, so a path may neither start nor end at one.
    let mut evidence = precedence
        .iter()
        .map(|pair| (*pair, None))
        .collect::<BTreeMap<_, Option<(DiagramOccurrenceIdentity, DiagramOccurrenceIdentity)>>>();
    let events_of = |source: &Option<DiagramOccurrenceIdentity>,
                     target: &Option<DiagramOccurrenceIdentity>| {
        [source.as_ref(), target.as_ref()]
            .into_iter()
            .flatten()
            .cloned()
            .collect::<BTreeSet<_>>()
    };
    let message_event_sets = message_events
        .iter()
        .map(|(index, source, target)| (*index, events_of(source, target)))
        .collect::<BTreeMap<_, _>>();
    let mut messages_by_event = BTreeMap::<&DiagramOccurrenceIdentity, Vec<u32>>::new();
    for (index, events) in &message_event_sets {
        for event in events {
            messages_by_event.entry(event).or_default().push(*index);
        }
    }
    let mut cyclic_events = BTreeSet::new();
    // Visit each distinct endpoint once and discard its search tree after use, rather than
    // retaining a potentially graph-sized predecessor map for every event.
    for (event, indexes) in &messages_by_event {
        let reachable = directed_sequence_parents(&event_successors, *event);
        if reachable.contains_key(*event) {
            cyclic_events.insert(*event);
        }
        for (next, next_events) in &message_event_sets {
            if next_events.contains(*event) {
                continue;
            }
            for next_event in next_events {
                if !reachable.contains_key(next_event) {
                    continue;
                }
                for index in indexes {
                    if index == next || message_event_sets[index].contains(next_event) {
                        continue;
                    }
                    let pair = (*index, *next);
                    precedence.insert(pair);
                    evidence
                        .entry(pair)
                        .or_insert_with(|| Some(((*event).clone(), next_event.clone())));
                }
            }
        }
    }
    for (index, source, target) in &message_events {
        // A receive-only event cycle is contradictory even if it cannot return to the send.
        for event in [source.as_ref(), target.as_ref()].into_iter().flatten() {
            if cyclic_events.contains(event) {
                let pair = (*index, *index);
                precedence.insert(pair);
                evidence
                    .entry(pair)
                    .or_insert_with(|| Some((event.clone(), event.clone())));
            }
        }
    }
    let order = canonical_sequence_order(&message_indexes, precedence.iter().copied());
    let graph = precedence.iter().fold(
        BTreeMap::<u32, BTreeSet<u32>>::new(),
        |mut graph, (a, b)| {
            graph.entry(*a).or_default().insert(*b);
            graph
        },
    );
    let cyclic = message_indexes
        .iter()
        .copied()
        .filter(|index| !order.contains_key(index))
        .filter(|index| directed_sequence_path(&graph, index, index).is_some())
        .collect::<BTreeSet<_>>();
    // Publish a real closed path for each cycle not already witnessed. Downstream messages
    // blocked by a cycle are distinct from messages that belong to it.
    let mut witnessed = BTreeSet::new();
    for index in &cyclic {
        if witnessed.contains(index) {
            continue;
        }
        let path = directed_sequence_path(&graph, index, index).expect("cyclic member has a path");
        witnessed.extend(path.iter().copied());
        let witness = path
            .windows(2)
            .map(|pair| {
                let event_ends = &evidence[&(pair[0], pair[1])];
                let events = event_ends
                    .as_ref()
                    .map(|(source, target)| {
                        directed_sequence_path(&event_successors, source, target)
                            .expect("precedence has an event path")
                    })
                    .unwrap_or_default();
                DiagramSequenceCycleStep {
                    source: elements[pair[0] as usize].occurrence_id.clone(),
                    target: elements[pair[1] as usize].occurrence_id.clone(),
                    authored: event_ends.is_none(),
                    events: events.into_boxed_slice(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        reasons.insert(DiagramIncompleteReason::SequenceOrderingCycle { witness });
    }

    let messages = messages
        .into_iter()
        .map(|(index, element)| {
            let source = endpoint(element, DiagramRelationshipKind::FlowSource);
            let target = endpoint(element, DiagramRelationshipKind::FlowTarget);
            if matches!(source, DiagramSequenceEndpoint::OutsideLifeline)
                || matches!(target, DiagramSequenceEndpoint::OutsideLifeline)
            {
                reasons.insert(DiagramIncompleteReason::SequenceMessageEndpointOutsideLifeline);
            }
            DiagramSequenceMessage {
                origin: index,
                label: element.name.clone(),
                source,
                target,
                order: order
                    .get(&index)
                    .copied()
                    .map(DiagramSequenceOrder::Resolved)
                    .unwrap_or_else(|| {
                        if cyclic.contains(&index) {
                            DiagramSequenceOrder::Cyclic
                        } else {
                            DiagramSequenceOrder::BlockedByCycle
                        }
                    }),
                provenance: if relationships
                    .iter()
                    .filter(|relationship| {
                        relationship.source == element.occurrence_id
                            && matches!(
                                relationship.kind,
                                DiagramRelationshipKind::FlowSource
                                    | DiagramRelationshipKind::FlowTarget
                            )
                    })
                    .all(|relationship| relationship.provenance == RelationshipProvenance::Authored)
                {
                    RelationshipProvenance::Authored
                } else {
                    RelationshipProvenance::Implied
                },
                source_location: element.source,
            }
        })
        .collect::<Vec<_>>();
    DiagramSequenceScene {
        lifelines: lifelines.into_boxed_slice(),
        messages: messages.into_boxed_slice(),
    }
}

/// Breadth-first predecessor tree. The source has a predecessor only for a real nonempty cycle.
fn directed_sequence_parents<N: Ord + Clone>(
    graph: &BTreeMap<N, BTreeSet<N>>,
    source: &N,
) -> BTreeMap<N, N> {
    let mut pending = VecDeque::from([source.clone()]);
    let mut parents = BTreeMap::new();
    let mut seen = BTreeSet::from([source.clone()]);
    while let Some(current) = pending.pop_front() {
        for next in graph.get(&current).into_iter().flatten() {
            if next == source {
                parents
                    .entry(next.clone())
                    .or_insert_with(|| current.clone());
            }
            if seen.insert(next.clone()) {
                parents.insert(next.clone(), current.clone());
                pending.push_back(next.clone());
            }
        }
    }
    parents
}

/// A shortest nonempty directed path, with stable neighbor order. Identity alone is not a path.
fn directed_sequence_path<N: Ord + Clone>(
    graph: &BTreeMap<N, BTreeSet<N>>,
    source: &N,
    target: &N,
) -> Option<Vec<N>> {
    let parents = directed_sequence_parents(graph, source);
    let mut current = parents.get(target)?.clone();
    let mut path = vec![target.clone(), current.clone()];
    while &current != source {
        current = parents.get(&current)?.clone();
        path.push(current.clone());
    }
    path.reverse();
    Some(path)
}

fn canonical_sequence_order(
    message_indexes: &BTreeSet<u32>,
    precedence: impl IntoIterator<Item = (u32, u32)>,
) -> BTreeMap<u32, u32> {
    let mut successors = BTreeMap::<u32, BTreeSet<u32>>::new();
    let mut incoming = message_indexes
        .iter()
        .map(|index| (*index, 0_u32))
        .collect::<BTreeMap<_, _>>();
    for (source, target) in precedence {
        if !message_indexes.contains(&source) || !message_indexes.contains(&target) {
            continue;
        }
        if successors.entry(source).or_default().insert(target) {
            *incoming
                .get_mut(&target)
                .expect("every message has an indegree") += 1;
        }
    }
    let mut ready = incoming
        .iter()
        .filter_map(|(index, degree)| (*degree == 0).then_some(*index))
        .collect::<BTreeSet<_>>();
    let mut order = BTreeMap::new();
    while let Some(index) = ready.pop_first() {
        order.insert(index, order.len() as u32 + 1);
        for successor in successors.get(&index).into_iter().flatten() {
            let degree = incoming
                .get_mut(successor)
                .expect("a successor is a projected message");
            *degree -= 1;
            if *degree == 0 {
                ready.insert(*successor);
            }
        }
    }
    order
}

/// The effect of a transition whose `do action a : A` binds `A` from the owned effect action.
fn transition_effect_of_owned_action(
    origin: &DiagramElement,
    relationships: &[DiagramRelationship],
    entries: &BTreeMap<SymbolId, SymbolEntry>,
) -> DiagramTransitionFeature {
    relationships
        .iter()
        .find(|relationship| {
            relationship.kind == DiagramRelationshipKind::TransitionEffect
                && relationship.source.semantic_path.len()
                    == origin.occurrence_id.semantic_path.len() + 1
                && relationship
                    .source
                    .semantic_path
                    .starts_with(&origin.occurrence_id.semantic_path)
        })
        .map_or(DiagramTransitionFeature::Absent, |relationship| {
            match &relationship.target {
                DiagramRelationshipTarget::Resolved(target) => DiagramTransitionFeature::Resolved {
                    // SysML 8.2.3.18 names the effect action; its type when it has no name.
                    label: entries
                        .get(&relationship.source.semantic_id())
                        .and_then(|entry| entry.name.clone())
                        .or_else(|| {
                            entries
                                .get(&target.semantic_id)
                                .and_then(|entry| entry.name.clone())
                        })
                        .unwrap_or_default(),
                    target: target.semantic_id,
                    source: relationship.source_location.unwrap_or(origin.source),
                },
                DiagramRelationshipTarget::Unresolved => DiagramTransitionFeature::Unresolved,
                DiagramRelationshipTarget::Ambiguous(_) => DiagramTransitionFeature::Ambiguous,
                DiagramRelationshipTarget::Unsupported => DiagramTransitionFeature::Unsupported,
            }
        })
}

fn transition_feature(
    origin: &DiagramElement,
    kind: DiagramRelationshipKind,
    relationships: &[DiagramRelationship],
    entries: &BTreeMap<SymbolId, SymbolEntry>,
) -> DiagramTransitionFeature {
    let Some(relationship) = relationships.iter().find(|relationship| {
        relationship.source == origin.occurrence_id && relationship.kind == kind
    }) else {
        return DiagramTransitionFeature::Absent;
    };
    match &relationship.target {
        DiagramRelationshipTarget::Resolved(target) => DiagramTransitionFeature::Resolved {
            label: entries
                .get(&target.semantic_id)
                .and_then(|entry| entry.name.clone())
                .unwrap_or_default(),
            target: target.semantic_id,
            source: relationship.source_location.unwrap_or(origin.source),
        },
        DiagramRelationshipTarget::Unresolved => DiagramTransitionFeature::Unresolved,
        DiagramRelationshipTarget::Ambiguous(_) => DiagramTransitionFeature::Ambiguous,
        DiagramRelationshipTarget::Unsupported => DiagramTransitionFeature::Unsupported,
    }
}

fn composed_edge(
    origin: u32,
    relationships: &[&DiagramRelationship],
    source_kind: DiagramRelationshipKind,
    target_kind: DiagramRelationshipKind,
    kind: DiagramEdgeKind,
) -> Option<DiagramEdge> {
    let source_relationship = relationships
        .iter()
        .find(|relationship| relationship.kind == source_kind)?;
    let target_relationship = relationships
        .iter()
        .find(|relationship| relationship.kind == target_kind)?;
    let source = resolved_target(&source_relationship.target)?;
    let target = resolved_target(&target_relationship.target)?;
    Some(edge_from_relationships(
        origin,
        source,
        target,
        kind,
        relationships,
    ))
}

fn edge_from_relationships(
    origin: u32,
    source: &DiagramOccurrenceIdentity,
    target: &DiagramOccurrenceIdentity,
    kind: DiagramEdgeKind,
    relationships: &[&DiagramRelationship],
) -> DiagramEdge {
    DiagramEdge {
        origin,
        source: source.clone(),
        source_semantic_id: source.semantic_id(),
        target: target.clone(),
        target_semantic_id: target.semantic_id(),
        kind,
        provenance: if relationships
            .iter()
            .all(|relationship| relationship.provenance == RelationshipProvenance::Authored)
        {
            RelationshipProvenance::Authored
        } else {
            RelationshipProvenance::Implied
        },
        source_location: relationships
            .iter()
            .find_map(|relationship| relationship.source_location),
    }
}

fn resolved_values<T>(outcome: QueryOutcome<Box<[T]>>) -> Box<[T]> {
    match outcome.answer {
        QueryAnswer::Resolved(values) => values,
        _ => Box::new([]),
    }
}

fn diagram_element_typing(outcome: QueryOutcome<ElementDetails>) -> DiagramElementTyping {
    match outcome.answer {
        QueryAnswer::Resolved(details) => {
            let types = details
                .typing
                .targets
                .iter()
                .map(|entry| entry.identity)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            match details.typing.outcome {
                RelationshipOutcome::NotApplicable => DiagramElementTyping::Absent,
                RelationshipOutcome::Resolved => DiagramElementTyping::Resolved(types),
                RelationshipOutcome::Partial => DiagramElementTyping::Partial(types),
                RelationshipOutcome::Unresolved => DiagramElementTyping::Unresolved,
                RelationshipOutcome::Ambiguous => DiagramElementTyping::Ambiguous(
                    details
                        .typing
                        .candidates
                        .iter()
                        .map(|entry| entry.identity)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
                RelationshipOutcome::Unsupported => DiagramElementTyping::Unsupported,
            }
        }
        QueryAnswer::Unresolved => DiagramElementTyping::Unresolved,
        QueryAnswer::Ambiguous(_) => DiagramElementTyping::Ambiguous(Box::default()),
        QueryAnswer::Unsupported => DiagramElementTyping::Unsupported,
        QueryAnswer::Recovery => DiagramElementTyping::Recovery,
        QueryAnswer::Incomplete => DiagramElementTyping::Incomplete,
    }
}

fn usable_value<T>(outcome: QueryOutcome<T>) -> Option<T> {
    match outcome.answer {
        QueryAnswer::Resolved(value) => Some(value),
        QueryAnswer::Unresolved
        | QueryAnswer::Ambiguous(_)
        | QueryAnswer::Unsupported
        | QueryAnswer::Recovery
        | QueryAnswer::Incomplete => None,
    }
}

/// The canonical reference-kind name the inspection publishes, as the diagram vocabulary variant.
/// The kind one canonical name states, or `None` where the name is not one of them.
fn relationship_kind_from_name(name: &str) -> Option<DiagramRelationshipKind> {
    DiagramRelationshipKind::ALL
        .iter()
        .copied()
        .find(|kind| kind.name() == name)
}

/// Whether an element is a port -- the only kind [`DiagramElement::direction`] and
/// [`DiagramElement::conjugated`] carry a fact for.
fn is_port_kind(kind: ElementKind) -> bool {
    matches!(kind, ElementKind::PortDefinition | ElementKind::PortUsage)
}

/// Whether an element is a node the Interconnection View draws: a part, a port, or a namespace
/// that contains one. `part-ref` is an ordinary `PartUsage` here.
fn is_interconnection_node(kind: ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::PartDefinition
            | ElementKind::PartUsage
            | ElementKind::PortDefinition
            | ElementKind::PortUsage
            | ElementKind::Namespace
            | ElementKind::Package
            | ElementKind::LibraryPackage
    )
}

/// Whether an element is one of the connector usages the Interconnection View draws as an edge
/// rather than a node. Its `memberAccessOperand` references are its connector ends (the dotted
/// `connect a.b to c.d` spelling), not expression operands.
fn is_binding_connector(kind: ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::BindingConnectorAsUsage | ElementKind::BindingConnector
    )
}

fn is_connector_edge_element(kind: ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::ConnectionUsage
            | ElementKind::InterfaceUsage
            | ElementKind::AllocationUsage
            | ElementKind::FlowConnectionUsage
            | ElementKind::SuccessionFlowUsage
            | ElementKind::BindingConnectorAsUsage
    )
}

/// The typed reason one unresolved connector end contributes to a projection's completeness.
fn connector_end_incomplete_reason(target: &DiagramRelationshipTarget) -> DiagramIncompleteReason {
    match target {
        DiagramRelationshipTarget::Ambiguous(_) => DiagramIncompleteReason::RelationshipAmbiguous {
            relationship: DiagramRelationshipKind::ConnectorEnd,
        },
        DiagramRelationshipTarget::Unsupported => {
            DiagramIncompleteReason::RelationshipUnsupported {
                relationship: DiagramRelationshipKind::ConnectorEnd,
            }
        }
        // Unresolved, or resolved to an element outside this projection: either way the diagram
        // has no node to attach the end to.
        DiagramRelationshipTarget::Unresolved | DiagramRelationshipTarget::Resolved(_) => {
            DiagramIncompleteReason::RelationshipUnresolved {
                relationship: DiagramRelationshipKind::ConnectorEnd,
            }
        }
    }
}

/// The General View edge kind for one specialization-family relationship, or `None` for a
/// relationship kind the view does not draw as a line (`typeFeaturing`,
/// unions, and the rest). The names match `spec42_generator_protocol::RelationshipKind` so the
/// renderer applies the SysML 8.2.3.6 glyph for each.
fn general_specialization_edge_kind(kind: DiagramRelationshipKind) -> Option<&'static str> {
    match kind {
        DiagramRelationshipKind::Specialization => Some("specializes"),
        DiagramRelationshipKind::Subsetting => Some("subsetting"),
        DiagramRelationshipKind::Redefinition => Some("redefinition"),
        DiagramRelationshipKind::FeatureTyping => Some("typing"),
        DiagramRelationshipKind::ReferenceSubsetting => Some("reference-subsetting"),
        _ => None,
    }
}

/// Directional end roles are semantic facts; the drawing never infers them from labels.
fn general_binary_relationship(
    kind: ElementKind,
) -> Option<(
    DiagramRelationshipKind,
    DiagramRelationshipKind,
    &'static str,
)> {
    use DiagramRelationshipKind as R;
    match kind {
        ElementKind::AllocationUsage => Some((R::AllocateSource, R::AllocateTarget, "allocate")),
        ElementKind::Dependency => Some((R::DependencyClient, R::DependencySupplier, "dependency")),
        ElementKind::BindingConnectorAsUsage | ElementKind::BindingConnector => {
            Some((R::BindSource, R::BindTarget, "bind"))
        }
        _ => None,
    }
}

/// The suffix a scene id carries for an edge of this kind.
fn edge_scene_suffix(kind: &DiagramEdgeKind) -> &'static str {
    match kind {
        DiagramEdgeKind::Containment => "containment",
        DiagramEdgeKind::InitialState => "initial",
        _ => "edge",
    }
}

impl DiagramViewProjection {
    /// The boundary form of a transition's scene id: the origin occurrence's settled key and the
    /// role suffix. Rendered where a protocol needs text; the projection itself carries handles.
    pub fn transition_scene_id(&self, transition: &DiagramStateTransition) -> Option<String> {
        let origin = self.elements.get(transition.origin as usize)?;
        Some(format!(
            "{}#{}",
            origin.occurrence_id.stable_key(),
            transition.role.scene_suffix()
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        authored_chain_score, canonical_sequence_order, compartment_kind, directed_sequence_path,
        usable_value, DiagramCompartmentKind,
    };
    use crate::{
        ElementKind, PublicationCompleteness, PublicationObstacle, QueryAnswer, QueryOutcome,
    };

    #[test]
    fn unsupported_outcomes_retain_their_usable_payload() {
        let completeness = PublicationCompleteness::Complete
            .with(PublicationObstacle::ParseRecovery)
            .with(PublicationObstacle::UnsupportedSyntax);
        assert_eq!(
            usable_value(QueryOutcome::new(completeness, QueryAnswer::Resolved(42))),
            Some(42)
        );
        assert_eq!(
            usable_value::<u8>(QueryOutcome::new(completeness, QueryAnswer::Unsupported)),
            None
        );
    }

    #[test]
    fn compartment_families_are_owned_by_the_semantic_projection() {
        assert_eq!(
            compartment_kind(ElementKind::AttributeUsage),
            Some(DiagramCompartmentKind::Attributes)
        );
        assert_eq!(
            compartment_kind(ElementKind::PartUsage),
            Some(DiagramCompartmentKind::Parts)
        );
        assert_eq!(
            compartment_kind(ElementKind::PortUsage),
            Some(DiagramCompartmentKind::Ports)
        );
        assert_eq!(
            compartment_kind(ElementKind::ConstraintUsage),
            Some(DiagramCompartmentKind::Constraints)
        );
        assert_eq!(
            compartment_kind(ElementKind::RequirementUsage),
            Some(DiagramCompartmentKind::Requirements)
        );
        assert_eq!(
            compartment_kind(ElementKind::ActionUsage),
            Some(DiagramCompartmentKind::Actions)
        );
        assert_eq!(
            compartment_kind(ElementKind::StateUsage),
            Some(DiagramCompartmentKind::States)
        );
        assert_eq!(
            compartment_kind(ElementKind::CalculationUsage),
            Some(DiagramCompartmentKind::Calculations)
        );
        assert_eq!(
            compartment_kind(ElementKind::ConnectionUsage),
            Some(DiagramCompartmentKind::Connections)
        );
        assert_eq!(
            compartment_kind(ElementKind::InterfaceUsage),
            Some(DiagramCompartmentKind::Interfaces)
        );
        assert_eq!(
            compartment_kind(ElementKind::OccurrenceUsage),
            Some(DiagramCompartmentKind::Occurrences)
        );
        assert_eq!(compartment_kind(ElementKind::PartDefinition), None);
    }

    #[test]
    fn sequence_order_respects_every_predecessor_in_a_diamond() {
        let messages = BTreeSet::from([0, 1, 2, 3]);
        let order = canonical_sequence_order(&messages, [(0, 1), (0, 2), (1, 3), (2, 3)]);
        assert_eq!(order.len(), 4);
        for (before, after) in [(0, 1), (0, 2), (1, 3), (2, 3)] {
            assert!(order[&before] < order[&after]);
        }
    }

    #[test]
    fn sequence_order_leaves_cycles_explicit() {
        let messages = BTreeSet::from([0, 1, 2]);
        let order = canonical_sequence_order(&messages, [(0, 1), (1, 0)]);
        assert_eq!(order, [(2, 1)].into_iter().collect());
    }

    #[test]
    fn a_shared_endpoint_is_not_a_nonempty_ordering_path() {
        let graph = [(0, BTreeSet::from([1, 2])), (3, BTreeSet::from([1]))]
            .into_iter()
            .collect();
        assert_eq!(directed_sequence_path(&graph, &0, &0), None);
        assert_eq!(directed_sequence_path(&graph, &0, &3), None);
        assert_eq!(directed_sequence_path(&graph, &3, &0), None);
        assert_eq!(directed_sequence_path(&graph, &0, &1), Some(vec![0, 1]));
    }

    #[test]
    fn cycle_witness_is_closed_and_does_not_include_a_blocked_successor() {
        let graph = [(0, BTreeSet::from([1])), (1, BTreeSet::from([0, 2]))]
            .into_iter()
            .collect();
        assert_eq!(directed_sequence_path(&graph, &0, &0), Some(vec![0, 1, 0]));
        assert_eq!(directed_sequence_path(&graph, &2, &2), None);
        let graph = [(0, BTreeSet::from([0]))].into_iter().collect();
        assert_eq!(directed_sequence_path(&graph, &0, &0), Some(vec![0, 0]));
    }

    #[test]
    fn authored_chain_score_uses_intermediate_segments() {
        let unit1 = ["vehicle", "propulsion", "propulsionUnit1", "cmd"];
        let unit2 = ["vehicle", "propulsion", "propulsionUnit2", "cmd"];
        assert_eq!(
            authored_chain_score(Some("propulsion::propulsionUnit1::cmd"), &unit1),
            3
        );
        assert_eq!(
            authored_chain_score(Some("propulsion::propulsionUnit1::cmd"), &unit2),
            1
        );
        assert_eq!(authored_chain_score(Some("pcb"), &["left", "pcb"]), 1);
        assert_eq!(authored_chain_score(None, &unit1), 0);
    }
}
