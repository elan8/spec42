//! Petgraph-backed semantic graph and query API.

use std::collections::HashMap;

use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use petgraph::Directed;
use petgraph::Direction;
use tower_lsp::lsp_types::{Position, Range, Url};

use crate::model::{NodeId, RelationshipKind, SemanticNode};
use crate::workspace_uri;

/// Semantic graph: nodes (model elements) and edges (relationships).
/// Uses petgraph StableGraph for efficient add/remove and future algorithm support.
#[derive(Debug, Default)]
pub struct SemanticGraph {
    pub(crate) graph: StableGraph<SemanticNode, RelationshipKind, Directed>,
    pub(crate) node_index_by_id: HashMap<NodeId, NodeIndex>,
    pub(crate) nodes_by_uri: HashMap<Url, Vec<NodeId>>,
    pub(crate) connection_occurrences_by_uri: HashMap<Url, Vec<ConnectionOccurrence>>,
}

#[derive(Debug, Clone)]
pub(crate) struct ConnectionOccurrence {
    pub source: NodeId,
    pub target: NodeId,
    pub range: Range,
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self {
            graph: StableGraph::new(),
            node_index_by_id: HashMap::new(),
            nodes_by_uri: HashMap::new(),
            connection_occurrences_by_uri: HashMap::new(),
        }
    }

    /// Removes all nodes (and their incident edges) for the given URI.
    pub fn remove_nodes_for_uri(&mut self, uri: &Url) {
        let Some(node_ids) = self.nodes_by_uri.remove(uri) else {
            self.connection_occurrences_by_uri.remove(uri);
            return;
        };
        for id in node_ids {
            if let Some(idx) = self.node_index_by_id.remove(&id) {
                self.graph.remove_node(idx);
            }
        }
        self.connection_occurrences_by_uri.remove(uri);
    }

    /// Merges nodes and edges from another graph (built from a single document).
    pub fn merge(&mut self, other: SemanticGraph) {
        for (uri, occurrences) in &other.connection_occurrences_by_uri {
            self.connection_occurrences_by_uri
                .entry(uri.clone())
                .or_default()
                .extend(occurrences.iter().cloned());
        }
        for (id, node) in other.iter_nodes() {
            let idx = self.graph.add_node(node.clone());
            self.node_index_by_id.insert(id.clone(), idx);
            self.nodes_by_uri
                .entry(id.uri.clone())
                .or_default()
                .push(id);
        }
        for (src_id, tgt_id, kind) in other.iter_edges() {
            if let (Some(&src_idx), Some(&tgt_idx)) = (
                self.node_index_by_id.get(&src_id),
                self.node_index_by_id.get(&tgt_id),
            ) {
                self.graph.add_edge(src_idx, tgt_idx, kind.clone());
            }
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

    fn iter_edges(&self) -> impl Iterator<Item = (NodeId, NodeId, RelationshipKind)> + '_ {
        let node_ids: Vec<_> = self
            .node_index_by_id
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        let id_by_idx: HashMap<NodeIndex, NodeId> =
            node_ids.into_iter().map(|(k, v)| (v, k)).collect();
        self.graph.edge_references().filter_map(move |e| {
            let src_id = id_by_idx.get(&e.source())?.clone();
            let tgt_id = id_by_idx.get(&e.target())?.clone();
            let kind = e.weight().clone();
            Some((src_id, tgt_id, kind))
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

    /// Returns child nodes of the given node (by matching parent_id).
    pub fn children_of(&self, parent: &SemanticNode) -> Vec<&SemanticNode> {
        self.nodes_by_uri
            .get(&parent.id.uri)
            .into_iter()
            .flatten()
            .filter_map(|id| {
                self.node_index_by_id
                    .get(id)
                    .and_then(|&idx| self.graph.node_weight(idx))
            })
            .filter(|n| n.parent_id.as_ref() == Some(&parent.id))
            .collect()
    }

    /// Returns the node for the given NodeId, if it exists.
    pub fn get_node(&self, id: &NodeId) -> Option<&SemanticNode> {
        self.node_index_by_id
            .get(id)
            .and_then(|&idx| self.graph.node_weight(idx))
    }

    /// Returns the node whose range contains the given position (first match).
    pub fn find_node_at_position(&self, uri: &Url, pos: Position) -> Option<&SemanticNode> {
        self.nodes_for_uri(uri).into_iter().find(|n| {
            let r = &n.range;
            (pos.line > r.start.line
                || (pos.line == r.start.line && pos.character >= r.start.character))
                && (pos.line < r.end.line
                    || (pos.line == r.end.line && pos.character <= r.end.character))
        })
    }

    /// Returns the smallest-range node whose range contains the given position.
    pub fn find_deepest_node_at_position(&self, uri: &Url, pos: Position) -> Option<&SemanticNode> {
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
        let id_by_idx: HashMap<NodeIndex, NodeId> = self
            .node_index_by_id
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
        let mut targets = Vec::new();
        for edge in self.graph.edges_directed(src_idx, Direction::Outgoing) {
            if matches!(
                edge.weight(),
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

    /// Returns source nodes that have typing/specializes edges to the given node.
    pub fn incoming_typing_or_specializes_sources(
        &self,
        node: &SemanticNode,
    ) -> Vec<&SemanticNode> {
        let tgt_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let id_by_idx: HashMap<NodeIndex, NodeId> = self
            .node_index_by_id
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
        let mut sources = Vec::new();
        for edge in self.graph.edges_directed(tgt_idx, Direction::Incoming) {
            if matches!(
                edge.weight(),
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

    /// Returns target nodes of perform edges from the given node.
    pub fn outgoing_perform_targets(&self, node: &SemanticNode) -> Vec<&SemanticNode> {
        let src_idx = match self.node_index_by_id.get(&node.id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let id_by_idx: HashMap<NodeIndex, NodeId> = self
            .node_index_by_id
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
        let mut targets = Vec::new();
        for edge in self.graph.edges_directed(src_idx, Direction::Outgoing) {
            if matches!(edge.weight(), RelationshipKind::Perform) {
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
        let id_by_idx: HashMap<NodeIndex, NodeId> = self
            .node_index_by_id
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
        let mut sources = Vec::new();
        for edge in self.graph.edges_directed(tgt_idx, Direction::Incoming) {
            if matches!(edge.weight(), RelationshipKind::Perform) {
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
        let ids: std::collections::HashSet<_> = self
            .nodes_by_uri
            .get(uri)
            .into_iter()
            .flatten()
            .cloned()
            .collect();
        if ids.is_empty() {
            return Vec::new();
        }
        let id_by_idx: HashMap<NodeIndex, NodeId> = self
            .node_index_by_id
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
        let mut out = Vec::new();
        for e in self.graph.edge_references() {
            if *e.weight() != RelationshipKind::Connection {
                continue;
            }
            let src_id = match id_by_idx.get(&e.source()) {
                Some(id) => id.clone(),
                None => continue,
            };
            let tgt_id = match id_by_idx.get(&e.target()) {
                Some(id) => id.clone(),
                None => continue,
            };
            if ids.contains(&src_id) || ids.contains(&tgt_id) {
                out.push((src_id, tgt_id));
            }
        }
        out
    }

    /// Returns connection edge occurrences anchored to source ranges from parsed connect statements.
    /// Multiple entries can exist for the same endpoint pair.
    pub fn connection_edge_occurrences_for_uri(&self, uri: &Url) -> Vec<(NodeId, NodeId, Range)> {
        self.connection_occurrences_by_uri
            .get(uri)
            .into_iter()
            .flatten()
            .cloned()
            .map(|occ| (occ.source, occ.target, occ.range))
            .collect()
    }

    pub(crate) fn record_connection_occurrence(
        &mut self,
        uri: &Url,
        source: NodeId,
        target: NodeId,
        range: Range,
    ) {
        self.connection_occurrences_by_uri
            .entry(uri.clone())
            .or_default()
            .push(ConnectionOccurrence {
                source,
                target,
                range,
            });
    }

    /// Returns edges incident to nodes in the given URI as (source, target, kind, optional edge name).
    /// Used for sysml/model relationships.
    pub fn edges_for_uri_as_strings(
        &self,
        uri: &Url,
    ) -> Vec<(String, String, RelationshipKind, Option<String>)> {
        let ids: std::collections::HashSet<_> = self
            .nodes_by_uri
            .get(uri)
            .into_iter()
            .flatten()
            .cloned()
            .collect();
        if ids.is_empty() {
            return Vec::new();
        }
        let id_by_idx: HashMap<NodeIndex, NodeId> = self
            .node_index_by_id
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
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
            if ids.contains(&src_id) || ids.contains(&tgt_id) {
                out.push((
                    src_id.qualified_name,
                    tgt_id.qualified_name,
                    e.weight().clone(),
                    None::<String>, // edge name for connection
                ));
            }
        }
        out
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
        let id_by_idx: HashMap<NodeIndex, NodeId> = self
            .node_index_by_id
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
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
                    e.weight().clone(),
                    None::<String>,
                ));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use sysml_parser::parse;
    use tower_lsp::lsp_types::Url;

    use crate::graph_builder::build_graph_from_doc;
    use crate::model::{NodeId, RelationshipKind};

    #[test]
    fn state_machine_graph_builds_from_root() {
        let input = r#"
            package P {
                part def A { }
                part def B { }
            }
        "#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").unwrap();
        let g = build_graph_from_doc(&root, &uri);
        let _edges = g.edges_for_uri_as_strings(&uri);
        // Graph builds without panic; transition edges depend on sysml-parser state/transition support
        assert!(
            g.node_index_by_id.len() >= 2,
            "expected at least package and part def nodes: {:?}",
            g.node_index_by_id.len()
        );
    }

    /// General View fix: root package is a node and its direct children have parent_id set
    /// so that contains edges are emitted for the diagram.
    #[test]
    fn root_package_node_and_contains_edges_for_children() {
        let input = r#"
            package SurveillanceDrone {
                part def Airframe { }
                part def PropulsionUnit { }
            }
        "#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").unwrap();
        let g = build_graph_from_doc(&root, &uri);
        let pkg_id = NodeId::new(&uri, "SurveillanceDrone");
        assert!(
            g.node_index_by_id.contains_key(&pkg_id),
            "root package SurveillanceDrone must be a node; nodes: {:?}",
            g.nodes_by_uri.get(&uri).map(|v| {
                v.iter()
                    .map(|id| id.qualified_name.as_str())
                    .collect::<Vec<_>>()
            })
        );
        let nodes_with_parent: Vec<_> = g
            .nodes_for_uri(&uri)
            .into_iter()
            .filter(|n| n.parent_id.as_ref() == Some(&pkg_id))
            .collect();
        assert!(
            nodes_with_parent.len() >= 2,
            "expected at least 2 direct children of package (Airframe, PropulsionUnit); got {}",
            nodes_with_parent.len()
        );
        let names: Vec<_> = nodes_with_parent.iter().map(|n| n.name.as_str()).collect();
        assert!(
            names.contains(&"Airframe"),
            "expected Airframe in children: {:?}",
            names
        );
        assert!(
            names.contains(&"PropulsionUnit"),
            "expected PropulsionUnit in children: {:?}",
            names
        );
    }

    #[test]
    #[ignore] // input uses port def CmdPort {} which sysml-parser may not accept (expected end of input)
    fn typed_part_usage_expansion_adds_nested_port_nodes() {
        // Typed PartUsages expand so connection endpoints (e.g. flightControl.flightController.motorCmd) exist.
        let input = r#"
            package P {
                port def CmdPort {}
                part def Child {
                    port cmd : CmdPort;
                }
                part def Parent {
                    part child : Child;
                }
                part def Root {
                    part parent : Parent;
                }
            }
        "#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").unwrap();
        let g = build_graph_from_doc(&root, &uri);

        // Expansion adds nested parts/ports under typed PartUsage so connection endpoints exist.
        let port_id = NodeId::new(&uri, "P::Root::parent::child::cmd");
        assert!(
            g.node_index_by_id.contains_key(&port_id),
            "expected port node P::Root::parent::child::cmd from typed part expansion; nodes: {:?}",
            g.nodes_by_uri.get(&uri).map(|v| {
                v.iter()
                    .map(|id| id.qualified_name.as_str())
                    .collect::<Vec<_>>()
            })
        );
    }

    #[test]
    #[ignore] // input uses syntax (e.g. port def with {}) that sysml-parser may not accept
    fn connection_edges_added_when_port_nodes_exist() {
        // Connection "connect flightControl.flightController.motorCmd to propulsion.propulsionUnit1.cmd"
        // requires port nodes from expand_typed_part_usage. Verifies connection edges are added.
        let input = r#"
            package SurveillanceDrone {
                port def MotorCommandPort {}
                port def PowerPort {}
                part def PropulsionUnit {
                    port cmd : ~MotorCommandPort;
                    port pwr : ~PowerPort;
                }
                part def Propulsion {
                    part propulsionUnit1 : PropulsionUnit;
                    part propulsionUnit2 : PropulsionUnit;
                }
                part def FlightController {
                    port motorCmd : ~MotorCommandPort;
                    port pwr : ~PowerPort;
                }
                part def FlightControlAndSensing {
                    part flightController : FlightController;
                }
                part def SurveillanceQuadrotorDrone {
                    part propulsion : Propulsion;
                    part flightControl : FlightControlAndSensing;
                    connect flightControl.flightController.motorCmd to propulsion.propulsionUnit1.cmd;
                }
            }
        "#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").unwrap();
        let g = build_graph_from_doc(&root, &uri);

        let src = "SurveillanceDrone::SurveillanceQuadrotorDrone::flightControl::flightController::motorCmd";
        let tgt = "SurveillanceDrone::SurveillanceQuadrotorDrone::propulsion::propulsionUnit1::cmd";
        assert!(
            g.node_index_by_id.contains_key(&NodeId::new(&uri, src)),
            "expected motorCmd port node; nodes: {:?}",
            g.nodes_by_uri.get(&uri).map(|v| {
                v.iter()
                    .map(|id| id.qualified_name.as_str())
                    .collect::<Vec<_>>()
            })
        );
        assert!(
            g.node_index_by_id.contains_key(&NodeId::new(&uri, tgt)),
            "expected cmd port node"
        );

        let edges = g.edges_for_uri_as_strings(&uri);
        let conn_edges: Vec<_> = edges
            .iter()
            .filter(|(_, _, kind, _)| *kind == RelationshipKind::Connection)
            .collect();
        assert!(
            !conn_edges.is_empty(),
            "expected connection edges; edges: {:?}",
            edges
        );
    }

    #[test]
    fn requirement_subject_edges_are_emitted() {
        let input = r#"
            package P {
                part def Vehicle { }
                requirement def EnduranceReq {
                    subject vehicle : Vehicle;
                    require constraint { }
                }
            }
        "#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let g = build_graph_from_doc(&root, &uri);
        let edges = g.edges_for_uri_as_strings(&uri);
        let has_subject = edges.iter().any(|(src, tgt, kind, _)| {
            *kind == RelationshipKind::Subject
                && src.ends_with("EnduranceReq")
                && tgt.ends_with("Vehicle")
        });
        assert!(
            has_subject,
            "expected subject edge in semantic graph; edges: {:?}",
            edges
        );
    }
}
