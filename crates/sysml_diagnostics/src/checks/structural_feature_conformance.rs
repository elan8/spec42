//! KerML feature and SysML variation constraints represented directly by the
//! current semantic graph.  Do not add type-name equality checks here: type
//! conformance belongs to graph traversal in `kind_compatibility`.

use std::collections::HashSet;

use url::Url;

use crate::helpers::{diag, diagnostic_range};
use crate::types::DiagnosticSeverity;
use crate::SemanticDiagnostic;
use sysml_model::semantic::model::DeclaredFeatureProperties;
use sysml_model::{ElementKind, RelationshipKind, SemanticGraph, SemanticNode};

fn properties(node: &SemanticNode) -> Option<&DeclaredFeatureProperties> {
    node.declared_facts.feature_properties.as_ref()
}

fn is_connection_like_definition(kind: &ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::ConnectionDef | ElementKind::FlowDef | ElementKind::AllocationDef
    )
}

/// A declaration that specializes another connection-like definition can inherit its ends. The
/// current graph does not yet materialize an effective end closure, so the end-count rule must
/// suppress rather than guess whenever an explicit specialization chain contains an end fact.
fn inherits_any_positional_end(graph: &SemanticGraph, node: &SemanticNode) -> bool {
    let mut pending: Vec<_> = graph
        .outgoing_targets_by_kind(node, RelationshipKind::Specializes)
        .into_iter()
        .map(|target| target.id.clone())
        .collect();
    let mut visited = HashSet::new();

    while let Some(id) = pending.pop() {
        if !visited.insert(id.clone()) {
            continue;
        }
        let Some(ancestor) = graph.get_node(&id) else {
            continue;
        };
        if !graph.positional_end_features(ancestor).is_empty() {
            return true;
        }
        pending.extend(
            graph
                .outgoing_targets_by_kind(ancestor, RelationshipKind::Specializes)
                .into_iter()
                .map(|target| target.id.clone()),
        );
    }
    false
}

fn variant_kind_is_compatible(parent: &ElementKind, child: &ElementKind) -> bool {
    matches!(
        (parent, child),
        (ElementKind::PartDef | ElementKind::Part, ElementKind::Part)
            | (
                ElementKind::AttributeDef | ElementKind::Attribute,
                ElementKind::Attribute
            )
            | (ElementKind::ItemDef | ElementKind::Item, ElementKind::Item)
            | (ElementKind::PortDef | ElementKind::Port, ElementKind::Port)
            | (
                ElementKind::ActionDef | ElementKind::Action,
                ElementKind::Action
            )
            | (
                ElementKind::StateDef | ElementKind::State,
                ElementKind::State
            )
    )
}

/// Checks constraints whose operands are explicit parser facts or resolved
/// feature relationship edges.  This is intentionally conservative when a
/// fact is absent: absent is not the same as KerML's default value.
pub(crate) fn collect_structural_feature_conformance_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();

    for node in graph.nodes_for_uri(uri) {
        // A connection-like declaration which authors exactly one end has an incomplete binary
        // end pair. Do not infer an effective end closure: declarations with no authored ends,
        // declared abstractness, or an explicit specialized ancestor with an end stay silent
        // until their complete structural facts are represented.
        if is_connection_like_definition(&node.element_kind)
            && !properties(node).is_some_and(|props| props.is_abstract)
            && !inherits_any_positional_end(graph, node)
            && graph.positional_end_features(node).len() == 1
        {
            diagnostics.push(diag(
                uri,
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::Warning,
                "semantic",
                "incomplete_connection_like_end_pair",
                format!(
                    "{} '{}' declares one end; a binary connection-like declaration needs a second end.",
                    node.element_kind, node.name
                ),
            ));
        }

        let Some(props) = properties(node) else {
            continue;
        };

        // KerML §8.3.3.3.1: end features cannot carry direction, derived,
        // abstract, or composite properties.
        if props.is_end {
            if props.direction.is_some() {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "end_feature_has_direction",
                    format!("End feature '{}' must not declare a direction.", node.name),
                ));
            }
            if props.is_derived || props.is_abstract || props.is_composite == Some(true) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "end_feature_invalid_restrictions",
                    format!(
                        "End feature '{}' must not be derived, abstract, or composite.",
                        node.name
                    ),
                ));
            }
        }

        // SysML §8.4.3: a typed `variant` member must use the variation's
        // usage kind. Bare `variant name;` is intentionally ignored because it
        // has no materialized member kind in the graph.
        if props.is_variation {
            for child in graph.children_of(node) {
                if !child
                    .attributes
                    .get("isVariant")
                    .and_then(|value| value.as_bool())
                    .unwrap_or(false)
                {
                    continue;
                }
                if !variant_kind_is_compatible(&node.element_kind, &child.element_kind) {
                    diagnostics.push(diag(
                        uri,
                        diagnostic_range(graph, child, Some(node)),
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "invalid_variation_member_kind",
                        format!(
                            "Variant member '{}' ({}) is not compatible with variation '{}' ({}).",
                            child.name, child.element_kind, node.name, node.element_kind
                        ),
                    ));
                }
            }
        }

        for target in graph.outgoing_targets_by_kind(node, RelationshipKind::Redefinition) {
            let target_props = properties(target);
            // KerML §8.3.3.3.8: redefining an end requires an end feature.
            if target_props.is_some_and(|p| p.is_end) && !props.is_end {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, Some(target)),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "redefinition_end_mismatch",
                    format!(
                        "Feature '{}' redefines end feature '{}' but is not an end feature.",
                        node.name, target.name
                    ),
                ));
            }
            // The direction rule is only sound when both declared directions
            // are represented. Effective direction/conjugation is not yet a
            // graph fact, so do not infer it here.
            if let (Some(actual), Some(expected)) = (
                props.direction.as_deref(),
                target_props.and_then(|p| p.direction.as_deref()),
            ) {
                if actual != expected {
                    diagnostics.push(diag(uri, diagnostic_range(graph, node, Some(target)), DiagnosticSeverity::Warning, "semantic", "redefinition_direction_mismatch", format!("Feature '{}' direction '{}' does not conform to redefined feature '{}' direction '{}'.", node.name, actual, target.name, expected)));
                }
            }
        }

        for target in graph.outgoing_targets_by_kind(node, RelationshipKind::Subsetting) {
            let target_props = properties(target);
            // KerML §8.3.3.3.10. Check only explicit values: several current
            // graph builders do not project uniqueness for every feature kind.
            if props.is_unique == Some(false)
                && target_props.and_then(|p| p.is_unique) == Some(true)
            {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, Some(target)),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "subsetting_uniqueness_mismatch",
                    format!(
                        "Non-unique feature '{}' subsets unique feature '{}'.",
                        node.name, target.name
                    ),
                ));
            }
        }
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_model::semantic::text_span::{TextPosition, TextRange};
    use sysml_model::{NodeId, SemanticEdge, SemanticNode};

    fn node(
        uri: &Url,
        name: &str,
        kind: ElementKind,
        props: DeclaredFeatureProperties,
    ) -> SemanticNode {
        SemanticNode {
            id: NodeId::new(uri, name),
            element_kind: kind,
            name: name.into(),
            range: TextRange {
                start: TextPosition {
                    line: 0,
                    character: 0,
                },
                end: TextPosition {
                    line: 0,
                    character: 1,
                },
            },
            attributes: Default::default(),
            declared_facts: sysml_model::semantic::model::DeclaredSemanticFacts {
                feature_properties: Some(props),
                ..Default::default()
            },
            parent_id: None,
        }
    }

    #[test]
    fn validates_end_and_resolved_feature_relationship_facts() {
        let uri = Url::parse("file:///structural.sysml").unwrap();
        let mut graph = SemanticGraph::new();
        let mut end = DeclaredFeatureProperties {
            is_end: true,
            direction: Some("in".into()),
            ..Default::default()
        };
        end.is_composite = Some(true);
        let base = node(&uri, "base", ElementKind::Attribute, end);
        let redefining = node(
            &uri,
            "replacement",
            ElementKind::Attribute,
            DeclaredFeatureProperties {
                direction: Some("out".into()),
                is_unique: Some(false),
                ..Default::default()
            },
        );
        let unique = node(
            &uri,
            "uniqueBase",
            ElementKind::Attribute,
            DeclaredFeatureProperties {
                is_unique: Some(true),
                ..Default::default()
            },
        );
        for item in [&base, &redefining, &unique] {
            graph.insert_workspace_node(item.clone());
        }
        graph.insert_workspace_edge(
            &redefining.id,
            &base.id,
            SemanticEdge::plain(RelationshipKind::Redefinition),
        );
        graph.insert_workspace_edge(
            &redefining.id,
            &unique.id,
            SemanticEdge::plain(RelationshipKind::Subsetting),
        );
        let codes: Vec<_> = collect_structural_feature_conformance_diagnostics(&graph, &uri)
            .into_iter()
            .map(|d| d.code)
            .collect();
        for expected in [
            "end_feature_has_direction",
            "end_feature_invalid_restrictions",
            "redefinition_end_mismatch",
            "redefinition_direction_mismatch",
            "subsetting_uniqueness_mismatch",
        ] {
            assert!(
                codes.iter().any(|code| code == expected),
                "missing {expected}: {codes:?}"
            );
        }
    }

    #[test]
    fn incomplete_end_pair_uses_only_authored_and_specialized_end_facts() {
        let uri = Url::parse("file:///connection-like-ends.sysml").unwrap();
        let mut graph = SemanticGraph::new();
        let connection = node(
            &uri,
            "Incomplete",
            ElementKind::ConnectionDef,
            DeclaredFeatureProperties::default(),
        );
        let source_end = SemanticNode {
            parent_id: Some(connection.id.clone()),
            ..node(
                &uri,
                "Incomplete::source",
                ElementKind::InterfaceEnd,
                DeclaredFeatureProperties {
                    is_end: true,
                    ..Default::default()
                },
            )
        };
        let base = node(
            &uri,
            "Base",
            ElementKind::ConnectionDef,
            DeclaredFeatureProperties::default(),
        );
        let base_end = SemanticNode {
            parent_id: Some(base.id.clone()),
            ..node(
                &uri,
                "Base::source",
                ElementKind::InterfaceEnd,
                DeclaredFeatureProperties {
                    is_end: true,
                    ..Default::default()
                },
            )
        };
        let base_target_end = SemanticNode {
            parent_id: Some(base.id.clone()),
            ..node(
                &uri,
                "Base::target",
                ElementKind::InterfaceEnd,
                DeclaredFeatureProperties {
                    is_end: true,
                    ..Default::default()
                },
            )
        };
        let derived = node(
            &uri,
            "Derived",
            ElementKind::ConnectionDef,
            DeclaredFeatureProperties::default(),
        );
        let derived_end = SemanticNode {
            parent_id: Some(derived.id.clone()),
            ..node(
                &uri,
                "Derived::target",
                ElementKind::InterfaceEnd,
                DeclaredFeatureProperties {
                    is_end: true,
                    ..Default::default()
                },
            )
        };
        let abstract_connection = node(
            &uri,
            "Abstract",
            ElementKind::ConnectionDef,
            DeclaredFeatureProperties {
                is_abstract: true,
                ..Default::default()
            },
        );
        let abstract_end = SemanticNode {
            parent_id: Some(abstract_connection.id.clone()),
            ..node(
                &uri,
                "Abstract::source",
                ElementKind::InterfaceEnd,
                DeclaredFeatureProperties {
                    is_end: true,
                    ..Default::default()
                },
            )
        };
        for item in [
            &connection,
            &source_end,
            &base,
            &base_end,
            &base_target_end,
            &derived,
            &derived_end,
            &abstract_connection,
            &abstract_end,
        ] {
            graph.insert_workspace_node(item.clone());
        }
        graph.insert_workspace_edge(
            &derived.id,
            &base.id,
            SemanticEdge::plain(RelationshipKind::Specializes),
        );

        let reported: HashSet<_> = collect_structural_feature_conformance_diagnostics(&graph, &uri)
            .into_iter()
            .filter(|diagnostic| diagnostic.code == "incomplete_connection_like_end_pair")
            .map(|diagnostic| diagnostic.message)
            .collect();
        assert_eq!(
            reported.len(),
            1,
            "only the direct incomplete declaration is reportable"
        );
        assert!(reported
            .iter()
            .any(|message| message.contains("Incomplete")));
    }

    #[test]
    fn rejects_typed_variant_with_a_different_usage_kind() {
        let uri = Url::parse("file:///variation.sysml").unwrap();
        let mut graph = SemanticGraph::new();
        let variation = node(
            &uri,
            "choices",
            ElementKind::PartDef,
            DeclaredFeatureProperties {
                is_variation: true,
                ..Default::default()
            },
        );
        let mut invalid_variant = node(
            &uri,
            "wrongKind",
            ElementKind::Attribute,
            DeclaredFeatureProperties::default(),
        );
        invalid_variant.parent_id = Some(variation.id.clone());
        invalid_variant
            .attributes
            .insert("isVariant".into(), serde_json::json!(true));
        graph.insert_workspace_node(variation);
        graph.insert_workspace_node(invalid_variant);

        assert!(
            collect_structural_feature_conformance_diagnostics(&graph, &uri)
                .iter()
                .any(|diagnostic| diagnostic.code == "invalid_variation_member_kind")
        );
    }
}
