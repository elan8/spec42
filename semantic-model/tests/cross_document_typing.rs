//! Integration: merged workspace graph resolves typing across documents.

use semantic_model::{
    add_cross_document_edges_for_uri, build_graph_from_doc, RelationshipKind, SemanticGraph,
};
use sysml_parser::parse;
use tower_lsp::lsp_types::Url;

#[test]
fn cross_document_requirement_usage_typing_after_merge() {
    let defs = r#"
        package R {
            requirement def EnduranceReq;
        }
    "#;
    let usage = r#"
        package R {
            requirement enduranceCheck : EnduranceReq;
        }
    "#;
    let root_defs = parse(defs).expect("parse defs");
    let root_usage = parse(usage).expect("parse usage");
    let uri_defs = Url::parse("file:///requirements_defs.sysml").expect("uri defs");
    let uri_usage = Url::parse("file:///requirements_usage.sysml").expect("uri usage");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_defs, &uri_defs));
    g.merge(build_graph_from_doc(&root_usage, &uri_usage));

    add_cross_document_edges_for_uri(&mut g, &uri_usage);

    let edges = g.edges_for_uri_as_strings(&uri_usage);
    let has_typing = edges.iter().any(|(src, tgt, kind, _)| {
        *kind == RelationshipKind::Typing
            && src.ends_with("enduranceCheck")
            && tgt.ends_with("EnduranceReq")
    });
    assert!(
        has_typing,
        "expected cross-document typing edge enduranceCheck -> EnduranceReq; edges: {:?}",
        edges
    );
}
