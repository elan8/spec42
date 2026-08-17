use sysml_model::{
    build_semantic_graph_from_documents, NodeId, SysmlDocument, SysmlDocumentSourceKind,
};
use url::Url;

fn linked_graph_for(input: &str) -> (sysml_model::SemanticGraph, Url) {
    let document = SysmlDocument::from_memory_path(
        "workspace",
        "p1.sysml",
        input.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document");
    let uri = document.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");
    (graph, uri)
}

#[test]
fn specialization_query_is_transitive_and_cycle_safe() {
    let (graph, uri) = linked_graph_for(
        r#"
            package P {
                part def Base;
                part def Middle :> Base;
                part def Leaf :> Middle;
                part def CycleA :> CycleB;
                part def CycleB :> CycleA;
            }
        "#,
    );
    let base = graph
        .get_node(&NodeId::new(&uri, "P::Base"))
        .expect("base node");
    let leaf = graph
        .get_node(&NodeId::new(&uri, "P::Leaf"))
        .expect("leaf node");
    let cycle_a = graph
        .get_node(&NodeId::new(&uri, "P::CycleA"))
        .expect("cycle node");

    assert!(graph.specializes_transitively(leaf, base));
    assert!(!graph.specializes_transitively(cycle_a, base));
}
