use semantic_core::{
    build_semantic_graph_from_documents, build_view_catalog, RelationshipKind, SysmlDocument,
    SysmlDocumentSourceKind,
};
use url::Url;

const MVP_ROUNDTRIP_SYSML: &str = r#"
package Demo {
  interface def PowerInterface;

  connection def PowerConnection {
    connect Battery.powerOut to Controller.powerIn;
  }

  part def Battery {
    port powerOut;
  }

  part def Controller {
    port powerIn;
  }

  part def System {
    part battery : Battery;
    part controller : Controller;
    connect battery.powerOut to controller.powerIn;
  }

  requirement def BatteryRuntime;
  satisfy Battery by BatteryRuntime;

  verification def VerifyRuntime {
    objective {
      verify BatteryRuntime;
    }
  }

  state def Operating {
    state off;
    state on;
    transition power_up first off then on;
  }

  view def PhysicalView;
  view physical : PhysicalView {
    expose Battery;
  }
}
"#;

fn workspace_fixture() -> (Url, Vec<SysmlDocument>) {
    let doc = SysmlDocument::from_memory_path(
        "mvp",
        "demo.sysml",
        MVP_ROUNDTRIP_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    (doc.uri.clone(), vec![doc])
}

#[test]
fn mvp_roundtrip_subset_has_authoritative_graph_facts() {
    let (_uri, documents) = workspace_fixture();
    let (graph, _parsed) = build_semantic_graph_from_documents(&documents).expect("semantic graph");

    let node_kind = |name: &str| {
        graph
            .nodes_named(name)
            .into_iter()
            .map(|node| node.element_kind.as_str())
            .collect::<Vec<_>>()
    };

    assert!(node_kind("PowerInterface").contains(&"interface"));
    assert!(node_kind("PowerConnection").contains(&"connection def"));
    assert!(node_kind("Battery").contains(&"part def"));
    assert!(node_kind("powerOut").contains(&"port"));
    assert!(node_kind("BatteryRuntime").contains(&"requirement def"));

    let edges = graph.edges_for_workspace_as_strings(&[]);
    let has_edge = |source: &str, target: &str, kind: RelationshipKind| {
        edges
            .iter()
            .any(|(src, tgt, edge_kind, _)| src == source && tgt == target && *edge_kind == kind)
    };

    let connection_edges: Vec<_> = edges
        .iter()
        .filter(|(_, _, kind, _)| *kind == RelationshipKind::Connection)
        .collect();
    assert!(
        connection_edges.iter().any(|(src, tgt, _, _)| {
            src.ends_with("Battery::powerOut") && tgt.ends_with("Controller::powerIn")
        }),
        "expected connection between battery and controller ports, got: {:?}",
        connection_edges
    );
    let connect_edges = graph.connect_statement_edges_for_uri(&documents[0].uri);
    assert!(
        connect_edges.iter().any(|(_, _, connect)| {
            connect.source_expression == "battery::powerOut"
                && connect.target_expression == "controller::powerIn"
        }),
        "expected part-body connect to record usage-relative endpoints, got: {:?}",
        connect_edges
    );
    assert!(has_edge(
        "Demo::Battery",
        "Demo::BatteryRuntime",
        RelationshipKind::Satisfy
    ));
    assert!(has_edge(
        "Demo::VerifyRuntime",
        "Demo::BatteryRuntime",
        RelationshipKind::Subject
    ));
    assert!(has_edge(
        "Demo::Operating::off",
        "Demo::Operating::on",
        RelationshipKind::Transition
    ));
}

#[test]
fn mvp_roundtrip_subset_keeps_views_in_explicit_catalog() {
    let (uri, documents) = workspace_fixture();
    let (_graph, parsed) = build_semantic_graph_from_documents(&documents).expect("semantic graph");

    let catalog = build_view_catalog(&[uri], &parsed);

    let definition = catalog
        .definitions
        .get("Demo::PhysicalView")
        .expect("physical view definition");
    assert_eq!(catalog.definitions.len(), 1);
    assert_eq!(definition.name, "PhysicalView");
    assert_eq!(catalog.usages.len(), 1);
    assert_eq!(catalog.usages[0].name, "physical");
    assert_eq!(
        catalog.usages[0].definition_ref.as_deref(),
        Some("PhysicalView")
    );
    assert_eq!(catalog.usages[0].exposes.len(), 1);
}
