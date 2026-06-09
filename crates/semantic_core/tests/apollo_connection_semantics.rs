use semantic_core::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph,
    resolve_expression_endpoint_strict, resolve_workspace_pending_relationships,
    DiagnosticsOptions, ResolveResult,
};
use url::Url;

fn workspace_doc(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "apollo",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("doc")
}

#[test]
fn apollo_style_interface_connect_resolves_individual_and_inherited_features() {
    let components = workspace_doc(
        "TechnicalComponentsPackage.sysml",
        r#"package TechnicalComponentsPackage {
  port def PayloadInterfacePort;
  part def SaturnVInstrumentUnit {
    port payloadInterfacePort : PayloadInterfacePort;
  }
  part def SaturnV {
    part instrumentUnit : SaturnVInstrumentUnit;
  }
  part def ApolloSpacecraftLMAdapter {
    port launchVehicleInterfacePort : PayloadInterfacePort;
  }
  part def ApolloSpacecraft {
    part spacecraftLMAdapter : ApolloSpacecraftLMAdapter;
  }
}"#,
    );
    let individuals = workspace_doc(
        "TechnicalIndividualsPackage.sysml",
        r#"package TechnicalIndividualsPackage {
  private import TechnicalComponentsPackage::*;
  individual part def 'SA-506' :> SaturnV { }
}"#,
    );
    let system = workspace_doc(
        "SystemPackage.sysml",
        r#"package SystemPackage {
  private import TechnicalComponentsPackage::*;
  private import TechnicalIndividualsPackage::*;
  part def Apollo11MissionSystem {
    individual part launchVehicle : 'SA-506';
    part spacecraft : ApolloSpacecraft;
    interface lvToPayload connect
      launchVehicle.instrumentUnit.payloadInterfacePort to spacecraft.spacecraftLMAdapter.launchVehicleInterfacePort;
  }
}"#,
    );

    let (mut graph, _parsed) =
        build_semantic_graph_from_documents(&[components, individuals, system]).expect("graph");
    resolve_workspace_pending_relationships(&mut graph);

    let uri = Url::parse("memory://apollo/SystemPackage.sysml").expect("uri");
    let prefix = "SystemPackage::Apollo11MissionSystem";
    let source = resolve_expression_endpoint_strict(
        &graph,
        &uri,
        Some(prefix),
        "launchVehicle.instrumentUnit.payloadInterfacePort",
    );
    let target = resolve_expression_endpoint_strict(
        &graph,
        &uri,
        Some(prefix),
        "spacecraft.spacecraftLMAdapter.launchVehicleInterfacePort",
    );
    assert!(
        matches!(source, ResolveResult::Resolved(_)),
        "expected source endpoint to resolve, got {source:?}"
    );
    assert!(
        matches!(target, ResolveResult::Resolved(_)),
        "expected target endpoint to resolve, got {target:?}"
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !diagnostics
            .iter()
            .any(|d| d.code == "unresolved_pending_expression_relationship"),
        "unexpected pending connection diagnostics: {diagnostics:?}"
    );
}

#[test]
fn homonymous_imported_port_defs_resolve_to_local_port_under_container() {
    let doc = workspace_doc(
        "homonym_ports.sysml",
        r#"package PortPkgA {
  port def homonym;
}
package PortPkgB {
  port def homonym;
}
package Use {
  private import PortPkgA::*;
  private import PortPkgB::*;
  part def Robot {
    port homonym : PortPkgA::homonym;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let uri = Url::parse("memory://apollo/homonym_ports.sysml").expect("uri");
    match resolve_expression_endpoint_strict(&graph, &uri, Some("Use::Robot"), "homonym") {
        ResolveResult::Resolved(id) => {
            assert!(
                id.qualified_name.ends_with("::Robot::homonym"),
                "expected local port under container, got {}",
                id.qualified_name
            );
        }
        other => panic!("expected resolved local homonym port, got {other:?}"),
    }
}
