use std::collections::HashSet;

use url::Url;

use crate::checks::import_resolution::{
    import_target, import_target_range, is_expose, is_import_all, is_recursive_import,
};
use crate::helpers::{condition_expression_is_boolean, diag, diagnostic_range};
use crate::types::DiagnosticSeverity;
use crate::{DiagnosticRelatedInfo, SemanticDiagnostic};
use sysml_model::semantic::kinds::is_namespace;
use sysml_model::{resolve_import_target, ImportTargetResolution, SemanticGraph};

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
        if is_expose(node) {
            // Expose uses its own typed scope/expansion resolver, not Import conformance.
            continue;
        }
        let Some(target) = import_target(node) else {
            continue;
        };
        let range =
            import_target_range(node).unwrap_or_else(|| diagnostic_range(graph, node, None));

        let resolution = resolve_import_target(graph, node);
        if matches!(resolution, ImportTargetResolution::UnsupportedFiltered) {
            let key = format!("unsupported-filter|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    range,
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "unsupported_filtered_import",
                    "Filtered namespace imports are parsed but not semantically supported."
                        .to_string(),
                ));
            }
            continue;
        }
        if matches!(resolution, ImportTargetResolution::Unresolved) {
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
        if matches!(resolution, ImportTargetResolution::NotApplicable) {
            let key = format!("invalid-import-target|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    range,
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "invalid_import_target",
                    "Import declaration is incomplete and has no applicable typed target resolution."
                        .to_string(),
                ));
            }
            continue;
        }
        if let ImportTargetResolution::Ambiguous { candidates } = resolution {
            let key = format!("ambiguous|{}", node.id.qualified_name);
            if seen.insert(key) {
                let mut diagnostic = diag(
                    uri,
                    range,
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "ambiguous_import_target",
                    format!(
                        "Imported package/member '{}' is ambiguous across {} semantic targets.",
                        target,
                        candidates.len()
                    ),
                );
                diagnostic.related_information = candidates
                    .into_iter()
                    .filter_map(|candidate| {
                        graph
                            .get_node(&candidate)
                            .map(|resolved| DiagnosticRelatedInfo {
                                uri: resolved.id.uri.clone(),
                                range: resolved.range,
                                message: format!(
                                    "Ambiguous import candidate '{}'.",
                                    candidate.qualified_name
                                ),
                            })
                    })
                    .collect();
                diagnostics.push(diagnostic);
            }
            continue;
        }

        if let ImportTargetResolution::Resolved {
            target: resolved_id,
        } = resolution
        {
            let resolved_kind = graph
                .get_node(&resolved_id)
                .map(|node| node.element_kind.clone());
            if let Some(resolved_kind) = resolved_kind {
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
        let Some(condition) = node.expression_text.condition.as_deref() else {
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
