use std::collections::HashSet;

use url::Url;

use crate::checks::import_resolution::{
    import_target, import_target_resolves, is_expose, is_import_all, is_recursive_import,
};
use crate::helpers::{
    condition_expression_is_boolean, diag, diagnostic_range, reference_token_range,
};
use crate::types::DiagnosticSeverity;
use crate::SemanticDiagnostic;
use sysml_model::semantic::kinds::is_namespace;
use sysml_model::{resolve_expose_target, ExposeTargetResolution, SemanticGraph};

fn import_target_is_resolved(
    graph: &SemanticGraph,
    uri: &Url,
    node: &sysml_model::SemanticNode,
    target: &str,
) -> bool {
    if is_expose(node) {
        return matches!(
            resolve_expose_target(
                graph,
                Some(uri),
                node.parent_id
                    .as_ref()
                    .map(|parent| parent.qualified_name.as_str()),
                target,
            ),
            ExposeTargetResolution::Resolved(_)
        );
    }
    import_target_resolves(graph, node)
}

fn normalized_namespace_target(target: &str) -> String {
    target
        .trim()
        .trim_end_matches("::**")
        .trim_end_matches("::*")
        .trim()
        .to_string()
}

fn resolve_import_target_kind(
    graph: &SemanticGraph,
    import_node: &sysml_model::SemanticNode,
) -> Option<sysml_model::ElementKind> {
    let target = import_target(import_node)?;
    let lookup = if is_import_all(import_node) {
        normalized_namespace_target(target)
    } else {
        target.trim().trim_end_matches("::**").trim().to_string()
    };
    graph
        .nodes_by_uri
        .values()
        .flatten()
        .find(|id| id.qualified_name == lookup)
        .and_then(|id| graph.get_node(id))
        .map(|node| node.element_kind.clone())
}

pub(crate) fn collect_import_conformance_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = HashSet::new();

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != sysml_model::ElementKind::Import {
            continue;
        }
        let Some(target) = import_target(node) else {
            continue;
        };
        let range = reference_token_range(node, target)
            .unwrap_or_else(|| diagnostic_range(graph, node, None));

        if !import_target_is_resolved(graph, uri, node, target) {
            let key = format!("unresolved|{}", target);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    range,
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "unresolved_import_target",
                    format!(
                        "Imported package/member '{}' could not be resolved in the semantic graph.",
                        target
                    ),
                ));
            }
            continue;
        }

        if let Some(resolved_kind) = resolve_import_target_kind(graph, node) {
            if is_import_all(node) && !is_namespace(&resolved_kind) {
                let key = format!("kind|{}", node.id.qualified_name);
                if seen.insert(key) {
                    diagnostics.push(diag(
                        uri,
                        range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "import_kind_mismatch",
                        format!(
                            "Namespace import '{}' targets '{}' which is a '{}', not a namespace.",
                            target, target, resolved_kind
                        ),
                    ));
                }
            }
            if !is_import_all(node)
                && is_namespace(&resolved_kind)
                && (target.contains("::*") || target.ends_with("::**"))
            {
                let key = format!("membership|{}", node.id.qualified_name);
                if seen.insert(key) {
                    diagnostics.push(diag(
                        uri,
                        range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "import_kind_mismatch",
                        format!(
                            "Membership import '{}' targets namespace '{}'; use a namespace import for wildcard targets.",
                            target, target
                        ),
                    ));
                }
            }
            if is_recursive_import(node) && !is_namespace(&resolved_kind) {
                let key = format!("recursive|{}", node.id.qualified_name);
                if seen.insert(key) {
                    diagnostics.push(diag(
                        uri,
                        range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "invalid_recursive_import",
                        format!(
                            "Recursive import '{}' targets '{}', which is not a namespace.",
                            target, resolved_kind
                        ),
                    ));
                }
            }
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != sysml_model::ElementKind::Filter {
            continue;
        }
        let owner_kind = node
            .attributes
            .get("filterOwnerKind")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if matches!(owner_kind, "view" | "view def" | "rendering def") {
            continue;
        }
        let Some(condition) = node
            .attributes
            .get("condition")
            .and_then(|value| value.as_str())
        else {
            continue;
        };
        if condition_expression_is_boolean(node, condition) {
            continue;
        }
        let key = format!("filter|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "invalid_import_filter",
            format!(
                "Import filter expression '{}' must be Boolean-valued.",
                condition
            ),
        ));
    }

    diagnostics
}
