use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{
    ElementDetails, ElementKind, ElementSearch, ElementSource, PublishedResolution, QueryOutcome,
    RelationshipOutcome, RelationshipProvenance, RelationshipTarget, SourceLocation, SymbolEntry,
    SymbolId, ViewSelectionObstacle, ViewSelectionOutcome,
};

pub use sysml_contract::{
    DiagramCompartmentKind, DiagramCompartmentProvenance, DiagramStateVertexKind, DiagramViewKind,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramViewCatalogEntry {
    pub kind: DiagramViewKind,
    pub semantic_id: SymbolId,
    pub reference: DiagramSemanticReference,
    pub name: Box<str>,
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
    ExposureUnresolved { exposure: SymbolId },
    ExposureAmbiguous { exposure: SymbolId },
    ExposureUnsupported { exposure: SymbolId },
    RelationshipUnresolved { relationship: Box<str> },
    RelationshipAmbiguous { relationship: Box<str> },
    RelationshipUnsupported { relationship: Box<str> },
    ViewFilterUnresolved,
    ViewFilterAmbiguous,
    ViewFilterUnsupported,
    GeometryFactsUnavailable,
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

    fn stable_key(&self) -> &str {
        &self.key
    }
}

fn push_key_segment(key: &mut String, token: &str) {
    key.push_str(&token.len().to_string());
    key.push(':');
    key.push_str(token);
}

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
    pub semantic_id: Box<str>,
    pub source: DiagramOccurrenceIdentity,
    pub source_semantic_id: SymbolId,
    pub kind: Box<str>,
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
    pub semantic_id: Box<str>,
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
    Sequence,
    Browser,
    Grid,
    Geometry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramStateTransitionScene {
    pub machine: Option<SymbolId>,
    pub vertices: Box<[DiagramStateVertex]>,
    pub transitions: Box<[DiagramStateTransition]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramStateVertex {
    pub semantic_id: SymbolId,
    pub label: Box<str>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagramStateTransition {
    pub semantic_id: Box<str>,
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
                return QueryOutcome::Ambiguous(Box::new([]));
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
                    reference: semantic_reference(&entry, &BTreeMap::new()),
                    name: entry
                        .name
                        .clone()
                        .unwrap_or_else(|| entry.qualified_name.clone()),
                    source: entry.location.clone(),
                });
            }
        }
        // A handle sorts as its canonical identity does, so this is the identity order the
        // catalog has always published, without materialising either string.
        catalog.sort_by_key(|a| a.semantic_id);
        QueryOutcome::Resolved(catalog.into_boxed_slice())
    }

    pub fn diagram_view(&self, view: SymbolId) -> QueryOutcome<DiagramViewProjection> {
        let catalog = match self.diagram_view_catalog() {
            QueryOutcome::Resolved(value) | QueryOutcome::Recovered(value) => value,
            QueryOutcome::Ambiguous(_) => return QueryOutcome::Ambiguous(Box::new([])),
            QueryOutcome::UnsupportedWith(_) | QueryOutcome::Unsupported => {
                return QueryOutcome::Unsupported;
            }
            QueryOutcome::Unresolved => return QueryOutcome::Unresolved,
            QueryOutcome::Recovery => return QueryOutcome::Recovery,
            QueryOutcome::Incomplete => return QueryOutcome::Incomplete,
        };
        let Some(view_entry) = catalog
            .iter()
            .find(|entry| entry.semantic_id == view)
            .cloned()
        else {
            return QueryOutcome::Unresolved;
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
            match self.inspect(expose.identity) {
                QueryOutcome::Resolved(inspection)
                | QueryOutcome::Recovered(inspection)
                | QueryOutcome::UnsupportedWith(inspection) => {
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
                QueryOutcome::Unresolved => {
                    reasons.insert(DiagramIncompleteReason::ExposureUnresolved {
                        exposure: expose.identity,
                    });
                }
                QueryOutcome::Ambiguous(_) => {
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
                reference: semantic_reference(entry, &all),
                kind: entry.kind,
                name: entry.name.clone(),
                typing: diagram_element_typing(self.element_details(entry.identity)),
                owner: owner.clone(),
                source: entry.location.clone(),
                compartments: Box::default(),
            })
            .collect::<Vec<_>>();
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
                    let target = match &relationship.target {
                        RelationshipTarget::Resolved(target) => {
                            DiagramRelationshipTarget::Resolved(contextual_endpoint(
                                &element.occurrence_id,
                                *target,
                                &elements,
                            ))
                        }
                        RelationshipTarget::Ambiguous(candidates) => {
                            if relationship_is_required(view_entry.kind, relationship.kind) {
                                reasons.insert(DiagramIncompleteReason::RelationshipAmbiguous {
                                    relationship: relationship.kind.into(),
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
                            if relationship_is_required(view_entry.kind, relationship.kind) {
                                reasons.insert(DiagramIncompleteReason::RelationshipUnresolved {
                                    relationship: relationship.kind.into(),
                                });
                            }
                            DiagramRelationshipTarget::Unresolved
                        }
                        RelationshipTarget::Unsupported => {
                            if relationship_is_required(view_entry.kind, relationship.kind) {
                                reasons.insert(DiagramIncompleteReason::RelationshipUnsupported {
                                    relationship: relationship.kind.into(),
                                });
                            }
                            DiagramRelationshipTarget::Unsupported
                        }
                    };
                    relationships.push(DiagramRelationship {
                        semantic_id: format!(
                            "{}#{}:{index}",
                            element.occurrence_id.stable_key(),
                            relationship.kind
                        )
                        .into(),
                        source: element.occurrence_id.clone(),
                        source_semantic_id: element.semantic_id,
                        kind: relationship.kind.into(),
                        target,
                        provenance: relationship.provenance,
                        source_location: relationship.location.clone(),
                    });
                }
            }
        }
        relationships.sort_by(|a, b| a.semantic_id.cmp(&b.semantic_id));
        let mut edges = elements
            .iter()
            .filter_map(|element| {
                element.owner.as_ref().map(|owner| DiagramEdge {
                    semantic_id: format!("{}#containment", element.occurrence_id.stable_key(),)
                        .into(),
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
                    source_location: Some(element.source.clone()),
                })
            })
            .collect::<Vec<_>>();
        for element in &elements {
            let outgoing = relationships
                .iter()
                .filter(|relationship| relationship.source == element.occurrence_id)
                .collect::<Vec<_>>();
            if let Some(edge) = composed_edge(
                element,
                &outgoing,
                "transitionSource",
                "transitionTarget",
                DiagramEdgeKind::Transition,
            ) {
                edges.push(edge);
                continue;
            }
            if let Some(edge) = composed_edge(
                element,
                &outgoing,
                "flowSource",
                "flowTarget",
                DiagramEdgeKind::Flow,
            ) {
                edges.push(edge);
                continue;
            }
            for (relationship_kind, edge_kind) in [
                ("connectorEnd", DiagramEdgeKind::Connector),
                ("succession", DiagramEdgeKind::Succession),
            ] {
                let endpoints = outgoing
                    .iter()
                    .filter(|relationship| relationship.kind.as_ref() == relationship_kind)
                    .filter_map(|relationship| resolved_target(&relationship.target))
                    .collect::<Vec<_>>();
                if let [source, target] = endpoints.as_slice() {
                    edges.push(edge_from_relationships(
                        element, source, target, edge_kind, &outgoing,
                    ));
                }
            }
            if let Some(initial) = outgoing
                .iter()
                .find(|relationship| relationship.kind.as_ref() == "initialState")
            {
                if let Some(target) = resolved_target(&initial.target) {
                    edges.push(DiagramEdge {
                        semantic_id: format!("{}#initial", element.occurrence_id.stable_key())
                            .into(),
                        source: element.occurrence_id.clone(),
                        source_semantic_id: element.semantic_id,
                        target: target.clone(),
                        target_semantic_id: target.semantic_id(),
                        kind: DiagramEdgeKind::InitialState,
                        provenance: initial.provenance,
                        source_location: initial.source_location.clone(),
                    });
                }
            }
        }
        edges.sort_by(|a, b| a.semantic_id.cmp(&b.semantic_id));
        let scene = diagram_scene(
            view_entry.kind,
            &projected_roots,
            &elements,
            &relationships,
            &edges,
            &all,
        );
        if view_entry.kind == DiagramViewKind::Geometry && !elements.is_empty() {
            reasons.insert(DiagramIncompleteReason::GeometryFactsUnavailable);
        }
        QueryOutcome::Resolved(DiagramViewProjection {
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
        })
    }

    fn diagram_candidate_selected(
        &self,
        view: SymbolId,
        candidate: SymbolId,
        reasons: &mut BTreeSet<DiagramIncompleteReason>,
    ) -> bool {
        match self.view_selection(view, candidate) {
            QueryOutcome::Resolved(selection)
            | QueryOutcome::Recovered(selection)
            | QueryOutcome::UnsupportedWith(selection) => match selection.outcome {
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
            QueryOutcome::Ambiguous(_) => {
                reasons.insert(DiagramIncompleteReason::ViewFilterAmbiguous);
                false
            }
            QueryOutcome::Unresolved => {
                reasons.insert(DiagramIncompleteReason::ViewFilterUnresolved);
                false
            }
            QueryOutcome::Unsupported | QueryOutcome::Recovery | QueryOutcome::Incomplete => {
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

fn relationship_is_required(view: DiagramViewKind, kind: &str) -> bool {
    match view {
        DiagramViewKind::Interconnection => kind == "connectorEnd",
        DiagramViewKind::ActionFlow => matches!(kind, "flowSource" | "flowTarget" | "succession"),
        DiagramViewKind::StateTransition => matches!(
            kind,
            "initialState"
                | "transitionSource"
                | "transitionTarget"
                | "transitionTrigger"
                | "transitionGuard"
                | "transitionEffect"
        ),
        DiagramViewKind::Sequence => matches!(kind, "messageSource" | "messageTarget"),
        DiagramViewKind::General
        | DiagramViewKind::Browser
        | DiagramViewKind::Grid
        | DiagramViewKind::Geometry => false,
    }
}

fn semantic_reference(
    entry: &SymbolEntry,
    entries: &BTreeMap<SymbolId, SymbolEntry>,
) -> DiagramSemanticReference {
    if entry.name.is_some() {
        DiagramSemanticReference::Qualified {
            document: entry.location.document.clone(),
            qualified_name: entry.qualified_name.clone(),
        }
    } else {
        DiagramSemanticReference::SourceAnchor {
            document: entry.location.document.clone(),
            owner_qualified_name: entry
                .owner
                .as_ref()
                .and_then(|owner| entries.get(owner))
                .map(|owner| owner.qualified_name.clone()),
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
) -> DiagramScene {
    match kind {
        DiagramViewKind::General => DiagramScene::General,
        DiagramViewKind::Interconnection => DiagramScene::Interconnection,
        DiagramViewKind::ActionFlow => DiagramScene::ActionFlow,
        DiagramViewKind::Sequence => DiagramScene::Sequence,
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
                        label: element.name.clone().unwrap_or_default(),
                        kind,
                        source: element.source.clone(),
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
                    let origin = elements.iter().find(|element| {
                        edge.semantic_id.as_ref()
                            == format!("{}#edge", element.occurrence_id.stable_key())
                            || edge.semantic_id.as_ref()
                                == format!("{}#initial", element.occurrence_id.stable_key())
                    })?;
                    let feature = |relationship_kind: &str| {
                        transition_feature(origin, relationship_kind, relationships, entries)
                    };
                    Some(DiagramStateTransition {
                        semantic_id: edge.semantic_id.clone(),
                        label: origin.name.clone(),
                        source: edge.source_semantic_id,
                        target: edge.target_semantic_id,
                        trigger: if edge.kind == DiagramEdgeKind::InitialState {
                            DiagramTransitionFeature::Absent
                        } else {
                            feature("transitionTrigger")
                        },
                        guard: feature("transitionGuard"),
                        effect: feature("transitionEffect"),
                        provenance: edge.provenance,
                        source_location: edge
                            .source_location
                            .clone()
                            .unwrap_or_else(|| origin.source.clone()),
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

fn transition_feature(
    origin: &DiagramElement,
    kind: &str,
    relationships: &[DiagramRelationship],
    entries: &BTreeMap<SymbolId, SymbolEntry>,
) -> DiagramTransitionFeature {
    let Some(relationship) = relationships.iter().find(|relationship| {
        relationship.source == origin.occurrence_id && relationship.kind.as_ref() == kind
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
            source: relationship
                .source_location
                .clone()
                .unwrap_or_else(|| origin.source.clone()),
        },
        DiagramRelationshipTarget::Unresolved => DiagramTransitionFeature::Unresolved,
        DiagramRelationshipTarget::Ambiguous(_) => DiagramTransitionFeature::Ambiguous,
        DiagramRelationshipTarget::Unsupported => DiagramTransitionFeature::Unsupported,
    }
}

fn composed_edge(
    element: &DiagramElement,
    relationships: &[&DiagramRelationship],
    source_kind: &str,
    target_kind: &str,
    kind: DiagramEdgeKind,
) -> Option<DiagramEdge> {
    let source_relationship = relationships
        .iter()
        .find(|relationship| relationship.kind.as_ref() == source_kind)?;
    let target_relationship = relationships
        .iter()
        .find(|relationship| relationship.kind.as_ref() == target_kind)?;
    let source = resolved_target(&source_relationship.target)?;
    let target = resolved_target(&target_relationship.target)?;
    Some(edge_from_relationships(
        element,
        source,
        target,
        kind,
        relationships,
    ))
}

fn edge_from_relationships(
    element: &DiagramElement,
    source: &DiagramOccurrenceIdentity,
    target: &DiagramOccurrenceIdentity,
    kind: DiagramEdgeKind,
    relationships: &[&DiagramRelationship],
) -> DiagramEdge {
    DiagramEdge {
        semantic_id: format!("{}#edge", element.occurrence_id.stable_key()).into(),
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
            .find_map(|relationship| relationship.source_location.clone()),
    }
}

fn resolved_values<T>(outcome: QueryOutcome<Box<[T]>>) -> Box<[T]> {
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => values,
        _ => Box::new([]),
    }
}

fn diagram_element_typing(outcome: QueryOutcome<ElementDetails>) -> DiagramElementTyping {
    match outcome {
        QueryOutcome::Resolved(details)
        | QueryOutcome::Recovered(details)
        | QueryOutcome::UnsupportedWith(details) => {
            let types = details
                .effective_typing
                .types
                .iter()
                .map(|entry| entry.element.identity)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            match details.effective_typing.outcome {
                RelationshipOutcome::NotApplicable => DiagramElementTyping::Absent,
                RelationshipOutcome::Resolved => DiagramElementTyping::Resolved(types),
                RelationshipOutcome::Partial => DiagramElementTyping::Partial(types),
                RelationshipOutcome::Unresolved => DiagramElementTyping::Unresolved,
                RelationshipOutcome::Ambiguous => DiagramElementTyping::Ambiguous(
                    details
                        .effective_typing
                        .candidates
                        .iter()
                        .map(|entry| entry.element.identity)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
                RelationshipOutcome::Unsupported => DiagramElementTyping::Unsupported,
            }
        }
        QueryOutcome::Unresolved => DiagramElementTyping::Unresolved,
        QueryOutcome::Ambiguous(_) => DiagramElementTyping::Ambiguous(Box::default()),
        QueryOutcome::Unsupported => DiagramElementTyping::Unsupported,
        QueryOutcome::Recovery => DiagramElementTyping::Recovery,
        QueryOutcome::Incomplete => DiagramElementTyping::Incomplete,
    }
}

fn usable_value<T>(outcome: QueryOutcome<T>) -> Option<T> {
    match outcome {
        QueryOutcome::Resolved(value)
        | QueryOutcome::Recovered(value)
        | QueryOutcome::UnsupportedWith(value) => Some(value),
        QueryOutcome::Unresolved
        | QueryOutcome::Ambiguous(_)
        | QueryOutcome::Unsupported
        | QueryOutcome::Recovery
        | QueryOutcome::Incomplete => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{compartment_kind, usable_value, DiagramCompartmentKind};
    use crate::{ElementKind, QueryOutcome};

    #[test]
    fn unsupported_outcomes_retain_their_usable_payload() {
        assert_eq!(usable_value(QueryOutcome::UnsupportedWith(42)), Some(42));
        assert_eq!(usable_value::<u8>(QueryOutcome::Unsupported), None);
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
}
