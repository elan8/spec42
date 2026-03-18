//! Semantic graph model for SysML v2 documents.
//!
//! Builds a petgraph-based graph from parsed ASTs. Nodes represent model elements
//! (packages, parts, ports, etc.); edges represent SysML relationships
//! (typing, specializes, connection, bind, allocate, transition).

mod graph_builder;
mod relationships;

pub use graph_builder::build_graph_from_doc;

use sysml_parser::ast::{PackageBody, PackageBodyElement, RootElement};

use crate::ast_util::identification_name;

/// Extracts (elements, qualified, name_display, span) from Package or Namespace RootElement.
/// Returns None if body is not Brace.
pub(crate) fn root_element_body<'a>(
    re: &'a RootElement,
) -> Option<(
    &'a [sysml_parser::Node<PackageBodyElement>],
    String,
    String,
    &'a sysml_parser::Span,
)> {
    let (ident, body, span) = match re {
        RootElement::Package(p) => (&p.identification, &p.body, &p.span),
        RootElement::Namespace(n) => (&n.identification, &n.body, &n.span),
        RootElement::LibraryPackage(lp) => (&lp.identification, &lp.body, &lp.span),
        RootElement::Import(_) => return None,
    };
    let name = identification_name(ident);
    let qualified = if name.is_empty() {
        "(top level)".to_string()
    } else {
        name.clone()
    };
    let name_display = if name.is_empty() {
        "(top level)".to_string()
    } else {
        name
    };
    match body {
        PackageBody::Brace { elements } => Some((elements, qualified, name_display, span)),
        _ => None,
    }
}
pub use relationships::add_cross_document_edges_for_uri;

use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use petgraph::Directed;
use petgraph::Direction;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Position, Range, Url};

use tower_lsp::lsp_types::SymbolKind;

use crate::language::SymbolEntry;

/// Unique identifier for a node in the semantic graph.
/// Combines document URI and qualified name for workspace-wide uniqueness.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct NodeId {
    pub uri: Url,
    pub qualified_name: String,
}

impl NodeId {
    pub fn new(uri: &Url, qualified_name: impl Into<String>) -> Self {
        Self {
            uri: uri.clone(),
            qualified_name: qualified_name.into(),
        }
    }
}

/// SysML v2 relationship kinds (edges in the graph).
#[allow(dead_code)] // some relationship kinds are staged for upcoming semantic features
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelationshipKind {
    Typing,
    Specializes,
    Connection,
    Bind,
    Perform,
    Allocate,
    Satisfy,
    Transition,
}

impl RelationshipKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationshipKind::Typing => "typing",
            RelationshipKind::Specializes => "specializes",
            RelationshipKind::Connection => "connection",
            RelationshipKind::Bind => "bind",
            RelationshipKind::Perform => "perform",
            RelationshipKind::Allocate => "allocate",
            RelationshipKind::Satisfy => "satisfy",
            RelationshipKind::Transition => "transition",
        }
    }
}

/// A node in the semantic graph representing a model element.
#[derive(Debug, Clone)]
pub struct SemanticNode {
    pub id: NodeId,
    pub element_kind: String,
    pub name: String,
    pub range: Range,
    pub attributes: HashMap<String, serde_json::Value>,
    pub parent_id: Option<NodeId>,
}

/// Semantic graph: nodes (model elements) and edges (relationships).
/// Uses petgraph StableGraph for efficient add/remove and future algorithm support.
#[derive(Debug, Default)]
pub struct SemanticGraph {
    pub(crate) graph: StableGraph<SemanticNode, RelationshipKind, Directed>,
    pub(crate) node_index_by_id: HashMap<NodeId, NodeIndex>,
    pub(crate) nodes_by_uri: HashMap<Url, Vec<NodeId>>,
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self {
            graph: StableGraph::new(),
            node_index_by_id: HashMap::new(),
            nodes_by_uri: HashMap::new(),
        }
    }

    /// Removes all nodes (and their incident edges) for the given URI.
    pub fn remove_nodes_for_uri(&mut self, uri: &Url) {
        let Some(node_ids) = self.nodes_by_uri.remove(uri) else {
            return;
        };
        for id in node_ids {
            if let Some(idx) = self.node_index_by_id.remove(&id) {
                self.graph.remove_node(idx);
            }
        }
    }

    /// Merges nodes and edges from another graph (built from a single document).
    pub fn merge(&mut self, other: SemanticGraph) {
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
}

/// Maps element_kind from the semantic model to LSP SymbolKind.
fn element_kind_to_symbol_kind(kind: &str) -> SymbolKind {
    match kind {
        "package" => SymbolKind::MODULE,
        "part def" => SymbolKind::CLASS,
        "part" => SymbolKind::VARIABLE,
        "attribute def" => SymbolKind::PROPERTY,
        "attribute" => SymbolKind::PROPERTY,
        "port def" => SymbolKind::INTERFACE,
        "port" => SymbolKind::INTERFACE,
        "interface" => SymbolKind::INTERFACE,
        "connection" => SymbolKind::VARIABLE,
        "item def" => SymbolKind::CONSTANT,
        "item" => SymbolKind::CONSTANT,
        "requirement def" => SymbolKind::STRING,
        "requirement" => SymbolKind::STRING,
        "action def" => SymbolKind::FUNCTION,
        "state def" => SymbolKind::ENUM_MEMBER,
        "state" => SymbolKind::ENUM_MEMBER,
        "use case def" => SymbolKind::EVENT,
        "actor def" => SymbolKind::CONSTRUCTOR,
        _ => SymbolKind::NULL,
    }
}

/// Builds a signature string from node attributes (partType, specializes, etc.).
pub(crate) fn signature_from_node(node: &SemanticNode) -> Option<String> {
    let kind = node.element_kind.as_str();
    let mult = node
        .attributes
        .get("multiplicity")
        .and_then(|v| v.as_str())
        .map(|m| format!(" {}", m))
        .unwrap_or_default();
    let (type_attr, type_suffix) = match kind {
        "part def" | "part" => (
            node.attributes
                .get("partType")
                .or_else(|| node.attributes.get("specializes")),
            " : ",
        ),
        "attribute def" | "attribute" => (node.attributes.get("attributeType"), " : "),
        "port def" | "port" => (node.attributes.get("portType"), " : "),
        "actor def" => (node.attributes.get("actorType"), " : "),
        "item def" => (node.attributes.get("specializes"), " :> "),
        "item" => (node.attributes.get("itemType"), " : "),
        _ => (None, ""),
    };
    let type_part = type_attr
        .and_then(|v| v.as_str())
        .map(|t| format!("{}{}", type_suffix, t))
        .unwrap_or_default();
    Some(format!("{} {}{}{};", kind, node.name, type_part, mult))
}

pub fn hover_markdown_for_node(
    graph: &SemanticGraph,
    node: &SemanticNode,
    show_location: bool,
) -> String {
    let mut md = format!("**{}** `{}`\n\n", node.element_kind, node.name);
    let code_block = signature_from_node(node)
        .unwrap_or_else(|| format!("{} {};", node.element_kind, node.name));
    md.push_str("```sysml\n");
    md.push_str(&code_block);
    md.push_str("\n```\n\n");

    if let Some(parent_id) = &node.parent_id {
        if let Some(parent) = graph.get_node(parent_id) {
            md.push_str(&format!("*Container:* `{}`\n\n", parent.name));
        }
    }

    if let Some(type_name) = node
        .attributes
        .get("partType")
        .or_else(|| node.attributes.get("attributeType"))
        .or_else(|| node.attributes.get("portType"))
        .or_else(|| node.attributes.get("actorType"))
        .or_else(|| node.attributes.get("itemType"))
        .and_then(|value| value.as_str())
    {
        md.push_str(&format!("*Type:* `{}`\n\n", type_name));
    }

    if let Some(multiplicity) = node
        .attributes
        .get("multiplicity")
        .and_then(|value| value.as_str())
    {
        md.push_str(&format!("*Multiplicity:* `{}`\n\n", multiplicity));
    }

    let typed_targets = graph.outgoing_typing_or_specializes_targets(node);
    if let Some(target) = typed_targets.first() {
        md.push_str(&format!(
            "*Resolves to:* `{}` ({})\n\n",
            target.name, target.element_kind
        ));
    }

    if show_location {
        md.push_str(&format!("*Defined in:* {}", node.id.uri.path()));
    }

    md
}

/// Collects symbol entries for a URI from the semantic graph (replaces AST-based collect_symbol_entries).
pub fn symbol_entries_for_uri(graph: &SemanticGraph, uri: &Url) -> Vec<SymbolEntry> {
    let mut out = Vec::new();
    for node in graph.nodes_for_uri(uri) {
        let container_name = node
            .parent_id
            .as_ref()
            .and_then(|pid| graph.get_node(pid))
            .map(|p| p.name.clone());
        let description = format!("{} '{}'", node.element_kind, node.name);
        let signature = signature_from_node(node);
        out.push(SymbolEntry {
            name: node.name.clone(),
            uri: node.id.uri.clone(),
            range: node.range,
            kind: element_kind_to_symbol_kind(&node.element_kind),
            container_name,
            detail: Some(node.element_kind.clone()),
            description: Some(description),
            signature,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_parser::parse;

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
            g.nodes_by_uri.get(&uri).map(|v| v
                .iter()
                .map(|id| id.qualified_name.as_str())
                .collect::<Vec<_>>())
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
            g.nodes_by_uri.get(&uri).map(|v| v
                .iter()
                .map(|id| id.qualified_name.as_str())
                .collect::<Vec<_>>())
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
            g.nodes_by_uri.get(&uri).map(|v| v
                .iter()
                .map(|id| id.qualified_name.as_str())
                .collect::<Vec<_>>())
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
}
