//! Petgraph-backed semantic graph and query API.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::semantic::text_span::{TextPosition, TextRange};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use petgraph::Directed;
use petgraph::Direction;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

use crate::semantic::model::{
    node_matches_simple_name, ConnectStatementDetail, ElementKind, NodeId, RelationshipKind,
    SemanticEdge, SemanticNode,
};

fn serialize_url<S: Serializer>(url: &Url, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(url.as_str())
}

fn deserialize_url<'de, D: Deserializer<'de>>(d: D) -> Result<Url, D::Error> {
    let s = String::deserialize(d)?;
    Url::parse(&s).map_err(serde::de::Error::custom)
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
    /// Rebuilt after deserialization via [`rebuild_derived_indexes`].
    #[serde(skip)]
    pub node_index_by_id: HashMap<NodeId, NodeIndex>,
    pub nodes_by_uri: HashMap<Url, Vec<NodeId>>,
    pub node_ids_by_qualified_name: HashMap<String, Vec<NodeId>>,
    /// Rebuilt after deserialization via [`rebuild_derived_indexes`].
    #[serde(skip)]
    pub children_by_parent_id: HashMap<NodeId, Vec<NodeId>>,
    pub pending_expression_relationships: Vec<PendingExpressionRelationship>,
    pub pending_relationships: Vec<PendingRelationship>,
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
    /// re-checking it. Rebuilt after deserialization via [`rebuild_derived_indexes`].
    #[serde(skip)]
    pub document_dependency_targets: HashMap<Url, HashSet<Url>>,
    /// Reverse of `document_dependency_targets`: for each URI, the other URIs that statically
    /// depend on it. This is `refresh_relationship_frontier`'s frontier source — every URI
    /// that might need its relationships re-resolved after `changed_uri` is edited. Rebuilt
    /// after deserialization via [`rebuild_derived_indexes`].
    #[serde(skip)]
    pub document_dependents: HashMap<Url, HashSet<Url>>,
    /// The exact (src, tgt, kind) triples `add_cross_document_edges_for_uri` last added for a
    /// given source URI. Lets a re-resolve for that URI cleanly remove its own prior
    /// cross-document edges before adding fresh ones, without touching edges owned by other
    /// passes. Rebuilt after deserialization via [`rebuild_derived_indexes`].
    #[serde(skip)]
    pub cross_document_edges_by_source_uri: HashMap<Url, Vec<(NodeId, NodeId, RelationshipKind)>>,
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
            children_by_parent_id: self.children_by_parent_id.clone(),
            pending_expression_relationships: self.pending_expression_relationships.clone(),
            pending_relationships: self.pending_relationships.clone(),
            import_lookup_cache: Mutex::new(HashMap::new()),
            query_indexes: Mutex::new(None),
            shape_cache: Mutex::new(ShapeCache::default()),
            document_dependency_targets: self.document_dependency_targets.clone(),
            document_dependents: self.document_dependents.clone(),
            cross_document_edges_by_source_uri: self.cross_document_edges_by_source_uri.clone(),
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

    pub fn into_data(self) -> SemanticGraphData {
        Arc::try_unwrap(self.0).unwrap_or_else(|arc| (*arc).clone())
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
    /// Rebuild `node_index_by_id` and `children_by_parent_id` from the petgraph
    /// `graph` after deserialization (both fields are `#[serde(skip)]`).
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
            children_by_parent_id: HashMap::new(),
            pending_expression_relationships: Vec::new(),
            pending_relationships: Vec::new(),
            import_lookup_cache: Mutex::new(HashMap::new()),
            query_indexes: Mutex::new(None),
            shape_cache: Mutex::new(ShapeCache::default()),
            document_dependency_targets: HashMap::new(),
            document_dependents: HashMap::new(),
            cross_document_edges_by_source_uri: HashMap::new(),
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
        let mut guard = self
            .query_indexes
            .lock()
            .expect("semantic graph query indexes lock");
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
    /// `graph_builder::attach_short_name_attribute`) — the same qualified name a sibling
    /// declared under the short name directly would get. `None` if `node` has no short name.
    pub(crate) fn short_name_alias_qualified(node: &SemanticNode) -> Option<String> {
        let short_name = node.attributes.get("shortName").and_then(|v| v.as_str())?;
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
            self.node_ids_by_qualified_name
                .entry(short_qualified)
                .or_default()
                .push(id.clone());
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

    /// Removes all nodes (and their incident edges) for the given URI.
    pub fn remove_nodes_for_uri(&mut self, uri: &Url) {
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
    /// Skips a library node when its qualified name already exists, or when it
    /// belongs to a package name declared in `shadowed_packages` (workspace wins).
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
        self.pending_relationships
            .extend(other.pending_relationships.iter().cloned());
        self.pending_expression_relationships
            .extend(other.pending_expression_relationships.iter().cloned());
        for (id, node) in other.iter_nodes() {
            if let Some(shadowed) = shadowed_packages {
                if self
                    .node_ids_by_qualified_name
                    .contains_key(&id.qualified_name)
                    || Self::qualified_name_under_packages(&id.qualified_name, shadowed)
                {
                    continue;
                }
            }
            let idx = self.graph.add_node(node.clone());
            self.node_index_by_id.insert(id.clone(), idx);
            self.nodes_by_uri
                .entry(id.uri.clone())
                .or_default()
                .push(id.clone());
            self.node_ids_by_qualified_name
                .entry(id.qualified_name.clone())
                .or_default()
                .push(id.clone());
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
        self.nodes_by_uri
            .entry(node.id.uri.clone())
            .or_default()
            .push(node.id.clone());
        self.node_ids_by_qualified_name
            .entry(node.id.qualified_name.clone())
            .or_default()
            .push(node.id.clone());
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
