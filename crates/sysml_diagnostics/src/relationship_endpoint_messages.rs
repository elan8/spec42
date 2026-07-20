//! Actionable messages for allocate/satisfy builder diagnostics.

use url::Url;

use sysml_model::semantic::reference_resolution::resolve_expression_endpoint_workspace;
use sysml_model::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult, SemanticGraph,
    SemanticNode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EndpointRole {
    Source,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RelationshipKindHint {
    Allocate,
    Satisfy,
    ViewpointConformance,
}

#[derive(Debug, Clone)]
enum EndpointResolveOutcome {
    UsageLike,
    Definitional(Box<SemanticNode>),
    Unresolved,
}

fn relationship_kind_from_code(code: &str) -> Option<(RelationshipKindHint, EndpointRole)> {
    match code {
        "unresolved_allocate_source" => {
            Some((RelationshipKindHint::Allocate, EndpointRole::Source))
        }
        "unresolved_allocate_target" => {
            Some((RelationshipKindHint::Allocate, EndpointRole::Target))
        }
        "unresolved_satisfy_source" => Some((RelationshipKindHint::Satisfy, EndpointRole::Source)),
        "unresolved_satisfy_target" => Some((RelationshipKindHint::Satisfy, EndpointRole::Target)),
        "unresolved_viewpoint_conformance_target" => Some((
            RelationshipKindHint::ViewpointConformance,
            EndpointRole::Target,
        )),
        _ => None,
    }
}

fn is_definitional_element_kind(kind: &sysml_model::ElementKind) -> bool {
    kind.is_definition()
}

fn prefers_usage_endpoint(kind: RelationshipKindHint, role: EndpointRole) -> bool {
    match kind {
        RelationshipKindHint::Allocate => true,
        RelationshipKindHint::Satisfy => matches!(role, EndpointRole::Target),
        RelationshipKindHint::ViewpointConformance => false,
    }
}

fn usage_preferred_code(kind: RelationshipKindHint) -> &'static str {
    match kind {
        RelationshipKindHint::Allocate => "allocate_endpoint_prefers_usage",
        RelationshipKindHint::Satisfy => "satisfy_endpoint_prefers_usage",
        RelationshipKindHint::ViewpointConformance => "unresolved_viewpoint_conformance_target",
    }
}

fn diagnostic_container_prefix(node: &SemanticNode) -> &str {
    node.id
        .qualified_name
        .rsplit_once("::")
        .map(|(prefix, _)| prefix)
        .unwrap_or("")
}

fn resolve_endpoint_reference(
    graph: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    reference_name: &str,
) -> EndpointResolveOutcome {
    if let ResolveResult::Resolved(id) =
        resolve_expression_endpoint_strict(graph, uri, container_prefix, reference_name)
    {
        return classify_resolved_node(graph, id);
    }

    let normalized = reference_name.replace('.', "::");
    let segments: Vec<&str> = normalized
        .split("::")
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.len() > 1 {
        let owner_expr = segments[0];
        let ResolveResult::Resolved(mut current_id) =
            resolve_expression_endpoint_strict(graph, uri, container_prefix, owner_expr)
        else {
            return EndpointResolveOutcome::Unresolved;
        };
        for member in segments.iter().skip(1) {
            let Some(owner) = graph.get_node(&current_id) else {
                return EndpointResolveOutcome::Unresolved;
            };
            let ResolveResult::Resolved(next_id) = resolve_member_via_type(graph, owner, member)
            else {
                return EndpointResolveOutcome::Unresolved;
            };
            current_id = next_id;
        }
        return classify_resolved_node(graph, current_id);
    }

    match resolve_expression_endpoint_workspace(graph, reference_name) {
        ResolveResult::Resolved(id) => classify_resolved_node(graph, id),
        ResolveResult::Ambiguous | ResolveResult::Unresolved => EndpointResolveOutcome::Unresolved,
    }
}

fn classify_resolved_node(
    graph: &SemanticGraph,
    id: sysml_model::semantic::model::NodeId,
) -> EndpointResolveOutcome {
    let Some(node) = graph.get_node(&id).cloned() else {
        return EndpointResolveOutcome::Unresolved;
    };
    if is_definitional_element_kind(&node.element_kind) {
        EndpointResolveOutcome::Definitional(Box::new(node))
    } else {
        EndpointResolveOutcome::UsageLike
    }
}

fn find_case_alternate_part_usage(graph: &SemanticGraph, uri: &Url, name: &str) -> Option<String> {
    graph
        .nodes_for_uri(uri)
        .into_iter()
        .find(|node| {
            node.element_kind == sysml_model::ElementKind::Part
                && node.name != name
                && node.name.eq_ignore_ascii_case(name)
        })
        .map(|node| node.name.clone())
}

fn format_definition_preferred_message(
    graph: &SemanticGraph,
    uri: &Url,
    reference_name: &str,
    node: &SemanticNode,
    kind: RelationshipKindHint,
    role: EndpointRole,
) -> String {
    let relation = match kind {
        RelationshipKindHint::Allocate => "allocate",
        RelationshipKindHint::Satisfy => "satisfy",
        RelationshipKindHint::ViewpointConformance => "viewpoint conformance",
    };
    let endpoint = match role {
        EndpointRole::Source => "source",
        EndpointRole::Target => "target",
    };
    let usage_example = match (kind, role) {
        (RelationshipKindHint::Allocate, EndpointRole::Source) => "webshopSystem.checkoutService",
        (RelationshipKindHint::Allocate, EndpointRole::Target) => "commerceCluster",
        (RelationshipKindHint::Satisfy, EndpointRole::Target) => "webshopSystem.checkoutService",
        _ => "owner.feature",
    };
    let mut message = format!(
        "'{reference_name}' refers to a {element_kind} ({qualified}). \
         {relation} {endpoint} endpoints should be part usages or feature paths in the solution model, not type names alone. \
         For example: {relation} {usage_example} to <target>;",
        element_kind = node.element_kind,
        qualified = node.id.qualified_name,
    );
    if let Some(alternate) = find_case_alternate_part_usage(graph, uri, reference_name) {
        message.push_str(&format!(
            " Did you mean the part usage '{alternate}' instead of the part def '{reference_name}'?"
        ));
    }
    message
}

fn format_unresolved_message(
    graph: &SemanticGraph,
    uri: &Url,
    reference_name: &str,
    kind: RelationshipKindHint,
    role: EndpointRole,
) -> String {
    let relation = match kind {
        RelationshipKindHint::Allocate => "allocate",
        RelationshipKindHint::Satisfy => "satisfy",
        RelationshipKindHint::ViewpointConformance => "viewpoint conformance",
    };
    let endpoint = match role {
        EndpointRole::Source => "source",
        EndpointRole::Target => "target",
    };
    let mut message = format!(
        "Could not resolve {relation} {endpoint} '{reference_name}'. \
         Check spelling, imports, and qualified paths (e.g. Package::element or owner.member)."
    );
    if let Some(alternate) = find_case_alternate_part_usage(graph, uri, reference_name) {
        message.push_str(&format!(
            " Did you mean the part usage '{alternate}' instead of '{reference_name}'?"
        ));
    }
    if matches!(kind, RelationshipKindHint::Allocate) {
        message.push_str(
            " allocate maps one solution feature to another (typically part usages), not part def names alone.",
        );
    } else if matches!(kind, RelationshipKindHint::Satisfy) && role == EndpointRole::Target {
        message.push_str(
            " satisfy by should name the design feature that fulfills the requirement (e.g. webshopSystem.checkoutService).",
        );
    }
    message
}

/// Refine or suppress a builder relationship diagnostic after the workspace graph is merged.
pub(crate) fn builder_relationship_diagnostic_to_emit(
    graph: &SemanticGraph,
    uri: &Url,
    node: &SemanticNode,
    code: &str,
    message: &str,
) -> Option<(String, String)> {
    let Some((kind, role)) = relationship_kind_from_code(code) else {
        return Some((code.to_string(), message.to_string()));
    };
    let Some(reference_name) = extract_single_quoted_value(message) else {
        return Some((code.to_string(), message.to_string()));
    };
    let container_prefix = diagnostic_container_prefix(node);
    let mut outcome =
        resolve_endpoint_reference(graph, uri, Some(container_prefix), &reference_name);
    if matches!(outcome, EndpointResolveOutcome::Unresolved) {
        outcome = resolve_endpoint_reference(graph, uri, None, &reference_name);
    }

    match outcome {
        EndpointResolveOutcome::UsageLike => None,
        EndpointResolveOutcome::Definitional(resolved) if prefers_usage_endpoint(kind, role) => {
            Some((
                usage_preferred_code(kind).to_string(),
                format_definition_preferred_message(
                    graph,
                    uri,
                    &reference_name,
                    &resolved,
                    kind,
                    role,
                ),
            ))
        }
        EndpointResolveOutcome::Definitional(_) => None,
        EndpointResolveOutcome::Unresolved => Some((
            code.to_string(),
            format_unresolved_message(graph, uri, &reference_name, kind, role),
        )),
    }
}

fn extract_single_quoted_value(message: &str) -> Option<String> {
    let start = message.find('\'')?;
    let rest = &message[start + 1..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

#[cfg(test)]
mod tests {
    use crate::collect_diagnostics_from_graph;
    use sysml_model::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
    use sysml_model::semantic::workspace_graph::build_semantic_graph_from_documents;
    use crate::DiagnosticsOptions;

    #[test]
    fn allocate_part_def_endpoint_reports_prefers_usage_diagnostic() {
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
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &example.uri, DiagnosticsOptions::default());
        let prefers_usage = diagnostics
            .iter()
            .find(|d| d.code == "allocate_endpoint_prefers_usage")
            .expect("allocate_endpoint_prefers_usage diagnostic");
        assert!(
            prefers_usage.message.contains("part def"),
            "message should explain definitional endpoint: {}",
            prefers_usage.message
        );
        assert!(
            prefers_usage
                .message
                .contains("webshopSystem.checkoutService"),
            "message should suggest usage path: {}",
            prefers_usage.message
        );
        assert!(
            !diagnostics.iter().any(|d| d.code == "unresolved_allocate_source"),
            "should not report generic unresolved_allocate_source when name resolves to part def: {:?}",
            diagnostics
        );
    }

    #[test]
    fn unresolved_allocate_target_suggests_case_matching_part_usage() {
        let example = SysmlDocument::from_memory_path(
            "workspace",
            "webshop.sysml",
            r#"package WebShopExample {
                part def CommerceCluster;
                part commerceCluster : CommerceCluster;
                part webshopSystem;
                allocate webshopSystem to CommerceCLuster;
            }"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("example doc");
        let (graph, _parsed) =
            build_semantic_graph_from_documents(std::slice::from_ref(&example)).expect("graph");
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &example.uri, DiagnosticsOptions::default());
        let target_diag = diagnostics
            .iter()
            .find(|d| d.code == "unresolved_allocate_target")
            .expect("unresolved_allocate_target");
        assert!(
            target_diag.message.contains("commerceCluster"),
            "expected case hint toward part usage: {}",
            target_diag.message
        );
    }
}
