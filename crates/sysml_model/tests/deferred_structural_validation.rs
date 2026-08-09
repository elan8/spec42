//! Deferred structural-validation regressions from the compatibility corpus.
//!
//! These regressions stay visible until the graph contains the exact facts needed
//! for their structural checks.  Active cases assert parser-backed graph facts;
//! unsupported closures remain explicit skips.

use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, ElementKind, SysmlDocument, SysmlDocumentSourceKind,
};

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
fn binary_connector_end_cardinality_fixture() {
    let source = r#"
package P {
  occurrence def Occurrence;
  connection def Incomplete {
    end feature source : Occurrence[1];
  }
}
"#;
    let doc = workspace_doc("incomplete.sysml", source);
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let connection = graph
        .nodes_named("Incomplete")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::ConnectionDef)
        .expect("connection definition");

    let ends = graph.positional_end_features(connection);
    assert_eq!(ends.len(), 1, "the graph must not fabricate a missing end");
    assert_eq!(ends[0].name, "source");
    assert!(
        ends[0]
            .declared_facts
            .feature_properties
            .as_ref()
            .is_some_and(|properties| properties.is_end),
        "the parsed `end` declaration must be an explicit end-feature fact"
    );
    assert!(
        ends[0]
            .declared_facts
            .multiplicity
            .as_ref()
            .is_some_and(|fact| !fact.is_implied),
        "an authored end multiplicity must remain an authored fact"
    );
    assert!(
        collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default())
            .iter()
            .any(|diagnostic| diagnostic.code == "incomplete_connection_like_end_pair"),
        "the incomplete end pair must be reported from the positional graph fact"
    );
}

#[test]
fn connection_like_definition_ends_share_one_positional_graph_fact() {
    let doc = workspace_doc(
        "connection_like_ends.sysml",
        r#"package P {
  occurrence def Occurrence;
  connection def Connector {
    end feature source : Occurrence;
    end feature target : Occurrence;
  }
  flow def Transfer {
    end feature source : Occurrence;
    end feature target : Occurrence;
  }
  allocation def Allocation {
    end feature source : Occurrence;
    end feature target : Occurrence;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    for (name, kind) in [
        ("Connector", ElementKind::ConnectionDef),
        ("Transfer", ElementKind::FlowDef),
        ("Allocation", ElementKind::AllocationDef),
    ] {
        let owner = graph
            .nodes_named(name)
            .into_iter()
            .find(|node| node.element_kind == kind)
            .unwrap_or_else(|| panic!("missing {name} {kind}"));
        let names: Vec<_> = graph
            .positional_end_features(owner)
            .into_iter()
            .map(|end| end.name.as_str())
            .collect();
        assert_eq!(names, ["source", "target"], "unexpected ends for {name}");
    }
    assert!(
        !collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default())
            .iter()
            .any(|diagnostic| diagnostic.code == "incomplete_connection_like_end_pair"),
        "complete connection-like definitions must not receive the incomplete-pair diagnostic"
    );
}

#[test]
fn binary_flow_and_allocation_definitions_reject_excess_authored_ends() {
    let doc = workspace_doc(
        "overfull_binary_ends.sysml",
        r#"package P {
  occurrence def Occurrence;
  flow def OverfullFlow {
    end feature source : Occurrence;
    end feature target : Occurrence;
    end feature extra : Occurrence;
  }
  allocation def OverfullAllocation {
    end feature source : Occurrence;
    end feature target : Occurrence;
    end feature extra : Occurrence;
  }
  connection def NaryConnection {
    end feature first : Occurrence;
    end feature second : Occurrence;
    end feature third : Occurrence;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    for name in ["OverfullFlow", "OverfullAllocation", "NaryConnection"] {
        let owner = graph
            .nodes_named(name)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("missing {name}"));
        assert_eq!(
            graph.positional_end_features(owner).len(),
            3,
            "the graph must preserve all authored ends for {name}"
        );
    }

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let count = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "invalid_binary_connection_like_end_count")
        .count();
    assert_eq!(count, 2, "only flow and allocation definitions are binary");
    assert!(diagnostics.iter().all(|diagnostic| {
        diagnostic.code != "invalid_binary_connection_like_end_count"
            || !diagnostic.message.contains("NaryConnection")
    }));
}

#[test]
#[ignore = "SKIP S42-COMPAT-STRUCT-002: FlowPayload typing is represented, but no graph-backed payload-occurrence conformance rule is defined"]
fn skip_flow_payload_occurrence_type_fixture() {
    let source = r#"
package P {
  attribute def Scalar;
  part def Source;
  part def Target;
  flow transfer of Scalar from Source to Target;
}
"#;
    panic!("SKIP: retain fixture until payload typing supports a real conformance rule: {source}");
}

#[test]
#[ignore = "SKIP S42-COMPAT-STRUCT-003: graph has ownership but no complete featuring-type closure for the KerML overlap constraint"]
fn skip_redefinition_featuring_type_overlap_fixture() {
    let source = r#"
package P {
  part def Vehicle { attribute mass : Real; }
  part def Car :> Vehicle { attribute mass :>> mass; }
}
"#;
    panic!("SKIP: retain fixture until featuring-type closure is graph-backed: {source}");
}
