use std::collections::{HashMap, HashSet};

use url::Url;

use crate::semantic::diagnostics::checks::import_resolution::import_target_resolves;
use crate::semantic::diagnostics::helpers::{diag, diagnostic_range, is_synthetic};
use crate::semantic::diagnostics::types::DiagnosticSeverity;
use crate::semantic::relationships::resolve_type_target_in_workspace;
use crate::{SemanticDiagnostic, SemanticGraph};

const BUILTIN_MODELED_DECL_KEYWORDS: &[&str] = &[
    "feature",
    "class",
    "classifier",
    "struct",
    "structure",
    "subclassifier",
];

fn is_user_defined_modeled_keyword(keyword: &str) -> bool {
    !BUILTIN_MODELED_DECL_KEYWORDS.contains(&keyword)
}

pub(in crate::semantic::diagnostics) fn collect_view_metadata_conformance_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = HashSet::new();

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "view" || is_synthetic(node) {
            continue;
        }
        let has_expose = node
            .attributes
            .get("hasExpose")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let has_body = node
            .attributes
            .get("hasViewBody")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !has_body || has_expose {
            continue;
        }
        let key = format!("expose|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Information,
            "semantic",
            "view_expose_empty",
            format!(
                "View '{}' declares a body but exposes no members.",
                node.name
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "view rendering" || is_synthetic(node) {
            continue;
        }
        let allowed = ["rendering def", "rendering"];
        let type_ref = node
            .attributes
            .get("renderingType")
            .and_then(|v| v.as_str());
        let Some(type_ref) = type_ref.map(str::trim).filter(|value| !value.is_empty()) else {
            continue;
        };
        if resolve_type_target_in_workspace(graph, node, type_ref, &allowed).is_some() {
            continue;
        }
        let key = format!("rendering|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "view_rendering_invalid_target",
            format!(
                "View rendering '{}' type '{}' must resolve to a rendering definition or usage.",
                node.name, type_ref
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "metadata usage" || is_synthetic(node) {
            continue;
        }
        if node
            .attributes
            .get("metadataType")
            .and_then(|v| v.as_str())
            .is_some_and(|value| !value.trim().is_empty())
        {
            continue;
        }
        let annotation_name = node
            .attributes
            .get("annotationName")
            .and_then(|v| v.as_str())
            .unwrap_or(node.name.as_str());
        let allowed = ["metadata def"];
        if resolve_type_target_in_workspace(graph, node, annotation_name, &allowed).is_some() {
            continue;
        }
        let key = format!("metadata|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "metadata_annotation_unresolved",
            format!(
                "Metadata annotation '{}' does not resolve to a metadata definition.",
                annotation_name
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "import" || is_synthetic(node) {
            continue;
        }
        let Some(parent) = node
            .parent_id
            .as_ref()
            .and_then(|id| graph.get_node(id))
        else {
            continue;
        };
        if !matches!(
            parent.element_kind.as_str(),
            "viewpoint" | "viewpoint def" | "frame"
        ) {
            continue;
        }
        if import_target_resolves(graph, node) {
            continue;
        }
        let key = format!("viewpoint_import|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        let target = node
            .attributes
            .get("importTarget")
            .and_then(|v| v.as_str())
            .unwrap_or("import");
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "viewpoint_reference_unresolved",
            format!(
                "Viewpoint import target '{}' does not resolve in the current workspace.",
                target
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "textualRep" || is_synthetic(node) {
            continue;
        }
        let Some(parent) = node
            .parent_id
            .as_ref()
            .and_then(|id| graph.get_node(id))
        else {
            continue;
        };
        if !matches!(
            parent.element_kind.as_str(),
            "viewpoint" | "viewpoint def" | "frame" | "requirement def"
        ) {
            continue;
        }
        let language = node
            .attributes
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        if !language.is_empty() {
            continue;
        }
        let key = format!("viewpoint_rep|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "viewpoint_rep_language_unresolved",
            format!(
                "Textual representation '{}' on '{}' is missing a language identifier.",
                node.name, parent.name
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if !matches!(node.element_kind.as_str(), "feature decl" | "classifier decl")
            || is_synthetic(node)
        {
            continue;
        }
        let Some(keyword) = node.attributes.get("keyword").and_then(|v| v.as_str()) else {
            continue;
        };
        let keyword = keyword.trim();
        if keyword.is_empty() || !is_user_defined_modeled_keyword(keyword) {
            continue;
        }
        if resolve_type_target_in_workspace(graph, node, keyword, &["metadata def"]).is_some() {
            continue;
        }
        let diag_key = format!("metadata_kw|{}", node.id.qualified_name);
        if !seen.insert(diag_key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "metadata_keyword_unresolved",
            format!(
                "User-defined keyword '{}' does not resolve to a metadata definition.",
                keyword
            ),
        ));
    }

    let mut metadata_names: HashMap<String, Vec<String>> = HashMap::new();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "metadata def" || is_synthetic(node) {
            continue;
        }
        metadata_names
            .entry(node.name.clone())
            .or_default()
            .push(node.id.qualified_name.clone());
    }
    for (name, qualified_names) in metadata_names {
        if qualified_names.len() <= 1 {
            continue;
        }
        let key = format!("metadata_collision|{name}");
        if !seen.insert(key) {
            continue;
        }
        let Some(first) = graph.get_node(&crate::NodeId::new(uri, qualified_names[0].clone()))
        else {
            continue;
        };
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, first, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "metadata_keyword_collision",
            format!(
                "Metadata definition name '{}' is declared {} times in this document.",
                name,
                qualified_names.len()
            ),
        ));
    }

    diagnostics
}
