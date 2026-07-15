//! Host validation and projection assembly from a built semantic graph.

use std::collections::{BTreeSet, HashMap};

use sha2::{Digest, Sha256};

use sysml_model::{
    DiagnosticSeverity, SemanticDiagnostic, SemanticGraph, SysmlDocument, UnitRegistry,
    resolved_usage_context, typed_by_reference,
};
use url::Url;

use super::discovery::path_to_file_url;
use super::projection::{
    HostExpression, HostExpressionArgument, HostFeatureValue, HostMultiplicity,
    HostRelationshipMetaclass, HostSemanticModelNode, HostSemanticModelRelationship,
    HostSemanticProjection,
};
use super::validation::{HostValidatedDocument, HostValidationReport, HostValidationSummary};

pub(crate) fn collect_host_validation_report(
    graph: &SemanticGraph,
    documents: &[SysmlDocument],
    library_urls: &[Url],
    target_files: &[std::path::PathBuf],
    workspace_root: Option<&std::path::Path>,
    library_paths_display: &[std::path::PathBuf],
    strict_diagnostics: bool,
) -> crate::error::WorkspaceResult<HostValidationReport> {
    let target_urls = target_file_urls(target_files)?;
    let unit_registry = UnitRegistry::from_graph(graph);
    // Keyed by normalized URI: document URIs may differ from `target_urls` in drive-letter
    // case (documents come from whatever the caller/provider constructed, `target_urls` is
    // always canonicalized by `path_to_file_url`), so raw string keys would silently miss.
    let document_text: HashMap<String, &str> = documents
        .iter()
        .map(|doc| {
            (
                language_service::uri::normalize_uri(&doc.uri).to_string(),
                doc.content.as_str(),
            )
        })
        .collect();
    let mut host_documents = Vec::new();

    for uri in &target_urls {
        let text = document_text
            .get(language_service::uri::normalize_uri(uri).as_str())
            .copied()
            .unwrap_or("");
        let diagnostics = collect_host_document_diagnostics(
            graph,
            &unit_registry,
            library_urls,
            uri,
            text,
            strict_diagnostics,
        );
        host_documents.push(HostValidatedDocument {
            uri: uri.to_string(),
            diagnostics,
        });
    }

    Ok(HostValidationReport {
        workspace_root: workspace_root.map(|path| path.display().to_string()),
        resolved_library_paths: library_paths_display
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        documents: host_documents.clone(),
        summary: summarize_host_documents(&host_documents),
    })
}

pub(crate) fn project_host_semantic_model(
    graph: &SemanticGraph,
    target_files: &[std::path::PathBuf],
) -> crate::error::WorkspaceResult<HostSemanticProjection> {
    let target_urls = target_file_urls(target_files)?;
    let mut nodes = Vec::new();
    for uri in &target_urls {
        for node in graph.nodes_for_uri(uri) {
            if node.element_kind == sysml_model::ElementKind::Diagnostic {
                continue;
            }
            let mut attributes = node.attributes.clone();
            // Additive: resolve the usage's canonical type reference and, from it,
            // the definition's direct implementation context. Existing textual
            // hints (`partType`, `type`, `typing`, ...) are left untouched.
            // See docs/engineering/COMPONENT-IMPLEMENTATION-CONTEXT-ROADMAP.md.
            if let Some(typed_by) = typed_by_reference(graph, node) {
                if let Ok(value) = serde_json::to_value(&typed_by) {
                    attributes.insert("typedBy".to_string(), value);
                }
            }
            if let Some(context) = resolved_usage_context(graph, node) {
                if let Ok(value) = serde_json::to_value(&context) {
                    attributes.insert("resolvedUsageContext".to_string(), value);
                }
            }

            nodes.push(HostSemanticModelNode {
                semantic_id: semantic_element_id(
                    node.id.uri.as_str(),
                    &node.element_kind,
                    node.range.start.line,
                    node.range.start.character,
                ),
                uri: node.id.uri.to_string(),
                qualified_name: node.id.qualified_name.clone(),
                name: node.name.clone(),
                element_kind: node.element_kind.clone(),
                range: node.range,
                parent: node
                    .parent_id
                    .as_ref()
                    .map(|parent| parent.qualified_name.clone()),
                attributes,
            });
        }
    }
    nodes.sort_by(|a, b| {
        a.uri
            .cmp(&b.uri)
            .then_with(|| a.qualified_name.cmp(&b.qualified_name))
            .then_with(|| a.element_kind.as_str().cmp(b.element_kind.as_str()))
    });

    let semantic_ids = nodes
        .iter()
        .map(|node| (node.qualified_name.as_str(), node.semantic_id.as_str()))
        .collect::<HashMap<_, _>>();
    let mut relationships = Vec::new();
    for node in &nodes {
        let Some(parent) = node.parent.as_deref() else {
            continue;
        };
        let Some(owner_id) = semantic_ids.get(parent).copied() else {
            continue;
        };
        relationships.push(HostSemanticModelRelationship {
            semantic_id: semantic_relationship_id(
                &sysml_model::RelationshipKind::Reference,
                owner_id,
                &node.semantic_id,
                relationships.len(),
            ),
            source_id: owner_id.to_owned(),
            target_id: node.semantic_id.clone(),
            owner_id: Some(owner_id.to_owned()),
            related_element_ids: vec![owner_id.to_owned(), node.semantic_id.clone()],
            range: Some(node.range),
            is_implied: false,
            metaclass: HostRelationshipMetaclass::Membership,
            source: parent.to_owned(),
            target: node.qualified_name.clone(),
            kind: sysml_model::RelationshipKind::Reference,
            connect: None,
        });
    }
    for uri in &target_urls {
        for (src_id, tgt_id, edge) in graph.edges_for_uri(uri) {
            let source_id = semantic_ids
                .get(src_id.qualified_name.as_str())
                .copied()
                .unwrap_or_default()
                .to_owned();
            let target_id = semantic_ids
                .get(tgt_id.qualified_name.as_str())
                .copied()
                .unwrap_or_default()
                .to_owned();
            // A resolved graph edge is owned by the specific/source element.
            // Its containing membership is separately projected above.
            let owner_id = (!source_id.is_empty()).then(|| source_id.clone());
            relationships.push(HostSemanticModelRelationship {
                semantic_id: semantic_relationship_id(
                    &edge.kind,
                    &source_id,
                    &target_id,
                    relationships.len(),
                ),
                source_id,
                target_id,
                owner_id,
                related_element_ids: [
                    semantic_ids
                        .get(src_id.qualified_name.as_str())
                        .copied()
                        .unwrap_or_default()
                        .to_owned(),
                    semantic_ids
                        .get(tgt_id.qualified_name.as_str())
                        .copied()
                        .unwrap_or_default()
                        .to_owned(),
                ]
                .into_iter()
                .filter(|id| !id.is_empty())
                .collect(),
                range: edge.connect.as_ref().map(|detail| detail.range),
                is_implied: false,
                metaclass: relationship_metaclass(&edge.kind),
                source: src_id.qualified_name,
                target: tgt_id.qualified_name,
                kind: edge.kind,
                connect: edge.connect,
            });
        }
    }
    fn connect_sort_key(c: &HostSemanticModelRelationship) -> Option<(&str, &str, u32, u32)> {
        c.connect.as_ref().map(|detail| {
            (
                detail.source_expression.as_str(),
                detail.target_expression.as_str(),
                detail.range.start.line,
                detail.range.start.character,
            )
        })
    }
    relationships.sort_by(|a, b| {
        a.source
            .cmp(&b.source)
            .then_with(|| a.target.cmp(&b.target))
            .then_with(|| a.metaclass.cmp(&b.metaclass))
            .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
            .then_with(|| connect_sort_key(a).cmp(&connect_sort_key(b)))
    });
    relationships.dedup_by(|a, b| {
        a.source == b.source
            && a.target == b.target
            && a.metaclass == b.metaclass
            && a.kind == b.kind
            && a.connect == b.connect
    });

    let mut expressions = Vec::new();
    let mut multiplicities = Vec::new();
    for node in &nodes {
        let Some(facts) = graph
            .node_ids_by_qualified_name
            .get(&node.qualified_name)
            .and_then(|ids| ids.first())
            .and_then(|id| graph.get_node(id))
            .map(|semantic| &semantic.declared_facts)
        else {
            continue;
        };
        let Some(multiplicity) = &facts.multiplicity else {
            continue;
        };
        let multiplicity_id = derived_fact_id("multiplicity", &node.semantic_id, "");
        let lower_bound_id = multiplicity
            .lower
            .as_ref()
            .map(|value| project_expression(value, &multiplicity_id, "lower", &mut expressions));
        let upper_bound_id = multiplicity
            .upper
            .as_ref()
            .map(|value| project_expression(value, &multiplicity_id, "upper", &mut expressions));
        multiplicities.push(HostMultiplicity {
            semantic_id: multiplicity_id,
            owner_id: node.semantic_id.clone(),
            lower_bound_id,
            upper_bound_id,
            range: multiplicity.range,
            is_implied: multiplicity.is_implied,
            is_ordered: multiplicity.is_ordered,
            is_unique: multiplicity.is_unique,
        });
    }
    Ok(HostSemanticProjection {
        nodes,
        relationships,
        multiplicities,
        expressions,
        feature_values: Vec::<HostFeatureValue>::new(),
    })
}

fn project_expression(
    expression: &sysml_model::DeclaredExpression,
    owner_id: &str,
    path: &str,
    output: &mut Vec<HostExpression>,
) -> String {
    let id = derived_fact_id("expression", owner_id, path);
    let operand_ids = expression
        .children
        .iter()
        .enumerate()
        .map(|(index, child)| project_expression(child, &id, &format!("operand-{index}"), output))
        .collect();
    let arguments = expression
        .arguments
        .iter()
        .enumerate()
        .map(|(index, argument)| HostExpressionArgument {
            name: argument.name.clone(),
            value_id: project_expression(
                &argument.value,
                &id,
                &format!("argument-{index}"),
                output,
            ),
        })
        .collect();
    output.push(HostExpression {
        semantic_id: id.clone(),
        kind: expression.kind.clone(),
        range: expression.range,
        literal: expression.literal.clone(),
        reference: expression.reference.clone(),
        operator: expression.operator.clone(),
        operand_ids,
        arguments,
    });
    id
}

fn derived_fact_id(kind: &str, owner_id: &str, path: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"spec42-semantic-fact-v4\0");
    hasher.update(kind.as_bytes());
    hasher.update([0]);
    hasher.update(owner_id.as_bytes());
    hasher.update([0]);
    hasher.update(path.as_bytes());
    format!("s42f:{:x}", hasher.finalize())
}

fn relationship_metaclass(kind: &sysml_model::RelationshipKind) -> HostRelationshipMetaclass {
    match kind {
        sysml_model::RelationshipKind::Typing => HostRelationshipMetaclass::FeatureTyping,
        sysml_model::RelationshipKind::Specializes => HostRelationshipMetaclass::Specialization,
        _ => HostRelationshipMetaclass::Relationship,
    }
}

fn semantic_element_id(
    uri: &str,
    kind: &sysml_model::ElementKind,
    line: u32,
    character: u32,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"spec42-semantic-element-v2\0");
    hasher.update(uri.as_bytes());
    hasher.update([0]);
    hasher.update(kind.as_str().as_bytes());
    hasher.update([0]);
    hasher.update(line.to_le_bytes());
    hasher.update(character.to_le_bytes());
    format!("s42e:{:x}", hasher.finalize())
}

fn semantic_relationship_id(
    kind: &sysml_model::RelationshipKind,
    source_id: &str,
    target_id: &str,
    ordinal: usize,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"spec42-semantic-relationship-v2\0");
    hasher.update(kind.as_str().as_bytes());
    hasher.update([0]);
    hasher.update(source_id.as_bytes());
    hasher.update([0]);
    hasher.update(target_id.as_bytes());
    hasher.update([0]);
    hasher.update(ordinal.to_le_bytes());
    format!("s42r:{:x}", hasher.finalize())
}

fn target_file_urls(
    target_files: &[std::path::PathBuf],
) -> crate::error::WorkspaceResult<BTreeSet<Url>> {
    target_files
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<Result<BTreeSet<_>, _>>()
}

fn collect_host_document_diagnostics(
    graph: &SemanticGraph,
    unit_registry: &UnitRegistry,
    library_urls: &[Url],
    uri: &Url,
    text: &str,
    strict_diagnostics: bool,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = sysml_model::collect_document_diagnostics(
        graph,
        unit_registry,
        !library_urls.is_empty(),
        uri,
        text,
        strict_diagnostics,
    );

    let has_parse_error = diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error && diagnostic.source == "sysml"
    });
    if strict_diagnostics && has_parse_error {
        diagnostics.retain(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error && diagnostic.source == "sysml"
        });
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::discovery::path_to_file_url;
    use sysml_model::{InMemoryDocumentProvider, build_semantic_graph_with_provider};

    fn make_provider(uri: &str, content: &str) -> InMemoryDocumentProvider {
        let doc = sysml_model::SysmlDocument {
            uri: url::Url::parse(uri).unwrap(),
            content: content.to_string(),
            path_hint: None,
            source_kind: sysml_model::SysmlDocumentSourceKind::Workspace,
            sha256: None,
            byte_size: None,
        };
        InMemoryDocumentProvider::new(vec![doc])
    }

    #[test]
    fn diagnostic_nodes_excluded_from_projection_and_present_in_validation() {
        // A connect statement with an unresolvable source produces a diagnostic node in the graph.
        let content = r#"
package Pkg {
    part def A { port pA; }
    part def B { port pB; }
    part a : A;
    part b : B;
    connection : connect a::pA to b::pBMissing;
}
"#;
        let uri = "file:///workspace/pkg.sysml";
        let provider = make_provider(uri, content);
        let (graph, _docs) = build_semantic_graph_with_provider(&provider).expect("graph");

        let target = std::path::PathBuf::from("/workspace/pkg.sysml");
        let projection =
            project_host_semantic_model(&graph, &[target.clone()]).expect("projection");

        assert!(
            projection
                .nodes
                .iter()
                .all(|n| n.element_kind != sysml_model::ElementKind::Diagnostic),
            "diagnostic pseudo-nodes must not appear in HostSemanticProjection"
        );
    }

    #[test]
    fn projection_exposes_typed_by_and_resolved_usage_context_for_part_usage() {
        // See docs/engineering/COMPONENT-IMPLEMENTATION-CONTEXT-ROADMAP.md: selecting a usage
        // (`cleaningHead`) should expose the implementation context of its resolved definition
        // (`CleaningHead`), not just the usage's own (empty) direct children.
        let content = r#"
package Demo {
    part def BrushMotor;
    part def CleaningHead {
        part brushMotor : BrushMotor;
    }
    part def Robot {
        part cleaningHead : CleaningHead;
    }
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/pkg.sysml"
        } else {
            "/workspace/pkg.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace pkg uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _docs) = build_semantic_graph_with_provider(&provider).expect("graph");

        let projection = project_host_semantic_model(&graph, &[target]).expect("projection");

        let usage = projection
            .nodes
            .iter()
            .find(|n| n.qualified_name == "Demo::Robot::cleaningHead")
            .expect("cleaningHead usage node present");

        let typed_by = usage
            .attributes
            .get("typedBy")
            .expect("typedBy attribute present");
        assert_eq!(
            typed_by.get("qualifiedName").and_then(|v| v.as_str()),
            Some("Demo::CleaningHead")
        );

        let context = usage
            .attributes
            .get("resolvedUsageContext")
            .expect("resolvedUsageContext attribute present");
        assert_eq!(
            context
                .get("resolvedDefinition")
                .and_then(|d| d.get("qualifiedName"))
                .and_then(|v| v.as_str()),
            Some("Demo::CleaningHead")
        );
        let parts = context
            .get("parts")
            .and_then(|v| v.as_array())
            .expect("parts array present");
        assert_eq!(parts.len(), 1);
        assert_eq!(
            parts[0].get("name").and_then(|v| v.as_str()),
            Some("brushMotor")
        );

        let ids_by_name = projection
            .nodes
            .iter()
            .map(|node| (node.qualified_name.as_str(), node.semantic_id.as_str()))
            .collect::<HashMap<_, _>>();
        assert!(
            projection
                .nodes
                .iter()
                .all(|node| node.semantic_id.starts_with("s42e:")),
            "every projected semantic element needs an opaque v2 identity"
        );
        assert!(
            projection.relationships.iter().all(|relationship| {
                relationship.semantic_id.starts_with("s42r:")
                    && ids_by_name.get(relationship.source.as_str())
                        == Some(&relationship.source_id.as_str())
                    && ids_by_name.get(relationship.target.as_str())
                        == Some(&relationship.target_id.as_str())
                    && relationship.related_element_ids
                        == vec![
                            relationship.source_id.clone(),
                            relationship.target_id.clone(),
                        ]
                    && !relationship.is_implied
            }),
            "relationship endpoints must be semantic IDs, not qualified-name identity"
        );
        assert!(
            projection.relationships.iter().any(|relationship| {
                relationship.metaclass == HostRelationshipMetaclass::Membership
                    && relationship.source == "Demo::Robot"
                    && relationship.target == "Demo::Robot::cleaningHead"
                    && relationship.owner_id.as_deref() == ids_by_name.get("Demo::Robot").copied()
                    && relationship.range.is_some()
            }),
            "parent ownership must be an addressable membership relationship"
        );
    }

    #[test]
    fn projection_materializes_typed_part_multiplicity_and_bounds() {
        let content = r#"
package Demo {
    part def Wheel;
    part def Car { part wheel : Wheel [1..*]; }
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/multiplicity.sysml"
        } else {
            "/workspace/multiplicity.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let projection = project_host_semantic_model(&graph, &[target]).expect("projection");
        let wheel = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name == "Demo::Car::wheel")
            .expect("wheel usage");
        let multiplicity = projection
            .multiplicities
            .iter()
            .find(|value| value.owner_id == wheel.semantic_id)
            .expect("multiplicity");
        let lower = multiplicity
            .lower_bound_id
            .as_deref()
            .and_then(|id| {
                projection
                    .expressions
                    .iter()
                    .find(|expression| expression.semantic_id == id)
            })
            .expect("lower bound");
        assert_eq!(lower.kind, "integerLiteral");
        assert_eq!(lower.literal, Some(serde_json::json!(1)));
        assert!(multiplicity.upper_bound_id.is_none(), "* is unbounded");
    }
}

fn summarize_host_documents(documents: &[HostValidatedDocument]) -> HostValidationSummary {
    let mut summary = HostValidationSummary {
        document_count: documents.len(),
        ..HostValidationSummary::default()
    };
    for document in documents {
        for diagnostic in &document.diagnostics {
            match diagnostic.severity {
                DiagnosticSeverity::Error => summary.error_count += 1,
                DiagnosticSeverity::Warning => summary.warning_count += 1,
                DiagnosticSeverity::Information | DiagnosticSeverity::Hint => {
                    summary.information_count += 1
                }
            }
        }
    }
    summary
}
