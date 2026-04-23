use tower_lsp::lsp_types::Url;

use crate::semantic_model::{resolve_expression_endpoint_strict, ResolveResult, SemanticGraph};

pub(super) fn should_suppress_builder_diagnostic(
    graph: &SemanticGraph,
    uri: &Url,
    node: &crate::semantic_model::SemanticNode,
    code: &str,
    message: &str,
) -> bool {
    if !matches!(
        code,
        "unresolved_satisfy_source" | "unresolved_satisfy_target"
    ) {
        return false;
    }
    let Some(reference_name) = extract_single_quoted_value(message) else {
        return false;
    };
    if matches!(
        resolve_expression_endpoint_strict(
            graph,
            uri,
            Some(diagnostic_container_prefix(node)),
            &reference_name
        ),
        ResolveResult::Resolved(_)
    ) {
        return true;
    }
    matches!(
        resolve_expression_endpoint_strict(graph, uri, None, &reference_name),
        ResolveResult::Resolved(_)
    )
}

fn extract_single_quoted_value(message: &str) -> Option<String> {
    let start = message.find('\'')?;
    let rest = &message[start + 1..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

fn diagnostic_container_prefix(node: &crate::semantic_model::SemanticNode) -> &str {
    node.id
        .qualified_name
        .rsplit_once("::")
        .map(|(prefix, _)| prefix)
        .unwrap_or("")
}
