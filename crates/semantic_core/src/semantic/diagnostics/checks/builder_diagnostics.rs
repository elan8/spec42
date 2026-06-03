use url::Url;

use crate::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult, SemanticGraph,
    SemanticNode,
};
use crate::semantic::reference_resolution::resolve_expression_endpoint_workspace;

pub(crate) fn should_suppress_builder_diagnostic(
    graph: &SemanticGraph,
    uri: &Url,
    node: &SemanticNode,
    code: &str,
    message: &str,
) -> bool {
    if !matches!(
        code,
        "unresolved_satisfy_source"
            | "unresolved_satisfy_target"
            | "unresolved_viewpoint_conformance_target"
            | "unresolved_allocate_source"
            | "unresolved_allocate_target"
    ) {
        return false;
    }
    let Some(reference_name) = extract_single_quoted_value(message) else {
        return false;
    };
    if endpoint_reference_resolves(
        graph,
        uri,
        Some(diagnostic_container_prefix(node)),
        &reference_name,
    ) {
        return true;
    }
    endpoint_reference_resolves(graph, uri, None, &reference_name)
}

fn extract_single_quoted_value(message: &str) -> Option<String> {
    let start = message.find('\'')?;
    let rest = &message[start + 1..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

fn diagnostic_container_prefix(node: &SemanticNode) -> &str {
    node.id
        .qualified_name
        .rsplit_once("::")
        .map(|(prefix, _)| prefix)
        .unwrap_or("")
}

fn endpoint_reference_resolves(
    graph: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    reference_name: &str,
) -> bool {
    if let ResolveResult::Resolved(_) =
        resolve_expression_endpoint_strict(graph, uri, container_prefix, reference_name)
    {
        return true;
    }

    let normalized = reference_name.replace('.', "::");
    let segments: Vec<&str> = normalized
        .split("::")
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.len() <= 1 {
        return matches!(
            resolve_expression_endpoint_workspace(graph, reference_name),
            ResolveResult::Resolved(_)
        );
    }
    let owner_expr = segments[0];
    let ResolveResult::Resolved(mut current_id) =
        resolve_expression_endpoint_strict(graph, uri, container_prefix, owner_expr)
    else {
        return false;
    };
    for member in segments.iter().skip(1) {
        let Some(owner) = graph.get_node(&current_id) else {
            return false;
        };
        let ResolveResult::Resolved(next_id) = resolve_member_via_type(graph, owner, member) else {
            return false;
        };
        current_id = next_id;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::collect_diagnostics_from_graph;
    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
    use crate::semantic::workspace_graph::build_semantic_graph_from_documents;
    use crate::DiagnosticsOptions;

    #[test]
    fn suppresses_unresolved_allocate_source_for_imported_part_def() {
        let architecture = SysmlDocument::from_memory_path(
            "workspace",
            "WebShopArchitecture.sysml",
            r#"package WebShopArchitecture {
                part def CheckoutService;
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
                part commerceCluster;
                allocate CheckoutService to commerceCluster;
            }"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("example doc");
        let (graph, _parsed) =
            build_semantic_graph_from_documents(&[architecture, example.clone()]).expect("graph");
        let diagnostics = collect_diagnostics_from_graph(
            &graph,
            &example.uri,
            DiagnosticsOptions::default(),
        );
        assert!(
            !diagnostics.iter().any(|d| d.code == "unresolved_allocate_source"),
            "unexpected unresolved_allocate_source: {:?}",
            diagnostics
                .iter()
                .filter(|d| d.code == "unresolved_allocate_source")
                .collect::<Vec<_>>()
        );
    }

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
        let (graph, _parsed) = build_semantic_graph_from_documents(&[
            architecture,
            requirements,
            example.clone(),
        ])
        .expect("graph");
        let diagnostics = collect_diagnostics_from_graph(
            &graph,
            &example.uri,
            DiagnosticsOptions::default(),
        );
        assert!(
            !diagnostics.iter().any(|d| d.code == "unresolved_satisfy_target"),
            "unexpected unresolved_satisfy_target: {:?}",
            diagnostics
                .iter()
                .filter(|d| d.code == "unresolved_satisfy_target")
                .collect::<Vec<_>>()
        );
    }
}
