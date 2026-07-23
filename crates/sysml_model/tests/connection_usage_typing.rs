use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

#[test]
fn cross_document_connection_usage_resolves_redefined_features_from_its_type() {
    let contract = SysmlDocument::from_memory_path(
        "connection-contract",
        "Contract.sysml",
        r#"
        package InterfaceControl {
            connection def DataFlowContract {
                attribute producerFeature : String;
                attribute consumerFeature : String;
            }
        }
        "#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("contract document");
    let usage = SysmlDocument::from_memory_path(
        "connection-usage",
        "Usage.sysml",
        r#"
        package FirmwareArchitecture {
            private import InterfaceControl::*;
            connection lidarScanToSlamContract : DataFlowContract {
                attribute :>> producerFeature = "lidar.scan";
                attribute :>> consumerFeature = "slam.scan";
            }
        }
        "#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("usage document");
    let usage_uri = usage.uri.clone();

    let (graph, _) =
        build_semantic_graph_from_documents(&[contract, usage]).expect("semantic graph");
    let diagnostics =
        collect_diagnostics_from_graph(&graph, &usage_uri, DiagnosticsOptions::default());

    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "unresolved_redefines_target"),
        "cross-document connection typing must resolve inherited features: {diagnostics:?}"
    );
    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "incompatible_type_kind"),
        "connection usage must be compatible with connection def: {diagnostics:?}"
    );
}
