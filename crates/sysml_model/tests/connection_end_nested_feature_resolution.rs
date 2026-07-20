//! Regression coverage for S42-LIM-011: connection usage ends redefined via `::>` to a nested
//! feature path (e.g. `sensor.reading`) should resolve the same way `flow` statement endpoints
//! already do, instead of emitting `unresolved_type_reference`.

use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind};

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
fn connection_end_redefined_to_nested_feature_path_resolves_without_diagnostics() {
    let doc = workspace_doc(
        "robot_contract.sysml",
        r#"package Arch {
  part def Sensor {
    attribute reading : Real;
  }
  part def Consumer {
    attribute reading : Real;
  }

  connection def Contract {
    end producer;
    end consumer;
  }

  part def Robot {
    part sensor : Sensor;
    part consumer : Consumer;

    flow sensor.reading to consumer.reading;

    connection c : Contract {
      end producer ::> sensor.reading;
      end consumer ::> consumer.reading;
    }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let unresolved: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.code == "unresolved_type_reference")
        .filter(|d| d.message.contains("'producer'") || d.message.contains("'consumer'"))
        .collect();
    assert!(
        unresolved.is_empty(),
        "connection ends redefined to nested feature paths should not be unresolved: {unresolved:#?}"
    );

    let sensor_reading = graph
        .nodes_named("reading")
        .into_iter()
        .find(|node| node.id.qualified_name == "Arch::Sensor::reading")
        .expect("Sensor.reading node");
    let consumer_reading = graph
        .nodes_named("reading")
        .into_iter()
        .find(|node| node.id.qualified_name == "Arch::Consumer::reading")
        .expect("Consumer.reading node");

    let connection_targets =
        graph.outgoing_targets_by_kind(sensor_reading, RelationshipKind::Connection);
    assert!(
        connection_targets
            .iter()
            .any(|target| target.id == consumer_reading.id),
        "expected a semantic connection edge from sensor.reading to consumer.reading, got: {connection_targets:#?}"
    );
}
