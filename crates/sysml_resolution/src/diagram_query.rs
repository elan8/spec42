use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{
    ElementDetails, ElementKind, ElementSearch, ElementSource, PublishedResolution, QueryAnswer,
    QueryOutcome, RelationshipOutcome, RelationshipProvenance, RelationshipTarget, SourceLocation,
    SymbolEntry, SymbolId, ViewSelectionObstacle, ViewSelectionOutcome,
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
    SequenceOrderingCycle,
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
    pub name: Option<Box<str>>,
    /// Authored FeatureTyping for compact labels; not the effective-type closure.
    pub typing: DiagramElementTyping,
    pub owner: Option<DiagramOccurrenceIdentity>,
    pub source: SourceLocation,
    pub compartments: Box<[DiagramCompartment]>,
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
            for ty in types {
                let Some(kind) = definitions.get(&ty.symbol).copied() else {
                    continue;
                };
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

    pub fn diagram_view(&self, view: SymbolId) -> QueryOutcome<DiagramViewProjection> {
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
            .find(|entry| entry.semantic_id == view)
            .cloned()
        else {
            return self.model.query_outcome(QueryAnswer::Unresolved);
        };

        let all = self.diagram_entries();
        let workspace = self
            .diagram_entries_for(ElementSource::Workspace)
            .into_keys()
            .collect::<BTreeSet<_>>();
        let mut roots = BTreeSet::new();
        let mut reasons = BTreeSet::new();
        for expose in all
            .values()
            .filter(|entry| entry.owner == Some(view) && entry.kind == ElementKind::Expose)
        {
            match self.inspect(expose.identity).answer {
                QueryAnswer::Resolved(inspection) => {
                    for relationship in inspection
                        .relationships
                        .iter()
                        .filter(|relationship| relationship.kind == "viewExpose")
                    {
                        match &relationship.target {
                            RelationshipTarget::Resolved(target) => {
                                roots.insert(*target);
                            }
                            RelationshipTarget::Ambiguous(_) => {
                                reasons.insert(DiagramIncompleteReason::ExposureAmbiguous {
                                    exposure: expose.identity,
                                });
                            }
                            RelationshipTarget::Unresolved => {
                                reasons.insert(DiagramIncompleteReason::ExposureUnresolved {
                                    exposure: expose.identity,
                                });
                            }
                            RelationshipTarget::Unsupported => {
                                reasons.insert(DiagramIncompleteReason::ExposureUnsupported {
                                    exposure: expose.identity,
                                });
                            }
                        }
                    }
                }
                QueryAnswer::Unresolved => {
                    reasons.insert(DiagramIncompleteReason::ExposureUnresolved {
                        exposure: expose.identity,
                    });
                }
                QueryAnswer::Ambiguous(_) => {
                    reasons.insert(DiagramIncompleteReason::ExposureAmbiguous {
                        exposure: expose.identity,
                    });
                }
                _ => {
                    reasons.insert(DiagramIncompleteReason::ExposureUnsupported {
                        exposure: expose.identity,
                    });
                }
            }
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
        let mut queue = VecDeque::new();
        for root in &roots {
            if !self.diagram_candidate_selected(view, *root, &mut reasons) {
                continue;
            }
            // The scene id of an occurrence is published text, so the boundary token is taken
            // here, once, where the publication is in hand.
            let Some(token) = self.symbol_token(*root) else {
                continue;
            };
            let occurrence = DiagramOccurrenceIdentity::root(*root, token.as_str());
            occurrences.insert(occurrence.clone(), (*root, None));
            queue.push_back(occurrence);
        }
        while let Some(owner_occurrence) = queue.pop_front() {
            let owner = occurrences
                .get(&owner_occurrence)
                .expect("queued diagram occurrence must exist")
                .0;
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
            for child in children {
                if !workspace.contains(&child.identity) {
                    continue;
                }
                if !self.diagram_candidate_selected(view, child.identity, &mut reasons) {
                    continue;
                }
                let Some(token) = self.symbol_token(child.identity) else {
                    continue;
                };
                let occurrence = owner_occurrence.child(child.identity, token.as_str());
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
        }
        let projected_roots = occurrences
            .iter()
            .filter_map(|(_, (identity, owner))| owner.is_none().then_some(*identity))
            .collect::<BTreeSet<_>>();
        let mut elements = occurrences
            .iter()
            .filter_map(|(occurrence_id, (identity, owner))| {
                all.get(identity).map(|entry| (occurrence_id, owner, entry))
            })
            .map(|(occurrence_id, owner, entry)| DiagramElement {
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
                name: entry.name.clone(),
                typing: diagram_element_typing(self.element_details(entry.identity)),
                owner: owner.clone(),
                source: entry.location,
                compartments: Box::default(),
            })
            .collect::<Vec<_>>();
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
                            DiagramRelationshipTarget::Resolved(contextual_endpoint(
                                &element.occurrence_id,
                                *target,
                                &elements,
                            ))
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
        let mut edges = elements
            .iter()
            .enumerate()
            .filter_map(|(origin, element)| {
                element.owner.as_ref().map(|owner| DiagramEdge {
                    origin: origin as u32,
                    source: owner.clone(),
                    source_semantic_id: owner.semantic_id(),
                    target: element.occurrence_id.clone(),
                    target_semantic_id: element.semantic_id,
                    kind: DiagramEdgeKind::Containment,
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
            // A `connect a to b` usage publishes its two ends as `connectorEnd` relationships; the
            // dotted `connect a.b to c.d` form publishes them as `memberAccessOperand` on the same
            // connection usage. Compose one Connector edge from whichever pair the usage carries so
            // a connection is an edge, not a peer box. An end that is unresolved, ambiguous, or
            // resolved outside the projection is a typed incomplete reason, never a guessed line.
            {
                // The dotted `connect a.b to c.d` ends and the typed incomplete reasons are an
                // Interconnection View concern; other views keep the prior plain-`connectorEnd`
                // behaviour untouched.
                let dotted_ends = view_entry.kind == DiagramViewKind::Interconnection
                    && is_connector_edge_element(element.kind);
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
            // SysML 8.2.3.6: the General View draws authored subclassification, subsetting,
            // redefinition, and feature typing between two elements it projects. Implied library
            // subsetting (`Parts::parts`, `Items::items`, the kernel chain) stays off the canvas --
            // the typed facts remain on `relationships` for inspectors -- and an end that is
            // unresolved, ambiguous, or outside the view yields no edge rather than a guessed line.
            if view_entry.kind == DiagramViewKind::General {
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
        DiagramViewKind::General
        | DiagramViewKind::Browser
        | DiagramViewKind::Grid
        | DiagramViewKind::Geometry => false,
    }
}

/// The boundary form of one element reference.
///
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

fn contextual_endpoint(
    source: &DiagramOccurrenceIdentity,
    semantic_id: SymbolId,
    elements: &[DiagramElement],
) -> DiagramRelationshipEndpoint {
    let mut candidates = elements
        .iter()
        .filter(|element| element.semantic_id == semantic_id)
        .map(|element| {
            let shared = source
                .semantic_path
                .iter()
                .zip(element.occurrence_id.semantic_path.iter())
                .take_while(|(left, right)| left == right)
                .count();
            (shared, element.occurrence_id.clone())
        })
        .collect::<Vec<_>>();
    candidates.sort();
    let occurrence = match candidates.last().map(|candidate| candidate.0) {
        None => DiagramEndpointOccurrence::OutsideProjection,
        Some(best) => {
            let best = candidates
                .into_iter()
                .filter_map(|(shared, occurrence)| (shared == best).then_some(occurrence))
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
        ElementKind::StateUsage | ElementKind::FinalState => Compartment::States,
        ElementKind::CalculationUsage => Compartment::Calculations,
        ElementKind::ConnectionUsage
        | ElementKind::FlowConnectionUsage
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
                            ElementKind::StateUsage => DiagramStateVertexKind::State,
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
                        // No reference kind publishes a transition guard, so the guard slot is
                        // always absent. The enum makes that explicit; the string form silently
                        // looked for a name the authority never emits.
                        guard: DiagramTransitionFeature::Absent,
                        effect: feature(DiagramRelationshipKind::TransitionEffect),
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
    let lifelines = elements
        .iter()
        .filter(is_direct_member)
        .filter(|element| {
            matches!(
                element.kind,
                ElementKind::PartUsage | ElementKind::PortUsage
            )
        })
        .map(|element| element.occurrence_id.clone())
        .collect::<Vec<_>>();
    let lifeline_set = lifelines.iter().cloned().collect::<BTreeSet<_>>();
    let messages = elements
        .iter()
        .enumerate()
        .filter(|(_, element)| element.kind == ElementKind::FlowConnectionUsage)
        .filter(|(_, element)| is_direct_member(element))
        .map(|(index, element)| (index as u32, element))
        .collect::<Vec<_>>();

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
    let precedence = edges
        .iter()
        .filter(|edge| edge.kind == DiagramEdgeKind::Succession)
        .filter_map(|edge| {
            Some((
                *occurrence_to_index.get(&edge.source)?,
                *occurrence_to_index.get(&edge.target)?,
            ))
        });
    let order = canonical_sequence_order(&message_indexes, precedence);
    if order.len() != messages.len() {
        reasons.insert(DiagramIncompleteReason::SequenceOrderingCycle);
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
                    .unwrap_or(DiagramSequenceOrder::Cyclic),
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
    match name {
        "namespaceImport" => Some(DiagramRelationshipKind::NamespaceImport),
        "membershipImport" => Some(DiagramRelationshipKind::MembershipImport),
        "filterImport" => Some(DiagramRelationshipKind::FilterImport),
        "featureTyping" => Some(DiagramRelationshipKind::FeatureTyping),
        "typeFeaturing" => Some(DiagramRelationshipKind::TypeFeaturing),
        "featureChaining" => Some(DiagramRelationshipKind::FeatureChaining),
        "specialization" => Some(DiagramRelationshipKind::Specialization),
        "subsetting" => Some(DiagramRelationshipKind::Subsetting),
        "redefinition" => Some(DiagramRelationshipKind::Redefinition),
        "referenceSubsetting" => Some(DiagramRelationshipKind::ReferenceSubsetting),
        "crossSubsetting" => Some(DiagramRelationshipKind::CrossSubsetting),
        "intersects" => Some(DiagramRelationshipKind::Intersects),
        "unioning" => Some(DiagramRelationshipKind::Unioning),
        "intersecting" => Some(DiagramRelationshipKind::Intersecting),
        "differencing" => Some(DiagramRelationshipKind::Differencing),
        "disjoining" => Some(DiagramRelationshipKind::Disjoining),
        "aliasBinding" => Some(DiagramRelationshipKind::AliasBinding),
        "connectorEnd" => Some(DiagramRelationshipKind::ConnectorEnd),
        "succession" => Some(DiagramRelationshipKind::Succession),
        "entryActionBinding" => Some(DiagramRelationshipKind::EntryActionBinding),
        "doActionBinding" => Some(DiagramRelationshipKind::DoActionBinding),
        "exitActionBinding" => Some(DiagramRelationshipKind::ExitActionBinding),
        "initialState" => Some(DiagramRelationshipKind::InitialState),
        "expressionOperand" => Some(DiagramRelationshipKind::ExpressionOperand),
        "transitionSource" => Some(DiagramRelationshipKind::TransitionSource),
        "transitionTarget" => Some(DiagramRelationshipKind::TransitionTarget),
        "transitionTrigger" => Some(DiagramRelationshipKind::TransitionTrigger),
        "transitionEffect" => Some(DiagramRelationshipKind::TransitionEffect),
        "metadataAnnotation" => Some(DiagramRelationshipKind::MetadataAnnotation),
        "filterMetadataTest" => Some(DiagramRelationshipKind::FilterMetadataTest),
        "satisfySource" => Some(DiagramRelationshipKind::SatisfySource),
        "satisfyTarget" => Some(DiagramRelationshipKind::SatisfyTarget),
        "allocateSource" => Some(DiagramRelationshipKind::AllocateSource),
        "allocateTarget" => Some(DiagramRelationshipKind::AllocateTarget),
        "bindSource" => Some(DiagramRelationshipKind::BindSource),
        "bindTarget" => Some(DiagramRelationshipKind::BindTarget),
        "variant" => Some(DiagramRelationshipKind::Variant),
        "includeUseCase" => Some(DiagramRelationshipKind::IncludeUseCase),
        "viewExpose" => Some(DiagramRelationshipKind::ViewExpose),
        "memberAccessOperand" => Some(DiagramRelationshipKind::MemberAccessOperand),
        "invocationCallee" => Some(DiagramRelationshipKind::InvocationCallee),
        "thenTarget" => Some(DiagramRelationshipKind::ThenTarget),
        "acceptVia" => Some(DiagramRelationshipKind::AcceptVia),
        "sendTarget" => Some(DiagramRelationshipKind::SendTarget),
        "acceptPayloadType" => Some(DiagramRelationshipKind::AcceptPayloadType),
        "terminateTarget" => Some(DiagramRelationshipKind::TerminateTarget),
        "flowSource" => Some(DiagramRelationshipKind::FlowSource),
        "flowTarget" => Some(DiagramRelationshipKind::FlowTarget),
        "typeCheckTarget" => Some(DiagramRelationshipKind::TypeCheckTarget),
        "metaCastTarget" => Some(DiagramRelationshipKind::MetaCastTarget),
        "stakeholderTarget" => Some(DiagramRelationshipKind::StakeholderTarget),
        "purposeTarget" => Some(DiagramRelationshipKind::PurposeTarget),
        "verifyRequirementTarget" => Some(DiagramRelationshipKind::VerifyRequirementTarget),
        "assignTarget" => Some(DiagramRelationshipKind::AssignTarget),
        "dependencyClient" => Some(DiagramRelationshipKind::DependencyClient),
        "dependencySupplier" => Some(DiagramRelationshipKind::DependencySupplier),
        "performParameterTarget" => Some(DiagramRelationshipKind::PerformParameterTarget),
        "flowPayloadType" => Some(DiagramRelationshipKind::FlowPayloadType),
        _ => None,
    }
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
fn is_connector_edge_element(kind: ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::ConnectionUsage
            | ElementKind::InterfaceUsage
            | ElementKind::AllocationUsage
            | ElementKind::FlowConnectionUsage
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
/// relationship kind the view does not draw as a line (`typeFeaturing`, `referenceSubsetting`,
/// unions, and the rest). The names match `spec42_generator_protocol::RelationshipKind` so the
/// renderer applies the SysML 8.2.3.6 glyph for each.
fn general_specialization_edge_kind(kind: DiagramRelationshipKind) -> Option<&'static str> {
    match kind {
        DiagramRelationshipKind::Specialization => Some("specializes"),
        DiagramRelationshipKind::Subsetting => Some("subsetting"),
        DiagramRelationshipKind::Redefinition => Some("redefinition"),
        DiagramRelationshipKind::FeatureTyping => Some("typing"),
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

    use super::{canonical_sequence_order, compartment_kind, usable_value, DiagramCompartmentKind};
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
}
