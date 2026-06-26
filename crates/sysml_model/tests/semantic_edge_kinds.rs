use sysml_model::{
    add_semantic_edge_once, AddSemanticEdgeResult, ConnectStatementDetail, ElementKind, NodeId,
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
            element_kind: ElementKind::Port,
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
fn fan_out_connect_statements_with_distinct_targets_add_parallel_connection_edges() {
    let uri = Url::parse("memory://workspace/fanout_connect.sysml").expect("uri");
    let (mut graph, a, b) = port_nodes(&uri, "Pkg::MotorCmd", "Pkg::UnitCmd");
    let connect = |target: &str| ConnectStatementDetail {
        declaring_uri: uri.clone(),
        range: TextRange::new(TextPosition::new(1, 0), TextPosition::new(1, 1)),
        source_expression: "flightController.motorCmd".to_string(),
        target_expression: target.to_string(),
        container_prefix: Some("Pkg::Drone".to_string()),
    };
    for target in [
        "propulsion.propulsionUnit1.cmd",
        "propulsion.propulsionUnit2.cmd",
        "propulsion.propulsionUnit3.cmd",
        "propulsion.propulsionUnit4.cmd",
    ] {
        assert_eq!(
            add_semantic_edge_once(
                &mut graph,
                &a,
                &b,
                SemanticEdge::connection_with_connect(connect(target)),
            ),
            AddSemanticEdgeResult::Added
        );
    }
    assert_eq!(graph.graph.edge_count(), 4);
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
