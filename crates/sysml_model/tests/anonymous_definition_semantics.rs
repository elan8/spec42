//! Anonymous definitions get kind-tagged synthetic names and stay addressable (#32).

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
fn anonymous_item_def_materializes_with_kind_tagged_name_and_nested_member() {
    let doc = workspace_doc(
        "anon_item.sysml",
        r#"package P {
  item def {
    attribute id : String;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let item_def = graph
        .nodes_named("_itemDef")
        .into_iter()
        .find(|node| node.element_kind == "item def")
        .expect("anonymous item def");
    assert_eq!(item_def.id.qualified_name, "P::_itemDef");
    assert_eq!(
        item_def.attributes.get("isAnonymous"),
        Some(&serde_json::json!(true))
    );

    let child = graph
        .children_of(item_def)
        .into_iter()
        .find(|node| node.name == "id")
        .expect("nested attribute on anonymous item def");
    assert_eq!(child.parent_id.as_ref(), Some(&item_def.id));
    assert!(
        child.id.qualified_name.starts_with("P::_itemDef::"),
        "nested member should inherit synthetic prefix, got {}",
        child.id.qualified_name
    );
}

#[test]
fn sibling_anonymous_item_defs_get_distinct_qualified_names() {
    let doc = workspace_doc(
        "anon_siblings.sysml",
        r#"package P {
  item def { attribute a : String; }
  item def { attribute b : String; }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let mut anon_items: Vec<_> = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .filter(|n| {
            n.element_kind == "item def"
                && n.attributes.get("isAnonymous") == Some(&serde_json::json!(true))
        })
        .collect();
    assert_eq!(anon_items.len(), 2, "expected two anonymous item defs");
    anon_items.sort_by(|a, b| a.id.qualified_name.cmp(&b.id.qualified_name));

    assert_eq!(anon_items[0].id.qualified_name, "P::_itemDef");
    assert!(
        anon_items[1].id.qualified_name.starts_with("P::_itemDef#"),
        "second sibling should use #kind disambiguation, got {}",
        anon_items[1].id.qualified_name
    );

    for item in &anon_items {
        assert!(
            !graph.children_of(item).is_empty(),
            "each anonymous def should keep nested members under {}",
            item.id.qualified_name
        );
    }
}

#[test]
fn anonymous_constraint_def_and_calc_def_stay_addressable() {
    let doc = workspace_doc(
        "anon_constraint_calc.sysml",
        r#"package P {
  constraint def {
    doc /* empty body */
  }
  calc def {
    doc /* empty body */
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let constraint_def = graph
        .nodes_named("_constraintDef")
        .into_iter()
        .find(|n| n.element_kind == "constraint def")
        .expect("anonymous constraint def");
    assert_eq!(
        constraint_def.attributes.get("isAnonymous"),
        Some(&serde_json::json!(true))
    );

    let calc_def = graph
        .nodes_named("_calcDef")
        .into_iter()
        .find(|n| n.element_kind == "calc def")
        .expect("anonymous calc def");
    assert_eq!(
        calc_def.attributes.get("isAnonymous"),
        Some(&serde_json::json!(true))
    );
}
