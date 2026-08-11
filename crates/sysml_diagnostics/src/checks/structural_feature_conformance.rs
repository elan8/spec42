//! KerML feature and SysML variation constraints represented directly by the
//! current semantic graph.  Do not add type-name equality checks here: type
//! conformance belongs to graph traversal in `kind_compatibility`.

use std::collections::HashSet;

use url::Url;

use crate::helpers::{diag, diagnostic_range};
use crate::types::DiagnosticSeverity;
use crate::SemanticDiagnostic;
use sysml_model::semantic::kinds::is_flow_payload_occurrence_type;
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

/// Flow and allocation definitions are binary connection-like definitions.  General connection
/// definitions, in contrast, may be n-ary, so their end cardinality is only constrained by the
/// incomplete-pair check below.
fn requires_exactly_two_ends(kind: &ElementKind) -> bool {
    matches!(kind, ElementKind::FlowDef | ElementKind::AllocationDef)
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
        // SysML flow payloads carry occurrences, not values. The payload's real child feature
        // and its resolved typing edge are materialized by the graph builder, so this check
        // needs neither a textual `of` clause nor a parallel type resolver. Leave an untyped
        // payload to the existing unresolved-type diagnostics rather than guessing.
        if node.element_kind == ElementKind::FlowPayload {
            for target in graph.outgoing_targets_by_kind(node, RelationshipKind::Typing) {
                if !is_flow_payload_occurrence_type(&target.element_kind) {
                    diagnostics.push(diag(
                        uri,
                        diagnostic_range(graph, node, Some(target)),
                        DiagnosticSeverity::Error,
                        "semantic",
                        "flow_payload_type_not_occurrence",
                        format!(
                            "Flow payload '{}' is typed by '{}' ({}), which is not an occurrence type.",
                            node.name, target.name, target.element_kind
                        ),
                    ));
                }
            }
        }

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

        // SysML flow and allocation definitions are binary.  This deliberately covers only an
        // authored excess of ends: zero authored ends may be a declaration completed by an
        // effective/inherited closure which is not yet materialized by the graph, and one end is
        // already reported by the narrower incomplete-pair diagnostic above.  Connection
        // definitions remain n-ary-capable.
        if requires_exactly_two_ends(&node.element_kind)
            && !properties(node).is_some_and(|props| props.is_abstract)
            && !inherits_any_positional_end(graph, node)
        {
            let end_count = graph.positional_end_features(node).len();
            if end_count > 2 {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "invalid_binary_connection_like_end_count",
                    format!(
                        "{} '{}' declares {end_count} ends; binary definitions require exactly two.",
                        node.element_kind, node.name
                    ),
                ));
            }
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
            // A redefinition can only be introduced by the same featuring type or one that
            // specializes the type featuring the redefined member. These are published
            // effective facts after linking; absent/ambiguous featuring closure stays silent
            // instead of being reconstructed from a name or source range.
            if let (Some(redefining_type), Some(redefined_type)) = (
                graph
                    .effective_facts_for(node)
                    .and_then(|facts| facts.featuring_type.as_ref())
                    .and_then(|id| graph.get_node(id)),
                graph
                    .effective_facts_for(target)
                    .and_then(|facts| facts.featuring_type.as_ref())
                    .and_then(|id| graph.get_node(id)),
            ) {
                if !graph.specializes_transitively(redefining_type, redefined_type) {
                    diagnostics.push(diag(
                        uri,
                        diagnostic_range(graph, node, Some(target)),
                        DiagnosticSeverity::Error,
                        "semantic",
                        "redefinition_featuring_type_incompatible",
                        format!(
                            "Feature '{}' is featured by '{}' and cannot redefine '{}' featured by unrelated type '{}'.",
                            node.name,
                            redefining_type.name,
                            target.name,
                            redefined_type.name
                        ),
                    ));
                }
            }

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
            declared_name: Some(name.into()),
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
            source_text: Default::default(),
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
    fn redefinition_requires_overlapping_effective_featuring_types() {
        let uri = Url::parse("file:///redefinition-featuring.sysml").unwrap();
        let mut graph = SemanticGraph::new();
        let base_type = node(
            &uri,
            "Base",
            ElementKind::PartDef,
            DeclaredFeatureProperties::default(),
        );
        let derived_type = node(
            &uri,
            "Derived",
            ElementKind::PartDef,
            DeclaredFeatureProperties::default(),
        );
        let unrelated_type = node(
            &uri,
            "Unrelated",
            ElementKind::PartDef,
            DeclaredFeatureProperties::default(),
        );
        let base_feature = SemanticNode {
            parent_id: Some(base_type.id.clone()),
            ..node(
                &uri,
                "Base::feature",
                ElementKind::Attribute,
                DeclaredFeatureProperties::default(),
            )
        };
        let valid_redefinition = SemanticNode {
            parent_id: Some(derived_type.id.clone()),
            ..node(
                &uri,
                "Derived::feature",
                ElementKind::Attribute,
                DeclaredFeatureProperties::default(),
            )
        };
        let invalid_redefinition = SemanticNode {
            parent_id: Some(unrelated_type.id.clone()),
            ..node(
                &uri,
                "Unrelated::feature",
                ElementKind::Attribute,
                DeclaredFeatureProperties::default(),
            )
        };
        for item in [
            &base_type,
            &derived_type,
            &unrelated_type,
            &base_feature,
            &valid_redefinition,
            &invalid_redefinition,
        ] {
            graph.insert_workspace_node(item.clone());
        }
        graph.insert_workspace_edge(
            &derived_type.id,
            &base_type.id,
            SemanticEdge::plain(RelationshipKind::Specializes),
        );
        graph.insert_workspace_edge(
            &valid_redefinition.id,
            &base_feature.id,
            SemanticEdge::plain(RelationshipKind::Redefinition),
        );
        graph.insert_workspace_edge(
            &invalid_redefinition.id,
            &base_feature.id,
            SemanticEdge::plain(RelationshipKind::Redefinition),
        );
        graph.refresh_effective_facts();

        let diagnostics = collect_structural_feature_conformance_diagnostics(&graph, &uri);
        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "redefinition_featuring_type_incompatible"
                    && diagnostic.message.contains("Unrelated::feature")
            }),
            "unrelated featuring types must be diagnosed: {diagnostics:?}"
        );
        assert!(
            !diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "redefinition_featuring_type_incompatible"
                    && diagnostic.message.contains("Derived::feature")
            }),
            "specializing featuring type must remain accepted: {diagnostics:?}"
        );
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
    fn rejects_only_excess_authored_ends_for_binary_connection_like_definitions() {
        let uri = Url::parse("file:///binary-ends.sysml").unwrap();
        let mut graph = SemanticGraph::new();
        for (owner_name, kind) in [
            ("Flow", ElementKind::FlowDef),
            ("Allocation", ElementKind::AllocationDef),
            ("Connection", ElementKind::ConnectionDef),
        ] {
            let owner = node(&uri, owner_name, kind, DeclaredFeatureProperties::default());
            graph.insert_workspace_node(owner.clone());
            for end_name in ["source", "target", "extra"] {
                graph.insert_workspace_node(SemanticNode {
                    parent_id: Some(owner.id.clone()),
                    ..node(
                        &uri,
                        &format!("{owner_name}::{end_name}"),
                        ElementKind::InterfaceEnd,
                        DeclaredFeatureProperties {
                            is_end: true,
                            ..Default::default()
                        },
                    )
                });
            }
        }

        let messages: Vec<_> = collect_structural_feature_conformance_diagnostics(&graph, &uri)
            .into_iter()
            .filter(|diagnostic| diagnostic.code == "invalid_binary_connection_like_end_count")
            .map(|diagnostic| diagnostic.message)
            .collect();
        assert_eq!(
            messages.len(),
            2,
            "only flow/allocation definitions are binary"
        );
        assert!(messages.iter().any(|message| message.contains("Flow")));
        assert!(messages
            .iter()
            .any(|message| message.contains("Allocation")));
        assert!(messages
            .iter()
            .all(|message| !message.contains("Connection")));
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
