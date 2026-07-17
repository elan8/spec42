//! Host validation and projection assembly from a built semantic graph.

use std::collections::{BTreeSet, HashMap};

use sha2::{Digest, Sha256};

use sysml_model::{
    resolved_usage_context, typed_by_reference, DiagnosticSeverity, SemanticDiagnostic,
    SemanticGraph, SysmlDocument, UnitRegistry,
};
use url::Url;

use super::discovery::path_to_file_url;
use super::projection::{
    HostConnectorEnd, HostElementFacts, HostExpression, HostExpressionArgument,
    HostFeatureProperties, HostFeatureValue, HostMembershipKind, HostMultiplicity,
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
    library_urls: &[Url],
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

            let documentation = attributes
                .get("doc")
                .and_then(|value| value.as_str())
                .map(str::to_owned);
            let declared_short_name = attributes
                .get("shortName")
                .and_then(|value| value.as_str())
                .map(str::to_owned);

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
                facts: HostElementFacts {
                    declared_name: (!node.name.is_empty()).then(|| node.name.clone()),
                    effective_name: node.name.clone(),
                    owner_id: None,
                    owning_membership_id: None,
                    is_library_element: sysml_model::semantic::workspace_uri::uri_under_any_library(
                        &node.id.uri,
                        library_urls,
                    ),
                    documentation,
                    declared_short_name,
                    element_type: host_element_type(&node.element_kind),
                    feature_properties: node.declared_facts.feature_properties.as_ref().map(
                        |properties| HostFeatureProperties {
                            direction: properties.direction.clone(),
                            is_abstract: properties.is_abstract,
                            is_variation: properties.is_variation,
                            is_individual: properties.is_individual,
                            is_derived: properties.is_derived,
                            is_constant: properties.is_constant,
                            is_end: properties.is_end,
                            is_composite: properties.is_composite,
                            is_reference: properties.is_reference,
                            is_conjugated: properties.is_conjugated,
                            is_ordered: properties.is_ordered,
                            is_unique: properties.is_unique,
                        },
                    ),
                },
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
        .map(|node| (node.qualified_name.clone(), node.semantic_id.clone()))
        .collect::<HashMap<_, _>>();
    let element_kinds_by_qualified_name = nodes
        .iter()
        .map(|node| (node.qualified_name.clone(), node.element_kind.clone()))
        .collect::<HashMap<_, _>>();
    for node in &mut nodes {
        let Some(parent) = node.parent.as_deref() else {
            continue;
        };
        let Some(owner_id) = semantic_ids.get(parent) else {
            continue;
        };
        node.facts.owner_id = Some(owner_id.clone());
        node.facts.owning_membership_id = Some(semantic_relationship_id(
            &sysml_model::RelationshipKind::Reference,
            owner_id,
            &node.semantic_id,
            "membership".to_string(),
        ));
    }
    let mut relationships = Vec::new();
    for node in &nodes {
        let Some(parent) = node.parent.as_deref() else {
            continue;
        };
        let Some(owner_id) = semantic_ids.get(parent) else {
            continue;
        };
        let owner_kind = element_kinds_by_qualified_name.get(parent);
        let membership_kind = membership_kind(node, owner_kind);
        relationships.push(HostSemanticModelRelationship {
            semantic_id: node
                .facts
                .owning_membership_id
                .clone()
                .expect("owned node must have a membership identity"),
            source_id: owner_id.clone(),
            target_id: node.semantic_id.clone(),
            owner_id: Some(owner_id.clone()),
            related_element_ids: vec![owner_id.clone(), node.semantic_id.clone()],
            range: Some(node.range),
            is_implied: false,
            metaclass: membership_relationship_metaclass(node, membership_kind),
            membership_kind: Some(membership_kind),
            visibility: membership_visibility(node),
            source: parent.to_owned(),
            target: node.qualified_name.clone(),
            kind: sysml_model::RelationshipKind::Reference,
            connect: None,
            flow: None,
        });
    }
    for uri in &target_urls {
        for (src_id, tgt_id, edge) in graph.edges_for_uri(uri) {
            let source_id = semantic_ids
                .get(&src_id.qualified_name)
                .cloned()
                .unwrap_or_default()
                .to_owned();
            let target_id = semantic_ids
                .get(&tgt_id.qualified_name)
                .cloned()
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
                    edge_identity_discriminator(&edge),
                ),
                source_id,
                target_id,
                owner_id,
                related_element_ids: [
                    semantic_ids
                        .get(&src_id.qualified_name)
                        .cloned()
                        .unwrap_or_default()
                        .to_owned(),
                    semantic_ids
                        .get(&tgt_id.qualified_name)
                        .cloned()
                        .unwrap_or_default()
                        .to_owned(),
                ]
                .into_iter()
                .filter(|id| !id.is_empty())
                .collect(),
                range: edge.connect.as_ref().map(|detail| detail.range),
                is_implied: false,
                metaclass: relationship_metaclass(&edge.kind),
                membership_kind: None,
                visibility: None,
                source: src_id.qualified_name,
                target: tgt_id.qualified_name,
                kind: edge.kind,
                connect: edge.connect,
                flow: edge.flow,
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
    let mut feature_values = Vec::new();
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
        if let Some(multiplicity) = &facts.multiplicity {
            let multiplicity_id = derived_fact_id("multiplicity", &node.semantic_id, "");
            let lower_bound_id = multiplicity.lower.as_ref().map(|value| {
                project_expression(value, &multiplicity_id, "lower", &mut expressions)
            });
            let upper_bound_id = multiplicity.upper.as_ref().map(|value| {
                project_expression(value, &multiplicity_id, "upper", &mut expressions)
            });
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
        if let Some(value) = &facts.feature_value {
            let feature_value_id = derived_fact_id("featureValue", &node.semantic_id, "");
            let expression_id = project_expression(
                &value.expression,
                &feature_value_id,
                "expression",
                &mut expressions,
            );
            feature_values.push(HostFeatureValue {
                semantic_id: feature_value_id,
                owner_id: node.semantic_id.clone(),
                expression_id,
                kind: format!("{:?}", value.kind).to_ascii_lowercase(),
                range: value.range,
                is_implied: false,
            });
        }
    }
    // Connector ends for the binary `from ... to ...` case: derived directly from the
    // relationship's own already-resolved `source_id`/`target_id`, since a resolved `connect`
    // statement's endpoints are exactly a connector's first two ends. N-ary `connect (a, b, c,
    // ...)` ends beyond the binary pair are parsed (`ConnectStmt::extra_ends`) but not yet
    // resolved to feature IDs anywhere in the graph builder, so they are not projected here --
    // see spec42-systems-modeling-api-gaps.md for that follow-up.
    let connector_ends = relationships
        .iter()
        .filter(|relationship| relationship.connect.is_some())
        .flat_map(|relationship| {
            let detail = relationship
                .connect
                .as_ref()
                .expect("filtered to Some above");
            [
                (0u32, &relationship.source_id),
                (1u32, &relationship.target_id),
            ]
            .into_iter()
            .filter(|(_, feature_id)| !feature_id.is_empty())
            .map(|(end_index, feature_id)| HostConnectorEnd {
                semantic_id: derived_fact_id(
                    "connectorEnd",
                    &relationship.semantic_id,
                    &end_index.to_string(),
                ),
                owner_id: relationship.semantic_id.clone(),
                end_index,
                target_feature_id: Some(feature_id.clone()),
                range: detail.range,
                is_implied: relationship.is_implied,
            })
            .collect::<Vec<_>>()
        })
        .collect();

    Ok(HostSemanticProjection {
        nodes,
        relationships,
        multiplicities,
        expressions,
        feature_values,
        connector_ends,
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
        sysml_model::RelationshipKind::Specializes => HostRelationshipMetaclass::Subclassification,
        sysml_model::RelationshipKind::Subsetting => HostRelationshipMetaclass::Subsetting,
        sysml_model::RelationshipKind::Redefinition => HostRelationshipMetaclass::Redefinition,
        sysml_model::RelationshipKind::Annotation => HostRelationshipMetaclass::Annotation,
        sysml_model::RelationshipKind::Satisfy => HostRelationshipMetaclass::Satisfy,
        sysml_model::RelationshipKind::Subject => HostRelationshipMetaclass::Subject,
        _ => HostRelationshipMetaclass::Relationship,
    }
}

fn membership_kind(
    node: &HostSemanticModelNode,
    owner_kind: Option<&sysml_model::ElementKind>,
) -> HostMembershipKind {
    use sysml_model::ElementKind;

    if node
        .attributes
        .get("isVariant")
        .and_then(|value| value.as_bool())
        == Some(true)
        || node.element_kind.as_str() == "variant"
    {
        return HostMembershipKind::VariantMembership;
    }

    match &node.element_kind {
        ElementKind::Import => HostMembershipKind::Import,
        ElementKind::Alias => HostMembershipKind::Alias,
        ElementKind::Actor => HostMembershipKind::ActorMembership,
        ElementKind::Subject => HostMembershipKind::SubjectMembership,
        ElementKind::Stakeholder => HostMembershipKind::StakeholderMembership,
        ElementKind::Objective => HostMembershipKind::ObjectiveMembership,
        ElementKind::ViewRendering => HostMembershipKind::ViewRenderingMembership,
        // `InOutDecl` is shared grammar: Action/Calc definition and usage bodies own genuine
        // Behavior parameters (ParameterMembership per KerML 8.3.19.2), but Port/PortDef bodies
        // reuse the same production for directed (in/out) features, which are ordinary
        // FeatureMembership, not parameters. Only the owner distinguishes the two; the parser
        // does not project a `constraint def`'s InOutDecl as a node at all (folded into a text
        // attribute), so no owner_kind arm is needed for Constraint here.
        ElementKind::InOutParameter => match owner_kind {
            Some(
                ElementKind::ActionDef
                | ElementKind::Action
                | ElementKind::CalcDef
                | ElementKind::Calc,
            ) => HostMembershipKind::ParameterMembership,
            _ => HostMembershipKind::FeatureMembership,
        },
        kind if kind.is_definition() => HostMembershipKind::OwningMembership,
        ElementKind::Package | ElementKind::KermlDecl | ElementKind::Filter => {
            HostMembershipKind::OwningMembership
        }
        ElementKind::Part
        | ElementKind::Port
        | ElementKind::Item
        | ElementKind::Attribute
        | ElementKind::Action
        | ElementKind::State
        | ElementKind::Requirement
        | ElementKind::UseCase
        | ElementKind::Concern
        | ElementKind::Analysis
        | ElementKind::Verification
        | ElementKind::View
        | ElementKind::Viewpoint
        | ElementKind::Rendering
        | ElementKind::MetadataUsage
        | ElementKind::Flow
        | ElementKind::Allocation
        | ElementKind::Perform
        | ElementKind::Ref
        | ElementKind::Constraint
        | ElementKind::Connection
        | ElementKind::Individual
        | ElementKind::Occurrence
        | ElementKind::Calc
        | ElementKind::Interface
        | ElementKind::IncludeUseCase
        | ElementKind::VerifiedRequirement => HostMembershipKind::FeatureMembership,
        ElementKind::Documentation => HostMembershipKind::OwningMembership,
        _ => HostMembershipKind::OwningMembership,
    }
}

fn host_element_type(kind: &sysml_model::ElementKind) -> Option<String> {
    match kind {
        sysml_model::ElementKind::Ref => Some("ReferenceUsage".to_owned()),
        sysml_model::ElementKind::Documentation => Some("Documentation".to_owned()),
        _ => None,
    }
}

fn membership_relationship_metaclass(
    node: &HostSemanticModelNode,
    kind: HostMembershipKind,
) -> HostRelationshipMetaclass {
    match kind {
        HostMembershipKind::Import => {
            if node
                .attributes
                .get("importAll")
                .and_then(|value| value.as_bool())
                == Some(true)
            {
                HostRelationshipMetaclass::NamespaceImport
            } else {
                HostRelationshipMetaclass::MembershipImport
            }
        }
        HostMembershipKind::Alias => HostRelationshipMetaclass::AliasMembership,
        _ => HostRelationshipMetaclass::Membership,
    }
}

fn membership_visibility(node: &HostSemanticModelNode) -> Option<String> {
    node.attributes
        .get("visibility")
        .and_then(|value| value.as_str())
        .map(|value| value.trim_matches('"').to_owned())
}

fn edge_identity_discriminator(edge: &sysml_model::SemanticEdge) -> String {
    edge.connect
        .as_ref()
        .map(|detail| {
            format!(
                "{}:{}:{}:{}:{}",
                detail.declaring_uri,
                detail.range.start.line,
                detail.range.start.character,
                detail.source_expression,
                detail.target_expression
            )
        })
        .unwrap_or_default()
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
    discriminator: String,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"spec42-semantic-relationship-v2\0");
    hasher.update(kind.as_str().as_bytes());
    hasher.update([0]);
    hasher.update(source_id.as_bytes());
    hasher.update([0]);
    hasher.update(target_id.as_bytes());
    hasher.update([0]);
    hasher.update(discriminator.as_bytes());
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
    use sysml_model::{build_semantic_graph_with_provider, InMemoryDocumentProvider};

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
            project_host_semantic_model(&graph, &[target.clone()], &[]).expect("projection");

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

        let projection = project_host_semantic_model(&graph, &[target], &[]).expect("projection");

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
        let membership = projection
            .relationships
            .iter()
            .find(|relationship| {
                relationship.metaclass == HostRelationshipMetaclass::Membership
                    && relationship.target == "Demo::Robot::cleaningHead"
            })
            .expect("feature membership");
        assert_eq!(
            membership.membership_kind,
            Some(HostMembershipKind::FeatureMembership)
        );
        assert_eq!(
            usage.facts.owner_id.as_deref(),
            Some(membership.source_id.as_str())
        );
        assert_eq!(
            usage.facts.owning_membership_id.as_deref(),
            Some(membership.semantic_id.as_str())
        );
        assert_eq!(usage.facts.declared_name.as_deref(), Some("cleaningHead"));
        assert_eq!(usage.facts.effective_name, "cleaningHead");
        assert!(!usage.facts.is_library_element);
        let properties = usage
            .facts
            .feature_properties
            .as_ref()
            .expect("part usage retains declared feature properties");
        assert!(!properties.is_abstract);
        assert!(!properties.is_derived);
        assert_eq!(properties.is_ordered, Some(false));
    }

    #[test]
    fn projection_exposes_declared_feature_properties_for_modifiers() {
        let content = r#"
package Demo {
    attribute def Temperature;
    abstract part def Sensor {
        attribute reading : Temperature ordered;
        derived constant attribute bias : Temperature;
        end attribute mount;
    }
    port def SignalPort {
        in item request;
        out item response;
    }
    individual part sensor : Sensor;
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/feature_properties.sysml"
        } else {
            "/workspace/feature_properties.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let projection = project_host_semantic_model(&graph, &[target], &[]).expect("projection");
        let names = projection
            .nodes
            .iter()
            .map(|node| node.qualified_name.as_str())
            .collect::<Vec<_>>();

        let sensor_def = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name == "Demo::Sensor")
            .unwrap_or_else(|| panic!("Sensor definition missing; nodes={names:?}"));
        let def_props = sensor_def
            .facts
            .feature_properties
            .as_ref()
            .expect("definition feature properties");
        assert!(def_props.is_abstract);
        assert!(!def_props.is_individual);

        let reading = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::reading"))
            .unwrap_or_else(|| panic!("reading attribute missing; nodes={names:?}"));
        let reading_props = reading
            .facts
            .feature_properties
            .as_ref()
            .expect("reading feature properties");
        assert_eq!(reading_props.is_ordered, Some(true));
        assert_eq!(reading_props.is_unique, Some(true));

        let bias = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::bias"))
            .unwrap_or_else(|| panic!("bias attribute missing; nodes={names:?}"));
        let bias_props = bias
            .facts
            .feature_properties
            .as_ref()
            .expect("bias feature properties");
        assert!(bias_props.is_derived);
        assert!(bias_props.is_constant);

        let mount = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::mount"))
            .unwrap_or_else(|| panic!("mount attribute missing; nodes={names:?}"));
        assert!(
            mount
                .facts
                .feature_properties
                .as_ref()
                .expect("mount feature properties")
                .is_end
        );

        let request = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::request"))
            .unwrap_or_else(|| panic!("request item missing; nodes={names:?}"));
        assert_eq!(
            request
                .facts
                .feature_properties
                .as_ref()
                .expect("request feature properties")
                .direction
                .as_deref(),
            Some("in")
        );

        let response = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::response"))
            .unwrap_or_else(|| panic!("response item missing; nodes={names:?}"));
        assert_eq!(
            response
                .facts
                .feature_properties
                .as_ref()
                .expect("response feature properties")
                .direction
                .as_deref(),
            Some("out")
        );

        let sensor_usage = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name == "Demo::sensor")
            .unwrap_or_else(|| panic!("sensor usage missing; nodes={names:?}"));
        assert!(
            sensor_usage
                .facts
                .feature_properties
                .as_ref()
                .expect("usage feature properties")
                .is_individual
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
        let projection = project_host_semantic_model(&graph, &[target], &[]).expect("projection");
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

    #[test]
    fn relationship_ids_are_independent_of_graph_enumeration_order() {
        let content = r#"
package Demo {
    part def Wheel;
    part def Car { part wheel : Wheel; }
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/stable.sysml"
        } else {
            "/workspace/stable.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let first = project_host_semantic_model(&graph, std::slice::from_ref(&target), &[])
            .expect("first projection");
        let second =
            project_host_semantic_model(&graph, &[target], &[]).expect("second projection");
        assert_eq!(
            first
                .relationships
                .iter()
                .map(|relationship| &relationship.semantic_id)
                .collect::<Vec<_>>(),
            second
                .relationships
                .iter()
                .map(|relationship| &relationship.semantic_id)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn projection_exposes_ref_ownership_and_composite_usage_defaults() {
        let content = r#"
package Demo {
    part def Tree;
    part def Orbit {
        ref sharedBranch : Tree;
        part localBranch : Tree;
    }
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/ref_ownership.sysml"
        } else {
            "/workspace/ref_ownership.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let projection = project_host_semantic_model(&graph, &[target], &[]).expect("projection");

        let shared = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::sharedBranch"))
            .expect("sharedBranch ref");
        let shared_props = shared
            .facts
            .feature_properties
            .as_ref()
            .expect("ref feature properties");
        assert_eq!(shared_props.is_reference, Some(true));
        assert_eq!(shared_props.is_composite, Some(false));
        assert_eq!(shared.facts.element_type.as_deref(), Some("ReferenceUsage"));
        assert_eq!(
            projection
                .relationships
                .iter()
                .find(|relationship| {
                    relationship.metaclass == HostRelationshipMetaclass::Membership
                        && relationship.target == shared.qualified_name
                })
                .expect("ref membership")
                .membership_kind,
            Some(HostMembershipKind::FeatureMembership)
        );

        let local = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::localBranch"))
            .expect("localBranch part");
        let local_props = local
            .facts
            .feature_properties
            .as_ref()
            .expect("part feature properties");
        assert_eq!(local_props.is_composite, Some(true));
        assert_eq!(local_props.is_reference, Some(false));
    }

    #[test]
    fn projection_membership_kinds_for_import_alias_actor_and_defs() {
        let content = r#"
package Demo {
    private import Outer::*;
    alias AliasName for Outer::Thing;
    part def Thing;
    requirement def Req {
        subject subj : Thing;
        stakeholder holder : Thing;
    }
    use case def Mission {
        actor operator;
        objective goal : Req;
    }
    action def Survey {
        in target : Thing;
        out result : Thing;
    }
    port def Feed {
        in signal : Thing;
    }
    rendering def Style {
        render diagram : Thing;
    }
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/membership_kinds.sysml"
        } else {
            "/workspace/membership_kinds.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let projection = project_host_semantic_model(&graph, &[target], &[]).expect("projection");

        let membership_for = |qualified: &str| {
            projection
                .relationships
                .iter()
                .find(|relationship| relationship.target == qualified)
                .map(|relationship| (relationship.metaclass, relationship.membership_kind))
        };

        let thing = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::Thing"))
            .expect("Thing def");
        assert_eq!(
            membership_for(&thing.qualified_name),
            Some((
                HostRelationshipMetaclass::Membership,
                Some(HostMembershipKind::OwningMembership)
            ))
        );

        let import_node = projection
            .nodes
            .iter()
            .find(|node| node.element_kind == sysml_model::ElementKind::Import)
            .expect("import node");
        assert_eq!(
            membership_for(&import_node.qualified_name),
            Some((
                HostRelationshipMetaclass::NamespaceImport,
                Some(HostMembershipKind::Import)
            ))
        );

        let alias_node = projection
            .nodes
            .iter()
            .find(|node| node.element_kind == sysml_model::ElementKind::Alias)
            .expect("alias node");
        assert_eq!(
            membership_for(&alias_node.qualified_name),
            Some((
                HostRelationshipMetaclass::AliasMembership,
                Some(HostMembershipKind::Alias)
            ))
        );

        if let Some(actor_node) = projection
            .nodes
            .iter()
            .find(|node| node.element_kind == sysml_model::ElementKind::Actor)
        {
            assert_eq!(
                membership_for(&actor_node.qualified_name),
                Some((
                    HostRelationshipMetaclass::Membership,
                    Some(HostMembershipKind::ActorMembership)
                ))
            );
        }

        // Regression guard: `subject`/`stakeholder` used to fall into the generic
        // `FeatureMembership` bucket, and `objective` was unhandled entirely (silently defaulted
        // to `OwningMembership`).
        let subject_node = projection
            .nodes
            .iter()
            .find(|node| node.element_kind == sysml_model::ElementKind::Subject)
            .expect("subject node");
        assert_eq!(
            membership_for(&subject_node.qualified_name),
            Some((
                HostRelationshipMetaclass::Membership,
                Some(HostMembershipKind::SubjectMembership)
            ))
        );

        let stakeholder_node = projection
            .nodes
            .iter()
            .find(|node| node.element_kind == sysml_model::ElementKind::Stakeholder)
            .expect("stakeholder node");
        assert_eq!(
            membership_for(&stakeholder_node.qualified_name),
            Some((
                HostRelationshipMetaclass::Membership,
                Some(HostMembershipKind::StakeholderMembership)
            ))
        );

        let objective_node = projection
            .nodes
            .iter()
            .find(|node| node.element_kind == sysml_model::ElementKind::Objective)
            .expect("objective node");
        assert_eq!(
            membership_for(&objective_node.qualified_name),
            Some((
                HostRelationshipMetaclass::Membership,
                Some(HostMembershipKind::ObjectiveMembership)
            ))
        );

        // Regression guard: `InOutDecl` is shared grammar between Action/Calc parameters and
        // Port directed features. Only the owner distinguishes genuine KerML ParameterMembership
        // (Behavior parameters) from ordinary FeatureMembership (port in/out features) — both
        // used to fall into the generic OwningMembership default.
        let action_param = projection
            .nodes
            .iter()
            .find(|node| {
                node.element_kind == sysml_model::ElementKind::InOutParameter
                    && node.qualified_name.ends_with("::target")
            })
            .expect("action parameter node");
        assert_eq!(
            membership_for(&action_param.qualified_name),
            Some((
                HostRelationshipMetaclass::Membership,
                Some(HostMembershipKind::ParameterMembership)
            ))
        );

        let port_feature = projection
            .nodes
            .iter()
            .find(|node| {
                node.element_kind == sysml_model::ElementKind::InOutParameter
                    && node.qualified_name.ends_with("::signal")
            })
            .expect("port in/out feature node");
        assert_eq!(
            membership_for(&port_feature.qualified_name),
            Some((
                HostRelationshipMetaclass::Membership,
                Some(HostMembershipKind::FeatureMembership)
            ))
        );

        // Regression guard: `view rendering` members used to fall into the generic
        // `FeatureMembership` bucket alongside every other feature-owning usage kind.
        let view_rendering_node = projection
            .nodes
            .iter()
            .find(|node| node.element_kind == sysml_model::ElementKind::ViewRendering)
            .expect("view rendering node");
        assert_eq!(
            membership_for(&view_rendering_node.qualified_name),
            Some((
                HostRelationshipMetaclass::Membership,
                Some(HostMembershipKind::ViewRenderingMembership)
            ))
        );
    }

    #[test]
    fn projection_relationship_family_subset_redefine_specialize_annotation() {
        let content = r#"
package Demo {
    metadata def Tag;
    part def Base {
        attribute mass;
        port signal;
    }
    part def Child specializes Base {
        attribute payload subsets mass;
        port cmd redefines signal;
        @Tag;
    }
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/rel_family.sysml"
        } else {
            "/workspace/rel_family.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let projection = project_host_semantic_model(&graph, &[target], &[]).expect("projection");

        assert!(
            projection.relationships.iter().any(|relationship| {
                relationship.kind == sysml_model::RelationshipKind::Specializes
                    && relationship.metaclass == HostRelationshipMetaclass::Subclassification
                    && relationship.source.ends_with("::Child")
            }),
            "specializes should project as Subclassification"
        );
        assert!(
            projection.relationships.iter().any(|relationship| {
                relationship.kind == sysml_model::RelationshipKind::Subsetting
                    && relationship.metaclass == HostRelationshipMetaclass::Subsetting
            }),
            "subsets should project as Subsetting; relationships={:?}",
            projection
                .relationships
                .iter()
                .map(|r| (&r.kind, &r.metaclass, &r.source, &r.target))
                .collect::<Vec<_>>()
        );
        assert!(
            projection.relationships.iter().any(|relationship| {
                relationship.kind == sysml_model::RelationshipKind::Redefinition
                    && relationship.metaclass == HostRelationshipMetaclass::Redefinition
            }),
            "redefines should project as Redefinition"
        );
        assert!(
            projection.relationships.iter().any(|relationship| {
                relationship.kind == sysml_model::RelationshipKind::Annotation
                    && relationship.metaclass == HostRelationshipMetaclass::Annotation
            }),
            "annotation edges should project Annotation metaclass"
        );
    }

    #[test]
    fn projection_lifts_doc_short_name_and_conjugated_port() {
        let content = r#"
package Demo {
    part def <'CB'> ControlBoard {
        doc /* Control board assembly */
        port power : ~PowerPort;
    }
    port def PowerPort;
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/names_conj.sysml"
        } else {
            "/workspace/names_conj.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let projection = project_host_semantic_model(&graph, &[target], &[]).expect("projection");

        let board = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::ControlBoard"))
            .expect("ControlBoard");
        assert_eq!(board.facts.declared_short_name.as_deref(), Some("CB"));
        assert_eq!(
            board.facts.documentation.as_deref(),
            Some("Control board assembly")
        );
        let documentation = projection
            .nodes
            .iter()
            .find(|node| node.element_kind == sysml_model::ElementKind::Documentation)
            .expect("Documentation node");
        assert_eq!(
            documentation.facts.element_type.as_deref(),
            Some("Documentation")
        );
        assert!(
            projection.relationships.iter().any(|relationship| {
                relationship.metaclass == HostRelationshipMetaclass::Annotation
                    && relationship.source_id == documentation.semantic_id
                    && relationship.target_id == board.semantic_id
            }),
            "Documentation should annotate ControlBoard"
        );

        let power = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::power"))
            .expect("power port");
        assert!(
            power
                .facts
                .feature_properties
                .as_ref()
                .expect("port properties")
                .is_conjugated
        );
    }

    #[test]
    fn projection_attribute_usage_multiplicity_is_addressable() {
        let content = r#"
package Demo {
    part def Sensor {
        attribute mass [0..1] ordered;
        attribute tags : String[0..*] nonunique;
    }
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/workspace/attr_mult.sysml"
        } else {
            "/workspace/attr_mult.sysml"
        });
        let uri = path_to_file_url(target.as_path()).expect("workspace uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let projection = project_host_semantic_model(&graph, &[target], &[]).expect("projection");

        let mass = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::mass"))
            .expect("mass");
        assert!(
            projection
                .multiplicities
                .iter()
                .any(|value| value.owner_id == mass.semantic_id && value.is_ordered),
            "mass should project ordered multiplicity; multiplicities={:?}",
            projection.multiplicities
        );

        let tags = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with("::tags"))
            .expect("tags");
        assert!(
            projection
                .multiplicities
                .iter()
                .any(|value| value.owner_id == tags.semantic_id && value.is_unique == Some(false)),
            "tags should project nonunique multiplicity"
        );
    }

    #[test]
    fn projection_marks_library_elements_from_library_urls() {
        let content = r#"
package LibPkg {
    part def LibraryPart;
}
"#;
        let target = std::path::PathBuf::from(if cfg!(windows) {
            "c:/libs/std/lib.sysml"
        } else {
            "/libs/std/lib.sysml"
        });
        let library_root = std::path::PathBuf::from(if cfg!(windows) {
            "c:/libs/std"
        } else {
            "/libs/std"
        });
        let uri = path_to_file_url(target.as_path()).expect("library uri");
        let library_url = path_to_file_url(library_root.as_path()).expect("library root uri");
        let provider = make_provider(uri.as_str(), content);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        let projection =
            project_host_semantic_model(&graph, &[target], &[library_url]).expect("projection");
        assert!(
            projection
                .nodes
                .iter()
                .any(|node| node.facts.is_library_element),
            "nodes under library URLs should set is_library_element"
        );
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
