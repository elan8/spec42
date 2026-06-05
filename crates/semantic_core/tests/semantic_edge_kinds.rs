use semantic_core::{
    add_semantic_edge_once, AddSemanticEdgeResult, ConnectStatementDetail, NodeId,
    RelationshipKind, SemanticEdge, SemanticGraph, SemanticNode, TextPosition, TextRange,
};
use url::Url;

fn port_nodes(uri: &Url, a_qn: &str, b_qn: &str) -> (SemanticGraph, NodeId, NodeId) {
    let mut graph = SemanticGraph::new();
    let a = NodeId::new(uri, a_qn);
    let b = NodeId::new(uri, b_qn);
    let range = TextRange::new(TextPosition::new(1, 0), TextPosition::new(1, 1));
    for (id, name) in [(a.clone(), "a"), (b.clone(), "b")] {
        graph.insert_workspace_node(SemanticNode {
            id,
            element_kind: "port".to_string(),
            name: name.to_string(),
            range,
            attributes: Default::default(),
            parent_id: None,
        });
    }
    (graph, a, b)
}

#[test]
fn typing_and_connection_edges_can_coexist_between_same_node_pair() {
    let uri = Url::parse("memory://workspace/edge_kinds.sysml").expect("uri");
    let (mut graph, a, b) = port_nodes(&uri, "Pkg::A", "Pkg::B");
    graph.insert_workspace_edge(&a, &b, SemanticEdge::plain(RelationshipKind::Typing));
    graph.insert_workspace_edge(&a, &b, SemanticEdge::plain(RelationshipKind::Connection));
    assert_eq!(graph.graph.edge_count(), 2);
    let node = graph.get_node(&a).expect("node");
    let kinds: Vec<_> = graph
        .outgoing_relationships(node)
        .into_iter()
        .map(|(_, kind)| kind)
        .collect();
    assert!(kinds.contains(&RelationshipKind::Typing));
    assert!(kinds.contains(&RelationshipKind::Connection));
}

#[test]
fn duplicate_connect_on_existing_connection_pair_is_rejected() {
    let uri = Url::parse("memory://workspace/dup_connect.sysml").expect("uri");
    let (mut graph, a, b) = port_nodes(&uri, "Pkg::A", "Pkg::B");
    let connect = ConnectStatementDetail {
        declaring_uri: uri.clone(),
        range: TextRange::new(TextPosition::new(1, 0), TextPosition::new(1, 1)),
        source_expression: "a".to_string(),
        target_expression: "b".to_string(),
        container_prefix: None,
    };
    assert_eq!(
        add_semantic_edge_once(
            &mut graph,
            &a,
            &b,
            SemanticEdge::connection_with_connect(connect.clone()),
        ),
        AddSemanticEdgeResult::Added
    );
    assert_eq!(
        add_semantic_edge_once(
            &mut graph,
            &a,
            &b,
            SemanticEdge::connection_with_connect(connect),
        ),
        AddSemanticEdgeResult::DuplicateConnect
    );
    assert_eq!(graph.graph.edge_count(), 1);
}
