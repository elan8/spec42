use sysml_model::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
use sysml_model::{
    build_semantic_graph_from_documents, resolve_expression_endpoint_strict, ResolveResult,
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
