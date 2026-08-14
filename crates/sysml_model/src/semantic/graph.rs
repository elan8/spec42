//! Petgraph-backed semantic graph and query API.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use crate::semantic::text_span::{TextPosition, TextRange};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use petgraph::Directed;
use petgraph::Direction;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

use crate::semantic::model::{
    node_matches_simple_name, ConnectStatementDetail, ConstructionOwner,
    DeclaredExpressionRelationship, DeclaredFeatureValueKind, DeclaredMembershipFacts,
    DerivedRelationshipResolution, EffectiveFeatureOwnership, EffectiveMembershipVisibility,
    EffectiveSemanticFacts, ElementKind, EvaluationPublicationState, ExpressionEvaluationQuery,
    ExpressionResultId, ExpressionResultRole, FeatureOwnershipProvenance, FlowStatementDetail,
    ImpliedFeatureOwnership, ImpliedFeatureValueBinding, ImpliedMultiplicity,
    ImpliedRelationshipRule, MembershipVisibilityProvenance, NodeEvaluationFacts, NodeId,
    RelationshipKind, RelationshipProvenance, SemanticEdge, SemanticNode, VisibilityKind,
};
use crate::semantic::publication::SemanticPublication;

fn serialize_url<S: Serializer>(url: &Url, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(url.as_str())
}

fn deserialize_url<'de, D: Deserializer<'de>>(d: D) -> Result<Url, D::Error> {
    let s = String::deserialize(d)?;
    Url::parse(&s).map_err(serde::de::Error::custom)
}

/// Inserts `id` into `ids` at its canonical `NodeId` order position
/// (`planning/ROUNDTRIP_SEMGRAPH_PREREQS.md` §6: normalized URI, then qualified name).
///
/// Every mutation site that appends to a `node_ids_by_qualified_name` bucket
/// (`add_node_and_recurse`, `merge_inner`, `insert_workspace_node`, `register_short_name_alias`)
/// goes through this function rather than a bare `push`, so the resulting candidate vector's
/// order is a function of canonical `NodeId` order alone -- never of document/merge insertion
/// order, document-set traversal order, or `HashMap` iteration order. This is what makes building
/// the same sources in forward and reverse order produce byte-identical qualified-name lookup
/// vectors (B3), and what makes a first-match consumer of these vectors deterministic rather than
/// accidentally order-dependent.
///
/// Deliberately **not** used for `nodes_by_uri`: that map's per-URI vector order is not a
/// cross-document candidate-precedence list the way `node_ids_by_qualified_name` is -- a single
/// URI's nodes only ever originate from that one document's own deterministic AST-traversal
/// order, so it carries no accidental cross-document ordering dependency, and at least one
/// consumer (`find_deepest_node_at_position`'s span-length tie-break) relies on that stable
/// declaration order. See the matching comment at each `nodes_by_uri` mutation site.
pub(crate) fn insert_canonical(ids: &mut Vec<NodeId>, id: NodeId) {
    let pos = ids.partition_point(|existing| existing < &id);
    ids.insert(pos, id);
}
use crate::semantic::workspace_uri;

/// Cached reverse index from petgraph node index to [`NodeId`] (invalidated on structural mutation).
/// Also indexes edges by URI for O(edges_in_uri) queries instead of O(all_edges).
#[derive(Debug, Clone)]
struct GraphQueryIndexes {
    index_to_node_id: HashMap<NodeIndex, NodeId>,
    /// All edges where the source **or** target node belongs to a given URI.
    edges_by_uri: HashMap<Url, Vec<(NodeId, NodeId, SemanticEdge)>>,
    /// Connection edges indexed by their `declaring_uri` (from `ConnectStatementDetail`).
    connect_edges_by_declaring_uri: HashMap<Url, Vec<(NodeId, NodeId, ConnectStatementDetail)>>,
}

/// Lazily computed workspace-level cache of `has_materialized_shape` per NodeId.
/// Invalidated together with `query_indexes` on structural mutations.
#[derive(Debug, Clone, Default)]
struct ShapeCache {
    by_node_id: HashMap<NodeId, bool>,
}

/// Inner data of the semantic graph. Use [`SemanticGraph`] as the public handle.
#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticGraphData {
    pub graph: StableGraph<SemanticNode, SemanticEdge, Directed>,
    /// Rebuilt after deserialization via [`SemanticGraphData::rebuild_derived_indexes`].
    #[serde(skip)]
    pub node_index_by_id: HashMap<NodeId, NodeIndex>,
    pub nodes_by_uri: HashMap<Url, Vec<NodeId>>,
    pub node_ids_by_qualified_name: HashMap<String, Vec<NodeId>>,
    /// Document identities admitted as canonical library sources for universal standard-library
    /// relationship resolution. A workspace declaration with a matching qualified name is never
    /// a substitute for one of these targets.
    #[serde(default)]
    pub standard_library_uris: HashSet<Url>,
    /// The complete normalized Workspace/StandardLibrary/Library/External classification for
    /// every admitted source URI (the canonical source-ordering contract's "complete normalized
    /// source-origin map"). Reuses `source_identity::SourceRole` rather than defining a second
    /// enum. `standard_library_uris` above remains the fast-path set consulted by universal
    /// standard-library relationship resolution; this map is the superset classification that
    /// also distinguishes `Library` from `Workspace`/`External`, and is the one place source
    /// precedence policy (`Self::source_precedence_rank`) reads role from. Populated by the
    /// pipeline entry points that know each document's `SysmlDocumentSourceKind`
    /// (`semantic::pipeline::source_role_for`), the same call sites that already populate
    /// `standard_library_uris`.
    #[serde(default)]
    pub source_origins: HashMap<Url, source_identity::SourceRole>,
    /// Rebuilt after deserialization via [`SemanticGraphData::rebuild_derived_indexes`].
    #[serde(skip)]
    pub children_by_parent_id: HashMap<NodeId, Vec<NodeId>>,
    pub pending_expression_relationships: Vec<PendingExpressionRelationship>,
    /// Parser-backed authored endpoint expressions captured before any relationship linking.
    /// `owner` is always an explicit parser-materialized semantic scope identity.
    #[serde(default)]
    pub declared_expression_relationships: Vec<DeclaredExpressionRelationshipRecord>,
    pub pending_relationships: Vec<PendingRelationship>,
    /// Build-local typed handoff from an AST membership adapter to its immediately following
    /// node materialization. It is never serialized or published as an attribute projection.
    #[serde(skip)]
    pub(crate) pending_declared_membership_facts: HashMap<NodeId, DeclaredMembershipFacts>,
    /// Build-local typed handoff from an AST identification adapter to its immediately following
    /// node materialization. It is never serialized or published as an attribute projection.
    #[serde(skip)]
    pub(crate) pending_declared_short_names: HashMap<NodeId, String>,
    /// Authoritative effective facts published after semantic linking. Unlike query indexes,
    /// this is model state: consumers use it instead of re-deriving defaults or closure facts.
    #[serde(default)]
    pub effective_facts_by_node_id: HashMap<NodeId, EffectiveSemanticFacts>,
    /// Graph-owned resolution outcomes for universal implied relationships. This is semantic
    /// state, not a host projection cache: absent standard-library prerequisites and ambiguity
    /// remain observable rather than being collapsed into a missing edge.
    #[serde(default)]
    pub derived_relationship_resolution_by_source_id:
        HashMap<NodeId, DerivedRelationshipResolution>,
    /// Authoritative results from the evaluation phase. Interpret absence through
    /// `evaluation_publication`: it is `NotRun` before the barrier and `NotApplicable` after.
    #[serde(default)]
    pub evaluation_facts_by_node_id: HashMap<NodeId, NodeEvaluationFacts>,
    /// Completeness marker for `evaluation_facts_by_node_id`. A map with no entries is only
    /// `NotApplicable` after this barrier is complete; otherwise it is explicitly `NotRun`.
    #[serde(default)]
    pub evaluation_publication: EvaluationPublicationState,
    /// When set, document builders may record authored facts but must not resolve endpoints or
    /// install semantic relationship edges. This is used only by the canonical publication path.
    #[serde(skip)]
    pub(crate) structural_input_only: bool,
    #[serde(skip)]
    pub import_lookup_cache: Mutex<HashMap<(NodeId, String, bool), Vec<NodeId>>>,
    #[serde(skip)]
    query_indexes: Mutex<Option<Arc<GraphQueryIndexes>>>,
    #[serde(skip)]
    shape_cache: Mutex<ShapeCache>,
    /// For each URI, the set of OTHER URIs its own parsed content (import statements +
    /// `::`-qualified references) could plausibly depend on. Computed purely from that URI's
    /// own nodes — see `compute_static_dependency_targets` — and recomputed only when that URI
    /// itself is patched (`update_static_dependency_targets_for_uri`), never as a side effect
    /// of another document's resolution outcome. Deliberately NOT a cache of resolution
    /// results — a reference that temporarily fails to resolve (e.g. its target is renamed
    /// away then back) must not cause this to go stale, since nothing would ever trigger
    /// re-checking it. Rebuilt after deserialization via [`SemanticGraphData::rebuild_derived_indexes`].
    #[serde(skip)]
    pub document_dependency_targets: HashMap<Url, HashSet<Url>>,
    /// Reverse of `document_dependency_targets`: for each URI, the other URIs that statically
    /// depend on it. This is `refresh_relationship_frontier`'s frontier source — every URI
    /// that might need its relationships re-resolved after `changed_uri` is edited. Rebuilt
    /// after deserialization via [`SemanticGraphData::rebuild_derived_indexes`].
    #[serde(skip)]
    pub document_dependents: HashMap<Url, HashSet<Url>>,
    /// The exact (src, tgt, kind) triples of Typing/Specializes/Subject edges currently owned by
    /// [`crate::semantic::model::ConstructionOwner::WorkspaceCrossDocumentLinking`], keyed by
    /// each edge's *source* node's URI. Lets a re-resolve for that URI cleanly remove its own
    /// prior cross-document edges before adding fresh ones, without touching edges owned by
    /// other passes.
    ///
    /// Maintained incrementally by `add_semantic_edge_once` (relationships.rs) as edges are
    /// added during any build path -- whole, parallel, merge-from-base, or scoped/incremental --
    /// so all of them converge on the same content for equivalent graph state. This is a derived
    /// index, not stored truth: it is `#[serde(skip)]` and rebuilt from the graph's own edges
    /// (owner + source identity) via [`SemanticGraphData::rebuild_cross_document_edge_ownership_index`],
    /// called from [`SemanticGraphData::rebuild_derived_indexes`] after deserialization.
    #[serde(skip)]
    pub cross_document_edges_by_source_uri: HashMap<Url, Vec<(NodeId, NodeId, RelationshipKind)>>,
    /// The graph's own publication identity, phase, and completeness (`planning/ROUNDTRIP_SEMGRAPH_PREREQS.md`
    /// B4, `planning/UNIFY_CACHE_PLAN.md` §4.3). Stamped with a real source root and completeness by the
    /// pipeline entry points that own document enumeration
    /// ([`crate::semantic::pipeline::build_and_link_graph`],
    /// [`crate::semantic::pipeline::build_and_link_graph_parallel`]); advanced through
    /// [`SemanticPublication::advance_phase`] at the barriers the pipeline already crosses. See
    /// [`SemanticGraph::is_storage_eligible`] for the one place storage eligibility is decided.
    #[serde(default)]
    pub publication: SemanticPublication,
}

impl Default for SemanticGraphData {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SemanticGraphData {
    fn clone(&self) -> Self {
        Self {
            graph: self.graph.clone(),
            node_index_by_id: self.node_index_by_id.clone(),
            nodes_by_uri: self.nodes_by_uri.clone(),
            node_ids_by_qualified_name: self.node_ids_by_qualified_name.clone(),
            standard_library_uris: self.standard_library_uris.clone(),
            source_origins: self.source_origins.clone(),
            children_by_parent_id: self.children_by_parent_id.clone(),
            pending_expression_relationships: self.pending_expression_relationships.clone(),
            declared_expression_relationships: self.declared_expression_relationships.clone(),
            pending_relationships: self.pending_relationships.clone(),
            // A cloned/published graph must never inherit an unfinished builder handoff.
            pending_declared_membership_facts: HashMap::new(),
            pending_declared_short_names: HashMap::new(),
            effective_facts_by_node_id: self.effective_facts_by_node_id.clone(),
            derived_relationship_resolution_by_source_id: self
                .derived_relationship_resolution_by_source_id
                .clone(),
            evaluation_facts_by_node_id: self.evaluation_facts_by_node_id.clone(),
            evaluation_publication: self.evaluation_publication,
            structural_input_only: self.structural_input_only,
            import_lookup_cache: Mutex::new(HashMap::new()),
            query_indexes: Mutex::new(None),
            shape_cache: Mutex::new(ShapeCache::default()),
            document_dependency_targets: self.document_dependency_targets.clone(),
            document_dependents: self.document_dependents.clone(),
            cross_document_edges_by_source_uri: self.cross_document_edges_by_source_uri.clone(),
            publication: self.publication,
        }
    }
}

/// Cheap-clone handle to a semantic graph. Cloning increments an Arc refcount.
/// Mutation via `DerefMut` triggers copy-on-write (clones inner data only when shared).
#[derive(Debug)]
pub struct SemanticGraph(Arc<SemanticGraphData>);

impl SemanticGraph {
    pub fn new() -> Self {
        SemanticGraph::default()
    }

    /// Wraps already-constructed graph data as a handle, e.g. after directly building or
    /// mutating a [`SemanticGraphData`] (such as [`SemanticGraphData::into_data`]'s inverse).
    pub fn from_data(data: SemanticGraphData) -> Self {
        SemanticGraph(Arc::new(data))
    }

    pub fn into_data(self) -> SemanticGraphData {
        Arc::try_unwrap(self.0).unwrap_or_else(|arc| (*arc).clone())
    }

    /// Returns the effective facts published for `node` at the graph's current semantic barrier.
    pub fn effective_facts_for(&self, node: &SemanticNode) -> Option<&EffectiveSemanticFacts> {
        self.effective_facts_by_node_id.get(&node.id)
    }

    /// Returns the authored membership visibility or the parser-documented contextual default:
    /// public for members of a package, private for nested members. KerML `Import` defaults to
    /// private regardless of owner. `Expose` has a distinct scope/expansion contract, so this
    /// query returns `None` rather than fabricating an import visibility for it. The source fact
    /// remains absent when no prefix was written, so consumers can preserve provenance instead
    /// of mistaking the default for authored source.
    pub fn effective_membership_visibility_for(
        &self,
        node: &SemanticNode,
    ) -> Option<EffectiveMembershipVisibility> {
        let membership = node.declared_facts.membership.as_ref()?;
        if membership
            .import
            .as_ref()
            .is_some_and(|import| import.origin == crate::semantic::model::ImportOrigin::Expose)
        {
            return None;
        }
        Some(match membership.visibility {
            Some(value) => EffectiveMembershipVisibility {
                value,
                provenance: MembershipVisibilityProvenance::Authored,
            },
            None => EffectiveMembershipVisibility {
                value: if membership.import.is_some() {
                    // KerML Import's abstract-syntax default is private; it does not inherit
                    // package-member visibility merely because it is declared in a package.
                    VisibilityKind::Private
                } else {
                    match node
                        .parent_id
                        .as_ref()
                        .and_then(|parent| self.get_node(parent))
                    {
                        None => VisibilityKind::Public,
                        Some(parent) if parent.element_kind == ElementKind::Package => {
                            VisibilityKind::Public
                        }
                        Some(_) => VisibilityKind::Private,
                    }
                },
                provenance: MembershipVisibilityProvenance::Implied,
            },
        })
    }

    /// Returns the graph-published outcome for the universal implied relationship of `node`.
    /// Non-applicable kinds are explicit, rather than being represented as a successful absence.
    pub fn universal_relationship_resolution_for(
        &self,
        node: &SemanticNode,
    ) -> DerivedRelationshipResolution {
        match self
            .derived_relationship_resolution_by_source_id
            .get(&node.id)
        {
            Some(resolution) => resolution.clone(),
            None if node
                .element_kind
                .universal_standard_library_relationship()
                .is_some() =>
            {
                DerivedRelationshipResolution::NotRun
            }
            None => DerivedRelationshipResolution::NotApplicable,
        }
    }

    /// Marks document identities that are trusted library sources for the current graph
    /// publication. This metadata is an input to derived relationship resolution, not a cache.
    pub fn set_standard_library_uris<I>(&mut self, uris: I)
    where
        I: IntoIterator<Item = Url>,
    {
        self.standard_library_uris = uris.into_iter().collect();
        self.derived_relationship_resolution_by_source_id.clear();
    }

    /// Adds canonical library document identities while preserving any already-merged library
    /// graph provenance (for example when linking workspace documents onto a cached library).
    pub fn add_standard_library_uris<I>(&mut self, uris: I)
    where
        I: IntoIterator<Item = Url>,
    {
        self.standard_library_uris.extend(uris);
        self.derived_relationship_resolution_by_source_id.clear();
    }

    /// Records `uri`'s complete [`source_identity::SourceRole`] classification in the graph's
    /// source-origin map (B3's "complete normalized source-origin map"). Idempotent: a later call
    /// for the same URI overwrites its role, which is what re-classifying a source (e.g. a graph
    /// hit whose provider now reports a different role for the same URI) must do.
    pub fn set_source_origin(&mut self, uri: Url, role: source_identity::SourceRole) {
        self.source_origins.insert(uri, role);
    }

    /// Bulk form of [`Self::set_source_origin`] for a whole document set.
    pub fn set_source_origins<I>(&mut self, origins: I)
    where
        I: IntoIterator<Item = (Url, source_identity::SourceRole)>,
    {
        self.source_origins.extend(origins);
    }

    /// The classified [`source_identity::SourceRole`] for `uri`, or `None` if `uri` has not been
    /// admitted through a build entry point that classifies its source (`set_source_origin`/
    /// `set_source_origins`). Never inferred from URI shape or scheme.
    pub fn source_role_for_uri(&self, uri: &Url) -> Option<source_identity::SourceRole> {
        self.source_origins.get(uri).copied()
    }

    /// The complete source-origin map, sorted by normalized URI string (§6's document-order
    /// rule for workspace/external sources). Library-root configured precedence order is not
    /// reconstructible from this map alone -- a caller that needs it supplies its own
    /// `SourceManifest`/root-slot ordering (see `source_identity::SourceManifest`); this accessor
    /// only guarantees the map itself is enumerated deterministically.
    pub fn source_origins_sorted(&self) -> Vec<(Url, source_identity::SourceRole)> {
        let mut origins: Vec<_> = self
            .source_origins
            .iter()
            .map(|(uri, role)| (uri.clone(), *role))
            .collect();
        origins.sort_by(|(left, _), (right, _)| left.as_str().cmp(right.as_str()));
        origins
    }

    /// Recomputes universal standard-library relationships from this coherent graph state.
    /// It first removes this rule's previous edges and statuses, so merge, patch, and library
    /// changes cannot publish a stale resolved target.
    pub fn refresh_universal_standard_library_relationships(&mut self) {
        let stale_edges = self
            .graph
            .edge_references()
            .filter(|edge| {
                matches!(
                    edge.weight().provenance,
                    RelationshipProvenance::Implied(
                        ImpliedRelationshipRule::UniversalStandardLibraryRelationship
                    )
                )
            })
            .map(|edge| edge.id())
            .collect::<Vec<_>>();
        for edge in stale_edges {
            self.graph.remove_edge(edge);
        }
        self.derived_relationship_resolution_by_source_id.clear();

        let nodes = self.node_index_by_id.keys().cloned().collect::<Vec<_>>();
        let mut nodes = nodes;
        nodes.sort_by(|left, right| {
            left.uri
                .as_str()
                .cmp(right.uri.as_str())
                .then_with(|| left.qualified_name.cmp(&right.qualified_name))
        });

        for source_id in nodes {
            let Some(source) = self.get_node(&source_id) else {
                continue;
            };
            let Some(specification) = source
                .element_kind
                .universal_standard_library_relationship()
            else {
                continue;
            };
            let mut candidates = self
                .node_ids_for_qualified_name(specification.target.qualified_name())
                .unwrap_or_default()
                .to_vec();
            candidates.retain(|candidate| self.standard_library_uris.contains(&candidate.uri));
            candidates.sort_by(|left, right| {
                left.uri
                    .as_str()
                    .cmp(right.uri.as_str())
                    .then_with(|| left.qualified_name.cmp(&right.qualified_name))
            });

            let non_self_candidates = candidates
                .iter()
                .filter(|candidate| **candidate != source_id)
                .cloned()
                .collect::<Vec<_>>();
            let resolution = match non_self_candidates.as_slice() {
                [] if candidates.iter().any(|candidate| candidate == &source_id) => {
                    DerivedRelationshipResolution::SelfTargetSuppressed {
                        target: source_id.clone(),
                    }
                }
                [] => DerivedRelationshipResolution::MissingPrerequisite {
                    target: specification.target,
                },
                [target] => {
                    let source_index = self.node_index_by_id.get(&source_id).copied();
                    let target_index = self.node_index_by_id.get(target).copied();
                    let authored_equivalent_exists = source_index.zip(target_index).is_some_and(
                        |(source_index, target_index)| {
                            self.graph
                                .edges_connecting(source_index, target_index)
                                .any(|edge| {
                                    edge.weight().kind == specification.kind
                                        && edge.weight().provenance
                                            == RelationshipProvenance::Authored
                                })
                        },
                    );
                    if !authored_equivalent_exists {
                        self.insert_workspace_edge(
                            &source_id,
                            target,
                            SemanticEdge::implied(
                                specification.kind.clone(),
                                ImpliedRelationshipRule::UniversalStandardLibraryRelationship,
                            ),
                        );
                    }
                    DerivedRelationshipResolution::Resolved {
                        target: target.clone(),
                    }
                }
                _ => DerivedRelationshipResolution::Ambiguous {
                    candidates: non_self_candidates,
                },
            };
            self.derived_relationship_resolution_by_source_id
                .insert(source_id, resolution);
        }
        self.invalidate_query_indexes();
    }

    /// Returns the raw evaluation facts published for `node`. Call
    /// [`Self::expression_evaluation_for`] when the NotRun/NotApplicable distinction matters.
    pub fn evaluation_facts_for(&self, node: &SemanticNode) -> Option<&NodeEvaluationFacts> {
        self.evaluation_facts_by_node_id.get(&node.id)
    }

    pub fn expression_evaluation_for(&self, node: &SemanticNode) -> ExpressionEvaluationQuery<'_> {
        if self.evaluation_publication == EvaluationPublicationState::NotRun {
            return ExpressionEvaluationQuery::NotRun;
        }
        self.evaluation_facts_for(node)
            .and_then(|facts| facts.expression.as_ref())
            .map(ExpressionEvaluationQuery::Result)
            .unwrap_or(ExpressionEvaluationQuery::NotApplicable)
    }

    /// Invalidates the whole atomic evaluation publication after a semantic mutation.
    pub fn invalidate_evaluation_facts(&mut self) {
        self.evaluation_facts_by_node_id.clear();
        self.evaluation_publication = EvaluationPublicationState::NotRun;
        self.retreat_publication_after_structural_mutation();
    }

    /// The single typed predicate for whether this graph may be accepted into persistent cache
    /// storage (the `SemanticPublication` contract, `planning/UNIFY_CACHE_PLAN.md` §4.3).
    ///
    /// Requires both [`SemanticPublication::is_storage_eligible`] (phase == settled/evaluated,
    /// completeness == complete) **and** `evaluation_publication == Complete`. The two are kept
    /// in lockstep by construction -- `publication`'s phase only ever reaches
    /// [`SemanticPhase::SettledEvaluated`] at the same pipeline barrier that sets
    /// `evaluation_publication` to `Complete`, and [`Self::invalidate_evaluation_facts`] retreats
    /// both together -- but this predicate checks both explicitly rather than trusting that
    /// invariant silently, so a future caller that only mutates one of the two cannot
    /// accidentally publish a mismatched pair as storage-eligible.
    pub fn is_storage_eligible(&self) -> bool {
        self.publication.is_storage_eligible()
            && self.evaluation_publication == EvaluationPublicationState::Complete
    }

    /// Returns a feature's ownership after applying its parser-backed modifier or the one
    /// contextual SysML default published at the semantic barrier.
    ///
    /// The source fact stays in [`DeclaredFeatureProperties`](crate::semantic::model::DeclaredFeatureProperties).
    /// Consumers must use this query for ownership semantics rather than treating an absent
    /// authored modifier as a negative result.
    pub fn effective_feature_ownership_for(
        &self,
        node: &SemanticNode,
    ) -> Option<EffectiveFeatureOwnership> {
        let declared = node.declared_facts.feature_properties.as_ref()?;
        if declared.is_composite.is_some() || declared.is_reference.is_some() {
            return Some(EffectiveFeatureOwnership {
                is_composite: declared.is_composite.unwrap_or(false),
                is_reference: declared.is_reference.unwrap_or(false),
                provenance: FeatureOwnershipProvenance::Authored,
            });
        }

        self.effective_facts_for(node)
            .and_then(|facts| facts.implied_feature_ownership)
            .map(|ownership| EffectiveFeatureOwnership {
                is_composite: ownership.is_composite,
                is_reference: ownership.is_reference,
                provenance: FeatureOwnershipProvenance::Implied,
            })
    }

    /// Publish all derived effective facts after relationship linking has settled.
    ///
    /// This is intentionally a graph-level publication rather than a collection of consumer
    /// fallbacks: authored facts remain on their nodes, while normalized and implied facts are
    /// computed once from the complete graph and exposed here with explicit provenance.
    pub fn refresh_effective_facts(&mut self) {
        let mut nodes: Vec<SemanticNode> = self.graph.node_weights().cloned().collect();
        nodes.sort_by(|left, right| {
            left.id
                .uri
                .as_str()
                .cmp(right.id.uri.as_str())
                .then_with(|| left.id.qualified_name.cmp(&right.id.qualified_name))
        });

        let mut effective_facts_by_node_id = HashMap::with_capacity(nodes.len());
        for node in &nodes {
            let implied_multiplicity =
                self.has_implied_exactly_one_multiplicity(node)
                    .then_some(ImpliedMultiplicity {
                        lower: 1,
                        upper: Some(1),
                        is_ordered: false,
                        is_unique: None,
                    });
            let implied_feature_ownership =
                self.has_implied_feature_ownership(node)
                    .then_some(ImpliedFeatureOwnership {
                        is_composite: true,
                        is_reference: false,
                    });
            let featuring_type = self.nearest_featuring_type(node);
            let implied_feature_value_binding = node
                .declared_facts
                .feature_value
                .as_ref()
                .filter(|value| matches!(value.kind, DeclaredFeatureValueKind::Bound))
                .map(|_| ImpliedFeatureValueBinding {
                    expression_result: ExpressionResultId {
                        owner_id: node.id.clone(),
                        role: ExpressionResultRole::FeatureValue,
                    },
                });

            let facts = EffectiveSemanticFacts {
                implied_multiplicity,
                featuring_type,
                implied_feature_value_binding,
                implied_feature_ownership,
            };
            if facts != EffectiveSemanticFacts::default() {
                effective_facts_by_node_id.insert(node.id.clone(), facts);
            }
        }
        self.effective_facts_by_node_id = effective_facts_by_node_id;
    }

    /// Whether a feature receives SysML's implicit `[1..1]` multiplicity.
    ///
    /// This is deliberately narrower than "any node with feature properties". The default
    /// belongs to ordinary, owned part/attribute/port usages only. Package members have
    /// namespace ownership rather than feature ownership, connection usages have their own
    /// semantics, and a resolved subsetting relationship supplies a different multiplicity
    /// context. All of those distinctions are graph facts, so this does not inspect display
    /// text or reconstruct the source declaration.
    fn has_implied_exactly_one_multiplicity(&self, node: &SemanticNode) -> bool {
        if node.declared_facts.multiplicity.is_some()
            || !matches!(
                node.element_kind,
                ElementKind::Part | ElementKind::Attribute | ElementKind::Port
            )
        {
            return false;
        }

        let Some(owner_id) = node.parent_id.as_ref() else {
            return false;
        };
        let Some(owner) = self.get_node(owner_id) else {
            return false;
        };
        if owner.element_kind == ElementKind::Package {
            return false;
        }

        self.outgoing_targets_by_kind(node, RelationshipKind::Subsetting)
            .is_empty()
    }

    /// Whether an ordinary owned usage receives SysML's contextual composite ownership default.
    ///
    /// The rule is deliberately conservative until all feature kinds have a complete typed
    /// ownership contract: it applies only to the parser-backed ordinary usage kinds below when
    /// directly owned by a SysML Type context (definition or usage). Namespace/package members, end features, directed
    /// parameters, and explicit `ref` declarations remain outside the default. Each condition is
    /// read from typed graph facts, never from display attributes or source text.
    fn has_implied_feature_ownership(&self, node: &SemanticNode) -> bool {
        if !node.element_kind.is_composite_by_default_usage() {
            return false;
        }

        let Some(properties) = node.declared_facts.feature_properties.as_ref() else {
            return false;
        };
        if properties.is_composite.is_some()
            || properties.is_reference.is_some()
            || properties.is_end
            || properties.direction.is_some()
        {
            return false;
        }

        node.parent_id
            .as_ref()
            .and_then(|owner_id| self.get_node(owner_id))
            .is_some_and(|owner| owner.element_kind.is_type_context())
    }

    fn nearest_featuring_type(&self, node: &SemanticNode) -> Option<NodeId> {
        let mut current = node.parent_id.clone();
        let mut visited = HashSet::new();
        while let Some(owner_id) = current {
            if !visited.insert(owner_id.clone()) {
                return None;
            }
            let owner = self.get_node(&owner_id)?;
            if owner.element_kind.is_definition() {
                return Some(owner.id.clone());
            }
            let typed_owners: Vec<_> = self
                .outgoing_typing_or_specializes_targets(owner)
                .into_iter()
                .filter(|target| target.element_kind.is_definition())
                .map(|target| target.id.clone())
                .collect();
            if typed_owners.len() == 1 {
                return typed_owners.into_iter().next();
            }
            current = owner.parent_id.clone();
        }
        None
    }
}

impl Default for SemanticGraph {
    fn default() -> Self {
        SemanticGraph(Arc::new(SemanticGraphData::new()))
    }
}

impl Clone for SemanticGraph {
    fn clone(&self) -> Self {
        SemanticGraph(Arc::clone(&self.0))
    }
}

impl Serialize for SemanticGraph {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for SemanticGraph {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        SemanticGraphData::deserialize(d).map(|mut data| {
            data.rebuild_derived_indexes();
            SemanticGraph(Arc::new(data))
        })
    }
}

impl std::ops::Deref for SemanticGraph {
    type Target = SemanticGraphData;
    fn deref(&self) -> &SemanticGraphData {
        &self.0
    }
}

impl std::ops::DerefMut for SemanticGraph {
    fn deref_mut(&mut self) -> &mut SemanticGraphData {
        Arc::make_mut(&mut self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingExpressionRelationship {
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    pub uri: Url,
    pub source_expression: String,
    pub target_expression: String,
    pub kind: RelationshipKind,
    pub container_prefix: Option<String>,
    pub source_range: TextRange,
    #[serde(default)]
    pub is_interface_usage: bool,
    #[serde(default)]
    pub interface_type: Option<String>,
    /// Authored flow metadata retained until endpoint typing makes the edge resolvable.
    #[serde(default)]
    pub flow: Option<FlowStatementDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclaredExpressionRelationshipRecord {
    pub owner: NodeId,
    pub authored_ordinal: u32,
    pub relationship: DeclaredExpressionRelationship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingRelationship {
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    pub uri: Url,
    pub source_qualified: String,
    pub target_qualified: String,
    pub kind: RelationshipKind,
    pub target_kinds: Option<Vec<ElementKind>>,
}

impl SemanticGraphData {
    pub(crate) fn set_structural_input_only(&mut self, value: bool) {
        self.structural_input_only = value;
    }

    /// Returns the parser-materialized root scope for a document. This is deliberately narrower
    /// than a generic URI node lookup: authored document-level expressions must have a real
    /// semantic owner rather than borrowing an arbitrary first node.
    pub(crate) fn root_scope_id(&self, uri: &Url) -> Option<NodeId> {
        let mut roots = self
            .nodes_by_uri
            .get(uri)
            .into_iter()
            .flatten()
            .filter_map(|id| self.get_node(id))
            .filter(|node| node.parent_id.is_none())
            .map(|node| node.id.clone())
            .collect::<Vec<_>>();
        roots.sort();
        roots.into_iter().next()
    }

    /// Rebuild `node_index_by_id`, `children_by_parent_id`, `document_dependency_targets`/
    /// `document_dependents`, and `cross_document_edges_by_source_uri` from the petgraph `graph`
    /// after deserialization (all `#[serde(skip)]`).
    pub fn rebuild_derived_indexes(&mut self) {
        self.node_index_by_id = HashMap::with_capacity(self.graph.node_count());
        self.children_by_parent_id = HashMap::new();
        for idx in self.graph.node_indices() {
            if let Some(node) = self.graph.node_weight(idx) {
                self.node_index_by_id.insert(node.id.clone(), idx);
                if let Some(parent_id) = &node.parent_id {
                    self.children_by_parent_id
                        .entry(parent_id.clone())
                        .or_default()
                        .push(node.id.clone());
                }
            }
        }
        crate::semantic::relationships::rebuild_static_dependency_index(self);
        self.rebuild_cross_document_edge_ownership_index();
    }

    /// Rebuilds `cross_document_edges_by_source_uri` from the graph's own edges — their
    /// `ConstructionOwner` plus their source node's URI — rather than treating the field as
    /// stored truth. This is the sole reconstruction path for a decoded/deserialized graph
    /// (where the field starts empty, see its `#[serde(skip)]`), and it is safe to call at any
    /// time on a live graph too: it always reproduces exactly what `add_semantic_edge_once`
    /// would have accumulated incrementally, because both derive from the same rule (edge kind
    /// in {Typing, Specializes, Subject} and owner
    /// [`crate::semantic::model::ConstructionOwner::WorkspaceCrossDocumentLinking`]).
    pub fn rebuild_cross_document_edge_ownership_index(&mut self) {
        self.cross_document_edges_by_source_uri.clear();
        for edge_ref in self.graph.edge_references() {
            let weight = edge_ref.weight();
            if weight.owner != ConstructionOwner::WorkspaceCrossDocumentLinking {
                continue;
            }
            if !matches!(
                weight.kind,
                RelationshipKind::Typing
                    | RelationshipKind::Specializes
                    | RelationshipKind::Subject
            ) {
                continue;
            }
            let Some(source_node) = self.graph.node_weight(edge_ref.source()) else {
                continue;
            };
            let Some(target_node) = self.graph.node_weight(edge_ref.target()) else {
                continue;
            };
            self.cross_document_edges_by_source_uri
                .entry(source_node.id.uri.clone())
                .or_default()
                .push((
                    source_node.id.clone(),
                    target_node.id.clone(),
                    weight.kind.clone(),
                ));
        }
    }

    /// Test-only simulation of what deserialization produces: resets every `#[serde(skip)]`
    /// derived index this module owns to its default (exactly what `#[serde(skip)]` makes real
    /// deserialization do) and then calls `rebuild_derived_indexes()` — without requiring a
    /// full serde round-trip through a concrete codec. A true round-trip through postcard (the
    /// project's actual cache codec) is blocked today by `SemanticNode.attributes:
    /// HashMap<String, serde_json::Value>` (see B9); `serde_json` round-trips but silently
    /// diverges from the real cache codec's map-key behavior. This directly exercises the same
    /// post-deserialize contract (`rebuild_derived_indexes`) that both real codecs would call,
    /// without depending on either.
    #[cfg(test)]
    pub fn simulate_decode_reset_for_test(&mut self) {
        self.node_index_by_id = HashMap::new();
        self.children_by_parent_id = HashMap::new();
        self.document_dependency_targets = HashMap::new();
        self.document_dependents = HashMap::new();
        self.cross_document_edges_by_source_uri = HashMap::new();
        self.rebuild_derived_indexes();
    }

    /// Removes `uri`'s previously-recorded outgoing cross-document edges (Typing/Specializes/
    /// Subject) from the graph and from `cross_document_edges_by_source_uri`, without touching
    /// in-document edges or edges owned by other passes (derivation connections, case-subject
    /// links resolved outside `resolve_cross_document_edges_for_uri`). Used both by
    /// `remove_nodes_for_uri` (whose node removal already dropped the underlying graph edges —
    /// this just cleans up the now-stale index entry) and by `add_cross_document_edges_for_uri`
    /// (which needs the edges actually removed from the graph before re-adding fresh ones,
    /// since its nodes are *not* being removed).
    ///
    /// Deliberately does NOT touch `document_dependency_targets`/`document_dependents` — those
    /// are static, resolution-independent facts about `uri`'s own content, maintained solely by
    /// `update_static_dependency_targets_for_uri` when `uri` itself is patched. See that
    /// function's doc comment for why conflating the two was the root cause of a real bug.
    pub(crate) fn remove_recorded_cross_document_edges_for_uri(&mut self, uri: &Url) {
        let Some(previous) = self.cross_document_edges_by_source_uri.remove(uri) else {
            return;
        };
        for (src_id, tgt_id, kind) in &previous {
            if let (Some(&src_idx), Some(&tgt_idx)) = (
                self.node_index_by_id.get(src_id),
                self.node_index_by_id.get(tgt_id),
            ) {
                if let Some(edge_idx) = self
                    .graph
                    .edges_connecting(src_idx, tgt_idx)
                    .find(|edge| edge.weight().kind == *kind)
                    .map(|edge| edge.id())
                {
                    self.graph.remove_edge(edge_idx);
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            graph: StableGraph::new(),
            node_index_by_id: HashMap::new(),
            nodes_by_uri: HashMap::new(),
            node_ids_by_qualified_name: HashMap::new(),
            standard_library_uris: HashSet::new(),
            source_origins: HashMap::new(),
            children_by_parent_id: HashMap::new(),
            pending_expression_relationships: Vec::new(),
            declared_expression_relationships: Vec::new(),
            pending_relationships: Vec::new(),
            pending_declared_membership_facts: HashMap::new(),
            pending_declared_short_names: HashMap::new(),
            effective_facts_by_node_id: HashMap::new(),
            derived_relationship_resolution_by_source_id: HashMap::new(),
            evaluation_facts_by_node_id: HashMap::new(),
            evaluation_publication: EvaluationPublicationState::NotRun,
            structural_input_only: false,
            import_lookup_cache: Mutex::new(HashMap::new()),
            query_indexes: Mutex::new(None),
            shape_cache: Mutex::new(ShapeCache::default()),
            document_dependency_targets: HashMap::new(),
            document_dependents: HashMap::new(),
            cross_document_edges_by_source_uri: HashMap::new(),
            publication: SemanticPublication::default(),
        }
    }

    fn build_query_indexes(&self) -> GraphQueryIndexes {
        let mut index_to_node_id = HashMap::with_capacity(self.node_index_by_id.len());
        for (id, idx) in &self.node_index_by_id {
            index_to_node_id.insert(*idx, id.clone());
        }

        // Build URI edge indexes in a single pass over all edges.
        let mut edges_by_uri: HashMap<Url, Vec<(NodeId, NodeId, SemanticEdge)>> = HashMap::new();
        let mut connect_edges_by_declaring_uri: HashMap<
            Url,
            Vec<(NodeId, NodeId, ConnectStatementDetail)>,
        > = HashMap::new();

        for e in self.graph.edge_references() {
            let Some(src_id) = index_to_node_id.get(&e.source()) else {
                continue;
            };
            let Some(tgt_id) = index_to_node_id.get(&e.target()) else {
                continue;
            };
            let weight = e.weight();

            // Index by source URI; also by target URI when it differs.
            edges_by_uri.entry(src_id.uri.clone()).or_default().push((
                src_id.clone(),
                tgt_id.clone(),
                weight.clone(),
            ));
            if tgt_id.uri != src_id.uri {
                edges_by_uri.entry(tgt_id.uri.clone()).or_default().push((
                    src_id.clone(),
                    tgt_id.clone(),
                    weight.clone(),
                ));
            }

            // Index connect-statement edges by their declaring URI.
            if weight.kind == RelationshipKind::Connection {
                if let Some(connect) = &weight.connect {
                    connect_edges_by_declaring_uri
                        .entry(connect.declaring_uri.clone())
                        .or_default()
                        .push((src_id.clone(), tgt_id.clone(), connect.clone()));
                }
            }
        }

        GraphQueryIndexes {
            index_to_node_id,
            edges_by_uri,
            connect_edges_by_declaring_uri,
        }
    }

    fn query_indexes(&self) -> Arc<GraphQueryIndexes> {
        // Recover from poison instead of panicking the whole analysis pipeline;
        // a poisoned cache is rebuilt below just like an empty one.
        let mut guard = self
            .query_indexes
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(indexes) = guard.as_ref() {
            return Arc::clone(indexes);
        }
        let built = Arc::new(self.build_query_indexes());
        *guard = Some(Arc::clone(&built));
        built
    }

    pub fn invalidate_query_indexes(&self) {
        if let Ok(mut guard) = self.query_indexes.lock() {
            *guard = None;
        }
        if let Ok(mut cache) = self.shape_cache.lock() {
            cache.by_node_id.clear();
        }
    }

    /// Returns the cached `has_materialized_shape` result for the node, if available.
    pub(crate) fn get_cached_shape(&self, node: &SemanticNode) -> Option<bool> {
        self.shape_cache
            .lock()
            .ok()
            .and_then(|cache| cache.by_node_id.get(&node.id).copied())
    }

    /// Stores a `has_materialized_shape` result in the workspace-level cache.
    pub(crate) fn set_cached_shape(&self, node_id: &NodeId, value: bool) {
        if let Ok(mut cache) = self.shape_cache.lock() {
            cache.by_node_id.insert(node_id.clone(), value);
        }
    }

    /// The short-name-qualified alias for `node` (see
    /// [`crate::semantic::model::DeclaredSemanticFacts::short_name`]) — the same qualified name
    /// a sibling declared under the short name directly would get. `None` if `node` has no short
    /// name.
    pub(crate) fn short_name_alias_qualified(node: &SemanticNode) -> Option<String> {
        let short_name = node.declared_facts.short_name.as_deref()?;
        let container_prefix = node
            .parent_id
            .as_ref()
            .map(|parent_id| parent_id.qualified_name.as_str());
        Some(crate::semantic::graph_builder::qualified_name(
            container_prefix,
            short_name,
        ))
    }

    /// Registers `node`'s short-name-qualified alias in `node_ids_by_qualified_name`,
    /// pointing at `id`, so qualified-name lookups (typing, specializes, ...) resolve short
    /// names the same way as the node's own declared name. No-op if `node` has no short name.
    /// Called from every place a node is inserted into a graph (`add_node_and_recurse`,
    /// `merge_inner`, `insert_workspace_node`) — see `remove_nodes_for_uri`'s matching cleanup.
    pub(crate) fn register_short_name_alias(&mut self, id: &NodeId, node: &SemanticNode) {
        let Some(short_qualified) = Self::short_name_alias_qualified(node) else {
            return;
        };
        if short_qualified != id.qualified_name {
            insert_canonical(
                self.node_ids_by_qualified_name
                    .entry(short_qualified)
                    .or_default(),
                id.clone(),
            );
        }
    }

    /// Removes `id` from its short-name-qualified alias entry (the reverse of
    /// `register_short_name_alias`), so a removed node's alias doesn't dangle. No-op if `node`
    /// has no short name.
    fn deregister_short_name_alias(&mut self, id: &NodeId, node: &SemanticNode) {
        let Some(short_qualified) = Self::short_name_alias_qualified(node) else {
            return;
        };
        let mut remove_entry = false;
        if let Some(ids) = self.node_ids_by_qualified_name.get_mut(&short_qualified) {
            ids.retain(|existing| existing != id);
            remove_entry = ids.is_empty();
        }
        if remove_entry {
            self.node_ids_by_qualified_name.remove(&short_qualified);
        }
    }

    /// Retreats `publication`'s phase to [`SemanticPhase::Parsed`] (never advances it) after a
    /// structural mutation that invalidated settled/evaluated state, so a stale
    /// `SettledEvaluated` publication can never survive a mutation that did not re-cross that
    /// barrier.
    ///
    /// [`SemanticPublication::advance_phase`] is deliberately forward-only, so this is the one
    /// place that moves `phase` backward. Retreating all the way to `Parsed` (not merely to
    /// `StructurallyLinked`) matters: the mutation's own caller has not necessarily relinked yet
    /// either (e.g. `patch_graph_for_document(..., evaluate: false)` merges a document's nodes
    /// and returns without relinking), so claiming `StructurallyLinked` here would be exactly the
    /// kind of unearned phase this type exists to prevent. Whichever pipeline function performs
    /// real relinking/evaluation afterward advances the phase again through its own barriers.
    ///
    /// Deliberately leaves `completeness` and `root_digest` untouched: a document patch (a
    /// remove followed by a re-merge) does not by itself make the graph cover less than its
    /// admitted source set, nor does it change which sources are admitted; see
    /// [`SemanticCompleteness::Partial`]'s doc comment. A caller with genuine new information
    /// about parse quality (e.g. a whole-document-set pipeline entry point that inspects real
    /// parse diagnostics) sets completeness explicitly via [`SemanticPublication::set_identity`];
    /// this retreat must not silently invent a downgrade the caller never observed.
    ///
    /// Called from every structural mutation point ([`Self::remove_nodes_for_uri`],
    /// [`Self::merge_inner`], [`SemanticGraph::invalidate_evaluation_facts`]) so a caller can
    /// never observe a settled/evaluated, storage-eligible publication for content that was never
    /// relinked or re-evaluated.
    fn retreat_publication_after_structural_mutation(&mut self) {
        self.publication = SemanticPublication::new(
            self.publication.root_digest(),
            self.publication.completeness(),
        );
    }

    /// Removes all nodes (and their incident edges) for the given URI.
    pub fn remove_nodes_for_uri(&mut self, uri: &Url) {
        // A URI alone is never durable evidence of standard-library provenance. Once its
        // canonical document is removed, any replacement must be admitted again through the
        // source-kind-aware build boundary before it can satisfy a universal relationship.
        self.standard_library_uris.remove(uri);
        self.declared_expression_relationships
            .retain(|record| &record.owner.uri != uri);
        // Likewise for the complete source-origin classification (B3): a removed document's
        // prior Workspace/StandardLibrary/Library/External role must not survive as a stale
        // entry once none of its nodes remain -- otherwise a document-delete-then-rebuild
        // comparison diverges on `source_origins` alone despite an identical node/edge graph
        // (found by the B11 differential post-edit suite's `delete_a_document` case).
        self.source_origins.remove(uri);
        let Some(node_ids) = self.nodes_by_uri.remove(uri) else {
            self.clear_import_lookup_cache();
            return;
        };
        // Clone each node's current weight before removal — needed both for the parent's
        // children-index update and to deregister any short-name alias (both read fields off
        // the node itself, which won't be reachable once `node_index_by_id`/the graph node are
        // removed below).
        let removals: Vec<(NodeId, SemanticNode)> = node_ids
            .iter()
            .filter_map(|id| {
                let node = self
                    .node_index_by_id
                    .get(id)
                    .and_then(|&idx| self.graph.node_weight(idx))?
                    .clone();
                Some((id.clone(), node))
            })
            .collect();

        for (id, node) in &removals {
            let mut remove_lookup_entry = false;
            if let Some(ids) = self.node_ids_by_qualified_name.get_mut(&id.qualified_name) {
                ids.retain(|existing| existing != id);
                remove_lookup_entry = ids.is_empty();
            }
            if remove_lookup_entry {
                self.node_ids_by_qualified_name.remove(&id.qualified_name);
            }
            self.deregister_short_name_alias(id, node);
            if let Some(idx) = self.node_index_by_id.remove(id) {
                self.graph.remove_node(idx);
            }
            self.children_by_parent_id.remove(id);
        }
        // Remove each node from its parent's children list.
        for (id, node) in removals {
            if let Some(pid) = node.parent_id {
                if let Some(children) = self.children_by_parent_id.get_mut(&pid) {
                    children.retain(|c| c != &id);
                }
            }
        }
        self.effective_facts_by_node_id.clear();
        self.derived_relationship_resolution_by_source_id.clear();
        self.evaluation_facts_by_node_id.clear();
        self.evaluation_publication = EvaluationPublicationState::NotRun;
        self.retreat_publication_after_structural_mutation();
        self.remove_recorded_cross_document_edges_for_uri(uri);
        self.invalidate_query_indexes();
        self.clear_import_lookup_cache();
    }

    /// Merges nodes and edges from another graph (built from a single document).
    pub fn merge(&mut self, other: SemanticGraph) {
        self.merge_inner(other.into_data(), None);
    }

    /// Merges another graph but skips nodes already declared in the workspace.
    ///
    /// Skips a library node when it belongs to a most-specific package declared in
    /// `shadowed_packages` (workspace wins), or when a non-package element with the same
    /// qualified name already exists. Duplicate ancestor package nodes are retained so a
    /// namespace can be assembled from workspace and library contributions.
    pub fn merge_skip_existing_qualified_names(
        &mut self,
        other: SemanticGraph,
        shadowed_packages: &std::collections::HashSet<String>,
    ) {
        self.merge_inner(other.into_data(), Some(shadowed_packages));
    }

    fn merge_inner(
        &mut self,
        other: SemanticGraphData,
        shadowed_packages: Option<&std::collections::HashSet<String>>,
    ) {
        self.effective_facts_by_node_id.clear();
        self.derived_relationship_resolution_by_source_id.clear();
        self.evaluation_facts_by_node_id.clear();
        self.evaluation_publication = EvaluationPublicationState::NotRun;
        self.retreat_publication_after_structural_mutation();
        self.pending_relationships
            .extend(other.pending_relationships.iter().cloned());
        self.pending_expression_relationships
            .extend(other.pending_expression_relationships.iter().cloned());
        self.declared_expression_relationships.extend(
            other
                .declared_expression_relationships
                .iter()
                .filter(|record| {
                    shadowed_packages.is_none_or(|packages| {
                        !Self::qualified_name_under_packages(&record.owner.qualified_name, packages)
                    })
                })
                .cloned(),
        );
        for (id, node) in other.iter_nodes() {
            if let Some(shadowed) = shadowed_packages {
                let exact_name_exists = self
                    .node_ids_by_qualified_name
                    .contains_key(&id.qualified_name);
                let is_package = matches!(node.element_kind, ElementKind::Package);
                let is_canonical_library_node = self.standard_library_uris.contains(&id.uri);
                if Self::qualified_name_under_packages(&id.qualified_name, shadowed)
                    || (exact_name_exists && !is_package && !is_canonical_library_node)
                {
                    continue;
                }
            }
            let idx = self.graph.add_node(node.clone());
            self.node_index_by_id.insert(id.clone(), idx);
            // `nodes_by_uri`'s per-URI vector deliberately stays insertion-ordered, not
            // canonicalized: a single URI's nodes only ever originate from that one document's
            // own deterministic AST-traversal order (each `merge_inner` call's `other` is a
            // single-document graph, so `other.iter_nodes()` -- itself keyed by URI -- yields
            // that document's nodes in a fixed order regardless of build/merge order across
            // *other* documents). Position-sensitive consumers such as
            // `find_deepest_node_at_position` rely on that stable declaration order as a
            // deterministic tie-break for overlapping same-span ranges; canonicalizing this
            // vector by qualified name would replace one deterministic, source-order-derived
            // tie-break with a different (alphabetical) one for no B3 benefit, since this vector
            // is not a cross-document candidate-precedence list the way
            // `node_ids_by_qualified_name` is.
            self.nodes_by_uri
                .entry(id.uri.clone())
                .or_default()
                .push(id.clone());
            insert_canonical(
                self.node_ids_by_qualified_name
                    .entry(id.qualified_name.clone())
                    .or_default(),
                id.clone(),
            );
            // Re-derive the short-name-qualified alias too — merging rebuilds
            // `node_ids_by_qualified_name` from each node's own canonical qualified name only,
            // so the alias registered when the node was first built would otherwise be
            // silently dropped here.
            self.register_short_name_alias(&id, node);
            if let Some(parent_id) = &node.parent_id {
                self.children_by_parent_id
                    .entry(parent_id.clone())
                    .or_default()
                    .push(id);
            }
        }
        for (src_id, tgt_id, edge) in other.iter_edges() {
            if let (Some(&src_idx), Some(&tgt_idx)) = (
                self.node_index_by_id.get(&src_id),
                self.node_index_by_id.get(&tgt_id),
            ) {
                self.graph.add_edge(src_idx, tgt_idx, edge.clone());
            }
        }
        self.invalidate_query_indexes();
    }

    fn qualified_name_under_packages(
        qualified_name: &str,
        packages: &std::collections::HashSet<String>,
    ) -> bool {
        packages.iter().any(|pkg| {
            qualified_name == pkg.as_str() || qualified_name.starts_with(&format!("{pkg}::"))
        })
    }

    pub(crate) fn clear_import_lookup_cache(&self) {
        if let Ok(mut cache) = self.import_lookup_cache.lock() {
            cache.clear();
        }
    }

    fn iter_nodes(&self) -> impl Iterator<Item = (NodeId, &SemanticNode)> {
        self.nodes_by_uri.values().flatten().filter_map(|id| {
            self.node_index_by_id
                .get(id)
                .and_then(|&idx| self.graph.node_weight(idx))
                .map(|n| (id.clone(), n))
        })
    }

    /// Returns a stable snapshot of all nodes for crate-private semantic phases.
    ///
    /// Published consumers must use `SemanticModel`/`ResolutionView`; this is intentionally
    /// crate-private so the resolver can construct its immutable publication without exposing
    /// the mutable graph representation as another semantic authority.
    pub(crate) fn semantic_nodes(&self) -> Vec<SemanticNode> {
        self.iter_nodes().map(|(_, node)| node.clone()).collect()
    }

    /// Internal publication support for deterministic model diagnostics.
    pub(crate) fn semantic_node_refs(&self) -> Vec<&SemanticNode> {
        self.iter_nodes().map(|(_, node)| node).collect()
    }

    /// Returns a stable snapshot of all graph edges for crate-private semantic phases.
    #[cfg(test)]
    pub(crate) fn semantic_edges(&self) -> Vec<(NodeId, NodeId, SemanticEdge)> {
        self.iter_edges().collect()
    }

    pub fn node_ids_for_qualified_name(&self, qualified_name: &str) -> Option<&[NodeId]> {
        self.node_ids_by_qualified_name
            .get(qualified_name)
            .map(Vec::as_slice)
    }

    fn iter_edges(&self) -> impl Iterator<Item = (NodeId, NodeId, SemanticEdge)> + '_ {
        let indexes = self.query_indexes();
        self.graph.edge_references().filter_map(move |e| {
            let src_id = indexes.index_to_node_id.get(&e.source())?.clone();
            let tgt_id = indexes.index_to_node_id.get(&e.target())?.clone();
            let edge = e.weight().clone();
            Some((src_id, tgt_id, edge))
        })
    }

    /// Returns URIs that have nodes in the graph (for debugging).
    pub fn uris_with_nodes(&self) -> Vec<String> {
        self.nodes_by_uri
            .keys()
            .take(5)
            .map(|u| u.as_str().to_string())
            .collect()
    }

    /// Returns all URIs that have nodes in the graph.
    pub fn all_uris(&self) -> Vec<Url> {
        self.nodes_by_uri.keys().cloned().collect()
    }

    /// Returns all nodes that belong to the given URI (document).
    pub fn nodes_for_uri(&self, uri: &Url) -> Vec<&SemanticNode> {
        let Some(ids) = self.nodes_by_uri.get(uri) else {
            return Vec::new();
        };
        ids.iter()
            .filter_map(|id| {
                self.node_index_by_id
                    .get(id)
                    .and_then(|&idx| self.graph.node_weight(idx))
            })
            .collect()
    }

    /// Returns all nodes in the merged graph whose simple name matches `name`.
    pub fn nodes_named(&self, name: &str) -> Vec<&SemanticNode> {
        self.nodes_by_uri
            .values()
            .flatten()
            .filter_map(|id| {
                self.node_index_by_id
                    .get(id)
                    .and_then(|&idx| self.graph.node_weight(idx))
            })
            .filter(|node| node_matches_simple_name(node, name))
            .collect()
    }

    /// Returns child nodes of the given node using the parent→children index (O(1) lookup).
    pub fn children_of(&self, parent: &SemanticNode) -> Vec<&SemanticNode> {
        self.children_by_parent_id
            .get(&parent.id)
            .into_iter()
            .flatten()
            .filter_map(|id| self.get_node(id))
            .collect()
    }

    /// Returns directly owned end features in authored source order.
    ///
    /// An end is a declared structural fact, not a convention based on its name or its type.
    /// This is the canonical positional view for connection-like definitions/usages; it includes
    /// only children whose parser-backed feature properties declare `is_end`, and never invents
    /// a missing counterpart for an incomplete declaration.
    pub fn positional_end_features(&self, owner: &SemanticNode) -> Vec<&SemanticNode> {
        let mut ends: Vec<_> = self
            .children_of(owner)
            .into_iter()
            .filter(|child| {
                child
                    .declared_facts
                    .feature_properties
                    .as_ref()
                    .is_some_and(|properties| properties.is_end)
            })
            .collect();
        ends.sort_by_key(|child| {
            (
                child.range.start.line,
                child.range.start.character,
                child.range.end.line,
                child.range.end.character,
                child.id.qualified_name.as_str(),
            )
        });
        ends
    }

    /// Returns the node for the given NodeId, if it exists.
    pub fn get_node(&self, id: &NodeId) -> Option<&SemanticNode> {
        self.node_index_by_id
            .get(id)
            .and_then(|&idx| self.graph.node_weight(idx))
    }

    /// Returns a mutable reference to the node for the given NodeId, if it exists.
    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut SemanticNode> {
        let idx = *self.node_index_by_id.get(id)?;
        self.graph.node_weight_mut(idx)
    }

    /// Registers a parser-backed membership fact for the node identity about to be materialized.
    /// The graph builder consumes it during node insertion; duplicate registration is a builder
    /// bug because one declaration has exactly one membership fact.
    pub(crate) fn register_declared_membership_facts(
        &mut self,
        id: NodeId,
        facts: DeclaredMembershipFacts,
    ) {
        assert!(
            self.pending_declared_membership_facts
                .insert(id, facts)
                .is_none(),
            "membership facts registered twice for one node"
        );
    }

    pub(crate) fn take_declared_membership_facts(
        &mut self,
        id: &NodeId,
    ) -> Option<DeclaredMembershipFacts> {
        self.pending_declared_membership_facts.remove(id)
    }

    pub(crate) fn assert_no_pending_declared_membership_facts(&self) {
        assert!(
            self.pending_declared_membership_facts.is_empty(),
            "all parser-authored membership facts must be consumed before graph publication"
        );
    }

    /// Registers a parser-backed short name for the node identity about to be materialized.
    /// The graph builder consumes it during node insertion; duplicate registration is a builder
    /// bug because one declaration has exactly one short name.
    pub(crate) fn register_declared_short_name(&mut self, id: NodeId, short_name: String) {
        assert!(
            self.pending_declared_short_names
                .insert(id, short_name)
                .is_none(),
            "short name registered twice for one node"
        );
    }

    pub(crate) fn take_declared_short_name(&mut self, id: &NodeId) -> Option<String> {
        self.pending_declared_short_names.remove(id)
    }

    pub(crate) fn assert_no_pending_declared_short_names(&self) {
        assert!(
            self.pending_declared_short_names.is_empty(),
            "all parser-authored short names must be consumed before graph publication"
        );
    }

    /// Returns the node whose range contains the given position (first match).
    pub fn find_node_at_position(&self, uri: &Url, pos: TextPosition) -> Option<&SemanticNode> {
        self.nodes_for_uri(uri).into_iter().find(|n| {
            let r = &n.range;
            (pos.line > r.start.line
                || (pos.line == r.start.line && pos.character >= r.start.character))
                && (pos.line < r.end.line
                    || (pos.line == r.end.line && pos.character <= r.end.character))
        })
    }

    /// Returns the smallest-range node whose range contains the given position.
    pub fn find_deepest_node_at_position(
        &self,
        uri: &Url,
        pos: TextPosition,
    ) -> Option<&SemanticNode> {
        self.nodes_for_uri(uri)
            .into_iter()
            .filter(|n| {
                let r = &n.range;
                (pos.line > r.start.line
                    || (pos.line == r.start.line && pos.character >= r.start.character))
                    && (pos.line < r.end.line
                        || (pos.line == r.end.line && pos.character <= r.end.character))
            })
            .min_by_key(|n| {
                let line_span = n.range.end.line.saturating_sub(n.range.start.line);
                let char_span = n
                    .range
                    .end
                    .character
                    .saturating_sub(n.range.start.character);
                line_span.saturating_mul(10000).saturating_add(char_span)
            })
    }

    /// Returns the direct parent node if present.
    pub fn parent_of(&self, node: &SemanticNode) -> Option<&SemanticNode> {
        node.parent_id
            .as_ref()
            .and_then(|parent_id| self.get_node(parent_id))
    }

    /// Returns all ancestors from nearest parent to root.
    pub fn ancestors_of(&self, node: &SemanticNode) -> Vec<&SemanticNode> {
        let mut out = Vec::new();
        let mut current = self.parent_of(node);
        while let Some(parent) = current {
            out.push(parent);
            current = self.parent_of(parent);
        }
        out
    }

    /// Returns direct children by exact name under the given parent.
    pub fn child_named(&self, parent_id: &NodeId, name: &str) -> Vec<&SemanticNode> {
        let Some(parent) = self.get_node(parent_id) else {
            return Vec::new();
        };
        self.children_of(parent)
            .into_iter()
            .filter(|child| node_matches_simple_name(child, name))
            .collect()
    }

    /// Returns target nodes of typing or specializes edges from the given node.
    pub fn outgoing_typing_or_specializes_targets(
        &self,
        node: &SemanticNode,
    ) -> Vec<&SemanticNode> {
        let src_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut targets = Vec::new();
        for edge in self.graph.edges_directed(src_idx, Direction::Outgoing) {
            if matches!(
                edge.weight().kind,
                RelationshipKind::Typing | RelationshipKind::Specializes
            ) {
                if let Some(tgt_id) = id_by_idx.get(&edge.target()) {
                    if let Some(tgt) = self.get_node(tgt_id) {
                        targets.push(tgt);
                    }
                }
            }
        }
        targets
    }

    /// Returns target nodes of outgoing edges with the given relationship kind.
    pub fn outgoing_targets_by_kind(
        &self,
        node: &SemanticNode,
        kind: RelationshipKind,
    ) -> Vec<&SemanticNode> {
        let src_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut targets = Vec::new();
        for edge in self.graph.edges_directed(src_idx, Direction::Outgoing) {
            if edge.weight().kind == kind {
                if let Some(tgt_id) = id_by_idx.get(&edge.target()) {
                    if let Some(tgt) = self.get_node(tgt_id) {
                        targets.push(tgt);
                    }
                }
            }
        }
        targets
    }

    /// Returns outgoing targets with both relationship kind and provenance selected. This is the
    /// provenance-aware companion to [`Self::outgoing_targets_by_kind`]; the latter intentionally
    /// returns authored and implied facts from the one canonical edge store.
    pub fn outgoing_targets_by_kind_and_provenance(
        &self,
        node: &SemanticNode,
        kind: RelationshipKind,
        provenance: RelationshipProvenance,
    ) -> Vec<&SemanticNode> {
        let Some(&source_index) = self.node_index_by_id.get(&node.id) else {
            return Vec::new();
        };
        let indexes = self.query_indexes();
        self.graph
            .edges_directed(source_index, Direction::Outgoing)
            .filter(|edge| edge.weight().kind == kind && edge.weight().provenance == provenance)
            .filter_map(|edge| indexes.index_to_node_id.get(&edge.target()))
            .filter_map(|target_id| self.get_node(target_id))
            .collect()
    }

    /// Returns whether `specific` is identical to, or specializes, `general`.
    ///
    /// This follows only resolved `Specializes` relationships.  Callers that
    /// need type conformance must use this graph fact rather than comparing
    /// declared type text.
    pub fn specializes_transitively(
        &self,
        specific: &SemanticNode,
        general: &SemanticNode,
    ) -> bool {
        if specific.id == general.id {
            return true;
        }

        let mut seen = HashSet::new();
        let mut pending: VecDeque<NodeId> = self
            .outgoing_targets_by_kind(specific, RelationshipKind::Specializes)
            .into_iter()
            .map(|node| node.id.clone())
            .collect();

        while let Some(current_id) = pending.pop_front() {
            if !seen.insert(current_id.clone()) {
                continue;
            }
            if current_id == general.id {
                return true;
            }
            let Some(current) = self.get_node(&current_id) else {
                continue;
            };
            pending.extend(
                self.outgoing_targets_by_kind(current, RelationshipKind::Specializes)
                    .into_iter()
                    .map(|node| node.id.clone()),
            );
        }

        false
    }

    /// Returns whether every resolved type of `specific_feature` conforms to
    /// the corresponding resolved type of `general_feature`.
    ///
    /// An untyped feature inherits the other feature's typing, so it does not
    /// violate this check.  For each type of the general feature, at least one
    /// type of the specific feature must be identical to it or specialize it.
    pub fn feature_typing_conforms(
        &self,
        specific_feature: &SemanticNode,
        general_feature: &SemanticNode,
    ) -> bool {
        let specific_types =
            self.outgoing_targets_by_kind(specific_feature, RelationshipKind::Typing);
        let general_types =
            self.outgoing_targets_by_kind(general_feature, RelationshipKind::Typing);

        if specific_types.is_empty() || general_types.is_empty() {
            return true;
        }

        general_types.iter().all(|general_type| {
            specific_types
                .iter()
                .any(|specific_type| self.specializes_transitively(specific_type, general_type))
        })
    }

    /// Returns source nodes that have typing/specializes edges to the given node.
    pub fn incoming_typing_or_specializes_sources(
        &self,
        node: &SemanticNode,
    ) -> Vec<&SemanticNode> {
        let tgt_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut sources = Vec::new();
        for edge in self.graph.edges_directed(tgt_idx, Direction::Incoming) {
            if matches!(
                edge.weight().kind,
                RelationshipKind::Typing | RelationshipKind::Specializes
            ) {
                if let Some(src_id) = id_by_idx.get(&edge.source()) {
                    if let Some(src) = self.get_node(src_id) {
                        sources.push(src);
                    }
                }
            }
        }
        sources
    }

    /// Returns source nodes of incoming edges with the given relationship kind.
    pub fn incoming_sources_by_kind(
        &self,
        node: &SemanticNode,
        kind: RelationshipKind,
    ) -> Vec<&SemanticNode> {
        let tgt_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut sources = Vec::new();
        for edge in self.graph.edges_directed(tgt_idx, Direction::Incoming) {
            if edge.weight().kind == kind {
                if let Some(src_id) = id_by_idx.get(&edge.source()) {
                    if let Some(src) = self.get_node(src_id) {
                        sources.push(src);
                    }
                }
            }
        }
        sources
    }

    /// Returns all direct outgoing relationships from the given node.
    pub fn outgoing_relationships(
        &self,
        node: &SemanticNode,
    ) -> Vec<(&SemanticNode, RelationshipKind)> {
        let src_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut relationships = Vec::new();
        for edge in self.graph.edges_directed(src_idx, Direction::Outgoing) {
            if let Some(tgt_id) = id_by_idx.get(&edge.target()) {
                if let Some(tgt) = self.get_node(tgt_id) {
                    relationships.push((tgt, edge.weight().kind.clone()));
                }
            }
        }
        relationships
    }

    /// Returns all direct incoming relationships into the given node.
    pub fn incoming_relationships(
        &self,
        node: &SemanticNode,
    ) -> Vec<(&SemanticNode, RelationshipKind)> {
        let tgt_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut relationships = Vec::new();
        for edge in self.graph.edges_directed(tgt_idx, Direction::Incoming) {
            if let Some(src_id) = id_by_idx.get(&edge.source()) {
                if let Some(src) = self.get_node(src_id) {
                    relationships.push((src, edge.weight().kind.clone()));
                }
            }
        }
        relationships
    }

    /// Returns target nodes of perform edges from the given node.
    pub fn outgoing_perform_targets(&self, node: &SemanticNode) -> Vec<&SemanticNode> {
        let src_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut targets = Vec::new();
        for edge in self.graph.edges_directed(src_idx, Direction::Outgoing) {
            if edge.weight().kind == RelationshipKind::Perform {
                if let Some(tgt_id) = id_by_idx.get(&edge.target()) {
                    if let Some(tgt) = self.get_node(tgt_id) {
                        targets.push(tgt);
                    }
                }
            }
        }
        targets
    }

    /// Returns source nodes of perform edges into the given node.
    pub fn incoming_perform_sources(&self, node: &SemanticNode) -> Vec<&SemanticNode> {
        let tgt_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut sources = Vec::new();
        for edge in self.graph.edges_directed(tgt_idx, Direction::Incoming) {
            if edge.weight().kind == RelationshipKind::Perform {
                if let Some(src_id) = id_by_idx.get(&edge.source()) {
                    if let Some(src) = self.get_node(src_id) {
                        sources.push(src);
                    }
                }
            }
        }
        sources
    }

    /// Returns connection edges that touch the given URI, as (source NodeId, target NodeId).
    /// Used for semantic checks (port type compatibility, endpoint kind).
    pub fn connection_edge_node_pairs_for_uri(&self, uri: &Url) -> Vec<(NodeId, NodeId)> {
        let indexes = self.query_indexes();
        indexes
            .edges_by_uri
            .get(uri)
            .into_iter()
            .flatten()
            .filter(|(_, _, e)| e.kind == RelationshipKind::Connection)
            .map(|(src, tgt, _)| (src.clone(), tgt.clone()))
            .collect()
    }

    /// Returns all `Connection` edges incident to nodes in the given URI.
    pub fn connection_edges_touching_uri(&self, uri: &Url) -> Vec<(NodeId, NodeId, SemanticEdge)> {
        let indexes = self.query_indexes();
        indexes
            .edges_by_uri
            .get(uri)
            .into_iter()
            .flatten()
            .filter(|(_, _, e)| e.kind == RelationshipKind::Connection)
            .cloned()
            .collect()
    }

    /// Returns structural interconnection edges (`connect` and `bind`) incident to nodes in the
    /// given URI. Kept separate from `connection_edges_touching_uri` because callers performing
    /// connection type compatibility checks must not treat bindings as connections.
    pub fn interconnection_edges_touching_uri(
        &self,
        uri: &Url,
    ) -> Vec<(NodeId, NodeId, SemanticEdge)> {
        let indexes = self.query_indexes();
        indexes
            .edges_by_uri
            .get(uri)
            .into_iter()
            .flatten()
            .filter(|(_, _, edge)| {
                matches!(
                    edge.kind,
                    RelationshipKind::Connection | RelationshipKind::Bind
                )
            })
            .cloned()
            .collect()
    }

    /// Returns `Connection` edges declared in the given URI with `connect` metadata.
    pub fn connect_statement_edges_for_uri(
        &self,
        uri: &Url,
    ) -> Vec<(NodeId, NodeId, ConnectStatementDetail)> {
        let indexes = self.query_indexes();
        indexes
            .connect_edges_by_declaring_uri
            .get(uri)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns all edges incident to nodes in the given URI with full edge detail.
    pub fn edges_for_uri(&self, uri: &Url) -> Vec<(NodeId, NodeId, SemanticEdge)> {
        let indexes = self.query_indexes();
        indexes.edges_by_uri.get(uri).cloned().unwrap_or_default()
    }

    /// Returns edges incident to nodes in the given URI as (source, target, kind, optional edge name).
    /// Used for sysml/model relationships.
    pub fn edges_for_uri_as_strings(
        &self,
        uri: &Url,
    ) -> Vec<(String, String, RelationshipKind, Option<String>)> {
        let indexes = self.query_indexes();
        indexes
            .edges_by_uri
            .get(uri)
            .into_iter()
            .flatten()
            .map(|(src, tgt, e)| {
                (
                    src.qualified_name.clone(),
                    tgt.qualified_name.clone(),
                    e.kind.clone(),
                    None::<String>,
                )
            })
            .collect()
    }

    /// Returns workspace URIs represented in the graph, excluding configured library roots.
    /// Returns a clone of this graph containing only nodes from library paths.
    /// Used to extract a cacheable library-only subgraph after a full startup build.
    pub fn extract_library_subgraph(&self, library_paths: &[Url]) -> SemanticGraph {
        let mut subgraph = SemanticGraph(Arc::new(self.clone()));
        let workspace_uris: Vec<Url> = subgraph.workspace_uris_excluding_libraries(library_paths);
        for uri in workspace_uris {
            subgraph.remove_nodes_for_uri(&uri);
        }
        subgraph
    }

    pub fn workspace_uris_excluding_libraries(&self, library_paths: &[Url]) -> Vec<Url> {
        self.nodes_by_uri
            .keys()
            .filter(|uri| !workspace_uri::uri_under_any_library(uri, library_paths))
            .cloned()
            .collect()
    }

    /// Returns semantic nodes for workspace files (excluding configured library roots).
    pub fn workspace_nodes_excluding_libraries(&self, library_paths: &[Url]) -> Vec<&SemanticNode> {
        self.nodes_by_uri
            .iter()
            .filter(|(uri, _)| !workspace_uri::uri_under_any_library(uri, library_paths))
            .flat_map(|(_, ids)| ids.iter())
            .filter_map(|id| self.get_node(id))
            .collect()
    }

    /// Returns edges where both endpoints are workspace nodes (excluding libraries).
    pub fn edges_for_workspace_as_strings(
        &self,
        library_paths: &[Url],
    ) -> Vec<(String, String, RelationshipKind, Option<String>)> {
        let workspace_ids: std::collections::HashSet<_> = self
            .nodes_by_uri
            .iter()
            .filter(|(uri, _)| !workspace_uri::uri_under_any_library(uri, library_paths))
            .flat_map(|(_, ids)| ids.iter().cloned())
            .collect();
        if workspace_ids.is_empty() {
            return Vec::new();
        }
        let indexes = self.query_indexes();
        let id_by_idx = &indexes.index_to_node_id;
        let mut out = Vec::new();
        for e in self.graph.edge_references() {
            let src_id = match id_by_idx.get(&e.source()) {
                Some(id) => id.clone(),
                None => continue,
            };
            let tgt_id = match id_by_idx.get(&e.target()) {
                Some(id) => id.clone(),
                None => continue,
            };
            if workspace_ids.contains(&src_id) && workspace_ids.contains(&tgt_id) {
                out.push((
                    src_id.qualified_name,
                    tgt_id.qualified_name,
                    e.weight().kind.clone(),
                    None::<String>,
                ));
            }
        }
        out
    }

    /// Inserts a workspace node when rebuilding a graph from a persisted slice.
    pub fn insert_workspace_node(&mut self, node: SemanticNode) {
        if self.node_index_by_id.contains_key(&node.id) {
            return;
        }
        let idx = self.graph.add_node(node.clone());
        self.node_index_by_id.insert(node.id.clone(), idx);
        // See `merge_inner`'s matching comment: `nodes_by_uri` stays insertion-ordered.
        self.nodes_by_uri
            .entry(node.id.uri.clone())
            .or_default()
            .push(node.id.clone());
        insert_canonical(
            self.node_ids_by_qualified_name
                .entry(node.id.qualified_name.clone())
                .or_default(),
            node.id.clone(),
        );
        self.register_short_name_alias(&node.id, &node);
        if let Some(parent_id) = &node.parent_id {
            self.children_by_parent_id
                .entry(parent_id.clone())
                .or_default()
                .push(node.id.clone());
        }
        self.invalidate_query_indexes();
    }

    /// Inserts a directed relationship between existing workspace nodes.
    pub fn insert_workspace_edge(&mut self, source: &NodeId, target: &NodeId, edge: SemanticEdge) {
        let Some(&source_idx) = self.node_index_by_id.get(source) else {
            return;
        };
        let Some(&target_idx) = self.node_index_by_id.get(target) else {
            return;
        };
        self.graph.add_edge(source_idx, target_idx, edge);
        self.invalidate_query_indexes();
    }

    pub fn restore_pending_relationship(&mut self, pending: PendingRelationship) {
        self.pending_relationships.push(pending);
    }

    pub fn restore_pending_expression_relationship(
        &mut self,
        pending: PendingExpressionRelationship,
    ) {
        self.pending_expression_relationships.push(pending);
    }
}
