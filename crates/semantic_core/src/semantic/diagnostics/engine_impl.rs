use std::collections::HashSet;

use crate::semantic::text_span::TextRange;
use url::Url;

use crate::{resolve_type_reference_targets, NodeId, SemanticGraph};
use crate::semantic::diagnostics::types::{DiagnosticSeverity, SemanticDiagnostic};

use crate::semantic::diagnostics::checks::import_resolution::{import_target, import_target_resolves};

pub(crate) fn compute_semantic_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();

    // Explicit semantic diagnostics emitted by graph-builder passes.
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "diagnostic" {
            continue;
        }
        let code = node
            .attributes
            .get("code")
            .and_then(|value| value.as_str())
            .unwrap_or("semantic_diagnostic");
        let message = node
            .attributes
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("semantic diagnostic")
            .to_string();
        diagnostics.push(diag(
            uri,
            node.range,
            DiagnosticSeverity::Warning,
            "semantic",
            code,
            message,
        ));
    }

    // Unresolved import targets.
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "import" || import_target_resolves(graph, node) {
            continue;
        }
        let Some(target) = import_target(node) else {
            continue;
        };
        diagnostics.push(diag(
            uri,
            node.range,
            DiagnosticSeverity::Warning,
            "semantic",
            "unresolved_import_target",
            format!(
                "Imported package/member '{}' could not be resolved in the semantic graph.",
                target
            ),
        ));
    }

    // Duplicate endpoint pairs.
    let mut seen_pairs: HashSet<(NodeId, NodeId)> = HashSet::new();
    for (src_id, tgt_id) in graph.connection_edge_node_pairs_for_uri(uri) {
        let pair = normalize_edge_pair(&src_id, &tgt_id);
        if !seen_pairs.insert(pair) {
            if let Some(tgt) = graph.get_node(&tgt_id) {
                diagnostics.push(diag(
                    uri,
                    tgt.range,
                    DiagnosticSeverity::Information,
                    "semantic",
                    "duplicate_connection",
                    "Duplicate connection between the same two endpoints.".to_string(),
                ));
            }
        }
    }

    // Unresolved declared type references.
    for node in graph.nodes_for_uri(uri) {
        let Some(type_ref) = declared_type_ref(node) else {
            continue;
        };
        let normalized = normalize_declared_type_ref(type_ref);
        if normalized.is_empty() || is_builtin_type_ref(&normalized) {
            continue;
        }
        if !graph.outgoing_typing_or_specializes_targets(node).is_empty() {
            continue;
        }
        let resolved_via_import_scope = !resolve_type_reference_targets(
            graph,
            node,
            type_ref,
            &[
                "part def",
                "port def",
                "interface",
                "item def",
                "attribute def",
                "action def",
                "actor def",
                "occurrence def",
                "flow def",
                "allocation def",
                "state def",
                "requirement def",
                "use case def",
                "concern def",
                "enum def",
                "alias",
                "kermlDecl",
            ],
        )
        .is_empty();
        if resolved_via_import_scope {
            continue;
        }
        diagnostics.push(diag(
            uri,
            node.range,
            DiagnosticSeverity::Warning,
            "semantic",
            "unresolved_type_reference",
            format!(
                "Type reference '{}' for '{}' could not be resolved in the semantic graph.",
                type_ref, node.name
            ),
        ));
    }

    diagnostics
}

fn normalize_edge_pair(a: &NodeId, b: &NodeId) -> (NodeId, NodeId) {
    if a.qualified_name <= b.qualified_name {
        (a.clone(), b.clone())
    } else {
        (b.clone(), a.clone())
    }
}

fn diag(
    uri: &Url,
    range: TextRange,
    severity: DiagnosticSeverity,
    source: &str,
    code: &str,
    message: String,
) -> SemanticDiagnostic {
    SemanticDiagnostic {
        uri: uri.clone(),
        range,
        severity,
        code: code.to_string(),
        source: source.to_string(),
        message,
        related_information: Vec::new(),
    }
}

fn declared_type_ref(node: &crate::SemanticNode) -> Option<&str> {
    [
        "partType",
        "attributeType",
        "portType",
        "actionType",
        "actorType",
        "itemType",
        "occurrenceType",
        "flowType",
        "allocationType",
        "stateType",
        "requirementType",
        "useCaseType",
        "concernType",
    ]
    .iter()
    .find_map(|key| {
        node.attributes
            .get(*key)
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
    })
}

fn normalize_declared_type_ref(type_ref: &str) -> String {
    type_ref
        .trim()
        .trim_start_matches('~')
        .replace(' ', "")
        .replace('.', "::")
}

fn is_builtin_type_ref(type_ref: &str) -> bool {
    matches!(type_ref, "Boolean")
}
