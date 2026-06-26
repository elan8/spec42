//! Petgraph-backed semantic graph and query API.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::semantic::text_span::{TextPosition, TextRange};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use petgraph::Directed;
use petgraph::Direction;
use url::Url;

use crate::semantic::model::{
    ConnectStatementDetail, NodeId, RelationshipKind, SemanticEdge, SemanticNode,
};
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
#[derive(Debug)]
pub struct SemanticGraphData {
    pub graph: StableGraph<SemanticNode, SemanticEdge, Directed>,
    pub node_index_by_id: HashMap<NodeId, NodeIndex>,
    pub nodes_by_uri: HashMap<Url, Vec<NodeId>>,
    pub node_ids_by_qualified_name: HashMap<String, Vec<NodeId>>,
    /// Incrementally maintained parent → children index. O(1) children lookup.
    pub children_by_parent_id: HashMap<NodeId, Vec<NodeId>>,
    pub pending_expression_relationships: Vec<PendingExpressionRelationship>,
    pub pending_relationships: Vec<PendingRelationship>,
    pub import_lookup_cache: Mutex<HashMap<(NodeId, String, bool), Vec<NodeId>>>,
    query_indexes: Mutex<Option<Arc<GraphQueryIndexes>>>,
    shape_cache: Mutex<ShapeCache>,
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

#[derive(Debug, Clone)]
pub struct PendingExpressionRelationship {
    pub uri: Url,
    pub source_expression: String,
    pub target_expression: String,
    pub kind: RelationshipKind,
    pub container_prefix: Option<String>,
    pub source_range: TextRange,
}

#[derive(Debug, Clone)]
pub struct PendingRelationship {
    pub uri: Url,
    pub source_qualified: String,
    pub target_qualified: String,
    pub kind: RelationshipKind,
    pub target_kinds: Option<Vec<String>>,
}

impl SemanticGraphData {
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
            edges_by_uri
                .entry(src_id.uri.clone())
                .or_default()
                .push((src_id.clone(), tgt_id.clone(), weight.clone()));
            if tgt_id.uri != src_id.uri {
                edges_by_uri
                    .entry(tgt_id.uri.clone())
                    .or_default()
                    .push((src_id.clone(), tgt_id.clone(), weight.clone()));
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

    /// Removes all nodes (and their incident edges) for the given URI.
    pub fn remove_nodes_for_uri(&mut self, uri: &Url) {
        let Some(node_ids) = self.nodes_by_uri.remove(uri) else {
            self.clear_import_lookup_cache();
            return;
        };
        // Collect parent_ids before removal so we can update the children index.
        let parent_ids: Vec<(NodeId, Option<NodeId>)> = node_ids
            .iter()
            .map(|id| {
                let parent = self
                    .node_index_by_id
                    .get(id)
                    .and_then(|&idx| self.graph.node_weight(idx))
                    .and_then(|n| n.parent_id.clone());
                (id.clone(), parent)
            })
            .collect();

        for id in &node_ids {
            let mut remove_lookup_entry = false;
            if let Some(ids) = self.node_ids_by_qualified_name.get_mut(&id.qualified_name) {
                ids.retain(|existing| existing != id);
                remove_lookup_entry = ids.is_empty();
            }
            if remove_lookup_entry {
                self.node_ids_by_qualified_name.remove(&id.qualified_name);
            }
            if let Some(idx) = self.node_index_by_id.remove(id) {
                self.graph.remove_node(idx);
            }
            self.children_by_parent_id.remove(id);
        }
        // Remove each node from its parent's children list.
        for (id, parent_id) in parent_ids {
            if let Some(pid) = parent_id {
                if let Some(children) = self.children_by_parent_id.get_mut(&pid) {
                    children.retain(|c| c != &id);
                }
            }
        }
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
            .filter(|node| node.name == name)
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
            .filter(|child| child.name == name)
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
        indexes
            .edges_by_uri
            .get(uri)
            .cloned()
            .unwrap_or_default()
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
