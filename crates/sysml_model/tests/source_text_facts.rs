//! Regression coverage for `SourceTextFacts` (B9 source-fidelity/documentation-text chunk):
//! documentation and representation text must survive graph construction as a typed fact on
//! `SemanticNode::source_text`, independent of the legacy untyped `attributes` map.

use sysml_model::{build_graph_from_doc, SemanticGraph};
use sysml_v2_parser::parse;
use url::Url;

fn graph_node<'a>(
    graph: &'a SemanticGraph,
    uri: &Url,
    kind: &str,
    name: &str,
) -> &'a sysml_model::SemanticNode {
    graph
        .nodes_for_uri(uri)
        .into_iter()
        .find(|node| node.element_kind == kind && node.name == name)
        .unwrap_or_else(|| panic!("expected {kind} node named {name}"))
}

#[test]
fn multiline_doc_comment_survives_as_typed_source_text_fact() {
    let input = r#"package P {
  part def Widget {
    doc /* First line of documentation.
    Second line of documentation. */
  }
}"#;
    let root = parse(input).expect("parse");
    let uri = Url::parse("file:///widget.sysml").expect("uri");
    let graph = build_graph_from_doc(&root, &uri);
    let widget = graph_node(&graph, &uri, "part def", "Widget");

    let doc = widget
        .source_text
        .doc
        .as_deref()
        .expect("typed doc fact present");
    assert!(
        doc.contains("First line of documentation.")
            && doc.contains("Second line of documentation."),
        "typed source_text.doc should retain the full multi-line comment: {doc:?}"
    );
}

#[test]
fn multiple_doc_comments_join_in_typed_source_text_fact() {
    let input = r#"package P {
  part def Widget {
    doc /* First doc block. */
    doc /* Second doc block. */
  }
}"#;
    let root = parse(input).expect("parse");
    let uri = Url::parse("file:///widget2.sysml").expect("uri");
    let graph = build_graph_from_doc(&root, &uri);
    let widget = graph_node(&graph, &uri, "part def", "Widget");

    let doc = widget
        .source_text
        .doc
        .as_deref()
        .expect("typed doc fact present");
    assert!(
        doc.contains("First doc block.") && doc.contains("Second doc block."),
        "typed source_text.doc should join multiple doc blocks: {doc:?}"
    );
}

#[test]
fn textual_representation_language_and_text_survive_as_typed_source_text_fact() {
    let input = r#"package P {
  requirement def Safety {
    rep note language "text/markdown" /* Keep the speed under the limit. */
  }
}"#;
    let root = parse(input).expect("parse");
    let uri = Url::parse("file:///safety.sysml").expect("uri");
    let graph = build_graph_from_doc(&root, &uri);
    let rep = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.id.qualified_name == "P::Safety::note")
        .expect("named requirement-body textual representation node");

    assert_eq!(
        rep.source_text.language.as_deref(),
        Some("text/markdown"),
        "typed source_text.language should retain the declared language tag"
    );
    let text = rep
        .source_text
        .text
        .as_deref()
        .expect("typed text fact present");
    assert!(
        text.contains("Keep the speed under the limit."),
        "typed source_text.text should retain the representation content: {text:?}"
    );
}
