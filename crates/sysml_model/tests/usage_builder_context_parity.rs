//! `part` and `attribute` usages can legally appear in three different containing bodies:
//! directly in a package body, inside a `part def { ... }` body, and inside a `part <usage>
//! { ... }` body. All three previously had independently hand-written graph-builder logic in
//! `package_body.rs`, `part_def.rs`, and `part_usage.rs`; they drifted apart (`part_def.rs`'s
//! copy silently dropped the `usagePrefix` attribute the other two set). These tests pin the
//! contract that the same usage syntax produces the same node attributes regardless of which
//! body it is nested in, so a future regression in one context/copy is caught immediately.

use sysml_model::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

fn workspace_doc(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "workspace",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

#[test]
fn abstract_part_usage_prefix_is_preserved_in_every_containing_context() {
    let contexts = [
        (
            "top_level.sysml",
            r#"package P {
  part def Bar;
  abstract part y : Bar;
}"#,
        ),
        (
            "inside_part_def.sysml",
            r#"package P {
  part def Bar;
  part def Foo {
    abstract part y : Bar;
  }
}"#,
        ),
        (
            "inside_part_usage.sysml",
            r#"package P {
  part def Bar;
  part def Bar2;
  part x : Bar2 {
    abstract part y : Bar;
  }
}"#,
        ),
    ];

    for (path, src) in contexts {
        let doc = workspace_doc(path, src);
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let node = graph
            .nodes_named("y")
            .into_iter()
            .find(|node| node.element_kind == "part")
            .unwrap_or_else(|| panic!("expected part usage `y` to exist in context {path}"));
        assert_eq!(
            node.attributes.get("usagePrefix").and_then(|v| v.as_str()),
            Some("abstract"),
            "expected `y` to keep usagePrefix=\"abstract\" in context {path}, got {:#?}",
            node.attributes
        );
    }
}

#[test]
fn attribute_redefining_a_port_is_classified_as_port_in_every_part_usage_containing_context() {
    // `materialize_attribute_usage` classifies `attribute redefines <port>` as a port by
    // resolving the redefined member through the immediately containing usage's type. That
    // container's typing edge must already be wired before its body is walked — true for
    // `part` usages (top-level or nested) which this test covers. `part def` bodies wire their
    // specializes edge *after* walking the body, so the same inference does not (yet) resolve
    // there; that is a separate, pre-existing edge-ordering gap outside this refactor's scope.
    let contexts = [
        (
            "top_level_part_usage.sysml",
            r#"package P {
  port def PD;
  part def Base {
    port p : PD;
  }
  part y : Base {
    attribute redefines p;
  }
}"#,
        ),
        (
            "nested_part_usage.sysml",
            r#"package P {
  port def PD;
  part def Base {
    port p : PD;
  }
  part def Baz;
  part x : Baz {
    part y : Base {
      attribute redefines p;
    }
  }
}"#,
        ),
    ];

    for (path, src) in contexts {
        let doc = workspace_doc(path, src);
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let node = graph
            .nodes_named("p")
            .into_iter()
            .find(|node| node.attributes.get("redefines").and_then(|v| v.as_str()) == Some("p"));
        let node = node.unwrap_or_else(|| {
            panic!("expected an attribute usage redefining `p` to exist in context {path}")
        });
        assert_eq!(
            node.element_kind, "port",
            "expected `attribute redefines p` to be classified as a port in context {path}, got {:?}",
            node.element_kind
        );
    }
}
