#[cfg(test)]
mod tests {
    use crate::collect_diagnostics_from_graph;
    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
    use crate::semantic::workspace_graph::build_semantic_graph_from_documents;
    use crate::DiagnosticsOptions;

    #[test]
    fn suppresses_unresolved_satisfy_target_for_typed_part_member() {
        let architecture = SysmlDocument::from_memory_path(
            "workspace",
            "WebShopArchitecture.sysml",
            r#"package WebShopArchitecture {
                part def CheckoutService;
                part def WebShopSystem {
                    part checkoutService : CheckoutService;
                }
            }"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("architecture doc");
        let example = SysmlDocument::from_memory_path(
            "workspace",
            "webshop.sysml",
            r#"package WebShopExample {
                import WebShopArchitecture::*;
                import WebShopRequirements::*;
                part webshopSystem : WebShopSystem;
                requirement checkoutLatency : CheckoutLatencyReq;
                satisfy checkoutLatency by webshopSystem.checkoutService;
            }"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("example doc");
        let requirements = SysmlDocument::from_memory_path(
            "workspace",
            "WebShopRequirements.sysml",
            r#"package WebShopRequirements {
                requirement def CheckoutLatencyReq;
            }"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("requirements doc");
        let (graph, _parsed) =
            build_semantic_graph_from_documents(&[architecture, requirements, example.clone()])
                .expect("graph");
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &example.uri, DiagnosticsOptions::default());
        assert!(
            !diagnostics
                .iter()
                .any(|d| d.code == "unresolved_satisfy_target"),
            "unexpected unresolved_satisfy_target: {:?}",
            diagnostics
                .iter()
                .filter(|d| d.code == "unresolved_satisfy_target")
                .collect::<Vec<_>>()
        );
    }
}
