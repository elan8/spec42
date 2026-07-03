//! Several AST usage constructs (`part`, `attribute`, `occurrence`, `requirement`) can legally
//! appear in more than one containing body: directly in a package body, inside a
//! `part def { ... }` body, inside a `part <usage> { ... }` body, and — for `occurrence` and
//! `requirement` — inside nested occurrence/state bodies too. Each containing body previously
//! had independently hand-written graph-builder logic that drifted apart in several ways:
//! - `part_def.rs`'s copy of `PartUsage` silently dropped the `usagePrefix` attribute the
//!   other two copies set.
//! - Def-kind match arms across the graph builder inconsistently wired the `Specializes` edge
//!   either before or after recursing into the def's body, which broke inherited-member
//!   resolution (e.g. `attribute redefines <inheritedPort>`) for whichever arms wired it last.
//! - `part_usage.rs`'s copy of `OccurrenceUsage` never recursed into the usage's own body at
//!   all, silently dropping every nested child of an `occurrence { ... }` declared inside a
//!   `part` usage.
//! - `package_body.rs`'s copy of `RequirementUsage` (top-level package body) never read the
//!   `subsets` field, silently dropping the `subsetsFeature` attribute the `part def`/state-body
//!   copies set for `requirement ... :> other;`.
//!
//! These tests pin the contract that the same syntax produces the same node attributes/children
//! regardless of which body it is nested in, so a future regression in one context/copy is
//! caught immediately.

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
fn attribute_redefining_a_port_is_classified_as_port_in_every_containing_context() {
    // `materialize_attribute_usage` classifies `attribute redefines <port>` as a port by
    // resolving the redefined member through the immediately containing usage's/def's type.
    // This requires the container's typing/specializes edge to already be wired before its
    // body is walked, which is why `wire_def_specialization_edge`/`add_specializes_edge_if_exists`
    // must run *before* the body-element recursion loop in every def-kind match arm (previously
    // several arms wired it after, so this inference silently failed to resolve members declared
    // via inheritance inside those bodies).
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
        (
            "inside_part_def.sysml",
            r#"package P {
  port def PD;
  part def Base {
    port p : PD;
  }
  part def Foo :> Base {
    attribute redefines p;
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

#[test]
fn occurrence_usage_body_recurses_into_children_in_every_containing_context() {
    let contexts = [
        (
            "top_level.sysml",
            r#"package P { occurrence x { attribute id : String; } }"#,
        ),
        (
            "inside_part_def.sysml",
            r#"package P { part def Foo { occurrence x { attribute id : String; } } }"#,
        ),
        (
            "inside_part_usage.sysml",
            r#"package P {
  part def Bar;
  part y : Bar {
    occurrence x {
      attribute id : String;
    }
  }
}"#,
        ),
        (
            "inside_occurrence.sysml",
            r#"package P { occurrence outer { occurrence x { attribute id : String; } } }"#,
        ),
    ];

    for (path, src) in contexts {
        let doc = workspace_doc(path, src);
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let node = graph
            .nodes_named("x")
            .into_iter()
            .find(|node| node.element_kind == "occurrence")
            .unwrap_or_else(|| panic!("expected occurrence usage `x` to exist in context {path}"));
        let has_child_attribute = graph
            .children_of(node)
            .into_iter()
            .any(|child| child.element_kind == "attribute" && child.name == "id");
        assert!(
            has_child_attribute,
            "expected occurrence `x` to have a nested attribute `id` in context {path}, got children {:?}",
            graph
                .children_of(node)
                .iter()
                .map(|c| &c.name)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn requirement_usage_subsets_feature_is_preserved_in_every_containing_context() {
    let contexts = [
        (
            "top_level.sysml",
            r#"package P {
  requirement def Req1;
  requirement def Req2;
  requirement r1 : Req1;
  requirement r2 : Req2 :> r1;
}"#,
        ),
        (
            "inside_part_def.sysml",
            r#"package P {
  requirement def Req1;
  requirement def Req2;
  requirement r1 : Req1;
  part def Foo {
    requirement r2 : Req2 :> r1;
  }
}"#,
        ),
        (
            "inside_state.sysml",
            r#"package P {
  requirement def Req1;
  requirement def Req2;
  requirement r1 : Req1;
  state def Machine {
    requirement r2 : Req2 :> r1;
  }
}"#,
        ),
    ];

    for (path, src) in contexts {
        let doc = workspace_doc(path, src);
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let node = graph
            .nodes_named("r2")
            .into_iter()
            .find(|node| node.element_kind == "requirement")
            .unwrap_or_else(|| panic!("expected requirement usage `r2` to exist in context {path}"));
        assert_eq!(
            node.attributes.get("subsetsFeature").and_then(|v| v.as_str()),
            Some("r1"),
            "expected `r2` to keep subsetsFeature=\"r1\" in context {path}, got {:#?}",
            node.attributes
        );
    }
}
