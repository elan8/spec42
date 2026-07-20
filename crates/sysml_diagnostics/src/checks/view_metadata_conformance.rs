use std::collections::{HashMap, HashSet};

use url::Url;

use crate::checks::import_resolution::import_target_resolves;
use crate::helpers::{
    diag, diagnostic_range, is_synthetic, is_unknown_range, parse_attribute_text_range,
};
use crate::types::DiagnosticSeverity;
use sysml_model::semantic::model::{ElementKind, RelationshipKind};
use sysml_model::semantic::reference_resolution::{resolve_expose_target, ExposeTargetResolution};
use sysml_model::semantic::relationships::{
    resolve_type_target_in_workspace, ANNOTATED_ELEMENT_TARGET_KINDS,
};
use sysml_model::semantic::standard_views::is_non_standard_explicit_view_type;
use sysml_model::semantic::text_span::TextRange;
use crate::SemanticDiagnostic;
use sysml_model::SemanticGraph;

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

pub(crate) fn collect_view_metadata_conformance_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = HashSet::new();

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != ElementKind::View || is_synthetic(node) {
            continue;
        }
        if let Some(view_type) = node
            .attributes
            .get("viewType")
            .and_then(|value| value.as_str())
        {
            if is_non_standard_explicit_view_type(view_type) {
                let key = format!(
                    "view_type_non_standard|{}|{}",
                    node.id.qualified_name, view_type
                );
                if seen.insert(key) {
                    diagnostics.push(diag(
                        uri,
                        diagnostic_range(graph, node, None),
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "view_type_non_standard",
                        format!(
                            "View '{}' uses non-standard view type '{}'; use a SysML v2 standard view definition from §9.2.20 Table 34.",
                            node.name, view_type
                        ),
                    ));
                }
            }
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != ElementKind::View || is_synthetic(node) {
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
        if node.element_kind != ElementKind::View || is_synthetic(node) {
            continue;
        }
        let Some(targets) = node
            .attributes
            .get("exposeTargets")
            .and_then(|value| value.as_array())
        else {
            continue;
        };
        let container_prefix = node
            .id
            .qualified_name
            .rsplit_once("::")
            .map(|(prefix, _)| prefix);
        for target in targets {
            let Some(target_text) = target.get("target").and_then(|value| value.as_str()) else {
                continue;
            };
            let range = expose_target_entry_range(node, target);
            let key = format!(
                "expose_unresolved|{}|{}",
                node.id.qualified_name, target_text
            );
            match resolve_expose_target(graph, Some(uri), container_prefix, target_text) {
                ExposeTargetResolution::Unresolved => {
                    if !seen.insert(key) {
                        continue;
                    }
                    diagnostics.push(diag(
                        uri,
                        range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "view_expose_unresolved",
                        format!(
                            "View '{}' expose target '{}' does not resolve to any element.",
                            node.name, target_text
                        ),
                    ));
                }
                ExposeTargetResolution::Ambiguous => {
                    if !seen.insert(key) {
                        continue;
                    }
                    diagnostics.push(diag(
                        uri,
                        range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "view_expose_unresolved",
                        format!(
                            "View '{}' expose target '{}' is ambiguous.",
                            node.name, target_text
                        ),
                    ));
                }
                ExposeTargetResolution::Resolved(names) if names.is_empty() => {
                    if !seen.insert(key) {
                        continue;
                    }
                    diagnostics.push(diag(
                        uri,
                        range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "view_expose_unresolved",
                        format!(
                            "View '{}' expose target '{}' does not resolve to any element.",
                            node.name, target_text
                        ),
                    ));
                }
                ExposeTargetResolution::Resolved(_) => {}
            }
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != ElementKind::ViewRendering || is_synthetic(node) {
            continue;
        }
        let allowed = [ElementKind::RenderingDef, ElementKind::Rendering];
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
        if node.element_kind != ElementKind::MetadataUsage || is_synthetic(node) {
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
        let allowed = [ElementKind::MetadataDef];
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
        if is_synthetic(node) {
            continue;
        }
        let (target, key_prefix) = if node.element_kind == ElementKind::Import {
            let Some(parent) = node.parent_id.as_ref().and_then(|id| graph.get_node(id)) else {
                continue;
            };
            if !(matches!(
                parent.element_kind,
                ElementKind::Viewpoint | ElementKind::ViewpointDef
            ) || parent.element_kind.as_str() == "frame")
            {
                continue;
            }
            if import_target_resolves(graph, node) {
                continue;
            }
            let target = node
                .attributes
                .get("importTarget")
                .and_then(|v| v.as_str())
                .unwrap_or("import");
            (target.to_string(), "viewpoint_import")
        } else if matches!(
            node.element_kind,
            ElementKind::Stakeholder | ElementKind::Purpose
        ) {
            let Some(parent) = node.parent_id.as_ref().and_then(|id| graph.get_node(id)) else {
                continue;
            };
            if !matches!(
                parent.element_kind,
                ElementKind::Viewpoint | ElementKind::ViewpointDef
            ) {
                continue;
            }
            let Some(target) = node
                .attributes
                .get("refTarget")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
            else {
                continue;
            };
            if resolve_type_target_in_workspace(
                graph,
                node,
                target,
                &[
                    ElementKind::RequirementDef,
                    ElementKind::Concern,
                    ElementKind::ConcernDef,
                    ElementKind::Requirement,
                    ElementKind::PartDef,
                    ElementKind::Part,
                ],
            )
            .is_some()
            {
                continue;
            }
            (target.to_string(), "viewpoint_ref")
        } else {
            continue;
        };
        let key = format!("{key_prefix}|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "viewpoint_reference_unresolved",
            format!(
                "Viewpoint reference target '{}' does not resolve in the current workspace.",
                target
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "textualRep" || is_synthetic(node) {
            continue;
        }
        let Some(parent) = node.parent_id.as_ref().and_then(|id| graph.get_node(id)) else {
            continue;
        };
        if !(matches!(
            parent.element_kind,
            ElementKind::Viewpoint | ElementKind::ViewpointDef | ElementKind::RequirementDef
        ) || parent.element_kind.as_str() == "frame")
        {
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
        let range = parse_attribute_text_range(node, "languageSpan")
            .filter(|range| !is_unknown_range(*range))
            .unwrap_or_else(|| diagnostic_range(graph, node, None));
        diagnostics.push(diag(
            uri,
            range,
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
        if is_synthetic(node) {
            continue;
        }
        let keyword = if matches!(
            node.element_kind.as_str(),
            "feature decl" | "classifier decl"
        ) {
            let Some(keyword) = node.attributes.get("keyword").and_then(|v| v.as_str()) else {
                continue;
            };
            let keyword = keyword.trim();
            if keyword.is_empty() || !is_user_defined_modeled_keyword(keyword) {
                continue;
            }
            keyword.to_string()
        } else if node.element_kind == ElementKind::MetadataKeyword {
            let Some(keyword) = node.attributes.get("keyword").and_then(|v| v.as_str()) else {
                continue;
            };
            let keyword = keyword.trim();
            if keyword.is_empty() {
                continue;
            }
            keyword.to_string()
        } else {
            continue;
        };
        if resolve_type_target_in_workspace(graph, node, &keyword, &[ElementKind::MetadataDef])
            .is_some()
        {
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
        if node.element_kind != ElementKind::MetadataDef || is_synthetic(node) {
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
        let Some(first) = graph.get_node(&sysml_model::NodeId::new(uri, qualified_names[0].clone()))
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

    for node in graph.nodes_for_uri(uri) {
        if !matches!(
            node.element_kind,
            ElementKind::MetadataUsage | ElementKind::MetadataKeyword
        ) || is_synthetic(node)
        {
            continue;
        }
        let Some(targets) = node
            .attributes
            .get("aboutTargets")
            .and_then(|v| v.as_array())
        else {
            continue;
        };
        for target in targets {
            let Some(target_ref) = target.as_str().map(str::trim).filter(|s| !s.is_empty()) else {
                continue;
            };
            if resolve_type_target_in_workspace(
                graph,
                node,
                target_ref,
                ANNOTATED_ELEMENT_TARGET_KINDS,
            )
            .is_some()
            {
                continue;
            }
            let key = format!("about|{}|{}", node.id.qualified_name, target_ref);
            if !seen.insert(key) {
                continue;
            }
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "metadata_about_unresolved",
                format!(
                    "Metadata '{}' about target '{}' does not resolve to a model element.",
                    node.name, target_ref
                ),
            ));
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != ElementKind::MetadataUsage || is_synthetic(node) {
            continue;
        }
        let Some(def_node) = graph
            .outgoing_targets_by_kind(node, RelationshipKind::Typing)
            .into_iter()
            .find(|target| target.element_kind == ElementKind::MetadataDef)
        else {
            continue;
        };
        let required: HashSet<String> = graph
            .children_of(def_node)
            .into_iter()
            .filter(|child| {
                matches!(
                    child.element_kind,
                    ElementKind::AttributeDef | ElementKind::Attribute
                )
            })
            .map(|child| child.name.clone())
            .collect();
        if required.is_empty() {
            continue;
        }
        let bound: HashSet<String> = graph
            .children_of(node)
            .into_iter()
            .filter(|child| child.element_kind == ElementKind::Attribute)
            .map(|child| child.name.clone())
            .collect();
        for name in required.difference(&bound) {
            let key = format!("binding_missing|{}|{}", node.id.qualified_name, name);
            if !seen.insert(key) {
                continue;
            }
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "metadata_binding_missing",
                format!(
                    "Metadata usage '{}' is missing binding for attribute '{}'.",
                    node.name, name
                ),
            ));
        }
        for name in bound.difference(&required) {
            let key = format!("binding_unknown|{}|{}", node.id.qualified_name, name);
            if !seen.insert(key) {
                continue;
            }
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "metadata_binding_unknown",
                format!(
                    "Metadata usage '{}' binds unknown attribute '{}'.",
                    node.name, name
                ),
            ));
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != ElementKind::MetadataDef || is_synthetic(node) {
            continue;
        }
        let specializes_semantic = graph
            .outgoing_targets_by_kind(node, RelationshipKind::Specializes)
            .into_iter()
            .any(|target| target.name == "SemanticMetadata");
        if !specializes_semantic {
            continue;
        }
        let restrictions: Vec<String> = graph
            .children_of(node)
            .into_iter()
            .filter_map(annotated_element_restriction_type)
            .collect();
        if restrictions.is_empty() {
            continue;
        }
        for usage in graph.nodes_for_uri(uri) {
            if usage.element_kind != ElementKind::MetadataUsage || is_synthetic(usage) {
                continue;
            }
            let typed_to_def = graph
                .outgoing_targets_by_kind(usage, RelationshipKind::Typing)
                .into_iter()
                .any(|target| target.id == node.id);
            if !typed_to_def {
                continue;
            }
            for annotated in graph.outgoing_targets_by_kind(usage, RelationshipKind::Annotation) {
                let compatible = restrictions.iter().any(|restriction| {
                    annotated_element_matches_restriction(&annotated.element_kind, restriction)
                });
                if compatible {
                    continue;
                }
                let key = format!(
                    "annotated_incompatible|{}|{}",
                    usage.id.qualified_name, annotated.id.qualified_name
                );
                if !seen.insert(key) {
                    continue;
                }
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, usage, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "metadata_annotated_element_incompatible",
                    format!(
                        "Metadata usage '{}' annotates '{}' which is incompatible with SemanticMetadata restrictions.",
                        usage.name, annotated.name
                    ),
                ));
            }
        }
    }

    diagnostics
}

fn annotated_element_restriction_type(
    child: &sysml_model::semantic::model::SemanticNode,
) -> Option<String> {
    if child.element_kind != ElementKind::Attribute {
        return None;
    }
    let is_annotated_element_restriction = child
        .attributes
        .get("redefines")
        .and_then(|v| v.as_str())
        .is_some_and(|r| r.contains("annotatedElement"))
        || child
            .attributes
            .get("subsetsFeature")
            .and_then(|v| v.as_str())
            .is_some_and(|r| r.contains("annotatedElement"))
        || child.name == "annotatedElement";
    if !is_annotated_element_restriction {
        return None;
    }
    child
        .attributes
        .get("attributeType")
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

fn expose_target_entry_range(
    node: &sysml_model::semantic::model::SemanticNode,
    entry: &serde_json::Value,
) -> TextRange {
    if let Some(range) = entry.get("range") {
        let start = range.get("start");
        let end = range.get("end");
        if let (Some(start), Some(end)) = (start, end) {
            if let (Some(start_line), Some(start_character), Some(end_line), Some(end_character)) = (
                start.get("line").and_then(|v| v.as_u64()),
                start.get("character").and_then(|v| v.as_u64()),
                end.get("line").and_then(|v| v.as_u64()),
                end.get("character").and_then(|v| v.as_u64()),
            ) {
                return TextRange {
                    start: sysml_model::semantic::text_span::TextPosition::new(
                        start_line as u32,
                        start_character as u32,
                    ),
                    end: sysml_model::semantic::text_span::TextPosition::new(
                        end_line as u32,
                        end_character as u32,
                    ),
                };
            }
        }
    }
    node.range
}

fn annotated_element_matches_restriction(
    element_kind: &sysml_model::ElementKind,
    restriction: &str,
) -> bool {
    let element_kind = element_kind.as_str();
    let local = restriction.rsplit("::").next().unwrap_or(restriction);
    let normalized = local
        .trim_end_matches("Definition")
        .trim_end_matches("Usage")
        .to_ascii_lowercase();
    if normalized.is_empty() {
        return true;
    }
    element_kind
        .to_ascii_lowercase()
        .contains(normalized.as_str())
}
