use tower_lsp::lsp_types::Url;

use crate::semantic::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult, SemanticGraph,
};

pub(super) fn should_suppress_builder_diagnostic(
    graph: &SemanticGraph,
    uri: &Url,
    node: &crate::semantic::SemanticNode,
    code: &str,
    message: &str,
) -> bool {
    if !matches!(
        code,
        "unresolved_satisfy_source"
            | "unresolved_satisfy_target"
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

fn diagnostic_container_prefix(node: &crate::semantic::SemanticNode) -> &str {
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
        return false;
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
